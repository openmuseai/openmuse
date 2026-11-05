import 'package:appflowy/plugins/dsh_agent/dsh_markdown_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_word_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_workspace_catalog.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';

/// Builds the same workspace/markdown Facet envelopes Web E4 posts over postMessage.
class DshHostPluginPlane {
  DshHostPluginPlane({int Function()? clock})
      : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);

  final int Function() _clock;
  var _catalogRevision = 0;
  var _snapshotRevision = 0;

  MuseContextContributionV1? catalog({
    required String workspaceId,
    required List<DshCatalogViewInput> views,
    String? instanceRef,
  }) {
    final payload = buildWorkspaceCatalog(
      workspaceId: workspaceId,
      views: views,
    );
    if (payload == null) {
      return null;
    }
    final now = _clock();
    final surface = 'surface.appflowy.workspace.$workspaceId';
    return MuseContextContributionV1(
      pluginId: 'muse.appflowy.workspace',
      pluginVersion: '1.0.0',
      facetInstanceRef: 'facet.${instanceRef ?? workspaceId}.workspace',
      surfaceInstanceRef: surface,
      surfaceKind: 'appflowy.workspace',
      scopeRef: 'workspace.$workspaceId',
      contextType: workspaceCatalogContextType,
      contextSchemaDigest: workspaceCatalogDigest,
      contextRevision: '${++_catalogRevision}',
      epochRef: 'epoch.$workspaceId',
      lane: MuseContextLane.control,
      capturedAt: now,
      expiresAt: now + 120000,
      payload: payload.toJson(),
    );
  }

  MuseContextContributionV1? snapshot({
    required String workspaceId,
    required String viewId,
    required String text,
    String? instanceRef,
  }) {
    final payload = buildMarkdownSnapshot(
      viewId: viewId,
      text: text,
      workspaceId: workspaceId,
    );
    if (payload == null) {
      return null;
    }
    final now = _clock();
    return MuseContextContributionV1(
      pluginId: 'muse.appflowy.markdown',
      pluginVersion: '2.0.0',
      facetInstanceRef: 'facet.${instanceRef ?? viewId}.markdown',
      surfaceInstanceRef: 'surface.appflowy.doc.$viewId',
      surfaceKind: 'appflowy.markdown',
      scopeRef: viewId,
      contextType: markdownSnapshotContextType,
      contextSchemaDigest: markdownSnapshotDigest,
      contextRevision: '${++_snapshotRevision}',
      epochRef: 'epoch.$workspaceId.$viewId',
      lane: MuseContextLane.state,
      capturedAt: now,
      expiresAt: now + 90000,
      payload: payload.toJson(),
    );
  }

  MuseContextContributionV1? wordSnapshot({
    required String workspaceId,
    required String viewId,
    required String text,
    String? instanceRef,
  }) {
    final payload = buildWordSnapshot(
      viewId: viewId,
      text: text,
      workspaceId: workspaceId,
    );
    if (payload == null) {
      return null;
    }
    final now = _clock();
    return MuseContextContributionV1(
      pluginId: 'muse.appflowy.word',
      pluginVersion: '1.0.0',
      facetInstanceRef: 'facet.${instanceRef ?? viewId}.word',
      surfaceInstanceRef: 'surface.appflowy.word.$viewId',
      surfaceKind: 'word.document',
      scopeRef: viewId,
      contextType: wordSnapshotContextType,
      contextSchemaDigest: wordSnapshotDigest,
      contextRevision: '${++_snapshotRevision}',
      epochRef: 'epoch.$workspaceId.$viewId',
      lane: MuseContextLane.state,
      capturedAt: now,
      expiresAt: now + 90000,
      payload: payload.toJson(),
    );
  }
}
