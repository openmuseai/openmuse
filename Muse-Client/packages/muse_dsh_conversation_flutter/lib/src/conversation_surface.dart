import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'dart:async';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

import 'native_component_registry.dart';
import 'official/deliverables/presented_files.dart';
import 'official/goal/goal_bar.dart';
import 'official/todo/todo_panel.dart';
import 'process_trail.dart';

typedef DshSendPrompt = Future<void> Function(String text, {String mode});
typedef DshQuestionAnswer = Future<void> Function(List<JsonMap> answers);
typedef DshWorkspaceChangesLoader =
    Future<DshNativeWorkspaceChanges> Function(int seq);
typedef DshWorkspaceChangePreviewLoader =
    Future<DshNativeArtifactPreview> Function(int seq, int index);
typedef DshWorkspaceChangeOpener = Future<void> Function(int seq, int index);

final class DshConversationSurface extends StatefulWidget {
  const DshConversationSurface({
    super.key,
    required this.store,
    required this.onSend,
    required this.onCancel,
    required this.onFallbackRequested,
    this.nativeContributions = const [],
    this.componentRegistry = const DshNativeComponentRegistry(),
    this.onNativeCommand,
    this.onQuestionAnswer,
    this.onLoadWorkspaceChanges,
    this.onLoadWorkspaceChangePreview,
    this.onOpenWorkspaceChange,
    this.onOpenPresentedFile,
    this.onReportResponse,
    this.showHeader = true,
    this.showComposer = true,
    this.compatibilityPlugins = const [],
  });

  final DshConversationStore store;
  final DshSendPrompt onSend;
  final Future<void> Function() onCancel;
  final VoidCallback onFallbackRequested;
  final List<DshNativeContribution> nativeContributions;
  final DshNativeComponentRegistry componentRegistry;
  final DshNativeCommandHandler? onNativeCommand;
  final DshQuestionAnswer? onQuestionAnswer;
  final DshWorkspaceChangesLoader? onLoadWorkspaceChanges;
  final DshWorkspaceChangePreviewLoader? onLoadWorkspaceChangePreview;
  final DshWorkspaceChangeOpener? onOpenWorkspaceChange;
  final DshPresentedFileOpener? onOpenPresentedFile;
  final ValueChanged<DshConversationRow>? onReportResponse;
  final bool showHeader;
  final bool showComposer;
  final List<JsonMap> compatibilityPlugins;

  @override
  State<DshConversationSurface> createState() => _DshConversationSurfaceState();
}

final class _DshConversationSurfaceState extends State<DshConversationSurface> {
  final TextEditingController _composer = TextEditingController();
  final ScrollController _scroll = ScrollController();
  StreamSubscription<void>? _changes;
  bool _sending = false;

  @override
  void initState() {
    super.initState();
    _changes = widget.store.changes.listen((_) {
      if (!mounted) return;
      setState(() {});
      WidgetsBinding.instance.addPostFrameCallback((_) => _scrollToEnd());
    });
  }

