import 'dart:convert';
import 'package:archive/archive.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';

void main() {
  final tree = {
    'ID': '20261102123456-abcdefg',
    'Spec': '2',
    'Type': 'NodeDocument',
    'Children': [
      {
        'Type': 'NodeParagraph',
        'Children': [
          {'Type': 'NodeText', 'Data': 'Native block text'},
        ],
      },
    ],
  };
  List<int> archive(Map<String, List<int>> files) {
    final value = Archive();
    for (final e in files.entries) {
      value.addFile(ArchiveFile(e.key, e.value.length, e.value));
    }
    return ZipEncoder().encode(value);
  }

  test('native document archive retains binary assets and attribute views', () {
    final files = {
      'Note/20261102123456-abcdefg.sy': utf8.encode(jsonEncode(tree)),
      'Note/assets/image.png': [1, 2, 3, 4],
      'Note/data/av/20261102123456-bcdefgh.json': utf8.encode(
        '{"id":"native-av"}',
      ),
    };
    final bytes = archive(files), value = SiYuanDocumentArchive.decode(bytes);
    expect(value.bytes, bytes);
    expect(value.documents.length, 1);
    expect(value.conversationText(), contains('Native block text'));
  });
  test('raw sy is parsed and wrapped without flattening its native tree', () {
    final value = SiYuanDocumentArchive.decode(
      utf8.encode(jsonEncode(tree)),
      raw: true,
    );
    expect(value.documents.values.single['Spec'], '2');
    expect(
      ZipDecoder().decodeBytes(value.bytes).files.single.name,
      'OpenMuse/20261102123456-abcdefg.sy',
    );
  });
  test(
    'legacy root-level notes import through a native container without losing assets',
    () {
      final source = utf8.encode(jsonEncode(tree));
      final image = [1, 2, 3, 4];
      final value = SiYuanDocumentArchive.decode(
        archive({
          '20261102123456-abcdefg.sy': source,
          'assets/image.png': image,
        }),
      );
      final normalized = ZipDecoder().decodeBytes(value.bytes);
      expect(normalized.files.map((f) => f.name), [
        'OpenMuse/20261102123456-abcdefg.sy',
        'OpenMuse/assets/image.png',
      ]);
      expect(normalized.files.first.content, source);
      expect(normalized.files.last.content, image);
      // A second open must not accumulate wrappers or rewrite a native archive.
      expect(SiYuanDocumentArchive.decode(value.bytes).bytes, value.bytes);
    },
  );
  test('rejects traversal, absolute and duplicate entries before import', () {
    for (final name in ['../outside.sy', '/outside.sy', 'notes/./outside.sy']) {
      expect(
        () => SiYuanDocumentArchive.decode(
          archive({name: utf8.encode(jsonEncode(tree))}),
        ),
        throwsFormatException,
      );
    }

    final duplicate = archive({
      'one.sy': utf8.encode(jsonEncode(tree)),
      'two.sy': utf8.encode(jsonEncode(tree)),
    });
    final from = utf8.encode('two.sy'), to = utf8.encode('one.sy');
    for (var i = 0; i <= duplicate.length - from.length; i++) {
      if (List.generate(
        from.length,
        (j) => duplicate[i + j] == from[j],
      ).every((v) => v)) {
        for (var j = 0; j < to.length; j++) {
          duplicate[i + j] = to[j];
        }
      }
    }
    expect(
      () => SiYuanDocumentArchive.decode(duplicate),
      throwsFormatException,
    );
  });
  test('rejects arbitrary JSON and non-native archives', () {
    expect(
      () => SiYuanDocumentArchive.decode(utf8.encode('{}'), raw: true),
      throwsFormatException,
    );
    expect(
      () => SiYuanDocumentArchive.decode(
        archive({
          'text.txt': [1, 2],
        }),
      ),
      throwsFormatException,
    );
  });
}
