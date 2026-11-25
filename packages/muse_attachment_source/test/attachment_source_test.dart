import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:file_selector/file_selector.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_asset_contract/muse_asset_contract.dart';
import 'package:muse_attachment_source/muse_attachment_source.dart';

class _Selected extends XFile {
  _Selected(this.declared, this.chunks) : super('unused.txt');
  final int declared;
  final Stream<List<int>> chunks;
  @override
  String get name => 'selected.txt';
  @override
  Future<int> length() async => declared;
  @override
  Stream<Uint8List> openRead([int? start, int? end]) =>
      chunks.map(Uint8List.fromList);
}

void main() {
  test(
    'selected bytes are immutable and filenames never carry source paths',
    () {
      final bytes = Uint8List.fromList([65, 66, 67]);
      final selected = PickedAttachment(bytes, r'C:\private\附件.txt');
      bytes[0] = 0;
      expect(selected.bytes, [65, 66, 67]);
      expect(() => selected.bytes[0] = 0, throwsUnsupportedError);
      expect(selected.name, '附件.txt');
      expect(selected.mediaType, 'text/plain');
    },
  );
  test('empty, oversized and unsafe names fail before upload', () {
    for (final bytes in [Uint8List(0), Uint8List(attachmentByteLimit + 1)]) {
      expect(
        () => PickedAttachment(bytes, 'file.txt'),
        throwsA(isA<AssetFailure>()),
      );
    }
    for (final name in ['', '.', '..', 'bad\u0000name', '文' * 86]) {
      expect(
        () => PickedAttachment(Uint8List.fromList([1]), name),
        throwsA(
          isA<AssetFailure>().having(
            (e) => e.code,
            'code',
            'ATTACHMENT_NAME_INVALID',
          ),
        ),
      );
    }
  });
  test('declared size rejects before the selected stream is read', () async {
    var listened = false;
    final stream = Stream<List<int>>.multi((controller) {
      listened = true;
      controller.close();
    });
    await expectLater(
      readAttachmentFile(_Selected(attachmentByteLimit + 1, stream)),
      throwsA(isA<AssetFailure>()),
    );
    expect(listened, false);
  });
  test(
    'actual stream budget is enforced even if selection metadata is smaller',
    () async {
      var cancelled = false;
      final controller = StreamController<List<int>>(
        onCancel: () => cancelled = true,
      );
      final reading = readAttachmentFile(_Selected(1, controller.stream));
      controller.add(Uint8List(attachmentByteLimit + 1));
      await expectLater(
        reading,
        throwsA(
          isA<AssetFailure>().having((e) => e.code, 'code', 'ASSET_TOO_LARGE'),
        ),
      );
      expect(cancelled, true);
      await controller.close();
    },
  );
  test(
    'a changed selected file cannot silently upload a different size',
    () async {
      await expectLater(
        readAttachmentFile(_Selected(3, Stream.value([1, 2]))),
        throwsA(
          isA<AssetFailure>().having(
            (e) => e.code,
            'code',
            'ATTACHMENT_SIZE_CHANGED',
          ),
        ),
      );
    },
  );
  test(
    'original XFile disk reader retains exact selected bytes and neutral MIME',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'muse-attachment-source-',
      );
      try {
        final file = File('${root.path}/payload.unknown-extension');
        final bytes = Uint8List.fromList([1, 2, 3, 4, 5, 6]);
        await file.writeAsBytes(bytes);
        final selected = await readAttachmentFile(XFile(file.path));
        expect(selected.bytes, bytes);
        expect(selected.name, 'payload.unknown-extension');
        expect(selected.mediaType, 'application/octet-stream');
      } finally {
        await root.delete(recursive: true);
      }
    },
  );
}
