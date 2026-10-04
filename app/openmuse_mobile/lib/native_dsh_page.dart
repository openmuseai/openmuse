import 'dart:async';

import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

import 'remote_dsh_page.dart';

final class NativeDshSessionHandle extends ChangeNotifier {
  DshConversationController? _conversation;
  StreamSubscription<void>? _changes;
  DshNativeSessionOptions? _options;

  DshConversationSnapshot? get snapshot => _conversation?.store.snapshot;
  DshNativeSessionOptions? get options => _options;
  bool get ready => _conversation != null;

  void attach(
    DshConversationController conversation,
    DshNativeSessionOptions? options,
  ) {
    _changes?.cancel();
    _conversation = conversation;
    _options = options;
    _changes = conversation.store.changes.listen((_) => notifyListeners());
    notifyListeners();
  }

  void detach(DshConversationController conversation) {
    if (!identical(_conversation, conversation)) return;
    _changes?.cancel();
    _changes = null;
    _conversation = null;
    _options = null;
    notifyListeners();
  }

  Future<void> send(String text, {String mode = 'queue'}) async {
    final conversation = _conversation;
    if (conversation == null) throw StateError('DSH session is not ready');
    await conversation.send(text, mode: mode);
  }

  Future<void> cancel() async => _conversation?.cancel();

  Future<void> selectModel(
    DshNativeModelOption option, {
    String? effort,
  }) async {
    final conversation = _conversation;
    final sessionId = conversation?.store.snapshot.sessionId;
    if (conversation == null || sessionId == null) return;
    await conversation.client.selectModel(
      sessionId: sessionId,
      provider: option.provider,
      model: option.id,
      reasoningEffort: effort,
    );
  }

  Future<void> selectPermission(String preset) async {
    final conversation = _conversation;
    final sessionId = conversation?.store.snapshot.sessionId;
    if (conversation == null || sessionId == null) return;
    await conversation.client.selectPermission(
      sessionId: sessionId,
      preset: preset,
    );
  }

  @override
  void dispose() {
    _changes?.cancel();
    super.dispose();
  }
}

DshNativeSessionSummary? chooseNativeDshSession(
  DshSessionDescriptor descriptor,
  List<DshNativeSessionSummary> sessions, {
  String? requestedSessionId,
}) {
  if (requestedSessionId != null) {
    for (final session in sessions) {
      if (session.sessionId == requestedSessionId) return session;
    }
    return null;
  }
  final segments = Uri(path: descriptor.path).pathSegments;
  if (segments.length >= 2 && segments.first == 'session') {
    final requested = segments[1];
    for (final session in sessions) {
      if (session.sessionId == requested) return session;
    }
  }
  for (final session in sessions) {
    if (session.running) return session;
  }
  for (final session in sessions) {
    if (!session.blank) return session;
  }
  return sessions.isEmpty ? null : sessions.first;
}

final class NativeDshPage extends StatefulWidget {
  const NativeDshPage({
    super.key,
    required this.session,
    required this.workspaceTitle,
    this.requestedSessionId,
    this.active = true,
    this.embedded = false,
    this.showComposer = true,
    this.handle,
    this.initialPrompt,
    this.onInitialPromptConsumed,
  });

  final DshSessionDescriptor session;
  final String workspaceTitle;
  final String? requestedSessionId;
  final bool active;
  final bool embedded;
  final bool showComposer;
  final NativeDshSessionHandle? handle;
  final String? initialPrompt;
  final VoidCallback? onInitialPromptConsumed;

  @override
  State<NativeDshPage> createState() => _NativeDshPageState();
}

final class _NativeDshPageState extends State<NativeDshPage> {
  DshNativeGatewayClient? _client;
  DshConversationController? _conversation;
  DshNativeNegotiation? _negotiation;
  DshNativeSessionOptions? _options;
  String? _failure;
  bool _webFallback = false;
  bool _started = false;

  @override
  void initState() {
    super.initState();
    if (widget.active) _ensureOpen();
  }

  void _ensureOpen() {
    if (_started) return;
    _started = true;
    unawaited(_openWithRetry());
  }

