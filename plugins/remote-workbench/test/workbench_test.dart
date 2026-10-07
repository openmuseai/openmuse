import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';

const _context = RemoteConnectionContext(
  actorRef: 'actor.1',
  mobileDeviceRef: 'mobile.1',
  desktopDeviceRef: 'desktop.1',
  workspaceRef: 'ws.opaque.1',
  permissions: {'workspace.resource.read', 'social.content.publish'},
);

void main() {
  testWidgets('RS-UI-01 renders the declarative surface and previews it', (
    tester,
  ) async {
    final harness = _Harness();
    await tester.pumpWidget(harness.app());
    await tester.pumpAndSettle();

    expect(find.text('社媒发布'), findsOneWidget);
    expect(find.text('封面'), findsOneWidget);
    expect(find.text('12 秒'), findsOneWidget);
    expect(find.text('尚未预览'), findsOneWidget);
    expect(find.text('不支持的组件'), findsOneWidget);

    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.preview')),
    );
    await tester.pumpAndSettle();

    expect(find.text('预览已生成：周末散步'), findsOneWidget);
    expect(harness.publish.readExecutions, 1);
  });

  testWidgets('RS-UI-02 an optional unknown component does not block preview', (
    tester,
  ) async {
    final harness = _Harness();
    await tester.pumpWidget(harness.app());
    await tester.pumpAndSettle();

    expect(
      find.byKey(const ValueKey('remote-unsupported-extra')),
      findsOneWidget,
    );
    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.preview')),
    );
    await tester.pumpAndSettle();
    expect(find.text('预览已生成：周末散步'), findsOneWidget);
  });

  testWidgets('RS-UI-03 publish confirmation survives a dropped response', (
    tester,
  ) async {
    final harness = _Harness();
    await tester.pumpWidget(harness.app());
    await tester.pumpAndSettle();
    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.preview')),
    );
    await tester.pumpAndSettle();
    harness.transport.dropSubmitResponse = true;

    await tester.ensureVisible(
      find.byKey(const ValueKey('remote-action-social.publish.commit')),
    );
    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.publish.commit')),
    );
    await tester.pumpAndSettle();

    expect(harness.transport.submits, 2);
    expect(harness.publish.externalExecutions, 1);
    expect(find.text('发布已提交'), findsOneWidget);
    expect(find.text('任务状态 running'), findsOneWidget);
  });
}

final class _Harness {
  _Harness() {
    host.register(publish);
    transport = RemoteSurfaceFaultTransport(
      InMemoryRemoteSurfaceTransport(host),
    );
    controller = RemoteWorkbenchController(
      transport: transport,
      context: _context,
    );
  }

  final host = RemoteSurfaceHost();
  final publish = FakePublishSurfaceProvider();
  late final RemoteSurfaceFaultTransport transport;
  late final RemoteWorkbenchController controller;

  Widget app() =>
      MaterialApp(home: RemoteWorkbenchPage(controller: controller));
}
