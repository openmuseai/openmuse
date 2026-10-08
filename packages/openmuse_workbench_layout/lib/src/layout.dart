import 'dart:convert';
import 'dart:math' as math;
import 'dart:ui';

import 'package:flutter/foundation.dart';
import 'package:flutter/painting.dart' show Axis;

const int workbenchLayoutJsonVersion = 1;
const int maxLayoutDepth = 8;
const int maxLayoutNodes = 31;
const double minSplitRatio = 0.1;
const double maxSplitRatio = 0.9;

enum SurfaceMobility { retainSameView, snapshotRestore, recreateSafe, fixed }

sealed class SurfaceLocation {
  const SurfaceLocation();
}

final class EmbeddedSurfaceLocation extends SurfaceLocation {
  const EmbeddedSurfaceLocation(this.paneId);
  final String paneId;
}

final class FloatingSurfaceLocation extends SurfaceLocation {
  const FloatingSurfaceLocation(this.windowRef);
  final String windowRef;
}

double _validateRatio(double ratio) {
  if (!ratio.isFinite || ratio < minSplitRatio || ratio > maxSplitRatio) {
    throw ArgumentError.value(
      ratio,
      'ratio',
      'Must be finite and between $minSplitRatio and $maxSplitRatio.',
    );
  }
  return ratio;
}

String _validateId(String value, String name) {
  if (value.isEmpty || value.length > 256) {
    throw ArgumentError.value(value, name, 'Must contain 1–256 characters.');
  }
  return value;
}

@immutable
sealed class LayoutNode {
  const LayoutNode();

  Map<String, Object?> toJson();
}

@immutable
final class PaneNode extends LayoutNode {
  PaneNode({required String paneId, this.minWidth = 120, this.minHeight = 80})
    : paneId = _validateId(paneId, 'paneId') {
    if (!minWidth.isFinite ||
        !minHeight.isFinite ||
        minWidth < 0 ||
        minHeight < 0) {
      throw ArgumentError(
        'Pane minimum dimensions must be finite and positive.',
      );
    }
  }

  final String paneId;
  final double minWidth;
  final double minHeight;

  @override
  Map<String, Object?> toJson() => {
    'type': 'pane',
    'paneId': paneId,
    'minWidth': minWidth,
    'minHeight': minHeight,
  };

  @override
  bool operator ==(Object other) =>
      other is PaneNode &&
      other.paneId == paneId &&
      other.minWidth == minWidth &&
      other.minHeight == minHeight;

  @override
  int get hashCode => Object.hash(PaneNode, paneId, minWidth, minHeight);

  @override
  String toString() => 'PaneNode($paneId)';
}

@immutable
final class SplitNode extends LayoutNode {
  SplitNode({
    required this.axis,
    required double ratio,
    required this.first,
    required this.second,
  }) : ratio = _validateRatio(ratio);

  final Axis axis;
  final double ratio;
  final LayoutNode first;
  final LayoutNode second;

  SplitNode copyWith({
    Axis? axis,
    double? ratio,
    LayoutNode? first,
    LayoutNode? second,
  }) => SplitNode(
    axis: axis ?? this.axis,
    ratio: ratio ?? this.ratio,
    first: first ?? this.first,
    second: second ?? this.second,
  );

  @override
  Map<String, Object?> toJson() => {
    'type': 'split',
    'axis': axis.name,
    'ratio': ratio,
    'first': first.toJson(),
    'second': second.toJson(),
  };

  @override
  bool operator ==(Object other) =>
      other is SplitNode &&
      other.axis == axis &&
      other.ratio == ratio &&
      other.first == first &&
      other.second == second;

  @override
  int get hashCode => Object.hash(axis, ratio, first, second);

  @override
  String toString() => 'SplitNode($axis, $ratio, $first, $second)';
}

@immutable
final class SurfaceBinding {
  SurfaceBinding({
    required String bindingId,
    required String surfaceRef,
    required String instanceRef,
    this.mobility = SurfaceMobility.fixed,
  }) : bindingId = _validateId(bindingId, 'bindingId'),
       surfaceRef = _validateId(surfaceRef, 'surfaceRef'),
       instanceRef = _validateId(instanceRef, 'instanceRef');

  final String bindingId;
  final String surfaceRef;
  final String instanceRef;
  final SurfaceMobility mobility;

  Map<String, Object?> toJson() => {
    'bindingId': bindingId,
    'surfaceRef': surfaceRef,
    'instanceRef': instanceRef,
    'mobility': mobility.name,
  };

