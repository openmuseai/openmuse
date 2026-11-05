import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// Drag handle on the left edge of the DSH panel (drag left to widen).
///
/// Uses the engine pointer router so a drag still ends when the pointer
/// travels over the embedded WebView. Hit-tested [GestureDetector]s miss
/// mouse-up in that case and leave the width glued to later movement.
class DshPanelResizer extends StatefulWidget {
  const DshPanelResizer({super.key});

  @override
  State<DshPanelResizer> createState() => _DshPanelResizerState();
}

class _DshPanelResizerState extends State<DshPanelResizer> {
  final ValueNotifier<bool> _hovered = ValueNotifier(false);
  final ValueNotifier<bool> _dragging = ValueNotifier(false);
  int? _pointer;
  double _originX = 0;
  double _originWidth = 0;
  DshAgentController? _controller;

  @override
  void dispose() {
    _detachRouter();
    _hovered.dispose();
    _dragging.dispose();
    super.dispose();
  }

  void _onGlobalPointer(PointerEvent event) {
    if (_pointer == null || event.pointer != _pointer) return;
    final controller = _controller;
    if (controller == null) return;
    if (event is PointerMoveEvent) {
      controller.setWidth(_originWidth - (event.position.dx - _originX));
      return;
    }
    if (event is PointerUpEvent || event is PointerCancelEvent) {
      _finish();
    }
  }

  void _detachRouter() {
    if (_pointer == null) return;
    GestureBinding.instance.pointerRouter.removeGlobalRoute(_onGlobalPointer);
    _pointer = null;
  }

  void _finish() {
    if (_pointer == null) return;
    _detachRouter();
    _dragging.value = false;
    _controller?.persistWidth();
  }

  void _start(PointerDownEvent event, DshAgentController controller) {
    if (event.buttons != kPrimaryButton) return;
    _finish();
    _controller = controller;
    _pointer = event.pointer;
    _originX = event.position.dx;
    _originWidth = controller.width;
    _dragging.value = true;
    GestureBinding.instance.pointerRouter.addGlobalRoute(_onGlobalPointer);
  }

  @override
  Widget build(BuildContext context) {
    final controller = context.read<DshAgentController>();
    return MouseRegion(
      cursor: SystemMouseCursors.resizeLeftRight,
      onEnter: (_) => _hovered.value = true,
      onExit: (_) => _hovered.value = false,
      child: Listener(
        behavior: HitTestBehavior.translucent,
        onPointerDown: (event) => _start(event, controller),
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
