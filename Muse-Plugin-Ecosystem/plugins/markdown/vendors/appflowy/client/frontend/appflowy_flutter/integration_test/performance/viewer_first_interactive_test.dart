import 'dart:io';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/open_file_viewer_resource_surface.dart';
import 'package:appflowy/plugins/resource_surface/viewer/open_file_viewer_runtime_broker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'Windows Viewer text first-interactive p95 stays below 500 ms',
    (tester) async {
      if (!Platform.isWindows) {
        markTestSkipped('WebView2 performance gate requires Windows');
        return;
      }

      final temp = await Directory.systemTemp.createTemp('muse-viewer-ttfi-');
      final durations = <int>[];
      try {
        await OpenFileViewerRuntimeBroker.instance.warmUp();
        final iterations = int.tryParse(
              Platform.environment['MUSE_PERF_ITERATIONS'] ?? '',
            ) ??
            5;
        expect(iterations, greaterThan(0));
        for (var index = 0; index < iterations; index++) {
          final file = File('${temp.path}/viewer-sample-$index.txt');
          await file.writeAsString(
            List.filled(200, 'Viewer first interactive sample $index')
                .join('\n'),
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
                child: OpenFileViewerResourceSurface(
                  file: file,
                  performanceTrace: trace,
                ),
              ),
            ),
          );

          final deadline = Stopwatch()..start();
          while (!trace.isFinished &&
              deadline.elapsed < const Duration(seconds: 15)) {
            await tester.pump(const Duration(milliseconds: 16));
            await Future<void>.delayed(const Duration(milliseconds: 8));
          }
          expect(trace.isFinished, isTrue, reason: 'Viewer never reached TTFI');
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
          // ignore: avoid_print
          print('VIEWER_TTFI sample=$index total=${duration}ms stages=$stages');

          await tester.pumpWidget(const SizedBox.shrink());
          await tester.pump(const Duration(milliseconds: 150));
          await OpenFileViewerRuntimeBroker.instance.warmUp();
        }

        durations.sort();
        final p95 = durations[(durations.length * .95).ceil() - 1];
        final budget = int.tryParse(
              Platform.environment['MUSE_VIEWER_TTFI_P95_MS'] ?? '',
            ) ??
            500;
        // ignore: avoid_print
        print(
          'VIEWER_TTFI_SUMMARY iterations=$iterations samples=$durations '
          'p95=${p95}ms budget=${budget}ms',
        );
        expect(
          p95,
          lessThan(budget),
          reason: 'Viewer TTFI samples=$durations, p95 budget=${budget}ms',
        );
      } finally {
        await tester.pumpWidget(const SizedBox.shrink());
        OpenFileViewerRuntimeBroker.instance.dispose();
        await temp.delete(recursive: true);
      }
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