  @override
  void didUpdateWidget(covariant DshConversationSurface oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.store != widget.store) {
      _changes?.cancel();
      _changes = widget.store.changes.listen((_) {
        if (mounted) setState(() {});
      });
    }
  }

  @override
  void dispose() {
    _changes?.cancel();
    _composer.dispose();
    _scroll.dispose();
    super.dispose();
  }

  Future<void> _submit({String mode = 'queue'}) async {
    final text = _composer.text.trim();
    if (text.isEmpty || _sending) return;
    setState(() {
      _sending = true;
      _composer.clear();
    });
    try {
      await widget.onSend(text, mode: mode);
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  bool _foldsTool(DshConversationRow row) {
    final toolLike =
        row.kind == DshConversationRowKind.tool ||
        (row.kind == DshConversationRowKind.error && row.nativeKey != null);
    if (!toolLike || row.nativeKey == 'ask_user_question') return false;
    return !widget.nativeContributions.any(
      (value) =>
          value.slot == 'tool.call.toolview' && value.key == row.nativeKey,
    );
  }

  void _scrollToEnd() {
    if (!_scroll.hasClients) return;
    _scroll.animateTo(
      _scroll.position.maxScrollExtent,
      duration: const Duration(milliseconds: 180),
      curve: Curves.easeOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    final snapshot = widget.store.snapshot;
    return Material(
      color: Theme.of(context).colorScheme.surface,
      child: Column(
        children: [
          if (widget.showHeader)
            _SessionHeader(
              snapshot: snapshot,
              onFallbackRequested: widget.onFallbackRequested,
            ),
          _ConnectionBanner(snapshot: snapshot),
          if (widget.compatibilityPlugins.any(
            (value) => value['mode'] == 'incompatible',
          ))
            _PluginCompatibilityNotice(
              plugins: widget.compatibilityPlugins,
              onFallbackRequested: widget.onFallbackRequested,
            ),
          Expanded(
            child: snapshot.rows.isEmpty
                ? const _EmptyConversation()
                : Builder(
                    builder: (context) {
                      final entries = dshTimelineEntries(
                        snapshot.rows,
                        foldsTool: _foldsTool,
                      );
                      return ListView.builder(
                        key: const ValueKey('dsh-native.timeline'),
                        controller: _scroll,
                        padding: const EdgeInsets.fromLTRB(16, 30, 16, 18),
                        itemCount: entries.length,
                        itemBuilder: (context, index) {
                          final entry = entries[index];
                          return Align(
                            alignment: Alignment.topCenter,
                            child: ConstrainedBox(
                              constraints: const BoxConstraints(maxWidth: 720),
                              child: switch (entry) {
                                DshTimelineProcessEntry(:final moments) =>
                                  DshProcessTrail(moments: moments),
                                DshTimelineRowEntry(
                                  :final row,
                                  :final hideReasoning,
                                ) =>
                                  _ConversationRow(
                                    row: row,
                                    hideReasoning: hideReasoning,
                                    nativeContributions:
                                        widget.nativeContributions,
                                    componentRegistry: widget.componentRegistry,
                                    onNativeCommand: widget.onNativeCommand,
                                    onQuestionAnswer: widget.onQuestionAnswer,
                                    onLoadWorkspaceChanges:
                                        widget.onLoadWorkspaceChanges,
                                    onLoadWorkspaceChangePreview:
                                        widget.onLoadWorkspaceChangePreview,
                                    onOpenWorkspaceChange:
                                        widget.onOpenWorkspaceChange,
                                    onOpenPresentedFile:
                                        widget.onOpenPresentedFile,
                                    onReportResponse: widget.onReportResponse,
                                    onFallbackRequested:
                                        widget.onFallbackRequested,
                                  ),
                              },
                            ),
                          );
                        },
                      );
                    },
                  ),
          ),
          if (widget.showComposer)
            _Composer(
              controller: _composer,
              running: snapshot.running,
              sending: _sending,
              enabled: snapshot.phase == DshConnectionPhase.live,
              model: snapshot.model,
              reasoningEffort: snapshot.reasoningEffort,
              turnCount: snapshot.turnCount,
              stepCount: snapshot.stepCount,
              onSubmit: _submit,
              onCancel: widget.onCancel,
            ),
        ],
      ),
    );
  }
}

final class _SessionHeader extends StatelessWidget {
  const _SessionHeader({
    required this.snapshot,
    required this.onFallbackRequested,
  });

  final DshConversationSnapshot snapshot;
  final VoidCallback onFallbackRequested;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final title = snapshot.title?.trim();
    final preset = switch (snapshot.agentPreset) {
      'standard' => stringsOf(context).dshStandardMode(),
      final value? when value.isNotEmpty => value,
      _ => stringsOf(context).dshStandardMode(),
    };
    return Material(
      key: const ValueKey('dsh-native.header'),
      color: colors.surface,
      child: SafeArea(
        bottom: false,
        child: DecoratedBox(
          decoration: BoxDecoration(
            border: Border(bottom: BorderSide(color: colors.outlineVariant)),
          ),
          child: Column(
            children: [
              SizedBox(
                height: 48,
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Row(
                    children: [
                      Expanded(
                        child: Text(
                          title == null || title.isEmpty
                              ? stringsOf(context).dshConversation()
                              : title,
                          key: const ValueKey('dsh-native.session-title'),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            fontSize: 14,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Icon(
                        Icons.smart_toy_outlined,
                        size: 14,
                        color: colors.onSurfaceVariant,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        preset,
                        style: TextStyle(
                          fontSize: 12,
                          color: colors.onSurfaceVariant,
                        ),
                      ),
                      const SizedBox(width: 4),
                      IconButton(
                        key: const ValueKey('dsh-native.use-web'),
                        tooltip: stringsOf(context).dshFullFrontend(),
                        visualDensity: VisualDensity.compact,
                        onPressed: onFallbackRequested,
                        icon: const Icon(Icons.more_horiz, size: 20),
                      ),
                    ],
                  ),
                ),
              ),
              SizedBox(
                height: 36,
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Row(
                    children: [
                      _HeaderTab(
                        label: stringsOf(context).dshChat(),
                        selected: true,
                      ),
                      const SizedBox(width: 28),
                      _HeaderTab(
                        label: stringsOf(context).dshTrail(),
                        selected: false,
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _HeaderTab extends StatelessWidget {
  const _HeaderTab({required this.label, required this.selected});
  final String label;
  final bool selected;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Container(
      height: 36,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            width: 2,
            color: selected ? const Color(0xFF5375F6) : Colors.transparent,
          ),
        ),
      ),
      child: Text(
        label,
        style: TextStyle(
          fontSize: 13,
          color: selected ? const Color(0xFF4166E9) : colors.onSurfaceVariant,
        ),
      ),
    );
  }
}

final class _ConnectionBanner extends StatelessWidget {
  const _ConnectionBanner({required this.snapshot});
  final DshConversationSnapshot snapshot;

  @override
  Widget build(BuildContext context) {
    final visible =
        snapshot.phase == DshConnectionPhase.connecting ||
        snapshot.phase == DshConnectionPhase.reconnecting ||
        snapshot.phase == DshConnectionPhase.failed;
    if (!visible) return const SizedBox.shrink();
    final reconnecting = snapshot.phase == DshConnectionPhase.reconnecting;
    return Material(
      color: Theme.of(context).colorScheme.secondaryContainer,
      child: SafeArea(
        bottom: false,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: Row(
            children: [
              const SizedBox.square(
                dimension: 14,
                child: CircularProgressIndicator.adaptive(strokeWidth: 2),
              ),
              const SizedBox(width: 10),
              Expanded(
                child: Text(
                  reconnecting
                      ? stringsOf(context).dshReconnecting()
                      : stringsOf(context).dshConnecting(),
                  key: const ValueKey('dsh-native.connection-state'),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _EmptyConversation extends StatelessWidget {
  const _EmptyConversation();

  @override
  Widget build(BuildContext context) => Center(
    child: ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 340),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.explore_outlined, size: 40),
          SizedBox(height: 12),
          Text(
            stringsOf(context).dshExplore(),
            style: TextStyle(fontSize: 19, fontWeight: FontWeight.w600),
          ),
          SizedBox(height: 6),
          Text(
            stringsOf(context).dshSharedSession(),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    ),
  );
}

final class _ConversationRow extends StatelessWidget {
  const _ConversationRow({
    required this.row,
    required this.nativeContributions,
    required this.componentRegistry,
    this.hideReasoning = false,
    this.onNativeCommand,
    this.onQuestionAnswer,
    this.onLoadWorkspaceChanges,
    this.onLoadWorkspaceChangePreview,
    this.onOpenWorkspaceChange,
    this.onOpenPresentedFile,
    this.onReportResponse,
    required this.onFallbackRequested,
  });
  final DshConversationRow row;
  final bool hideReasoning;
  final List<DshNativeContribution> nativeContributions;
  final DshNativeComponentRegistry componentRegistry;
  final DshNativeCommandHandler? onNativeCommand;
  final DshQuestionAnswer? onQuestionAnswer;
  final DshWorkspaceChangesLoader? onLoadWorkspaceChanges;
  final DshWorkspaceChangePreviewLoader? onLoadWorkspaceChangePreview;
  final DshWorkspaceChangeOpener? onOpenWorkspaceChange;
  final DshPresentedFileOpener? onOpenPresentedFile;
  final ValueChanged<DshConversationRow>? onReportResponse;
  final VoidCallback onFallbackRequested;

  bool get _user =>
      row.kind == DshConversationRowKind.user ||
      row.kind == DshConversationRowKind.optimisticUser;

  @override
  Widget build(BuildContext context) {
    if (row.kind == DshConversationRowKind.incompatible) {
      return _ElementCompatibilityNotice(
        row: row,
        onFallbackRequested: onFallbackRequested,
      );
    }
    if (row.kind == DshConversationRowKind.error && row.nativeKey == null) {
      return _FailureSection(row: row);
    }
    if (row.nativeKey == 'todo/write') {
      final todos = row.nativeContext['todos'];
      return DshTodoPanel(
        todos: todos is List
            ? [
                for (final item in todos.whereType<Map>())
                  item.cast<String, Object?>(),
              ]
            : const [],
      );
    }
    if (row.nativeKey == 'goal/change') {
      final goal = row.nativeContext['goal'];
      if (goal is Map) {
        return DshGoalBar(goal: goal.cast<String, Object?>());
      }
    }
    if (row.kind == DshConversationRowKind.artifact &&
        row.nativeKey == 'workspace/changes') {
      return _WorkspaceChangesCard(
        row: row,
        load: onLoadWorkspaceChanges,
        preview: onLoadWorkspaceChangePreview,
        open: onOpenWorkspaceChange,
      );
    }
    if (row.kind == DshConversationRowKind.tool || row.nativeKey != null) {
      if (row.nativeKey == 'ask_user_question') {
        return _QuestionCard(row: row, onAnswer: onQuestionAnswer);
      }
      final native = nativeContributions.where(
        (value) =>
            value.slot == 'tool.call.toolview' && value.key == row.nativeKey,
      );
      if (native.isNotEmpty) {
        return Builder(
          builder: (context) => componentRegistry.buildContribution(
            context,
            native.first,
            DshNativeBindingContext(row.nativeContext),
            onCommand: onNativeCommand,
          ),
        );
      }
      return _ToolCard(row: row);
    }
    final colors = Theme.of(context).colorScheme;
    final dark = Theme.of(context).brightness == Brightness.dark;
    return Padding(
      key: ValueKey(row.key),
      padding: const EdgeInsets.only(bottom: 24),
      child: Column(
        crossAxisAlignment: _user
            ? CrossAxisAlignment.end
            : CrossAxisAlignment.start,
        children: [
          Align(
            alignment: _user ? Alignment.centerRight : Alignment.centerLeft,
            child: FractionallySizedBox(
              widthFactor: _user ? .84 : 1,
              child: DecoratedBox(
                key: ValueKey('dsh-native.message:${row.key}'),
                decoration: BoxDecoration(
                  color: _user
                      ? (dark
                            ? colors.primaryContainer.withValues(alpha: .42)
                            : const Color(0xFFF0F4FF))
                      : Colors.transparent,
                  borderRadius: BorderRadius.circular(17),
                ),
                child: Padding(
                  padding: _user
                      ? const EdgeInsets.symmetric(horizontal: 16, vertical: 11)
                      : EdgeInsets.zero,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      if (!hideReasoning && row.reasoning.isNotEmpty)
                        ExpansionTile(
                          tilePadding: EdgeInsets.zero,
                          childrenPadding: const EdgeInsets.only(bottom: 8),
                          dense: true,
                          title: Text(stringsOf(context).dshReasoning()),
                          children: [SelectableText(row.reasoning)],
                        ),
                      if (row.text.isNotEmpty)
                        _MessageBody(markdown: row.text, plain: _user),
                      if (!_user && row.nativeContext['presented'] is List)
                        DshPresentedFiles(
                          files: [
                            for (final item
                                in (row.nativeContext['presented'] as List)
                                    .whereType<Map>())
                              item.cast<String, Object?>(),
                          ],
                          onOpen: onOpenPresentedFile,
                        ),
                      if (row.pending || row.failed) ...[
                        const SizedBox(height: 6),
                        Text(
                          row.failed
                              ? stringsOf(
                                  context,
                                ).dshSendFailed(detail: row.detail ?? '')
                              : stringsOf(context).dshSending(),
                          style: Theme.of(context).textTheme.labelSmall
                              ?.copyWith(
                                color: row.failed
                                    ? colors.error
                                    : colors.onSurfaceVariant,
                              ),
                        ),
                      ],
                    ],
                  ),
                ),
              ),
            ),
          ),
          const SizedBox(height: 5),
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                _timeLabel(row.time),
                style: TextStyle(fontSize: 11, color: colors.onSurfaceVariant),
              ),
              const SizedBox(width: 6),
              InkResponse(
                key: ValueKey('dsh-native.copy:${row.key}'),
                radius: 16,
                onTap: row.text.isEmpty
                    ? null
                    : () => Clipboard.setData(ClipboardData(text: row.text)),
                child: Icon(
                  Icons.copy_outlined,
                  size: 14,
                  color: colors.onSurfaceVariant,
                ),
              ),
              if (row.kind == DshConversationRowKind.assistant &&
                  (row.text.isNotEmpty ||
                      (row.nativeContext['presented'] is List &&
                          (row.nativeContext['presented'] as List)
                              .isNotEmpty)) &&
                  onReportResponse != null)
                IconButton(
                  key: ValueKey('dsh-native.report:${row.key}'),
                  tooltip: stringsOf(context).commonAiReportTitle(),
                  onPressed: () => onReportResponse!(row),
                  icon: const Icon(Icons.flag_outlined, size: 18),
                ),
            ],
          ),
        ],
      ),
    );
  }
}

final class _MessageBody extends StatelessWidget {
  const _MessageBody({required this.markdown, this.plain = false});

  final String markdown;
  final bool plain;

  @override
  Widget build(BuildContext context) {
    final routing = ResourceLinkRouting.maybeOf(context);
    if (routing != null && markdown.contains('openmuse-resource://v1/')) {
      final pattern = RegExp(
        r'\[([^\]\n]+)\]\((openmuse-resource://v1/[A-Za-z0-9_-]+)\)',
      );
      final matches = pattern.allMatches(markdown).toList();
      if (matches.isNotEmpty) {
        final body = markdown
            .replaceAll(pattern, '')
            .replaceAll(
              RegExp(
                r'<resource-context digest="[a-f0-9]+">[\s\S]*?</resource-context>',
              ),
              '',
            )
            .trim();
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (body.isNotEmpty)
              SelectableText(
                body,
                style: const TextStyle(fontSize: 14, height: 1.55),
              ),
            for (final match in matches)
              Padding(
                padding: const EdgeInsets.only(top: 8),
                child: OutlinedButton.icon(
                  key: ValueKey('conversation-resource-${match.group(2)!}'),
                  icon: const Icon(Icons.link, size: 18),
                  label: Text(
                    match.group(1)!,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  onPressed: () => unawaited(routing.open(match.group(2)!)),
                ),
              ),
          ],
        );
      }
    }
    if (plain) {
      return SelectableText(
        markdown,
        style: const TextStyle(fontSize: 14, height: 1.55),
      );
    }
    final colors = Theme.of(context).colorScheme;
    return MarkdownBody(
      key: const ValueKey('dsh-native.markdown'),
      data: markdown,
      onTapLink: (text, href, title) {
        if (href != null && href.startsWith('openmuse-resource://v1/'))
          unawaited(routing?.open(href));
      },
      selectable: false,
      softLineBreak: true,
      styleSheet: MarkdownStyleSheet.fromTheme(Theme.of(context)).copyWith(
        p: const TextStyle(fontSize: 14, height: 1.55),
        listBullet: const TextStyle(fontSize: 14, height: 1.55),
        code: TextStyle(
          fontFamily: 'monospace',
          fontSize: 12.5,
          color: colors.onSurface,
          backgroundColor: colors.surfaceContainerHighest,
        ),
        codeblockDecoration: BoxDecoration(
          color: colors.surfaceContainerHighest,
          borderRadius: BorderRadius.circular(10),
        ),
        codeblockPadding: const EdgeInsets.all(12),
        blockquoteDecoration: BoxDecoration(
          color: colors.surfaceContainerLow,
          border: Border(left: BorderSide(color: colors.primary, width: 3)),
        ),
        blockquotePadding: const EdgeInsets.fromLTRB(12, 6, 8, 6),
      ),
    );
  }
}

final class _QuestionCard extends StatefulWidget {
  const _QuestionCard({required this.row, this.onAnswer});

  final DshConversationRow row;
  final DshQuestionAnswer? onAnswer;

  @override
  State<_QuestionCard> createState() => _QuestionCardState();
}

final class _QuestionCardState extends State<_QuestionCard> {
  final Map<String, Set<String>> _selected = {};
  final Map<String, TextEditingController> _custom = {};
  final Set<String> _skipped = {};
  bool _busy = false;
  bool _submitted = false;
  String? _error;

  List<Map> get _questions {
    final tool = widget.row.nativeContext['tool'];
    final arguments = tool is Map ? tool['arguments'] : null;
    final questions = arguments is Map ? arguments['questions'] : null;
    return questions is List
        ? questions.whereType<Map>().toList(growable: false)
        : const <Map>[];
  }

  String? _idOf(Map question) =>
      question['id'] is String ? question['id'] as String : null;

  @override
  void dispose() {
    for (final controller in _custom.values) {
      controller.dispose();
    }
    super.dispose();
  }

  bool get _canSubmit =>
      _questions.isNotEmpty &&
      _questions.every((question) {
        final id = _idOf(question);
        if (id == null) return false;
        return _skipped.contains(id) ||
            (_selected[id]?.isNotEmpty ?? false) ||
            (_custom[id]?.text.trim().isNotEmpty ?? false);
      });

  Future<void> _submit() async {
    final answer = widget.onAnswer;
    if (answer == null || !_canSubmit || _busy) return;
    final values = <JsonMap>[];
    for (final question in _questions) {
      final id = _idOf(question)!;
      final custom = _custom[id]?.text.trim() ?? '';
      final multi = question['multiSelect'] == true;
      final selected = _skipped.contains(id) || (!multi && custom.isNotEmpty)
          ? const <String>[]
          : (_selected[id] ?? const <String>{}).toList(growable: false);
      values.add({
        'id': id,
        'selected': selected,
        if (!_skipped.contains(id) && custom.isNotEmpty) 'custom': custom,
      });
    }
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      await answer(values);
      if (mounted) setState(() => _submitted = true);
    } on Object catch (error) {
      if (mounted) setState(() => _error = error.toString());
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final row = widget.row;
    final tool = row.nativeContext['tool'];
    final phase = tool is Map ? tool['phase'] : null;
    final result = tool is Map ? tool['result'] : null;
    final interactive =
        phase == 'call' && widget.onAnswer != null && !_submitted;
    final colors = Theme.of(context).colorScheme;
    return Card.outlined(
      key: ValueKey('dsh-native.question:${row.key}'),
      margin: const EdgeInsets.only(bottom: 16),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 14, 16, 14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Icon(Icons.help_outline, size: 19, color: colors.primary),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    stringsOf(context).dshAnswerRequired(),
                    style: TextStyle(fontWeight: FontWeight.w600),
                  ),
                ),
                Text(
                  _submitted
                      ? stringsOf(context).dshSubmitted()
                      : (phase == 'result'
                            ? stringsOf(context).dshAnswered()
                            : stringsOf(context).dshWaiting()),
                  style: TextStyle(
                    fontSize: 12,
                    color: colors.onSurfaceVariant,
                  ),
                ),
              ],
            ),
            for (final question in _questions) ...[
              const SizedBox(height: 14),
              if (question['header'] is String)
                Text(
                  question['header'] as String,
                  style: TextStyle(
                    fontSize: 12,
                    color: colors.onSurfaceVariant,
                  ),
                ),
              const SizedBox(height: 4),
              Text(
                question['question']?.toString() ??
                    stringsOf(context).dshSelect(),
                style: const TextStyle(
                  fontSize: 14,
                  height: 1.4,
                  fontWeight: FontWeight.w500,
                ),
              ),
              if (question['detail'] is String &&
                  (question['detail'] as String).isNotEmpty) ...[
                const SizedBox(height: 8),
                _MessageBody(markdown: question['detail'] as String),
              ],
              if (question['options'] is List) ...[
                const SizedBox(height: 8),
                for (final option
                    in (question['options'] as List).whereType<Map>())
                  _QuestionOption(
                    questionId: _idOf(question) ?? '',
                    option: option,
                    selected:
                        _selected[_idOf(question)]?.contains(option['label']) ==
                        true,
                    enabled: interactive && !_busy,
                    onTap: () {
                      final id = _idOf(question);
                      final label = option['label'];
                      if (id == null || label is! String) return;
                      setState(() {
                        _skipped.remove(id);
                        final values = _selected.putIfAbsent(
                          id,
                          () => <String>{},
                        );
                        if (question['multiSelect'] == true) {
                          if (!values.add(label)) values.remove(label);
                        } else {
                          values
                            ..clear()
                            ..add(label);
                          _custom[id]?.clear();
                        }
                      });
                    },
                  ),
              ],
              if (interactive) ...[
                const SizedBox(height: 6),
                TextField(
                  key: ValueKey(
                    'dsh-native.question.custom:${_idOf(question)}',
                  ),
                  controller: _custom.putIfAbsent(
                    _idOf(question) ?? '',
                    TextEditingController.new,
                  ),
                  maxLines: 3,
                  minLines: 1,
                  onChanged: (_) => setState(() {
                    final id = _idOf(question);
                    if (id != null) _skipped.remove(id);
                  }),
                  decoration: InputDecoration(
                    hintText: stringsOf(context).dshOtherAnswer(),
                    isDense: true,
                    border: OutlineInputBorder(),
                  ),
                ),
                Align(
                  alignment: Alignment.centerRight,
                  child: TextButton(
                    onPressed: _busy
                        ? null
                        : () => setState(() {
                            final id = _idOf(question);
                            if (id == null) return;
                            _selected[id]?.clear();
                            _custom[id]?.clear();
                            _skipped.add(id);
                          }),
                    child: Text(
                      _skipped.contains(_idOf(question))
                          ? stringsOf(context).dshSkipped()
                          : stringsOf(context).dshSkip(),
                    ),
                  ),
                ),
              ],
            ],
            if (_questions.isEmpty && row.text.isNotEmpty) ...[
              const SizedBox(height: 12),
              SelectableText(row.text),
            ],
            if (interactive) ...[
              if (_error != null) ...[
                const SizedBox(height: 8),
                Text(
                  _error!,
                  style: TextStyle(fontSize: 12, color: colors.error),
                ),
              ],
              const SizedBox(height: 8),
              FilledButton.icon(
                key: const ValueKey('dsh-native.question.submit'),
                onPressed: _canSubmit && !_busy ? _submit : null,
                icon: _busy
                    ? const SizedBox.square(
                        dimension: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : const Icon(Icons.check_rounded),
                label: Text(stringsOf(context).dshContinue()),
              ),
            ],
            if (_submitted)
              Padding(
                padding: EdgeInsets.only(top: 10),
                child: Text(stringsOf(context).dshAnswerSent()),
              ),
            if (phase == 'result' && result != null) ...[
              const SizedBox(height: 12),
              Divider(color: colors.outlineVariant),
              const SizedBox(height: 8),
              Text(
                stringsOf(context).dshAnswerResult(),
                style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant),
              ),
              const SizedBox(height: 4),
              SelectableText(_prettyNativeValue(result)),
            ],
          ],
        ),
      ),
    );
  }
}

final class _QuestionOption extends StatelessWidget {
  const _QuestionOption({
    required this.questionId,
    required this.option,
    required this.selected,
    required this.enabled,
    required this.onTap,
  });

  final String questionId;
  final Map option;
  final bool selected;
  final bool enabled;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final label = option['label']?.toString() ?? '';
    return Padding(
      padding: const EdgeInsets.only(bottom: 6),
      child: InkWell(
        key: ValueKey('dsh-native.question.option:$questionId:$label'),
        borderRadius: BorderRadius.circular(10),
        onTap: enabled ? onTap : null,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 120),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 9),
          decoration: BoxDecoration(
            color: selected
                ? colors.primaryContainer
                : colors.surfaceContainerHighest,
            borderRadius: BorderRadius.circular(10),
            border: Border.all(
              color: selected ? colors.primary : Colors.transparent,
            ),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      label,
                      style: const TextStyle(fontWeight: FontWeight.w500),
                    ),
                    if (option['description'] is String) ...[
                      const SizedBox(height: 2),
                      Text(
                        option['description'] as String,
                        style: TextStyle(
                          fontSize: 12,
                          color: colors.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ],
                ),
              ),
              if (selected)
                Icon(Icons.check_circle, size: 18, color: colors.primary),
            ],
          ),
        ),
      ),
    );
  }
}

