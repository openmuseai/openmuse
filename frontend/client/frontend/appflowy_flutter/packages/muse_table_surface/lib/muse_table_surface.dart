import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

const museDatabasePluginId = 'muse.appflowy.database';
const museDatabasePluginVersion = '1.0.0';
const museDatabaseFacetRef = 'facet.flutter.database.1';
const museDatabaseSurfaceKind = 'database.grid';
const _activeCellDigest =
    'sha256:1111111111111111111111111111111111111111111111111111111111111111';
const _rangeDigest =
    'sha256:2222222222222222222222222222222222222222222222222222222222222222';
const _filterDigest =
    'sha256:3333333333333333333333333333333333333333333333333333333333333333';

final class MuseTableSurfaceFacet implements MuseUiFacet {
  MuseTableSurfaceFacet({required this.onExternalReconcile});

  final Future<void> Function(String revision) onExternalReconcile;
  String? _projectedRevision;

  @override
  Future<void> dispose() async {}

  @override
  Future<void> onDomainChange(MuseDomainChangeV1 change) async {
    if (change.origin == MuseMutationOrigin.uiOptimistic ||
        change.domainRevision == _projectedRevision) return;
    await onExternalReconcile(change.domainRevision);
    _projectedRevision = change.domainRevision;
  }
}

final class MuseTableSurfaceBinding {
  MuseTableSurfaceBinding._({required this.runtime, required this.lease, required this.scopeRef})
      : _epochRef = 'epoch.table.${DateTime.now().microsecondsSinceEpoch}';

  final MuseUiSurfaceRuntime runtime;
  final MuseSurfaceLease lease;
  final String scopeRef;
  final String _epochRef;
  var _revision = 0;
  var _closed = false;

  static MuseTableSurfaceBinding open({
    required MuseUiSurfaceRuntime runtime,
    required String databaseId,
    required String windowRef,
    required MuseTableSurfaceFacet facet,
  }) {
    final lease = runtime.openSurface(
      MuseOpenSurface(
        facetInstanceRef: museDatabaseFacetRef,
        surfaceKind: museDatabaseSurfaceKind,
        scopeRef: databaseId,
        resourceRef: databaseId,
        windowRef: windowRef,
      ),
      facet,
    );
    runtime
      ..setActive(lease, active: true)
      ..setFocused(lease, focused: true);
    return MuseTableSurfaceBinding._(runtime: runtime, lease: lease, scopeRef: databaseId);
  }

  Future<void> publishActiveCell({required String rowRef, required String columnRef, Object? value}) => _publish(
        type: 'table.active-cell', digest: _activeCellDigest, lane: MuseContextLane.control,
        ttl: const Duration(seconds: 10), payload: {'rowRef': rowRef, 'columnRef': columnRef, 'value': value},
      );

  Future<void> publishVisibleRange({required List<String> rowRefs, required List<String> columnRefs}) => _publish(
        type: 'table.visible-range', digest: _rangeDigest, lane: MuseContextLane.control,
        ttl: const Duration(seconds: 10), payload: {'rowRefs': rowRefs.take(256).toList(), 'columnRefs': columnRefs.take(64).toList()},
      );

  Future<void> publishFilter({required String filterRef, required String summary}) => _publish(
        type: 'table.filter', digest: _filterDigest, lane: MuseContextLane.state,
        ttl: const Duration(minutes: 2), payload: {'filterRef': filterRef, 'summary': summary.substring(0, summary.length > 512 ? 512 : summary.length)},
      );

  Future<void> close() async { if (_closed) return; _closed = true; await runtime.closeSurface(lease); }

  Future<void> _publish({required String type, required String digest, required MuseContextLane lane, required Duration ttl, required Object payload}) {
    if (_closed) throw StateError('TABLE_SURFACE_CLOSED');
    final now = DateTime.now().millisecondsSinceEpoch;
    return runtime.publishContext(lease, MuseContextContributionV1(
      pluginId: museDatabasePluginId, pluginVersion: museDatabasePluginVersion, facetInstanceRef: museDatabaseFacetRef,
      surfaceInstanceRef: lease.surfaceInstanceRef, surfaceKind: museDatabaseSurfaceKind, scopeRef: scopeRef,
      contextType: type, contextSchemaDigest: digest, contextRevision: (++_revision).toString(), epochRef: _epochRef,
      lane: lane, capturedAt: now, expiresAt: now + ttl.inMilliseconds, payload: payload,
    ));
  }
}

void registerMuseTableSurface(MuseUiSurfaceRuntime runtime) => runtime.registerFacet(const MuseFacetRegistration(
      pluginId: museDatabasePluginId, pluginVersion: museDatabasePluginVersion, facetInstanceRef: museDatabaseFacetRef,
      surfaceKinds: {museDatabaseSurfaceKind},
    ));
