import 'dart:collection';
import 'dart:convert';

/// Monotonic, engine-agnostic performance tracing for user-visible operations.
///
/// The tracer intentionally has no Flutter, logging, resource, or editor
/// dependency. Product features provide a [MusePerformanceSink] and propagate a
/// [MusePerformanceTrace] explicitly across async and engine boundaries.
typedef MusePerformanceClock = int Function();

enum MusePerformanceEventKind { traceStart, spanStart, mark, spanEnd, traceEnd }

final class MusePerformanceEvent {
  const MusePerformanceEvent({
    required this.traceId,
    required this.spanId,
    required this.name,
    required this.category,
    required this.kind,
    required this.timestampMicros,
    required this.attributes,
    this.parentSpanId,
    this.durationMicros,
    this.status,
  });

  final String traceId;
  final String spanId;
  final String? parentSpanId;
  final String name;
  final String category;
  final MusePerformanceEventKind kind;
  final int timestampMicros;
  final int? durationMicros;
  final String? status;
  final Map<String, Object?> attributes;

  Map<String, Object?> toJson() => {
        'traceId': traceId,
        'spanId': spanId,
        if (parentSpanId != null) 'parentSpanId': parentSpanId,
        'name': name,
        'category': category,
        'kind': kind.name,
        'timestampMicros': timestampMicros,
        if (durationMicros != null) 'durationMicros': durationMicros,
        if (status != null) 'status': status,
        if (attributes.isNotEmpty) 'attributes': attributes,
      };

  /// Chrome/Perfetto trace-event representation. Async operations use nestable
  /// async events, so spans remain correct when Dart awaits on another task.
  Map<String, Object?> toChromeTraceEvent() {
    final phase = switch (kind) {
      MusePerformanceEventKind.traceStart ||
      MusePerformanceEventKind.spanStart =>
        'b',
      MusePerformanceEventKind.spanEnd ||
      MusePerformanceEventKind.traceEnd =>
        'e',
      MusePerformanceEventKind.mark => 'n',
    };
    return {
      'name': name,
      'cat': category,
      'ph': phase,
      'ts': timestampMicros,
      'pid': 1,
      'tid': 1,
      'id': spanId,
      'args': {
        'traceId': traceId,
        if (parentSpanId != null) 'parentSpanId': parentSpanId,
        if (durationMicros != null) 'durationMicros': durationMicros,
        if (status != null) 'status': status,
        ...attributes,
      },
    };
  }
}

abstract interface class MusePerformanceSink {
  void record(MusePerformanceEvent event);
}

/// Bounded in-memory trace storage suitable for diagnostics and test export.
final class MuseRingBufferPerformanceSink implements MusePerformanceSink {
  MuseRingBufferPerformanceSink({this.capacity = 4096}) : assert(capacity > 0);

  final int capacity;
  final Queue<MusePerformanceEvent> _events = Queue<MusePerformanceEvent>();

  @override
  void record(MusePerformanceEvent event) {
    if (_events.length == capacity) _events.removeFirst();
    _events.addLast(event);
  }

  List<MusePerformanceEvent> snapshot() => List.unmodifiable(_events);

  String exportJson() => jsonEncode([
        for (final event in _events) event.toJson(),
      ]);

  String exportChromeTraceJson() => jsonEncode({
        'traceEvents': [
          for (final event in _events) event.toChromeTraceEvent(),
        ],
      });

  void clear() => _events.clear();
}

final class MuseCallbackPerformanceSink implements MusePerformanceSink {
  const MuseCallbackPerformanceSink(this.callback);

  final void Function(MusePerformanceEvent event) callback;

  @override
  void record(MusePerformanceEvent event) => callback(event);
}

final class MuseCompositePerformanceSink implements MusePerformanceSink {
  const MuseCompositePerformanceSink(this.sinks);

  final List<MusePerformanceSink> sinks;

  @override
  void record(MusePerformanceEvent event) {
    for (final sink in sinks) {
      try {
        sink.record(event);
      } on Object {
        // Observability is never allowed to break the observed operation. A
        // composite still gives the remaining sinks a chance to record it.
      }
    }
  }
}

final class MusePerformanceTracer {
  MusePerformanceTracer({
    MusePerformanceSink? sink,
    MusePerformanceClock? clock,
  })  : sink = sink ?? MuseRingBufferPerformanceSink(),
        _clock = clock ?? _defaultClock {
    _originMicros = _clock();
  }

  static final Stopwatch _stopwatch = Stopwatch()..start();
  static int _defaultClock() => _stopwatch.elapsedMicroseconds;

  /// Process-wide bounded diagnostics buffer. A diagnostics UI or an E2E test
  /// can export this without depending on a concrete engine.
  static final MuseRingBufferPerformanceSink defaultBuffer =
      MuseRingBufferPerformanceSink();

  /// Process-wide default. Tests and subsystems may inject a separate tracer.
  static final MusePerformanceTracer instance = MusePerformanceTracer(
    sink: defaultBuffer,
  );

  static String exportDefaultChromeTraceJson() =>
      defaultBuffer.exportChromeTraceJson();

  final MusePerformanceSink sink;
  final MusePerformanceClock _clock;
  late final int _originMicros;
  int _nextId = 0;

  MusePerformanceTrace startTrace(
    String name, {
    String category = 'application',
    Map<String, Object?> attributes = const {},
  }) {
    final traceId = _id('trace');
    final rootSpanId = _id('span');
    final now = _now();
    _record(
      MusePerformanceEvent(
        traceId: traceId,
        spanId: rootSpanId,
        name: name,
        category: category,
        kind: MusePerformanceEventKind.traceStart,
        timestampMicros: now,
        attributes: _safeAttributes(attributes),
      ),
    );
    return MusePerformanceTrace._(
      tracer: this,
      traceId: traceId,
      rootSpanId: rootSpanId,
      name: name,
      category: category,
      startedAtMicros: now,
    );
  }

