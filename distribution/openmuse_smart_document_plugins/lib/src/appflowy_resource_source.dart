part of 'appflowy_document_workspace.dart';

bool _cloudUuid(String value) => RegExp(
  r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
).hasMatch(value);

/// Admission is installed by product composition for its configured origin.
/// URI authors cannot register a source, supply credentials or select a host.
final class _CloudReferenceSource implements ResourceReferenceSource {
  _CloudReferenceSource(this.workspace, this.center);
  final AppFlowyDocumentWorkspace workspace;
  final ResourceCenterController center;
  final providers = <String, _CloudResourceProvider>{};
  bool get available =>
      identical(workspace._center, center) &&
      center.editorLifetime.active &&
      workspace.authentication.snapshot.isAuthenticated &&
      workspace.authentication.snapshot.identity?.subject == center.hub.scope;
  void check() {
    if (!center.editorLifetime.active) {
      throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
    }
    if (!available) throw const ResourceFailure('DENIED');
  }

  _CloudResourceProvider ensureSource(String workspaceId, String mount) {
    check();
    if (!_cloudUuid(workspaceId) || !{'pages', 'databases'}.contains(mount)) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
    return providers.putIfAbsent('$workspaceId/$mount', () {
      final provider = _CloudResourceProvider(this, workspaceId, mount);
      center.hub.register(provider);
      return provider;
    });
  }

  @override
  Future<void> prepare(
    ResourceReference reference,
    ResourceAccess access,
  ) async {
    check();
    // These revisions describe device checkpoints, not server history. A pin
    // must never silently become a live read or another account's cached copy.
    if (reference.version != null || reference.retained != null) {
      throw const ResourceFailure('VERSION_UNAVAILABLE');
    }
    final id = reference.identity;
    if (id.scope != center.hub.scope ||
        id.source != workspace.sourceId ||
        !_cloudUuid(id.resource)) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
    access.check(
      id,
      ResourceOperation.read,
      currentGeneration: center.hub.generation,
    );
    ensureSource(id.workspace, id.mount);
  }
}