final class _FailureSection extends StatelessWidget {
  const _FailureSection({required this.row});
  final DshConversationRow row;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    const error = Color(0xFFEF3E45);
    return Padding(
      key: ValueKey(row.key),
      padding: const EdgeInsets.only(bottom: 28),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Text(
            stringsOf(context).dshFailed(),
            style: TextStyle(fontSize: 13, color: colors.onSurfaceVariant),
          ),
          const SizedBox(height: 10),
          Divider(height: 1, color: colors.outlineVariant),
          const SizedBox(height: 14),
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Padding(
                padding: EdgeInsets.only(top: 7),
                child: Icon(Icons.circle, size: 7, color: error),
              ),
              const SizedBox(width: 10),
              Expanded(
                child: Wrap(
                  spacing: 6,
                  runSpacing: 2,
                  crossAxisAlignment: WrapCrossAlignment.center,
                  children: [
                    Text(
                      row.title ?? stringsOf(context).dshRunFailed(),
                      style: const TextStyle(
                        color: error,
                        fontSize: 13,
                        height: 1.55,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    Text(
                      row.text,
                      style: TextStyle(
                        height: 1.55,
                        fontSize: 13,
                        color: colors.onSurfaceVariant,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
          if (row.detail != null && row.detail!.isNotEmpty) ...[
            const SizedBox(height: 8),
            Align(
              alignment: Alignment.centerRight,
              child: Text(
                row.detail!,
                style: TextStyle(
                  fontFamily: 'monospace',
                  fontSize: 11,
                  color: colors.onSurfaceVariant,
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

final class _WorkspaceChangesCard extends StatefulWidget {
  const _WorkspaceChangesCard({
    required this.row,
    this.load,
    this.preview,
    this.open,
  });

  final DshConversationRow row;
  final DshWorkspaceChangesLoader? load;
  final DshWorkspaceChangePreviewLoader? preview;
  final DshWorkspaceChangeOpener? open;

  @override
  State<_WorkspaceChangesCard> createState() => _WorkspaceChangesCardState();
}

final class _WorkspaceChangesCardState extends State<_WorkspaceChangesCard> {
  DshNativeWorkspaceChanges? _summary;
  Object? _failure;
  int? _opening;

  int? get _seq {
    final value = widget.row.nativeContext['workspaceChanges'];
    return value is Map && value['seq'] is int ? value['seq'] as int : null;
  }

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    final load = widget.load;
    final seq = _seq;
    if (load == null || seq == null) return;
    try {
      final value = await load(seq);
      if (mounted) setState(() => _summary = value);
    } on Object catch (error) {
      if (mounted) setState(() => _failure = error);
    }
  }

  Future<void> _open(int index) async {
    final open = widget.open;
    final seq = _seq;
    if (open == null || seq == null || _opening != null) return;
    setState(() => _opening = index);
    try {
      await open(seq, index);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(stringsOf(context).dshOpenedDesktop())),
        );
      }
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(stringsOf(context).dshOpenError())),
        );
      }
    } finally {
      if (mounted) setState(() => _opening = null);
    }
  }

  void _preview(int index) {
    final load = widget.preview;
    final seq = _seq;
    final summary = _summary;
    if (load == null || seq == null || summary == null) return;
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => _ArtifactPreviewPage(
          files: summary.files,
          initialIndex: index,
          load: (target) => load(seq, target),
          onOpenDesktop: widget.open == null ? null : _open,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final summary = _summary;
    return Card.outlined(
      key: ValueKey('dsh-native.workspace-changes:${widget.row.key}'),
      margin: const EdgeInsets.only(bottom: 20),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(14, 12, 10, 10),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Icon(
                  Icons.folder_copy_outlined,
                  size: 20,
                  color: colors.primary,
                ),
                const SizedBox(width: 9),
                Expanded(
                  child: Text(
                    summary == null
                        ? stringsOf(context).dshArtifacts()
                        : stringsOf(
                            context,
                          ).dshUpdatedFiles(count: summary.total),
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                ),
                if (summary != null)
                  Text(
                    '+${summary.added}  −${summary.deleted}',
                    style: TextStyle(
                      fontSize: 12,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
              ],
            ),
            if (summary == null && _failure == null)
              const Padding(
                padding: EdgeInsets.symmetric(vertical: 16),
                child: Center(
                  child: SizedBox.square(
                    dimension: 18,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  ),
                ),
              ),
            if (_failure != null)
              Padding(
                padding: const EdgeInsets.fromLTRB(29, 10, 4, 4),
                child: Text(
                  stringsOf(context).dshSummaryUnavailable(),
                  style: TextStyle(
                    fontSize: 12,
                    color: colors.onSurfaceVariant,
                  ),
                ),
              ),
            if (summary != null)
              for (var index = 0; index < summary.files.length; index++)
                ListTile(
                  key: ValueKey('dsh-native.workspace-change:$index'),
                  dense: true,
                  minLeadingWidth: 20,
                  contentPadding: const EdgeInsets.only(left: 22, right: 0),
                  leading: const Icon(
                    Icons.insert_drive_file_outlined,
                    size: 18,
                  ),
                  title: Text(
                    summary.files[index].display,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(fontSize: 13),
                  ),
                  subtitle: Text(
                    summary.files[index].binary
                        ? stringsOf(context).dshBinary()
                        : summary.files[index].oversized
                        ? stringsOf(context).dshTooLarge()
                        : '+${summary.files[index].added}  −${summary.files[index].deleted}',
                    style: TextStyle(
                      fontSize: 11,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                  onTap: widget.preview == null ? null : () => _preview(index),
                  trailing: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (widget.preview != null)
                        const Icon(Icons.chevron_right_rounded, size: 20),
                      if (_opening == index)
                        const Padding(
                          padding: EdgeInsets.all(12),
                          child: SizedBox.square(
                            dimension: 18,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          ),
                        )
                      else
                        IconButton(
                          tooltip: stringsOf(context).dshOpenDesktop(),
                          onPressed: widget.open == null
                              ? null
                              : () => _open(index),
                          icon: const Icon(Icons.open_in_new_rounded, size: 19),
                        ),
                    ],
                  ),
                ),
          ],
        ),
      ),
    );
  }
}

final class _ArtifactPreviewPage extends StatefulWidget {
  const _ArtifactPreviewPage({
    required this.files,
    required this.initialIndex,
    required this.load,
    this.onOpenDesktop,
  });

  final List<DshNativeChangedFile> files;
  final int initialIndex;
  final Future<DshNativeArtifactPreview> Function(int index) load;
  final Future<void> Function(int index)? onOpenDesktop;

  @override
  State<_ArtifactPreviewPage> createState() => _ArtifactPreviewPageState();
}

final class _ArtifactPreviewPageState extends State<_ArtifactPreviewPage> {
  late Future<DshNativeArtifactPreview> _preview;
  late int _index;
  bool _rendered = false;
  _ArtifactDiffLayout? _layoutOverride;

  @override
  void initState() {
    super.initState();
    _index = widget.initialIndex;
    _preview = widget.load(_index);
  }

  void _retry() => setState(() => _preview = widget.load(_index));

  void _selectArtifact(int index) {
    if (index < 0 || index >= widget.files.length || index == _index) return;
    setState(() {
      _index = index;
      _rendered = false;
      _preview = widget.load(index);
    });
  }

  bool _isMarkdown(String path) {
    final lower = path.toLowerCase();
    return lower.endsWith('.md') || lower.endsWith('.mdx');
  }

  String _createdContent(DshNativeArtifactPreview preview) => preview.hunks
      .expand((hunk) => hunk.lines)
      .where((line) => line.isEmpty || !line.startsWith('-'))
      .map((line) => line.isEmpty ? '' : line.substring(1))
      .join('\n');

  @override
  Widget build(BuildContext context) {
    final defaultLayout = MediaQuery.sizeOf(context).width < 600
        ? _ArtifactDiffLayout.unified
        : _ArtifactDiffLayout.split;
    final layout = _layoutOverride ?? defaultLayout;
    return Scaffold(
      key: const ValueKey('dsh-native.artifact-preview'),
      appBar: AppBar(
        title: Text(stringsOf(context).dshPreviewTitle()),
        actions: [
          _ArtifactLayoutAction(
            key: const ValueKey('dsh-native.artifact-preview.layout-unified'),
            tooltip: stringsOf(context).dshUnifiedDiff(),
            selected: layout == _ArtifactDiffLayout.unified,
            icon: Icons.view_agenda_outlined,
            onPressed: () => setState(() {
              _rendered = false;
              _layoutOverride = _ArtifactDiffLayout.unified;
            }),
          ),
          _ArtifactLayoutAction(
            key: const ValueKey('dsh-native.artifact-preview.layout-split'),
            tooltip: stringsOf(context).dshSplitDiff(),
            selected: layout == _ArtifactDiffLayout.split,
            icon: Icons.view_column_outlined,
            onPressed: () => setState(() {
              _rendered = false;
              _layoutOverride = _ArtifactDiffLayout.split;
            }),
          ),
          if (widget.onOpenDesktop != null)
            IconButton(
              tooltip: stringsOf(context).dshOpenDesktop(),
              onPressed: () => widget.onOpenDesktop!(_index),
              icon: const Icon(Icons.open_in_new_rounded),
            ),
        ],
      ),
      body: FutureBuilder<DshNativeArtifactPreview>(
        future: _preview,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator.adaptive());
          }
          if (snapshot.hasError) {
            return _ArtifactPreviewUnavailable(
              message: stringsOf(context).dshChangesUnavailable(),
              onRetry: _retry,
            );
          }
          final preview = snapshot.requireData;
          if (preview.kind == DshNativeArtifactPreviewKind.binary) {
            return _ArtifactPreviewUnavailable(
              message: stringsOf(context).dshBinaryUnavailable(),
            );
          }
          if (preview.kind == DshNativeArtifactPreviewKind.oversized) {
            return _ArtifactPreviewUnavailable(
              message: stringsOf(context).dshSizeUnavailable(),
            );
          }
          final canRender = preview.isCreated && _isMarkdown(preview.display);
          return Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              _ArtifactPreviewHeader(
                preview: preview,
                file: widget.files[_index],
                index: _index,
                total: widget.files.length,
                onPrevious: _index == 0
                    ? null
                    : () => _selectArtifact(_index - 1),
                onNext: _index == widget.files.length - 1
                    ? null
                    : () => _selectArtifact(_index + 1),
              ),
              if (canRender)
                Padding(
                  padding: const EdgeInsets.fromLTRB(16, 10, 16, 8),
                  child: SegmentedButton<bool>(
                    segments: [
                      ButtonSegment(
                        value: false,
                        label: Text(stringsOf(context).dshChanges()),
                      ),
                      ButtonSegment(
                        value: true,
                        label: Text(stringsOf(context).dshPreview()),
                      ),
                    ],
                    selected: {_rendered},
                    onSelectionChanged: (value) {
                      setState(() => _rendered = value.single);
                    },
                  ),
                ),
              Expanded(
                child: _rendered && canRender
                    ? SingleChildScrollView(
                        key: const ValueKey(
                          'dsh-native.artifact-preview.markdown',
                        ),
                        padding: const EdgeInsets.fromLTRB(18, 10, 18, 32),
                        child: _MessageBody(markdown: _createdContent(preview)),
                      )
                    : _ArtifactDiffView(preview: preview, layout: layout),
              ),
            ],
          );
        },
      ),
    );
  }
}