  @override
  void didUpdateWidget(covariant NativeDshPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.active && !oldWidget.active) _ensureOpen();
    final conversation = _conversation;
    if (conversation == null) return;
    if (oldWidget.active != widget.active) {
      if (widget.active) {
        conversation.resume();
      } else {
        conversation.pause();
      }
    }
    if (oldWidget.handle != widget.handle) {
      oldWidget.handle?.detach(conversation);
      widget.handle?.attach(conversation, _options);
    }
  }

  Future<void> _openWithRetry() async {
    var attempt = 0;
    while (mounted) {
      if (!widget.active) {
        _started = false;
        return;
      }
      try {
        await _open();
        return;
      } catch (error) {
        _client?.close();
        _client = null;
        if (!mounted) return;
        debugPrint('OpenMuse native DSH open: attempt=$attempt error=$error');
        setState(() => _failure = 'Desktop 连接暂时不可用，正在自动重试…');
        final seconds = 1 << attempt.clamp(0, 3);
        attempt++;
        await Future<void>.delayed(Duration(seconds: seconds));
      }
    }
  }

  Future<void> _open() async {
    final client = DshNativeGatewayClient(
      origin: Uri.parse(widget.session.origin),
      bootstrapPath: widget.session.path,
      allowInsecureLoopback: widget.session.allowInsecureLoopback,
      allowInsecurePrivateNetworkForTesting:
          widget.session.allowInsecurePrivateNetworkForTesting,
    );
    _client = client;
    final hello = await client.initialize();
    if (!hello.contractSupported) {
      client.close();
      _useWebFallback();
      return;
    }
    final values = await Future.wait<Object>([
      client.negotiate(DshNativeCapabilities.standard()),
      client.listSessions(),
    ]);
    final negotiation = values[0] as DshNativeNegotiation;
    final sessions = values[1] as List<DshNativeSessionSummary>;
    final selected = chooseNativeDshSession(
      widget.session,
      sessions,
      requestedSessionId: widget.requestedSessionId,
    );
    if (selected == null) {
      client.close();
      debugPrint(
        'OpenMuse native DSH: session ${widget.requestedSessionId} '
        'is absent from Desktop catalog',
      );
      _useWebFallback();
      return;
    }
    final conversation = DshConversationController(client: client);
    await conversation.start(selected.sessionId, running: selected.running);
    if (!widget.active) conversation.pause();
    if (!mounted) {
      await conversation.dispose();
      return;
    }
    setState(() {
      _conversation = conversation;
      _negotiation = negotiation;
      _failure = null;
    });
    widget.handle?.attach(conversation, null);
    unawaited(_loadOptions(conversation, selected.sessionId));
    final initialPrompt = widget.initialPrompt?.trim();
    if (initialPrompt != null && initialPrompt.isNotEmpty) {
      try {
        await conversation.send(initialPrompt);
        widget.onInitialPromptConsumed?.call();
      } catch (error) {
        debugPrint('OpenMuse native DSH prompt: $error');
      }
    }
  }

  Future<void> _loadOptions(
    DshConversationController conversation,
    String sessionId,
  ) async {
    try {
      final options = await conversation.client.sessionOptions(sessionId);
      if (!mounted || !identical(_conversation, conversation)) return;
      _options = options;
      widget.handle?.attach(conversation, options);
    } on Object {
      // Older pinned bridge builds can still render and send safely.
    }
  }

  void _useWebFallback({String? failure}) {
    if (!mounted) return;
    setState(() {
      _webFallback = true;
      _failure = failure;
    });
  }

  @override
  void dispose() {
    final conversation = _conversation;
    if (conversation != null) {
      widget.handle?.detach(conversation);
      unawaited(conversation.dispose());
    } else {
      _client?.close();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_webFallback) {
      return RemoteDshPage(
        session: widget.session,
        workspaceTitle: widget.workspaceTitle,
      );
    }
    final conversation = _conversation;
    final content = conversation == null
        ? Center(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const CircularProgressIndicator.adaptive(),
                const SizedBox(height: 12),
                Text(_failure ?? '正在连接 Desktop DSH…'),
              ],
            ),
          )
        : DshConversationSurface(
            store: conversation.store,
            nativeContributions: _negotiation?.contributions ?? const [],
            compatibilityPlugins: _negotiation?.plugins ?? const [],
            onSend: conversation.send,
            onCancel: conversation.cancel,
            onFallbackRequested: _useWebFallback,
            onQuestionAnswer: (answers) => conversation.client.answerQuestion(
              sessionId: conversation.store.snapshot.sessionId!,
              answers: answers,
            ),
            onLoadWorkspaceChanges: (seq) =>
                conversation.client.workspaceChanges(
                  sessionId: conversation.store.snapshot.sessionId!,
                  seq: seq,
                ),
            onLoadWorkspaceChangePreview: (seq, index) =>
                conversation.client.workspaceChangePreview(
                  sessionId: conversation.store.snapshot.sessionId!,
                  seq: seq,
                  index: index,
                ),
            onOpenWorkspaceChange: (seq, index) =>
                conversation.client.openWorkspaceChangeOnDesktop(
                  sessionId: conversation.store.snapshot.sessionId!,
                  seq: seq,
                  index: index,
                ),
            showHeader: !widget.embedded,
            showComposer: widget.showComposer,
          );
    return widget.embedded ? content : Scaffold(body: content);
  }
}
