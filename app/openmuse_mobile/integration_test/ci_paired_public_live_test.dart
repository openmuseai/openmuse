import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:openmuse_mobile/main.dart' as application;
import 'package:openmuse_mcp_app_runtime/openmuse_mcp_app_runtime.dart';
import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  recordDeviceResult(binding);
  testWidgets(
    'foldable Apps tab controls CI using only the domestic public paired relay',
    (tester) async {
      final milestones = <String>[];
      binding.reportData = {
        'milestones': milestones,
        'transport': 'domestic-public-paired-desktop',
        'publicAcceptancePassed': false,
      };
      final support = await getApplicationSupportDirectory(),
          file = File('${support.path}/ci-e2e-account.json');
      final config = jsonDecode(await file.readAsString()) as Map;
      await file.delete();
      Future<void> until(
        bool Function() check,
        String label, {
        int seconds = 90,
      }) async {
        final end = DateTime.now().add(Duration(seconds: seconds));
        while (!check() && DateTime.now().isBefore(end)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump();
        }
        expect(check(), isTrue, reason: label);
        milestones.add(label);
        debugPrint('CI_PUBLIC_PASS $label');
      }

      await application.main();
      await tester.pump();
      await until(
        () => find
            .byType(application.OpenMuseMobileApplication)
            .evaluate()
            .isNotEmpty,
        'ordinary Mobile application started',
      );
      final host = tester.widget<application.OpenMuseMobileApplication>(
        find.byType(application.OpenMuseMobileApplication),
      );
      expect(
        host.remoteApps,
        isEmpty,
        reason:
            'A direct loopback developer installation must not satisfy this test',
      );
      await tester.runAsync(
        () => host.authenticationPlugin.authentication.signInWithPassword(
          config['email'] as String,
          config['password'] as String,
        ),
      );
      await until(
        () =>
            host.authenticationPlugin.authentication.snapshot.identity != null,
        'China production account authenticated with exact password',
      );
      final paired = host.pairedDesktopPlugin;
      await tester.runAsync(() => paired.directory!.refresh());
      await until(
        () => paired.controller.devices.any(
          (d) => d.deviceRef == config['desktopDeviceRef'],
        ),
        'same-account Desktop discovered through production directory',
      );
      final device = paired.controller.devices.singleWhere(
        (d) => d.deviceRef == config['desktopDeviceRef'],
      );
      expect(
        await tester.runAsync(() => paired.controller.connectDevice(device)),
        isTrue,
      );
      final connection = paired.controller.snapshot.connection!;
      final origin = Uri.parse(connection.session.origin);
      expect(origin.scheme, 'https');
      expect(origin.host, 'cn.openmuseai.com');
      milestones.add('paired grant uses domestic HTTPS production relay');
      final state = tester.state<application.OpenMuseMobileApplicationState>(
        find.byType(application.OpenMuseMobileApplication),
      );
      await until(
        () => state.installedPluginIds.contains('io.openmuse.muse-ci'),
        'Desktop-delegated CI installed into ordinary Mobile plugin registry',
      );
      // Dismiss the first sign-in pairing sheet if it remains above the shell.
      final barrier = find.byType(ModalBarrier).hitTestable();
      if (barrier.evaluate().isNotEmpty) {
        await tester.tapAt(const Offset(5, 80));
        await tester.pump();
      }
      final apps = find.byKey(const ValueKey('wb-tab-应用')).evaluate().isNotEmpty
          ? find.byKey(const ValueKey('wb-tab-应用'))
          : find.byKey(const ValueKey('wb-tab-Apps'));
      await tester.tap(apps);
      await tester.pump();
      final card = find.byKey(
        const ValueKey('workbench-card-io.openmuse.muse-ci'),
      );
      await tester.ensureVisible(card);
      await tester.tap(card);
      await tester.pump();
      await until(
        () => find.byType(RemoteMcpAppView).evaluate().isNotEmpty,
        'Apps tab card opened full CI plugin',
      );
      final view = tester.widget<RemoteMcpAppView>(
        find.byType(RemoteMcpAppView),
      );
      expect(view.app.forwardRequest, isNotNull);
      var networkReads = 0;
      view.app.onRequestCompleted = (method, tool, agent) {
        networkReads++;
      };
      await until(
        () => find.byType(WebViewWidget).evaluate().isNotEmpty,
        'paired MCP UI connected without Mobile backend credential',
      );
      final web = WebViewController.fromPlatform(
        tester
            .widget<WebViewWidget>(find.byType(WebViewWidget))
            .platform
            .params
            .controller,
      );
      Future<bool> has(String text) async {
        final r = await web.runJavaScriptReturningResult(
          'document.body.innerText.includes(${jsonEncode(text)})',
        );
        return r == true || r == 'true';
      }

      bool ready = false;
      for (var i = 0; i < 90 && !ready; i++) {
        ready = await tester.runAsync(() => has('已连接')) ?? false;
        if (!ready) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 300)),
          );
          await tester.pump();
        }
      }
      expect(ready, isTrue);
      milestones.add('actual CI MCP canvas connected over public relay');
      binding.reportData!['logicalWidth'] =
          tester.view.physicalSize.width / tester.view.devicePixelRatio;
      binding.reportData!['networkReads'] = networkReads;
      binding.reportData!['entryAcceptancePassed'] = true;
      // Full node/artifact/Diff acceptance is deliberately separate from entry.
      // Do not mark publicAcceptancePassed until all those actions are recorded.
    },
  );
}
