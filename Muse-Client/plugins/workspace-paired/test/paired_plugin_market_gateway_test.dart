import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'market calls require a current paired grant and a fixed action',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      upstream.listen((request) async {
        await request.drain<void>();
        await request.response.close();
      });
      var account = 'account-1';
      final calls = <PairedPluginMarketRequest>[];
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => account,
        validateToken: (_) async => 'account-1',
        dshEndpoint: () async =>
            Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Workspace',
        port: 0,
        pluginMarket: (request) async {
          calls.add(request);
          return {'ok': true};
        },
      );
      await gateway.start();
      final pair = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'token',
        deviceRef: 'mobile-1',
        allowInsecureLoopback: true,
      );
      final connection = await pair.connectSameAccount(
        targetDeviceRef: 'desktop.local',
      );
      final browser = HttpClient();

      Future<int> send(String operation, {bool granted = true}) async {
        final request = await browser.postUrl(
          gateway.origin!.replace(path: '/openmuse/plugin-market/v1'),
        );
        if (granted)
          request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
        request.write(
          jsonEncode({'operation': operation, 'pluginId': 'archify'}),
        );
        final response = await request.close();
        await response.drain<void>();
        return response.statusCode;
      }

      expect(await send('install', granted: false), HttpStatus.unauthorized);
      expect(await send('shell'), HttpStatus.badRequest);
      expect(await send('install'), HttpStatus.ok);
      expect(calls.single.accountRef, 'account-1');
      expect(calls.single.pluginId, 'archify');
      account = 'account-2';
      expect(await send('disable'), HttpStatus.unauthorized);
      expect(calls, hasLength(1));

      browser.close(force: true);
      pair.close();
      await gateway.stop();
      await upstream.close(force: true);
    },
  );
}
