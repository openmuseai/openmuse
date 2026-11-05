/// Which body snapshot ControlHost may emit for the focused view.
enum DshBodySnapshotKind { none, markdown, word }

DshBodySnapshotKind dshBodySnapshotKind({
  int? layout,
  String? viewId,
}) {
  final id = viewId?.trim() ?? '';
  if (id.isEmpty) return DshBodySnapshotKind.none;
  if (layout == 9) return DshBodySnapshotKind.word;
  if (layout == 0) return DshBodySnapshotKind.markdown;
  return DshBodySnapshotKind.none;
}
