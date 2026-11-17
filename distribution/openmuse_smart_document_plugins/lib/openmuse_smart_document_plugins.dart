library;

import 'src/appflowy_document_workspace.dart';
export 'src/appflowy_document_workspace.dart';
export 'src/row_body_metadata_observer.dart';

import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'src/platform_cores.dart';
import 'package:url_launcher/url_launcher.dart' as links;
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart'
    show
        LocalAssetRepository,
        SnapshotStoreAssetCache,
        DocumentImagePicker,
        DocumentNetworkImageReader,
        createDocumentNetworkImageReader,
        DocumentCore;

/// Native resource contributions. Content and transport remain in their plugins.
/// Cloud capabilities and embedded resolution require separate Host bindings;
/// this default composition exposes existing local resources without a grant.
List<OpenMusePlugin> createNativeSmartDocumentPlugins({
  Directory? storageRoot,
  DocumentCore? documentCore,
  DatasetCore? datasetCore,
  AppFlowyDocumentWorkspace? cloudDocuments,
  DocumentImagePicker? imagePicker,
  ImagePicker? datasetImagePicker,
  AttachmentPicker? datasetAttachmentPicker,
  AttachmentSavePicker? datasetAttachmentSavePicker,
  DocumentNetworkImageReader? networkImageReader,
  DatasetLinkOpener? datasetLinkOpener,
  SmartDocumentStoreFactory? replacementDraftStoreFactory,
  SmartDocumentStoreFactory? documentStoreFactory,
}) {
  late final OpenMuseSmartDocumentPlugin document;
  document = OpenMuseSmartDocumentPlugin(
    core: documentCore,
    storeFactory: documentStoreFactory,
    contentObserverFactory: cloudDocuments?.databases.observeDocumentContent,
    imagePicker: imagePicker,
    networkImageReader:
        networkImageReader ?? createDocumentNetworkImageReader(),
    replacementDraftStoreFactory: replacementDraftStoreFactory,
    providerFactory: cloudDocuments?.providerFor,
    ownerAssetFactory: cloudDocuments == null
        ? null
        : (session, owner) => cloudDocuments.assetsFor(
            session,
            ownerActive: () =>
                owner.canRead &&
                (session.isActive ||
                    document.hasActiveResourceOwner(session.ownershipScope)),
          ),
    realtimeFactory: cloudDocuments?.realtimeFor,
    headerActions: cloudDocuments?.headerActions,
  );
  return [
    document,
    OpenMuseDatasetPlugin(
      core: datasetCore ?? _LazyDatasetCore(),
      linkOpener:
          datasetLinkOpener ??
          (target) async =>
              {'http', 'https', 'mailto', 'tel'}.contains(target.scheme) &&
              await links.launchUrl(target),
      ownerAssetFactory: (session, owner) async {
        if (cloudDocuments != null)
          return cloudDocuments.databases.assetsFor(
            session,
            owner,
            picker: datasetImagePicker,
            attachmentPicker: datasetAttachmentPicker,
            savePicker: datasetAttachmentSavePicker,
          );
        final scope = sha256
            .convert(utf8.encode(session.record.identity.key))
            .toString();
        return DatasetAssetSource(
          picker: datasetImagePicker,
          attachmentPicker: datasetAttachmentPicker,
          savePicker: datasetAttachmentSavePicker,
          scope: scope,
          repository: LocalAssetRepository(
            scope: scope,
            cache: SnapshotStoreAssetCache(
              (key) => createSmartDocumentStore(
                'dataset-assets/$scope',
                key,
                nativeRoot: storageRoot?.path,
              ),
            ),
          ),
        );
      },
      collaborationFactory: cloudDocuments?.databases.providersFor,
      realtimeFactory: cloudDocuments?.databases.realtimeFor,
      viewIdFor: cloudDocuments?.databases.viewIdFor,
      headerActions: cloudDocuments?.databases.headerActions,
      allowRowCreationFor: cloudDocuments?.databases.allowRowCreationFor,
      rowCreationFor: cloudDocuments?.databases.rowCreationFor,
      recordDocumentFactory: cloudDocuments?.databases.recordDocumentFor,
      storeFactory: (session) async {
        final scope =
            '${session.record.identity.key}:${session.opened.fromSnapshot ? session.record.version.digest : 'live'}';
        final key = sha256.convert(utf8.encode(scope));
        return createSmartDocumentStore(
          'dataset',
          '$key',
          nativeRoot: storageRoot?.path,
        );
      },
    ),
  ];
}

/// Installing a plugin contribution must not eagerly open a native content core.
final class _LazyDatasetCore implements DatasetCore {
  DatasetCore? _inner;
  @override
  Map<String, dynamic> call(Map<String, dynamic> request) =>
      (_inner ??= createPlatformDatasetCore()).call(request);
}
