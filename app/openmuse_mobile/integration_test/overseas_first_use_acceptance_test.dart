import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'support/device_result.dart';
import 'support/launch_checkpoint.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  testWidgets(
    'first use requires explicit terms acceptance before any business boot',
    (tester) async {
      var boots = 0;
      final store = MemoryConfigStore();
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          store: (_) async => store,
          boot: (_, _) async {
            boots++;
            return const MaterialApp(home: Text('Consent completed'));
          },
        ),
      );
      await tester.pumpAndSettle();
      if (find.byKey(const ValueKey('region-us')).evaluate().isNotEmpty) {
        await tester.tap(find.byKey(const ValueKey('region-us')));
        await tester.pump();
        await tester.tap(find.byKey(const ValueKey('region-continue')));
        await tester.pumpAndSettle();
      }
      expect(boots, 0);
      expect(find.text('Privacy Policy'), findsOneWidget);
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('legal-accept')))
            .onPressed,
        isNull,
      );
      await launchCheckpoint(tester, binding, 'legal-first-use');
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-decline')));
      await tester.pump();
      expect(boots, 0);
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-accept')));
      await tester.pumpAndSettle();
      expect(boots, 1);
      expect(await store.read(), contains('legalConsent'));
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );
}