  @override
  bool operator ==(Object other) =>
      other is SurfaceBinding &&
      other.bindingId == bindingId &&
      other.surfaceRef == surfaceRef &&
      other.instanceRef == instanceRef &&
      other.mobility == mobility;

  @override
  int get hashCode => Object.hash(bindingId, surfaceRef, instanceRef, mobility);
}

@immutable
final class WorkbenchLayoutSnapshot {
  WorkbenchLayoutSnapshot({
    required this.root,
    Map<String, SurfaceBinding> bindings = const {},
    this.focusedPaneId,
  }) : bindings = Map.unmodifiable(bindings) {
    _validate();
  }

  factory WorkbenchLayoutSnapshot.fromJson(Map<String, Object?> json) {
    if (json['version'] != workbenchLayoutJsonVersion) {
      throw const FormatException('Unsupported workbench layout version.');
    }
    final parser = _LayoutJsonParser();
    final root = parser.parseNode(json['root'], depth: 1);
    final rawBindings = json['bindings'];
    if (rawBindings is! Map) {
      throw const FormatException('bindings must be an object.');
    }
    final bindings = <String, SurfaceBinding>{};
    for (final entry in rawBindings.entries) {
      if (entry.key is! String || entry.value is! Map) {
        throw const FormatException('Invalid binding entry.');
      }
      final value = entry.value as Map;
      final bindingId = value['bindingId'];
      final surfaceRef = value['surfaceRef'];
      final instanceRef = value['instanceRef'];
      final mobilityName = value['mobility'] ?? SurfaceMobility.fixed.name;
      if (bindingId is! String ||
          surfaceRef is! String ||
          instanceRef is! String ||
          mobilityName is! String) {
        throw const FormatException('Invalid surface binding.');
      }
      try {
        bindings[entry.key as String] = SurfaceBinding(
          bindingId: bindingId,
          surfaceRef: surfaceRef,
          instanceRef: instanceRef,
          mobility: SurfaceMobility.values.firstWhere(
            (value) => value.name == mobilityName,
            orElse: () =>
                throw const FormatException('Invalid surface mobility.'),
          ),
        );
      } on ArgumentError {
        throw const FormatException('Invalid surface binding identifier.');
      }
    }
    final focusedPaneId = json['focusedPaneId'];
    if (focusedPaneId != null && focusedPaneId is! String) {
      throw const FormatException('focusedPaneId must be a string or null.');
    }
    try {
      return WorkbenchLayoutSnapshot(
        root: root,
        bindings: bindings,
        focusedPaneId: focusedPaneId as String?,
      );
    } on ArgumentError catch (error) {
      throw FormatException('Invalid workbench layout: $error');
    } on StateError catch (error) {
      throw FormatException('Invalid workbench layout: $error');
    }
  }

  final LayoutNode root;
  final Map<String, SurfaceBinding> bindings;
  final String? focusedPaneId;

  Set<String> get paneIds => Set.unmodifiable(_paneIds(root));

  SurfaceBinding? bindingFor(String paneId) => bindings[paneId];

  WorkbenchLayoutSnapshot copyWith({
    LayoutNode? root,
    Map<String, SurfaceBinding>? bindings,
    Object? focusedPaneId = _notProvided,
  }) => WorkbenchLayoutSnapshot(
    root: root ?? this.root,
    bindings: bindings ?? this.bindings,
    focusedPaneId: identical(focusedPaneId, _notProvided)
        ? this.focusedPaneId
        : focusedPaneId as String?,
  );

  Map<String, Object?> toJson() => {
    'version': workbenchLayoutJsonVersion,
    'root': root.toJson(),
    'bindings': {
      for (final entry in bindings.entries) entry.key: entry.value.toJson(),
    },
    'focusedPaneId': focusedPaneId,
  };

  String encode() => jsonEncode(toJson());

  static WorkbenchLayoutSnapshot? tryFromJson(Object? value) {
    try {
      if (value is! Map) return null;
      return WorkbenchLayoutSnapshot.fromJson(Map<String, Object?>.from(value));
    } on Object {
      return null;
    }
  }

  static WorkbenchLayoutSnapshot? tryDecode(String source) {
    try {
      return tryFromJson(jsonDecode(source));
    } on Object {
      return null;
    }
  }

