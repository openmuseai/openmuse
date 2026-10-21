import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/scheduled_task_service.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_host/src/host/workspace_mirror_service.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'Mobile → paired HTTP → Desktop domains → DSH prompt and files',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'openmuse-domain-e2e-',
      );
      final folder = await Directory('${root.path}/notes').create();
      await File('${folder.path}/plan.md').writeAsString('# Release plan');
      final dshRequests = <Map<String, Object?>>[];
      final dsh = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      dsh.listen((request) async {
        final payload = jsonDecode(await utf8.decoder.bind(request).join());
        dshRequests.add({
          'path': request.uri.path,
          'token': request.headers.value('x-openmuse-bridge-token'),
          'body': payload,
        });
        request.response.headers.contentType = ContentType.json;
        request.response.write(
          jsonEncode(
            request.uri.path.endsWith('/create')
                ? {'sessionId': 'session-e2e'}
                : {'accepted': true},
          ),
        );
        await request.response.close();
      });
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final mirror = DesktopWorkspaceMirrorService(workspace);
      var now = DateTime.utc(2026, 10, 15, 9);
      final storeFile = File('${root.path}/scheduled.json');
      DesktopScheduledTaskService makeSchedule() => DesktopScheduledTaskService(
        storeFile: storeFile,
        clock: () => now,
        executor: DshScheduledPromptExecutor(
          endpoint: () async => Uri.parse('http://127.0.0.1:${dsh.port}/'),
          bridgeToken: 'private-desktop-token',
        ).call,
      );
      var schedules = makeSchedule();
      await schedules.restore();
      var currentAccount = 'account-e2e';
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => currentAccount,
        validateToken: (_) async => 'account-e2e',
        dshEndpoint: () async => Uri.parse('http://127.0.0.1:${dsh.port}/'),
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Project Workspace',
        deviceRef: 'desktop-e2e',
        port: 0,
        workspaceMirror: mirror.handle,
        workspaceMirrorResource: mirror.readResource,
        scheduledTasks: (request) async {
          switch (request.operation) {
            case 'list':
              return {'tasks': schedules.list()};
            case 'create':
              return {
                'task': await schedules.create(
                  workspaceId: request.workspaceId!,
                  message: request.message!,
                  dueAt: request.dueAt!,
                ),
              };
            case 'cancel':
              await schedules.cancel(request.id!);
              return {'cancelled': true};
            default:
              throw const FormatException('unknown schedule operation');
          }
        },
      );
      await gateway.start();
      final pair = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'account-token',
        deviceRef: 'mobile-e2e',
        allowInsecureLoopback: true,
      );
      try {
        final connection = await pair.connectSameAccount(
          targetDeviceRef: 'desktop-e2e',
        );
        final mobileSchedules = MobileScheduledTasksPlugin();
        mobileSchedules.updateContext(
          connection: connection,
          workspaces: const [
            WbWorkspace(
              id: 'dsh.workspace.workspace-e2e',
              name: 'Project',
              deviceId: 'desktop-e2e',
            ),
          ],
          selectedWorkspaceId: 'dsh.workspace.workspace-e2e',
        );
        final bootstrap =
            await mobileSchedules.handleMobileDetailRequest('bootstrap', {})
                as Map;
        expect(bootstrap['defaultWorkspaceId'], 'workspace-e2e');
        final created =
            await mobileSchedules.handleMobileDetailRequest('create', {
                  'workspaceId': 'workspace-e2e',
                  'message': '请总结发布计划',
                  'dueAt': now
                      .add(const Duration(minutes: 2))
                      .toIso8601String(),
                })
                as Map;
        final id = (created['task'] as Map)['id'];
        expect(id, isA<String>());
        expect(schedules.list().single['state'], 'pending');
        now = now.add(const Duration(minutes: 3));
        await schedules.runDue();
        await schedules.runDue();
        expect(schedules.list().single['state'], 'completed');
        expect(dshRequests, hasLength(2));
        expect(dshRequests.first['token'], 'private-desktop-token');
        expect((dshRequests.last['body'] as Map)['requestId'], id);
        expect((dshRequests.last['body'] as Map)['content'], [
          {'type': 'text', 'text': '请总结发布计划'},
        ]);
        schedules.dispose();
        schedules = makeSchedule();
        await schedules.restore();
        expect(schedules.list().single['state'], 'completed');
        expect(dshRequests, hasLength(2));

        final mobileFiles = MobileWorkspaceFilesPlugin();
        mobileFiles.updateConnection(connection);
        final mounts =
            await mobileFiles.handleMobileDetailRequest('mounts', {}) as Map;
        final mount = (mounts['mounts'] as List).single as Map;
        expect(jsonEncode(mounts), isNot(contains(root.path)));
        final children =
            await mobileFiles.handleMobileDetailRequest('children', {
                  'mountRef': mount['mountRef'],
                  'parentRef': mount['rootRef'],
                })
                as Map;
        final directory =
            (children['entries'] as List).singleWhere(
                  (item) => (item as Map)['name'] == 'notes',
                )
                as Map;
        final nested =
            await mobileFiles.handleMobileDetailRequest('children', {
                  'mountRef': mount['mountRef'],
                  'parentRef': directory['nodeRef'],
                })
                as Map;
        final file = (nested['entries'] as List).single as Map;
        expect(file['name'], 'plan.md');
        final search =
            await mobileFiles.handleMobileDetailRequest('search', {
                  'query': 'plan',
                })
                as Map;
        expect((search['entries'] as List).single['name'], 'plan.md');
        expect(jsonEncode(search), isNot(contains(root.path)));
        final preview =
            await mobileFiles.handleMobileDetailRequest('preview', {
                  'resourceRef': file['resourceRef'],
                })
                as Map;
        expect(preview['type'], 'text');
        expect(preview['content'], '# Release plan');
        currentAccount = 'account-other';
        await expectLater(
          mobileFiles.handleMobileDetailRequest('mounts', {}),
          throwsStateError,
        );
        await expectLater(
          mobileSchedules.handleMobileDetailRequest('bootstrap', {}),
          throwsStateError,
        );
      } finally {
        pair.close();
        await gateway.stop();
        schedules.dispose();
        workspace.dispose();
        await dsh.close(force: true);
        await root.delete(recursive: true);
      }
    },
  );
}
