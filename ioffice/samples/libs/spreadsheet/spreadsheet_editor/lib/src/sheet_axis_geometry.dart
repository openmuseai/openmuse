/// One workbook axis in scaled logical pixels. A sparse override list keeps
/// offsets and hit testing independent of the sheet's million-row capacity.
final class SheetAxisGeometry {
  SheetAxisGeometry({
    required this.count,
    required this.defaultExtent,
    Map<int, double> overrides = const {},
  }) : assert(count > 0),
       assert(defaultExtent > 0),
       overrides = Map.unmodifiable(overrides) {
    _indices = this.overrides.keys.where((i) => i >= 0 && i < count).toList()
      ..sort();
    _prefixDelta = <double>[0];
    for (final index in _indices) {
      _prefixDelta.add(
        _prefixDelta.last + this.overrides[index]! - defaultExtent,
      );
    }
  }

  factory SheetAxisGeometry.fromPairs({
    required int count,
    required double defaultExtent,
    required double zoomFactor,
    required Object? pairs,
  }) {
    final overrides = <int, double>{};
    if (pairs is List) {
      for (final pair in pairs) {
        if (pair is! List || pair.length != 2) continue;
        final index = (pair[0] as num?)?.toInt();
        final extent = (pair[1] as num?)?.toDouble();
        if (index != null &&
            index >= 0 &&
            index < count &&
            extent != null &&
            extent.isFinite &&
            extent >= 0) {
          overrides[index] = extent * zoomFactor;
        }
      }
    }
    return SheetAxisGeometry(
      count: count,
      defaultExtent: defaultExtent * zoomFactor,
      overrides: overrides,
    );
  }

  final int count;
  final double defaultExtent;
  final Map<int, double> overrides;
  late final List<int> _indices;
  late final List<double> _prefixDelta;

  double extentAt(int index) => overrides[index] ?? defaultExtent;

  /// Leading edge of an index, including the sheet end at [count].
  double leading(int index) {
    final bounded = index.clamp(0, count);
    var low = 0;
    var high = _indices.length;
    while (low < high) {
      final middle = (low + high) >> 1;
      if (_indices[middle] < bounded) {
        low = middle + 1;
      } else {
        high = middle;
      }
    }
    return bounded * defaultExtent + _prefixDelta[low];
  }

  double get totalExtent => leading(count);

  /// First nonzero item intersecting a position; zero-sized hidden items are
  /// skipped even when several share the same leading edge.
  int indexAt(double offset) {
    final position = offset.clamp(0.0, totalExtent);
    var low = 0;
    var high = count;
    while (low < high) {
      final middle = (low + high) >> 1;
      if (leading(middle + 1) <= position) {
        low = middle + 1;
      } else {
        high = middle;
      }
    }
    return low.clamp(0, count - 1);
  }

  int visibleCount(int start, double viewport) {
    final end = indexAt(leading(start) + viewport);
    return (end - start + 1).clamp(1, count);
  }

  bool sameLayout(SheetAxisGeometry other) {
    if (count != other.count ||
        defaultExtent != other.defaultExtent ||
        overrides.length != other.overrides.length) {
      return false;
    }
    for (final entry in overrides.entries) {
      if (other.overrides[entry.key] != entry.value) return false;
    }
    return true;
  }
}
