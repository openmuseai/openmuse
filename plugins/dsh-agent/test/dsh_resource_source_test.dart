import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/dsh_resource_source.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test(
    'resource identity survives owner restart and expired transport handles without fuzzy rebinding',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var expired = false;
      var canonical = 0;
      final content = utf8.encode('owner content');
      server.listen((r) async {
        final body = jsonDecode(await utf8.decoder.bind(r).join()) as Map;
        expect(r.headers.value('x-openmuse-bridge-token'), 'test-token');
        r.response.headers.contentType = ContentType.json;
        if (r.uri.path.endsWith('/resolve')) {
          final locator = body['locator'] as Map;
          if (locator['kind'] == 'canonical') {
            canonical++;
            expect(locator['target'], 'src/example.dart');
          }
          if (locator['kind'] == 'relative') {
            expect(locator['target'], '../guides/next.md');
            expect(locator['context']['sessionId'], 's-1');
            expect(locator['context']['baseRef'], 'transport-3');
          }
          r.response.write(
            jsonEncode({
              'matches': [
                {
                  'ref': 'transport-$canonical',
                  'label': locator['kind'] == 'relative'
                      ? 'guides/next.md'
                      : 'src/example.dart',
                },
              ],
              'truncated': false,
            }),
          );
        } else if (expired && body['ref'] == 'transport-0') {
          r.response.statusCode = 400;
          r.response.write(jsonEncode({'error': 'EXPIRED'}));
        } else {
          r.response.write(
            jsonEncode({
              'label': 'src/example.dart',
              'bytes': base64Encode(content),
              'digest': resourceDigest(content),
            }),
          );
        }
        await r.response.close();
      });
      final store = MemoryResourceStateStore();
      final ctx = ResourceSourceContext(
        scope: 'account',
        deviceRef: 'desktop',
        deviceLabel: 'Desktop',
        workspace: 'workspace',
        workspaceLabel: 'Workspace',
        generation: () => 1,
        state: store,
      );
      DshResourceSource source() => DshResourceSource(
        ctx,
        endpoint: () => Uri.parse('http://127.0.0.1:${server.port}'),
        token: () => 'test-token',
        ready: () async {},
        enabled: () => true,
      );
      final access = ResourceAccess(
        scope: 'account',
        audience: 'test',
        generation: 1,
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
        operations: {ResourceOperation.read},
      );
      final first = source();
      final result = await first.locate(
        ResourceLocator(
          owner: 'dsh',
          kind: 'path',
          target: 'example.dart',
          context: {'sessionId': 's-1'},
        ),
        access,
      );
      final id = result.matches.single.reference.identity;
      expect(id.resource, isNot('transport-0'));
      expired = true;
      expect((await first.read(id.resource, null, access)).bytes, content);
      final restarted = source();
      expect(
        (await restarted.read(id.resource, null, access)).record.identity,
        id,
      );
      expect(canonical, 2);
      final again = await restarted.locate(
        ResourceLocator(
          owner: 'dsh',
          kind: 'path',
          target: 'example.dart',
          context: {'sessionId': 's-1'},
        ),
        access,
      );
      expect(again.matches.single.reference.identity, id);
      final neighbor = await restarted.locateRelative(
        result.matches.single.reference,
        '../guides/next.md',
        access,
      );
      expect(neighbor.matches.single.label, 'guides/next.md');
      expect(canonical, 3);
      await server.close(force: true);
    },
  );
}
