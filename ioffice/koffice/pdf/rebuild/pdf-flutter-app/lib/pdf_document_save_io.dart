import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

bool get supportsPlatformDocumentSave => Platform.isAndroid || Platform.isIOS;

const _channel = MethodChannel('cn.wps.pdf_rebuild_flutter/document_io');

/// GenOffice-style generated output for desktop, under the user's Documents.
/// Mobile and Web must use their platform document authorization flow.
Future<String?> saveGeneratedPdfDesktop(
  String name,
  List<int> bytes, {
  String? configuredDirectory,
}) async {
  if (!(Platform.isMacOS || Platform.isWindows || Platform.isLinux)) {
    return null;
  }
  final directory = await resolveGeneratedPdfDirectory(
    configuredDirectory: configuredDirectory,
    fallbackDirectory: () async =>
        '${(await getApplicationDocumentsDirectory()).path}/OpenMuse',
  );
  return writeGeneratedPdfToDirectory(directory, name, bytes);
}

/// Resolves a Host preference at the moment of export, so disconnected drives
/// and revoked folder access fall back without changing the saved preference.
Future<String> resolveGeneratedPdfDirectory({
  String? configuredDirectory,
  required Future<String> Function() fallbackDirectory,
}) async {
  if (configuredDirectory != null &&
      p.isAbsolute(configuredDirectory) &&
      await _isWritableDirectory(configuredDirectory)) {
    return configuredDirectory;
  }
  final fallback = await fallbackDirectory();
  if (await _isWritableDirectory(fallback)) return fallback;
  throw FileSystemException('默认 PDF 保存目录不可写', fallback);
}

Future<bool> _isWritableDirectory(String path) async {
  try {
    final directory = await Directory(path).create(recursive: true);
    final probe = File(
      p.join(
        directory.path,
        '.openmuse-pdf-write-${DateTime.now().microsecondsSinceEpoch}-$pid',
      ),
    );
    await probe.create(exclusive: true);
    await probe.delete();
    return true;
  } on FileSystemException {
    return false;
  }
}

Future<String> writeGeneratedPdfToDirectory(
  String path,
  String name,
  List<int> bytes,
) async {
  final directory = await Directory(path).create(recursive: true);
  final safeName = _safePdfName(name);
  final stem = safeName.substring(0, safeName.length - 4);
  for (var index = 1; index < 10000; index++) {
    final candidate = index == 1 ? safeName : '$stem-$index.pdf';
    final file = File('${directory.path}/$candidate');
    try {
      await file.create(exclusive: true);
    } on FileSystemException {
      if (await file.exists()) continue;
      rethrow;
    }
    try {
      await file.writeAsBytes(bytes, flush: true);
      return file.path;
    } catch (_) {
      if (await file.exists()) await file.delete();
      rethrow;
    }
  }
  throw StateError('Too many generated PDF files with the same name');
}

/// Returns null on platforms without a directory picker, or the number saved.
/// A returned zero means the user cancelled the one directory choice.
Future<int?> savePdfBatchWithPlatformPicker(
  Map<String, List<int>> files,
) async {
  if (files.isEmpty) return 0;
  if (Platform.isAndroid) {
    final temporary = await Directory.systemTemp.createTemp(
      'pdf-rebuild-batch-',
    );
    try {
      final descriptors = <Map<String, String>>[];
      for (final entry in files.entries) {
        final name = _safePdfName(entry.key);
        final source = File('${temporary.path}/$name');
        await source.writeAsBytes(entry.value, flush: true);
        descriptors.add({'name': name, 'sourcePath': source.path});
      }
      try {
        return await _channel.invokeMethod<int>('savePdfBatch', {
              'files': descriptors,
            }) ??
            0;
      } on MissingPluginException {
        // The standalone PDF sample may run without the Host batch adapter.
        return null;
      }
    } finally {
      await temporary.delete(recursive: true);
    }
  }
  if (Platform.isMacOS || Platform.isWindows || Platform.isLinux) {
    final path = await getDirectoryPath(confirmButtonText: '导出');
    if (path == null) return 0;
    return writePdfBatchToDirectory(path, files);
  }
  return null;
}

/// Stages all outputs before publishing any of them; rejects existing names.
Future<int> writePdfBatchToDirectory(
  String path,
  Map<String, List<int>> files,
) async {
  final directory = Directory(path);
  final names = files.keys.map(_safePdfName).toList();
  for (final name in names) {
    if (await File('${directory.path}/$name').exists()) {
      throw FileSystemException('目标目录已有同名文件', name);
    }
  }
  final staging = await directory.createTemp('.openmuse-pdf-export-');
  final committed = <File>[];
  try {
    for (var index = 0; index < names.length; index++) {
      await File(
        '${staging.path}/${names[index]}',
      ).writeAsBytes(files.values.elementAt(index), flush: true);
    }
    for (final name in names) {
      final target = File('${directory.path}/$name');
      await File('${staging.path}/$name').rename(target.path);
      committed.add(target);
    }
    return names.length;
  } catch (_) {
    for (final file in committed) {
      if (await file.exists()) await file.delete();
    }
    rethrow;
  } finally {
    if (await staging.exists()) await staging.delete(recursive: true);
  }
}

String _safePdfName(String name) {
  final safe = name.split(RegExp(r'[/\\]')).last;
  if (safe.isEmpty ||
      safe == '.' ||
      safe == '..' ||
      !safe.toLowerCase().endsWith('.pdf')) {
    throw FormatException('Invalid PDF export name', name);
  }
  return safe;
}

Future<bool> savePdfWithPlatformPicker(
  String name,
  Future<void> Function(String path) writeCopy,
) async {
  if (!supportsPlatformDocumentSave) {
    throw UnsupportedError('Platform document save is unavailable');
  }
  final temporary = await Directory.systemTemp.createTemp('pdf-rebuild-save-');
  try {
    final baseName = name.split(RegExp(r'[/\\]')).last;
    final safeName = baseName.isEmpty ? 'document.pdf' : baseName;
    final source = File('${temporary.path}/$safeName');
    await writeCopy(source.path);
    if (!await source.exists()) {
      throw StateError('编辑内核未生成 PDF 副本');
    }
    return await _channel.invokeMethod<bool>('savePdf', {
          'name': name,
          'sourcePath': source.path,
        }) ??
        false;
  } finally {
    await temporary.delete(recursive: true);
  }
}
