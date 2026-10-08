import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';

import 'layout.dart';

typedef WorkbenchPaneBuilder =
    Widget Function(
      BuildContext context,
      String paneId,
      SurfaceBinding? binding,
    );
typedef WorkbenchPaneMenuBuilder =
    Widget? Function(
      BuildContext context,
      String paneId,
      SurfaceBinding? binding,
    );

/// Shared Desktop/Web surface placement and divider interaction.
final class WorkbenchCanvas extends StatelessWidget {
  const WorkbenchCanvas({
    super.key,
    required this.controller,
    required this.paneBuilder,
    this.overlayMenuBuilder,
    this.onPaneFocus,
    this.hiddenPaneIds = const {},
    this.gutter = 6,
    this.onSizeChanged,
  });

  final WorkbenchLayoutController controller;
  final WorkbenchPaneBuilder paneBuilder;
  final WorkbenchPaneMenuBuilder? overlayMenuBuilder;
  final void Function(String paneId, SurfaceBinding binding)? onPaneFocus;
  final Set<String> hiddenPaneIds;
  final double gutter;
  final ValueChanged<Size>? onSizeChanged;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) => LayoutBuilder(
      builder: (context, constraints) {
        onSizeChanged?.call(constraints.biggest);
        final snapshot = controller.snapshot;
        final root = _withoutPanes(snapshot.root, hiddenPaneIds);
        if (root == null) {
          return const Center(child: Text('没有可见窗格。请重置布局。'));
        }
        final geometry = const WorkbenchLayoutSolver().solve(
          root,
          constraints.biggest,
          gutter: gutter,
        );
        final bindings =
            snapshot.bindings.entries
                .where((entry) => geometry.paneRects.containsKey(entry.key))
                .toList()
              ..sort(
                (a, b) => a.value.instanceRef.compareTo(b.value.instanceRef),
              );
        return Stack(
          clipBehavior: Clip.hardEdge,
          children: [
            const Positioned.fill(
              child: ColoredBox(color: OpenMuseTokens.canvas),
            ),
            for (final entry in geometry.paneRects.entries)
              Positioned.fromRect(
                key: ValueKey('pane-background:${entry.key}'),
                rect: entry.value,
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface,
                    border: Border.all(
                      color: snapshot.focusedPaneId == entry.key
                          ? Theme.of(context).colorScheme.primary
                          : Theme.of(context).dividerColor,
                    ),
                  ),
                ),
              ),
            for (final entry in bindings)
              Positioned.fromRect(
                key: ValueKey(entry.value.instanceRef),
                rect: geometry.paneRects[entry.key]!,
                child: Listener(
                  behavior: HitTestBehavior.translucent,
                  onPointerDown: (_) {
                    controller.focus(entry.key);
                    onPaneFocus?.call(entry.key, entry.value);
                  },
                  child: paneBuilder(context, entry.key, entry.value),
                ),
              ),
            for (final entry in geometry.paneRects.entries)
              if (snapshot.bindingFor(entry.key) == null)
                Positioned.fromRect(
                  key: ValueKey('empty-pane:${entry.key}'),
                  rect: entry.value,
                  child: paneBuilder(context, entry.key, null),
                ),
            for (final divider in geometry.dividers)
              Positioned.fromRect(
                key: ValueKey('layout-divider:${divider.path.join('.')}'),
                rect: divider.rect,
                child: OpenMusePaneResizer(
                  axis: divider.axis,
                  onDelta: (delta) {
                    final extent = divider.axis == Axis.horizontal
                        ? divider.containerRect.width - gutter
                        : divider.containerRect.height - gutter;
                    if (extent <= 0) return;
                    final current = controller.ratioBetween(
                      divider.leadingPaneId,
                      divider.trailingPaneId,
                    );
                    controller.resizeBetween(
                      divider.leadingPaneId,
                      divider.trailingPaneId,
                      (current + delta / extent).clamp(
                        minSplitRatio,
                        maxSplitRatio,
                      ),
                    );
                  },
                ),
              ),
            if (overlayMenuBuilder != null)
              for (final entry in geometry.paneRects.entries)
                if (overlayMenuBuilder!(
                      context,
                      entry.key,
                      snapshot.bindingFor(entry.key),
                    )
                    case final menu?)
                  Positioned(
                    key: ValueKey('pane-menu:${entry.key}'),
                    left: entry.value.right - 30,
                    top: entry.value.top + 8,
                    width: 26,
                    height: 26,
                    child: menu,
                  ),
          ],
        );
      },
    ),
  );
}

LayoutNode? _withoutPanes(LayoutNode node, Set<String> hidden) {
  switch (node) {
    case PaneNode(:final paneId):
      return hidden.contains(paneId) ? null : node;
    case SplitNode(:final first, :final second):
      final left = _withoutPanes(first, hidden);
      final right = _withoutPanes(second, hidden);
      if (left == null) return right;
      if (right == null) return left;
      return node.copyWith(first: left, second: right);
  }
}

/// Global pointer tracking keeps drag active over Desktop's native DSH view.
final class OpenMusePaneResizer extends StatefulWidget {
  const OpenMusePaneResizer({
    super.key,
    required this.axis,
    required this.onDelta,
  });
  final Axis axis;
  final ValueChanged<double> onDelta;

  @override
  State<OpenMusePaneResizer> createState() => _OpenMusePaneResizerState();
}

final class _OpenMusePaneResizerState extends State<OpenMusePaneResizer> {
  int? _pointer;
  Offset _last = Offset.zero;
  bool _hovered = false;
  bool _dragging = false;

  void _globalPointer(PointerEvent event) {
    if (event.pointer != _pointer) return;
    if (event is PointerMoveEvent) {
      final delta = widget.axis == Axis.horizontal
          ? event.position.dx - _last.dx
          : event.position.dy - _last.dy;
      _last = event.position;
      widget.onDelta(delta);
    } else if (event is PointerUpEvent || event is PointerCancelEvent) {
      _finish();
    }
  }

  void _finish() {
    if (_pointer == null) return;
    GestureBinding.instance.pointerRouter.removeGlobalRoute(_globalPointer);
    _pointer = null;
    if (mounted) setState(() => _dragging = false);
  }

  @override
  void dispose() {
    if (_pointer != null) {
      GestureBinding.instance.pointerRouter.removeGlobalRoute(_globalPointer);
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => MouseRegion(
    cursor: widget.axis == Axis.horizontal
        ? SystemMouseCursors.resizeLeftRight
        : SystemMouseCursors.resizeUpDown,
    onEnter: (_) => setState(() => _hovered = true),
    onExit: (_) => setState(() => _hovered = false),
    child: Listener(
      behavior: HitTestBehavior.opaque,
      onPointerDown: (event) {
        if (event.buttons != kPrimaryButton) return;
        _finish();
        _pointer = event.pointer;
        _last = event.position;
        setState(() => _dragging = true);
        GestureBinding.instance.pointerRouter.addGlobalRoute(_globalPointer);
      },
      child: Container(
        key: const Key('pane-resizer'),
        width: widget.axis == Axis.horizontal ? 6 : null,
        height: widget.axis == Axis.vertical ? 6 : null,
        color: _hovered || _dragging
            ? OpenMuseTokens.cyan.withValues(alpha: 0.65)
            : Theme.of(context).dividerColor.withValues(alpha: 0.5),
      ),
    ),
  );
}
