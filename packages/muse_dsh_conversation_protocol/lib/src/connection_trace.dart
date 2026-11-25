import 'dart:collection';
import 'dart:convert';
import 'dart:math';

/// Bounded, content-free connection diagnostics. Never pass URLs or errors here.
final class ConnectionTrace {
  ConnectionTrace({String? traceId, this.parentSpanId})
    : traceId =
          traceId != null &&
              RegExp(r'^[0-9a-f]{32}$').hasMatch(traceId) &&
              traceId != '0' * 32
          ? traceId
          : _id(16);
  final String traceId;
  final String? parentSpanId;
  static final _random = Random.secure();
  static final _recent = Queue<Map<String, Object?>>();
  static const logging = bool.fromEnvironment('OPENMUSE_CONNECTION_TRACE');
  static void Function(String)? sink;
  static List<Map<String, Object?>> get recent => List.unmodifiable(_recent);
  static String _id(int bytes) => List.generate(
    bytes,
    (_) => _random.nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();

  factory ConnectionTrace.fromTraceparent(String? value) {
    final match = RegExp(
      r'^00-([0-9a-f]{32})-([0-9a-f]{16})-0[01]$',
    ).firstMatch(value ?? '');
    if (match == null || match[1] == '0' * 32 || match[2] == '0' * 16)
      return ConnectionTrace();
    return ConnectionTrace(traceId: match[1], parentSpanId: match[2]);
  }

  ConnectionSpan start(String stage) {
    if (!RegExp(r'^[a-z][a-z0-9_.]{0,79}$').hasMatch(stage))
      throw ArgumentError('Invalid trace stage');
    return ConnectionSpan._(this, stage);
  }

  Future<T> measure<T>(
    String stage,
    Future<T> Function(ConnectionSpan) action,
  ) async {
    final span = start(stage);
    try {
      final result = await action(span);
      span.end();
      return result;
    } catch (_) {
      span.end(ok: false);
      rethrow;
    }
  }

  static void _emit(Map<String, Object?> event) {
    _recent.add(Map.unmodifiable(event));
    while (_recent.length > 512) {
      _recent.removeFirst();
    }
    // Observability must never break application I/O.
    try {
      final output = sink;
      if (output != null) {
        output('OMTRACE ${jsonEncode(event)}');
      } else if (logging) {
        print('OMTRACE ${jsonEncode(event)}');
      }
    } on Object {
      /* Diagnostic sinks are best effort. */
    }
  }

  static String route(String path) =>
      const {
        '/openmuse-native/v1/hello': 'hello',
        '/openmuse-native/v1/session/files/resolve': 'files_resolve',
        '/openmuse-native/v1/session/files/read': 'files_read',
        '/openmuse-native/v1/negotiate': 'negotiate',
        '/openmuse-native/v1/sessions': 'sessions',
        '/openmuse-native/v1/workspaces': 'workspaces',
        '/openmuse-native/v1/session/create': 'create',
        '/openmuse-native/v1/session/follow': 'follow',
        '/openmuse-native/v1/session/page': 'page',
        '/openmuse-native/v1/session/prompt': 'prompt',
        '/openmuse-native/v1/session/options': 'options',
        '/v1/account/open': 'account_open',
        '/v1/pair/open': 'pair_open',
      }[Uri.tryParse(path)?.path] ??
      'other';
}

final class ConnectionSpan {
  ConnectionSpan._(this.owner, this.stage) : spanId = ConnectionTrace._id(8) {
    mark('start');
  }
  final ConnectionTrace owner;
  final String stage;
  final String spanId;
  final _clock = Stopwatch()..start();
  bool _ended = false;
  String get traceparent => '00-${owner.traceId}-$spanId-01';
  ConnectionTrace get child =>
      ConnectionTrace(traceId: owner.traceId, parentSpanId: spanId);
  void mark(String phase, {bool? ok, int? bytes, int? status, bool? reused}) {
    if (_ended || !RegExp(r'^[a-z_]{1,40}$').hasMatch(phase)) return;
    ConnectionTrace._emit({
      'v': 1,
      'traceId': owner.traceId,
      'spanId': spanId,
      if (owner.parentSpanId != null) 'parentSpanId': owner.parentSpanId,
      'stage': stage,
      'phase': phase,
      'elapsedUs': _clock.elapsedMicroseconds,
      'tsUs': DateTime.now().microsecondsSinceEpoch,
      if (ok != null) 'ok': ok,
      if (bytes != null) 'bytes': bytes,
      if (status != null) 'status': status,
      if (reused != null) 'reused': reused,
    });
  }

  void end({bool ok = true}) {
    if (_ended) return;
    mark('end', ok: ok);
    _ended = true;
    _clock.stop();
  }
}
