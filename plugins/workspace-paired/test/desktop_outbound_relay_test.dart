import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'outbound relay resumes after token refresh failure and re-login',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final attached = Completer<void>();
      WebSocket? peer;
      final serving = server.listen((request) async {
        expect(
          request.headers.value('authorization'),
          'Bearer restored-access',
        );
        final socket = await WebSocketTransformer.upgrade(request);
        peer = socket;
        socket.listen((data) {
          final hello = jsonDecode(data as String) as Map;
          expect(hello['deviceId'], 'desktop-test');
          socket.add(jsonEncode({'type': 'hello-ok'}));
        });
      });
      var tokenRequests = 0;
      final relay = DesktopOutboundRelay(
        attachUri: Uri.parse('ws://127.0.0.1:${server.port}/attach'),
        accessToken: () async {
          tokenRequests++;
          if (tokenRequests == 1)
            throw StateError('refresh temporarily failed');
          if (tokenRequests == 2) return null;
          return 'restored-access';
        },
        deviceId: 'desktop-test',
        localGateway: () => Uri.parse('http://127.0.0.1:13180'),
        publicOrigin: Uri.parse('https://relay.example.test'),
        random: Random(1),
      );
      relay.onChanged = () {
        if (relay.phase == DesktopRelayPhase.attached &&
            !attached.isCompleted) {
          attached.complete();
        }
      };
      try {
        await relay.start();
        await attached.future.timeout(const Duration(seconds: 8));
        expect(tokenRequests, 3);
        expect(relay.phase, DesktopRelayPhase.attached);
      } finally {
        await relay.stop();
        await peer?.close();
        await serving.cancel();
        await server.close(force: true);
      }
    },
  );
}
