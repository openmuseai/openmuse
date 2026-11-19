import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/openmuse_mcp_app_runtime.dart';

void main() {
  late Directory directory;
  late Map<String, dynamic> profile;
  final secrets = <String, String>{};
  var scope = 'account-a';
  setUp(() async {
    directory = await Directory.systemTemp.createTemp('remote-app-test-');
    secrets.clear();
    scope = 'account-a';
    const html = '<!doctype html><html>reviewed</html>';
    profile = {
      'id': 'example.app',
      'title': 'Example',
      'version': '0.1.0',
      'endpoint': 'http://127.0.0.1:1/mcp',
      'uiResourceUri': 'ui://example/main',
      'uiFile': 'ui.html',
      'uiSha256': sha256.convert(utf8.encode(html)).toString(),
    };
    await File('${directory.path}/ui.html').writeAsString(html);
  });
  tearDown(() => directory.delete(recursive: true));
  Future<List<RemoteMcpApp>> load() async {
    await File(
      '${directory.path}/apps.json',
    ).writeAsString(jsonEncode([profile]));
    return RemoteMcpApp.load(
      directory: directory,
      scope: () => scope,
      readSecret: (key) async => secrets[key],
      writeSecret: (key, value) async {
        secrets[key] = value;
      },
    );
  }

  test(
    'rejects remote plaintext, endpoint credentials and path traversal',
    () async {
      for (final endpoint in [
        'http://example.com/mcp',
        'https://user:pass@example.com/mcp',
        'https://example.com/mcp?token=secret',
      ]) {
        profile['endpoint'] = endpoint;
        await expectLater(load(), throwsFormatException);
      }
      profile['endpoint'] = 'https://example.com/mcp';
      profile['uiFile'] = '../ui.html';
      await expectLater(load(), throwsFormatException);
    },
  );
  test('rejects tampered UI before loading or contacting a server', () async {
    await File(
      '${directory.path}/ui.html',
    ).writeAsString('<script>tampered()</script>');
    await expectLater(load(), throwsStateError);
  });
  test(
    'bound installation rejects a different account before network access',
    () async {
      profile['owner'] = sha256.convert(utf8.encode(scope)).toString();
      final app = (await load()).single;
      scope = 'account-b';
      await expectLater(app.request('tools/list', {}), throwsStateError);
      expect(secrets, isEmpty);
    },
  );
  test(
    'Agent routing discovers installed Apps and opens only the requested App',
    () async {
      final app = (await load()).single;
      var opens = 0;
      app.onOpen = () => opens++;
      final router = RemoteMcpAppsRouter([app]);
      final listed = await router.call('apps_list', {});
      expect((listed['structuredContent'] as Map)['apps'], hasLength(1));
      await router.call('apps_open', {'appId': app.id});
      expect(opens, 1);
      await expectLater(
        router.call('apps_open', {'appId': 'unknown'}),
        throwsStateError,
      );
      expect(opens, 1);
    },
  );
  test(
    'actual development backend discovers tools, validates schema and protects credentials',
    () async {
      final envPath = Platform.environment['OPENMUSE_REMOTE_APP_TEST_ENV'];
      if (envPath == null) return;
      final vars = {
        for (final line in await File(envPath).readAsLines())
          if (line.contains('='))
            line.substring(0, line.indexOf('=')): line.substring(
              line.indexOf('=') + 1,
            ),
      };
      final html = await File(
        Platform.environment['OPENMUSE_REMOTE_APP_TEST_UI']!,
      ).readAsString();
      await File('${directory.path}/ui.html').writeAsString(html);
      profile['endpoint'] = 'http://127.0.0.1:4180/mcp';
      profile['uiResourceUri'] = 'ui://muse-ci/main';
      profile['uiSha256'] = sha256.convert(utf8.encode(html)).toString();
      await File(
        '${directory.path}/example.app.token',
      ).writeAsString(vars['CI_MCP_TOKEN']!);
      final app = (await load()).single;
      addTearDown(app.closeSession);
      final tools = await app.request('tools/list', {});
      expect(
        (tools['tools'] as List).any((t) => t['name'] == 'ci_project_get'),
        isTrue,
      );
      expect(
        await File('${directory.path}/example.app.token').exists(),
        isFalse,
      );
      final result = await app.request('tools/call', {
        'name': 'ci_capabilities',
        'arguments': {},
      });
      expect(result['isError'], isNot(true));
      expect(jsonEncode(result), isNot(contains(vars['CI_MCP_TOKEN'])));
      // Exercise new nested document-edit schemas through the real shared
      // Host runtime, not only the backend's TypeScript client.
      final marker = DateTime.now().microsecondsSinceEpoch.toString();
      final created = await app.request('tools/call', {
        'name': 'ci_lifecycle_create',
        'arguments': {
          'kind': 'requirement',
          'title': 'HostRuntime LC2 $marker',
          'goal': 'Native document and Agent schema conformance',
          'commandId': 'host-lc2-$marker',
        },
      }, agent: true);
      final project = (created['structuredContent'] as Map)['project'] as Map;
      final docId =
          ((project['lifecycle']['documents'] as List).firstWhere(
                (d) => d['nodeId'] == 'feature-prd',
              )
              as Map)['id'];
      final read = await app.request('tools/call', {
        'name': 'ci_document_get',
        'arguments': {'projectId': project['id'], 'documentId': docId},
      }, agent: true);
      final nativeDoc = (read['structuredContent'] as Map)['document'] as Map;
      final changed = await app.request('tools/call', {
        'name': 'ci_agent_document_edit',
        'arguments': {
          'projectId': project['id'],
          'expectedVersion': project['version'],
          'commandId': 'host-edit-$marker',
          'documentId': docId,
          'expectedRevision': nativeDoc['revision'],
          'summary': 'Host runtime Agent contribution',
          'edits': [
            {
              'op': 'append',
              'type': 'paragraph',
              'text': 'Verified Host contribution $marker',
            },
          ],
        },
      }, agent: true);
      expect(changed['isError'], isNot(true));
      final updated = await app.request('tools/call', {
        'name': 'ci_document_get',
        'arguments': {'projectId': project['id'], 'documentId': docId},
      }, agent: true);
      final content = updated['structuredContent'] as Map;
      expect((content['document']['versions'] as List).last['role'], 'agent');
      expect(
        jsonEncode(content['blocks']),
        contains('Verified Host contribution $marker'),
      );
      expect(
        jsonDecode(content['nativeFile']['content'])['profile'],
        'muse.smart-document/phase1',
      );

      await expectLater(
        app.request('tools/call', {'name': 'undeclared', 'arguments': {}}),
        throwsStateError,
      );
      await expectLater(
        app.request('tools/call', {'name': 'ci_project_get', 'arguments': {}}),
        throwsA(anything),
      );
      scope = 'account-b';
      await expectLater(app.request('tools/list', {}), throwsStateError);
    },
    skip: Platform.environment['OPENMUSE_REMOTE_APP_TEST_ENV'] == null
        ? 'Opt-in local authenticated backend.'
        : false,
  );
}
