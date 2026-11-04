/// Visible one-based page ranges used by product commands. The accepted syntax
/// matches GenOffice's `parsePageRanges`: `1-3,5`, Chinese commas and en dashes.
List<int>? parsePdfPageRanges(String input, int pageCount) {
  final pages = <int>{};
  final range = RegExp(r'^(\d+)\s*[-–]\s*(\d+)$');
  final single = RegExp(r'^(\d+)$');
  for (final raw in input.split(RegExp(r'[,，]'))) {
    final part = raw.trim();
    if (part.isEmpty) continue;
    final pair = range.firstMatch(part);
    final one = single.firstMatch(part);
    if (pair == null && one == null) return null;
    final start = int.tryParse((pair ?? one)!.group(1)!);
    final end = pair == null ? start : int.tryParse(pair.group(2)!);
    if (start == null ||
        end == null ||
        start < 1 ||
        end > pageCount ||
        start > end) {
      return null;
    }
    for (var page = start; page <= end; page++) {
      pages.add(page);
    }
  }
  if (pages.isEmpty) return null;
  return pages.toList()..sort();
}