enum _ArtifactDiffLayout { unified, split }

final class _ArtifactLayoutAction extends StatelessWidget {
  const _ArtifactLayoutAction({
    super.key,
    required this.tooltip,
    required this.selected,
    required this.icon,
    required this.onPressed,
  });

  final String tooltip;
  final bool selected;
  final IconData icon;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return IconButton(
      tooltip: tooltip,
      isSelected: selected,
      style: selected
          ? IconButton.styleFrom(
              backgroundColor: colors.secondaryContainer,
              foregroundColor: colors.onSecondaryContainer,
            )
          : null,
      onPressed: onPressed,
      icon: Icon(icon),
    );
  }
}

final class _ArtifactPreviewHeader extends StatelessWidget {
  const _ArtifactPreviewHeader({
    required this.preview,
    required this.file,
    required this.index,
    required this.total,
    required this.onPrevious,
    required this.onNext,
  });

  final DshNativeArtifactPreview preview;
  final DshNativeChangedFile file;
  final int index;
  final int total;
  final VoidCallback? onPrevious;
  final VoidCallback? onNext;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return DecoratedBox(
      decoration: BoxDecoration(
        color: colors.surfaceContainerLow,
        border: Border(bottom: BorderSide(color: colors.outlineVariant)),
      ),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 12, 16, 12),
        child: Row(
          children: [
            const Icon(Icons.insert_drive_file_outlined, size: 20),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    preview.display,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                  const SizedBox(height: 3),
                  Text(
                    preview.isCreated
                        ? stringsOf(context).dshNewFile()
                        : preview.isDeleted
                        ? stringsOf(context).dshDeletedFile()
                        : stringsOf(context).dshChangedFile(),
                    style: TextStyle(
                      fontSize: 12,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    '${index + 1} / $total    +${file.added}  −${file.deleted}',
                    style: TextStyle(
                      fontSize: 11,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
            ),
            IconButton(
              key: const ValueKey('dsh-native.artifact-preview.previous'),
              tooltip: stringsOf(context).dshPreviousFile(),
              onPressed: onPrevious,
              icon: const Icon(Icons.chevron_left_rounded),
            ),
            IconButton(
              key: const ValueKey('dsh-native.artifact-preview.next'),
              tooltip: stringsOf(context).dshNextFile(),
              onPressed: onNext,
              icon: const Icon(Icons.chevron_right_rounded),
            ),
          ],
        ),
      ),
    );
  }
}

