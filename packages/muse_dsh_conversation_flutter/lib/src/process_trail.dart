import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';

/// One visible timeline slot: a normal row, or a collapsed run of tool calls
/// and reasoning.
sealed class DshTimelineEntry {
  const DshTimelineEntry();
}

final class DshTimelineRowEntry extends DshTimelineEntry {
  const DshTimelineRowEntry(this.row, {this.hideReasoning = false});

  final DshConversationRow row;
  final bool hideReasoning;
}

final class DshTimelineProcessEntry extends DshTimelineEntry {
  const DshTimelineProcessEntry(this.moments);

  final List<DshProcessMoment> moments;
}

final class DshProcessMoment {
  DshProcessMoment.tool(DshConversationRow tool)
    : row = tool,
      reasoning = null,
      pendingThinking = false,
      key = tool.key;

  const DshProcessMoment.thinking({
    required this.key,
    required this.reasoning,
    required this.pendingThinking,
  }) : row = null;

  final DshConversationRow? row;
  final String? reasoning;
  final bool pendingThinking;
  final String key;

  bool get isRunningTool {
    final value = row;
    if (value == null) return false;
    final tool = value.nativeContext['tool'];
    final phase = tool is Map ? tool['phase'] : null;
    return phase == 'call' || value.pending;
  }
}

List<DshTimelineEntry> dshTimelineEntries(
  List<DshConversationRow> rows, {
  required bool Function(DshConversationRow row) foldsTool,
}) {
  final entries = <DshTimelineEntry>[];
  final moments = <DshProcessMoment>[];

  void flush() {
    if (moments.isEmpty) return;
    final only = moments.length == 1 ? moments.single : null;
    if (only != null && only.row != null && !only.isRunningTool) {
      entries.add(DshTimelineRowEntry(only.row!));
    } else {
      entries.add(DshTimelineProcessEntry(List<DshProcessMoment>.of(moments)));
    }
    moments.clear();
  }

  for (final row in rows) {
    if (foldsTool(row)) {
      moments.add(DshProcessMoment.tool(row));
      continue;
    }
    if (row.kind == DshConversationRowKind.liveAssistant && row.text.isEmpty) {
      moments.add(
        DshProcessMoment.thinking(
          key: row.key,
          reasoning: row.reasoning,
          pendingThinking: true,
        ),
      );
      continue;
    }
    if (row.kind == DshConversationRowKind.assistant ||
        row.kind == DshConversationRowKind.liveAssistant) {
      if (row.reasoning.isNotEmpty) {
        moments.add(
          DshProcessMoment.thinking(
            key: '${row.key}:reasoning',
            reasoning: row.reasoning,
            pendingThinking: row.kind == DshConversationRowKind.liveAssistant,
          ),
        );
      }
      flush();
      final presented = row.nativeContext['presented'];
      final hasBody =
          row.text.isNotEmpty ||
          (presented is List && presented.isNotEmpty) ||
          row.pending ||
          row.failed;
      if (hasBody) {
        entries.add(
          DshTimelineRowEntry(row, hideReasoning: row.reasoning.isNotEmpty),
        );
      }
      continue;
    }
    flush();
    entries.add(DshTimelineRowEntry(row));
  }
  flush();
  return entries;
}

String dshProcessSummary(
  List<DshProcessMoment> moments, {
  OpenMuseStrings? strings,
}) {
  strings ??= OpenMuseStrings.fallback;
  final phrases = <String>[];
  var hasTool = false;
  var thinking = false;
  for (final moment in moments) {
    final row = moment.row;
    if (row != null) {
      hasTool = true;
      final phrase = _categoryPhrase(row.nativeKey, strings);
      if (phrase != null && !phrases.contains(phrase)) phrases.add(phrase);
    } else {
      thinking = true;
    }
  }
  if (!hasTool)
    return thinking ? strings.dshReasoning() : strings.dshTaskProgress();
  return strings.dshToolsSummary(tools: phrases.join(', '));
}

String? _categoryPhrase(String? name, OpenMuseStrings strings) =>
    switch (name) {
      'read' || 'read_image' => strings.dshReadFiles(),
      'write' => strings.dshWriteFiles(),
      'edit' || 'str_replace_editor' => strings.dshEditFiles(),
      'bash' || 'pwsh' || 'terminal_send' || 'shell' => strings.dshCommands(),
      'glob' || 'grep' => strings.dshSearchFiles(),
      'web_search' => strings.dshSearchWeb(),
      'web_fetch' => strings.dshFetchWeb(),
      'present' => strings.dshPresentFiles(),
      _ => null,
    };

