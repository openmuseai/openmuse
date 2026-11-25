import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as application;
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'support/device_result.dart';

// Returning-user regression: use the real main/region/legal boundary and the
// existing Keystore session. No sign-in fixture or legal-consent writes.
void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  recordDeviceResult(binding);
  testWidgets(
    'normal China startup restores account and opens installed CI',
    (tester) async {
      final milestones = <String>[];
      final tools = <String>{};
      final report = <String, Object?>{'milestones': milestones};
      binding.reportData = report;
      final support = await getApplicationSupportDirectory();
      Future<void> until(
        bool Function() ready,
        String step, {
        int seconds = 90,
      }) async {
        final end = DateTime.now().add(Duration(seconds: seconds));
        while (!ready() && DateTime.now().isBefore(end)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump();
        }
        expect(ready(), isTrue, reason: step);
        milestones.add(step);
        debugPrint('CI_NORMAL_PASS $step');
      }

      await application.main();
      await tester.pump();
      await until(
        () => find
            .byType(application.OpenMuseMobileApplication)
            .evaluate()
            .isNotEmpty,
        'normal main and persisted region/legal boundary reached full Host',
      );
      final host = tester.widget<application.OpenMuseMobileApplication>(
        find.byType(application.OpenMuseMobileApplication),
      );
      expect(
        host.runtime!.client.current.serviceUri(ServiceId.cloud).host,
        'cn.openmuseai.com',
      );
      await until(
        () =>
            host.authenticationPlugin.authentication.snapshot.identity != null,
        'China account restored from Keystore without a sign-in fixture',
      );
      final app = host.remoteApps.single;
      app.onRequestCompleted = (method, name, agent) {
        if (name != null) tools.add(name);
      };
      await until(
        () =>
            find.text('Muse-CI').hitTestable().evaluate().isNotEmpty ||
            find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty,
        'normal homepage ready',
      );
      if (find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty) {
        await tester.pumpAndSettle(const Duration(milliseconds: 100));
        final top = tester
            .getTopLeft(find.byKey(const ValueKey('wb-bind-desktop')))
            .dy;
        expect(top, greaterThan(48));
        await tester.tapAt(Offset(12, top / 2));
        await tester.pumpAndSettle(const Duration(milliseconds: 100));
      }
      await until(
        () => find.text('Muse-CI').hitTestable().evaluate().isNotEmpty,
        'normal plugin entry is clickable',
      );
      await tester.tap(find.text('Muse-CI').hitTestable().first);
      await until(
        () =>
            find.byType(WebViewWidget).evaluate().isNotEmpty ||
            find.text('重试').evaluate().isNotEmpty,
        'normal plugin connection resolved',
      );
      if (find.text('重试').evaluate().isNotEmpty) {
        String category = 'unknown';
        await tester.runAsync(() async {
          try {
            await app.request('tools/list', {});
            category = 'transport-ready-after-view-failure';
          } catch (error) {
            category =
                error is StateError &&
                    error.message.toString().startsWith('APP_')
                ? error.message.toString()
                : error.runtimeType.toString();
          }
        });
        fail('Normal App connection rejected: $category');
      }
      await until(
        () =>
            tools.contains('ci_capabilities') &&
            tools.contains('ci_projects_list') &&
            tools.contains('ci_project_get'),
        'normal native CI UI queried live backend through MCP bridge',
      );
      report['uiTools'] = tools.toList();
      await tester.pumpAndSettle(const Duration(milliseconds: 100));
      await File('${support.path}/ci-e2e-stage').writeAsString('normal-ci');
      if (await File('${support.path}/ci-e2e-capture-stages').exists()) {
        final ack = File('${support.path}/ci-e2e-capture-normal-ci');
        if (await ack.exists()) await ack.delete();
        final end = DateTime.now().add(const Duration(seconds: 45));
        while (!await ack.exists() && DateTime.now().isBefore(end)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump();
        }
        expect(await ack.exists(), isTrue, reason: 'capture normal entry UI');
      }
      await File(
        '${support.path}/ci-normal-entry-e2e.json',
      ).writeAsString(jsonEncode(report), flush: true);
      app.onRequestCompleted = null;
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