final class _ArtifactPreviewUnavailable extends StatelessWidget {
  const _ArtifactPreviewUnavailable({required this.message, this.onRetry});

  final String message;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) => Center(
    child: Padding(
      padding: const EdgeInsets.all(32),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const Icon(Icons.preview_outlined, size: 36),
          const SizedBox(height: 12),
          Text(message, textAlign: TextAlign.center),
          if (onRetry != null) ...[
            const SizedBox(height: 14),
            OutlinedButton(
              onPressed: onRetry,
              child: Text(stringsOf(context).dshRetry()),
            ),
          ],
        ],
      ),
    ),
  );
}

final class _ArtifactDiffLine {
  const _ArtifactDiffLine({
    required this.text,
    this.oldNumber,
    this.newNumber,
    this.kind = 'context',
    this.header = false,
  });

  final String text;
  final int? oldNumber;
  final int? newNumber;
  final String kind;
  final bool header;
}

final class _ArtifactSplitRow {
  const _ArtifactSplitRow({this.oldLine, this.newLine, this.header});

  final _ArtifactDiffLine? oldLine;
  final _ArtifactDiffLine? newLine;
  final String? header;
}

final class _ArtifactDiffView extends StatelessWidget {
  const _ArtifactDiffView({required this.preview, required this.layout});

