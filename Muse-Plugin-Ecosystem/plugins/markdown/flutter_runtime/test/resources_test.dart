import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';
import 'package:path/path.dart' as p;

List<int> bundle(Map<String, String> files, {Map<String, String>? catalog}) {
  final archive = Archive();
  for (final entry in files.entries) {
    archive.addFile(ArchiveFile.string(entry.key, entry.value));
  }
  archive.addFile(
    ArchiveFile.string(
      'catalog.json',
      jsonEncode(
        catalog ??
            {
              for (final entry in files.entries)
                entry.key: sha256.convert(utf8.encode(entry.value)).toString(),
            },
      ),
    ),
  );
  return ZipEncoder().encode(archive);
}

void main() {
  late Directory temporary;
  setUp(
    () => temporary = Directory.systemTemp.createTempSync('siyuan-resources-'),
  );
  tearDown(() => temporary.deleteSync(recursive: true));

  test(
    'upgrading immutable resources preserves notebooks and prior resources',
    () {
      final notebook = File(
        p.join(temporary.path, 'userdata', 'siyuan', 'data', 'note.sy'),
      );
      notebook.parent.createSync(recursive: true);
      notebook.writeAsStringSync('encrypted notebook bytes');
      final first = bundle({'app/example': 'v1'});
      final old = installSiYuanResources(
        first,
        temporary.path,
        sha256.convert(first).toString(),
      );
      final second = bundle({'app/example': 'v2'});
      final current = installSiYuanResources(
        second,
        temporary.path,
        sha256.convert(second).toString(),
      );
      expect(File(p.join(old, 'app/example')).readAsStringSync(), 'v1');
      expect(File(p.join(current, 'app/example')).readAsStringSync(), 'v2');
      expect(notebook.readAsStringSync(), 'encrypted notebook bytes');
    },
  );

  test(
    'traversal and forged catalog are rejected without publishing a version',
    () {
      for (final bytes in [
        bundle({'../outside': 'bad'}),
        bundle({'app/example': 'tampered'}, catalog: {'app/example': '0' * 64}),
      ]) {
        final digest = sha256.convert(bytes).toString();
        expect(
          () => installSiYuanResources(bytes, temporary.path, digest),
          throwsFormatException,
        );
        expect(
          Directory(p.join(temporary.path, 'resources', digest)).existsSync(),
          false,
        );
        expect(
          Directory(
            p.join(temporary.path, 'resources', '$digest.staging'),
          ).existsSync(),
          false,
        );
      }
    },
  );
}
