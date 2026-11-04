import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

/// Collapsible task list from `@deepseek-ai/dsh-client-ui-conversation@0.1.7-rc.1`.
///
/// The official dock starts collapsed. Each `todo/write` is a whole-list snapshot.
final class DshTodoPanel extends StatefulWidget {
  const DshTodoPanel({super.key, required this.todos});

  final List<JsonMap> todos;

  @override
  State<DshTodoPanel> createState() => _DshTodoPanelState();
}

final class _DshTodoPanelState extends State<DshTodoPanel> {
  bool _collapsed = true;

  @override
  Widget build(BuildContext context) {
    final todos = widget.todos;
    if (todos.isEmpty) return const SizedBox.shrink();
    final colors = Theme.of(context).colorScheme;
    final done = todos.where((item) => item['status'] == 'completed').length;
    final active = todos
        .where((item) => item['status'] == 'in_progress')
        .length;
    final pending = todos.length - done - active;
    final progress = [
      if (done > 0) stringsOf(context).dshCompletedCount(count: done),
      if (active > 0) stringsOf(context).dshActiveCount(count: active),
      if (pending > 0) stringsOf(context).dshPendingCount(count: pending),
    ].join('\u2002·\u2002');
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: colors.surfaceContainerLow,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: colors.outlineVariant),
        ),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(12, 6, 8, 6),
          child: Column(
            children: [
              InkWell(
                key: const ValueKey('dsh-native.todo.toggle'),
                onTap: () => setState(() => _collapsed = !_collapsed),
                child: SizedBox(
                  height: 32,
                  child: Row(
                    children: [
                      Icon(
                        Icons.checklist_rounded,
                        size: 16,
                        color: colors.onSurfaceVariant,
                      ),
                      const SizedBox(width: 10),
                      Text(
                        stringsOf(context).dshTasks(),
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(
                          progress,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            fontSize: 13,
                            color: colors.onSurfaceVariant,
                          ),
                        ),
                      ),
                      Icon(
                        _collapsed
                            ? Icons.keyboard_arrow_up_rounded
                            : Icons.keyboard_arrow_down_rounded,
                        size: 18,
                        color: colors.onSurfaceVariant,
                      ),
                    ],
                  ),
                ),
              ),
              if (!_collapsed)
                for (final item in todos)
                  Padding(
                    padding: const EdgeInsets.only(top: 6, bottom: 2),
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Padding(
                          padding: const EdgeInsets.only(top: 2),
                          child: _TodoMark(status: item['status'] as String?),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            item['content'] as String? ?? '',
                            style: TextStyle(
                              fontSize: 13,
                              height: 1.4,
                              decoration: item['status'] == 'completed'
                                  ? TextDecoration.lineThrough
                                  : null,
                              color: item['status'] == 'completed'
                                  ? colors.onSurfaceVariant
                                  : colors.onSurface,
                            ),
                          ),
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
}

final class _TodoMark extends StatelessWidget {
  const _TodoMark({required this.status});

  final String? status;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final color = status == 'in_progress'
        ? colors.primary
        : colors.onSurfaceVariant;
    final icon = switch (status) {
      'completed' => Icons.check_circle_outline_rounded,
      'in_progress' => Icons.timelapse_rounded,
      _ => Icons.circle_outlined,
    };
    return Icon(icon, size: 14, color: color);
  }
}
