import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';
import 'package:file_picker/file_picker.dart';

/// The original client export boundary, shared by application and resource views.
Future<Map<String, Object?>?> saveClientExport(
  String raw, {
  Future<bool> Function(String, Uint8List)? exportWriter,
}) async {
  Object? id;
  Map<String, Object?> result;
  try {
    if (raw.length > 90 * 1024 * 1024)
      throw const FormatException('Export exceeds budget');
    final request = jsonDecode(raw) as Map;
    id = request['id'];
    if (id is! String || id.length > 100 || request['method'] != 'saveExport') {
      throw const FormatException('Invalid export request');
    }
    final name = request['name'] as String;
    if (name.isEmpty ||
        name.length > 200 ||
        name.contains('/') ||
        name.contains('\\') ||
        name == '.' ||
        name == '..') {
      throw const FormatException('Invalid export name');
    }
    final bytes = base64Decode(request['blob'] as String);
    if (bytes.length > 64 * 1024 * 1024)
      throw const FormatException('Export exceeds 64 MiB');
    final writer = exportWriter;
    bool saved;
    if (writer != null) {
      saved = await writer(name, bytes);
    } else {
      final destination = await FilePicker.platform.saveFile(
        fileName: name,
        bytes: bytes,
      );
      saved = destination != null;
      if (destination != null && Platform.isMacOS) {
        final staging = File('$destination.siyuan-saving');
        await staging.writeAsBytes(bytes, flush: true);
        await staging.rename(destination);
      }
    }
    result = {'status': saved ? 'success' : 'canceled', 'name': name};
  } catch (error) {
    result = {'status': 'error', 'message': '$error'};
  }
  return id is String ? {'id': id, 'result': result} : null;
}