String dshProcessActivityLabel(
  DshConversationRow row, {
  OpenMuseStrings? strings,
}) {
  strings ??= OpenMuseStrings.fallback;
  final arguments = _toolArguments(row);
  final running = DshProcessMoment.tool(row).isRunningTool;
  final name = row.nativeKey;
  if (_isShell(name)) {
    final command = _commandOf(arguments);
    final verb = row.failed
        ? strings.dshRunFailed()
        : running
        ? strings.dshRunning()
        : strings.dshRan();
    if (command == null) return verb;
    return '$verb ${_oneLine(command)}';
  }
  final path = _argumentPath(arguments);
  if (path != null) {
    final base = path.split(RegExp(r'[\\/]')).where((part) => part.isNotEmpty);
    final file = base.isEmpty ? path : base.last;
    final verb = switch (name) {
      'read' || 'read_image' =>
        row.failed
            ? strings.dshReadFailed()
            : running
            ? strings.dshReading()
            : strings.dshRead(),
      'write' =>
        row.failed
            ? strings.dshWriteFailed()
            : running
            ? strings.dshWriting()
            : strings.dshWritten(),
      'edit' || 'str_replace_editor' =>
        row.failed
            ? strings.dshEditFailed()
            : running
            ? strings.dshEditing()
            : strings.dshEdited(),
      _ => null,
    };
    if (verb != null) return '$verb $file';
  }
  return row.title ?? strings.dshToolCall();
}

String? _argumentPath(Object? arguments) {
  if (arguments is! Map) return null;
  for (final key in ['path', 'file_path', 'filePath', 'target_file']) {
    final value = arguments[key];
    if (value is String && value.trim().isNotEmpty) return value.trim();
  }
  return null;
}

bool _isShell(String? name) => switch (name) {
  'bash' || 'pwsh' || 'terminal_send' || 'shell' => true,
  _ => false,
};

Object? _toolArguments(DshConversationRow row) {
  final tool = row.nativeContext['tool'];
  return tool is Map ? tool['arguments'] : null;
}

Object? _toolResult(DshConversationRow row) {
  final tool = row.nativeContext['tool'];
  return tool is Map ? tool['result'] : null;
}

String? _commandOf(Object? arguments) {
  if (arguments is String && arguments.trim().isNotEmpty) {
    return arguments.trim();
  }
  if (arguments is! Map) return null;
  for (final key in ['command', 'cmd', 'script']) {
    final value = arguments[key];
    if (value is String && value.trim().isNotEmpty) return value.trim();
  }
  return null;
}

String? _resultText(Object? result) {
  if (result == null) return null;
  final text = result is String ? result.trim() : result.toString().trim();
  return text.isEmpty ? null : text;
}

String _oneLine(String value) => value.replaceAll(RegExp(r'\s+'), ' ').trim();

IconData _momentIcon(String? name) => switch (name) {
  'read' || 'read_image' => Icons.description_outlined,
  'bash' || 'pwsh' || 'terminal_send' || 'shell' => Icons.terminal_outlined,
  'write' || 'edit' || 'str_replace_editor' => Icons.edit_outlined,
  'glob' || 'grep' || 'web_search' => Icons.search_rounded,
  _ => Icons.build_outlined,
};

/// Collapsed run of consecutive tool calls and reasoning. Expanded by tap.
///
/// The opened list stays within half the screen and scrolls inside that bound.
final class DshProcessTrail extends StatefulWidget {
  const DshProcessTrail({super.key, required this.moments});

  final List<DshProcessMoment> moments;

  @override
  State<DshProcessTrail> createState() => _DshProcessTrailState();
}

final class _DshProcessTrailState extends State<DshProcessTrail> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    final muted = Theme.of(context).colorScheme.onSurfaceVariant;
    final anchor = widget.moments.first.key;
    final halfScreen = MediaQuery.sizeOf(context).height / 2;
    final listMaxHeight = (halfScreen - 48).clamp(80.0, halfScreen);
    return Padding(
      key: ValueKey('dsh-native.process:$anchor'),
      padding: const EdgeInsets.only(bottom: 12),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _TrailLine(
            key: ValueKey('dsh-native.process.toggle:$anchor'),
            icon: true,
            label: dshProcessSummary(
              widget.moments,
              strings: stringsOf(context),
            ),
            muted: muted,
            expanded: _expanded,
            onTap: () => setState(() => _expanded = !_expanded),
          ),
          if (_expanded)
            ConstrainedBox(
              constraints: BoxConstraints(maxHeight: listMaxHeight),
              child: ListView(
                key: ValueKey('dsh-native.process.scroll:$anchor'),
                shrinkWrap: true,
                primary: false,
                padding: const EdgeInsets.only(top: 2),
                children: [
                  for (final moment in widget.moments)
                    if (moment.row case final row?)
                      _ProcessToolRow(row: row)
                    else
                      Padding(
                        padding: const EdgeInsets.fromLTRB(24, 4, 0, 8),
                        child: Text(
                          (moment.reasoning == null ||
                                  moment.reasoning!.isEmpty)
                              ? stringsOf(context).dshThinking()
                              : moment.reasoning!,
                          style: TextStyle(
                            fontSize: 13,
                            height: 1.45,
                            color: muted,
                          ),
                        ),
                      ),
                ],
              ),
            ),
        ],
      ),
    );
  }
}

