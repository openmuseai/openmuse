import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';

/// Filesystem blob store for office views.
///
/// Layout: `{root}/{viewId}.{extension}` + `{viewId}.rev`.
class OfficeBlobStore {
  OfficeBlobStore({
    required this.root,
    required this.extension,
    this.validate,
  });

  final Directory root;
  final String extension;
  final void Function(Uint8List bytes)? validate;

  static String revisionOf(Uint8List bytes) => sha256.convert(bytes).toString();

  File _blob(String viewId) =>
      File('${root.path}${Platform.pathSeparator}$viewId.$extension');
  File _rev(String viewId) =>
      File('${root.path}${Platform.pathSeparator}$viewId.rev');

  Future<String> put(String viewId, Uint8List bytes) async {
    validate?.call(bytes);
    await root.create(recursive: true);
    final tmp = File('${_blob(viewId).path}.tmp');
    await tmp.writeAsBytes(bytes, flush: true);
    await tmp.rename(_blob(viewId).path);
    final rev = revisionOf(bytes);
    await _rev(viewId).writeAsString(rev, flush: true);
    return rev;
  }

  Future<(Uint8List, String)?> get(String viewId) async {
    final file = _blob(viewId);
    if (!await file.exists()) return null;
    final bytes = Uint8List.fromList(await file.readAsBytes());
    validate?.call(bytes);
    String rev;
    final revFile = _rev(viewId);
    if (await revFile.exists()) {
      rev = (await revFile.readAsString()).trim();
    } else {
      rev = revisionOf(bytes);
    }
    return (bytes, rev);
  }

  Future<void> delete(String viewId) async {
    final blob = _blob(viewId);
    final rev = _rev(viewId);
    if (await blob.exists()) await blob.delete();
    if (await rev.exists()) await rev.delete();
  }
}
