import 'dart:async';

import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

import 'store.dart';

final class DshConversationController {
  DshConversationController({
    required this.client,
    DshConversationStore? store,
    this.reconciliationInterval = const Duration(seconds: 2),
    this.onConnectionFailure,
  }) : store = store ?? DshConversationStore();

  final DshNativeGatewayClient client;
  final DshConversationStore store;
  final Duration reconciliationInterval;
  final void Function(Object error)? onConnectionFailure;
  bool _closed = false;
  bool _active = false;
  int _generation = 0;
  String? _sessionId;
  StreamIterator<DshFollowFrame>? _followIterator;
  Future<void>? _followTask;
  Future<void>? _reconcileTask;
  int _requestSequence = 0;

  Future<void> start(String sessionId, {bool running = false}) async {
    if (_sessionId != null) throw StateError('conversation already started');
    _sessionId = sessionId;
    store.setRunning(running);
    resume();
  }

  /// Keeps the projected conversation in memory without holding a Desktop
  /// follow stream or polling an offstage session.
  void pause() {
    if (!_active) return;
    _active = false;
    _generation++;
    final iterator = _followIterator;
    _followIterator = null;
    if (iterator != null) {
      unawaited(iterator.cancel().catchError((Object _) {}));
    }
  }

  void resume() {
    final sessionId = _sessionId;
    if (_closed || _active || sessionId == null) return;
    _active = true;
    final generation = ++_generation;
    _followTask = _follow(sessionId, generation);
    _reconcileTask = _reconcile(sessionId, generation);
  }

  bool _isCurrent(int generation) =>
      !_closed && _active && generation == _generation;

  Future<void> send(String text, {String mode = 'queue'}) async {
    final sessionId = store.snapshot.sessionId;
    final normalized = text.trim();
    if (sessionId == null || normalized.isEmpty) return;
    final requestId =
        'flutter-${DateTime.now().microsecondsSinceEpoch}-${_requestSequence++}';
    print(
      'OpenMuse prompt: session=$sessionId request=$requestId chars=${normalized.length} mode=$mode',
    );
    store.beginPrompt(
      requestId,
      normalized,
      time: DateTime.now().millisecondsSinceEpoch,
    );
    try {
      await client.prompt(
        sessionId: sessionId,
        requestId: requestId,
        text: normalized,
        mode: mode,
      );
      print('OpenMuse prompt: accepted request=$requestId');
    } catch (error) {
      print('OpenMuse prompt: failed request=$requestId error=$error');
      store.promptFailed(requestId, error);
      rethrow;
    }
  }

  Future<void> cancel() async {
    final sessionId = store.snapshot.sessionId;
    if (sessionId != null) await client.cancel(sessionId);
  }

  Future<void> _follow(String sessionId, int generation) async {
    var attempt = 0;
    while (_isCurrent(generation)) {
      if (store.snapshot.rows.isEmpty) {
        store.connecting(sessionId, reconnecting: attempt > 0);
      }
      print(
        'OpenMuse follow: ${attempt == 0 ? 'start' : 'retry'} '
        'session=$sessionId attempt=$attempt',
      );
      StreamIterator<DshFollowFrame>? iterator;
      try {
        iterator = StreamIterator(client.follow(sessionId));
        _followIterator = iterator;
        while (await iterator.moveNext()) {
          if (!_isCurrent(generation)) return;
          final frame = iterator.current;
          attempt = 0;
          print('OpenMuse follow: ${_frameLog(frame)}');
          store.apply(frame);
        }
        if (!_isCurrent(generation)) return;
        throw const DshNativeGatewayException('DSH follow stream ended');
      } catch (error) {
        if (!_isCurrent(generation)) return;
        print('OpenMuse follow: error session=$sessionId error=$error');
        store.failed(error);
        attempt += 1;
        if (attempt >= 2 ||
            error is DshNativeGatewayException &&
                (error.statusCode == 401 || error.statusCode == 403)) {
          onConnectionFailure?.call(error);
        }
        final seconds = 1 << (attempt.clamp(1, 5) - 1);
        await Future<void>.delayed(Duration(seconds: seconds));
      } finally {
        if (identical(_followIterator, iterator)) _followIterator = null;
        if (iterator != null) {
          try {
            await iterator.cancel();
          } on Object {
            // Closing an in-flight SSE response can surface a socket error.
          }
        }
      }
    }
  }

  Future<void> _reconcile(String sessionId, int generation) async {
    var failures = 0;
    while (_isCurrent(generation)) {
      try {
        final sessions = await client.listSessions();
        if (!_isCurrent(generation)) return;
        DshNativeSessionSummary? authoritative;
        for (final session in sessions) {
          if (session.sessionId == sessionId) {
            authoritative = session;
            break;
          }
        }
        if (authoritative == null) {
          throw StateError('DSH session disappeared: $sessionId');
        }
        final current = store.snapshot;
        final needsPage =
            authoritative.headSeq > current.cursor ||
            store.hasPendingPrompts ||
            current.phase != DshConnectionPhase.live;
        if (needsPage) {
          final page = await client.sessionPage(authoritative);
          if (!_isCurrent(generation)) return;
          store.apply(page);
          print(
            'OpenMuse reconcile: session=$sessionId '
            'cursor=${page.cursor} records=${page.records.length} '
            'pending=${store.hasPendingPrompts}',
          );
        }
        store.setRunning(authoritative.running);
        failures = 0;
      } catch (error) {
        if (!_isCurrent(generation)) return;
        failures += 1;
        if (failures >= 2 ||
            error is DshNativeGatewayException &&
                (error.statusCode == 401 || error.statusCode == 403)) {
          onConnectionFailure?.call(error);
        }
        print(
          'OpenMuse reconcile: error session=$sessionId '
          'attempt=$failures error=$error',
        );
      }
      final multiplier = failures == 0 ? 1 : failures.clamp(1, 4);
      await Future<void>.delayed(reconciliationInterval * multiplier);
    }
  }

  String _frameLog(DshFollowFrame value) => switch (value) {
    DshSnapshotFrame(:final cursor, :final records) =>
      'snapshot cursor=$cursor records=${records.length}',
    DshEventFrame(:final event) => 'event ${event.type}@${event.seq}',
    DshAssistantStreamFrame(:final frame) =>
      'assistant-stream ${frame['type']}',
  };

  Future<void> dispose() async {
    _closed = true;
    pause();
    client.close();
    store.close();
    // The tasks observe [_closed] after their active I/O/delay. They are kept
    // as fields so failures are owned by this controller instead of becoming
    // unhandled detached futures.
    unawaited(_followTask?.catchError((_) {}) ?? Future<void>.value());
    unawaited(_reconcileTask?.catchError((_) {}) ?? Future<void>.value());
    await store.dispose();
  }
}
