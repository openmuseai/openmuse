import 'dart:async';
import 'dart:convert';

import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

enum DshConnectionPhase { idle, connecting, live, reconnecting, failed, closed }

enum DshConversationRowKind {
  user,
  assistant,
  tool,
  error,
  optimisticUser,
  liveAssistant,
  artifact,
  incompatible,
}

final class DshConversationRow {
  const DshConversationRow({
    required this.key,
    required this.kind,
    required this.text,
    required this.time,
    this.seq,
    this.reasoning = '',
    this.title,
    this.detail,
    this.pending = false,
    this.failed = false,
    this.nativeKey,
    this.nativeContext = const {},
  });

  final String key;
  final DshConversationRowKind kind;
  final String text;
  final String reasoning;
  final String? title;
  final String? detail;
  final int? seq;
  final int time;
  final bool pending;
  final bool failed;
  final String? nativeKey;
  final JsonMap nativeContext;
}

final class DshConversationSnapshot {
  const DshConversationSnapshot({
    required this.sessionId,
    required this.rows,
    required this.phase,
    required this.running,
    required this.hasMore,
    required this.cursor,
    required this.turnCount,
    required this.stepCount,
    this.failure,
    this.incompatibleReason,
    this.title,
    this.agentPreset,
    this.model,
    this.reasoningEffort,
    this.provider,
    this.permissionPreset,
  });

  final String? sessionId;
  final List<DshConversationRow> rows;
  final DshConnectionPhase phase;
  final bool running;
  final bool hasMore;
  final int cursor;
  final int turnCount;
  final int stepCount;
  final String? failure;
  final String? incompatibleReason;
  final String? title;
  final String? agentPreset;
  final String? model;
  final String? reasoningEffort;
  final String? provider;
  final String? permissionPreset;

  bool get requiresWebFallback => incompatibleReason != null;
}

final class DshConversationStore {
  final StreamController<void> _changes = StreamController<void>.broadcast(
    sync: true,
  );
  final Map<int, DshWireEvent> _journal = {};
  final Map<String, _PendingPrompt> _pending = {};
  final Map<String, _LiveAssistant> _live = {};
  String? _sessionId;
  DshConnectionPhase _phase = DshConnectionPhase.idle;
  bool _running = false;
  bool _hasMore = false;
  int _cursor = 0;
  String? _failure;
  String? _incompatibleReason;
  String? _title;
  String? _agentPreset;
  String? _model;
  String? _reasoningEffort;
  String? _provider;
  String? _permissionPreset;

  Stream<void> get changes => _changes.stream;
  bool get hasPendingPrompts => _pending.isNotEmpty;

  DshConversationSnapshot get snapshot => DshConversationSnapshot(
    sessionId: _sessionId,
    rows: List.unmodifiable(_projectRows()),
    phase: _phase,
    running: _running,
    hasMore: _hasMore,
    cursor: _cursor,
    turnCount: _journal.values
        .where((event) => event.type == 'turn/end')
        .length,
    stepCount: _journal.values
        .where((event) => event.type == 'step/end')
        .length,
    failure: _failure,
    incompatibleReason: _incompatibleReason,
    title: _title,
    agentPreset: _agentPreset,
    model: _model,
    reasoningEffort: _reasoningEffort,
    provider: _provider,
    permissionPreset: _permissionPreset,
  );

  void connecting(String sessionId, {bool reconnecting = false}) {
    _sessionId = sessionId;
    _phase = reconnecting
        ? DshConnectionPhase.reconnecting
        : DshConnectionPhase.connecting;
    _failure = null;
    _publish();
  }

  void apply(DshFollowFrame frame) {
    switch (frame) {
      case DshSnapshotFrame():
        // A page response can race with a live follow event. Never replace a
        // newer journal with an older recovery snapshot.
        if (frame.cursor < _cursor) return;
        _sessionId = frame.header['id'] as String? ?? _sessionId;
        _journal
          ..clear()
          ..addEntries(
            frame.records.map((event) => MapEntry(event.seq, event)),
          );
        _cursor = frame.cursor;
        _hasMore = frame.hasMore;
        _live.clear();
        _applyProjections(frame.projections);
        for (final event in frame.records) {
          _applyPresentationEvent(event);
          _retireObservedPrompt(event);
        }
        _phase = DshConnectionPhase.live;
        _failure = null;
        _scanCompatibility(frame.records);
        final last = frame.records.isEmpty ? null : frame.records.last;
        print(
          'OpenMuse store: snapshot session=$_sessionId cursor=${frame.cursor} '
          'records=${frame.records.length} pending=${_pending.length} '
          'last=${last == null ? '-' : '${last.type}@${last.seq}'}',
        );
      case DshEventFrame():
        final event = frame.event;
        _journal[event.seq] = event;
        if (event.seq > _cursor) _cursor = event.seq;
        if (event.type == 'turn/start') _running = true;
        if (event.type == 'turn/end') _running = false;
        _applyPresentationEvent(event);
        _retireObservedPrompt(event);
        _scanCompatibility([event]);
        print(
          'OpenMuse store: event ${event.type}@${event.seq} '
          'pending=${_pending.length}',
        );
      case DshAssistantStreamFrame():
        _applyAssistantStream(frame.frame);
    }
    _publish();
  }

