import 'dart:async';
import 'dart:convert';
import 'dart:math';

import 'package:muse_plugin_facets/muse_plugin_facets.dart';

final class MuseContribution {
  const MuseContribution({
    required this.ownerRef,
    required this.slot,
    required this.contributionId,
    required this.generation,
    required this.priority,
    required this.activate,
    this.replaces = const <String>{},
  });

  final String ownerRef;
  final String slot;
  final String contributionId;
  final int generation;
  final int priority;
  final Set<String> replaces;
  final FutureOr<void> Function(MuseFacetLease lease) activate;
}

final class MuseContributionDiagnostic {
  const MuseContributionDiagnostic(this.code, this.subject, this.selectedOwner);
  final String code;
  final String subject;
  final String selectedOwner;
}

final class MuseContributionRegistry {
  final _selected = <String, MuseContribution>{};
  final _diagnostics = <MuseContributionDiagnostic>[];

  List<MuseContributionDiagnostic> get diagnostics =>
      List.unmodifiable(_diagnostics);
  List<MuseContribution> get selected =>
      _selected.values.toList(growable: false)
        ..sort((a, b) => _key(a).compareTo(_key(b)));

  void register(MuseContribution contribution) {
    final key = '${contribution.slot}/${contribution.contributionId}';
    final current = _selected[key];
    if (current == null) {
      _selected[key] = contribution;
      return;
    }
    final winner = _compare(contribution, current) < 0 ? contribution : current;
    _selected[key] = winner;
    _diagnostics.add(MuseContributionDiagnostic(
      'CONTRIBUTION_CONFLICT',
      key,
      winner.ownerRef,
    ));
  }

  void removeGeneration(int generation) {
    _selected.removeWhere((_, value) => value.generation == generation);
    _diagnostics.removeWhere((value) =>
        !_selected.values.any((item) => item.ownerRef == value.selectedOwner));
  }

  static int _compare(MuseContribution a, MuseContribution b) =>
      b.priority.compareTo(a.priority) != 0
          ? b.priority.compareTo(a.priority)
          : a.ownerRef.compareTo(b.ownerRef);
  static String _key(MuseContribution value) =>
      '${value.slot}/${value.contributionId}/${value.ownerRef}';
}

final class MuseFacetLease {
  MuseFacetLease({
    required this.facetRef,
    required this.generation,
  });

  final String facetRef;
  final int generation;
  final _disposers = <FutureOr<void> Function()>[];
  var _closed = false;

  bool get isClosed => _closed;
  int get effectCount => _disposers.length;

  T own<T>(T value, FutureOr<void> Function(T value) dispose) {
    if (_closed) throw StateError('FACET_LEASE_CLOSED');
    _disposers.add(() => dispose(value));
    return value;
  }

  void add(FutureOr<void> Function() dispose) {
    if (_closed) throw StateError('FACET_LEASE_CLOSED');
    _disposers.add(dispose);
  }

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    Object? firstError;
    for (final dispose in _disposers.reversed) {
      try {
        await dispose();
      } catch (error) {
        firstError ??= error;
      }
    }
    _disposers.clear();
    if (firstError != null) throw firstError;
  }
}

final class MuseFocusCandidate {
  const MuseFocusCandidate({
    required this.surfaceRef,
    required this.windowRef,
    required this.visible,
    required this.lastUserInputAt,
    required this.focused,
  });
  final String surfaceRef;
  final String windowRef;
  final bool visible;
  final int lastUserInputAt;
  final bool focused;
}

final class MuseFocusArbiter {
  final _candidates = <String, MuseFocusCandidate>{};
  String? _pinned;

  void update(MuseFocusCandidate candidate) =>
      _candidates[candidate.surfaceRef] = candidate;
  void remove(String surfaceRef) {
    _candidates.remove(surfaceRef);
    if (_pinned == surfaceRef) _pinned = null;
  }

  void pin(String surfaceRef) {
    if (!_candidates.containsKey(surfaceRef)) {
      throw StateError('SURFACE_NOT_FOUND');
    }
    _pinned = surfaceRef;
  }

  void unpin() => _pinned = null;

