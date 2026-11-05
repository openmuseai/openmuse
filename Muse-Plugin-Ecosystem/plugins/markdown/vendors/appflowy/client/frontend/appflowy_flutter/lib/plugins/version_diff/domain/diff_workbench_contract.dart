import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';

/// Protocol-only contracts for Universal Diff Workbench 1.0.
///
/// These objects deliberately do not import Flutter. A domain provider emits a
/// semantic change set; a renderer projects its anchors into its own viewport.
enum MuseDiffSide { before, after, local, base, incoming }

enum MuseUniversalChangeKind {
  insert,
  delete,
  modify,
  move,
  format,
  structure,
  conflict,
}

enum MuseDiffQualityKind {
  exact,
  structural,
  heuristic,
  textFallback,
  binaryOnly,
  budgetExceeded,
  unsupported,
  failed,
}

enum MuseDiffViewMode { sideBySide, unified, overlay, timeline, threeWay }

final class MuseDiffInput {
  const MuseDiffInput({
    required this.side,
    required this.version,
    required this.title,
    this.editable = false,
  });

  final MuseDiffSide side;
  final MuseVersionRef version;
  final String title;
  final bool editable;
}

final class MuseComparisonOptions {
  const MuseComparisonOptions({
    this.algorithmPolicy = 'default',
    this.ignorePolicy = 'none',
    this.highlightPolicy = 'semantic',
    this.semanticDepth = 'balanced',
    this.computationBudget = const Duration(seconds: 10),
    this.memoryBudgetBytes = 256 * 1024 * 1024,
  });

  final String algorithmPolicy;
  final String ignorePolicy;
  final String highlightPolicy;
  final String semanticDepth;
  final Duration computationBudget;
  final int memoryBudgetBytes;
}

final class MuseDiffOpenContext {
  const MuseDiffOpenContext({
    required this.source,
    this.initialChangeId,
    this.preferredMode,
  });

  final String source;
  final String? initialChangeId;
  final MuseDiffViewMode? preferredMode;
}

final class MuseDiffRequest {
  MuseDiffRequest({
    required this.requestId,
    required this.comparisonId,
    required this.resource,
    required List<MuseDiffInput> inputs,
    this.options = const MuseComparisonOptions(),
    this.context = const MuseDiffOpenContext(source: 'host'),
  }) : inputs = List.unmodifiable(inputs) {
    if (inputs.length != 2 && inputs.length != 3) {
      throw ArgumentError.value(
        inputs.length,
        'inputs.length',
        'A diff request must contain two or three inputs',
      );
    }
  }

  final String requestId;
  final String comparisonId;
  final MuseResourceRef resource;
  final List<MuseDiffInput> inputs;
  final MuseComparisonOptions options;
  final MuseDiffOpenContext context;
}

sealed class MuseDiffAnchor {
  const MuseDiffAnchor(this.schema);

  final String schema;
}

final class MuseTextRangeAnchor extends MuseDiffAnchor {
  const MuseTextRangeAnchor({
    required this.startLine,
    required this.endLine,
    this.startColumn = 0,
    this.endColumn = 0,
  }) : super('muse.diff.anchor.text-range.v1');

  final int startLine;
  final int endLine;
  final int startColumn;
  final int endColumn;
}

final class MusePageRegionAnchor extends MuseDiffAnchor {
  const MusePageRegionAnchor({
    required this.page,
    required this.left,
    required this.top,
    required this.right,
    required this.bottom,
  }) : super('muse.diff.anchor.page-region.v1');

  final int page;
  final double left;
  final double top;
  final double right;
  final double bottom;
}

final class MuseCellRangeAnchor extends MuseDiffAnchor {
  const MuseCellRangeAnchor({
    required this.sheetId,
    required this.startRow,
    required this.endRow,
    required this.startColumn,
    required this.endColumn,
  }) : super('muse.diff.anchor.cell-range.v1');

  final String sheetId;
  final int startRow;
  final int endRow;
  final int startColumn;
  final int endColumn;
}

final class MuseTimelineRangeAnchor extends MuseDiffAnchor {
  const MuseTimelineRangeAnchor({
    required this.startMicroseconds,
    required this.endMicroseconds,
    this.trackId,
  }) : super('muse.diff.anchor.timeline-range.v1');

  final int startMicroseconds;
  final int endMicroseconds;
  final String? trackId;
}

final class MuseSemanticNodeAnchor extends MuseDiffAnchor {
  const MuseSemanticNodeAnchor({
    required this.nodeId,
    required this.nodeType,
    this.propertyPath,
  }) : super('muse.diff.anchor.semantic-node.v1');

  final String nodeId;
  final String nodeType;
  final String? propertyPath;
}

final class MuseDiffQuality {
  const MuseDiffQuality({
    required this.kind,
    this.confidence,
    this.reason,
  });

  final MuseDiffQualityKind kind;
  final double? confidence;
  final String? reason;
}

