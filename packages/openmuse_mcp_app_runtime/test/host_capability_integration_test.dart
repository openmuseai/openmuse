import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/openmuse_mcp_app_runtime.dart';

void main() {
  test(
    'actual plugin transport composes granted Host tools and revokes an in-flight account',
    () async {
      final config =
          Platform.environment['OPENMUSE_REVIEW_RUNTIME_TEST_CONFIG'];
      final values = jsonDecode(await File(config!).readAsString()) as Map;
      final root = await Directory.systemTemp.createTemp('review-runtime-');
      var scope = 'account-a';
      final secrets = <String, String>{};
      RemoteMcpApp? app;
      try {
        final html = await File(values['uiFile']).readAsString();
        await File('${root.path}/review.html').writeAsString(html);
        await File('${root.path}/apps.json').writeAsString(
          jsonEncode([
            {
              'id': 'review.integration',
              'title': 'Repository Review',
              'endpoint': 'http://127.0.0.1:4191/mcp',
              'uiResourceUri': 'ui://repository-review/main',
              'uiFile': 'review.html',
              'uiSha256': sha256.convert(utf8.encode(html)).toString(),
            },
          ]),
        );
        final token = (await File(values['tokenFile']).readAsString()).trim();
        await File(
          '${root.path}/review.integration.token',
        ).writeAsString(token);
        app = (await RemoteMcpApp.load(
          directory: root,
          scope: () => scope,
          readSecret: (key) async => secrets[key],
          writeSecret: (key, value) async {
            secrets[key] = value;
          },
        )).single;
        app.hostTools = [
          {
            'name': 'host_versions_list',
            'description': 'Explicitly granted Host versions',
            'inputSchema': {
              'type': 'object',
              'properties': {},
              'additionalProperties': false,
            },
          },
        ];
        app.callHostTool = (name, args) async => {
          'structuredContent': {
            'contract': 'openmuse.host-versions/1',
            'resources': [],
          },
          'content': [],
        };
        final tools = await app.request('tools/list', {});
        expect(
          (tools['tools'] as List).any(
            (t) => t['name'] == 'repository_compare',
          ),
          true,
        );
        expect(
          (tools['tools'] as List).any(
            (t) => t['name'] == 'host_versions_list',
          ),
          true,
        );
        final host = await app.request('tools/call', {
          'name': 'host_versions_list',
          'arguments': {},
        });
        expect(
          (host['structuredContent'] as Map)['contract'],
          'openmuse.host-versions/1',
        );
        expect(jsonEncode(host), isNot(contains(token)));
        final source = await app.request('tools/call', {
          'name': 'review_sources',
          'arguments': {},
        });
        expect(source['isError'], isNot(true));
        final waiting = Completer<Map<String, Object?>>();
        app.callHostTool = (name, args) => waiting.future;
        final pending = app.request('tools/call', {
          'name': 'host_versions_list',
          'arguments': {},
        });
        await Future<void>.delayed(const Duration(milliseconds: 50));
        scope = 'account-b';
        waiting.complete({
          'content': [],
          'structuredContent': {'private': 'must not escape'},
        });
        await expectLater(pending, throwsStateError);
      } finally {
        await app?.closeSession();
        await root.delete(recursive: true);
      }
    },
    skip: Platform.environment['OPENMUSE_REVIEW_RUNTIME_TEST_CONFIG'] == null,
  );
}
