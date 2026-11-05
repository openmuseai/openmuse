import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';

final class MuseVersionGraphEdge {
  const MuseVersionGraphEdge({required this.parent, required this.child});

  final MuseVersionRef parent;
  final MuseVersionRef child;
}

final class MuseVersionGraphNode {
  const MuseVersionGraphNode({
    required this.version,
    required this.depth,
    required this.isHead,
    required this.missingParentIds,
  });

  final MuseVersion version;
  final int depth;
  final bool isHead;
  final List<String> missingParentIds;
}

final class MuseVersionGraph {
  const MuseVersionGraph({required this.nodes, required this.edges});

  final List<MuseVersionGraphNode> nodes;
  final List<MuseVersionGraphEdge> edges;
}

/// Builds a bounded, provider-neutral Version DAG projection.
///
/// Lane geometry stays in the UI. This model only derives factual parent
/// edges, heads and a stable ancestor depth useful for list/tree renderers.
final class MuseVersionGraphProjector {
  const MuseVersionGraphProjector();

  MuseVersionGraph project(Iterable<MuseVersion> versions) {
    final byId = {for (final version in versions) version.ref.id: version};
    final childIds = <String>{};
    final edges = <MuseVersionGraphEdge>[];
    for (final version in byId.values) {
      for (final parent in version.parents) {
        if (byId.containsKey(parent.id)) {
          edges.add(MuseVersionGraphEdge(parent: parent, child: version.ref));
          childIds.add(parent.id);
        }
      }
    }

    int depthOf(String id, Set<String> visiting, Map<String, int> cache) {
      final cached = cache[id];
      if (cached != null) return cached;
      if (!visiting.add(id)) return 0;
      final version = byId[id];
      if (version == null || version.parents.isEmpty) {
        visiting.remove(id);
        return cache[id] = 0;
      }
      var depth = 0;
      for (final parent in version.parents) {
        if (byId.containsKey(parent.id)) {
          final candidate = depthOf(parent.id, visiting, cache) + 1;
          if (candidate > depth) depth = candidate;
        }
      }
      visiting.remove(id);
      return cache[id] = depth;
    }

    final depthCache = <String, int>{};
    final ordered = byId.values.toList()
      ..sort((a, b) {
        final byTime = b.createdAt.compareTo(a.createdAt);
        return byTime != 0 ? byTime : a.ref.id.compareTo(b.ref.id);
      });
    final nodes = [
      for (final version in ordered)
        MuseVersionGraphNode(
          version: version,
          depth: depthOf(version.ref.id, <String>{}, depthCache),
          isHead: !childIds.contains(version.ref.id),
          missingParentIds: [
            for (final parent in version.parents)
              if (!byId.containsKey(parent.id)) parent.id,
          ],
        ),
    ];
    return MuseVersionGraph(
      nodes: List.unmodifiable(nodes),
      edges: List.unmodifiable(edges),
    );
  }
}
