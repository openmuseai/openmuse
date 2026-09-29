enum WindowSizeClass { compact, medium, expanded }

enum SurfaceLifecycle { visible, warm, suspended }

final class LogicalWindow {
  const LogicalWindow(this.instanceRef);
  final String instanceRef;
}

final class DisplaySegment {
  const DisplaySegment(this.width, this.height);
  final double width;
  final double height;
}

final class WindowProjection {
  const WindowProjection(this.sizeClass, this.visible, this.lifecycle);
  final WindowSizeClass sizeClass;
  final List<LogicalWindow> visible;
  final Map<String, SurfaceLifecycle> lifecycle;
}

final class AdaptiveWindowProjector {
  const AdaptiveWindowProjector();

  WindowProjection project({
    required double width,
    required double height,
    required List<LogicalWindow> windows,
    List<DisplaySegment> segments = const [],
    int activeIndex = 0,
    double fontScale = 1,
    double imeHeight = 0,
  }) {
    final usableHeight = (height - imeHeight).clamp(1, double.infinity);
    final sizeClass = width < 600
        ? WindowSizeClass.compact
        : width < 840
        ? WindowSizeClass.medium
        : WindowSizeClass.expanded;
    final segmentCapacity = segments
        .where((segment) => segment.width >= 280 && segment.height >= 240)
        .length;
    final capacity = sizeClass == WindowSizeClass.compact
        ? 1
        : sizeClass == WindowSizeClass.medium
        ? 2
        : 3;
    final constrained = fontScale > 1.6 || usableHeight < 320 ? 1 : capacity;
    final count = segments.isEmpty
        ? constrained
        : constrained.clamp(1, segmentCapacity == 0 ? 1 : segmentCapacity);
    final start = activeIndex.clamp(
      0,
      windows.isEmpty ? 0 : windows.length - 1,
    );
    final visible = windows.skip(start).take(count).toList(growable: false);
    final visibleRefs = visible.map((window) => window.instanceRef).toSet();
    return WindowProjection(sizeClass, visible, {
      for (final window in windows)
        window.instanceRef: visibleRefs.contains(window.instanceRef)
            ? SurfaceLifecycle.visible
            : (window.instanceRef ==
                      windows.elementAtOrNull(start - 1)?.instanceRef
                  ? SurfaceLifecycle.warm
                  : SurfaceLifecycle.suspended),
    });
  }
}

extension<T> on List<T> {
  T? elementAtOrNull(int index) =>
      index >= 0 && index < length ? this[index] : null;
}
