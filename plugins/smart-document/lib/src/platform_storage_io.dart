import 'dart:io';
import 'package:path_provider/path_provider.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

DocumentCore createPlatformDocumentCore() => NativeDocumentCore();

DocumentStore createSmartDocumentStore(
  String namespace,
  String key, {
  String? nativeRoot,
}) => _ApplicationStore(namespace, key, nativeRoot);

final class _ApplicationStore implements DocumentStore {
  _ApplicationStore(this.namespace, this.key, this.nativeRoot);
  final String namespace, key;
  final String? nativeRoot;
  Future<DocumentStore>? _store;
  Future<DocumentStore> _open() => _store ??= () async {
    final root = nativeRoot ?? (await getApplicationSupportDirectory()).path;
    return FileDocumentStore(File('$root/openmuse/$namespace/$key.json'));
  }();
  @override
  Future<Map<String, dynamic>?> read() async => (await _open()).read();
  @override
  Future<void> write(Map<String, dynamic> value) async =>
      (await _open()).write(value);
}
