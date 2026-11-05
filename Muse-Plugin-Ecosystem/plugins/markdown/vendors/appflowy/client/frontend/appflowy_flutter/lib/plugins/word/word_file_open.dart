import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/startup/startup.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:path/path.dart' as p;

const wordOpenReadTimeout = Duration(seconds: 8);

/// Directory used as NSOpenPanel's starting folder.
///
/// Must not be `~/Documents`: on this host enumerating that folder hangs the
/// process (TCC / iCloud). Do not plant a sample .docx here — Import must
/// always go through the real file picker.
Future<Directory> wordSafeOpenDirectory() async {
  final home = Platform.environment['HOME'] ?? Directory.systemTemp.path;
  final dir = Directory(
    p.join(home, 'Library', 'Application Support', 'OpenMuse', 'word-open'),
  );
  await dir.create(recursive: true);
  return dir;
}

class PickedDocx {
  const PickedDocx({required this.name, required this.bytes});

  final String name;
  final Uint8List bytes;
}

Future<Uint8List?> readDocxFile(String path) async {
  try {
    final bytes = await File(path).readAsBytes().timeout(wordOpenReadTimeout);
    return Uint8List.fromList(bytes);
  } on TimeoutException {
    return null;
  } on FileSystemException {
    return null;
  }
}

/// Pick `.docx` files without starting in Documents or reading via `withData`.
Future<List<PickedDocx>> pickLocalDocxFiles({
  bool allowMultiple = true,
}) async {
  final initial = await wordSafeOpenDirectory();
  final result = await getIt<FilePickerService>().pickFiles(
    dialogTitle: 'Open Word document',
    type: FileType.custom,
    allowedExtensions: const ['docx'],
    allowMultiple: allowMultiple,
    withData: false,
    initialDirectory: initial.path,
  );
  if (result == null || result.files.isEmpty) {
    return const [];
  }
  final picked = <PickedDocx>[];
  for (final file in result.files) {
    Uint8List? bytes = file.bytes;
    final path = file.path;
    if ((bytes == null || bytes.isEmpty) && path != null) {
      bytes = await readDocxFile(path);
    }
    if (bytes == null || bytes.isEmpty) continue;
    picked.add(
      PickedDocx(
        name: file.name.isNotEmpty ? file.name : p.basename(path ?? 'Word'),
        bytes: bytes,
      ),
    );
  }
  return picked;
}
