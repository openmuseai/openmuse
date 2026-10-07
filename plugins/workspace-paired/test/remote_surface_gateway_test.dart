import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test('mobile prompt owns its plugin challenge and media', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    upstream.listen((request) async {
      await request.drain<void>();
      request.response.headers.contentType = ContentType.json;
      request.response.write('{"accepted":true}');
      await request.response.close();
    });
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      port: 0,
    );
    await gateway.start();
    final paired = PairedDesktopClient(
      origin: gateway.origin!,
      accessToken: () async => 'same-account',
      deviceRef: 'mobile-1',
      allowInsecureLoopback: true,
    );
    final connection = await paired.connectSameAccount(
      targetDeviceRef: 'desktop.local',
    );
    final browser = HttpClient();
    final prompt = await browser.postUrl(gateway.origin!.replace(
      path: '/openmuse-native/v1/session/prompt',
    ));
    prompt.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    prompt.write('{"sessionId":"session-1","text":"publish"}');
    expect((await prompt.close()).statusCode, HttpStatus.ok);
    expect(gateway.offerPluginInteraction(PairedPluginInteraction(
      id: 'challenge-1',
      pluginId: 'com.openmuse.easel',
      title: '扫码登录',
      imageBytes: Uint8List.fromList(const [1, 2, 3]),
      readStatus: () async => {'state': 'qr_ready', 'message': '等待扫码'},
    )), isTrue);
    final feed = await browser.getUrl(gateway.origin!.replace(
      path: '/openmuse/plugin-interaction/v1',
    ));
    feed.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    final decoded = jsonDecode(await utf8.decodeStream(await feed.close()));
    expect(decoded['interaction']['pluginId'], 'com.openmuse.easel');
    expect(decoded['interaction']['state'], 'qr_ready');
    expect(decoded.toString(), isNot(contains('imagePath')));
    final media = await browser.getUrl(gateway.origin!.replace(
      path: '/openmuse/plugin-interaction/media/v1',
      queryParameters: {'handle': 'challenge-1'},
    ));
    media.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    expect(await (await media.close()).fold<List<int>>(
      <int>[], (bytes, chunk) => bytes..addAll(chunk),
    ), [1, 2, 3]);
    final denied = await browser.getUrl(gateway.origin!.replace(
      path: '/openmuse/plugin-interaction/v1',
    ));
    expect((await denied.close()).statusCode, HttpStatus.unauthorized);
    browser.close(force: true);
    paired.close();
    await gateway.stop();
    await upstream.close(force: true);
  });

  test('RS-PAIR remote surface stays off the DSH proxy', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final seen = <String>[];
    upstream.listen((request) async {
      seen.add(request.uri.path);
      request.response.statusCode = HttpStatus.ok;
      await request.response.close();
    });
    final dispatched = <RemoteSurfaceDispatch>[];
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (token) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      port: 0,
      remoteSurface: (request) async {
        dispatched.add(request);
        return {'ok': true, 'operation': request.operation};
      },
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

    final granted = await browser.postUrl(
      Uri.parse('${gateway.origin}/openmuse/remote-surface/v1'),
    );
    granted.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    granted.write(
      jsonEncode({
        'operation': 'discover',
        'body': {'hello': 'mobile'},
      }),
    );
    final grantedResponse = await granted.close();
    expect(grantedResponse.statusCode, HttpStatus.ok);
    expect(jsonDecode(await utf8.decodeStream(grantedResponse)), {
      'ok': true,
      'operation': 'discover',
    });
    expect(dispatched.single.accountRef, 'account-1');
    expect(dispatched.single.deviceRef, 'mobile-1');
    expect(dispatched.single.workspaceRef, 'openmuse.local.default');

    final missing = await browser.postUrl(
      Uri.parse('${gateway.origin}/openmuse/remote-surface/v1'),
    );
    missing.write(jsonEncode({'operation': 'submit', 'body': {}}));
    final missingResponse = await missing.close();
    expect(missingResponse.statusCode, HttpStatus.unauthorized);
    expect(
      jsonDecode(await utf8.decodeStream(missingResponse))['code'],
      'GRANT_REQUIRED',
    );
    expect(seen, isEmpty);

    browser.close(force: true);
    client.close();
    await gateway.stop();
    await upstream.close(force: true);
  });

  test('RS-PAIR a gateway without a dispatcher does not proxy', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    var seen = 0;
    upstream.listen((request) async {
      seen += 1;
      await request.response.close();
    });
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (token) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Project Workspace',
      port: 0,
    );
    await gateway.start();
    final browser = HttpClient();
    final request = await browser.postUrl(
      Uri.parse('${gateway.origin}/openmuse/remote-surface/v1'),
    );
    request.write(jsonEncode({'operation': 'discover', 'body': {}}));
    final response = await request.close();
    expect(response.statusCode, HttpStatus.serviceUnavailable);
    expect(
      jsonDecode(await utf8.decodeStream(response))['code'],
      'SURFACE_UNAVAILABLE',
    );
    expect(seen, 0);
    browser.close(force: true);
    await gateway.stop();
    await upstream.close(force: true);
  });

  test('RS-MEDIA a byte range stays off the DSH proxy', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final seen = <String>[];
    upstream.listen((request) async {
      seen.add(request.uri.path);
      await request.response.close();
    });
    final payload = Uint8List.fromList(const [1, 2, 3, 4, 5, 6, 7, 8]);
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => 'account-1',
      validateToken: (token) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'ws.opaque.1',
      workspaceTitle: 'Project Workspace',
      deviceRef: 'desktop.1',
      port: 0,
      remoteSurface: (request) async => {'ok': true},
      remoteMedia: (query) async {
        expect(query.accountRef, 'account-1');
        expect(query.mobileDeviceRef, 'mobile-1');
        expect(query.desktopDeviceRef, 'desktop.1');
        expect(query.workspaceRef, 'ws.opaque.1');
        if (query.handle != 'media.cover.1') {
          return const RemoteMediaSliceDenied();
        }
        final end = query.endInclusive ?? payload.length - 1;
        if (query.start < 0 || end >= payload.length || end < query.start) {
          return const RemoteMediaSliceUnsatisfiable();
        }
        return RemoteMediaSliceBody(
          bytes: Uint8List.sublistView(payload, query.start, end + 1),
          total: payload.length,
          start: query.start,
        );
      },
    );
    await gateway.start();
    final client = PairedDesktopClient(
      origin: gateway.origin!,
      accessToken: () async => 'same-account',
      deviceRef: 'mobile-1',
      allowInsecureLoopback: true,
    );
    final connection = await client.connectSameAccount(
      targetDeviceRef: 'desktop.1',
      workspaceRef: 'ws.opaque.1',
    );
    final browser = HttpClient();
    final ranged = await browser.getUrl(
      Uri.parse(
        '${gateway.origin}/openmuse/remote-surface/media/v1/media.cover.1',
      ),
    );
    ranged.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    ranged.headers.set(HttpHeaders.rangeHeader, 'bytes=0-3');
    final rangedResponse = await ranged.close();
    final bytes = await rangedResponse.fold<List<int>>(
      <int>[],
      (collected, chunk) => collected..addAll(chunk),
    );
    expect(rangedResponse.statusCode, HttpStatus.partialContent);
    expect(
      rangedResponse.headers.value(HttpHeaders.contentRangeHeader),
      'bytes 0-3/8',
    );
    expect(bytes, [1, 2, 3, 4]);

    final missing = await browser.getUrl(
      Uri.parse(
        '${gateway.origin}/openmuse/remote-surface/media/v1/media.missing',
      ),
    );
    missing.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
    final missingResponse = await missing.close();
    expect(missingResponse.statusCode, HttpStatus.notFound);
    await missingResponse.drain<void>();
    expect(seen, isEmpty);

    browser.close(force: true);
    client.close();
    await gateway.stop();
    await upstream.close(force: true);
  });
}