final class MuseChangeAttribution {
  const MuseChangeAttribution({
    required this.actor,
    this.taskId,
    this.agentSessionId,
    this.toolCallId,
    this.intent,
    this.confidence,
  });

  final MuseActorRef actor;
  final String? taskId;
  final String? agentSessionId;
  final String? toolCallId;
  final String? intent;
  final double? confidence;
}

final class MuseSemanticChange {
  MuseSemanticChange({
    required this.id,
    required this.kind,
    required this.semanticPath,
    required this.label,
    List<MuseDiffAnchor> before = const [],
    List<MuseDiffAnchor> after = const [],
    List<String> childIds = const [],
    this.attribution,
    Map<String, Object?> properties = const {},
  })  : before = List.unmodifiable(before),
        after = List.unmodifiable(after),
        childIds = List.unmodifiable(childIds),
        properties = Map.unmodifiable(properties);

  final String id;
  final MuseUniversalChangeKind kind;
  final String semanticPath;
  final String label;
  final List<MuseDiffAnchor> before;
  final List<MuseDiffAnchor> after;
  final List<String> childIds;
  final MuseChangeAttribution? attribution;
  final Map<String, Object?> properties;
}

final class MuseSemanticChangeSet {
  MuseSemanticChangeSet({
    required this.providerId,
    required this.providerVersion,
    required this.comparisonId,
    required this.resource,
    required List<MuseSemanticChange> changes,
    required this.quality,
    this.schema = 'muse.diff.changeset.v1',
    Map<String, Object?> summary = const {},
    Map<String, Object?> domainIndex = const {},
  })  : changes = List.unmodifiable(changes),
        summary = Map.unmodifiable(summary),
        domainIndex = Map.unmodifiable(domainIndex);

  final String schema;
  final String providerId;
  final String providerVersion;
  final String comparisonId;
  final MuseResourceRef resource;
  final List<MuseSemanticChange> changes;
  final MuseDiffQuality quality;
  final Map<String, Object?> summary;
  final Map<String, Object?> domainIndex;
}

final class MuseDiffRendererCapabilities {
  const MuseDiffRendererCapabilities({
    this.selection = false,
    this.copy = false,
    this.search = false,
    this.syncScroll = false,
    this.alignment = false,
    this.folding = false,
    this.overview = false,
    this.editableSide = false,
    this.threeWay = false,
    this.partialLoading = false,
  });

  final bool selection;
  final bool copy;
  final bool search;
  final bool syncScroll;
  final bool alignment;
  final bool folding;
  final bool overview;
  final bool editableSide;
  final bool threeWay;
  final bool partialLoading;
}

final class MuseDiffRendererManifest {
  MuseDiffRendererManifest({
    required this.id,
    required Set<String> supportedChangeSchemas,
    required Set<MuseDiffViewMode> modes,
    required this.capabilities,
  })  : supportedChangeSchemas = Set.unmodifiable(supportedChangeSchemas),
        modes = Set.unmodifiable(modes);

  final String id;
  final Set<String> supportedChangeSchemas;
  final Set<MuseDiffViewMode> modes;
  final MuseDiffRendererCapabilities capabilities;
}

abstract interface class MuseSemanticDiffProvider {
  String get semanticProviderId;

  String get semanticProviderVersion;

  bool supportsSemanticDiff(MuseResourceRef resource);

  Future<MuseSemanticChangeSet> compareSemantic({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  });
}

final class MuseSemanticDiffProviderRegistry {
  final List<MuseSemanticDiffProvider> _providers = [];

  void register(MuseSemanticDiffProvider provider) {
    _providers.removeWhere(
      (candidate) =>
          candidate.semanticProviderId == provider.semanticProviderId,
    );
    _providers.add(provider);
  }

  void unregister(String id) {
    _providers.removeWhere((provider) => provider.semanticProviderId == id);
  }

  MuseSemanticDiffProvider? providerFor(MuseResourceRef resource) {
    for (final provider in _providers.reversed) {
      if (provider.supportsSemanticDiff(resource)) return provider;
    }
    return null;
  }
}

final class MuseDiffRendererRegistry {
  final List<MuseDiffRendererManifest> _renderers = [];

  void register(MuseDiffRendererManifest renderer) {
    _renderers.removeWhere((candidate) => candidate.id == renderer.id);
    _renderers.add(renderer);
  }

  void unregister(String id) {
    _renderers.removeWhere((renderer) => renderer.id == id);
  }

  MuseDiffRendererManifest? resolve({
    required String changeSetSchema,
    required MuseDiffViewMode preferredMode,
  }) {
    for (final renderer in _renderers.reversed) {
      if (renderer.supportedChangeSchemas.contains(changeSetSchema) &&
          renderer.modes.contains(preferredMode)) {
        return renderer;
      }
    }
    for (final renderer in _renderers.reversed) {
      if (renderer.supportedChangeSchemas.contains(changeSetSchema)) {
        return renderer;
      }
    }
    return null;
  }

  List<MuseDiffRendererManifest> get manifests => List.unmodifiable(_renderers);
}