  final DshNativeArtifactPreview preview;
  final _ArtifactDiffLayout layout;

  List<_ArtifactDiffLine> get _lines {
    final result = <_ArtifactDiffLine>[];
    for (final hunk in preview.hunks) {
      result.add(
        _ArtifactDiffLine(
          text:
              '@@ -${hunk.oldStart},${hunk.oldLines} +${hunk.newStart},${hunk.newLines} @@',
          header: true,
        ),
      );
      var oldNumber = hunk.oldStart;
      var newNumber = hunk.newStart;
      for (final source in hunk.lines) {
        final prefix = source.isEmpty ? ' ' : source[0];
        final text = source.isEmpty ? '' : source.substring(1);
        switch (prefix) {
          case '+':
            result.add(
              _ArtifactDiffLine(
                text: text,
                newNumber: newNumber++,
                kind: 'add',
              ),
            );
          case '-':
            result.add(
              _ArtifactDiffLine(
                text: text,
                oldNumber: oldNumber++,
                kind: 'delete',
              ),
            );
          default:
            result.add(
              _ArtifactDiffLine(
                text: text,
                oldNumber: oldNumber++,
                newNumber: newNumber++,
              ),
            );
        }
      }
    }
    return result;
  }

  List<_ArtifactSplitRow> get _splitRows {
    final result = <_ArtifactSplitRow>[];
    final pendingOld = <_ArtifactDiffLine>[];
    final pendingNew = <_ArtifactDiffLine>[];

    void flushChanges() {
      final count = pendingOld.length > pendingNew.length
          ? pendingOld.length
          : pendingNew.length;
      for (var index = 0; index < count; index++) {
        result.add(
          _ArtifactSplitRow(
            oldLine: index < pendingOld.length ? pendingOld[index] : null,
            newLine: index < pendingNew.length ? pendingNew[index] : null,
          ),
        );
      }
      pendingOld.clear();
      pendingNew.clear();
    }

    for (final line in _lines) {
      if (line.header) {
        flushChanges();
        result.add(_ArtifactSplitRow(header: line.text));
      } else if (line.kind == 'delete') {
        pendingOld.add(line);
      } else if (line.kind == 'add') {
        pendingNew.add(line);
      } else {
        flushChanges();
        result.add(_ArtifactSplitRow(oldLine: line, newLine: line));
      }
    }
    flushChanges();
    return result;
  }

  @override
  Widget build(BuildContext context) {
    return switch (layout) {
      _ArtifactDiffLayout.unified => _buildUnified(context),
      _ArtifactDiffLayout.split => _buildSplit(context),
    };
  }

