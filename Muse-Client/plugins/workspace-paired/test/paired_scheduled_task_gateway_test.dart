import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test('scheduled tasks require a current paired account grant', () async {
    final upstream = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    upstream.listen((request) async {
      await request.drain<void>();
      await request.response.close();
    });
    var account = 'account-1';
    final calls = <PairedScheduledTaskRequest>[];
    final gateway = PairedDesktopGateway(
      currentAccountRef: () => account,
      validateToken: (_) async => 'account-1',
      dshEndpoint: () async => Uri.parse('http://127.0.0.1:${upstream.port}/'),
      workspaceRef: 'openmuse.local.default',
      workspaceTitle: 'Workspace',
      port: 0,
      scheduledTasks: (request) async {
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

    Future<int> send(Map<String, Object?> body, {bool granted = true}) async {
      final request = await browser.postUrl(
        gateway.origin!.replace(path: '/openmuse/scheduled-tasks/v1'),
      );
      if (granted) {
        request.cookies.add(Cookie('OpenMuse-Paired', connection.grantRef));
      }
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(body));
      final response = await request.close();
      await response.drain<void>();
      return response.statusCode;
    }

    expect(
      await send({'operation': 'list'}, granted: false),
      HttpStatus.unauthorized,
    );
    expect(await send({'operation': 'shell'}), HttpStatus.badRequest);
    expect(
      await send({
        'operation': 'create',
        'workspaceId': 'workspace-1',
        'message': 'hello',
        'dueAt': DateTime.utc(2026, 10, 15).toIso8601String(),
      }),
      HttpStatus.ok,
    );
    expect(calls.single.accountRef, 'account-1');
    expect(calls.single.workspaceId, 'workspace-1');
    account = 'account-2';
    expect(await send({'operation': 'list'}), HttpStatus.unauthorized);
    expect(calls, hasLength(1));

    browser.close(force: true);
    pair.close();
    await gateway.stop();
    await upstream.close(force: true);
  });
}