/// Original Folder/catalog authorizes discovery; original Document0 or
/// Root1/Row4 authorizes content again. No client-side ACL or Row grant fanout.
final class _CloudResourceProvider
    implements ResourceProvider, ResourceDeviceCheckpointProvider {
  _CloudResourceProvider(this.bridge, String workspaceId, String mount)
    : source = ResourceSource(
        scope: bridge.center.hub.scope,
        id: bridge.workspace.sourceId,
        workspace: workspaceId,
        mount: mount,
        label: 'AppFlowy Cloud',
        workspaceLabel: 'Cloud workspace',
        mountLabel: mount == 'pages' ? 'Documents' : 'Databases',
        deviceLabel: 'AppFlowy Cloud',
        local: false,
        canEdit: true,
        exactVersions: false,
      );
  final _CloudReferenceSource bridge;
  @override
  ResourceSource source;
  AppFlowyDocumentWorkspace get workspace => bridge.workspace;
  @override
  bool get available => bridge.available;
  void _check(String resource, ResourceAccess access, ResourceOperation op) {
    bridge.check();
    access.check(
      source.identity(resource),
      op,
      currentGeneration: bridge.center.hub.generation,
    );
    if (resource != 'root' && !_cloudUuid(resource)) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
  }

  String _id(AppFlowyWorkspacePage page) =>
      page is AppFlowyDatabasePage ? page.databaseId : page.viewId;
  bool _accepts(AppFlowyWorkspacePage page) => source.mount == 'pages'
      ? page is AppFlowyDocumentPage
      : page is AppFlowyDatabasePage;
  Future<T> _authorized<T>(Future<T> Function() action) async {
    try {
      return await action();
    } on CloudServiceException catch (error) {
      if (error.code == CloudServiceErrorCode.unauthorized) {
        throw const ResourceFailure('DENIED');
      }
      rethrow;
    } on db.AppFlowyTransportFailure catch (error) {
      if ({'FORBIDDEN', 'UNAUTHORIZED'}.contains(error.code)) {
        throw const ResourceFailure('DENIED');
      }
      rethrow;
    }
  }

  Future<List<AppFlowyWorkspacePage>> _catalog(ResourceAccess access) =>
      _authorized(() async {
        final metadata = (await workspace.service.listWorkspaces())
            .where((w) => w.workspaceRef == source.workspace)
            .singleOrNull;
        bridge.check();
        if (metadata == null) throw const ResourceFailure('DENIED');
        if (source.workspaceLabel != metadata.title) {
          source = ResourceSource.fromJson({
            ...source.toJson(),
            'workspaceLabel': metadata.title,
          });
          bridge.center.hub.changed();
        }
        final pages = await workspace.service.workspacePages(source.workspace);
        bridge.check();
        return pages.where(_accepts).toList();
      });

  Future<AppFlowyWorkspacePage?> _resolvePage(
    String resource,
    ResourceAccess access, {
    ResourceOperation operation = ResourceOperation.read,
  }) => _authorized(() async {
    final listed = (await _catalog(
      access,
    )).where((p) => _id(p) == resource).firstOrNull;
    if (listed != null || source.mount != 'pages') return listed;
    // Discovery stays Folder-based. A known row-document locator is resolved
    // by the original server route, which checks the live Root/Row relation
    // and current Member/Guest scope; no guessed client-side ancestor/grant.
    final page = await workspace.service.documentPageForReference(
      source.workspace,
      resource,
    );
    _check(resource, access, operation);
    return page;
  });

  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    _check(parentRef, access, ResourceOperation.describe);
    if (parentRef != 'root' || limit < 1 || limit > 1000) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
    final offset = cursor == null ? 0 : int.tryParse(cursor);
    if (offset == null || offset < 0) {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
    final seen = <String>{};
    final pages = (await _catalog(
      access,
    )).where((p) => seen.add(_id(p))).toList();
    _check(parentRef, access, ResourceOperation.describe);
    return ResourcePage([
      for (final page in pages.skip(offset).take(limit))
        ResourceDirectoryEntry(
          nodeRef: _id(page),
          name: page.name,
          deferredIdentity: source.identity(_id(page)),
          mediaType: source.mount == 'pages'
              ? smartDocumentMediaType
              : dbp.datasetMediaType,
        ),
    ], nextCursor: offset + limit < pages.length ? '${offset + limit}' : null);
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access, ResourceOperation.describe);
    final page = await _resolvePage(
      resourceRef,
      access,
      operation: ResourceOperation.describe,
    );
    if (page == null) throw const ResourceFailure('DENIED');
    return (await _readPage(
      page,
      access,
      operation: ResourceOperation.describe,
    )).record;
  }

  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access, ResourceOperation.read);
    if (version != null) throw const ResourceFailure('VERSION_UNAVAILABLE');
    final page = await _resolvePage(resourceRef, access);
    if (page == null) throw const ResourceFailure('DENIED');
    return _readPage(page, access);
  }

  Future<ResourceContent> readRequested(
    AppFlowyWorkspacePage requested,
    ResourceAccess access,
  ) async {
    final resource = _id(requested);
    if (requested.workspaceId != source.workspace || !_accepts(requested)) {
      throw const ResourceFailure('RESOURCE_MISMATCH');
    }
    _check(resource, access, ResourceOperation.read);
    final page = await _resolvePage(resource, access);
    if (page == null || page.viewId != requested.viewId) {
      throw const ResourceFailure('DENIED');
    }
    return _readPage(page, access);
  }

  Future<ResourceContent> _readPage(
    AppFlowyWorkspacePage page,
    ResourceAccess access, {
    ResourceOperation operation = ResourceOperation.read,
  }) => _authorized(() => _loadPage(page, access, operation: operation));

  Future<ResourceContent> _loadPage(
    AppFlowyWorkspacePage page,
    ResourceAccess access, {
    ResourceOperation operation = ResourceOperation.read,
  }) async {
    final resource = _id(page);
    _check(resource, access, operation);
    bridge.check();
    final binding = _Binding(
      workspace.authentication,
      workspace.service,
      page,
      source.scope,
      bridge.center.editorLifetime,
      rowCreationSupported:
          page is AppFlowyDatabasePage &&
          await workspace.service.supportsDatabaseRowCreation(
            page.workspaceId,
            page.databaseId,
          ),
    );
    binding.check();
    List<int> bytes;
    if (page is AppFlowyDocumentPage) {
      final provider = binding.provider(
        client: workspace.collaborationHttpClientFactory?.call(),
      );
      try {
        final state = await provider.readState(page.viewId);
        binding.check();
        if (state == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
        final core = await loadDocumentCore();
        binding.check();
        final opened = core.call({
          'action': 'open',
          'objectId': page.viewId,
          'docState': base64Encode(state),
        });
        try {
          bytes = OpenMuseSmartDocumentPlugin.encodeSnapshot(opened);
        } finally {
          core.call({'action': 'close', 'handle': opened['handle']});
        }
      } finally {
        provider.close();
      }
    } else if (page is AppFlowyDatabasePage) {
      final initial = await db.DatasetController.open(
        core: await db.loadDatasetCore(),
        store: _TransientDatasetStore(),
        objectId: page.databaseId,
        providerFactory: workspace.databases._providers(binding),
        editable: false,
        syncInterval: null,
      );
      try {
        binding.check();
        if (!initial.canRead) throw const ResourceFailure('DENIED');
        bytes = dbp.OpenMuseDatasetPlugin.exportBundle(initial.bundle);
      } finally {
        await initial.close();
      }
    } else {
      throw const ResourceFailure('UNSUPPORTED_RESOURCE');
    }
    _check(resource, access, operation);
    if (page is AppFlowyDocumentPage) {
      // The authorized content read may outlive a newer complete Folder read.
      // Reuse its verified label; this never supplies content or a grant.
      final observed = workspace.service.currentPageNameObservation(
        page.workspaceId,
        page.viewId,
      );
      if (observed != null &&
          observed.page is AppFlowyDocumentPage &&
          workspace.service.isCurrentPageNameObservation(observed))
        binding.page = observed.page;
    }
    final record = _record(binding.page, bytes);
    workspace._bindings[record.identity] = binding;
    return ResourceContent(record, bytes);
  }

  ResourceRecord _record(AppFlowyWorkspacePage page, List<int> bytes) {
    final resource = _id(page), digest = resourceDigest(bytes);
    final revision = 'device-checkpoint-$digest';
    return ResourceRecord(
      identity: source.identity(resource),
      version: ResourceVersion(revision: revision, digest: digest),
      descriptor: MuseResourceDescriptorV1(
        resourceRef: resource,
        revision: revision,
        displayName: page.name,
        mediaType: source.mount == 'pages'
            ? smartDocumentMediaType
            : dbp.datasetMediaType,
        format: MuseResourceFormat(
          formatId: source.mount == 'pages' ? 'musedoc' : 'musedataset',
          confidence: MuseFormatConfidence.verified,
        ),
        capabilities: const [
          MuseResourceCapability.describe,
          MuseResourceCapability.snapshot,
        ],
        security: const MuseResourceSecurity(
          classification: MuseSecurityClassification.confidential,
          activeContent: MuseActiveContent.none,
        ),
      ),
      sourceLabel: 'AppFlowy Cloud',
      location: [page.name],
      parentRef: page.ancestors.lastOrNull ?? page.workspaceId,
      metadataRevision: 0,
    );
  }

  @override
  Future<ResourceRecord> checkpoint(
    ResourceRecord base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) async {
    _check(base.identity.resource, access, ResourceOperation.edit);
    if (base.identity.sourceKey != source.key) {
      throw const ResourceFailure('RESOURCE_MISMATCH');
    }
    final binding = workspace._bound(base);
    if (binding == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    _check(base.identity.resource, access, ResourceOperation.edit);
    final key = sha256.convert(utf8.encode(base.identity.key));
    if (binding.page is AppFlowyDocumentPage) {
      final snapshot = OpenMuseSmartDocumentPlugin.decode(bytes);
      if (snapshot['objectId'] != base.identity.resource) {
        throw const ResourceFailure('RESOURCE_MISMATCH');
      }
      await createSmartDocumentStore(
        'cloud-document-checkpoints',
        '$key',
        nativeRoot: workspace.checkpointRoot?.path,
      ).write({
        'objectId': snapshot['objectId'],
        'docState': snapshot['docState'],
        'checkpoint': 'device-only',
      });
    } else {
      final bundle = dbp.OpenMuseDatasetPlugin.decode(bytes);
      if (bundle['objectId'] != base.identity.resource) {
        throw const ResourceFailure('RESOURCE_MISMATCH');
      }
      await createSmartDocumentStore(
        'cloud-dataset-checkpoints',
        '$key',
        nativeRoot: workspace.checkpointRoot?.path,
      ).write({
        'profile': 'muse.dataset.device-checkpoint/1',
        'bundle': bundle,
      });
    }
    _check(base.identity.resource, access, ResourceOperation.edit);
    return _record(binding.page, bytes);
  }

  // Cloud domain writers own creation/rename and CRDT collaboration writes.
  // Resource bytes cannot bypass those ports through an Agent or generic SDK.
  @override
  Future<ResourceRecord> commit(
    String resourceRef,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) async => throw const ResourceFailure('COLLABORATION_REQUIRED');
  @override
  Future<ResourceRecord> create({
    required String parentRef,
    required String name,
    required String mediaType,
    required List<int> bytes,
    required String idempotencyKey,
    required ResourceAccess access,
    ResourceReference? provenance,
  }) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<ResourceRecord> rename(
    String resourceRef,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
}
