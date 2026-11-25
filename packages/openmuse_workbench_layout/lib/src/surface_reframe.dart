import 'package:flutter/painting.dart' show Axis;
import 'layout.dart';

/// Put existing surface panes into a new surrounding layout without changing
/// their bindings, instance identities, or splits inside the retained region.
/// No resource, content, or close policy belongs to this layout operation.
/// Additional surfaces have no current binding. Their identities are supplied
/// by the caller; only new pane identities and balanced placement belong here.
WorkbenchLayoutSnapshot reframeWorkbenchSurfaces({
  required WorkbenchLayoutSnapshot current,
  required WorkbenchLayoutSnapshot target,
  required String targetPaneId,
  required Set<String> retainedInstances,
  List<SurfaceBinding> additionalSurfaces = const [],
}) {
  if (!target.paneIds.contains(targetPaneId)) {
    throw ArgumentError.value(
      targetPaneId,
      'targetPaneId',
      'Missing target pane',
    );
  }
  if (retainedInstances.isEmpty && additionalSurfaces.isEmpty) return target;
  final retained = Map.fromEntries(
    current.bindings.entries.where(
      (entry) => retainedInstances.contains(entry.value.instanceRef),
    ),
  );
  if (retained.length != retainedInstances.length) {
    throw ArgumentError('Every retained instance must have a current binding');
  }
  LayoutNode? prune(LayoutNode node) => switch (node) {
    PaneNode(:final paneId) => retained.containsKey(paneId) ? node : null,
    SplitNode(:final first, :final second) => _combine(
      node,
      prune(first),
      prune(second),
    ),
  };
  final regionBindings = {...retained};
  final regions = <LayoutNode>[if (prune(current.root) case final node?) node];
  final reservedPanes = {...current.paneIds, ...target.paneIds};
  var recoverySequence = 0;
  for (final binding in additionalSurfaces) {
    if (current.bindings.values.any(
      (existing) =>
          existing.instanceRef == binding.instanceRef ||
          existing.bindingId == binding.bindingId,
    )) {
      throw ArgumentError(
        'An additional surface already has a current binding',
      );
    }
    String id;
    do {
      id = 'recovered-${++recoverySequence}';
    } while (!reservedPanes.add(id));
    regionBindings[id] = binding;
    regions.add(PaneNode(paneId: id));
  }
  // Keep each existing region intact; distribute new leaves without a linear
  // nesting chain. The final model still enforces its depth and node budget.
  LayoutNode combineRegions(List<LayoutNode> nodes) {
    if (nodes.length == 1) return nodes.single;
    final middle = nodes.length ~/ 2;
    return SplitNode(
      axis: Axis.horizontal,
      ratio: middle / nodes.length,
      first: combineRegions(nodes.sublist(0, middle)),
      second: combineRegions(nodes.sublist(middle)),
    );
  }

  final region = combineRegions(regions);
  final occupied = regionBindings.keys.toSet();
  final renamed = <String, String>{};
  var sequence = 0;
  LayoutNode surround(LayoutNode node) {
    switch (node) {
      case PaneNode(:final paneId, :final minWidth, :final minHeight):
        if (paneId == targetPaneId) return region;
        var id = paneId;
        if (occupied.contains(id)) {
          do {
            id = 'reframe-${++sequence}';
          } while (occupied.contains(id) || target.paneIds.contains(id));
        }
        occupied.add(id);
        renamed[paneId] = id;
        return PaneNode(paneId: id, minWidth: minWidth, minHeight: minHeight);
      case SplitNode(:final first, :final second):
        return node.copyWith(first: surround(first), second: surround(second));
    }
  }

  final root = surround(target.root);
  final bindings = {...regionBindings};
  for (final entry in target.bindings.entries) {
    if (entry.key == targetPaneId) continue;
    if (retainedInstances.contains(entry.value.instanceRef)) {
      throw ArgumentError(
        'Retained surface also occurs outside the target pane',
      );
    }
    bindings[renamed[entry.key]!] = entry.value;
  }
  // The existing model validates depth, node budget and unique identities.
  // Reject an oversized reframe before the caller mutates its live layout.
  return WorkbenchLayoutSnapshot(
    root: root,
    bindings: bindings,
    focusedPaneId: retained.containsKey(current.focusedPaneId)
        ? current.focusedPaneId
        : renamed[target.focusedPaneId] ?? regionBindings.keys.first,
  );
}

LayoutNode? _combine(SplitNode node, LayoutNode? first, LayoutNode? second) {
  if (first == null) return second;
  if (second == null) return first;
  return node.copyWith(first: first, second: second);
}
