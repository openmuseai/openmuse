import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'row_body_metadata_observer.dart';
import 'package:http/http.dart' as http;
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart' show kDebugMode;
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart'
    show CloudServiceException, CloudServiceErrorCode;
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center/openmuse_resource_center.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart' as dbp;
part 'appflowy_dataset_workspace.dart';
part 'appflowy_resource_source.dart';
part 'appflowy_record_document.dart';

/// Native product composition of original Cloud Folder/Document0/HTTP/WS2.
/// It does not implement membership, authorization or a second collaboration
/// database. SDK save is explicitly a device checkpoint, never a Cloud CAS/ACK.
final class AppFlowyDocumentWorkspace {
  AppFlowyDocumentWorkspace({
    required this.authentication,
    required this.service,
    this.checkpointRoot,
    this.collaborationHttpClientFactory,
    this.browserBaseUri,
  }) {
    authentication.addListener(_accountChanged);
    _nameSubscription = service.pageNameChanges.listen((observation) {
      unawaited(_observePageName(observation));
    });
  }
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final Uri? browserBaseUri;
  bool get hasBrowserLink =>
      browserBaseUri != null ||
      (kDebugMode &&
          const String.fromEnvironment(
            'OPENMUSE_DOCUMENT_WEB_BASE',
          ).isNotEmpty);

  Uri browserLinkFor(AppFlowyDocumentPage page) {
    const configured = String.fromEnvironment('OPENMUSE_DOCUMENT_WEB_BASE');
    final base = kDebugMode && configured.isNotEmpty
        ? Uri.parse(configured)
        : browserBaseUri;
    if (base == null) throw const ResourceFailure('BROWSER_LINK_UNAVAILABLE');
    if (base.userInfo.isNotEmpty ||
        base.hasQuery ||
        base.hasFragment ||
        (base.scheme != 'https' &&
            !(service.allowHttpForTesting &&
                base.scheme == 'http' &&
                {'127.0.0.1', 'localhost', '::1'}.contains(base.host)))) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
    return base.replace(
      fragment: '/documents/${page.workspaceId}/${page.viewId}',
    );
  }

  Uri? browserLinkForResource(ResourceRecord record) {
    final page = _bound(record)?.page;
    return page is AppFlowyDocumentPage ? browserLinkFor(page) : null;
  }

  late final StreamSubscription<AppFlowyPageNameObservation> _nameSubscription;

  Future<void> _observePageName(AppFlowyPageNameObservation observation) async {
    final bridge = _referenceSource;
    if (bridge == null ||
        !bridge.available ||
        !service.isCurrentPageNameObservation(observation))
      return;
    final page = observation.page;
    // A Document0 view is its resource identity. Multiple original database
    // views share one Root1; their presentation labels need a view selector,
    // and must not be fanned out as one root display name.
    if (page is! AppFlowyDocumentPage) return;
    for (final entry in _bindings.entries.toList()) {
      final binding = entry.value, old = entry.value.page;
      if (!binding.lifetime.active ||
          !identical(_bindings[entry.key], binding) ||
          old is! AppFlowyDocumentPage ||
          old.workspaceId != page.workspaceId ||
          old.viewId != page.viewId)
        continue;
      if (!bridge.available ||
          !service.isCurrentPageNameObservation(observation))
        return;
      binding.page = page;
      try {
        await bridge.center.observeResourceDisplayMetadata(
          entry.key,
          name: page.name,
          location: [page.name],
        );
      } on ResourceFailure {
        // A source/presentation can retire while a verified observation is queued.
      }
    }
  }

  final Directory? checkpointRoot;

  /// Each original scoped transport owns and closes its injected HTTP client.
  /// Authentication, object binding and realtime provider identity stay intact.
  final http.Client Function()? collaborationHttpClientFactory;
  ResourceCenterController? _center;
  late final databases = AppFlowyDatasetWorkspace._(this);
  void _accountChanged() {
    final center = _center;
    if (center != null &&
        (!authentication.snapshot.isAuthenticated ||
            center.hub.scope != authentication.snapshot.identity?.subject)) {
      center.editorLifetime.invalidate();
    }
  }