  Widget _buildUnified(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final addBackground = Color.alphaBlend(
      const Color(0xFF22C55E).withValues(alpha: 0.18),
      colors.surface,
    );
    final deleteBackground = Color.alphaBlend(
      colors.error.withValues(alpha: 0.18),
      colors.surface,
    );
    final lines = _lines;
    return ListView.builder(
      key: const ValueKey('dsh-native.artifact-preview.unified'),
      padding: const EdgeInsets.only(bottom: 32),
      itemCount: lines.length,
      itemBuilder: (context, index) {
        final line = lines[index];
        if (line.header) {
          return Container(
            color: colors.surfaceContainerHighest,
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
            child: Text(
              line.text,
              style: TextStyle(
                fontFamily: 'monospace',
                fontSize: 11,
                color: colors.onSurfaceVariant,
              ),
            ),
          );
        }
        final background = switch (line.kind) {
          'add' => addBackground,
          'delete' => deleteBackground,
          _ => Colors.transparent,
        };
        final markerColor = switch (line.kind) {
          'add' => const Color(0xFF34D17A),
          'delete' => const Color(0xFFFF5555),
          _ => colors.onSurfaceVariant,
        };
        final number = line.kind == 'delete'
            ? line.oldNumber
            : line.newNumber ?? line.oldNumber;
        return Container(
          color: background,
          constraints: const BoxConstraints(minHeight: 25),
          padding: const EdgeInsets.symmetric(vertical: 3),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                width: 46,
                child: Text(
                  number?.toString() ?? '',
                  textAlign: TextAlign.right,
                  style: _lineNumberStyle(colors).copyWith(color: markerColor),
                ),
              ),
              SizedBox(
                width: 24,
                child: Text(
                  switch (line.kind) {
                    'add' => '+',
                    'delete' => '−',
                    _ => ' ',
                  },
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    fontFamily: 'monospace',
                    fontWeight: FontWeight.w700,
                    color: markerColor,
                  ),
                ),
              ),
              const SizedBox(width: 4),
              Expanded(
                child: SelectableText(
                  line.text,
                  style: const TextStyle(
                    fontFamily: 'monospace',
                    fontSize: 12.5,
                    height: 1.45,
                  ),
                ),
              ),
              const SizedBox(width: 12),
            ],
          ),
        );
      },
    );
  }

  Widget _buildSplit(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final rows = _splitRows;
    return ListView.builder(
      key: const ValueKey('dsh-native.artifact-preview.split'),
      padding: const EdgeInsets.only(bottom: 32),
      itemCount: rows.length,
      itemBuilder: (context, index) {
        final row = rows[index];
        if (row.header != null) {
          return Row(
            children: [
              Expanded(child: _splitHeader(colors, row.header!)),
              VerticalDivider(width: 1, color: colors.outlineVariant),
              Expanded(child: _splitHeader(colors, row.header!)),
            ],
          );
        }
        return IntrinsicHeight(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Expanded(child: _SplitDiffCell(line: row.oldLine, oldSide: true)),
              VerticalDivider(width: 1, color: colors.outlineVariant),
              Expanded(
                child: _SplitDiffCell(line: row.newLine, oldSide: false),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _splitHeader(ColorScheme colors, String text) => Container(
    color: colors.surfaceContainerHighest,
    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
    child: Text(
      text,
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(
        fontFamily: 'monospace',
        fontSize: 11,
        color: colors.onSurfaceVariant,
      ),
    ),
  );

  TextStyle _lineNumberStyle(ColorScheme colors) => TextStyle(
    fontFamily: 'monospace',
    fontSize: 11,
    height: 1.55,
    color: colors.onSurfaceVariant,
  );
}

final class _SplitDiffCell extends StatelessWidget {
  const _SplitDiffCell({required this.line, required this.oldSide});

  final _ArtifactDiffLine? line;
  final bool oldSide;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final value = line;
    final changed =
        value != null &&
        (oldSide ? value.kind == 'delete' : value.kind == 'add');
    final color = oldSide ? const Color(0xFFFF5555) : const Color(0xFF34D17A);
    final changedBackground = Color.alphaBlend(
      (oldSide ? colors.error : const Color(0xFF22C55E)).withValues(
        alpha: 0.18,
      ),
      colors.surface,
    );
    final background = value == null
        ? colors.surfaceContainerLow
        : changed
        ? changedBackground
        : Colors.transparent;
    final number = value == null
        ? null
        : oldSide
        ? value.oldNumber
        : value.newNumber;
    return Container(
      color: background,
      constraints: const BoxConstraints(minHeight: 29),
      padding: const EdgeInsets.symmetric(vertical: 3),
      child: value == null
          ? const SizedBox.shrink()
          : Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                SizedBox(
                  width: 44,
                  child: Text(
                    number?.toString() ?? '',
                    textAlign: TextAlign.right,
                    style: TextStyle(
                      fontFamily: 'monospace',
                      fontSize: 11,
                      color: changed ? color : colors.onSurfaceVariant,
                    ),
                  ),
                ),
                SizedBox(
                  width: 22,
                  child: Text(
                    changed ? (oldSide ? '−' : '+') : ' ',
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontFamily: 'monospace',
                      fontWeight: FontWeight.w700,
                      color: color,
                    ),
                  ),
                ),
                Expanded(
                  child: SelectableText(
                    value.text,
                    style: const TextStyle(
                      fontFamily: 'monospace',
                      fontSize: 12.5,
                      height: 1.45,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
              ],
            ),
    );
  }
}

final class _ToolCard extends StatefulWidget {
  const _ToolCard({required this.row});
  final DshConversationRow row;

  @override
  State<_ToolCard> createState() => _ToolCardState();
}

final class _ToolCardState extends State<_ToolCard> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    final row = widget.row;
    final tool = row.nativeContext['tool'];
    final arguments = tool is Map ? tool['arguments'] : null;
    final result = tool is Map ? tool['result'] : null;
    final phase = tool is Map ? tool['phase'] : null;
    final colors = Theme.of(context).colorScheme;
    return Padding(
      key: ValueKey(row.key),
      padding: const EdgeInsets.only(bottom: 12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Material(
            color: Colors.transparent,
            child: InkWell(
              key: ValueKey('dsh-native.tool.toggle:${row.key}'),
              borderRadius: BorderRadius.circular(8),
              onTap: () => setState(() => _expanded = !_expanded),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 2, vertical: 7),
                child: Row(
                  children: [
                    Icon(
                      row.failed
                          ? Icons.error_outline
                          : _toolIcon(row.nativeKey),
                      size: 18,
                      color: row.failed
                          ? colors.error
                          : colors.onSurfaceVariant,
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        row.title ?? stringsOf(context).dshToolCall(),
                        style: const TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ),
                    if (phase == 'call')
                      const Padding(
                        padding: EdgeInsets.only(right: 8),
                        child: SizedBox.square(
                          dimension: 14,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        ),
                      ),
                    AnimatedRotation(
                      turns: _expanded ? .25 : 0,
                      duration: const Duration(milliseconds: 150),
                      child: Icon(
                        Icons.chevron_right_rounded,
                        size: 20,
                        color: colors.onSurfaceVariant,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          if (_expanded)
            Padding(
              key: ValueKey('dsh-native.tool.details:${row.key}'),
              padding: const EdgeInsets.fromLTRB(28, 2, 8, 10),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  if (arguments != null)
                    _ToolPayload(
                      label: stringsOf(context).dshInput(),
                      value: arguments,
                    ),
                  if (result != null)
                    _ToolPayload(
                      label: stringsOf(context).dshOutput(),
                      value: result,
                    )
                  else if (arguments == null && row.text.isNotEmpty)
                    _ToolPayload(
                      label: phase == 'result'
                          ? stringsOf(context).dshOutput()
                          : stringsOf(context).dshInput(),
                      value: row.text,
                    ),
                ],
              ),
            ),
        ],
      ),
    );
  }
}

IconData _toolIcon(String? name) => switch (name) {
  'read' || 'read_image' => Icons.folder_open_outlined,
  'write' || 'edit' || 'str_replace_editor' => Icons.edit_note_outlined,
  'glob' || 'grep' => Icons.search_rounded,
  'web_search' || 'web_fetch' => Icons.language_rounded,
  _ => Icons.terminal_rounded,
};

final class _ToolPayload extends StatelessWidget {
  const _ToolPayload({required this.label, required this.value});

  final String label;
  final Object value;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.only(top: 8),
    child: Align(
      alignment: Alignment.centerLeft,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: Theme.of(context).textTheme.labelMedium),
          const SizedBox(height: 4),
          SelectableText(
            _prettyNativeValue(value),
            style: const TextStyle(fontFamily: 'monospace', fontSize: 12),
          ),
        ],
      ),
    ),
  );
}

