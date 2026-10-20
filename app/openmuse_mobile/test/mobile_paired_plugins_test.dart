import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';

void main() {
  test(
    'paired Mobile plugins use the Desktop capability routes end to end',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      upstream.listen((request) async {
        await request.drain<void>();
        await request.response.close();
      });
      final scheduleCalls = <PairedScheduledTaskRequest>[];
      final mirrorCalls = <WorkspaceMirrorQuery>[];
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => 'account-1',
        validateToken: (_) async => 'account-1',
        dshEndpoint: () async =>
            Uri.parse('http://127.0.0.1:${upstream.port}/'),
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Project Workspace',
        port: 0,
        scheduledTasks: (request) async {
          scheduleCalls.add(request);
          return request.operation == 'list'
              ? {'tasks': <Object?>[]}
              : {
                  'task': {'id': 'task-1', 'state': 'pending'},
                };
        },
        workspaceMirror: (query) async {
          mirrorCalls.add(query);
          return switch (query.operation) {
            'mounts' => {
              'mounts': [
                {
                  'mountRef': 'mount-1',
                  'rootRef': 'root-1',
                  'title': 'Project',
                },
              ],
            },
            'children' || 'search' => {
              'entries': [
                {
                  'mountRef': 'mount-1',
                  'nodeRef': 'node-1',
                  'resourceRef': 'resource-1',
                  'name': 'README.md',
                  'isDirectory': false,
                },
              ],
              'nextCursor': null,
            },
            _ => throw const FormatException('unknown mirror operation'),
          };
        },
        workspaceMirrorResource: (query) async =>
            query.resourceRef == 'resource-pptx'
            ? WorkspaceMirrorResource(
                bytes: Uint8List(3 * 1024 * 1024),
                mediaType:
                    'application/vnd.openxmlformats-officedocument.presentationml.presentation',
              )
            : WorkspaceMirrorResource(
                bytes: Uint8List.fromList(utf8.encode('# Project')),
                mediaType: 'text/markdown; charset=utf-8',
              ),
      );
      await gateway.start();
      final pair = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'token',
        deviceRef: 'mobile-1',
        allowInsecureLoopback: true,
      );
      try {
        final connection = await pair.connectSameAccount(
          targetDeviceRef: 'desktop.local',
        );
        final schedules = MobileScheduledTasksPlugin();
        schedules.updateContext(
          connection: connection,
          workspaces: const [
            WbWorkspace(
              id: 'dsh.workspace.workspace-1',
              name: 'Project',
              deviceId: 'desktop.local',
            ),
          ],
          selectedWorkspaceId: 'dsh.workspace.workspace-1',
        );
        final bootstrap =
            await schedules.handleMobileDetailRequest('bootstrap', {}) as Map;
        expect(bootstrap['defaultWorkspaceId'], 'workspace-1');
        await schedules.handleMobileDetailRequest('create', {
          'workspaceId': 'workspace-1',
          'message': '请总结进度',
          'dueAt': DateTime.now()
              .toUtc()
              .add(const Duration(minutes: 2))
              .toIso8601String(),
        });
        expect(scheduleCalls.last.workspaceId, 'workspace-1');
        expect(scheduleCalls.last.message, '请总结进度');
        expect(scheduleCalls.last.accountRef, 'account-1');

        final files = MobileWorkspaceFilesPlugin();
        files.updateConnection(connection);
        final mounts =
            await files.handleMobileDetailRequest('mounts', {}) as Map;
        expect((mounts['mounts'] as List).single['title'], 'Project');
        final children =
            await files.handleMobileDetailRequest('children', {
                  'mountRef': 'mount-1',
                  'parentRef': 'root-1',
                })
                as Map;
        expect((children['entries'] as List).single['name'], 'README.md');
        final search =
            await files.handleMobileDetailRequest('search', {'query': 'README'})
                as Map;
        expect((search['entries'] as List).single['resourceRef'], 'resource-1');
        final preview =
            await files.handleMobileDetailRequest('preview', {
                  'resourceRef': 'resource-1',
                })
                as Map;
        expect(preview['content'], '# Project');
        var opened = false;
        files.onOpenResource = (resource, read, reportStep) async {
          expect(resource.displayName, 'Deck.pptx');
          final payload = await read();
          expect((payload['bytes'] as Uint8List).length, 3 * 1024 * 1024);
          expect(
            payload['mediaType'],
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          );
          expect(resource.uri.toString(), isNot(contains(connection.grantRef)));
          final progress =
              await files.handleMobileDetailRequest('openingProgress', {})
                  as Map;
          expect(progress['received'], 3 * 1024 * 1024);
          expect(progress['total'], 3 * 1024 * 1024);
          expect(progress['step'], '接收 Desktop 文件');
          reportStep('启动本地编辑器');
          opened = true;
        };
        await files.handleMobileDetailRequest('openResource', {
          'resourceRef': 'resource-pptx',
          'name': 'Deck.pptx',
        });
        expect(opened, isTrue);

        expect(
          mirrorCalls.map((call) => call.operation),
          containsAll(['mounts', 'children', 'search']),
        );
      } finally {
        pair.close();
        await gateway.stop();
        await upstream.close(force: true);
      }
    },
  );
}
