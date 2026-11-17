import 'dart:ui' as ui;
import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:mime/mime.dart';
import 'package:image/image.dart' as image_metadata;
import 'package:muse_asset_contract/muse_asset_contract.dart';

final class PickedImage {
  const PickedImage(this.bytes, this.name) : _validation = null;
  PickedImage._validated(this.bytes, this.name, this._validation);
  final Uint8List bytes;
  final String name;
  final ({String mediaType, int width, int height})? _validation;
  ({String mediaType, int width, int height})? get validation => _validation;
}

/// Reuses the original MIME/header/pixel/native decoder checks. A validated
/// selection owns an immutable byte copy, so a picker cannot mutate its proof.
Future<PickedImage> validateImageSelection(PickedImage candidate) async {
  if (candidate._validation != null) return candidate;
  if (candidate.bytes.isEmpty || candidate.bytes.length > 12 * 1024 * 1024)
    throw const AssetFailure('ASSET_TOO_LARGE');
  final picked = PickedImage(
    Uint8List.fromList(candidate.bytes).asUnmodifiableView(),
    candidate.name,
  );
  if (picked.bytes.isEmpty || picked.bytes.length > 12 * 1024 * 1024)
    throw const AssetFailure('ASSET_TOO_LARGE');
  final type = lookupMimeType(picked.name, headerBytes: picked.bytes);
  // Filename alone is not proof: the platform decoder also verifies bytes.
  if (!{'image/png', 'image/jpeg', 'image/gif', 'image/webp'}.contains(type))
    throw const AssetFailure('UNSUPPORTED_IMAGE_TYPE');
  // Existing image package parses headers before a platform decoder can
  // allocate pixels. ImageDescriptor.width is unsupported on Flutter Web.
  final metadata = image_metadata
      .findDecoderForData(picked.bytes)
      ?.startDecode(picked.bytes);
  void checkSize(int w, int h) {
    if (w < 1 || h < 1 || w > 20000 || h > 20000 || w * h > 40000000)
      throw const AssetFailure('IMAGE_PIXEL_LIMIT');
  }

  if (metadata == null) throw const AssetFailure('INVALID_IMAGE_BYTES');
  checkSize(metadata.width, metadata.height);
  var intrinsicWidth = metadata.width, intrinsicHeight = metadata.height;
  final buffer = await ui.ImmutableBuffer.fromUint8List(picked.bytes);
  final codec = await ui.instantiateImageCodecWithSize(
    buffer,
    getTargetSize: (w, h) {
      checkSize(w, h);
      intrinsicWidth = w;
      intrinsicHeight = h;
      return const ui.TargetImageSize(width: 1, height: 1);
    },
  );
  try {
    final frame = await codec.getNextFrame();
    frame.image.dispose();
  } finally {
    codec.dispose();
  }
  return PickedImage._validated(picked.bytes, picked.name, (
    mediaType: type!,
    width: intrinsicWidth,
    height: intrinsicHeight,
  ));
}

typedef ImagePicker = Future<PickedImage?> Function();

Future<PickedImage?> pickImage() async {
  final result = await FilePicker.platform.pickFiles(
    type: FileType.custom,
    allowedExtensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'],
    withData: true,
  );
  if (result == null) return null;
  final file = result.files.single;
  if (file.bytes == null) throw const AssetFailure('IMAGE_BYTES_UNAVAILABLE');
  return PickedImage(file.bytes!, file.name);
}