  void setRunning(bool value) {
    if (_running == value) return;
    _running = value;
    _publish();
  }

  void failed(Object error, {bool reconnecting = true}) {
    _phase = reconnecting
        ? DshConnectionPhase.reconnecting
        : DshConnectionPhase.failed;
    _failure = error.toString();
    _publish();
  }

  void beginPrompt(String requestId, String text, {required int time}) {
    _pending[requestId] = _PendingPrompt(requestId, text, time);
    _publish();
  }

  void promptFailed(String requestId, Object error) {
    final value = _pending[requestId];
    if (value == null) return;
    value
      ..failed = true
      ..failure = error.toString();
    _publish();
  }

  void close() {
    _phase = DshConnectionPhase.closed;
    _publish();
  }

  Future<void> dispose() async {
    await _changes.close();
  }

  void _retireObservedPrompt(DshWireEvent event) {
    if (event.type != 'user/message') return;
    final source = event.data['source'];
    if (source is Map && source['rpcId'] is String) {
      final rpcId = source['rpcId'] as String;
      final removed = _pending.remove(rpcId) != null;
      print(
        'OpenMuse store: retire rpc=$rpcId removed=$removed pending=${_pending.length}',
      );
    } else {
      print('OpenMuse store: user/message seq=${event.seq} has no rpcId');
    }
  }

  void _applyProjections(JsonMap projections) {
    final rawValues = projections['values'];
    if (rawValues is! Map) return;
    final values = rawValues.cast<Object?, Object?>();
    _title = values['title'] as String? ?? _title;
    _agentPreset = values['agentPreset'] as String? ?? _agentPreset;
    final rawSelection = values['modelSelection'];
    if (rawSelection is Map) {
      final rawModel = rawSelection['next'] ?? rawSelection['lastUsed'];
      if (rawModel is Map) {
        _provider = rawModel['provider'] as String? ?? _provider;
        _model = rawModel['model'] as String? ?? _model;
        _reasoningEffort =
            rawModel['reasoningEffort'] as String? ?? _reasoningEffort;
      }
    }
    final rawPermissions = values['permissions'];
    if (rawPermissions is Map) {
      _permissionPreset =
          rawPermissions['currentValue'] as String? ?? _permissionPreset;
    }
  }

  void _applyPresentationEvent(DshWireEvent event) {
    if (event.type == 'session/title') {
      _title = event.data['title'] as String? ?? _title;
    } else if (event.type == 'model/selection') {
      _provider = event.data['provider'] as String? ?? _provider;
      _model = event.data['model'] as String? ?? _model;
      _reasoningEffort =
          event.data['reasoningEffort'] as String? ?? _reasoningEffort;
    } else if (event.type == 'permission/preset') {
      _permissionPreset = event.data['preset'] as String? ?? _permissionPreset;
    }
  }

  void _applyAssistantStream(JsonMap frame) {
    final type = frame['type'];
    final attemptId = frame['attemptId'];
    if (attemptId is! String) return;
    switch (type) {
      case 'start':
        _live[attemptId] = _LiveAssistant(
          attemptId,
          frame['startedAfterSeq'] is int
              ? frame['startedAfterSeq']! as int
              : _cursor,
        );
      case 'chunk':
        final value = _live.putIfAbsent(
          attemptId,
          () => _LiveAssistant(attemptId, _cursor),
        );
        final chunk = frame['chunk'];
        if (chunk is Map) {
          if (chunk['type'] == 'text-delta' && chunk['text'] is String) {
            value.text.write(chunk['text']);
          }
          if (chunk['type'] == 'reasoning-delta' && chunk['text'] is String) {
            value.reasoning.write(chunk['text']);
          }
        }
      case 'end':
        final outcome = frame['outcome'];
        if (outcome is Map && outcome['kind'] == 'committed') {
          _live.remove(attemptId);
        }
    }
  }

