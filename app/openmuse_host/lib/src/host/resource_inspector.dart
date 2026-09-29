import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

/// Reads a bounded prefix off the UI thread before choosing an editor.
Future<OpenMuseResource> inspectLocalResource(OpenMuseResource resource) async {
  if (!resource.uri.isScheme('file')) return resource;
  final file = File(resource.uri.toFilePath());
  try {
    final handle = await file.open();
    late Uint8List bytes;
    try {
      bytes = await handle.read(16 * 1024);
    } finally {
      await handle.close();
    }
    return OpenMuseResource(
      uri: resource.uri,
      displayName: resource.displayName,
      mediaType: identifyLocalMediaType(bytes, resource.displayName),
    );
  } on FileSystemException {
    return resource;
  }
}

String identifyLocalMediaType(Uint8List bytes, String name) {
  bool begins(List<int> signature) =>
      bytes.length >= signature.length &&
      Iterable<int>.generate(
        signature.length,
      ).every((index) => bytes[index] == signature[index]);

  if (begins([0x25, 0x50, 0x44, 0x46, 0x2d])) return 'application/pdf';
  if (begins([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])) {
    return 'image/png';
  }
  if (begins([0xff, 0xd8, 0xff])) return 'image/jpeg';
  if (begins(ascii.encode('GIF87a')) || begins(ascii.encode('GIF89a'))) {
    return 'image/gif';
  }
  if (bytes.length >= 12 &&
      begins(ascii.encode('RIFF')) &&
      ascii.decode(bytes.sublist(8, 12), allowInvalid: true) == 'WEBP') {
    return 'image/webp';
  }
  if (begins([0x50, 0x4b, 0x03, 0x04]) ||
      begins([0x1f, 0x8b]) ||
      begins([0x7f, 0x45, 0x4c, 0x46]) ||
      begins([0x4d, 0x5a])) {
    return 'application/octet-stream';
  }

  final utf16 = begins([0xff, 0xfe]) || begins([0xfe, 0xff]);
  if (!utf16 && bytes.any((value) => value == 0 || (value < 9 && value != 0))) {
    return 'application/octet-stream';
  }
  String content;
  try {
    content = utf16 ? '' : utf8.decode(bytes, allowMalformed: false);
  } on FormatException {
    if (bytes.length != 16 * 1024) return 'application/octet-stream';
    // The bounded read may end halfway through a UTF-8 code point.
    String? prefix;
    for (var trim = 1; trim <= 3; trim++) {
      try {
        prefix = utf8.decode(bytes.sublist(0, bytes.length - trim));
        break;
      } on FormatException {
        // A malformed sequence before the boundary is binary data.
      }
    }
    if (prefix == null) return 'application/octet-stream';
    content = prefix;
  }
  final extension = p.extension(name).toLowerCase();
  if (extension == '.svg' ||
      content.trimLeft().startsWith('<svg') ||
      (content.trimLeft().startsWith('<?xml') && content.contains('<svg'))) {
    return 'image/svg+xml';
  }
  if (const {'.md', '.markdown', '.mdown'}.contains(extension)) {
    return 'text/markdown';
  }
  return 'text/plain';
}
