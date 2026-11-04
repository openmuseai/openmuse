import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_terminal_service.dart';

void main() {
  test(
    'terminal MCP rejects missing/foreign/revoked credentials and scope races',
    () async {
      var scope = 'account-a';
      final entered = Completer<void>(), finish = Completer<void>();
      final revoked = <String>[];
      final service = McpTerminalService(
        scope: () => scope,
        tools: [
          {
            'name': 'wait',
            'inputSchema': {'type': 'object', 'additionalProperties': false},
          },
        ],
        onRevoke: (caller) async {
          revoked.add(caller);
        },
        call: (caller, root, name, args) async {
          entered.complete();
          await finish.future;
          return {'secret': 'private result'};
        },
      );
      await service.start();
      final root = await Directory.systemTemp.createTemp('mcp-terminal-');
      final env = service.issue(root.path), http = HttpClient();
      Future<(int, String)> post({
        String? token,
        bool origin = false,
        String method = 'tools/list',
      }) async {
        final request = await http.postUrl(service.endpoint);
        if (token != null) {
          request.headers.set('Authorization', 'Bearer $token');
        }
        if (origin) request.headers.set('Origin', 'https://foreign.example');
        request.write(
          jsonEncode({
            'jsonrpc': '2.0',
            'id': 1,
            'method': method,
            'params': {'name': 'wait', 'arguments': {}},
          }),
        );
        final response = await request.close();
        return (response.statusCode, await utf8.decoder.bind(response).join());
      }

      try {
        expect((await post()).$1, 401);
        expect(
          (await post(token: env['MCP_HOST_TOKEN'], origin: true)).$1,
          401,
        );
        expect((await post(token: env['MCP_HOST_TOKEN'])).$1, 200);
        final pending = post(
          token: env['MCP_HOST_TOKEN'],
          method: 'tools/call',
        );
        await entered.future;
        await service.revoke(env['MCP_HOST_TOKEN']!);
        finish.complete();
        final denied = await pending;
        expect(denied.$2, isNot(contains('private result')));
        expect(jsonDecode(denied.$2)['error'], isNotNull);
        expect((await post(token: env['MCP_HOST_TOKEN'])).$1, 401);
        final other = service.issue(root.path);
        scope = 'account-b';
        expect((await post(token: other['MCP_HOST_TOKEN'])).$1, 401);
      } finally {
        await service.close();
        http.close(force: true);
        await root.delete(recursive: true);
      }
      expect(revoked, hasLength(2));
    },
  );
}
