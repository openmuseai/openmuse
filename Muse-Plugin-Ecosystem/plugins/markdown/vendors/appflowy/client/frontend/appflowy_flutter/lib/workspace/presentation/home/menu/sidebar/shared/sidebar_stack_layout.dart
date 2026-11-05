/// Layout policy for stacked sidebar spaces (Project, Personal, and later ones).
///
/// Spaces are laid out top-to-bottom at their content height. When the stacked
/// content no longer fits, every space except the active one collapses to its
/// header and the active space fills the remaining height.
class SidebarStackLayoutDecision {
  const SidebarStackLayoutDecision({
    required this.expandedIds,
    this.fillingId,
  });

  final Set<String> expandedIds;

  /// When set, that pane is wrapped in [Expanded] and should scroll.
  final String? fillingId;
}

SidebarStackLayoutDecision resolveSidebarStackLayout({
  required List<String> paneIds,
  required Set<String> expandedIds,
  required String? activeId,
  required Map<String, double> bodyHeights,
  required double availableHeight,
  double headerHeight = 32,
  double dividerHeight = 17,
  double sectionBodyGap = 4,
}) {
  if (paneIds.isEmpty || !availableHeight.isFinite || availableHeight <= 0) {
    return SidebarStackLayoutDecision(
      expandedIds: Set<String>.from(expandedIds),
    );
  }

  final expanded = <String>{
    for (final id in paneIds)
      if (expandedIds.contains(id)) id,
  };
  final chrome = paneIds.length * headerHeight +
      (paneIds.isEmpty ? 0 : (paneIds.length - 1) * dividerHeight) +
      expanded.length * sectionBodyGap;
  var content = 0.0;
  var unmeasured = false;
  for (final id in expanded) {
    if (!bodyHeights.containsKey(id)) {
      unmeasured = true;
    }
    content += bodyHeights[id] ?? 0;
  }

  String? keepOf(Iterable<String> ids) {
    if (ids.isEmpty) return null;
    if (activeId != null && ids.contains(activeId)) return activeId;
    if (unmeasured) return ids.first;
    return ids.reduce(
      (a, b) => (bodyHeights[a] ?? 0) >= (bodyHeights[b] ?? 0) ? a : b,
    );
  }

  if (expanded.length <= 1) {
    return SidebarStackLayoutDecision(
      expandedIds: expanded,
      fillingId: keepOf(expanded),
    );
  }

  final keep = keepOf(expanded);
  if (!unmeasured && chrome + content > availableHeight + 0.5) {
    return SidebarStackLayoutDecision(
      expandedIds: {keep!},
      fillingId: keep,
    );
  }

  // Two expanded spaces: the active one always fills leftover height so a
  // directory-tree expand/collapse cannot overflow the sidebar for a frame.
  return SidebarStackLayoutDecision(
    expandedIds: expanded,
    fillingId: keep,
  );
}
