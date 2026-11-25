import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import '../../../plugins/siyuan-client/integration_test/resource_flow.dart';
import 'support/device_result.dart';
import 'support/launch_checkpoint.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  setUpAll(() async {
    runApp(const SizedBox.shrink());
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  testWidgets(
    'OpenMuse Notes creates, edits, saves and reopens in Mobile Home',
    (tester) async {
      tester.testTextInput.register();
      addTearDown(tester.testTextInput.unregister);
      final release = await loadInstalledRelease();
      final app = await mobile.bootOpenMuseMobile(
        release.regions.firstWhere((r) => r.available).bundle!,
        'en',
      );
      await tester.pumpWidget(app);
      // The test binding otherwise drops real device touches. WKWebView needs
      // a native touch to bring up the iOS software keyboard; synthetic Flutter
      // taps and JavaScript focus do not establish that user gesture.
      final previousDevicePointers = binding.shouldPropagateDevicePointerEvents;
      binding.shouldPropagateDevicePointerEvents = true;
      try {
        final result = await runSiYuanResourceFlow(
          tester,
          desktop: false,
          manualVisualCheckpoints: false,
          externalDocumentChecks: false,
          onVisualCheckpoint: (stage) =>
              launchCheckpoint(tester, binding, 'notes-$stage'),
        );
        expect(result['passed'], isTrue);
      } finally {
        binding.shouldPropagateDevicePointerEvents = previousDevicePointers;
      }
      // Keep the verified original editor and real IME visible for screenshot
      // review; the isolated acceptance process is stopped before the next run.
    },
    timeout: const Timeout(Duration(minutes: 7)),
  );
}
