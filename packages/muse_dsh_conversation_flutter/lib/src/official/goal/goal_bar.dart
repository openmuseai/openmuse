import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

/// Read-only goal strip from `@deepseek-ai/dsh-client-ui-goal@0.1.7-rc.1`.
///
/// Pause, edit, and clear stay on Desktop. Those actions compare a goal
/// revision that this Native Gateway does not expose, so the strip does not
/// offer buttons that cannot be completed.
final class DshGoalBar extends StatelessWidget {
  const DshGoalBar({super.key, required this.goal});

  final JsonMap goal;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final label = goal['label'] as String? ?? stringsOf(context).dshGoal();
    final objective = goal['objective'] as String? ?? '';
    final blocked = goal['blockedMessage'] as String?;
    final paused = goal['phase'] == 'paused';
    final blockedPhase = goal['phase'] == 'blocked';
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: DecoratedBox(
        key: const ValueKey('dsh-native.goal'),
        decoration: BoxDecoration(
          color: colors.surfaceContainerLow,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: blockedPhase ? colors.error : colors.outlineVariant,
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          child: Row(
            children: [
              Icon(
                paused
                    ? Icons.pause_circle_outline_rounded
                    : Icons.flag_outlined,
                size: 16,
                color: blockedPhase ? colors.error : colors.primary,
              ),
              const SizedBox(width: 8),
              Text(
                label,
                style: const TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  blockedPhase && blocked != null && blocked.isNotEmpty
                      ? blocked
                      : objective,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontSize: 13,
                    color: blockedPhase ? colors.error : colors.onSurface,
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
