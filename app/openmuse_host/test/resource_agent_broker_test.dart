import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/resource_agent_broker.dart';

void main() {
  test('installed Apps do not authorize inactive documents', () async {
    final broker = ResourceAgentBroker();
    var appCalls = 0;
    broker.appToolCall = (name, args) async {
      appCalls++;
      return {
        'content': [
          {'type': 'text', 'text': name},
        ],
      };
    };
    final tools = await broker.dispatch('tools/list', {});
    expect(
      (tools['tools'] as List).any((t) => t['name'] == 'apps_list'),
      isTrue,
    );
    final result = await broker.dispatch('tools/call', {
      'name': 'apps_list',
      'arguments': {},
    });
    expect((result['content'] as List).single['text'], 'apps_list');
    expect(appCalls, 1);
    await expectLater(
      broker.dispatch('tools/call', {
        'name': 'call_resource_tool',
        'arguments': {'uri': 'file:///inactive', 'name': 'edit'},
      }),
      throwsStateError,
    );
    broker.appToolCall = null;
    await expectLater(
      broker.dispatch('tools/call', {'name': 'apps_list', 'arguments': {}}),
      throwsStateError,
    );
  });

  test(
    'only the active mounted resource receives opaque plugin tools',
    () async {
      final broker = ResourceAgentBroker();
      const uri = 'file:///test/example.custom';
      final calls = <Map<String, Object?>>[];
      final session = ResourceAppSession(
        uri: uri,
        title: 'example.custom',
        request: (method, params) async {
          if (method == 'tools/list') {
            return {
              'tools': [
                {
                  'name': 'vendor_edit',
                  'inputSchema': {'type': 'object'},
                },
              ],
            };
          }
          calls.add(params);
          return {
            'content': [
              {'type': 'text', 'text': 'plugin confirmed'},
            ],
          };
        },
      );
      broker.register(session);
      final call = {
        'name': 'call_resource_tool',
        'arguments': {
          'uri': uri,
          'name': 'vendor_edit',
          'arguments': {
            'businessField': {
              'arbitrary': [1, 2, 3],
            },
          },
        },
      };
      await expectLater(broker.dispatch('tools/call', call), throwsStateError);
      broker.activeUri = uri;
      final result = await broker.dispatch('tools/call', call);
      expect((result['content'] as List).single['text'], 'plugin confirmed');
      expect(calls.single['arguments'], {
        'businessField': {
          'arbitrary': [1, 2, 3],
        },
      });
      broker.unregister(session);
      await expectLater(broker.dispatch('tools/call', call), throwsStateError);
    },
  );
  test('MCP HTTP requires process token and refuses browser origins', () async {
    {
      final broker = ResourceAgentBroker();
      await broker.start();
      final client = HttpClient();
      try {
        Future<int> call({String? token, String? origin}) async {
          final r = await client.postUrl(broker.endpoint);
          if (token != null) r.headers.set('authorization', 'Bearer $token');
          if (origin != null) r.headers.set('origin', origin);
          r.write(
            jsonEncode({'jsonrpc': '2.0', 'id': 1, 'method': 'tools/list'}),
          );
          final response = await r.close();
          await response.drain<void>();
          return response.statusCode;
        }

        expect(await call(), 401);
        expect(
          await call(token: broker.token, origin: 'https://untrusted.example'),
          401,
        );
        expect(await call(token: broker.token), 200);
      } finally {
        client.close(force: true);
        await broker.close();
      }
    }
  });
}
