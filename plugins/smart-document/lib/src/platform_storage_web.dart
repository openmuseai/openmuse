import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document/browser.dart';

DocumentCore createPlatformDocumentCore() =>
    throw StateError('Browser document core must be loaded asynchronously');

DocumentStore createSmartDocumentStore(
  String namespace,
  String key, {
  String? nativeRoot,
}) => IndexedDbDocumentStore(namespace: namespace, objectId: key);
