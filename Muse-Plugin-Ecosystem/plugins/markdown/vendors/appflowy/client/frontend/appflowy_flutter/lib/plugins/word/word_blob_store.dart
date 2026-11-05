import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';

/// Filesystem blob store for Word views.
///
/// Layout: `{root}/{viewId}.docx` + `{viewId}.rev`.
class WordBlobStore {
  WordBlobStore({required this.root});

  final Directory root;

  static String revisionOf(Uint8List bytes) => sha256.convert(bytes).toString();

  static void validateDocx(Uint8List bytes) {
    if (bytes.length < 2 || bytes[0] != 0x50 || bytes[1] != 0x4b) {
      throw const FormatException('Word blob must be a zip/docx');
    }
  }

  File _docx(String viewId) => File('${root.path}${Platform.pathSeparator}$viewId.docx');
  File _rev(String viewId) => File('${root.path}${Platform.pathSeparator}$viewId.rev');

  Future<String> put(String viewId, Uint8List bytes) async {
    validateDocx(bytes);
    await root.create(recursive: true);
    final tmp = File('${_docx(viewId).path}.tmp');
    await tmp.writeAsBytes(bytes, flush: true);
    await tmp.rename(_docx(viewId).path);
    final rev = revisionOf(bytes);
    await _rev(viewId).writeAsString(rev, flush: true);
    return rev;
  }

  Future<(Uint8List, String)?> get(String viewId) async {
    final file = _docx(viewId);
    if (!await file.exists()) return null;
    final bytes = Uint8List.fromList(await file.readAsBytes());
    validateDocx(bytes);
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
    final docx = _docx(viewId);
    final rev = _rev(viewId);
    if (await docx.exists()) await docx.delete();
    if (await rev.exists()) await rev.delete();
  }
}
