library;

import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'openmuse_smart_document_plugins.dart';
export 'openmuse_smart_document_plugins.dart';

/// The same domain plugins with browser platform engines and checkpoints.
Future<List<OpenMusePlugin>> createBrowserSmartDocumentPlugins({
  required AppFlowyDocumentWorkspace cloudDocuments,
}) async => createNativeSmartDocumentPlugins(
  cloudDocuments: cloudDocuments,
  documentCore: await loadDocumentCore(),
  datasetCore: await db.loadDatasetCore(),
);