  void _validate() {
    final paneIds = <String>{};
    final bindingIds = <String>{};
    final instanceRefs = <String>{};
    var nodeCount = 0;

    void visit(LayoutNode node, int depth) {
      nodeCount++;
      if (nodeCount > maxLayoutNodes) {
        throw StateError('A layout may contain at most $maxLayoutNodes nodes.');
      }
      if (depth > maxLayoutDepth) {
        throw StateError(
          'A layout may be at most $maxLayoutDepth levels deep.',
        );
      }
      switch (node) {
        case PaneNode(:final paneId):
          if (!paneIds.add(paneId)) {
            throw StateError('Duplicate pane ID: $paneId');
          }
        case SplitNode(:final ratio, :final first, :final second):
          _validateRatio(ratio);
          visit(first, depth + 1);
          visit(second, depth + 1);
      }
    }

    visit(root, 1);
    for (final entry in bindings.entries) {
      if (!paneIds.contains(entry.key)) {
        throw StateError('Binding points to missing pane: ${entry.key}');
      }
      if (!bindingIds.add(entry.value.bindingId)) {
        throw StateError('Duplicate binding ID: ${entry.value.bindingId}');
      }
      if (!instanceRefs.add(entry.value.instanceRef)) {
        throw StateError(
          'Surface instance is bound more than once: ${entry.value.instanceRef}',
        );
      }
    }
    final focus = focusedPaneId;
    if (focus != null && !paneIds.contains(focus)) {
      throw StateError('Focused pane does not exist: $focus');
    }
  }

  @override
  bool operator ==(Object other) =>
      other is WorkbenchLayoutSnapshot &&
      other.root == root &&
      mapEquals(other.bindings, bindings) &&
      other.focusedPaneId == focusedPaneId;

  @override
  int get hashCode => Object.hash(
    root,
    Object.hashAllUnordered(
      bindings.entries.map((entry) => Object.hash(entry.key, entry.value)),
    ),
    focusedPaneId,
  );
}

const Object _notProvided = Object();

final class _LayoutJsonParser {
  var _nodeCount = 0;

  LayoutNode parseNode(Object? value, {required int depth}) {
    if (value is! Map) throw const FormatException('Invalid layout node.');
    if (depth > maxLayoutDepth || ++_nodeCount > maxLayoutNodes) {
      throw const FormatException('Layout exceeds safety limits.');
    }
    switch (value['type']) {
      case 'pane':
        final paneId = value['paneId'];
        final minWidth = value['minWidth'] ?? 120;
        final minHeight = value['minHeight'] ?? 80;
        if (paneId is! String || minWidth is! num || minHeight is! num) {
          throw const FormatException('Pane ID must be a string.');
        }
        try {
          return PaneNode(
            paneId: paneId,
            minWidth: minWidth.toDouble(),
            minHeight: minHeight.toDouble(),
          );
        } on ArgumentError {
          throw const FormatException('Invalid pane ID.');
        }
      case 'split':
        final axisName = value['axis'];
        final ratio = value['ratio'];
        if ((axisName != 'horizontal' && axisName != 'vertical') ||
            ratio is! num ||
            !ratio.isFinite) {
          throw const FormatException('Invalid split properties.');
        }
        try {
          return SplitNode(
            axis: axisName == 'horizontal' ? Axis.horizontal : Axis.vertical,
            ratio: ratio.toDouble(),
            first: parseNode(value['first'], depth: depth + 1),
            second: parseNode(value['second'], depth: depth + 1),
          );
        } on ArgumentError {
          throw const FormatException('Invalid split ratio.');
        }
      default:
        throw const FormatException('Unknown layout node type.');
    }
  }
}

Iterable<String> _paneIds(LayoutNode node) sync* {
  switch (node) {
    case PaneNode(:final paneId):
      yield paneId;
    case SplitNode(:final first, :final second):
      yield* _paneIds(first);
      yield* _paneIds(second);
  }
}

enum PaneDirection { up, down, left, right }

typedef NeighborDirection = PaneDirection;

@immutable
final class LayoutDivider {
  LayoutDivider({
    required List<int> path,
    required this.axis,
    required this.rect,
    required this.containerRect,
    required this.leadingPaneId,
    required this.trailingPaneId,
  }) : path = List.unmodifiable(path);

  final List<int> path;
  final Axis axis;
  final Rect rect;
  final Rect containerRect;
  final String leadingPaneId;
  final String trailingPaneId;
}

@immutable
final class WorkbenchLayoutGeometry {
  WorkbenchLayoutGeometry({
    required Map<String, Rect> paneRects,
    required List<LayoutDivider> dividers,
  }) : paneRects = Map.unmodifiable(paneRects),
       dividers = List.unmodifiable(dividers);

