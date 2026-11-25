import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:path_provider/path_provider.dart';

import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  SemanticsHandle? semantics;
  setUpAll(() async {
    // Match the other native suites: iOS enables an engine-owned semantics
    // handle asynchronously, before testWidgets captures its leak baseline.
    semantics = binding.ensureSemantics();
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  tearDownAll(() => semantics?.dispose());
  testWidgets(
    'Tasks stays bottom aligned through tab and real keyboard changes',
    (tester) async {
      Future<T> io<T>(Future<T> Function() action) async {
        late T result;
        await tester.runAsync(() async {
          result = await action();
        });
        return result;
      }

      Future<void> tick() async {
        await tester.pump(const Duration(milliseconds: 100));
        await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
      }

      Future<void> waitFor(bool Function() condition) async {
        for (var i = 0; i < 180; i++) {
          if (condition()) return;
          await tick();
        }
        fail('Tasks layout condition timed out');
      }

      Future<void> settleKeyboard() async {
        var previous = tester.view.viewInsets.bottom;
        var stable = 0;
        for (var i = 0; i < 30; i++) {
          await tick();
          final current = tester.view.viewInsets.bottom;
          stable = (current - previous).abs() < 0.01 ? stable + 1 : 0;
          previous = current;
          if (stable >= 3) return;
        }
        fail('Platform keyboard animation did not settle');
      }

      final release = await io(loadInstalledRelease);
      final app = await io(
        () => mobile.bootOpenMuseMobile(
          release.regions.firstWhere((region) => region.available).bundle!,
          'en',
        ),
      );
      await tester.pumpWidget(app);
      final tasks = find.byKey(const ValueKey('wb-tab-Tasks'));
      final home = find.byKey(const ValueKey('wb-tab-Home'));
      await waitFor(() => tasks.evaluate().isNotEmpty);
      final support = await io(getApplicationSupportDirectory);
      final visual = await io(
        () => File('${support.path}/tasks-layout-visual').exists(),
      );
      final measurements = <String, Object?>{};
      Future<void> checkpoint(String phase) async {
        await tick();
        final ratio = tester.view.devicePixelRatio;
        final keyboard = tester.view.viewInsets.bottom / ratio;
        final safe = tester.view.padding.bottom / ratio;
        final bottom = tester.getBottomLeft(tasks).dy;
        final gap =
            tester.view.physicalSize.height / ratio - keyboard - safe - bottom;
        expect(
          gap,
          inInclusiveRange(0, 12),
          reason: '$phase tabs must meet the visible safe-area bottom',
        );
        final panel = find
            .ancestor(
              of: find.byKey(const ValueKey('wb-composer')),
              matching: find.byType(DecoratedBox),
            )
            .first;
        final composerGap =
            tester.getTopLeft(tasks).dy - tester.getBottomLeft(panel).dy;
        expect(composerGap, inInclusiveRange(0, 32));
        expect(tester.takeException(), isNull);
        measurements[phase] = {
          'bottomGap': gap,
          'composerGap': composerGap,
          'imeInset': keyboard,
          'safeBottom': safe,
        };
        await io(
          () => File('${support.path}/tasks-layout-ready').writeAsString(phase),
        );
        if (visual) {
          final done = File('${support.path}/tasks-layout-release-$phase');
          final deadline = DateTime.now().add(const Duration(seconds: 60));
          while (!await io(done.exists) && DateTime.now().isBefore(deadline)) {
            await tick();
          }
        }
      }

      await tester.tap(home);
      await tick();
      final homeBottom = tester.getBottomLeft(home).dy;
      await tester.tap(tasks);
      await tester.pumpAndSettle();
      expect(tester.getBottomLeft(tasks).dy, closeTo(homeBottom, 1));
      await checkpoint('initial');
      tester
          .widget<TextField>(find.byKey(const ValueKey('wb-composer')))
          .focusNode!
          .requestFocus();
      await tick();
      unawaited(SystemChannels.textInput.invokeMethod<void>('TextInput.show'));
      await waitFor(
        () =>
            tester.view.viewInsets.bottom > 100 * tester.view.devicePixelRatio,
      );
      await settleKeyboard();
      await checkpoint('keyboard');
      FocusManager.instance.primaryFocus?.unfocus();
      unawaited(SystemChannels.textInput.invokeMethod<void>('TextInput.hide'));
      await waitFor(() => tester.view.viewInsets.bottom == 0);
      await settleKeyboard();
      await tester.tap(home);
      await tick();
      await tester.tap(tasks);
      await tester.pumpAndSettle();
      await checkpoint('closed');
      await io(
        () => File('${support.path}/tasks-bottom-layout.json').writeAsString(
          jsonEncode({
            'passed': true,
            'platform': Platform.operatingSystem,
            'measurements': measurements,
          }),
          flush: true,
        ),
      );
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
