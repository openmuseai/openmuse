import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

const wid = '10000000-0000-4000-8000-000000000001',
    root = '20000000-0000-4000-8000-000000000001',
    row = '30000000-0000-4000-8000-000000000001',
    doc = '40000000-0000-4000-8000-000000000001';
void main() {
  for (final mode in [
    'missing-writer',
    'missing-reader',
    'ready-reader',
    'create',
    'create-already-exists',
    'wrong-workspace',
    'wrong-root',
    'wrong-row',
    'doc-alias',
    'bad-profile',
    'bad-bool',
    'contradiction',
    'not-created',
    'denied',
    'unknown-result',
    'late-account',
  ]) {
    test('original row document discovery/create contract: $mode', () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var posts = 0, gets = 0;
      late AppFlowyCloudWorkspaceService service;
      service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'synthetic-token',
        accountSubject: () => 'alice',
        deviceId: 'row-doc-api-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      final create = {
        'create',
        'create-already-exists',
        'not-created',
        'unknown-result',
      }.contains(mode);
      unawaited(
        server.forEach((request) async {
          expect(
            request.uri.path,
            '/api/workspace/$wid/database/$root/row/$row/document',
          );
          expect(
            request.headers.value(HttpHeaders.authorizationHeader),
            'Bearer synthetic-token',
          );
          if (request.method == 'POST') {
            posts++;
            expect(jsonDecode(await utf8.decoder.bind(request).join()), {});
          } else {
            gets++;
            expect(request.method, 'GET');
          }
          if (mode == 'late-account') service.invalidateAccountOperations();
          if (mode == 'unknown-result') request.response.statusCode = 500;
          final exists = create
              ? mode != 'not-created'
              : mode == 'ready-reader' || mode == 'contradiction';
          request.response.headers.contentType = ContentType.json;
          request.response.write(
            jsonEncode({
              'code': mode == 'denied' ? 1012 : 0,
              'data': {
                'profile': mode == 'bad-profile'
                    ? 'other/1'
                    : 'appflowy.database-row-document/1',
                'workspaceId': mode == 'wrong-workspace' ? doc : wid,
                'rootId': mode == 'wrong-root' ? doc : root,
                'rowId': mode == 'wrong-row' ? doc : row,
                'documentId': mode == 'doc-alias' ? root : doc,
                'exists': mode == 'bad-bool' ? 1 : exists,
                'canCreate':
                    mode == 'missing-writer' || mode == 'contradiction',
              },
            }),
          );
          await request.response.close();
        }),
      );
      final future = create
          ? service.createEmptyRowDocument(wid, root, row)
          : service.inspectRowDocument(wid, root, row);
      if ({
        'missing-writer',
        'missing-reader',
        'ready-reader',
        'create',
        'create-already-exists',
      }.contains(mode)) {
        final info = await future;
        expect(info.rootId, root);
        expect(info.rowId, row);
        expect(info.documentId, doc);
        expect(info.exists, create || mode == 'ready-reader');
        expect(info.canCreate, mode == 'missing-writer');
      } else {
        await expectLater(future, throwsA(isA<CloudServiceException>()));
      }
      // No failure mode automatically retries creation or calls a second route.
      expect(posts, create ? 1 : 0);
      expect(gets, create ? 0 : 1);
    });
  }
}