  final Map<String, Rect> paneRects;
  final List<LayoutDivider> dividers;

  String? findNeighbor(String paneId, PaneDirection direction) {
    final source = paneRects[paneId];
    if (source == null) return null;
    _NeighborCandidate? best;
    for (final entry in paneRects.entries) {
      if (entry.key == paneId) continue;
      final candidate = _NeighborCandidate.between(
        entry.key,
        source,
        entry.value,
        direction,
      );
      if (candidate != null &&
          (best == null || candidate.compareTo(best) < 0)) {
        best = candidate;
      }
    }
    return best?.paneId;
  }
}

final class _NeighborCandidate implements Comparable<_NeighborCandidate> {
  const _NeighborCandidate({
    required this.paneId,
    required this.overlaps,
    required this.primaryGap,
    required this.perpendicularGap,
    required this.spatialOrder,
  });

  static _NeighborCandidate? between(
    String paneId,
    Rect source,
    Rect target,
    PaneDirection direction,
  ) {
    const epsilon = 0.000001;
    final (
      eligible,
      primaryGap,
      overlap,
      perpendicularGap,
    ) = switch (direction) {
      PaneDirection.left => (
        target.right <= source.left + epsilon,
        math.max(0.0, source.left - target.right),
        _overlap(source.top, source.bottom, target.top, target.bottom),
        (source.center.dy - target.center.dy).abs(),
      ),
      PaneDirection.right => (
        target.left >= source.right - epsilon,
        math.max(0.0, target.left - source.right),
        _overlap(source.top, source.bottom, target.top, target.bottom),
        (source.center.dy - target.center.dy).abs(),
      ),
      PaneDirection.up => (
        target.bottom <= source.top + epsilon,
        math.max(0.0, source.top - target.bottom),
        _overlap(source.left, source.right, target.left, target.right),
        (source.center.dx - target.center.dx).abs(),
      ),
      PaneDirection.down => (
        target.top >= source.bottom - epsilon,
        math.max(0.0, target.top - source.bottom),
        _overlap(source.left, source.right, target.left, target.right),
        (source.center.dx - target.center.dx).abs(),
      ),
    };
    if (!eligible) return null;
    return _NeighborCandidate(
      paneId: paneId,
      overlaps: overlap > epsilon,
      primaryGap: primaryGap,
      perpendicularGap: perpendicularGap,
      spatialOrder:
          direction == PaneDirection.left || direction == PaneDirection.right
          ? target.top
          : target.left,
    );
  }

  final String paneId;
  final bool overlaps;
  final double primaryGap;
  final double perpendicularGap;
  final double spatialOrder;

  @override
  int compareTo(_NeighborCandidate other) {
    if (overlaps != other.overlaps) return overlaps ? -1 : 1;
    final primary = primaryGap.compareTo(other.primaryGap);
    if (primary != 0) return primary;
    final perpendicular = perpendicularGap.compareTo(other.perpendicularGap);
    if (perpendicular != 0) return perpendicular;
    final spatial = spatialOrder.compareTo(other.spatialOrder);
    if (spatial != 0) return spatial;
    return paneId.compareTo(other.paneId);
  }
}

double _overlap(double aStart, double aEnd, double bStart, double bEnd) =>
    math.max(0, math.min(aEnd, bEnd) - math.max(aStart, bStart));

final class WorkbenchLayoutSolver {
  const WorkbenchLayoutSolver();

