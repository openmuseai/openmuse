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

  test(
    'switching account clears the previous device list immediately',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      server.listen((request) async {
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
      final auth = _Authentication(authenticated: true);
      final client = AccountDeviceDirectoryClient(
        cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'access',
        allowInsecureLoopback: true,
      );
      final directory = AccountDeviceDirectoryController(
        authentication: auth,
        client: client,
        registration: () => const AccountDeviceRegistration(
          deviceRef: 'mobile.1',
          displayName: 'Phone',
          platform: 'android',
          kind: AccountDeviceKind.mobile,
        ),
      );
      addTearDown(() async {
        directory.dispose();
        client.close();
        await server.close(force: true);
      });
      await directory.activate();
      expect(directory.snapshot.devices, hasLength(1));

      auth.switchSubject('account.2');
      expect(directory.snapshot.devices, isEmpty);
      final deadline = DateTime.now().add(const Duration(seconds: 3));
      while (directory.snapshot.devices.isEmpty &&
          DateTime.now().isBefore(deadline)) {
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }
      expect(directory.snapshot.devices, hasLength(1));
    },
  );

  test('device 401 refreshes the token and retries the request once', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final tokens = <String?>[];
    server.listen((request) async {
      tokens.add(request.headers.value(HttpHeaders.authorizationHeader));
      await request.drain<void>();
      request.response.headers.contentType = ContentType.json;
      if (tokens.length == 1) {
        request.response.statusCode = HttpStatus.unauthorized;
        request.response.write(jsonEncode({'code': 401}));
      } else {
        request.response.write(
          jsonEncode({
            'code': 0,
            'data': [_deviceJson(online: true)],
          }),
        );
      }
      await request.response.close();
    });
    var refreshes = 0;
    final client = AccountDeviceDirectoryClient(
      cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'stale',
      refreshAccessToken: () async {
        refreshes++;
        return 'fresh';
      },
      allowInsecureLoopback: true,
    );

    expect((await client.list()).single.deviceRef, 'desktop.1');
    expect(tokens, ['Bearer stale', 'Bearer fresh']);
    expect(refreshes, 1);
    client.close();
    await server.close(force: true);
  });

  test(
    'repeated device 401 is reported without another refresh loop',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var requests = 0;
      server.listen((request) async {
        requests++;
        await request.drain<void>();
        request.response.statusCode = HttpStatus.unauthorized;
        request.response.write(jsonEncode({'code': 401}));
        await request.response.close();
      });
      var refreshes = 0;
      final client = AccountDeviceDirectoryClient(
        cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'stale',
        refreshAccessToken: () async {
          refreshes++;
          return 'fresh';
        },
        allowInsecureLoopback: true,
      );

      await expectLater(
        client.list(),
        throwsA(
          isA<AccountDeviceDirectoryFailure>().having(
            (error) => error.code,
            'code',
            'AUTH_REJECTED',
          ),
        ),
      );
      expect(requests, 2);
      expect(refreshes, 1);
      client.close();
      await server.close(force: true);
    },
  );

  test('token refresh failure becomes a retryable device error', () async {
    final client = AccountDeviceDirectoryClient(
      cloudOrigin: Uri.parse('http://127.0.0.1:1'),
      accessToken: () async => throw StateError('network unavailable'),
      allowInsecureLoopback: true,
    );

    await expectLater(
      client.list(),
      throwsA(
        isA<AccountDeviceDirectoryFailure>().having(
          (error) => error.code,
          'code',
          'AUTH_REFRESH_FAILED',
        ),
      ),
    );
    client.close();
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

    final paired = await controller.connectDevice(
      AccountDevice.fromJson(_deviceJson(online: false)),
    );

    expect(paired, isFalse);
    expect(controller.snapshot.failureMessage, contains('离线'));
    controller.dispose();
    directory.dispose();
  });

  test('failed renewal keeps the previously selected Desktop grant', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    var opens = 0;
    server.listen((request) async {
      opens++;
      request.response.headers.contentType = ContentType.json;
      if (opens == 1) {
        request.response.write(
          jsonEncode({
            'accountRef': 'account.1',
            'deviceRef': 'desktop.1',
            'deviceName': 'MacBook Pro',
            'workspaceRef': 'workspace.1',
            'workspaceTitle': 'Workspace',
            'grantRef': 'grant.1',
            'expiresAtMs': DateTime.now()
                .add(const Duration(minutes: 30))
                .millisecondsSinceEpoch,
            'session': {
              'sessionRef': 'paired-dsh:grant.1',
              'origin': 'http://127.0.0.1:${server.port}',
              'path': '/u/grant.1',
              'generation': 1,
              'allowInsecureLoopback': true,
            },
          }),
        );
      } else {
        request.response.statusCode = HttpStatus.forbidden;
        request.response.write(
          jsonEncode({'code': 'DENIED', 'message': 'Temporary failure'}),
        );
      }
      await request.response.close();
    });
    final auth = _Authentication(authenticated: true);
    final directory = AccountDeviceDirectoryController(
      authentication: auth,
      client: AccountDeviceDirectoryClient(
        cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: auth.accessToken,
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
      accessToken: auth.accessToken,
      requesterDeviceRef: 'mobile.1',
      allowInsecureLoopback: true,
      allowInsecurePrivateNetworkForTesting: false,
    );
    final device = AccountDevice.fromJson({
      ..._deviceJson(online: true),
      'transportOrigin': 'http://127.0.0.1:${server.port}',
    });

    expect(await controller.connectDevice(device), isTrue);
    expect(await controller.connectDevice(device, force: true), isFalse);
    expect(controller.snapshot.connection?.grantRef, 'grant.1');
    expect(controller.snapshot.failureMessage, 'Temporary failure');

    controller.dispose();
    directory.dispose();
    await server.close(force: true);
  });

  test(
    'failed initial registration retries and recovers without relogin',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var registrations = 0;
      server.listen((request) async {
        if (request.uri.path == '/api/muse/devices/events') {
          request.response.statusCode = HttpStatus.notFound;
          await request.response.close();
          return;
        }
        request.response.headers.contentType = ContentType.json;
        if (request.method == 'POST') {
          registrations++;
          if (registrations == 1) {
            request.response
              ..statusCode = HttpStatus.serviceUnavailable
              ..write(jsonEncode({'code': 1}));
            await request.response.close();
            return;
          }
        }
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
      final auth = _Authentication(authenticated: true);
      final controller = AccountDeviceDirectoryController(
        authentication: auth,
        client: AccountDeviceDirectoryClient(
          cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
          accessToken: auth.accessToken,
          allowInsecureLoopback: true,
        ),
        registration: () => const AccountDeviceRegistration(
          deviceRef: 'mobile.1',
          displayName: 'Phone',
          platform: 'android',
          kind: AccountDeviceKind.mobile,
        ),
        heartbeatInterval: const Duration(seconds: 30),
        maxReconnectDelay: const Duration(milliseconds: 5),
      );

      await controller.activate();
      for (
        var attempt = 0;
        attempt < 20 && !controller.snapshot.registered;
        attempt++
      ) {
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }

      expect(registrations, greaterThanOrEqualTo(2));
      expect(controller.snapshot.registered, isTrue);
      controller.dispose();
      await server.close(force: true);
    },
  );

  test(
    'refreshing the same account does not restart device registration',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var registrations = 0;
      server.listen((request) async {
        if (request.uri.path.endsWith('/events')) {
          request.response.statusCode = HttpStatus.notFound;
          await request.response.close();
          return;
        }
        if (request.method == 'POST') registrations++;
        await request.drain<void>();
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
      final auth = _Authentication(authenticated: true);
      final controller = AccountDeviceDirectoryController(
        authentication: auth,
        client: AccountDeviceDirectoryClient(
          cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
          accessToken: auth.accessToken,
          allowInsecureLoopback: true,
        ),
        registration: () => const AccountDeviceRegistration(
          deviceRef: 'mobile.1',
          displayName: 'Phone',
          platform: 'android',
          kind: AccountDeviceKind.mobile,
        ),
        heartbeatInterval: const Duration(hours: 1),
        maxReconnectDelay: const Duration(hours: 1),
      );

      await controller.activate();
      auth.setPhase(OpenMuseAuthenticationPhase.refreshing);
      auth.setPhase(OpenMuseAuthenticationPhase.authenticated);
      await Future<void>.delayed(const Duration(milliseconds: 20));
      expect(registrations, 1);
      controller.dispose();
      await server.close(force: true);
    },
  );

  test(
    'device event websocket authenticates and delivers refresh hints',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      server.listen((request) async {
        expect(
          request.headers.value(HttpHeaders.authorizationHeader),
          'Bearer access',
        );
        final socket = await WebSocketTransformer.upgrade(request);
        socket.add(jsonEncode({'type': 'device.snapshot-required'}));
      });
      final client = AccountDeviceDirectoryClient(
        cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'access',
        allowInsecureLoopback: true,
      );

      final socket = await client.connectEvents();
      expect(await socket.first, contains('snapshot-required'));

      await socket.close();
      client.close();
      await server.close(force: true);
    },
  );
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
  _Authentication({this.authenticated = false});

  final bool authenticated;
  String subject = 'account.1';
  OpenMuseAuthenticationPhase phase = OpenMuseAuthenticationPhase.authenticated;

  void switchSubject(String value) {
    subject = value;
    notifyListeners();
  }

  void setPhase(OpenMuseAuthenticationPhase value) {
    phase = value;
    notifyListeners();
  }

  @override
  OpenMuseAuthenticationSnapshot get snapshot => authenticated
      ? OpenMuseAuthenticationSnapshot(
          phase: phase,
          identity: OpenMuseAuthenticatedIdentity(
            subject: subject,
            email: 'account@example.test',
          ),
        )
      : const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => 'access';

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}
