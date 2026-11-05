import 'dart:convert';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('records a trace, nested span and milestone with monotonic durations',
      () async {
    var now = 100;
    final sink = MuseRingBufferPerformanceSink();
    final tracer = MusePerformanceTracer(sink: sink, clock: () => now);
    final trace = tracer.startTrace(
      'resource.open',
      category: 'resource',
      attributes: const {'origin': 'workspace'},
    );
    now += 20;
    final span = trace.startSpan('resource.resolve');
    now += 30;
    span.end();
    now += 10;
    trace.mark('tab.visible');
    now += 40;
    trace.finish(attributes: const {'engine': 'helix'});

    final events = sink.snapshot();
    expect(events.map((event) => event.kind), [
      MusePerformanceEventKind.traceStart,
      MusePerformanceEventKind.spanStart,
      MusePerformanceEventKind.spanEnd,
      MusePerformanceEventKind.mark,
      MusePerformanceEventKind.traceEnd,
    ]);
    expect(events[2].durationMicros, 30);
    expect(events.last.durationMicros, 100);
    expect(events.last.attributes['engine'], 'helix');
    expect(trace.isFinished, isTrue);
  });

  test('ring buffer is bounded and exports Perfetto compatible events', () {
    var now = 0;
    final sink = MuseRingBufferPerformanceSink(capacity: 2);
    final tracer = MusePerformanceTracer(sink: sink, clock: () => now++);
    final trace = tracer.startTrace('open');
    trace.mark('one');
    trace.finish();

    expect(sink.snapshot(), hasLength(2));
    final exported = jsonDecode(sink.exportChromeTraceJson()) as Map;
    final events = exported['traceEvents'] as List;
    expect(events, hasLength(2));
    expect(events.last['ph'], 'e');
  });

  test('measure records failures without finishing the parent trace', () async {
    final sink = MuseRingBufferPerformanceSink();
    final trace = MusePerformanceTracer(sink: sink).startTrace('open');

    await expectLater(
      trace.measure<void>('engine.start', () async => throw StateError('bad')),
      throwsStateError,
    );

    expect(trace.isFinished, isFalse);
    expect(sink.snapshot().last.status, 'error');
    trace.fail(StateError('bad'));
    expect(sink.snapshot().last.kind, MusePerformanceEventKind.traceEnd);
  });

  test('a failing diagnostics sink never breaks the observed operation', () {
    final surviving = MuseRingBufferPerformanceSink();
    final tracer = MusePerformanceTracer(
      sink: MuseCompositePerformanceSink([
        MuseCallbackPerformanceSink((_) => throw StateError('sink failed')),
        surviving,
      ]),
    );

    final trace = tracer.startTrace('open');
    expect(trace.measureSync('work', () => 42), 42);
    trace.finish();

    expect(surviving.snapshot(), hasLength(4));
  });
}