  WorkbenchLayoutGeometry solve(
    LayoutNode root,
    Size size, {
    double gutter = 0,
  }) {
    if (!size.width.isFinite ||
        !size.height.isFinite ||
        size.width < 0 ||
        size.height < 0) {
      throw ArgumentError.value(
        size,
        'size',
        'Must be finite and non-negative.',
      );
    }
    if (!gutter.isFinite || gutter < 0) {
      throw ArgumentError.value(
        gutter,
        'gutter',
        'Must be finite and non-negative.',
      );
    }
    final panes = <String, Rect>{};
    final dividers = <LayoutDivider>[];

    void layout(LayoutNode node, Rect rect, List<int> path) {
      switch (node) {
        case PaneNode(:final paneId):
          panes[paneId] = rect;
        case SplitNode(:final axis, :final ratio, :final first, :final second):
          if (axis == Axis.horizontal) {
            final usable = math.max(0.0, rect.width - gutter);
            final firstMinimum = _minimumSize(first, gutter).width;
            final secondMinimum = _minimumSize(second, gutter).width;
            final firstWidth = _constrainedExtent(
              usable,
              ratio,
              firstMinimum,
              secondMinimum,
            );
            final divider = Rect.fromLTWH(
              rect.left + firstWidth,
              rect.top,
              math.min(gutter, rect.width),
              rect.height,
            );
            layout(
              first,
              Rect.fromLTWH(rect.left, rect.top, firstWidth, rect.height),
              [...path, 0],
            );
            dividers.add(
              LayoutDivider(
                path: path,
                axis: axis,
                rect: divider,
                containerRect: rect,
                leadingPaneId: _paneIds(first).first,
                trailingPaneId: _paneIds(second).first,
              ),
            );
            layout(
              second,
              Rect.fromLTWH(
                divider.right,
                rect.top,
                math.max(0, rect.right - divider.right),
                rect.height,
              ),
              [...path, 1],
            );
          } else {
            final usable = math.max(0.0, rect.height - gutter);
            final firstMinimum = _minimumSize(first, gutter).height;
            final secondMinimum = _minimumSize(second, gutter).height;
            final firstHeight = _constrainedExtent(
              usable,
              ratio,
              firstMinimum,
              secondMinimum,
            );
            final divider = Rect.fromLTWH(
              rect.left,
              rect.top + firstHeight,
              rect.width,
              math.min(gutter, rect.height),
            );
            layout(
              first,
              Rect.fromLTWH(rect.left, rect.top, rect.width, firstHeight),
              [...path, 0],
            );
            dividers.add(
              LayoutDivider(
                path: path,
                axis: axis,
                rect: divider,
                containerRect: rect,
                leadingPaneId: _paneIds(first).first,
                trailingPaneId: _paneIds(second).first,
              ),
            );
            layout(
              second,
              Rect.fromLTWH(
                rect.left,
                divider.bottom,
                rect.width,
                math.max(0, rect.bottom - divider.bottom),
              ),
              [...path, 1],
            );
          }
      }
    }

    layout(root, Offset.zero & size, const []);
    return WorkbenchLayoutGeometry(paneRects: panes, dividers: dividers);
  }
}

final class WorkbenchLayoutController extends ChangeNotifier {
  WorkbenchLayoutController(WorkbenchLayoutSnapshot initial)
    : _snapshot = initial;

  WorkbenchLayoutSnapshot _snapshot;
  var _nextPaneNumber = 1;

  WorkbenchLayoutSnapshot get snapshot => _snapshot;
  WorkbenchLayoutSnapshot get value => _snapshot;

  void restore(WorkbenchLayoutSnapshot snapshot) => _setSnapshot(snapshot);

  void reset() => _setSnapshot(createDefaultWorkbenchLayout());

  String splitPane(
    String paneId, {
    required Axis axis,
    double ratio = 0.5,
    String? newPaneId,
    bool newPaneFirst = false,
  }) {
    _requirePane(paneId);
    final id = newPaneId ?? _allocatePaneId();
    if (_snapshot.paneIds.contains(id)) {
      throw ArgumentError.value(id, 'newPaneId', 'Pane ID already exists.');
    }
    final existing = PaneNode(paneId: paneId);
    final added = PaneNode(paneId: id);
    final split = SplitNode(
      axis: axis,
      ratio: ratio,
      first: newPaneFirst ? added : existing,
      second: newPaneFirst ? existing : added,
    );
    _setSnapshot(
      _snapshot.copyWith(
        root: _replacePane(_snapshot.root, paneId, split),
        focusedPaneId: id,
      ),
    );
    return id;
  }

  bool closeEmptyPane(String paneId) {
    _requirePane(paneId);
    if (_snapshot.bindings.containsKey(paneId)) {
      throw StateError('Only an empty pane can be closed.');
    }
    if (_snapshot.root is PaneNode) return false;
    final root = _removePane(_snapshot.root, paneId);
    if (root == null) return false;
    final focus = _snapshot.focusedPaneId == paneId
        ? _paneIds(root).first
        : _snapshot.focusedPaneId;
    _setSnapshot(_snapshot.copyWith(root: root, focusedPaneId: focus));
    return true;
  }

  bool closePane(String paneId) => closeEmptyPane(paneId);

  void bind(String paneId, SurfaceBinding binding) {
    _requirePane(paneId);
    for (final entry in _snapshot.bindings.entries) {
      if (entry.key != paneId && entry.value.bindingId == binding.bindingId) {
        throw StateError('Binding ID already exists: ${binding.bindingId}');
      }
    }
    _setSnapshot(
      _snapshot.copyWith(bindings: {..._snapshot.bindings, paneId: binding}),
    );
  }

