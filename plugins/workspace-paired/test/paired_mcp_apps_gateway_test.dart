import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'paired App RPC authenticates every call and discards revoked in-flight responses',
    () async {
      String? account = 'account-1';
      var calls = 0;
      final entered = Completer<void>(), finish = Completer<void>();
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => account,
        validateToken: (_) async => 'account-1',
        dshEndpoint: () async => Uri.parse('http://127.0.0.1:9/'),
        workspaceRef: 'workspace',
        workspaceTitle: 'Workspace',
        port: 0,
        mcpApps: (request) async {
          calls++;
          expect(request.accountRef, 'account-1');
          if (request.body['method'] == 'tools/call') {
            entered.complete();
            await finish.future;
          }
          return {'apps': []};
        },
      );
      await gateway.start();
      final client = HttpClient();
      final pair = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'test-token',
        deviceRef: 'mobile-1',
        allowInsecureLoopback: true,
      );
      Future<int> post(Map<String, Object?> body, [String? grant]) async {
        final r = await client.postUrl(
          gateway.origin!.resolve('/openmuse/mcp-apps/v1'),
        );
        if (grant != null) r.cookies.add(Cookie('OpenMuse-Paired', grant));
        r.headers.contentType = ContentType.json;
        r.write(jsonEncode(body));
        final response = await r.close();
        await response.drain<void>();
        return response.statusCode;
      }

      try {
        expect(await post({'method': 'apps/list'}), 401);
        expect(calls, 0);
        final connection = await pair.connectSameAccount(
          targetDeviceRef: 'desktop.local',
          workspaceRef: 'workspace',
        );
        expect(await post({'method': 'apps/list'}, connection.grantRef), 200);
        expect(calls, 1);
        expect(await post({'method': 'arbitrary'}, connection.grantRef), 400);
        expect(calls, 1);
        final pending = post({
          'method': 'tools/call',
          'appId': 'example',
          'params': {},
        }, connection.grantRef);
        await entered.future;
        account = 'another-account';
        finish.complete();
        expect(await pending, 401);
      } finally {
        client.close(force: true);
        pair.close();
        await gateway.stop();
      }
    },
  );
}
