import 'package:muse_plugin_facets/muse_plugin_facets.dart';

const museWorkspacePlugin = 'muse.appflowy.workspace';
const museWorkspaceFocusDigest =
    'sha256:4c3a6bf1cd8249cc96f6aac63ad78b9be04172634f0313ce3ba52163f66043b7';
const museSurfaceOpenDigest =
    'sha256:c6337b4511a9ef40dbae92c23784cec068a2ff355ac6bb8e95141328c34905f0';
const museSurfaceRevealDigest =
    'sha256:f50404c3696300a2f00c7ac9ba5f413d48987d458669bf397fad0c6b85eabd8b';

/// AppFlowy business lives here, not in HTTP, JS channels, or the Agent loop.
/// Navigation resolves when the destination is ready, not when it is popped.
final class MuseWorkspaceUiFacet {
  MuseWorkspaceUiFacet(
      {required this.workspaceId,
      required this.title,
      required this.instanceRef,
      required this.navigate,
      int Function()? clock})
      : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);
  final String workspaceId;
  final String title;
  final String instanceRef;
  final Future<MusePresentationIntentStatus> Function(
      String viewId, String? blockId) navigate;
  final int Function() _clock;
  int _revision = 0;
  bool _closed = false;
  String get scopeRef => 'workspace.$workspaceId';
  String get surfaceRef => 'surface.$instanceRef.workspace';

  void register(MuseIntentInbox inbox) {
    for (final entry in {
      'surface.open': museSurfaceOpenDigest,
      'surface.revealRange': museSurfaceRevealDigest
    }.entries) {
      inbox.register(
          pluginId: museWorkspacePlugin,
          intentType: entry.key,
          schemaDigest: entry.value,
          handler: _handle);
    }
  }

  MuseContextContributionV1 focus({String? viewId}) {
    final now = _clock();
    return MuseContextContributionV1(
        pluginId: museWorkspacePlugin,
        pluginVersion: '1.0.0',
        facetInstanceRef: 'facet.$instanceRef.workspace',
        surfaceInstanceRef: surfaceRef,
        surfaceKind: 'appflowy.workspace',
        scopeRef: scopeRef,
        contextType: 'workspace.focus',
        contextSchemaDigest: museWorkspaceFocusDigest,
        contextRevision: '${++_revision}',
        epochRef: 'epoch.$instanceRef',
        lane: MuseContextLane.control,
        capturedAt: now,
        expiresAt: now + 120000,
        payload: {
          'workspaceId': workspaceId,
          'title': title,
          if (viewId != null) 'viewId': viewId
        });
  }

  Future<MusePresentationIntentResultV1> _handle(
      MusePresentationIntentV1 intent) async {
    MusePresentationIntentResultV1 result(MusePresentationIntentStatus status,
            [String? reason]) =>
        MusePresentationIntentResultV1(
            intentRef: intent.intentRef,
            status: status,
            reasonCode: reason,
            completedAt: _clock());
    if (_closed)
      return result(MusePresentationIntentStatus.surfaceClosed, 'FACET_CLOSED');
    if (intent.targetSurfaceInstanceRef != null &&
        intent.targetSurfaceInstanceRef != surfaceRef) {
      return result(MusePresentationIntentStatus.notFound, 'SURFACE_NOT_FOUND');
    }
    final payload = intent.payload;
    if (payload is! Map ||
        payload['viewId'] is! String ||
        (payload['viewId'] as String).trim().isEmpty) {
      return result(MusePresentationIntentStatus.rejected, 'VIEW_ID_REQUIRED');
    }
    if (payload['workspaceId'] != null &&
        payload['workspaceId'] != workspaceId) {
      return result(MusePresentationIntentStatus.rejected, 'SCOPE_MISMATCH');
    }
    final block =
        intent.intentType == 'surface.revealRange' ? payload['blockId'] : null;
    if (intent.intentType == 'surface.revealRange' &&
        (block is! String || block.isEmpty)) {
      return result(
          MusePresentationIntentStatus.notSupported, 'RANGE_NOT_SUPPORTED');
    }
    final status =
        await navigate(payload['viewId'] as String, block as String?);
    if (_closed)
      return result(MusePresentationIntentStatus.surfaceClosed, 'FACET_CLOSED');
    return result(status);
  }

  void close() {
    _closed = true;
  }
}
