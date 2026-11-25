import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

const wid = '10000000-0000-4000-8000-000000000001';
const doc = '20000000-0000-4000-8000-000000000001';
void main() {
  for (final mode in [
    'original-row-document',
    'named-page',
    'wrong-identity',
    'database-layout',
    'space',
    'malformed',
    'denied',
    'unsupported',
    'late-account',
  ]) {
    test('known document original page authorization: $mode', () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      late AppFlowyCloudWorkspaceService service;
      var gets = 0;
      service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'test-token',
        accountSubject: () => 'alice',
        deviceId: 'reference-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((request) async {
          expect(request.method, 'GET');
          expect(request.uri.path, '/api/workspace/$wid/page-view/$doc');
          expect(
            request.headers.value(HttpHeaders.authorizationHeader),
            'Bearer test-token',
          );
          gets++;
          if (mode == 'unsupported') request.response.statusCode = 404;
          if (mode == 'late-account') service.invalidateAccountOperations();
          request.response.headers.contentType = ContentType.json;
          request.response.write(
            jsonEncode({
              'code': mode == 'denied' ? 1012 : 0,
              'data': {
                'view': mode == 'malformed'
                    ? []
                    : {
                        'view_id': mode == 'wrong-identity' ? wid : doc,
                        'layout': mode == 'database-layout' ? 1 : 0,
                        'is_space': mode == 'space',
                        'name': mode == 'named-page' ? '原文档' : '',
                        'is_private': false,
                        'is_published': false,
                      },
              },
            }),
          );
          await request.response.close();
        }),
      );
      final future = service.documentPageForReference(wid, doc);
      if (mode == 'unsupported') {
        expect(await future, isNull);
      } else if ({'original-row-document', 'named-page'}.contains(mode)) {
        final page = await future;
        expect(page!.workspaceId, wid);
        expect(page.viewId, doc);
        expect(page.name, mode == 'named-page' ? '原文档' : '记录正文');
        // Original orphan flags never supply a guessed parent or access level.
        expect(page.ancestors, isEmpty);
      } else {
        await expectLater(future, throwsA(isA<CloudServiceException>()));
      }
      expect(gets, 1);
    });
  }
}
