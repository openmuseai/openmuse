import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/src/client_export.dart';

void main() {
  test(
    'native export keeps original bytes and cancellation semantics',
    () async {
      final raw = jsonEncode({
        'id': 'export-1',
        'method': 'saveExport',
        'name': 'Note.sy.zip',
        'blob': base64Encode([0, 1, 255]),
      });
      final result = await saveClientExport(
        raw,
        exportWriter: (name, bytes) async {
          expect(name, 'Note.sy.zip');
          expect(bytes, [0, 1, 255]);
          return false;
        },
      );
      expect((result!['result'] as Map)['status'], 'canceled');
    },
  );
  test('invalid export paths never reach the native writer', () async {
    var writes = 0;
    final result = await saveClientExport(
      jsonEncode({
        'id': 'export-2',
        'method': 'saveExport',
        'name': '../Note.zip',
        'blob': '',
      }),
      exportWriter: (_, _) async {
        writes++;
        return true;
      },
    );
    expect((result!['result'] as Map)['status'], 'error');
    expect(writes, 0);
  });
}
