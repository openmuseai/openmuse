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
  for (final mode in ['query-until-ready', 'account-ABA']) {
    test(
      'unconfirmed row body creation retains one account-scoped intent: $mode',
      () async {
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        var posts = 0, gets = 0, exists = false;
        var subject = 'alice';
        final service = AppFlowyCloudWorkspaceService(
          baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
          accessToken: () async => 'synthetic-token',
          accountSubject: () => subject,
          deviceId: 'row-pending-test',
          allowHttpForTesting: true,
        );
        addTearDown(() async {
          service.closeClient();
          await server.close(force: true);
        });
        unawaited(
          server.forEach((request) async {
            if (request.method == 'POST') {
              posts++;
              request.response.statusCode = 500;
            } else {
              gets++;
            }
            request.response.headers.contentType = ContentType.json;
            request.response.write(
              jsonEncode({
                'code': 0,
                'data': {
                  'profile': 'appflowy.database-row-document/1',
                  'workspaceId': wid,
                  'rootId': root,
                  'rowId': row,
                  'documentId': doc,
                  'exists': exists,
                  'canCreate': !exists,
                },
              }),
            );
            await request.response.close();
          }),
        );
        await expectLater(
          service.createEmptyRowDocument(wid, root, row),
          throwsA(isA<CloudServiceException>()),
        );
        await expectLater(
          service.createEmptyRowDocument(wid, root, row),
          throwsA(isA<AppFlowyRowDocumentCreationPending>()),
        );
        expect(posts, 1);
        var info = await service.inspectRowDocument(wid, root, row);
        expect(info.creationPending, isTrue);
        expect(info.exists, isFalse);
        if (mode == 'query-until-ready') {
          await expectLater(
            service.createEmptyRowDocument(wid, root, row),
            throwsA(isA<AppFlowyRowDocumentCreationPending>()),
          );
          exists = true;
          info = await service.inspectRowDocument(wid, root, row);
          expect(info.creationPending, isFalse);
          expect(info.exists, isTrue);
          expect(posts, 1);
          expect(gets, 2);
        } else {
          subject = 'bob';
          service.invalidateAccountOperations();
          info = await service.inspectRowDocument(wid, root, row);
          expect(info.creationPending, isFalse);
          subject = 'alice';
          service.invalidateAccountOperations();
          info = await service.inspectRowDocument(wid, root, row);
          expect(info.creationPending, isFalse);
          expect(posts, 1);
        }
      },
    );
  }
}