String _prettyNativeValue(Object value) {
  if (value is String) return value;
  try {
    return const JsonEncoder.withIndent('  ').convert(value);
  } on Object {
    return value.toString();
  }
}

final class _Composer extends StatelessWidget {
  const _Composer({
    required this.controller,
    required this.running,
    required this.sending,
    required this.enabled,
    required this.model,
    required this.reasoningEffort,
    required this.turnCount,
    required this.stepCount,
    required this.onSubmit,
    required this.onCancel,
  });

  final TextEditingController controller;
  final bool running;
  final bool sending;
  final bool enabled;
  final String? model;
  final String? reasoningEffort;
  final int turnCount;
  final int stepCount;
  final Future<void> Function({String mode}) onSubmit;
  final Future<void> Function() onCancel;

  @override
  Widget build(BuildContext context) => Material(
    color: Theme.of(context).colorScheme.surface,
    child: SafeArea(
      top: false,
      child: Align(
        alignment: Alignment.center,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 720),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(12, 8, 12, 8),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                DecoratedBox(
                  key: const ValueKey('dsh-native.composer-shell'),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface,
                    borderRadius: BorderRadius.circular(20),
                    border: Border.all(
                      color: Theme.of(context).colorScheme.outlineVariant,
                    ),
                    boxShadow: const [
                      BoxShadow(
                        color: Color(0x16000000),
                        blurRadius: 18,
                        offset: Offset(0, 5),
                      ),
                    ],
                  ),
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(14, 8, 8, 8),
                    child: Column(
                      children: [
                        TextField(
                          key: const ValueKey('dsh-native.composer'),
                          controller: controller,
                          enabled: enabled && !sending,
                          minLines: 1,
                          maxLines: 7,
                          textInputAction: TextInputAction.newline,
                          decoration: InputDecoration(
                            hintText: enabled
                                ? stringsOf(context).dshComposer()
                                : stringsOf(context).dshWaitConnection(),
                            border: InputBorder.none,
                            isDense: true,
                            contentPadding: const EdgeInsets.symmetric(
                              vertical: 9,
                            ),
                          ),
                          onSubmitted: (_) => onSubmit(),
                        ),
                        const SizedBox(height: 4),
                        Row(
                          children: [
                            _ComposerControl(
                              icon: Icons.add,
                              tooltip: stringsOf(context).dshAddContent(),
                              onPressed: enabled ? () {} : null,
                            ),
                            const SizedBox(width: 4),
                            Icon(
                              Icons.admin_panel_settings_outlined,
                              size: 16,
                              color: Theme.of(
                                context,
                              ).colorScheme.onSurfaceVariant,
                            ),
                            const SizedBox(width: 4),
                            Flexible(
                              child: Text(
                                stringsOf(context).dshWorkspaceEdits(),
                                overflow: TextOverflow.ellipsis,
                                style: TextStyle(
                                  fontSize: 12,
                                  color: Theme.of(
                                    context,
                                  ).colorScheme.onSurfaceVariant,
                                ),
                              ),
                            ),
                            const Spacer(),
                            Flexible(
                              child: Text(
                                _modelLabel(context, model, reasoningEffort),
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                                style: TextStyle(
                                  fontSize: 11,
                                  color: Theme.of(
                                    context,
                                  ).colorScheme.onSurfaceVariant,
                                ),
                              ),
                            ),
                            const SizedBox(width: 6),
                            running
                                ? IconButton.filledTonal(
                                    key: const ValueKey('dsh-native.stop'),
                                    tooltip: stringsOf(context).dshStop(),
                                    onPressed: onCancel,
                                    icon: const Icon(Icons.stop_rounded),
                                  )
                                : IconButton.filled(
                                    key: const ValueKey('dsh-native.send'),
                                    tooltip: stringsOf(context).dshSend(),
                                    onPressed: enabled && !sending
                                        ? onSubmit
                                        : null,
                                    style: IconButton.styleFrom(
                                      backgroundColor: const Color(0xFF94A9FF),
                                      foregroundColor: Colors.white,
                                    ),
                                    icon: const Icon(
                                      Icons.arrow_upward_rounded,
                                    ),
                                  ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 5),
                Text(
                  stringsOf(
                    context,
                  ).dshTurnSteps(turns: turnCount, steps: stepCount),
                  key: const ValueKey('dsh-native.session-stats'),
                  style: TextStyle(
                    fontSize: 11,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    ),
  );
}

final class _ComposerControl extends StatelessWidget {
  const _ComposerControl({
    required this.icon,
    required this.tooltip,
    required this.onPressed,
  });
  final IconData icon;
  final String tooltip;
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) => IconButton(
    tooltip: tooltip,
    visualDensity: VisualDensity.compact,
    onPressed: onPressed,
    style: IconButton.styleFrom(
      backgroundColor: Theme.of(context).colorScheme.surfaceContainerLow,
    ),
    icon: Icon(icon, size: 18),
  );
}

String _timeLabel(int milliseconds) {
  final value = DateTime.fromMillisecondsSinceEpoch(milliseconds).toLocal();
  String two(int number) => number.toString().padLeft(2, '0');
  return '${two(value.hour)}:${two(value.minute)}';
}

String _modelLabel(BuildContext context, String? model, String? effort) {
  final modelLabel = switch (model) {
    'deepseek-flash' => 'DeepSeek-V41-Flash',
    final value? when value.isNotEmpty => value,
    _ => stringsOf(context).dshSelectModel(),
  };
  final effortLabel = switch (effort) {
    'high' => 'High',
    'medium' => 'Medium',
    'low' => 'Low',
    final value? when value.isNotEmpty => value,
    _ => null,
  };
  return effortLabel == null ? modelLabel : '$modelLabel · $effortLabel';
}

final class _ElementCompatibilityNotice extends StatelessWidget {
  const _ElementCompatibilityNotice({
    required this.row,
    required this.onFallbackRequested,
  });
  final DshConversationRow row;
  final VoidCallback onFallbackRequested;

  @override
  Widget build(BuildContext context) => Padding(
    key: ValueKey(row.key),
    padding: const EdgeInsets.only(bottom: 14),
    child: Card.outlined(
      color: Theme.of(context).colorScheme.surfaceContainerLow,
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Icon(Icons.extension_off_outlined, size: 22),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    row.title ?? stringsOf(context).dshUnsupported(),
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                  const SizedBox(height: 3),
                  Text(row.text, style: Theme.of(context).textTheme.bodySmall),
                  if (row.detail case final detail?) ...[
                    const SizedBox(height: 3),
                    Text(detail, style: Theme.of(context).textTheme.bodySmall),
                  ],
                  const SizedBox(height: 8),
                  TextButton.icon(
                    key: ValueKey('dsh-native.open-web:${row.key}'),
                    onPressed: onFallbackRequested,
                    icon: const Icon(Icons.open_in_browser, size: 16),
                    label: Text(stringsOf(context).dshWebFallback()),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

final class _PluginCompatibilityNotice extends StatelessWidget {
  const _PluginCompatibilityNotice({
    required this.plugins,
    required this.onFallbackRequested,
  });
  final List<JsonMap> plugins;
  final VoidCallback onFallbackRequested;

  @override
  Widget build(BuildContext context) {
    final names = plugins
        .where((value) => value['mode'] == 'incompatible')
        .map((value) => value['pluginId'])
        .whereType<String>()
        .join('、');
    return Material(
      key: const ValueKey('dsh-native.plugin-compatibility'),
      color: Theme.of(context).colorScheme.surfaceContainerLow,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(14, 8, 8, 8),
        child: Row(
          children: [
            const Icon(Icons.info_outline, size: 17),
            const SizedBox(width: 8),
            Expanded(
              child: Text(
                stringsOf(context).dshUnsupportedNames(names: names),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ),
            TextButton(
              key: const ValueKey('dsh-native.open-web-fallback'),
              onPressed: onFallbackRequested,
              child: Text(stringsOf(context).dshCompatibility()),
            ),
          ],
        ),
      ),
    );
  }
}