  void _scanCompatibility(Iterable<DshWireEvent> events) {
    // Unknown plugin events are represented by an element-level row in
    // [_rowOf]. They must never evict the rest of a usable native transcript.
  }

  List<DshConversationRow> _projectRows() {
    final events = _journal.values.toList()
      ..sort((left, right) => left.seq.compareTo(right.seq));
    final activeSurface = <int>[];
    for (final event in events.where(_isSurfaceEvent)) {
      final op = event.surfaceOp;
      if (op == 'append' || op == null) {
        activeSurface.add(event.seq);
      } else if (op is Map && op['op'] == 'replace') {
        final start = op['startSeq'];
        final end = op['endSeq'];
        if (start is int && end is int) {
          activeSurface.removeWhere((seq) => seq >= start && seq <= end);
          activeSurface.add(event.seq);
          activeSurface.sort();
        }
      }
    }
    final active = activeSurface.toSet();
    final rows = <DshConversationRow>[];
    final toolCalls = <String, JsonMap>{};
    final completedToolCalls = <String>{};
    for (final event in events.where((event) => event.type == 'tool/call')) {
      final callId = event.data['callId'];
      if (callId is String) toolCalls[callId] = event.data;
    }
    for (final event in events.where((event) => event.type == 'tool/result')) {
      final message = event.data['message'];
      final callId = message is Map ? message['toolCallId'] : null;
      if (callId is String) completedToolCalls.add(callId);
    }
    for (final event in events) {
      if (_isSurfaceEvent(event) && !active.contains(event.seq)) continue;
      if (event.type == 'tool/call' &&
          completedToolCalls.contains(event.data['callId'])) {
        continue;
      }
      final row = _rowOf(event, toolCalls);
      if (row != null) rows.add(row);
    }
    rows.addAll(
      _pending.values.map(
        (value) => DshConversationRow(
          key: 'pending:${value.requestId}',
          kind: DshConversationRowKind.optimisticUser,
          text: value.text,
          detail: value.failure,
          time: value.time,
          pending: !value.failed,
          failed: value.failed,
        ),
      ),
    );
    rows.addAll(
      _live.values.map(
        (value) => DshConversationRow(
          key: 'live:${value.attemptId}',
          kind: DshConversationRowKind.liveAssistant,
          text: value.text.toString(),
          reasoning: value.reasoning.toString(),
          time: DateTime.now().millisecondsSinceEpoch,
          pending: true,
        ),
      ),
    );
    return rows;
  }