  SurfaceBinding? unbind(String paneId) {
    _requirePane(paneId);
    final removed = _snapshot.bindings[paneId];
    if (removed == null) return null;
    final bindings = {..._snapshot.bindings}..remove(paneId);
    _setSnapshot(_snapshot.copyWith(bindings: bindings));
    return removed;
  }

  void move(String fromPaneId, String toPaneId, {bool replace = false}) {
    _requirePane(fromPaneId);
    _requirePane(toPaneId);
    if (fromPaneId == toPaneId) return;
    final binding = _snapshot.bindings[fromPaneId];
    if (binding == null) throw StateError('Source pane is empty.');
    if (_snapshot.bindings.containsKey(toPaneId) && !replace) {
      throw StateError('Destination pane is not empty.');
    }
    final bindings = {..._snapshot.bindings}
      ..remove(fromPaneId)
      ..[toPaneId] = binding;
    _setSnapshot(
      _snapshot.copyWith(bindings: bindings, focusedPaneId: toPaneId),
    );
  }

  void moveBinding(
    String fromPaneId,
    String toPaneId, {
    bool replace = false,
  }) => move(fromPaneId, toPaneId, replace: replace);

  void swap(String firstPaneId, String secondPaneId) {
    _requirePane(firstPaneId);
    _requirePane(secondPaneId);
    if (firstPaneId == secondPaneId) return;
    final first = _snapshot.bindings[firstPaneId];
    final second = _snapshot.bindings[secondPaneId];
    final bindings = {..._snapshot.bindings};
    if (second == null) {
      bindings.remove(firstPaneId);
    } else {
      bindings[firstPaneId] = second;
    }
    if (first == null) {
      bindings.remove(secondPaneId);
    } else {
      bindings[secondPaneId] = first;
    }
    _setSnapshot(_snapshot.copyWith(bindings: bindings));
  }

  void swapBindings(String firstPaneId, String secondPaneId) =>
      swap(firstPaneId, secondPaneId);

  void focus(String paneId) {
    _requirePane(paneId);
    if (_snapshot.focusedPaneId == paneId) return;
    _setSnapshot(_snapshot.copyWith(focusedPaneId: paneId));
  }

  void resize(String paneId, double ratio) {
    _requirePane(paneId);
    final safeRatio = _validateRatio(ratio);
    final updated = _resizeParent(_snapshot.root, paneId, safeRatio);
    if (identical(updated, _snapshot.root)) {
      throw StateError('The root pane has no divider to resize.');
    }
    _setSnapshot(_snapshot.copyWith(root: updated));
  }

  double parentRatio(String paneId) {
    _requirePane(paneId);
    final ratio = _parentRatio(_snapshot.root, paneId);
    if (ratio == null) {
      throw StateError('The root pane has no divider.');
    }
    return ratio;
  }

  void resizeBetween(
    String leadingPaneId,
    String trailingPaneId,
    double ratio,
  ) {
    _requirePane(leadingPaneId);
    _requirePane(trailingPaneId);
    final safeRatio = _validateRatio(ratio);
    final updated = _resizeBetween(
      _snapshot.root,
      leadingPaneId,
      trailingPaneId,
      safeRatio,
    );
    if (identical(updated, _snapshot.root)) {
      throw StateError('Panes do not identify a shared divider.');
    }
    _setSnapshot(_snapshot.copyWith(root: updated));
  }

  double ratioBetween(String leadingPaneId, String trailingPaneId) {
    _requirePane(leadingPaneId);
    _requirePane(trailingPaneId);
    final ratio = _ratioBetween(_snapshot.root, leadingPaneId, trailingPaneId);
    if (ratio == null) {
      throw StateError('Panes do not identify a shared divider.');
    }
    return ratio;
  }

  void resizeSplit(List<int> path, double ratio) {
    final safeRatio = _validateRatio(ratio);
    final updated = _resizeAtPath(_snapshot.root, path, safeRatio, 0);
    _setSnapshot(_snapshot.copyWith(root: updated));
  }

  double ratioAtPath(List<int> path) {
    LayoutNode node = _snapshot.root;
    for (final direction in path) {
      if (node is! SplitNode || (direction != 0 && direction != 1)) {
        throw ArgumentError.value(path, 'path', 'Invalid split path.');
      }
      node = direction == 0 ? node.first : node.second;
    }
    if (node is! SplitNode) {
      throw ArgumentError.value(
        path,
        'path',
        'Path does not identify a split.',
      );
    }
    return node.ratio;
  }