  String? get primary {
    final pinned = _pinned == null ? null : _candidates[_pinned];
    if (pinned != null && pinned.visible) return pinned.surfaceRef;
    final values = _candidates.values.where((value) => value.visible).toList()
      ..sort((a, b) {
        final focused = (b.focused ? 1 : 0).compareTo(a.focused ? 1 : 0);
        if (focused != 0) return focused;
        final recent = b.lastUserInputAt.compareTo(a.lastUserInputAt);
        if (recent != 0) return recent;
        return a.surfaceRef.compareTo(b.surfaceRef);
      });
    return values.firstOrNull?.surfaceRef;
  }
}

final class MuseContextSnapshot {
  const MuseContextSnapshot({
    required this.primarySurfaceRef,
    required this.items,
    required this.bytes,
  });
  final String? primarySurfaceRef;
  final List<MuseContextContributionV1> items;
  final int bytes;
}

final class MuseContextCoordinator {
  MuseContextCoordinator({
    required MuseFocusArbiter focus,
    int Function()? clock,
    this.maxSnapshotBytes = 32 * 1024,
  })  : _focus = focus,
        _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);

  final MuseFocusArbiter _focus;
  final int Function() _clock;
  final int maxSnapshotBytes;
  final _latest = <String, MuseContextContributionV1>{};

  int get itemCount => _latest.length;

  void ingest(MuseContextContributionV1 value) {
    if (value.expiresAt <= _clock()) return;
    final key = '${value.surfaceInstanceRef}/${value.contextType}';
    final previous = _latest[key];
    if (previous != null &&
        int.parse(previous.contextRevision) >=
            int.parse(value.contextRevision)) {
      return;
    }
    _latest[key] = value;
  }

  void revokeSurface(String surfaceRef) =>
      _latest.removeWhere((_, value) => value.surfaceInstanceRef == surfaceRef);

  MuseContextSnapshot snapshot() {
    final now = _clock();
    _latest.removeWhere((_, value) => value.expiresAt <= now);
    final primary = _focus.primary;
    final candidates = _latest.values
        .where((value) => value.surfaceInstanceRef == primary)
        .toList()
      ..sort((a, b) {
        final lane = (a.lane == MuseContextLane.control ? 0 : 1)
            .compareTo(b.lane == MuseContextLane.control ? 0 : 1);
        return lane != 0 ? lane : a.contextType.compareTo(b.contextType);
      });
    final selected = <MuseContextContributionV1>[];
    var bytes = 0;
    for (final value in candidates) {
      final size = utf8.encode(jsonEncode(value.toJson())).length;
      if (bytes + size > maxSnapshotBytes) continue;
      selected.add(value);
      bytes += size;
    }
    return MuseContextSnapshot(
      primarySurfaceRef: primary,
      items: List.unmodifiable(selected),
      bytes: bytes,
    );
  }
}

enum MuseRendererKind { declarative, typedFlutter, dshWebCompatibility }

final class MuseRendererRegistration {
  const MuseRendererRegistration({
    required this.rendererId,
    required this.kind,
    required this.surfaceKinds,
    required this.build,
  });
  final String rendererId;
  final MuseRendererKind kind;
  final Set<String> surfaceKinds;
  final Object Function(String surfaceKind, Object? payload) build;
}

final class MuseRendererRegistry {
  final _bySurfaceKind = <String, MuseRendererRegistration>{};

  void register(MuseRendererRegistration registration, MuseFacetLease lease) {
    for (final kind in registration.surfaceKinds) {
      if (_bySurfaceKind.containsKey(kind)) {
        throw StateError('RENDERER_CONFLICT:$kind');
      }
      _bySurfaceKind[kind] = registration;
      lease.add(() => _bySurfaceKind.remove(kind));
    }
  }

  Object render(String surfaceKind, Object? payload) {
    final renderer = _bySurfaceKind[surfaceKind];
    if (renderer == null) throw StateError('RENDERER_NOT_FOUND');
    return renderer.build(surfaceKind, payload);
  }

  int get count => _bySurfaceKind.length;
}

