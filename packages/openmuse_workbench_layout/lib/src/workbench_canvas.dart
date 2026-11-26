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
final class WorkbenchCanvas extends StatefulWidget {
  const WorkbenchCanvas({
    super.key,
    required this.controller,
    required this.paneBuilder,
    this.overlayMenuBuilder,
    this.onPaneFocus,
    this.hiddenPaneIds = const {},
    this.retainedInstanceRefs = const {},
    this.gutter = 1,
    this.onSizeChanged,
    this.pinnedPaneId,
    this.pinnedPaneWidth = 400,
    this.onPinnedWidthChanged,
  });

  final WorkbenchLayoutController controller;
  final WorkbenchPaneBuilder paneBuilder;
  final WorkbenchPaneMenuBuilder? overlayMenuBuilder;
  final void Function(String paneId, SurfaceBinding binding)? onPaneFocus;
  final Set<String> hiddenPaneIds;

  /// Keep selected hidden surfaces mounted without focus, semantics or ticking.
  /// Resource lifetime and checkpoint policy remain with the caller.
  final Set<String> retainedInstanceRefs;
  final double gutter;
  final ValueChanged<Size>? onSizeChanged;

  /// A stable leading surface; other panes retain their own split tree.
  final String? pinnedPaneId;
  final double pinnedPaneWidth;
  final ValueChanged<double>? onPinnedWidthChanged;

  @override
  State<WorkbenchCanvas> createState() => _WorkbenchCanvasState();
}

