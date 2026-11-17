import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:file_selector/file_selector.dart' as files;
import 'package:mime/mime.dart';
import 'package:muse_asset_contract/muse_asset_contract.dart';

const attachmentByteLimit = 12 * 1024 * 1024;

/// A filename suggestion never carries an attachment's original local path.
String attachmentFileName(String name) {
  final leaf = name.split(RegExp(r'[/\\]')).last;
  if (leaf.trim().isEmpty ||
      leaf == '.' ||
      leaf == '..' ||
      utf8.encode(leaf).length > 255 ||
      leaf.contains(RegExp(r'[\x00-\x1f\x7f]'))) {
    throw const AssetFailure('ATTACHMENT_NAME_INVALID');
  }
  return leaf;
}

/// A snapshot of the selected bytes. No local path enters portable content.
final class PickedAttachment {
  factory PickedAttachment(Uint8List bytes, String name) {
    if (bytes.isEmpty || bytes.length > attachmentByteLimit) {
      throw const AssetFailure('ASSET_TOO_LARGE');
    }
    final leaf = attachmentFileName(name);
    final frozen = Uint8List.fromList(bytes).asUnmodifiableView();
    return PickedAttachment._(
      frozen,
      leaf,
      lookupMimeType(leaf, headerBytes: frozen) ?? 'application/octet-stream',
    );
  }
  const PickedAttachment._(this.bytes, this.name, this.mediaType);
  final Uint8List bytes;
  final String name, mediaType;
}

typedef AttachmentPicker = Future<PickedAttachment?> Function();
typedef AttachmentSavePicker =
    Future<AttachmentSaveTarget?> Function(String name);

/// Selection and writing are separate so the caller can recheck its source
/// lifetime, field/cell hashes and current read authority after the OS dialog.
abstract interface class AttachmentSaveTarget {
  Future<void> write(Uint8List bytes, {required String mediaType});
}

Future<PickedAttachment?> pickAttachment() async {
  final file = await files.openFile();
  if (file == null) return null;
  return readAttachmentFile(file);
}

/// Read the chosen platform file through the same bounded stream adapter.
Future<PickedAttachment> readAttachmentFile(files.XFile file) async {
  final length = await file.length();
  if (length <= 0 || length > attachmentByteLimit) {
    throw const AssetFailure('ASSET_TOO_LARGE');
  }
  final bytes = BytesBuilder(copy: false);
  try {
    await for (final chunk in file.openRead().timeout(
      const Duration(seconds: 30),
    )) {
      if (bytes.length + chunk.length > attachmentByteLimit) {
        throw const AssetFailure('ASSET_TOO_LARGE');
      }
      bytes.add(chunk);
    }
  } on TimeoutException {
    throw const AssetFailure('ATTACHMENT_READ_TIMEOUT');
  }
  if (bytes.length != length) {
    throw const AssetFailure('ATTACHMENT_SIZE_CHANGED');
  }
  return PickedAttachment(bytes.takeBytes(), file.name);
}

Future<AttachmentSaveTarget?> chooseAttachmentSaveTarget(String name) async {
  final leaf = attachmentFileName(name);
  final destination = await files.getSaveLocation(suggestedName: leaf);
  return destination == null ? null : _FileDestination(destination.path, leaf);
}

final class _FileDestination implements AttachmentSaveTarget {
  _FileDestination(this.path, this.name);
  final String path, name;
  @override
  Future<void> write(Uint8List bytes, {required String mediaType}) async {
    try {
      await files.XFile.fromData(
        bytes,
        name: name,
        mimeType: mediaType,
      ).saveTo(path);
    } on Object {
      throw const AssetFailure('ATTACHMENT_SAVE_FAILED');
    }
  }
}
