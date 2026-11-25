import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_resource_clipboard/openmuse_resource_clipboard.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'support/device_result.dart';
import 'support/launch_checkpoint.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  testWidgets(
    'restored onboarding stays hidden and resource paste requires the text menu',
    (tester) async {
      final release = await loadInstalledRelease();
      final region = release.regions.firstWhere((r) => r.available);
      // Isolated test preference fixture: no production consent or account is changed.
      final store = MemoryConfigStore();
      await RegionSelection(release, store).acceptLegal(region);
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          loadRelease: () async => release,
          store: (_) async => store,
          boot: (bundle, _) => mobile.bootOpenMuseMobile(bundle, 'en'),
        ),
      );
      final home = find.byKey(const ValueKey('wb-tab-Home'));
      for (var i = 0; i < 600; i++) {
        expect(find.byKey(const ValueKey('region-continue')), findsNothing);
        expect(find.byKey(const ValueKey('legal-checkbox')), findsNothing);
        if (home.evaluate().isNotEmpty) break;
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump(const Duration(milliseconds: 100));
      }
      expect(home, findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('wb-tab-Tasks')));
      await tester.pumpAndSettle();
      expect(find.text('Paste resource'), findsNothing);
      await launchCheckpoint(tester, binding, 'tasks-without-permanent-paste');
      final center = tester
          .widget<WorkBuddyShell>(find.byType(WorkBuddyShell))
          .resources!;
      final record = await tester.runAsync(
        () => center.documents.create(
          parentRef: 'root',
          name: 'Clipboard practice.md',
          mediaType: 'text/markdown',
          bytes: '# Public clipboard practice\n'.codeUnits,
          idempotencyKey: 'launch-polish-clipboard-fixture',
          access: center.hub.access('acceptance'),
        ),
      );
      await tester.runAsync(
        () => ResourceClipboard.write(record!.reference(pin: true)),
      );
      await tester.longPress(find.byKey(const ValueKey('wb-composer')));
      await tester.pumpAndSettle();
      expect(find.text('Paste resource'), findsOneWidget);
      await launchCheckpoint(tester, binding, 'tasks-resource-paste-menu');
      await tester.runAsync(() async {
        await tester.tap(find.text('Paste resource'));
        await Future<void>.delayed(const Duration(milliseconds: 200));
      });
      await tester.pumpAndSettle();
      expect(center.draft.items.single.record.identity, record!.identity);
      expect(find.text('Paste resource'), findsNothing);
      center.draft.clear();
      tester
          .widget<TextField>(find.byKey(const ValueKey('wb-composer')))
          .focusNode!
          .unfocus();
      await tester.pumpAndSettle();
      binding.reportData = {
        'milestones': {
          'restoredOnboardingNeverPresented': true,
          'noPermanentPasteAction': true,
          'longPressShowsResourcePaste': true,
          'explicitPasteAttachesReference': true,
        },
      };
    },
    timeout: const Timeout(Duration(minutes: 4)),
  );
}