  String? findNeighbor(
    String paneId,
    PaneDirection direction, {
    required Size size,
    double gutter = 0,
  }) {
    _requirePane(paneId);
    return const WorkbenchLayoutSolver()
        .solve(_snapshot.root, size, gutter: gutter)
        .findNeighbor(paneId, direction);
  }

  void _setSnapshot(WorkbenchLayoutSnapshot value) {
    if (value == _snapshot) return;
    _snapshot = value;
    notifyListeners();
  }

  void _requirePane(String paneId) {
    if (!_snapshot.paneIds.contains(paneId)) {
      throw ArgumentError.value(paneId, 'paneId', 'Pane does not exist.');
    }
  }

  String _allocatePaneId() {
    while (true) {
      final candidate = 'pane-${_nextPaneNumber++}';
      if (!_snapshot.paneIds.contains(candidate)) return candidate;
    }
  }
}

LayoutNode _replacePane(
  LayoutNode node,
  String paneId,
  LayoutNode replacement,
) {
  switch (node) {
    case PaneNode():
      return node.paneId == paneId ? replacement : node;
    case SplitNode():
      final first = _replacePane(node.first, paneId, replacement);
      final second = _replacePane(node.second, paneId, replacement);
      if (identical(first, node.first) && identical(second, node.second)) {
        return node;
      }
      return node.copyWith(first: first, second: second);
  }
}

LayoutNode? _removePane(LayoutNode node, String paneId) {
  switch (node) {
    case PaneNode():
      return node.paneId == paneId ? null : node;
    case SplitNode():
      final first = _removePane(node.first, paneId);
      final second = _removePane(node.second, paneId);
      if (first == null) return second;
      if (second == null) return first;
      if (identical(first, node.first) && identical(second, node.second)) {
        return node;
      }
      return node.copyWith(first: first, second: second);
  }
}

LayoutNode _resizeParent(LayoutNode node, String paneId, double ratio) {
  if (node is! SplitNode) return node;
  if ((node.first is PaneNode && (node.first as PaneNode).paneId == paneId) ||
      (node.second is PaneNode && (node.second as PaneNode).paneId == paneId)) {
    return node.copyWith(ratio: ratio);
  }
  final first = _resizeParent(node.first, paneId, ratio);
  if (!identical(first, node.first)) return node.copyWith(first: first);
  final second = _resizeParent(node.second, paneId, ratio);
  return identical(second, node.second) ? node : node.copyWith(second: second);
}

double? _parentRatio(LayoutNode node, String paneId) {
  if (node is! SplitNode) return null;
  if ((node.first is PaneNode && (node.first as PaneNode).paneId == paneId) ||
      (node.second is PaneNode && (node.second as PaneNode).paneId == paneId)) {
    return node.ratio;
  }
  return _parentRatio(node.first, paneId) ?? _parentRatio(node.second, paneId);
}

LayoutNode _resizeBetween(
  LayoutNode node,
  String leadingPaneId,
  String trailingPaneId,
  double ratio,
) {
  if (node is! SplitNode) return node;
  final firstIds = _paneIds(node.first).toSet();
  final secondIds = _paneIds(node.second).toSet();
  if (firstIds.contains(leadingPaneId) && secondIds.contains(trailingPaneId)) {
    return node.copyWith(ratio: ratio);
  }
  if (firstIds.contains(leadingPaneId) && firstIds.contains(trailingPaneId)) {
    final first = _resizeBetween(
      node.first,
      leadingPaneId,
      trailingPaneId,
      ratio,
    );
    return identical(first, node.first) ? node : node.copyWith(first: first);
  }
  if (secondIds.contains(leadingPaneId) && secondIds.contains(trailingPaneId)) {
    final second = _resizeBetween(
      node.second,
      leadingPaneId,
      trailingPaneId,
      ratio,
    );
    return identical(second, node.second)
        ? node
        : node.copyWith(second: second);
  }
  return node;
}

double? _ratioBetween(
  LayoutNode node,
  String leadingPaneId,
  String trailingPaneId,
) {
  if (node is! SplitNode) return null;
  final firstIds = _paneIds(node.first).toSet();
  final secondIds = _paneIds(node.second).toSet();
  if (firstIds.contains(leadingPaneId) && secondIds.contains(trailingPaneId)) {
    return node.ratio;
  }
  if (firstIds.contains(leadingPaneId) && firstIds.contains(trailingPaneId)) {
    return _ratioBetween(node.first, leadingPaneId, trailingPaneId);
  }
  if (secondIds.contains(leadingPaneId) && secondIds.contains(trailingPaneId)) {
    return _ratioBetween(node.second, leadingPaneId, trailingPaneId);
  }
  return null;
}

