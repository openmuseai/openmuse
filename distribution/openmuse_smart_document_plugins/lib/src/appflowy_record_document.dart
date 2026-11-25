part of 'appflowy_document_workspace.dart';

extension _OriginalRecordDocuments on AppFlowyDatasetWorkspace {
  db.DatasetRecordDocumentPort? recordDocumentsFor(
    ResourceEditorSession parent,
    db.DatasetController controller,
  ) {
    final binding = _binding(parent.record);
    if (binding == null || parent.opened.fromSnapshot) return null;
    final page = binding.page as AppFlowyDatabasePage;
    bool active() =>
        parent.isPresentationActive &&
        controller.canRead &&
        !controller.isClosed;
    void check(db.DatasetRecordDocumentReference reference) {
      binding.check();
      if (!active() ||
          reference.rootId != page.databaseId ||
          controller.objectId != page.databaseId) {
        throw const ResourceFailure('DENIED');
      }
      controller.readRow(reference.viewId, reference.rowId);
    }

    db.DatasetRecordDocumentInfo convert(AppFlowyRowDocumentInfo info) =>
        db.DatasetRecordDocumentInfo(
          rootId: info.rootId,
          rowId: info.rowId,
          documentId: info.documentId,
          exists: info.exists,
          canCreate: info.canCreate,
          creationPending: info.creationPending,
        );
    return db.DatasetRecordDocumentPort(
      isActive: active,
      changes: Listenable.merge([
        parent,
        if (parent.lifetime != null) parent.lifetime!,
      ]),
      inspect: (reference) async {
        check(reference);
        final info = await binding.service.inspectRowDocument(
          page.workspaceId,
          page.databaseId,
          reference.rowId,
        );
        check(reference);
        return convert(info);
      },
      createEmpty: (reference) async {
        check(reference);
        if (!parent.allowEditing || !controller.canWrite)
          throw const ResourceFailure('READ_ONLY');
        final info = await binding.service.createEmptyRowDocument(
          page.workspaceId,
          page.databaseId,
          reference.rowId,
        );
        check(reference);
        return convert(info);
      },
      open: (reference, info) async {
        check(reference);
        final current = await binding.service.inspectRowDocument(
          page.workspaceId,
          page.databaseId,
          reference.rowId,
        );
        check(reference);
        if (!current.exists ||
            current.documentId != info.documentId ||
            info.rootId != page.databaseId ||
            info.rowId != reference.rowId) {
          throw const ResourceFailure('RESOURCE_CONTENT_CHANGED');
        }
        final bridge = _workspace._referenceSource;
        if (bridge == null || !bridge.available)
          throw const ResourceFailure('DENIED');
        final content = await bridge
            .ensureSource(page.workspaceId, 'pages')
            .readRequested(
              AppFlowyDocumentPage(
                workspaceId: page.workspaceId,
                viewId: info.documentId,
                name: '记录正文',
              ),
              bridge.center.hub.access('record-document'),
            );
        check(reference);
        final choice = defaultResourceHandler(
          await bridge.center.handlers(content.record),
        );
        check(reference);
        if (choice == null ||
            choice.plugin is! OpenMuseResourceEmbedContributor ||
            choice.plugin is! OpenMuseResourceEmbedValidator) {
          throw const ResourceFailure('HANDLER_UNAVAILABLE');
        }
        final opened = OpenedResource(
          reference: content.record.reference(),
          content: content,
        );
        final target = ResourceEmbedTarget(
          reference: content.record.reference(),
          surface: 'smart-document.document',
          selector: {'objectId': info.documentId},
        );
        await (choice.plugin as OpenMuseResourceEmbedValidator)
            .validateResourceEmbedTarget(opened, target);
        check(reference);
        final child = bridge.center.createEditorSession(
          opened,
          choice,
          readOnly: !parent.allowEditing,
        );
        final detach = parent.addDependentSession(child);
        final scope = child.ownershipScope;
        final metadataEntry = _bodyMetadata.putIfAbsent(
          scope,
          _RowBodyMetadataEntry.new,
        );
        var recordAllowsEditing = false;
        final metadata = _RowBodyMetadataLink(
          database: controller,
          reference: reference,
          documentId: info.documentId,
          changes: Listenable.merge([
            parent,
            child,
            if (parent.lifetime != null) parent.lifetime!,
          ]),
          canObserve: () {
            try {
              check(reference);
              return recordAllowsEditing &&
                  parent.allowEditing &&
                  child.isPresentationActive &&
                  child.allowEditing;
            } on Object {
              return false;
            }
          },
        );
        metadataEntry.links.add(metadata);
        if (metadataEntry.document != null)
          metadata.attach(metadataEntry.document!);
        var prepared = false, finalized = false, released = false;
        Future<void>? releasing;
        Future<bool> prepare() async {
          if (prepared || finalized || released) return true;
          if (!await child.prepareToClose()) return false;
          prepared = true;
          return true;
        }

        Future<bool> finalize() async {
          if (finalized || released) return true;
          if (!await prepare()) return false;
          if (!await child.finalizeToClose()) {
            prepared = false;
            return false;
          }
          finalized = true;
          return true;
        }

        void cancel() {
          if (finalized || released) return;
          child.cancelClosePreparation();
          prepared = false;
        }

        Future<void> release() async {
          if (released) return;
          if (!await finalize())
            throw const ResourceFailure('CHECKPOINT_NOT_READY');
          detach();
          metadataEntry.links.remove(metadata);
          metadata.dispose();
          if (metadataEntry.links.isEmpty &&
              metadataEntry.document == null &&
              identical(_bodyMetadata[scope], metadataEntry))
            _bodyMetadata.remove(scope);
          child.dispose();
          released = true;
        }

        return db.DatasetRecordDocumentPresentation(
          build: (context, allowEditing) {
            final nextPolicy = allowEditing && parent.allowEditing;
            if (recordAllowsEditing != nextPolicy) {
              recordAllowsEditing = nextPolicy;
              metadata.observer?.retry();
            }
            return ResourceEditorSessionScope(
              session: child,
              child: ValueListenableBuilder<Object?>(
                valueListenable: metadata.failure,
                builder: (context, error, _) => Column(
                  children: [
                    if (error != null)
                      Row(
                        children: [
                          const Expanded(child: Text('正文状态尚未同步，请重试')),
                          TextButton(
                            onPressed: recordAllowsEditing
                                ? metadata.observer?.retryCheckpoint
                                : null,
                            child: const Text('重试'),
                          ),
                        ],
                      ),
                    Expanded(
                      child: (choice.plugin as OpenMuseResourceEmbedContributor)
                          .buildResourceEmbed(
                            context,
                            child,
                            target,
                            ResourceEmbedPolicy(
                              allowLive: true,
                              allowEditing: allowEditing && parent.allowEditing,
                            ),
                          ),
                    ),
                  ],
                ),
              ),
            );
          },
          prepareClose: prepare,
          finalizeClose: finalize,
          cancelClosePreparation: cancel,
          release: () =>
              releasing ??= release().whenComplete(() => releasing = null),
        );
      },
    );
  }
}

