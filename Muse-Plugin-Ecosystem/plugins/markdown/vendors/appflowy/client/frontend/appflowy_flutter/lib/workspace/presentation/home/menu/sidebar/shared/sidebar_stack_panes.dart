import 'dart:math' as math;

import 'package:appflowy/workspace/presentation/home/home_sizes.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/shared/sidebar_stack_layout.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class SidebarStackPane {
  const SidebarStackPane({
    required this.id,
    required this.builder,
  });

  final String id;
  final Widget Function(
    BuildContext context,
    SidebarStackPaneSlot slot,
  ) builder;
}

class SidebarStackPaneSlot {
  const SidebarStackPaneSlot({
    required this.expanded,
    required this.fillRemaining,
    required this.onToggle,
    required this.onContentHeight,
    required this.onActivate,
  });

  final bool expanded;
  final bool fillRemaining;
  final ValueChanged<bool> onToggle;
  final ValueChanged<double> onContentHeight;
  final VoidCallback onActivate;
}

class SidebarStackPanes extends StatefulWidget {
  const SidebarStackPanes({
    super.key,
    required this.panes,
    this.headerHeight = HomeSizes.workspaceSectionHeight,
    this.dividerHeight = 17,
  });

  final List<SidebarStackPane> panes;
  final double headerHeight;
  final double dividerHeight;

  @override
  State<SidebarStackPanes> createState() => _SidebarStackPanesState();
}

class _SidebarStackPanesState extends State<SidebarStackPanes> {
  late Set<String> _expanded;
  String? _activeId;
  final Map<String, double> _bodyHeights = {};

  @override
  void initState() {
    super.initState();
    _expanded = {for (final pane in widget.panes) pane.id};
    _activeId = widget.panes.isEmpty ? null : widget.panes.first.id;
  }

  @override
  void didUpdateWidget(covariant SidebarStackPanes oldWidget) {
    super.didUpdateWidget(oldWidget);
    final ids = widget.panes.map((pane) => pane.id).toSet();
    final oldIds = oldWidget.panes.map((pane) => pane.id).toSet();
    _expanded.removeWhere((id) => !ids.contains(id));
    for (final pane in widget.panes) {
      if (!oldIds.contains(pane.id)) {
        _expanded.add(pane.id);
      }
    }
    if (_activeId == null || !ids.contains(_activeId)) {
      _activeId = widget.panes.isEmpty ? null : widget.panes.first.id;
    }
  }

  void _toggle(String id, bool expanded) {
    setState(() {
      if (expanded) {
        _expanded.add(id);
        _activeId = id;
      } else {
        _expanded.remove(id);
        _bodyHeights[id] = 0;
        if (_activeId == id) {
          _activeId = _expanded.isEmpty ? null : _expanded.last;
        }
      }
    });
  }

  void _reportHeight(String id, double height) {
    final next = height < 0 ? 0.0 : height;
    final previous = _bodyHeights[id] ?? 0;
    if ((previous - next).abs() < 0.5) return;
    setState(() {
      _bodyHeights[id] = next;
      if (next > previous) {
        _activeId = id;
      }
    });
  }

  void _activate(String id) {
    if (_activeId == id) return;
    setState(() => _activeId = id);
  }

  void _syncExpanded(Set<String> next) {
    if (_setEquals(_expanded, next)) return;
    SchedulerBinding.instance.addPostFrameCallback((_) {
      if (!mounted || _setEquals(_expanded, next)) return;
      setState(() => _expanded = Set<String>.from(next));
    });
  }

  bool _setEquals(Set<String> a, Set<String> b) {
    if (a.length != b.length) return false;
    return a.containsAll(b);
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final paneIds = widget.panes.map((pane) => pane.id).toList();
        final decision = resolveSidebarStackLayout(
          paneIds: paneIds,
          expandedIds: _expanded,
          activeId: _activeId,
          bodyHeights: _bodyHeights,
          availableHeight: constraints.maxHeight,
          headerHeight: widget.headerHeight,
          dividerHeight: widget.dividerHeight,
        );
        _syncExpanded(decision.expandedIds);

        final maxHeight = constraints.maxHeight;
        final n = widget.panes.length;
        var leftover = maxHeight.isFinite
            ? maxHeight - (n > 1 ? (n - 1) * widget.dividerHeight : 0)
            : double.infinity;
        if (decision.fillingId != null && leftover.isFinite) {
          leftover -= widget.headerHeight + 4;
        }

        return ClipRect(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              for (var i = 0; i < widget.panes.length; i++) ...[
                if (i > 0) ...[
                  const VSpace(8),
                  const Padding(
                    padding: EdgeInsets.symmetric(horizontal: 4),
                    child: FlowyDivider(),
                  ),
                  const VSpace(8),
                ],
                _buildPane(widget.panes[i], decision, leftover, (used) {
                  leftover -= used;
                }),
              ],
            ],
          ),
        );
      },
    );
  }

  Widget _buildPane(
    SidebarStackPane pane,
    SidebarStackLayoutDecision decision,
    double leftover,
    void Function(double used) consume,
  ) {
    final expanded = decision.expandedIds.contains(pane.id);
    final filling = decision.fillingId == pane.id;
    final child = pane.builder(
      context,
      SidebarStackPaneSlot(
        expanded: expanded,
        fillRemaining: filling || expanded,
        onToggle: (next) => _toggle(pane.id, next),
        onContentHeight: (height) => _reportHeight(pane.id, height),
        onActivate: () => _activate(pane.id),
      ),
    );
    if (filling) return Expanded(child: child);
    if (!expanded) {
      consume(widget.headerHeight);
      return child;
    }
    final want = widget.headerHeight + 4 + (_bodyHeights[pane.id] ?? 0);
    final cap = leftover.isFinite ? math.max(0.0, leftover) : want;
    final height = math.min(want, cap);
    consume(height);
    return SizedBox(height: height, child: child);
  }
}

class SidebarContentHeightReporter extends StatefulWidget {
  const SidebarContentHeightReporter({
    super.key,
    required this.onHeight,
    required this.child,
  });

  final ValueChanged<double> onHeight;
  final Widget child;

  @override
  State<SidebarContentHeightReporter> createState() =>
      _SidebarContentHeightReporterState();
}

class _SidebarContentHeightReporterState
    extends State<SidebarContentHeightReporter> {
  double? _last;

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final height = context.size?.height ?? 0;
      if (_last != null && (height - _last!).abs() < 0.5) return;
      _last = height;
      widget.onHeight(height);
    });
    return widget.child;
  }
}
