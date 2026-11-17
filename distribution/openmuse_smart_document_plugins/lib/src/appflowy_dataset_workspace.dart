part of 'appflowy_document_workspace.dart';

/// Independent Root1/Row4 composition over the same authenticated workspace.
/// Folder view IDs select/share; canonical database IDs own the resource session.
final class AppFlowyDatasetWorkspace {
  AppFlowyDatasetWorkspace._(this._workspace);
  final AppFlowyDocumentWorkspace _workspace;
  final _bodyMetadata = <ResourceSessionScope, _RowBodyMetadataEntry>{};

  VoidCallback? observeDocumentContent(
    ResourceEditorSession session,
    SmartDocumentController document,
  ) {
    if (session.opened.fromSnapshot) return null;
    final scope = session.ownershipScope;
    final entry = _bodyMetadata.putIfAbsent(scope, _RowBodyMetadataEntry.new);
    entry.attach(document);
    return () {
      entry.detach(document);
      if (entry.links.isEmpty && identical(_bodyMetadata[scope], entry))
        _bodyMetadata.remove(scope);
    };
  }

  _Binding? _binding(ResourceRecord record) {
    final binding = _workspace._bound(record);
    return binding?.page is AppFlowyDatabasePage ? binding : null;
  }

  db.DatasetProviderFactory? providersFor(ResourceRecord record) {
    final binding = _binding(record);
    return binding == null ? null : _providers(binding);
  }

  db.DatasetProviderFactory _providers(_Binding binding) {
    final page = binding.page as AppFlowyDatabasePage;
    return (type) {
      if (!{1, 4}.contains(type))
        throw const ResourceFailure('INVALID_COLLAB_TYPE');
      return db.AppFlowyObjectTransport(
        client: _workspace.collaborationHttpClientFactory?.call(),
        origin: binding.service.baseUri,
        workspaceId: page.workspaceId,
        accessToken: binding.token,
        deviceId: binding.service.deviceId,
        binding: db.datasetBinding(type),
        collabType: type,
        boundObjectId: type == 1 ? page.databaseId : null,
        checkAuthority: binding.check,
      );
    };
  }

  String? viewIdFor(ResourceRecord record) => _binding(record)?.page.viewId;
  bool allowRowCreationFor(ResourceRecord record) {
    final binding = _binding(record);
    return binding == null || binding.rowCreationSupported;
  }

  db.DatasetRowCreationPort? rowCreationFor(ResourceRecord record) {
    final binding = _binding(record);
    if (binding == null) return null;
    final page = binding.page as AppFlowyDatabasePage;
    return (cells) async {
      binding.check();
      // A missing capability is a definitive refusal. Never fall back to raw
      // collab creation using a role captured when the editor first opened.
      if (!binding.rowCreationSupported) {
        throw const db.DatasetFailure('ROW_CREATION_UNSUPPORTED');
      }
      AppFlowyDatabaseRowCreation created;
      try {
        created = await binding.service.createDatabaseRow(
          page.workspaceId,
          page.databaseId,
          cells,
        );
      } on AppFlowyRowCreationRejected catch (error) {
        throw db.DatasetFailure(error.code);
      }
      binding.check();
      return db.DatasetCreatedRow(
        rootId: created.rootId,
        rowId: created.rowId,
        beforeRootState: created.beforeRootState,
        rootUpdate: created.rootUpdate,
        rowState: created.rowState,
      );
    };
  }

  Future<db.DatasetAssetSource?> assetsFor(
    ResourceEditorSession session,
    db.DatasetController controller, {
    db.ImagePicker? picker,
    db.AttachmentPicker? attachmentPicker,
    db.AttachmentSavePicker? savePicker,
  }) async {
    final binding = _binding(session.record);
    if (binding == null) return null;
    binding.check();
    final scope = sha256.convert(utf8.encode(session.record.identity.key));
    return db.DatasetAssetSource(
      picker: picker,
      attachmentPicker: attachmentPicker,
      savePicker: savePicker,
      scope: binding.page.workspaceId,
      repository: CachedAssetRepository(
        source: CloudAssetRepository(
          origin: binding.service.baseUri,
          workspaceId: binding.page.workspaceId,
          accessToken: binding.token,
        ),
        cache: SnapshotStoreAssetCache(
          (key) => createSmartDocumentStore(
            'cloud-dataset-assets/$scope',
            key,
            nativeRoot: _workspace.checkpointRoot?.path,
          ),
        ),
        allowOffline: () => controller.canRead && !controller.isClosed,
      ),
    );
  }

  List<Widget> headerActions(
    BuildContext context,
    ResourceEditorSession session,
  ) => _workspace._shareActions(context, session, 'dataset.share');
  Future<db.DatasetRealtimeSession?> realtimeFor(
    ResourceEditorSession session,
    db.DatasetController controller,
  ) async {
    final binding = _binding(session.record);
    if (binding == null) return null;
    final wire = await db.loadRealtimeWireCore();
    return db.DatasetRealtimeSession(
      controller: controller,
      presenceCore: wire,
      connection: db.AppFlowyRealtimeConnection(
        origin: binding.service.baseUri,
        workspaceId: binding.page.workspaceId,
        deviceId: binding.service.deviceId,
        clientId:
            (BigInt.from(Random.secure().nextInt(0x100000000)) + BigInt.one)
                .toString(),
        accessToken: binding.token,
        codec: db.AppFlowyRealtimeCodec(wire),
      ),
    );
  }

  db.DatasetRecordDocumentPort? recordDocumentFor(
    ResourceEditorSession session,
    db.DatasetController controller,
  ) => recordDocumentsFor(session, controller);

  Future<void> openDatabase(AppFlowyDatabasePage page) =>
      _workspace._openPage(page);
}

final class _TransientDatasetStore implements db.SnapshotStore {
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> data) async {
    value = (jsonDecode(jsonEncode(data)) as Map).cast<String, dynamic>();
  }
}