final class _WorkbenchCanvasState extends State<WorkbenchCanvas> {
  final _lastVisibleRects = <String, Rect>{};
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: widget.controller,
    builder: (context, _) => LayoutBuilder(
      builder: (context, constraints) {
        widget.onSizeChanged?.call(constraints.biggest);
        final snapshot = widget.controller.snapshot;
        final root = _withoutPanes(snapshot.root, widget.hiddenPaneIds);
        final geometry = solvePinnedWorkbenchLayout(
          root ?? snapshot.root,
          constraints.biggest,
          pinnedPaneId: widget.pinnedPaneId,
          pinnedPaneWidth: widget.pinnedPaneWidth,
          gutter: widget.gutter,
        );
        final fullGeometry = const WorkbenchLayoutSolver().solve(
          snapshot.root,
          constraints.biggest,
          gutter: widget.gutter,
        );
        final visibleRects = root == null
            ? <String, Rect>{}
            : geometry.paneRects;
        _lastVisibleRects.removeWhere(
          (instance, _) =>
              !snapshot.bindings.values.any((b) => b.instanceRef == instance),
        );
        for (final entry in snapshot.bindings.entries) {
          final rect = visibleRects[entry.key];
          if (rect != null) _lastVisibleRects[entry.value.instanceRef] = rect;
        }
        final bindings =
            snapshot.bindings.entries
                .where(
                  (entry) =>
                      visibleRects.containsKey(entry.key) ||
                      widget.retainedInstanceRefs.contains(
                        entry.value.instanceRef,
                      ),
                )
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
            if (root == null) const Center(child: Text('没有可见窗格。请重置布局。')),
            for (final entry in visibleRects.entries)
              Positioned.fromRect(
                key: ValueKey('pane-background:${entry.key}'),
                rect: entry.value,
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface,
                  ),
                ),
              ),
            for (final entry in bindings)
              Positioned.fromRect(
                key: ValueKey(entry.value.instanceRef),
                rect:
                    visibleRects[entry.key] ??
                    _lastVisibleRects[entry.value.instanceRef] ??
                    fullGeometry.paneRects[entry.key]!,
                child: Offstage(
                  offstage: !visibleRects.containsKey(entry.key),
                  child: TickerMode(
                    enabled: visibleRects.containsKey(entry.key),
                    child: ExcludeFocus(
                      excluding: !visibleRects.containsKey(entry.key),
                      child: Listener(
                        behavior: HitTestBehavior.translucent,
                        onPointerDown: (_) {
                          widget.controller.focus(entry.key);
                          widget.onPaneFocus?.call(entry.key, entry.value);
                        },
                        child: widget.paneBuilder(
                          context,
                          entry.key,
                          entry.value,
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            for (final entry in visibleRects.entries)
              if (snapshot.bindingFor(entry.key) == null)
                Positioned.fromRect(
                  key: ValueKey('empty-pane:${entry.key}'),
                  rect: entry.value,
                  child: widget.paneBuilder(context, entry.key, null),
                ),
            if (root != null)
              for (final divider in geometry.dividers)
                Positioned.fromRect(
                  key: ValueKey('layout-divider:${divider.path.join('.')}'),
                  rect: _dividerHitRect(divider.rect, divider.axis),
                  child: OpenMusePaneResizer(
                    axis: divider.axis,
                    onDelta: (delta) {
                      final extent = divider.axis == Axis.horizontal
                          ? divider.containerRect.width - widget.gutter
                          : divider.containerRect.height - widget.gutter;
                      if (extent <= 0) return;
                      final current = widget.controller.ratioBetween(
                        divider.leadingPaneId,
                        divider.trailingPaneId,
                      );
                      widget.controller.resizeBetween(
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
            if (visibleRects[widget.pinnedPaneId] case final pinned?
                when widget.onPinnedWidthChanged != null)
              Positioned(
                key: const ValueKey('agent-workspace-divider'),
                left: pinned.right - 3,
                top: pinned.top,
                width: 6,
                height: pinned.height,
                child: OpenMusePaneResizer(
                  axis: Axis.horizontal,
                  onDelta: (delta) => widget.onPinnedWidthChanged!(
                    widget.pinnedPaneWidth + delta,
                  ),
                ),
              ),
            if (widget.overlayMenuBuilder != null)
              for (final entry in visibleRects.entries)
                if (widget.overlayMenuBuilder!(
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

Rect _dividerHitRect(Rect rect, Axis axis) => axis == Axis.horizontal
    ? Rect.fromLTRB(rect.left - 3, rect.top, rect.right + 3, rect.bottom)
    : Rect.fromLTRB(rect.left, rect.top - 3, rect.right, rect.bottom + 3);

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
        color: Colors.transparent,
        alignment: Alignment.center,
        child: DecoratedBox(
          decoration: BoxDecoration(
            color: _hovered || _dragging
                ? OpenMuseTokens.cyan.withValues(alpha: 0.7)
                : (Theme.of(context).brightness == Brightness.dark
                      ? const Color(0x2EFFFFFF)
                      : const Color(0x14000000)),
          ),
          child: SizedBox(
            width: widget.axis == Axis.horizontal ? 1 : double.infinity,
            height: widget.axis == Axis.vertical ? 1 : double.infinity,
          ),
        ),
      ),
    ),
  );
}

/// Keep a leading pane at a stable width while solving the remaining region.
/// No binding, surface identity or resource state is changed by this projection.
WorkbenchLayoutGeometry solvePinnedWorkbenchLayout(
  LayoutNode root,
  Size size, {
  String? pinnedPaneId,
  double pinnedPaneWidth = 400,
  double gutter = 1,
}) {
  final solver = const WorkbenchLayoutSolver();
  LayoutNode? remove(LayoutNode node) => switch (node) {
    PaneNode(:final paneId) => paneId == pinnedPaneId ? null : node,
    SplitNode(:final first, :final second) => combine(
      node,
      remove(first),
      remove(second),
    ),
  };
  if (pinnedPaneId == null) return solver.solve(root, size, gutter: gutter);
  final original = solver.solve(root, size, gutter: gutter);
  if (!original.paneRects.containsKey(pinnedPaneId)) return original;
  final remainder = remove(root);
  if (remainder == null) return original;
  final width = pinnedPaneWidth.clamp(0.0, size.width);
  final offset = Offset((width + gutter).clamp(0.0, size.width), 0);
  final geometry = solver.solve(
    remainder,
    Size((size.width - offset.dx).clamp(0.0, size.width), size.height),
    gutter: gutter,
  );
  return WorkbenchLayoutGeometry(
    paneRects: {
      pinnedPaneId: Rect.fromLTWH(0, 0, width, size.height),
      for (final e in geometry.paneRects.entries) e.key: e.value.shift(offset),
    },
    dividers: [
      for (final d in geometry.dividers)
        LayoutDivider(
          path: d.path,
          axis: d.axis,
          rect: d.rect.shift(offset),
          containerRect: d.containerRect.shift(offset),
          leadingPaneId: d.leadingPaneId,
          trailingPaneId: d.trailingPaneId,
        ),
    ],
  );
}

LayoutNode? combine(SplitNode node, LayoutNode? first, LayoutNode? second) {
  if (first == null) return second;
  if (second == null) return first;
  return node.copyWith(first: first, second: second);
}