final class _TrailLine extends StatelessWidget {
  const _TrailLine({
    super.key,
    required this.icon,
    required this.label,
    required this.muted,
    required this.expanded,
    required this.onTap,
  });

  final bool icon;
  final String label;
  final Color muted;
  final bool expanded;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => InkWell(
    onTap: onTap,
    borderRadius: BorderRadius.circular(8),
    child: Padding(
      padding: const EdgeInsets.symmetric(vertical: 5),
      child: Row(
        children: [
          if (icon) ...[
            Icon(Icons.auto_awesome, size: 15, color: muted),
            const SizedBox(width: 8),
          ],
          Expanded(
            child: Text(
              label,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(fontSize: 14, color: muted),
            ),
          ),
          AnimatedRotation(
            turns: expanded ? .25 : 0,
            duration: const Duration(milliseconds: 150),
            child: Icon(Icons.chevron_right_rounded, size: 18, color: muted),
          ),
        ],
      ),
    ),
  );
}

final class _ProcessToolRow extends StatefulWidget {
  const _ProcessToolRow({required this.row});

  final DshConversationRow row;

  @override
  State<_ProcessToolRow> createState() => _ProcessToolRowState();
}

final class _ProcessToolRowState extends State<_ProcessToolRow> {
  bool _open = false;

  @override
  Widget build(BuildContext context) {
    final row = widget.row;
    final shell = _isShell(row.nativeKey);
    final muted = Theme.of(context).colorScheme.onSurfaceVariant;
    final command = _commandOf(_toolArguments(row));
    final output = _resultText(_toolResult(row)) ?? _resultText(row.text);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        InkWell(
          key: ValueKey('dsh-native.process.item:${row.key}'),
          onTap: shell ? () => setState(() => _open = !_open) : null,
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 6),
            child: Row(
              children: [
                Icon(
                  row.failed ? Icons.error_outline : _momentIcon(row.nativeKey),
                  size: 16,
                  color: row.failed
                      ? Theme.of(context).colorScheme.error
                      : muted,
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    dshProcessActivityLabel(row, strings: stringsOf(context)),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(fontSize: 14, color: muted),
                  ),
                ),
                if (shell)
                  AnimatedRotation(
                    turns: _open ? .25 : 0,
                    duration: const Duration(milliseconds: 150),
                    child: Icon(
                      Icons.chevron_right_rounded,
                      size: 18,
                      color: muted,
                    ),
                  ),
              ],
            ),
          ),
        ),
        if (shell && _open)
          _ShellCard(
            key: ValueKey('dsh-native.process.shell:${row.key}'),
            command: command ?? '',
            output: output,
          ),
      ],
    );
  }
}

final class _ShellCard extends StatelessWidget {
  const _ShellCard({super.key, required this.command, required this.output});

  final String command;
  final String? output;

  @override
  Widget build(BuildContext context) {
    final dark = Theme.of(context).brightness == Brightness.dark;
    final muted = Theme.of(context).colorScheme.onSurfaceVariant;
    return Container(
      margin: const EdgeInsets.only(left: 24, bottom: 8),
      padding: const EdgeInsets.fromLTRB(14, 12, 14, 12),
      decoration: BoxDecoration(
        color: dark ? const Color(0xFF2A2A2A) : const Color(0xFFF4F5F7),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Shell', style: TextStyle(fontSize: 13, color: muted)),
          if (command.isNotEmpty) ...[
            const SizedBox(height: 8),
            SelectableText(
              '\$ $command',
              style: const TextStyle(
                fontFamily: 'monospace',
                fontSize: 13,
                height: 1.45,
              ),
            ),
          ],
          if (output != null) ...[
            const SizedBox(height: 8),
            SelectableText(
              output!,
              style: TextStyle(
                fontFamily: 'monospace',
                fontSize: 13,
                height: 1.45,
                color: muted,
              ),
            ),
          ],
        ],
      ),
    );
  }
}