final class MuseApprovalRequest {
  const MuseApprovalRequest({
    required this.requestRef,
    required this.operation,
    required this.expiresAt,
  });
  final String requestRef;
  final String operation;
  final int expiresAt;
}

final class MuseApprovalGrant {
  const MuseApprovalGrant._(
    this.requestRef,
    this.grantRef,
    this.expiresAt,
    this._proof,
  );
  final String requestRef;
  final String grantRef;
  final int expiresAt;
  final String _proof;
}

final class MuseTrustedApprovalHost {
  MuseTrustedApprovalHost({int Function()? clock})
      : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch),
        _secret = _randomRef();
  final int Function() _clock;
  final String _secret;

  MuseApprovalGrant approve(MuseApprovalRequest request) {
    if (request.expiresAt <= _clock()) throw StateError('APPROVAL_EXPIRED');
    return MuseApprovalGrant._(
      request.requestRef,
      _randomRef(),
      request.expiresAt,
      '$_secret:${request.requestRef}',
    );
  }

  bool verify(MuseApprovalGrant grant) =>
      grant.expiresAt > _clock() &&
      grant._proof == '$_secret:${grant.requestRef}';
}

final class MuseWebCompatibilitySurface {
  MuseWebCompatibilitySurface({
    required this.origin,
    required this.csp,
    required this.storageNamespace,
  }) {
    final uri = Uri.parse(origin);
    if (uri.scheme != 'http' || uri.host != '127.0.0.1') {
      throw ArgumentError.value(origin, 'origin', 'loopback origin required');
    }
    if (!csp.contains("frame-ancestors 'none'") ||
        !csp.contains("default-src 'none'")) {
      throw ArgumentError.value(csp, 'csp', 'strict CSP required');
    }
    if (storageNamespace.isEmpty) {
      throw ArgumentError.value(storageNamespace, 'storageNamespace');
    }
  }
  final String origin;
  final String csp;
  final String storageNamespace;
}

final class MusePresentationKernel {
  final contributions = MuseContributionRegistry();
  final focus = MuseFocusArbiter();
  final renderers = MuseRendererRegistry();
  final _leases = <int, List<MuseFacetLease>>{};

  MuseFacetLease createFacetLease(String facetRef, int generation) {
    final lease = MuseFacetLease(facetRef: facetRef, generation: generation);
    (_leases[generation] ??= []).add(lease);
    return lease;
  }

  Future<void> activateGeneration(int generation) async {
    for (final contribution in contributions.selected
        .where((value) => value.generation == generation)) {
      final lease = createFacetLease(contribution.ownerRef, generation);
      await contribution.activate(lease);
    }
  }

  Future<void> disposeGeneration(int generation) async {
    for (final lease in (_leases.remove(generation) ?? const []).reversed) {
      await lease.close();
    }
    contributions.removeGeneration(generation);
  }

  int effectsForGeneration(int generation) => (_leases[generation] ?? const [])
      .fold(0, (sum, lease) => sum + lease.effectCount);
}

List<MuseContribution> museDefaultExperienceContributions({
  required int generation,
  required FutureOr<void> Function(MuseFacetLease lease) workspace,
  required FutureOr<void> Function(MuseFacetLease lease) assistant,
  required FutureOr<void> Function(MuseFacetLease lease) markdown,
}) =>
    [
      MuseContribution(
          ownerRef: 'muse.experience.workspace',
          slot: 'shell.workspace',
          contributionId: 'default',
          generation: generation,
          priority: 100,
          activate: workspace),
      MuseContribution(
          ownerRef: 'muse.experience.assistant',
          slot: 'shell.assistant',
          contributionId: 'default',
          generation: generation,
          priority: 100,
          activate: assistant),
      MuseContribution(
          ownerRef: 'muse.appflowy.markdown/ui',
          slot: 'workspace.editor',
          contributionId: 'markdown',
          generation: generation,
          priority: 100,
          activate: markdown),
    ];

String _randomRef() {
  final random = Random.secure();
  return List.generate(4,
          (_) => random.nextInt(0x100000000).toRadixString(16).padLeft(8, '0'))
      .join();
}
