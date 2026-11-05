import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_pty/flutter_pty.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'Pty.startAsync keeps UI event-loop stalls below budget',
    (tester) async {
      final iterations = int.tryParse(
            Platform.environment['MUSE_PERF_ITERATIONS'] ?? '',
          ) ??
          5;
      expect(iterations, greaterThan(0));
      await Pty.prewarmAsync();
      final clock = Stopwatch()..start();
      var lastTick = clock.elapsedMicroseconds;
      var maxGapMicros = 0;
      final heartbeat = Timer.periodic(const Duration(milliseconds: 5), (_) {
        final now = clock.elapsedMicroseconds;
        final gap = now - lastTick;
        if (gap > maxGapMicros) maxGapMicros = gap;
        lastTick = now;
      });

      for (var index = 0; index < iterations; index++) {
        final Pty pty;
        if (Platform.isWindows) {
          // Intentionally shell-free: quoting and CreateProcessW are exercised
          // by the plugin rather than hidden behind PowerShell.
          pty = await Pty.startAsync(
            'cmd.exe',
            arguments: [
              '/d',
              '/s',
              '/c',
              'echo __MUSE_ASYNC_PTY_$index',
            ],
          );
        } else {
          pty = await Pty.startAsync(
            '/bin/sh',
            arguments: ['-c', 'printf __MUSE_ASYNC_PTY_$index'],
          );
        }
        final output = await utf8.decoder
            .bind(pty.output.cast<List<int>>())
            .join()
            .timeout(const Duration(seconds: 10));
        expect(await pty.exitCode, 0);
        expect(output, contains('__MUSE_ASYNC_PTY_$index'));
      }

      heartbeat.cancel();
      // ignore: avoid_print
      print(
        'PTY_ASYNC iterations=$iterations total=${clock.elapsedMilliseconds}ms '
        'maxUiGap=${(maxGapMicros / 1000).toStringAsFixed(1)}ms',
      );
      // The broker's contract is about UI-isolate occupancy, not child process
      // wall time. Allow normal CI jitter while rejecting the old 360-1000 ms
      // synchronous CreateProcessW stall.
      expect(
        maxGapMicros / 1000,
        lessThan(100),
        reason: 'maximum UI event-loop gap must remain below 100 ms',
      );
    },
    timeout: const Timeout(Duration(seconds: 45)),
  );
}
