import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:test/test.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';

const workspace = '10000000-0000-4000-8000-000000000001';
const root = '20000000-0000-4000-8000-000000000001';
void main() {
  for (final mode in [
    'preflight-timeout',
    'preflight-unavailable',
    'post-timeout',
    'late-token',
  ]) {
    test('creation deadline preserves send boundary: $mode', () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final release = Completer<void>(), token = Completer<String>();
      var posts = 0, tokens = 0;
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async {
          tokens++;
          return mode == 'late-token' && tokens == 2
              ? await token.future
              : 'synthetic';
        },
        accountSubject: () => 'account',
        deviceId: 'creation-deadline',
        allowHttpForTesting: true,
        rowCreationTimeout: const Duration(milliseconds: 150),
      );
      addTearDown(() async {
        if (!release.isCompleted) release.complete();
        if (!token.isCompleted) token.complete('synthetic');
        service.closeClient();
        await server.close(force: true);
      });
      server.listen((request) async {
        try {
          if (request.method == 'GET') {
            if (mode == 'preflight-timeout') await release.future;
            request.response.statusCode = mode == 'preflight-unavailable'
                ? 503
                : 200;
            request.response.headers.contentType = ContentType.json;
            request.response.write(
              jsonEncode({
                'code': 0,
                'data': {
                  'profile': 'appflowy.database-row-creation/1',
                  'rootId': root,
                  'maxStateBytes': 16 * 1024 * 1024,
                },
              }),
            );
          } else {
            await request.drain<void>();
            posts++;
            if (mode == 'post-timeout') await release.future;
            request.response.headers.contentType = ContentType.json;
            request.response.write(
              jsonEncode({
                'code': 0,
                'data': {
                  'profile': 'appflowy.database-row-creation/1',
                  'rootId': root,
                  'rowId': '30000000-0000-4000-8000-000000000001',
                  'beforeRootState': 'AA==',
                  'rootUpdate': 'AA==',
                  'rowState': 'AA==',
                },
              }),
            );
          }
          await request.response.close();
        } on Object {
          /* Deadline deliberately aborts this response. */
        }
      });
      final result = service.createDatabaseRow(workspace, root, {
        'title': 'Preserved input',
      });
      if (mode.startsWith('preflight')) {
        await expectLater(
          result,
          throwsA(
            isA<AppFlowyRowCreationRejected>().having(
              (e) => e.code,
              'code',
              'ROW_CREATION_PREFLIGHT_FAILED',
            ),
          ),
        );
        expect(posts, 0);
      } else {
        await expectLater(result, throwsA(isA<TimeoutException>()));
        expect(posts, mode == 'post-timeout' ? 1 : 0);
      }
      if (!release.isCompleted) release.complete();
      if (!token.isCompleted) token.complete('synthetic');
      await Future<void>.delayed(const Duration(milliseconds: 250));
      expect(
        posts,
        mode == 'post-timeout' ? 1 : 0,
        reason: 'No late POST or mutation replay after the deadline',
      );
    });
  }
}