  DshConversationRow? _rowOf(
    DshWireEvent event,
    Map<String, JsonMap> toolCalls,
  ) {
    switch (event.type) {
      case 'user/message':
        final source = event.data['source'];
        if (source is Map && source['kind'] != 'user') return null;
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: DshConversationRowKind.user,
          text: _contentText(event.data['content']),
          time: event.time,
          seq: event.seq,
        );
      case 'assistant/message':
        final message = event.data['message'];
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: DshConversationRowKind.assistant,
          text: _contentText(message is Map ? message['content'] : null),
          reasoning: _contentText(
            message is Map ? message['content'] : null,
            type: 'reasoning',
          ),
          time: event.time,
          seq: event.seq,
        );
      case 'tool/call':
        final name = event.data['name'] as String? ?? 'Tool';
        final arguments = _decodeJsonOrText(event.data['arguments']);
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: DshConversationRowKind.tool,
          title: _toolTitle(name, completed: false),
          text: event.data['arguments'] as String? ?? '',
          detail: '运行中',
          time: event.time,
          seq: event.seq,
          nativeKey: name,
          nativeContext: {
            'tool': {
              'phase': 'call',
              'callId': event.data['callId'],
              'name': name,
              'arguments': arguments,
            },
          },
        );
      case 'tool/result':
        final message = event.data['message'];
        final failed = message is Map && message['isError'] == true;
        final callId = message is Map ? message['toolCallId'] : null;
        final call = callId is String ? toolCalls[callId] : null;
        final name = call?['name'] as String?;
        final resultText = _contentText(
          message is Map ? message['content'] : null,
        );
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: failed
              ? DshConversationRowKind.error
              : DshConversationRowKind.tool,
          title: _toolTitle(name ?? 'Tool', completed: true, failed: failed),
          text: resultText,
          detail: event.data['error'] is Map
              ? (event.data['error'] as Map)['reason'] as String?
              : null,
          failed: failed,
          time: event.time,
          seq: event.seq,
          nativeKey: name,
          nativeContext: {
            'tool': {
              'phase': 'result',
              'callId': callId,
              'name': name,
              'arguments': _decodeJsonOrText(call?['arguments']),
              'result': _decodeJsonOrText(resultText),
              'meta': event.data['meta'],
              'error': event.data['error'],
            },
          },
        );
      case 'llm/retry':
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: DshConversationRowKind.error,
          title: '模型重试',
          text: event.data['reason']?.toString() ?? '正在重试模型请求',
          time: event.time,
          seq: event.seq,
        );
      case 'workspace/changes':
        final turn = event.data['turn'];
        if (turn is! int || turn < 1) return null;
        return DshConversationRow(
          key: 'event:${event.seq}',
          kind: DshConversationRowKind.artifact,
          title: '工作区产物',
          text: '本轮生成或修改的文件',
          time: event.time,
          seq: event.seq,
          nativeKey: 'workspace/changes',
          nativeContext: {
            'workspaceChanges': {'seq': event.seq, 'turn': turn},
          },
        );
      case 'turn/end':
        final reason = event.data['reason'];
        if (reason is Map && reason['kind'] == 'error') {
          final error = reason['error'];
          return DshConversationRow(
            key: 'event:${event.seq}',
            kind: DshConversationRowKind.error,
            title: '本轮运行失败',
            text: error is Map
                ? error['message']?.toString() ?? 'DSH Agent 执行失败'
                : reason['message']?.toString() ?? 'DSH Agent 执行失败',
            detail: error is Map ? error['code']?.toString() : null,
            time: event.time,
            seq: event.seq,
          );
        }
      default:
        if (!_knownEvents.contains(event.type) && !event.ignorable) {
          return DshConversationRow(
            key: 'incompatible:${event.seq}',
            kind: DshConversationRowKind.incompatible,
            title: 'Mobile 暂不支持此 DSH 元素',
            text: event.type,
            detail: '其它消息仍以原生方式显示；可按需打开 Web 兼容模式。',
            time: event.time,
            seq: event.seq,
          );
        }
    }
    return null;
  }

  String _contentText(Object? content, {String type = 'text'}) {
    if (content is! List) return '';
    return content
        .whereType<Map>()
        .where((block) => block['type'] == type && block['text'] is String)
        .map((block) => block['text'] as String)
        .join();
  }

  Object? _decodeJsonOrText(Object? value) {
    if (value is! String) return value;
    try {
      return jsonDecode(value);
    } on FormatException {
      return value;
    }
  }

  String _toolTitle(
    String name, {
    required bool completed,
    bool failed = false,
  }) {
    final action = switch (name) {
      'read' || 'read_image' => '读取文件',
      'bash' || 'pwsh' || 'terminal_send' => '运行命令',
      'write' => '写入文件',
      'edit' || 'str_replace_editor' => '修改文件',
      'glob' || 'grep' => '搜索工作区',
      'web_search' => '搜索网络',
      'web_fetch' => '读取网页',
      'ask_user_question' => '提问',
      _ => null,
    };
    if (action == null) {
      if (failed) return '$name 失败';
      return completed ? '$name 完成' : name;
    }
    if (failed) return '$action失败';
    return completed ? '已$action' : '正在$action';
  }

  bool _isSurfaceEvent(DshWireEvent event) => const {
    'system/message',
    'developer/message',
    'user/message',
    'assistant/message',
    'tool/result',
  }.contains(event.type);

  void _publish() {
    if (!_changes.isClosed) _changes.add(null);
  }
}

final class _PendingPrompt {
  _PendingPrompt(this.requestId, this.text, this.time);
  final String requestId;
  final String text;
  final int time;
  bool failed = false;
  String? failure;
}

final class _LiveAssistant {
  _LiveAssistant(this.attemptId, this.startedAfterSeq);
  final String attemptId;
  final int startedAfterSeq;
  final StringBuffer text = StringBuffer();
  final StringBuffer reasoning = StringBuffer();
}

const _knownEvents = <String>{
  'permission/preset',
  'sandbox/mode',
  'approval/policy',
  'agent/inbox/spliced',
  'session/start',
  'session/end-seed',
  'session/title',
  'session/title-llm-request',
  'turn/start',
  'turn/end',
  'step/start',
  'step/end',
  'user/message',
  'developer/message',
  'system/message',
  'assistant/message',
  'assistant/attempt',
  'tool/call',
  'tool/result',
  'request/header',
  'request/context',
  // DSH command lifecycle records are transport/control events. User-visible
  // state is projected by the command itself (for example permission/preset),
  // so rendering these as unsupported plugin UI would be misleading.
  'command/run',
  'command/done',
  'model/selection',
  'llm/retry',
};
