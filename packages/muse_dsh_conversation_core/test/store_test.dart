import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

void main() {
  test('desktop journal events appear in native order', () {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: {'id': 's-1'},
        cursor: 2,
        records: [
          _event('user/message', 1, {
            'content': [_text('from desktop')],
            'source': {'kind': 'user'},
          }),
          _event('assistant/message', 2, {
            'message': {
              'content': [_text('same journal')],
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );
    expect(store.snapshot.rows.map((row) => row.text), [
      'from desktop',
      'same journal',
    ]);
    expect(store.snapshot.phase, DshConnectionPhase.live);
  });

  test('optimistic prompt retires on matching durable rpc id', () {
    final store = DshConversationStore();
    store.beginPrompt('rpc-1', 'hello', time: 1);
    expect(store.snapshot.rows.single.pending, isTrue);
    store.apply(
      DshEventFrame(
        _event('user/message', 1, {
          'content': [_text('hello')],
          'source': {'kind': 'user', 'rpcId': 'rpc-1'},
        }),
      ),
    );
    expect(store.snapshot.rows, hasLength(1));
    expect(store.snapshot.rows.single.kind, DshConversationRowKind.user);
  });

  test('recovery snapshot retires an optimistic prompt', () {
    final store = DshConversationStore();
    store.beginPrompt('rpc-page', 'sent through relay', time: 1);
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-page'},
        cursor: 2,
        records: [
          _event('user/message', 1, {
            'content': [_text('sent through relay')],
            'source': {'kind': 'user', 'rpcId': 'rpc-page'},
          }),
          _event('assistant/message', 2, {
            'message': {
              'content': [_text('recovered answer')],
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );

    expect(store.hasPendingPrompts, isFalse);
    expect(store.snapshot.rows.map((row) => row.text), [
      'sent through relay',
      'recovered answer',
    ]);
  });

  test('stale recovery snapshot cannot roll back a newer live event', () {
    final store = DshConversationStore();
    store.apply(
      DshEventFrame(
        _event('assistant/message', 3, {
          'message': {
            'content': [_text('newer')],
          },
        }),
      ),
    );
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-page'},
        cursor: 2,
        records: const [],
        hasMore: false,
        projections: const {},
      ),
    );
    expect(store.snapshot.cursor, 3);
    expect(store.snapshot.rows.single.text, 'newer');
  });

  test('unknown required event becomes an element-level compatibility row', () {
    final store = DshConversationStore();
    store.apply(DshEventFrame(_event('plugin/changes-chat', 1, const {})));
    expect(store.snapshot.requiresWebFallback, isFalse);
    expect(
      store.snapshot.rows.single.kind,
      DshConversationRowKind.incompatible,
    );
    expect(store.snapshot.rows.single.text, 'plugin/changes-chat');
  });

  test('official command lifecycle events stay hidden as native controls', () {
    final store = DshConversationStore();
    store.apply(DshEventFrame(_event('command/run', 1, const {})));
    store.apply(DshEventFrame(_event('command/done', 2, const {})));
    expect(store.snapshot.rows, isEmpty);
  });

  test('tool result replaces its running call without losing arguments', () {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-tools'},
        cursor: 2,
        records: [
          _event('tool/call', 1, {
            'callId': 'call-1',
            'name': 'weather',
            'arguments': '{"city":"上海"}',
          }),
          _event('tool/result', 2, {
            'message': {
              'toolCallId': 'call-1',
              'isError': false,
              'content': [
                {'type': 'text', 'text': '{"temperature":27}'},
              ],
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );

    expect(store.snapshot.rows, hasLength(1));
    expect(store.snapshot.rows.single.title, 'weather 完成');
    expect(
      ((store.snapshot.rows.single.nativeContext['tool'] as Map)['arguments']
          as Map)['city'],
      '上海',
    );
  });

  test('surface replacements remove shadowed message rows', () {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: {'id': 's-1'},
        cursor: 3,
        records: [
          _event('user/message', 1, {
            'content': [_text('old')],
            'source': {'kind': 'user'},
          }),
          _event('assistant/message', 2, {
            'message': {
              'content': [_text('old answer')],
            },
          }),
          DshWireEvent.fromJson({
            'type': 'user/message',
            'seq': 3,
            'time': 3,
            'data': {
              'content': [_text('summary')],
              'source': {'kind': 'user'},
            },
            'surfaceOp': {'op': 'replace', 'startSeq': 1, 'endSeq': 2},
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );
    expect(store.snapshot.rows.single.text, 'summary');
  });

  test('real DSH control events stay hidden and nested turn errors render', () {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-real'},
        cursor: 9,
        records: [
          _realEvent(0, 'permission/preset', {'preset': 'workspace-write'}),
          _realEvent(1, 'sandbox/mode', {'mode': 'workspace-write'}),
          _realEvent(2, 'approval/policy', {'policy': 'ask'}),
          _realEvent(3, 'agent/inbox/spliced', {
            'target': 'next-turn',
            'start': 0,
            'inserted': <Object?>[],
          }),
          _realEvent(4, 'turn/start', {'turn': 1}),
          _realEvent(5, 'user/message', {
            'content': [
              {'type': 'text', 'text': 'visible prompt'},
            ],
            'source': {'kind': 'user', 'rpcId': 'desktop-1'},
          }, surfaceOp: 'append'),
          _realEvent(6, 'user/message', {
            'content': [
              {'type': 'text', 'text': 'internal runtime context'},
            ],
            'source': {'kind': 'runtime-context', 'form': 'snapshot'},
          }, surfaceOp: 'append'),
          _realEvent(7, 'session/title', {'title': 'real session'}),
          _realEvent(8, 'session/title-llm-request', {
            'messageSeqs': [5],
          }),
          _realEvent(9, 'turn/end', {
            'turn': 1,
            'reason': {
              'kind': 'error',
              'error': {'message': 'missing model key', 'code': 'MISSING_KEY'},
            },
          }),
        ],
        hasMore: false,
        projections: const {
          'values': {
            'title': 'projected session',
            'agentPreset': 'standard',
            'modelSelection': {
              'lastUsed': {
                'provider': 'deepseek',
                'model': 'deepseek-flash',
                'reasoningEffort': 'high',
              },
            },
            'permissions': {'currentValue': 'workspace-write'},
          },
        },
      ),
    );

    expect(store.snapshot.requiresWebFallback, isFalse);
    expect(
      store.snapshot.rows.map((row) => row.text),
      containsAll(<String>['visible prompt', 'missing model key']),
    );
    expect(
      store.snapshot.rows.map((row) => row.text),
      isNot(contains('internal runtime context')),
    );
    expect(store.snapshot.rows.last.detail, 'MISSING_KEY');
    expect(store.snapshot.title, 'real session');
    expect(store.snapshot.agentPreset, 'standard');
    expect(store.snapshot.model, 'deepseek-flash');
    expect(store.snapshot.provider, 'deepseek');
    expect(store.snapshot.reasoningEffort, 'high');
    expect(store.snapshot.permissionPreset, 'workspace-write');
    expect(store.snapshot.turnCount, 1);
  });
}

DshWireEvent _realEvent(
  int seq,
  String type,
  Map<String, Object?> data, {
  Object? surfaceOp,
}) => DshWireEvent.fromJson({
  'type': type,
  'seq': seq,
  'time': seq + 1,
  'data': data,
  if (surfaceOp != null) 'surfaceOp': surfaceOp,
});

DshWireEvent _event(String type, int seq, Map<String, Object?> data) =>
    DshWireEvent.fromJson({
      'type': type,
      'seq': seq,
      'time': seq,
      'data': data,
      'surfaceOp': 'append',
    });

Map<String, Object?> _text(String value) => {'type': 'text', 'text': value};
