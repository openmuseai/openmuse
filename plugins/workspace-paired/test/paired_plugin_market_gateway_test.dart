import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'normal host advertises a free loopback port without taking over another installation',
    () async {
      final existing = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => 'account',
        validateToken: (_) async => 'account',
        dshEndpoint: () async => Uri.parse('http://127.0.0.1:1'),
        workspaceRef: 'workspace',
        workspaceTitle: 'Workspace',
        port: existing.port,
        fallbackOnPortConflict: true,
      );
      try {
        await gateway.start();
        expect(gateway.origin!.port, isNot(existing.port));
        expect(gateway.origin!.host, '127.0.0.1');
        expect(gateway.running, true);
      } finally {
        await gateway.stop();
        await existing.close(force: true);
      }
    },
  );
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

      Future<int> send(
        String operation, {
        bool granted = true,
        String pluginId = 'archify',
        Map<String, Object?> options = const {},
      }) async {
        final request = await browser.postUrl(
          gateway.origin!.replace(path: '/openmuse/plugin-market/v1'),
        );
        if (granted)
          request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
        request.write(
          jsonEncode({
            'operation': operation,
            'pluginId': pluginId,
            if (options.isNotEmpty) 'options': options,
          }),
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
      expect(await send('configure'), HttpStatus.badRequest);
      expect(
        await send(
          'configure',
          pluginId: 'com.openmuse.code-navigation.dart',
          options: {
            'expectedRevision': 3,
            'values': {'sdkPath': '/path/to/dart'},
          },
        ),
        HttpStatus.ok,
      );
      expect(calls.last.options['expectedRevision'], 3);
      final proxy = await browser.postUrl(
        gateway.origin!.resolve('/openmuse-native/v1/code-navigation/manage'),
      );
      proxy.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
      proxy.write(
        jsonEncode({'partition': 'another-account', 'operation': 'install'}),
      );
      final denied = await proxy.close();
      expect(denied.statusCode, HttpStatus.forbidden);
      await denied.drain<void>();
      account = 'account-2';
      expect(await send('disable'), HttpStatus.unauthorized);
      expect(calls, hasLength(2));

      browser.close(force: true);
      pair.close();
      await gateway.stop();
      await upstream.close(force: true);
    },
  );
}
