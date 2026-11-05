import 'dart:typed_data';

import 'package:appflowy/plugins/word/word_apply.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_plain_text.dart';

const wordSnapshotProtocol = 'muse.word/snapshot/v1';
const wordWrongLayout = 'WRONG_LAYOUT';
const wordNoCurrentSelection = 'NO_CURRENT_SELECTION';
const wordBlobMissing = 'BLOB_MISSING';

class WordQueryException implements Exception {
  const WordQueryException(this.code);
  final String code;

  @override
  String toString() => code;
}

class WordSnapshotV1 {
  const WordSnapshotV1({
    required this.resourceRef,
    required this.revision,
    required this.text,
    required this.truncated,
    required this.byteLength,
  });

  final String resourceRef;
  final String revision;
  final String text;
  final bool truncated;
  final int byteLength;

  Map<String, Object?> toJson() => {
        'protocol': wordSnapshotProtocol,
        'resourceRef': resourceRef,
        'revision': revision,
        'content': {
          'mediaType': 'text/plain',
          'text': text,
          'truncated': truncated,
          'byteLength': byteLength,
        },
      };
}

/// Host-selected Word page query. Models do not pass viewId.
WordSnapshotV1 queryCurrentWord({
  required int? layout,
  required String? viewId,
  required Uint8List? blob,
}) {
  final id = viewId?.trim() ?? '';
  if (id.isEmpty) {
    throw const WordQueryException(wordNoCurrentSelection);
  }
  if (layout != 9) {
    throw const WordQueryException(wordWrongLayout);
  }
  if (blob == null) {
    throw const WordQueryException(wordBlobMissing);
  }
  final text = plainTextFromDocx(blob);
  return WordSnapshotV1(
    resourceRef: id,
    revision: 'sha256:${WordBlobStore.revisionOf(blob)}',
    text: text,
    truncated: false,
    byteLength: text.length,
  );
}

void refuseMarkdownToolsOnWord(int? layout) {
  if (layout == 9) {
    throw const WordQueryException(wordWrongLayout);
  }
}

void refuseWordToolsOnMarkdown(int? layout) {
  if (layout == 0) {
    throw const WordQueryException(wordWrongLayout);
  }
  if (layout != 9) {
    throw const WordQueryException(wordNoCurrentSelection);
  }
}

Never refuseWordApply() => refuseStaleDocxSave();
