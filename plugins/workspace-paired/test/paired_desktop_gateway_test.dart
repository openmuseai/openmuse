import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'closing a mobile SSE consumer cancels upstream without an unhandled socket error',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final upstreamClosed = Completer<void>();
      upstream.listen((request) async {
        request.response.headers.contentType = ContentType(
          'text',
          'event-stream',
        );
        request.response.bufferOutput = false;
        unawaited(
          request.response.done.then<void>((_) {}, onError: (Object _) {}),
        );
        try {
          for (var i = 0; i < 200; i++) {
            request.response.write('data: {"type":"heartbeat"}\n\n');
            await request.response.flush();
            await Future<void>.delayed(const Duration(milliseconds: 20));
          }
          await request.response.close();
        } on HttpException {
          // Expected cancellation of this test's upstream SSE fixture.
        } on SocketException {
          // Expected cancellation of this test's upstream SSE fixture.
        } finally {
          if (!upstreamClosed.isCompleted) upstreamClosed.complete();
        }
      });
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => 'account-1',
        validateToken: (_) async => 'account-1',
        dshEndpoint: () async =>
            Uri.parse('http://127.0.0.1:${upstream.port}/'),
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Workspace',
        port: 0,
      );
      await gateway.start();
      final pair = PairedDesktopClient(
        origin: gateway.origin!,
        accessToken: () async => 'token',
        deviceRef: 'mobile-1',
        allowInsecureLoopback: true,
      );
      final client = HttpClient();
      try {
        final connection = await pair.connectSameAccount(
          targetDeviceRef: 'desktop.local',
        );
        final request = await client.getUrl(gateway.origin!.resolve('/stream'));
        request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
        final response = await request.close();
        final first = Completer<void>();
        final subscription = response.listen((_) {
          if (!first.isCompleted) first.complete();
        }, onError: (Object _) {});
        await first.future.timeout(const Duration(seconds: 3));
        await subscription.cancel();
        client.close(force: true);
        await upstreamClosed.future.timeout(const Duration(seconds: 5));
        await Future<void>.delayed(const Duration(milliseconds: 100));
      } finally {
        client.close(force: true);
        pair.close();
        await gateway.stop();
        await upstream.close(force: true);
      }
    },
  );

  test('account change invalidates paired proxy and workspace access', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    upstream.listen((request) async {
      await request.drain<void>();
      request.response.write('ok');
      await request.response.close();
    });
    var account = 'account-1';
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => account,
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Workspace',
      port: 0,
      workspaceMirror: (_) async => {'mounts': <Object>[]},
      remoteSurface: (_) async => {'ok': true},
    );
    await gateway.start();
    final pair = PairedDesktopClient(
      origin: gateway.origin!,
      accessToken: () async => 'token',
      deviceRef: 'mobile-1',
      allowInsecureLoopback: true,
    );
    final client = HttpClient();
    try {
      final connection = await pair.connectSameAccount(
        targetDeviceRef: 'desktop.local',
      );
      account = 'account-2';
      Future<int> get(String path, {bool cookie = true}) async {
        final request = await client.getUrl(gateway.origin!.resolve(path));
        if (cookie)
          request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
        final response = await request.close();
        await response.drain<void>();
        return response.statusCode;
      }

      expect(
        await get('/u/${connection.grantRef}', cookie: false),
        HttpStatus.unauthorized,
      );
      expect(await get('/api/session.history'), HttpStatus.unauthorized);
      expect(
        await get(
          '/openmuse/workspace-mirror/v1?operation=mounts&workspaceRef=openmuse.local.default',
        ),
        HttpStatus.unauthorized,
      );
      final request = await client.postUrl(
        gateway.origin!.resolve('/openmuse/remote-surface/v1'),
      );
      request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
      request.write('{"operation":"discover","body":{}}');
      final response = await request.close();
      await response.drain<void>();
      expect(response.statusCode, HttpStatus.unauthorized);
    } finally {
      client.close(force: true);
      pair.close();
      await gateway.stop();
      await upstream.close(force: true);
    }
  });

  test(
    'same-account device opens the real Desktop DSH without a code',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      String? observedOrigin;
      String? observedReferer;
      String? observedNativeToken;
      upstream.listen((request) async {
        if (request.uri.queryParameters['token'] == 'bootstrap') {
          request.response.cookies.add(Cookie('dsh-auth', 'accepted'));
          request.response
            ..statusCode = HttpStatus.seeOther
            ..headers.set(HttpHeaders.locationHeader, './');
        } else if (request.cookies.any(
          (cookie) => cookie.name == 'dsh-auth' && cookie.value == 'accepted',
        )) {
          observedNativeToken = request.headers.value(
            'x-openmuse-bridge-token',
          );
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
        nativeApiToken: List.filled(32, 'n').join(),
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

      final native = await browser.getUrl(
        Uri.parse(
          connection.session.origin,
        ).resolve('/openmuse-native/v1/hello'),
      );
      native.cookies.addAll(cookies);
      native.headers.set('x-openmuse-bridge-token', 'mobile-spoof');
      final nativeResponse = await native.close();
      await nativeResponse.drain<void>();
      expect(observedNativeToken, List.filled(32, 'n').join());

      browser.close(force: true);
      client.close();
      await gateway.stop();
      await upstream.close(force: true);
    },
  );

  test(
    'a short live follow reaches the client before the stream ends',
    () async {
      final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final release = Completer<void>();
      upstream.listen((request) async {
        if (!request.uri.path.contains('follow')) {
          request.response.write('ok');
          await request.response.close();
          return;
        }
        request.response
          ..bufferOutput = false
          ..headers.set(HttpHeaders.contentTypeHeader, 'text/event-stream')
          ..write('event: frame\ndata: {"type":"snapshot"}\n\n');
        await request.response.flush();
        await release.future;
        await request.response.close();
      });
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => 'account-1',
        validateToken: (token) async => 'account-1',
        dshEndpoint: () async =>
            Uri.parse('http://127.0.0.1:${upstream.port}/'),
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Project Workspace',
        port: 0,
        fixedPairingCode: '123456',
        nativeApiToken: List.filled(32, 'n').join(),
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
      final browser = HttpClient();
      final opened = await browser.getUrl(
        Uri.parse(connection.session.origin).resolve(connection.session.path),
      );
      opened.followRedirects = false;
      final openedResponse = await opened.close();
      final cookies = openedResponse.cookies;
      await openedResponse.drain<void>();
      final follow = await browser.getUrl(
        Uri.parse(
          connection.session.origin,
        ).resolve('/openmuse-native/v1/session/follow?sessionId=s-1'),
      );
      follow.cookies.addAll(cookies);
      final response = await follow.close().timeout(const Duration(seconds: 3));
      final body = StringBuffer();
      final lines = StreamIterator<String>(
        response
            .transform(utf8.decoder)
            .transform(const LineSplitter())
            .timeout(const Duration(seconds: 3)),
      );
      while (await lines.moveNext()) {
        body.write(lines.current);
        if (body.toString().contains('snapshot')) break;
      }
      expect(body.toString(), contains('snapshot'));
      release.complete();
      while (await lines.moveNext()) {}
      await lines.cancel();
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

  test('loopback hop header publishes the public HTTPS origin', () async {
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:54321/?token=x'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      deviceRef: 'desktop.expected',
      port: 0,
    );
    await gateway.start();
    final client = HttpClient();
    final request = await client.postUrl(
      gateway.origin!.replace(path: '/v1/account/open'),
    );
    request.headers
      ..contentType = ContentType.json
      ..set(HttpHeaders.authorizationHeader, 'Bearer token')
      ..set('x-openmuse-paired-public-origin', 'https://link.openmuse.test');
    request.add(
      utf8.encode(
        jsonEncode({
          'deviceRef': 'mobile-1',
          'targetDeviceRef': 'desktop.expected',
          'workspaceRef': 'openmuse.local.default',
        }),
      ),
    );
    final response = await request.close();
    final body = jsonDecode(await utf8.decodeStream(response)) as Map;
    expect(response.statusCode, HttpStatus.ok);
    expect((body['session'] as Map)['origin'], 'https://link.openmuse.test');
    expect((body['session'] as Map)['allowInsecureLoopback'], isFalse);
    client.close(force: true);
    await gateway.stop();
  });

  test(
    'workspace actions require a current scoped grant and JSON same-site request',
    () async {
      var account = 'account-1';
      final calls = <WorkspaceMirrorAction>[];
      final gateway = PairedDesktopGateway(
        currentAccountRef: () => account,
        validateToken: (_) async => account,
        dshEndpoint: () async => Uri.parse('http://127.0.0.1:1/'),
        workspaceRef: 'workspace-1',
        workspaceTitle: 'Workspace',
        port: 0,
        workspaceMirrorAction: (action) async {
          calls.add(action);
          return {'ok': true};
        },
      );
      await gateway.start();
      addTearDown(gateway.stop);
      final grant = gateway.issueLoopbackOperatorGrant(
        accountRef: account,
        deviceRef: 'browser',
      );
      final client = HttpClient();
      addTearDown(() => client.close(force: true));
      Future<int> send({
        bool cookie = true,
        String workspace = 'workspace-1',
        bool json = true,
        bool crossSite = false,
      }) async {
        final req = await client.postUrl(
          gateway.origin!.resolve('/openmuse/workspace-mirror/action/v1'),
        );
        if (cookie) req.cookies.add(Cookie('OpenMuse-Paired', grant));
        if (json) req.headers.contentType = ContentType.json;
        if (crossSite) req.headers.set('sec-fetch-site', 'cross-site');
        req.write(
          jsonEncode({
            'workspaceRef': workspace,
            'operation': 'new-file',
            'nodeRef': 'opaque',
            'name': 'hello.md',
          }),
        );
        final response = await req.close();
        await response.drain<void>();
        return response.statusCode;
      }

      expect(await send(cookie: false), 401);
      expect(await send(json: false), 415);
      expect(await send(crossSite: true), 403);
      expect(await send(workspace: 'other'), 400);
      expect(calls, isEmpty);
      expect(await send(), 200);
      expect(calls.single.accountRef, 'account-1');
      account = 'account-2';
      expect(await send(), 401);
      expect(calls.length, 1);
    },
  );

  test('workspace mirror requires grant and matching workspace', () async {
    final calls = <WorkspaceMirrorQuery>[];
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:54321/'),
      workspaceRef: 'workspace-1',
      workspaceTitle: 'Workspace',
      port: 0,
      workspaceMirror: (query) async {
        calls.add(query);
        return {'mounts': <Object>[]};
      },
      workspaceMirrorResource: (query) async => WorkspaceMirrorResource(
        bytes: Uint8List.fromList(utf8.encode('file-body')),
        mediaType: 'text/plain; charset=utf-8',
      ),
    );
    await gateway.start();
    addTearDown(gateway.stop);
    final client = HttpClient();
    addTearDown(() => client.close(force: true));
    Future<int> status(String workspaceRef, {String? grant}) async {
      final request = await client.getUrl(
        gateway.origin!.replace(
          path: '/openmuse/workspace-mirror/v1',
          queryParameters: {
            'operation': 'mounts',
            'workspaceRef': workspaceRef,
          },
        ),
      );
      if (grant != null) {
        request.cookies.add(Cookie('OpenMuse-Paired', grant));
      }
      final response = await request.close();
      await response.drain<void>();
      return response.statusCode;
    }

    expect(await status('workspace-1'), HttpStatus.unauthorized);
    final grant = gateway.issueLoopbackOperatorGrant(
      accountRef: 'account-1',
      deviceRef: 'browser-1',
    );
    expect(await status('workspace-2', grant: grant), HttpStatus.badRequest);
    expect(await status('workspace-1', grant: grant), HttpStatus.ok);
    expect(calls.single.deviceRef, 'browser-1');
    expect(calls.single.workspaceRef, 'workspace-1');
    Future<(int, String)> resourceStatus({String? grant}) async {
      final request = await client.getUrl(
        gateway.origin!.replace(
          path: '/openmuse/workspace-mirror/resource/v1',
          queryParameters: {
            'workspaceRef': 'workspace-1',
            'resourceRef': 'opaque-ref',
          },
        ),
      );
      if (grant != null) {
        request.cookies.add(Cookie('OpenMuse-Paired', grant));
      }
      final response = await request.close();
      return (response.statusCode, await utf8.decodeStream(response));
    }

    expect((await resourceStatus()).$1, HttpStatus.unauthorized);
    expect(await resourceStatus(grant: grant), (HttpStatus.ok, 'file-body'));
  });
}
