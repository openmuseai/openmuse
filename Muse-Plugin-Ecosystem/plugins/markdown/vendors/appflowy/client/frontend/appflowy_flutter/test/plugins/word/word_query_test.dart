import 'dart:typed_data';

import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:appflowy/plugins/word/word_apply.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_query.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('P3-T2 query Word page returns plain text and byte revision', () {
    final bytes = buildMinimalDocx(text: 'OpenMuse Word');
    final snapshot = queryCurrentWord(
      layout: 9,
      viewId: 'view-word',
      blob: bytes,
    );
    expect(snapshot.toJson()['protocol'], wordSnapshotProtocol);
    expect(snapshot.revision, 'sha256:${WordBlobStore.revisionOf(bytes)}');
    expect(snapshot.revision, isNot(contains(snapshot.text)));
    expect(snapshot.text, contains('OpenMuse Word'));
    expect(snapshot.toJson()['content'], containsPair('mediaType', 'text/plain'));
  });

  test('P3-T1 snapshot always includes revision', () {
    final snapshot = queryCurrentWord(
      layout: 9,
      viewId: 'view-word',
      blob: buildMinimalDocx(text: 'OpenMuse Word'),
    );
    expect(snapshot.toJson()['revision'], startsWith('sha256:'));
    expect((snapshot.toJson()['revision'] as String).length, 7 + 64);
  });

  test('P3-T7 Word tools on Markdown fail WRONG_LAYOUT', () {
    expect(
      () => queryCurrentWord(layout: 0, viewId: 'doc', blob: Uint8List(0)),
      throwsA(
        isA<WordQueryException>().having((e) => e.code, 'code', wordWrongLayout),
      ),
    );
    expect(
      () => refuseWordToolsOnMarkdown(0),
      throwsA(
        isA<WordQueryException>().having((e) => e.code, 'code', wordWrongLayout),
      ),
    );
  });

  test('P3-T8 Markdown tools on Word fail WRONG_LAYOUT', () {
    expect(
      () => refuseMarkdownToolsOnWord(9),
      throwsA(
        isA<WordQueryException>().having((e) => e.code, 'code', wordWrongLayout),
      ),
    );
  });

  test('P3-T10 apply without kernel is refused', () {
    expect(refuseWordApply, throwsA(isA<WordApplyBlocked>()));
  });

  test('no current selection', () {
    expect(
      () => queryCurrentWord(layout: 9, viewId: '', blob: buildMinimalDocx()),
      throwsA(
        isA<WordQueryException>()
            .having((e) => e.code, 'code', wordNoCurrentSelection),
      ),
    );
  });
}
