import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

final class _LiveLoopbackHttp extends HttpOverrides {}

void main() {
  LiveTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('RS-UI-06 the task drawer drives the paired gateway', (
    tester,
  ) async {
    await HttpOverrides.runWithHttpOverrides(
      () => _walkPairedWorkbench(tester),
      _LiveLoopbackHttp(),
    );
  });
}

Future<void> _walkPairedWorkbench(WidgetTester tester) async {
    await tester.binding.setSurfaceSize(const Size(390, 844));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

  final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  final proxied = <String>[];
  upstream.listen((request) async {
    proxied.add(request.uri.path);
    await request.response.close();
  });
  final desktop = AcceptanceDesktop();
  final gateway = PairedDesktopGateway(
    currentAccountRef: () => 'account-1',
    validateToken: (token) async => 'account-1',
    dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
    workspaceRef: 'ws.opaque.1',
    workspaceTitle: '验收工作区',
    deviceRef: 'desktop.1',
    port: 0,
    remoteSurface: (request) => dispatchRemoteSurface(
      host: desktop.host,
      operation: request.operation,
      body: request.body,
      context: RemoteConnectionContext(
        actorRef: request.accountRef,
        mobileDeviceRef: request.deviceRef,
        desktopDeviceRef: 'desktop.1',
        workspaceRef: request.workspaceRef,
        permissions: desktop.connection.permissions,
      ),
    ),
    remoteMedia: (query) async {
      final read = desktop.media.readRange(
        handle: query.handle,
        workspaceRef: query.workspaceRef,
        deviceRef: query.desktopDeviceRef,
        now: DateTime.now(),
        start: query.start,
        endInclusive: query.endInclusive,
      );
      return switch (read) {
        RemoteMediaDenied() => const RemoteMediaSliceDenied(),
        RemoteMediaUnsatisfiable() => const RemoteMediaSliceUnsatisfiable(),
        RemoteMediaBytes(:final bytes, :final total, :final start) =>
          RemoteMediaSliceBody(bytes: bytes, total: total, start: start),
      };
    },
  );
  await gateway.start();
  final paired = PairedDesktopClient(
    origin: gateway.origin!,
    accessToken: () async => 'same-account',
    deviceRef: 'mobile-1',
    allowInsecureLoopback: true,
  );
  final connection = await paired.connectSameAccount(
    targetDeviceRef: 'desktop.1',
    workspaceRef: 'ws.opaque.1',
  );
  final http = HttpClient();
  final transport = PairedRemoteSurfaceTransport(
    origin: gateway.origin!,
    grantRef: connection.grantRef,
    httpClient: http,
  );
  final controller = RemoteWorkbenchController(
    transport: transport,
    context: const RemoteConnectionContext(
      actorRef: 'client-claimed-actor',
      mobileDeviceRef: 'client-claimed-mobile',
      desktopDeviceRef: 'client-claimed-desktop',
      workspaceRef: 'client-claimed-workspace',
      permissions: {},
    ),
    notice: acceptanceLabNotice,
    loadMedia: transport.readMedia,
  );
  final buddy = WorkBuddyController();
  addTearDown(buddy.dispose);
  addTearDown(controller.dispose);
  addTearDown(desktop.controller.dispose);
  addTearDown(() async {
    await tester.pumpWidget(const SizedBox.shrink());
    paired.close();
    http.close(force: true);
    await gateway.stop();
    await upstream.close(force: true);
  });

  await tester.pumpWidget(
    MaterialApp(
      theme: workBuddyTheme(),
      home: WorkBuddyShell(controller: buddy, remoteWorkbench: controller),
    ),
  );
  await tester.tap(find.byKey(const ValueKey('wb-menu')));
  await tester.pumpAndSettle();
  await tester.tap(find.byKey(const ValueKey('wb-remote-workbench')));
  await tester.pumpAndSettle();

  expect(find.text('社媒发布'), findsOneWidget);
  expect(find.text('视频编辑'), findsOneWidget);
  expect(find.text('网页快照'), findsOneWidget);
  expect(find.text('UNSUPPORTED_MODE'), findsOneWidget);

  await tester.tap(
    find.byKey(
      const ValueKey(
        'remote-surface-com.openmuse.fake-publish-publish-workflow',
      ),
    ),
  );
  await tester.pumpAndSettle();
  expect(find.text('封面'), findsOneWidget);
  expect(find.textContaining('已读取'), findsWidgets);
  expect(find.byType(Image), findsWidgets);

  await tester.tap(find.byKey(const ValueKey('remote-action-social.preview')));
  await tester.pumpAndSettle();
  expect(find.text('预览已生成：周末散步'), findsOneWidget);
  await tester.ensureVisible(
    find.byKey(const ValueKey('remote-action-social.publish.commit')),
  );
  await tester.tap(
    find.byKey(const ValueKey('remote-action-social.publish.commit')),
  );
  await tester.pumpAndSettle();
  expect(find.text('发布已提交'), findsOneWidget);
  expect(desktop.publish.externalExecutions, 1);
  expect(
    desktop.audit.entries.any(
      (entry) =>
          entry.actionId == 'social.publish.commit' &&
          entry.status == 'accepted' &&
          entry.actorRef == 'account-1' &&
          entry.workspaceRef == 'ws.opaque.1',
    ),
    isTrue,
  );
  expect(
    desktop.audit.entries.any(
      (entry) => entry.actorRef == 'client-claimed-actor',
    ),
    isFalse,
  );

  await tester.tap(find.byKey(const ValueKey('remote-surface-list')));
  await tester.pumpAndSettle();
  await tester.tap(
    find.byKey(
      const ValueKey('remote-surface-com.openmuse.fake-video-edit-timeline'),
    ),
  );
  await tester.pumpAndSettle();
  await tester.tap(find.byKey(const ValueKey('remote-action-clip.trim')));
  await tester.pumpAndSettle();
  expect(find.text('2 秒'), findsOneWidget);

  await tester.tap(find.byKey(const ValueKey('remote-surface-list')));
  await tester.pumpAndSettle();
  await tester.tap(
    find.byKey(
      const ValueKey(
        'remote-surface-com.openmuse.fake-web-snapshot-page-snapshot',
      ),
    ),
  );
  await tester.pumpAndSettle();
  expect(find.text('只读网页快照'), findsOneWidget);
  expect(find.text('虚拟来源 surface.openmuse.invalid'), findsOneWidget);
  expect(proxied, isEmpty);
}
