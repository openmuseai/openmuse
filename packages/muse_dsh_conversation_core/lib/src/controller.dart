import 'dart:async';

import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

import 'store.dart';

final class DshConversationController {
  DshConversationController({
    required this.client,
    DshConversationStore? store,
    this.reconciliationInterval = const Duration(seconds: 2),
  }) : store = store ?? DshConversationStore();

  final DshNativeGatewayClient client;
  final DshConversationStore store;
  final Duration reconciliationInterval;
  bool _closed = false;
  Future<void>? _followTask;
  Future<void>? _reconcileTask;
  int _requestSequence = 0;

  Future<void> start(String sessionId, {bool running = false}) async {
    if (_followTask != null) throw StateError('conversation already started');
    store.setRunning(running);
    _followTask = _follow(sessionId);
    _reconcileTask = _reconcile(sessionId);
  }

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

  Future<void> _follow(String sessionId) async {
    var attempt = 0;
    while (!_closed) {
      store.connecting(sessionId, reconnecting: attempt > 0);
      print(
        'OpenMuse follow: ${attempt == 0 ? 'start' : 'retry'} '
        'session=$sessionId attempt=$attempt',
      );
      try {
        await for (final frame in client.follow(sessionId)) {
          if (_closed) return;
          attempt = 0;
          print('OpenMuse follow: ${_frameLog(frame)}');
          store.apply(frame);
        }
        if (_closed) return;
        throw const DshNativeGatewayException('DSH follow stream ended');
      } catch (error) {
        if (_closed) return;
        print('OpenMuse follow: error session=$sessionId error=$error');
        store.failed(error);
        attempt += 1;
        final seconds = 1 << (attempt.clamp(1, 5) - 1);
        await Future<void>.delayed(Duration(seconds: seconds));
      }
    }
  }

  Future<void> _reconcile(String sessionId) async {
    var failures = 0;
    while (!_closed) {
      try {
        final sessions = await client.listSessions();
        if (_closed) return;
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
          if (_closed) return;
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
        if (_closed) return;
        failures += 1;
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