  String _id(String prefix) => '$prefix-${_nextId++}';
  int _now() => _clock() - _originMicros;
  void _record(MusePerformanceEvent event) {
    try {
      sink.record(event);
    } on Object {
      // Trace exporters are optional diagnostics. Dropping an event is safer
      // than changing the behavior or latency of the product path.
    }
  }
}

final class MusePerformanceTrace {
  MusePerformanceTrace._({
    required MusePerformanceTracer tracer,
    required this.traceId,
    required this.rootSpanId,
    required this.name,
    required this.category,
    required int startedAtMicros,
  })  : _tracer = tracer,
        _startedAtMicros = startedAtMicros;

  final MusePerformanceTracer _tracer;
  final String traceId;
  final String rootSpanId;
  final String name;
  final String category;
  final int _startedAtMicros;
  bool _finished = false;

  bool get isFinished => _finished;
  int get elapsedMicros => _tracer._now() - _startedAtMicros;

  MusePerformanceSpan startSpan(
    String name, {
    String? category,
    String? parentSpanId,
    Map<String, Object?> attributes = const {},
  }) {
    final spanId = _tracer._id('span');
    final now = _tracer._now();
    _tracer._record(
      MusePerformanceEvent(
        traceId: traceId,
        spanId: spanId,
        parentSpanId: parentSpanId ?? rootSpanId,
        name: name,
        category: category ?? this.category,
        kind: MusePerformanceEventKind.spanStart,
        timestampMicros: now,
        attributes: _safeAttributes(attributes),
      ),
    );
    return MusePerformanceSpan._(
      trace: this,
      spanId: spanId,
      parentSpanId: parentSpanId ?? rootSpanId,
      name: name,
      category: category ?? this.category,
      startedAtMicros: now,
    );
  }

  void mark(
    String name, {
    String? category,
    String? parentSpanId,
    Map<String, Object?> attributes = const {},
  }) {
    if (_finished) return;
    _tracer._record(
      MusePerformanceEvent(
        traceId: traceId,
        spanId: _tracer._id('mark'),
        parentSpanId: parentSpanId ?? rootSpanId,
        name: name,
        category: category ?? this.category,
        kind: MusePerformanceEventKind.mark,
        timestampMicros: _tracer._now(),
        attributes: _safeAttributes(attributes),
      ),
    );
  }

  Future<T> measure<T>(
    String name,
    Future<T> Function() operation, {
    String? category,
    Map<String, Object?> attributes = const {},
  }) async {
    final span = startSpan(
      name,
      category: category,
      attributes: attributes,
    );
    try {
      final result = await operation();
      span.end();
      return result;
    } on Object catch (error) {
      span.fail(error);
      rethrow;
    }
  }

  T measureSync<T>(
    String name,
    T Function() operation, {
    String? category,
    Map<String, Object?> attributes = const {},
  }) {
    final span = startSpan(
      name,
      category: category,
      attributes: attributes,
    );
    try {
      final result = operation();
      span.end();
      return result;
    } on Object catch (error) {
      span.fail(error);
      rethrow;
    }
  }

  void finish({
    String status = 'ok',
    Map<String, Object?> attributes = const {},
  }) {
    if (_finished) return;
    _finished = true;
    final now = _tracer._now();
    _tracer._record(
      MusePerformanceEvent(
        traceId: traceId,
        spanId: rootSpanId,
        name: name,
        category: category,
        kind: MusePerformanceEventKind.traceEnd,
        timestampMicros: now,
        durationMicros: now - _startedAtMicros,
        status: status,
        attributes: _safeAttributes(attributes),
      ),
    );
  }

  void fail(Object error, {Map<String, Object?> attributes = const {}}) {
    finish(
      status: 'error',
      attributes: {
        ...attributes,
        'errorType': error.runtimeType.toString(),
      },
    );
  }
}

final class MusePerformanceSpan {
  MusePerformanceSpan._({
    required MusePerformanceTrace trace,
    required this.spanId,
    required this.parentSpanId,
    required this.name,
    required this.category,
    required int startedAtMicros,
  })  : _trace = trace,
        _startedAtMicros = startedAtMicros;

  final MusePerformanceTrace _trace;
  final String spanId;
  final String parentSpanId;
  final String name;
  final String category;
  final int _startedAtMicros;
  bool _ended = false;

  void end({
    String status = 'ok',
    Map<String, Object?> attributes = const {},
  }) {
    if (_ended) return;
    _ended = true;
    final now = _trace._tracer._now();
    _trace._tracer._record(
      MusePerformanceEvent(
        traceId: _trace.traceId,
        spanId: spanId,
        parentSpanId: parentSpanId,
        name: name,
        category: category,
        kind: MusePerformanceEventKind.spanEnd,
        timestampMicros: now,
        durationMicros: now - _startedAtMicros,
        status: status,
        attributes: _safeAttributes(attributes),
      ),
    );
  }

  void fail(Object error, {Map<String, Object?> attributes = const {}}) {
    end(
      status: 'error',
      attributes: {
        ...attributes,
        'errorType': error.runtimeType.toString(),
      },
    );
  }
}

Map<String, Object?> _safeAttributes(Map<String, Object?> attributes) {
  if (attributes.isEmpty) return const {};
  return Map.unmodifiable({
    for (final entry in attributes.entries)
      entry.key: switch (entry.value) {
        null || bool() || num() => entry.value,
        final String value =>
          value.length <= 256 ? value : '${value.substring(0, 253)}...',
        _ => entry.value.toString(),
      },
  });
}