LayoutNode _resizeAtPath(
  LayoutNode node,
  List<int> path,
  double ratio,
  int index,
) {
  if (index == path.length) {
    if (node is! SplitNode) {
      throw ArgumentError.value(
        path,
        'path',
        'Path does not identify a split.',
      );
    }
    return node.copyWith(ratio: ratio);
  }
  if (node is! SplitNode || (path[index] != 0 && path[index] != 1)) {
    throw ArgumentError.value(path, 'path', 'Invalid split path.');
  }
  return path[index] == 0
      ? node.copyWith(first: _resizeAtPath(node.first, path, ratio, index + 1))
      : node.copyWith(
          second: _resizeAtPath(node.second, path, ratio, index + 1),
        );
}

abstract interface class LayoutSnapshotWriter {
  Future<void> save(WorkbenchLayoutSnapshot snapshot);
}

WorkbenchLayoutSnapshot createDefaultWorkbenchLayout({
  double sidebarRatio = 0.22,
  double editorRatio = 0.72,
}) => WorkbenchLayoutSnapshot(
  root: SplitNode(
    axis: Axis.horizontal,
    ratio: sidebarRatio,
    first: PaneNode(paneId: 'workspace', minWidth: 232),
    second: SplitNode(
      axis: Axis.horizontal,
      ratio: editorRatio,
      first: PaneNode(paneId: 'editor', minWidth: 280),
      second: SplitNode(
        axis: Axis.vertical,
        ratio: 0.62,
        first: PaneNode(paneId: 'dsh', minWidth: 220),
        second: PaneNode(paneId: 'cloud-workspace', minWidth: 220),
      ),
    ),
  ),
  bindings: {
    'workspace': SurfaceBinding(
      bindingId: 'binding.workspace',
      surfaceRef: 'host.workspaceExplorer',
      instanceRef: 'surface.workspace',
      mobility: SurfaceMobility.snapshotRestore,
    ),
    'editor': SurfaceBinding(
      bindingId: 'binding.editor.primary',
      surfaceRef: 'host.editorGroup:editor.primary',
      instanceRef: 'surface.editor.primary',
      mobility: SurfaceMobility.snapshotRestore,
    ),
    'dsh': SurfaceBinding(
      bindingId: 'binding.dsh',
      surfaceRef: 'plugin.panel:dsh.agent',
      instanceRef: 'surface.dsh',
      mobility: SurfaceMobility.snapshotRestore,
    ),
    'cloud-workspace': SurfaceBinding(
      bindingId: 'binding.cloud-workspace',
      surfaceRef: 'plugin.panel:cloud.workspace',
      instanceRef: 'surface.cloud-workspace',
      mobility: SurfaceMobility.snapshotRestore,
    ),
  },
  focusedPaneId: 'editor',
);

abstract final class WorkbenchLayoutDefaults {
  static WorkbenchLayoutSnapshot threePane() => createDefaultWorkbenchLayout();
}

Size _minimumSize(LayoutNode node, double gutter) => switch (node) {
  PaneNode(:final minWidth, :final minHeight) => Size(minWidth, minHeight),
  SplitNode(axis: Axis.horizontal, :final first, :final second) => Size(
    _minimumSize(first, gutter).width +
        gutter +
        _minimumSize(second, gutter).width,
    math.max(
      _minimumSize(first, gutter).height,
      _minimumSize(second, gutter).height,
    ),
  ),
  SplitNode(axis: Axis.vertical, :final first, :final second) => Size(
    math.max(
      _minimumSize(first, gutter).width,
      _minimumSize(second, gutter).width,
    ),
    _minimumSize(first, gutter).height +
        gutter +
        _minimumSize(second, gutter).height,
  ),
};

double _constrainedExtent(
  double usable,
  double ratio,
  double firstMinimum,
  double secondMinimum,
) {
  if (usable <= 0) return 0;
  final minimumTotal = firstMinimum + secondMinimum;
  if (minimumTotal > usable && minimumTotal > 0) {
    return usable * (firstMinimum / minimumTotal);
  }
  return (usable * ratio).clamp(firstMinimum, usable - secondMinimum);
}
