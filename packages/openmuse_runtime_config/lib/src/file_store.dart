import 'dart:io';
import 'dart:math';

import 'client.dart';

final class FileConfigStore implements ConfigStore {
  FileConfigStore(this.file);
  final File file;

  @override
  Future<String?> read() async {
    if (!await file.exists()) return null;
    if (await file.length() > 8 * 1024 * 1024) {
      throw const FormatException('Config cache is too large');
    }
    return file.readAsString();
  }

  @override
  Future<void> commit(String record) async {
    await file.parent.create(recursive: true);
    final temporary = File(
      '${file.path}.$pid.${Random.secure().nextInt(1 << 32)}.tmp',
    );
    try {
      await temporary.writeAsString(record, flush: true);
      await temporary.rename(file.path);
    } finally {
      if (await temporary.exists()) await temporary.delete();
    }
  }
}
