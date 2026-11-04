import 'dart:async';
import 'dart:convert';
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

/// DSH's private native transport adapter. Consumers only see MCP tools and
/// opaque caller-owned references, never grants, session paths or credentials.
final class OpenMuseDshMobileAgent
    implements OpenMusePlugin, OpenMuseToolContributor {
  OpenMuseDshMobileAgent({required this.connection, required this.context});
  final BuildContext? Function() context;
  final PairedDesktopConnection? Function() connection;
  final _conversations = <String, _Conversation>{};
  final _tasks = <String, _Task>{};
  final _starting = <String>{};
  final _epochs = <String, int>{};
  String _id() => List.generate(
    24,
    (_) => Random.secure().nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.dsh-mobile-agent',
    name: 'DSH Agent',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {
    for (final c in {..._starting, ..._tasks.values.map((t) => t.caller)}) {
      await disposeToolCaller(c);
    }
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
  static Map<String, Object?> _tool(
    String name,
    Map<String, Object?> properties,
    List<String> required,
  ) => {
    'name': name,
    'description':
        'DSH generation v1; only this caller’s tasks and conversations. Output is stored in a real DSH session. Uses the Agent’s configured model and permissions.',
    'inputSchema': {
      'type': 'object',
      'properties': properties,
      'required': required,
      'additionalProperties': false,
    },
  };
  @override
  List<Map<String, Object?>> get contributedTools => [
    _tool(
      'dsh_generate',
      {
        'text': {'type': 'string', 'minLength': 1, 'maxLength': 32000},
        'conversationRef': {'type': 'string'},
      },
      ['text'],
    ),
    _tool(
      'dsh_generation_status',
      {
        'taskRef': {'type': 'string'},
      },
      ['taskRef'],
    ),
    _tool(
      'dsh_generation_cancel',
      {
        'taskRef': {'type': 'string'},
      },
      ['taskRef'],
    ),
  ];
  Map<String, Object?> _result(_Task task) => {
    'content': [
      {'type': 'text', 'text': jsonEncode(task.json())},
    ],
    'structuredContent': task.json(),
  };
  @override
  Future<Map<String, Object?>> callContributedTool(
    String caller,
    String name,
    Map<String, Object?> arguments,
  ) async {
    final link = connection();
    if (link == null ||
        link.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw StateError('DSH_CONNECTION_UNAVAILABLE');
    }
    if (name != 'dsh_generate') {
      if (!{'dsh_generation_status', 'dsh_generation_cancel'}.contains(name) ||
          arguments.keys.any((k) => k != 'taskRef')) {
        throw StateError('INVALID_INPUT');
      }
      final task = _tasks[arguments['taskRef']];
      if (task == null || task.caller != caller) {
        throw StateError('TASK_SCOPE_MISMATCH');
      }
      if (task.conversation.grant != link.grantRef) {
        await disposeToolCaller(caller);
        throw StateError('CONNECTION_CHANGED');
      }
      if (name == 'dsh_generation_cancel') {
        await _cancel(task);
      }
      return _result(task);
    }
    final text = arguments['text'];
    if (text is! String ||
        text.trim().isEmpty ||
        text.length > 32000 ||
        arguments.keys.any((k) => !{'text', 'conversationRef'}.contains(k))) {
      throw StateError('INVALID_INPUT');
    }
    if (_starting.contains(caller) ||
        _tasks.values.any((t) => t.caller == caller && t.status == 'running')) {
      throw StateError('GENERATION_ALREADY_RUNNING');
    }
    _tasks.removeWhere((_, t) => t.status != 'running' && _tasks.length >= 128);
    if (_tasks.length >= 128) {
      throw StateError('GENERATION_BUDGET_EXCEEDED');
    }
    final epoch = _epochs[caller] ?? 0;
    _starting.add(caller);
    _Conversation? created;
    try {
      final ref = arguments['conversationRef'];
      var conversation = ref == null ? null : _conversations[ref];
      if (ref != null &&
          (conversation == null ||
              conversation.caller != caller ||
              conversation.grant != link.grantRef)) {
        throw StateError('CONVERSATION_SCOPE_MISMATCH');
      }
      if (conversation == null) {
        if (_conversations.length >= 128) {
          throw StateError('CONVERSATION_BUDGET_EXCEEDED');
        }
        final client = DshNativeGatewayClient(
          origin: Uri.parse(link.session.origin),
          bootstrapPath: link.session.path,
          allowInsecureLoopback: link.session.allowInsecureLoopback,
          allowInsecurePrivateNetworkForTesting:
              link.session.allowInsecurePrivateNetworkForTesting,
        );
        try {
          final hello = await client.initialize();
          if (!hello.contractSupported) {
            throw StateError('DSH_VERSION_UNSUPPORTED');
          }
          final workspaces = await client.listWorkspaces();
          final matches = workspaces
              .where(
                (w) =>
                    w.workspaceId == link.workspaceRef ||
                    w.title == link.workspaceTitle,
              )
              .toList();
          var workspace = matches.length == 1
              ? matches.single
              : workspaces.length == 1
              ? workspaces.single
              : null;
          if (workspace == null) {
            final current = context();
            if (current == null || !current.mounted) {
              throw StateError('DSH_WORKSPACE_UNAVAILABLE');
            }
            workspace = await showDialog<DshNativeWorkspaceSummary>(
              context: current,
              builder: (dialog) => SimpleDialog(
                title: const Text('选择讲解会话的工作区'),
                children: [
                  for (final w in workspaces)
                    SimpleDialogOption(
                      onPressed: () => Navigator.pop(dialog, w),
                      child: Text(w.title),
                    ),
                ],
              ),
            );
          }
          if (workspace == null) {
            throw StateError('WORKSPACE_SELECTION_CANCELLED');
          }
          final session = await client.createSession(
            workspaceId: workspace.workspaceId,
          );
          conversation = _Conversation(
            _id(),
            caller,
            link.grantRef,
            session.sessionId,
            client,
          );
          created = conversation;
        } catch (_) {
          client.close();
          rethrow;
        }
      }
      if ((_epochs[caller] ?? 0) != epoch ||
          connection()?.grantRef != link.grantRef) {
        created?.client.close();
        throw StateError('CALLER_CLOSED');
      }
      _conversations[conversation.ref] = conversation;
      final task = _Task(_id(), caller, conversation);
      _tasks[task.ref] = task;
      unawaited(_run(task, text));
      return _result(task);
    } finally {
      _starting.remove(caller);
    }
  }

  Future<void> _run(_Task task, String text) async {
    final snapshot = Completer<void>();
    try {
      task.follow = task.conversation.client
          .follow(task.conversation.session)
          .listen(
            (frame) {
              if (frame is DshSnapshotFrame) {
                if (!snapshot.isCompleted) {
                  snapshot.complete();
                }
                return;
              }
              if (frame is! DshEventFrame ||
                  task.status != 'running' ||
                  !task.submitted) {
                return;
              }
              final event = frame.event;
              if (event.type == 'turn/start') {
                task.started = true;
              }
              if (!task.started) {
                return;
              }
              if (event.type == 'assistant/message') {
                final message = event.data['message'] as Map?;
                final output = (message?['content'] as List? ?? [])
                    .whereType<Map>()
                    .where((c) => c['type'] == 'text')
                    .map((c) => c['text'] as String? ?? '')
                    .join('\n');
                if (output.isNotEmpty) {
                  task.output.add(output);
                }
              }
              if (event.type == 'turn/end') {
                final reason = event.data['reason'] as Map?;
                task.status =
                    reason?['kind'] == 'completed' && task.output.isNotEmpty
                    ? 'completed'
                    : 'failed';
                if (task.status == 'failed') {
                  task.error = 'DSH_GENERATION_INCOMPLETE';
                }
                unawaited(_cleanup(task));
              }
            },
            onError: (Object error) {
              if (!snapshot.isCompleted) {
                snapshot.completeError(StateError('DSH_STREAM_DISCONNECTED'));
              }
              if (task.status == 'running') {
                task.status = 'failed';
                task.error = 'DSH_STREAM_DISCONNECTED';
              }
            },
            onDone: () {
              if (!snapshot.isCompleted) {
                snapshot.completeError(StateError('DSH_STREAM_CLOSED'));
              }
              if (task.status == 'running') {
                task.status = 'failed';
                task.error = 'DSH_STREAM_CLOSED';
              }
            },
          );
      await snapshot.future.timeout(const Duration(seconds: 30));
      if (task.status != 'running') {
        return;
      }
      task.submitted = true;
      await task.conversation.client.prompt(
        sessionId: task.conversation.session,
        requestId: 'plugin-${task.ref}',
        text: text,
      );
      if (task.status == 'cancelled') {
        await task.conversation.client.cancel(task.conversation.session);
        return;
      }
      if (task.status == 'running') {
        task.timer = Timer(
          const Duration(minutes: 3),
          () => unawaited(_cancel(task, timeout: true)),
        );
      }
    } catch (_) {
      if (task.status == 'running') {
        task.status = 'failed';
        task.error = 'DSH_GENERATION_FAILED';
      }
      await _cleanup(task);
    }
  }

  Future<void> _cleanup(_Task task) async {
    task.timer?.cancel();
    task.timer = null;
    await task.follow?.cancel();
    task.follow = null;
  }

  Future<void> _cancel(_Task task, {bool timeout = false}) async {
    if (task.status != 'running') {
      return;
    }
    task.status = timeout ? 'failed' : 'cancelled';
    if (timeout) {
      task.error = 'GENERATION_TIMEOUT';
    }
    try {
      if (task.submitted) {
        await task.conversation.client.cancel(task.conversation.session);
      }
    } catch (_) {
      task.status = 'failed';
      task.error = 'GENERATION_CANCEL_UNCONFIRMED';
    } finally {
      await _cleanup(task);
    }
  }

  @override
  Future<void> disposeToolCaller(String caller) async {
    _epochs[caller] = (_epochs[caller] ?? 0) + 1;
    for (final task
        in _tasks.values.where((t) => t.caller == caller).toList()) {
      await _cancel(task);
      _tasks.remove(task.ref);
    }
    for (final entry
        in _conversations.entries
            .where((e) => e.value.caller == caller)
            .toList()) {
      entry.value.client.close();
      _conversations.remove(entry.key);
    }
  }
}

final class _Conversation {
  _Conversation(this.ref, this.caller, this.grant, this.session, this.client);
  final String ref, caller, grant, session;
  final DshNativeGatewayClient client;
}

final class _Task {
  _Task(this.ref, this.caller, this.conversation);
  final String ref, caller;
  final _Conversation conversation;
  String status = 'running';
  String? error;
  bool submitted = false, started = false;
  final output = <String>[];
  StreamSubscription<DshFollowFrame>? follow;
  Timer? timer;
  Map<String, Object?> json() => {
    'taskRef': ref,
    'conversationRef': conversation.ref,
    'status': status,
    'text': output.join('\n'),
    if (error != null) 'error': error,
  };
}
