import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// Drag handle on the left edge of the DSH panel (drag left to widen).
class DshPanelResizer extends StatefulWidget {
  const DshPanelResizer({super.key});

  @override
  State<DshPanelResizer> createState() => _DshPanelResizerState();
}

class _DshPanelResizerState extends State<DshPanelResizer> {
  final ValueNotifier<bool> _hovered = ValueNotifier(false);
  final ValueNotifier<bool> _dragging = ValueNotifier(false);

  @override
  void dispose() {
    _hovered.dispose();
    _dragging.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final controller = context.read<DshAgentController>();
    return MouseRegion(
      cursor: SystemMouseCursors.resizeLeftRight,
      onEnter: (_) => _hovered.value = true,
      onExit: (_) => _hovered.value = false,
      child: GestureDetector(
        dragStartBehavior: DragStartBehavior.down,
        behavior: HitTestBehavior.translucent,
        onHorizontalDragStart: (_) => _dragging.value = true,
        onHorizontalDragUpdate: (details) {
          _dragging.value = true;
          controller.setWidth(controller.width - details.delta.dx);
        },
        onHorizontalDragEnd: (_) {
          _dragging.value = false;
          controller.persistWidth();
        },
        onHorizontalDragCancel: () {
          _dragging.value = false;
          controller.persistWidth();
        },
        child: ValueListenableBuilder<bool>(
          valueListenable: _hovered,
          builder: (context, hovered, _) {
            return ValueListenableBuilder<bool>(
              valueListenable: _dragging,
              builder: (context, dragging, _) {
                return Container(
                  width: 6,
                  color: hovered || dragging
                      ? const Color(0xFF00B5FF)
                      : Colors.transparent,
                );
              },
            );
          },
        ),
      ),
    );
  }
}
