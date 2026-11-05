import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

const museWordPluginId = 'muse.appflowy.word';
const museWordPluginVersion = '1.0.0';
const museWordFacetRef = 'facet.flutter.word.1';
const museWordSurfaceKind = 'word.document';
const wordSurfaceDigest =
    'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const wordSelectionDigest =
    'sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';

final class MuseWordSurfaceFacet implements MuseUiFacet {
  MuseWordSurfaceFacet({this.onExternalReconcile});

  final Future<void> Function(String revision)? onExternalReconcile;

  @override
  Future<void> dispose() async {}

  @override
  Future<void> onDomainChange(MuseDomainChangeV1 change) async {
    if (change.origin == MuseMutationOrigin.uiOptimistic) return;
    await onExternalReconcile?.call(change.domainRevision);
  }
}

final class MuseWordSurfaceBinding {
  MuseWordSurfaceBinding._({
    required this.runtime,
    required this.lease,
    required this.scopeRef,
    required this.viewId,
  }) : _epochRef = 'epoch.word.${DateTime.now().microsecondsSinceEpoch}';

  final MuseUiSurfaceRuntime runtime;
  final MuseSurfaceLease lease;
  final String scopeRef;
  final String viewId;
  final String _epochRef;
  var _revision = 0;
  var _closed = false;

  static MuseWordSurfaceBinding open({
    required String viewId,
    required String title,
    MuseUiSurfaceRuntime? runtime,
    MuseWordSurfaceFacet? facet,
    String windowRef = 'window.primary',
  }) {
    final resolved = runtime ?? MuseWordSurfaceRuntime.instance;
    final lease = resolved.openSurface(
      MuseOpenSurface(
        facetInstanceRef: museWordFacetRef,
        surfaceKind: museWordSurfaceKind,
        scopeRef: viewId,
        resourceRef: viewId,
        windowRef: windowRef,
      ),
      facet ?? MuseWordSurfaceFacet(),
    );
    resolved
      ..setActive(lease, active: true)
      ..setFocused(lease, focused: true);
    return MuseWordSurfaceBinding._(
      runtime: resolved,
      lease: lease,
      scopeRef: viewId,
      viewId: viewId,
    );
  }

  Future<void> publishSurface({
    required String title,
    required String mode,
    required bool ffiReady,
  }) =>
      _publish(
        type: 'word.surface',
        digest: wordSurfaceDigest,
        lane: MuseContextLane.control,
        ttl: const Duration(minutes: 5),
        payload: {
          'viewId': viewId,
          'title': title,
          'mode': mode,
          'ffiReady': ffiReady,
        },
      );

  Future<void> publishSelection({
    int? caretCp,
    int? selStart,
    int? selEnd,
    int? pageIndex,
  }) =>
      _publish(
        type: 'word.selection',
        digest: wordSelectionDigest,
        lane: MuseContextLane.control,
        ttl: const Duration(seconds: 10),
        payload: {
          'caretCp': caretCp,
          'selStart': selStart,
          'selEnd': selEnd,
          'pageIndex': pageIndex,
        },
      );

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    await runtime.closeSurface(lease);
  }

  Future<void> _publish({
    required String type,
    required String digest,
    required MuseContextLane lane,
    required Duration ttl,
    required Object payload,
  }) {
    if (_closed) throw StateError('WORD_SURFACE_CLOSED');
    final now = DateTime.now().millisecondsSinceEpoch;
    return runtime.publishContext(
      lease,
      MuseContextContributionV1(
        pluginId: museWordPluginId,
        pluginVersion: museWordPluginVersion,
        facetInstanceRef: museWordFacetRef,
        surfaceInstanceRef: lease.surfaceInstanceRef,
        surfaceKind: museWordSurfaceKind,
        scopeRef: scopeRef,
        contextType: type,
        contextSchemaDigest: digest,
        contextRevision: '${++_revision}',
        epochRef: _epochRef,
        lane: lane,
        capturedAt: now,
        expiresAt: now + ttl.inMilliseconds,
        payload: payload,
      ),
    );
  }
}

final class MuseWordSurfaceRuntime {
  static MuseUiSurfaceRuntime? _instance;
  static MuseUiSurfaceRuntime get instance {
    if (_instance != null) return _instance!;
    final runtime = MuseUiSurfaceRuntime(sink: _FeedSink());
    registerMuseWordSurface(runtime);
    return _instance = runtime;
  }
}

final class _FeedSink implements MuseSurfaceContextSink {
  @override
  Future<void> publish(MuseContextContributionV1 context) async {
    MuseSurfaceContextFeed.instance.publish(context);
  }

  @override
  Future<void> closeSurface(String surfaceInstanceRef, String scopeRef) async {
    MuseSurfaceContextFeed.instance.closeSurface(surfaceInstanceRef);
  }
}

void registerMuseWordSurface(MuseUiSurfaceRuntime runtime) =>
    runtime.registerFacet(
      const MuseFacetRegistration(
        pluginId: museWordPluginId,
        pluginVersion: museWordPluginVersion,
        facetInstanceRef: museWordFacetRef,
        surfaceKinds: {museWordSurfaceKind},
      ),
    );
