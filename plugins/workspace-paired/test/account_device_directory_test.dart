import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test('register and list use the authenticated account device API', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final requests = <String>[];
    server.listen((request) async {
      requests.add('${request.method} ${request.uri.path}');
      expect(
        request.headers.value(HttpHeaders.authorizationHeader),
        'Bearer access',
      );
      if (request.method == 'POST') {
        final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
        expect(body['deviceId'], 'mobile.1');
      }
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'code': 0,
          'data': request.method == 'GET'
              ? [_deviceJson(online: true)]
              : _deviceJson(online: true),
        }),
      );
      await request.response.close();
    });
    final client = AccountDeviceDirectoryClient(
      cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'access',
      allowInsecureLoopback: true,
    );

    await client.register(
      const AccountDeviceRegistration(
        deviceRef: 'mobile.1',
        displayName: 'Phone',
        platform: 'android',
        kind: AccountDeviceKind.mobile,
      ),
    );
    final devices = await client.list();

    expect(requests, ['POST /api/muse/devices', 'GET /api/muse/devices']);
    expect(devices.single.deviceRef, 'desktop.1');
    expect(devices.single.supportsPairedDesktop, isTrue);
    client.close();
    await server.close(force: true);
  });

  test('offline Desktop is rejected before opening its transport', () async {
    final auth = _Authentication();
    final directory = AccountDeviceDirectoryController(
      authentication: auth,
      client: AccountDeviceDirectoryClient(
        cloudOrigin: Uri.parse('http://127.0.0.1:1'),
        accessToken: () async => 'access',
        allowInsecureLoopback: true,
      ),
      registration: () => const AccountDeviceRegistration(
        deviceRef: 'mobile.1',
        displayName: 'Phone',
        platform: 'android',
        kind: AccountDeviceKind.mobile,
      ),
    );
    final controller = PairedDesktopMobileController.discovered(
      directory: directory,
      accessToken: () async => 'access',
      requesterDeviceRef: 'mobile.1',
      allowInsecureLoopback: true,
      allowInsecurePrivateNetworkForTesting: false,
    );

    final paired = await controller.pairDevice(
      AccountDevice.fromJson(_deviceJson(online: false)),
      '123456',
    );

    expect(paired, isFalse);
    expect(controller.snapshot.failureMessage, contains('离线'));
    controller.dispose();
    directory.dispose();
  });
}

Map<String, Object?> _deviceJson({required bool online}) => {
  'deviceId': 'desktop.1',
  'displayName': 'MacBook Pro',
  'platform': 'macos',
  'deviceKind': 'desktop',
  'capabilities': ['paired-desktop.transport'],
  'transportOrigin': 'https://desktop.invalid',
  'lastSeenAt': DateTime.now().millisecondsSinceEpoch,
  'online': online,
};

final class _Authentication extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot get snapshot =>
      const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => 'access';

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}