  void dispose() {
    unawaited(_nameSubscription.cancel());
    authentication.removeListener(_accountChanged);
    _detachSource();
    _center?.editorLifetime.invalidate();
    _center = null;
    _bindings.clear();
  }

  final _bindings = <ResourceIdentity, _Binding>{};
  late final sourceId =
      'appflowy-${sha256.convert(utf8.encode(service.baseUri.origin)).toString().substring(0, 24)}';

  _CloudReferenceSource? _referenceSource;
  void _detachSource() {
    final bridge = _referenceSource;
    if (bridge != null) {
      bridge.center.hub.unregisterReferenceSource(sourceId, bridge);
      for (final provider in bridge.providers.values) {
        bridge.center.hub.unregister(provider.source.key);
      }
    }
    _referenceSource = null;
  }

  void attach(ResourceCenterController center) {
    if (identical(_center, center) && _referenceSource != null) return;
    _detachSource();
    if (_center != null && !identical(_center, center)) {
      _center!.editorLifetime.invalidate();
    }
    _bindings.removeWhere((_, value) => !value.lifetime.active);
    _center = center;
    final bridge = _CloudReferenceSource(this, center);
    _referenceSource = bridge;
    center.hub.registerReferenceSource(sourceId, bridge);
  }

  _Binding? _bound(ResourceRecord record) {
    final binding = _bindings[record.identity];
    if (binding == null) return null;
    binding.check();
    return binding;
  }

  CollaborationProvider? providerFor(ResourceRecord record) {
    final binding = _bound(record);
    return binding?.page is AppFlowyDocumentPage
        ? binding!.provider(client: collaborationHttpClientFactory?.call())
        : null;
  }

  Future<AssetRepository?> assetsFor(
    ResourceEditorSession session, {
    bool Function()? ownerActive,
  }) async {
    final binding = _bound(session.record);
    if (binding == null || binding.page is! AppFlowyDocumentPage) return null;
    binding.check();
    final scope = sha256.convert(utf8.encode(session.record.identity.key));
    return CachedAssetRepository(
      source: CloudAssetRepository(
        origin: service.baseUri,
        workspaceId: binding.page.workspaceId,
        accessToken: binding.token,
      ),
      cache: DocumentStoreAssetCache(
        (key) => createSmartDocumentStore(
          'cloud-document-assets/$scope',
          key,
          nativeRoot: checkpointRoot?.path,
        ),
      ),
      // A closed first presentation must not disable the same live resource
      // owner's cache after its authorized committer has been transferred.
      // Domain/provider access checks still govern every content operation.
      allowOffline: () => ownerActive?.call() ?? session.isActive,
    );
  }

  Future<DocumentRealtimeSession?> realtimeFor(
    ResourceEditorSession session,
    SmartDocumentController controller,
  ) async {
    final binding = _bound(session.record);
    if (binding == null || binding.page is! AppFlowyDocumentPage) return null;
    final wire = await loadRealtimeWireCore();
    return DocumentRealtimeSession(
      controller: controller,
      presenceCore: wire,
      connection: AppFlowyRealtimeConnection(
        origin: service.baseUri,
        workspaceId: binding.page.workspaceId,
        deviceId: service.deviceId,
        clientId:
            (BigInt.from(Random.secure().nextInt(0x100000000)) + BigInt.one)
                .toString(),
        accessToken: binding.token,
        codec: AppFlowyRealtimeCodec(wire),
      ),
    );
  }

  List<Widget> headerActions(
    BuildContext context,
    ResourceEditorSession session,
  ) => _shareActions(context, session, 'smart-document.share');

