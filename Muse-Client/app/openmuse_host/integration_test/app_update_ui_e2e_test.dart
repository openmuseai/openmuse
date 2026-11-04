import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_host/src/host/desktop_update.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:pub_semver/pub_semver.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('download stays in sidebar and workbench remains usable', (
    tester,
  ) async {
    final root = await Directory.systemTemp.createTemp('openmuse-update-ui-');
    final bytes = List<int>.generate(65536, (index) => index % 251);
    final releaseSecondHalf = Completer<void>();
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    server.listen((request) async {
      request.response.contentLength = bytes.length;
      request.response.add(bytes.sublist(0, bytes.length ~/ 2));
      await request.response.flush();
      await releaseSecondHalf.future;
      request.response.add(bytes.sublist(bytes.length ~/ 2));
      await request.response.close();
    });
    final feed = OpenMuseUpdateFeed(
      origin: Uri.parse('http://127.0.0.1:${server.port}'),
      channel: 'canary',
      target: 'macos-universal',
      allowInsecureLoopback: true,
      fetchOverride: (_) async => jsonEncode({
        'schema': 'openmuse.app-update/v1',
        'channel': 'canary',
        'target': 'macos-universal',
        'version': '1.0.6',
        'build': 7,
        'publishedAt': '2026-10-13T00:00:00Z',
        'downloadUrl': 'http://127.0.0.1:${server.port}/downloads/update.zip',
        'sha256': sha256.convert(bytes).toString(),
        'sizeBytes': bytes.length,
        'installKind': 'macos-zip',
        'notes': {'zh': '后台下载验收', 'en': 'Background download test'},
      }),
    );
    final updates = OpenMuseUpdateController(
      feed: feed,
      installedVersion: Version.parse('1.0.5'),
      installedBuild: 6,
    );
    await updates.check();
    expect(updates.available, isNotNull);
    final desktopUpdate = OpenMuseDesktopUpdate(stagingDirectory: root);
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final workspace = LocalWorkspaceController(
      rootPath: root.path,
      initialResources: const [],
    );
    final settings = OpenMuseLocalSettings(
      file: File('${root.path}/settings.json'),
    );
    await settings.load();
    try {
      await tester.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          settings: settings,
          updates: updates,
          desktopUpdate: desktopUpdate,
        ),
      );
      await tester.pumpAndSettle();
      await tester.pump(const Duration(milliseconds: 100));
      expect(find.byKey(const ValueKey('app-update-dialog')), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('app-update-later')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('app-update-button')), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('app-update-button')));
      await tester.pump();
      for (var i = 0; i < 50; i++) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
        if (find.text('50%').evaluate().isNotEmpty) break;
      }
      expect(find.byKey(const ValueKey('app-update-progress')), findsOneWidget);
      expect(find.text('50%'), findsOneWidget);
      expect(
        find.byKey(const ValueKey('app-update-download-dialog')),
        findsNothing,
      );
      expect(find.textContaining('Project Workspace'), findsWidgets);
      releaseSecondHalf.complete();
      for (var i = 0; i < 50; i++) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
        if (find
            .byKey(const ValueKey('app-update-restart-dialog'))
            .evaluate()
            .isNotEmpty) {
          break;
        }
      }
      expect(
        find.byKey(const ValueKey('app-update-restart-dialog')),
        findsOneWidget,
      );
      await tester.tap(find.byKey(const ValueKey('app-update-restart-later')));
      await tester.pump();
      expect(
        find.byKey(const ValueKey('app-update-restart-button')),
        findsOneWidget,
      );
      expect(
        find.byKey(const ValueKey('notification-unread-dot')),
        findsOneWidget,
      );
      await tester.tap(find.byKey(const ValueKey('notifications-footer')));
      await tester.pumpAndSettle();
      expect(
        find.byKey(const ValueKey('notifications-dialog')),
        findsOneWidget,
      );
      expect(
        find.byKey(const ValueKey('notification-update-action')),
        findsOneWidget,
      );
      expect(
        find.byKey(const ValueKey('notification-unread-dot')),
        findsNothing,
      );
      await tester.tap(find.text('关闭'));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('app-update-dismiss')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('app-update-footer')), findsNothing);
      expect(
        find.byKey(const ValueKey('notifications-footer')),
        findsOneWidget,
      );
      await tester.tap(find.byKey(const ValueKey('notifications-footer')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('notification-update-action')));
      await tester.pumpAndSettle();
      expect(
        find.byKey(const ValueKey('app-update-restart-dialog')),
        findsOneWidget,
      );
      await tester.tap(find.byKey(const ValueKey('app-update-restart-later')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('app-update-footer')), findsNothing);
    } finally {
      if (!releaseSecondHalf.isCompleted) releaseSecondHalf.complete();
      await tester.pumpWidget(const SizedBox.shrink());
      desktopUpdate.dispose();
      settings.dispose();
      workspace.dispose();
      await server.close(force: true);
      await root.delete(recursive: true);
    }
  });
}
