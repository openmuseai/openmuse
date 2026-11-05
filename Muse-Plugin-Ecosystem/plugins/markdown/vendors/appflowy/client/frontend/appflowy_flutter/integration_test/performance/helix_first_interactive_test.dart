import 'dart:io';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/helix_resource_surface.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/material.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'Helix first-interactive p95 remains inside the desktop budget',
    (tester) async {
      await getIt.reset();
      final temp = await Directory.systemTemp.createTemp('muse-helix-ttfi-');
      final installerRoot = Directory('${temp.path}/language-servers');
      await installerRoot.create(recursive: true);
      final installer = HelixLanguageServerInstaller(root: installerRoot);
      final settings = HelixSettingsController(installer)..loaded = true;
      getIt.registerSingleton<HelixSettingsController>(settings);
      await Future.wait([
        Pty.prewarmAsync(),
        HelixInstall.resolve(),
      ]);
      final iterations = int.tryParse(
            Platform.environment['MUSE_PERF_ITERATIONS'] ?? '',
          ) ??
          5;
      expect(iterations, greaterThan(0));

      final durations = <int>[];
      try {
        for (var index = 0; index < iterations; index++) {
          final file = File('${temp.path}/sample-$index.md');
          await file.writeAsString(
            '# First interactive sample $index\n\n'
            'The file content must reach the terminal model before completion.\n',
          );
          final sink = MuseRingBufferPerformanceSink();
          final trace = MusePerformanceTracer(sink: sink).startTrace(
            'resource.open',
            category: 'resource',
            attributes: const {'origin': 'performance-test'},
          );
          await tester.pumpWidget(
            MaterialApp(
              home: SizedBox.expand(
                child: HelixResourceSurface(
                  file: file,
                  performanceTrace: trace,
                ),
              ),
            ),
          );

          final deadline = Stopwatch()..start();
          while (!trace.isFinished &&
              deadline.elapsed < const Duration(seconds: 10)) {
            await tester.pump(const Duration(milliseconds: 16));
            await Future<void>.delayed(const Duration(milliseconds: 8));
          }
          expect(trace.isFinished, isTrue, reason: 'Helix never reached TTFI');
          final traceEnd = sink.snapshot().lastWhere(
                (event) => event.kind == MusePerformanceEventKind.traceEnd,
              );
          final duration = ((traceEnd.durationMicros ?? 0) / 1000).ceil();
          durations.add(duration);
          final stages = {
            for (final event in sink.snapshot())
              if (event.kind == MusePerformanceEventKind.spanEnd)
                event.name: ((event.durationMicros ?? 0) / 1000).ceil(),
          };
          // Kept in the performance test output so a budget regression points
          // directly to routing, configuration, PTY creation, or rendering.
          // ignore: avoid_print
          print('HELIX_TTFI sample=$index total=${duration}ms stages=$stages');

          final firstFrame = sink.snapshot().where(
                (event) =>
                    event.name == 'engine.first-interactive-frame' &&
                    event.kind == MusePerformanceEventKind.spanEnd,
              );
          expect(firstFrame, hasLength(1));
          await tester.pumpWidget(const SizedBox.shrink());
          await tester.pump(const Duration(milliseconds: 250));
        }

        durations.sort();
        final p95 = durations[(durations.length * .95).ceil() - 1];
        final configured = int.tryParse(
          Platform.environment['MUSE_HELIX_TTFI_P95_MS'] ?? '',
        );
        final budget = configured ?? (Platform.isWindows ? 1500 : 1000);
        // ignore: avoid_print
        print(
          'HELIX_TTFI_SUMMARY iterations=$iterations samples=$durations '
          'p95=${p95}ms budget=${budget}ms',
        );
        expect(
          p95,
          lessThan(budget),
          reason: 'Helix TTFI samples=$durations, p95 budget=${budget}ms',
        );
      } finally {
        await tester.pumpWidget(const SizedBox.shrink());
        await getIt.reset();
        await temp.delete(recursive: true);
      }
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