  List<Widget> _shareActions(
    BuildContext context,
    ResourceEditorSession session,
    String key,
  ) {
    final binding = _bindings[session.record.identity];
    if (binding == null || !session.isActive) return const [];
    return [
      if (hasBrowserLink && binding.page is AppFlowyDocumentPage)
        TextButton.icon(
          key: const ValueKey('smart-document.browser-link'),
          onPressed: () => showDocumentBrowserLinkDialog(
            context,
            service: service,
            workspaceId: binding.page.workspaceId,
            viewId: binding.page.viewId,
            browserLink: browserLinkFor(binding.page as AppFlowyDocumentPage),
          ),
          icon: const Icon(Icons.link, size: 18),
          label: Text(
            Localizations.localeOf(context).languageCode == 'zh'
                ? '浏览链接'
                : 'Browser link',
          ),
        ),
      TextButton.icon(
        key: ValueKey(key),
        onPressed: () => showWorkspaceAccessDialog(
          context,
          authentication: authentication,
          service: service,
          workspaceId: binding.page.workspaceId,
          viewId: binding.page.viewId,
          ancestors: binding.page.ancestors,
          title: binding.page.name,
        ),
        icon: const Icon(Icons.person_add_outlined, size: 18),
        label: Text(
          Localizations.localeOf(context).languageCode == 'zh' ? '分享' : 'Share',
        ),
      ),
    ];
  }

  Future<void> openDocument(AppFlowyDocumentPage page) => _openPage(page);

  Future<void> _openPage(AppFlowyWorkspacePage page) async {
    final bridge = _referenceSource;
    if (bridge == null || !bridge.available) {
      throw const ResourceFailure('DENIED');
    }
    final center = bridge.center;
    final provider = bridge.ensureSource(
      page.workspaceId,
      page is AppFlowyDatabasePage ? 'databases' : 'pages',
    );
    final content = await provider.readRequested(
      page,
      center.hub.access('cloud-page'),
    );
    final choice = defaultResourceHandler(
      await center.handlers(content.record),
    );
    bridge.check();
    if (choice == null || center.presentEditor == null) {
      throw const ResourceFailure('HANDLER_UNAVAILABLE');
    }
    final session = center.createEditorSession(
      OpenedResource(reference: content.record.reference(), content: content),
      choice,
    );
    try {
      unawaited(
        center.presentEditor!(ResourceEditorPresentation(choice, session))
            .whenComplete(session.dispose)
            .catchError((Object error, StackTrace stack) {
              FlutterError.reportError(
                FlutterErrorDetails(
                  exception: error,
                  stack: stack,
                  library: 'appflowy_document_workspace',
                ),
              );
            }),
      );
    } catch (_) {
      session.dispose();
      rethrow;
    }
  }
}

final class _Binding {
  _Binding(
    this.auth,
    this.service,
    this.page,
    this.subject,
    this.lifetime, {
    this.rowCreationSupported = false,
  });
  final OpenMuseAuthenticationController auth;
  final AppFlowyCloudWorkspaceService service;
  AppFlowyWorkspacePage page;
  final bool rowCreationSupported;
  final String subject;
  final ResourceSessionLifetime lifetime;
  void check() {
    if (!lifetime.active ||
        !auth.snapshot.isAuthenticated ||
        auth.snapshot.identity?.subject != subject) {
      throw const db.AppFlowyTransportFailure('FORBIDDEN');
    }
  }

  Future<String> token() async {
    check();
    final token = await auth.accessToken();
    check();
    if (token == null || token.isEmpty)
      throw const db.AppFlowyTransportFailure('FORBIDDEN');
    return token;
  }

  AppFlowyCollaborationProvider provider({http.Client? client}) =>
      AppFlowyCollaborationProvider(
        client: client,
        origin: service.baseUri,
        workspaceId: page.workspaceId,
        accessToken: token,
        deviceId: service.deviceId,
        boundObjectId: page.viewId,
        checkAuthority: check,
      );
}
