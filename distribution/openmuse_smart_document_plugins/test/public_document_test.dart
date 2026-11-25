import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugins/public_document.dart';

void main() {
  const w = 'e5663016-6e7c-4de9-96ac-1f87d2c3fcbf';
  const d = '575140f1-7220-4cf5-a485-da5348793be1';
  const target = DocumentBrowserTarget(w, d);
  test(
    'document links carry identity only and reject alternate destinations',
    () {
      expect(DocumentBrowserTarget.parse('/documents/$w/$d')?.viewId, d);
      expect(
        DocumentBrowserTarget.parseEditing('/documents/$w/$d/edit')?.viewId,
        d,
      );
      expect(DocumentBrowserTarget.parse('/documents/$w/$d/edit'), isNull);
      expect(
        DocumentBrowserTarget.parseEditing('/documents/$w/$d/edit?host=other'),
        isNull,
      );
      for (final value in [
        '/documents/$w/$d?host=https://other.test',
        '/documents/$w/$d/extra',
        '/documents/$w/../$d',
        'https://other.test/documents/$w/$d',
      ]) {
        expect(DocumentBrowserTarget.parse(value), isNull);
      }
    },
  );
  test(
    'anonymous live reads never acquire a writer or send credentials',
    () async {
      var reads = 0;
      final reader = PublicDocumentReader(
        origin: Uri.parse('http://127.0.0.1:4176'),
        target: target,
        client: MockClient((request) async {
          expect(request.method, 'GET');
          expect(request.headers.containsKey('authorization'), false);
          expect(
            request.url.path,
            '/api/workspace/$w/published-document/$d/live',
          );
          reads++;
          return http.Response(
            jsonEncode({
              'code': 0,
              'data': {
                'profile': 'openmuse.public-document/1',
                'workspaceId': w,
                'objectId': d,
                'name': 'Published',
                'docState': base64Encode([reads]),
              },
            }),
            200,
          );
        }),
      );
      expect(await reader.readState(d), [1]);
      expect(await reader.readState(d), [2]);
      await expectLater(
        reader.exchange(d, Uint8List(0)),
        throwsA(isA<SmartDocumentFailure>()),
      );
      await expectLater(
        reader.initializeState(d, Uint8List(0)),
        throwsA(isA<SmartDocumentFailure>()),
      );
      expect(reads, 2);
      reader.close();
    },
  );
  test(
    'revocation invalidates the visible session and cannot use another object',
    () async {
      var invalidations = 0, requests = 0;
      final reader = PublicDocumentReader(
        origin: Uri.parse('https://docs.example.com'),
        target: target,
        onUnavailable: () => invalidations++,
        client: MockClient((_) async {
          requests++;
          return http.Response('', 404);
        }),
      );
      await expectLater(
        reader.readState(w),
        throwsA(isA<SmartDocumentFailure>()),
      );
      expect(requests, 0);
      await expectLater(
        reader.readState(d),
        throwsA(isA<SmartDocumentFailure>()),
      );
      expect(invalidations, 1);
      reader.close();
    },
  );
  test('a success envelope cannot substitute a different document', () async {
    final reader = PublicDocumentReader(
      origin: Uri.parse('https://docs.example.com'),
      target: target,
      client: MockClient(
        (_) async => http.Response(
          jsonEncode({
            'code': 0,
            'data': {
              'profile': 'openmuse.public-document/1',
              'workspaceId': w,
              'objectId': w,
              'name': 'wrong',
              'docState': 'AQ==',
            },
          }),
          200,
        ),
      ),
    );
    await expectLater(
      reader.readState(d),
      throwsA(isA<SmartDocumentFailure>()),
    );
    reader.close();
  });
}