final class _RowBodyMetadataEntry {
  SmartDocumentController? document;
  final links = <_RowBodyMetadataLink>{};
  void attach(SmartDocumentController value) {
    document = value;
    for (final link in links) {
      link.attach(value);
    }
  }

  void detach(SmartDocumentController value) {
    if (!identical(document, value)) return;
    document = null;
    for (final link in links) {
      link.detach();
    }
  }
}

final class _RowBodyMetadataLink {
  _RowBodyMetadataLink({
    required this.database,
    required this.reference,
    required this.documentId,
    required this.changes,
    required this.canObserve,
  });
  final db.DatasetController database;
  final db.DatasetRecordDocumentReference reference;
  final String documentId;
  final Listenable changes;
  final bool Function() canObserve;
  final failure = ValueNotifier<Object?>(null);
  RowBodyMetadataObserver? observer;
  void attach(SmartDocumentController document) {
    detach();
    observer = RowBodyMetadataObserver(
      document: document,
      database: database,
      viewId: reference.viewId,
      rowId: reference.rowId,
      documentId: documentId,
      canObserve: canObserve,
      authorityChanges: changes,
      onFailure: (value) => failure.value = value,
    );
  }

  void detach() {
    observer?.dispose();
    observer = null;
  }

  void dispose() {
    detach();
    failure.dispose();
  }
}
