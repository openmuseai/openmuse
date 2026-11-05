import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_document_contract/muse_document_contract.dart';

void main() {
  test('Dart round-trips shared muse.document@2 fixture', () async {
    final value = (jsonDecode(await File(
                '../../../../../../middlewares/dsh/core/contract-document/fixtures/v2/roundtrip.json')
            .readAsString()) as Map)
        .cast<String, Object?>();
    final snapshotRaw = (value['snapshot']! as Map).cast<String, Object?>();
    final eventRaw = (value['event']! as Map).cast<String, Object?>();
    expect(MuseDocumentSnapshotV2.fromJson(snapshotRaw).toJson(), snapshotRaw);
    expect(MuseDocumentCommitEventV2.fromJson(eventRaw).toJson(), eventRaw);
  });
}
