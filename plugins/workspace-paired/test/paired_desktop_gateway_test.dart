import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'same-account device opens the real Desktop DSH without a code',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      String? observedOrigin;
      String? observedReferer;
      upstream.listen((request) async {
        if (request.uri.queryParameters['token'] == 'bootstrap') {
          request.response.cookies.add(Cookie('dsh-auth', 'accepted'));
          request.response
            ..statusCode = HttpStatus.seeOther
            ..headers.set(HttpHeaders.locationHeader, './');
        } else if (request.cookies.any(
          (cookie) => cookie.name == 'dsh-auth' && cookie.value == 'accepted',
        )) {
          observedOrigin = request.headers.value('origin');
          observedReferer = request.headers.value(HttpHeaders.refererHeader);
          request.response.headers.set(
            HttpHeaders.contentEncodingHeader,
            'gzip',
          );
          request.response.add(
            gzip.encode(utf8.encode('desktop-live-history')),
          );
        } else {
          request.response.statusCode = HttpStatus.unauthorized;
        }
        await request.response.close();
      });
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => 'account-1',
        validateToken: (token) async =>
            token == 'same-account' ? 'account-1' : 'account-2',
        dshEndpoint: () async =>
            Uri.parse('http://127.0.0.1:${upstream.port}/?token=bootstrap'),
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Project Workspace',
        port: 0,
        fixedPairingCode: '123456',
      );
      await gateway.start();
      final client = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'same-account',
        deviceRef: 'mobile-1',
        allowInsecureLoopback: true,
      );

      final connection = await client.connectSameAccount(
        targetDeviceRef: 'desktop.local',
      );
      expect(connection.accountRef, 'account-1');
      expect(connection.workspaceRef, 'openmuse.local.default');
      expect(connection.session.path, startsWith('/u/'));

      final browser = HttpClient();
      final bootstrap = await browser.getUrl(
        Uri.parse(connection.session.origin).resolve(connection.session.path),
      );
      bootstrap.followRedirects = false;
      final bootstrapResponse = await bootstrap.close();
      expect(bootstrapResponse.statusCode, HttpStatus.seeOther);
      expect(bootstrapResponse.headers.value(HttpHeaders.locationHeader), '/');
      await bootstrapResponse.drain<void>();
      final cookies = bootstrapResponse.cookies;
      expect(cookies.any((cookie) => cookie.name == 'OpenMuse-Paired'), isTrue);
      expect(cookies.any((cookie) => cookie.name == 'dsh-auth'), isTrue);

      final history = await browser.getUrl(
        Uri.parse(connection.session.origin).resolve('/api/session.history'),
      );
      history.cookies.addAll(cookies);
      history.headers
        ..set('origin', Uri.parse(connection.session.origin).origin)
        ..set(
          HttpHeaders.refererHeader,
          '${Uri.parse(connection.session.origin).origin}/',
        );
      final historyResponse = await history.close();
      expect(
        historyResponse.headers.value(HttpHeaders.contentEncodingHeader),
        isNull,
      );
      expect(await utf8.decodeStream(historyResponse), 'desktop-live-history');
      expect(observedOrigin, 'http://127.0.0.1:${upstream.port}');
      expect(observedReferer, 'http://127.0.0.1:${upstream.port}/');

      browser.close(force: true);
      client.close();
      await gateway.stop();
      await upstream.close(force: true);
    },
  );

  test('account mismatch and invalid workspace fail closed', () async {
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (_) async => 'another-account',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:54321/?token=x'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      port: 0,
      fixedPairingCode: '123456',
    );
    await gateway.start();
    final client = PairedDesktopClient(
      origin: gateway.origin!,
      accessToken: () async => 'different-account',
      deviceRef: 'mobile-1',
      allowInsecureLoopback: true,
    );

    await expectLater(
      client.connectSameAccount(targetDeviceRef: 'desktop.local'),
      throwsA(
        isA<PairedDesktopFailure>().having(
          (value) => value.code,
          'code',
          'ACCOUNT_MISMATCH',
        ),
      ),
    );

    client.close();
    await gateway.stop();
  });

  test('selected device must match the receiving Desktop', () async {
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:54321/?token=x'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      deviceRef: 'desktop.expected',
      port: 0,
      fixedPairingCode: '123456',
    );
    await gateway.start();
    final client = PairedDesktopClient(
      origin: gateway.origin!,
      accessToken: () async => 'same-account',
      deviceRef: 'mobile-1',
      allowInsecureLoopback: true,
    );

    await expectLater(
      client.connectSameAccount(targetDeviceRef: 'desktop.another'),
      throwsA(
        isA<PairedDesktopFailure>().having(
          (value) => value.code,
          'code',
          'WORKSPACE_GRANT_DENIED',
        ),
      ),
    );

    client.close();
    await gateway.stop();
  });
}
