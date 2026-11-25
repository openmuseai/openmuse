import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center/openmuse_resource_center.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_smart_document_plugins/browser.dart';

/// Browser platform composition; the same plugins own content, authority and
/// editor lifetimes as Mobile/Desktop. No Desktop pairing grant is required.
final class BrowserCloudDocuments {
  BrowserCloudDocuments._(
    this.cloud,
    this.documents,
    this.registry,
    this.center,
  );
  final OpenMuseCloudWorkspacePlugin cloud;
  final AppFlowyDocumentWorkspace documents;
  final OpenMusePluginRegistry registry;
  final ResourceCenterController center;
  bool _closed = false;

  static Future<BrowserCloudDocuments> open({
    required OpenMuseAuthenticationController authentication,
    required Uri origin,
    required Future<void> Function(ResourceEditorPresentation) presentEditor,
  }) async {
    final subject = authentication.snapshot.identity?.subject;
    if (!authentication.snapshot.isAuthenticated || subject == null) {
      throw const ResourceFailure('DENIED');
    }
    late final AppFlowyDocumentWorkspace documents;
    final cloud = OpenMuseCloudWorkspacePlugin(
      authentication: authentication,
      cloudOrigin: origin,
      deviceId: 'web-smart-document-$subject',
      allowInsecureLoopback: origin.scheme == 'http',
      onOpenDocument: (page) => documents.openDocument(page),
      onOpenDatabase: (page) => documents.databases.openDatabase(page),
    );
    documents = AppFlowyDocumentWorkspace(
      authentication: authentication,
      service: cloud.service,
      browserBaseUri: Uri.parse(Uri.base.origin).resolve(Uri.base.path),
    );
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (_, _) async =>
            throw const ResourceFailure('DENIED'),
      ),
    );
    final center = ResourceCenterController(
      hub: ResourceCenterHub(scope: subject),
      registry: registry,
    )..presentEditor = presentEditor;
    final value = BrowserCloudDocuments._(cloud, documents, registry, center);
    try {
      registry.install(cloud);
      for (final plugin in await createBrowserSmartDocumentPlugins(
        cloudDocuments: documents,
      )) {
        registry.install(plugin);
      }
      if (!authentication.snapshot.isAuthenticated ||
          authentication.snapshot.identity?.subject != subject) {
        throw const ResourceFailure('DENIED');
      }
      for (final plugin in registry.descriptors.toList()) {
        await registry.activate(plugin.id);
      }
      documents.attach(center);
      return value;
    } catch (_) {
      await value.close();
      rethrow;
    }
  }

  Future<void> openDocument(String workspaceId, String viewId) async {
    final page = await cloud.service.documentPageForReference(
      workspaceId,
      viewId,
    );
    if (page == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    await documents.openDocument(page);
  }

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    // Invalidate authority before any asynchronous engine or checkpoint close.
    documents.dispose();
    for (final plugin in registry.descriptors.toList().reversed) {
      await registry.deactivate(plugin.id);
    }
    center.dispose();
    await center.hub.dispose();
    registry.dispose();
    cloud.service.closeClient();
  }
}
