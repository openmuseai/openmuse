import 'dart:convert';
import 'dart:typed_data';

import 'package:archive/archive.dart';

/// Best-effort plain text from a docx package (P3/P4 until kernel `plainText`).
String plainTextFromDocx(Uint8List bytes) {
  if (bytes.length < 2 || bytes[0] != 0x50 || bytes[1] != 0x4b) {
    return '';
  }
  Archive archive;
  try {
    archive = ZipDecoder().decodeBytes(bytes, verify: false);
  } catch (_) {
    return '';
  }
  ArchiveFile? document;
  for (final file in archive.files) {
    if (file.name == 'word/document.xml') {
      document = file;
      break;
    }
  }
  if (document == null) return '';
  final xml = utf8.decode(document.content as List<int>, allowMalformed: true);
  final texts = <String>[];
  final pattern = RegExp(r'<w:t[^>]*>([^<]*)</w:t>');
  for (final match in pattern.allMatches(xml)) {
    texts.add(_xmlUnescape(match.group(1) ?? ''));
  }
  return texts.join();
}

String _xmlUnescape(String value) => value
    .replaceAll('&lt;', '<')
    .replaceAll('&gt;', '>')
    .replaceAll('&quot;', '"')
    .replaceAll('&apos;', "'")
    .replaceAll('&amp;', '&');
