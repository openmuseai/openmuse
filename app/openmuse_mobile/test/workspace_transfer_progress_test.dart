import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_workspace_mirror_client.dart';

void main() {
  Future<void> withReader(
    Future<void> Function(HttpRequest) serve,
    Future<void> Function(MobileWorkspaceMirrorClient) verify, {
    Duration idle = const Duration(seconds: 2),
  }) async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    server.listen((r) async {
      try {
        await serve(r);
      } on Object {
        /* expected client abort */
      }
    });
    final connection = PairedDesktopConnection.fromJson({
      'accountRef': 'account',
      'deviceRef': 'desktop',
      'workspaceRef': 'workspace',
      'workspaceTitle': 'Workspace',
      'grantRef': 'private-grant',
      'expiresAtMs': DateTime.now().millisecondsSinceEpoch + 60000,
      'session': {
        'sessionRef': 's',
        'origin': 'http://127.0.0.1:${server.port}',
        'path': '/session/test',
        'generation': 1,
        'allowInsecureLoopback': true,
      },
    });
    final reader = MobileWorkspaceMirrorClient(connection, idleTimeout: idle);
    try {
      await verify(reader);
    } finally {
      reader.close();
      await server.close(force: true);
    }
  }

  test('progress arrives before completion and measures real bytes', () async {
    final first = Completer<void>(), release = Completer<void>();
    await withReader(
      (r) async {
        r.response.bufferOutput = false;
        r.response.contentLength = 6;
        r.response.add(utf8.encode('abc'));
        await r.response.flush();
        await release.future;
        r.response.add(utf8.encode('def'));
        await r.response.close();
      },
      (reader) async {
        final received = <int>[];
        final read = reader.readResource(
          'opaque-ref',
          onProgress: (step, n, total) {
            if (step == '接收 Desktop 文件') {
              received.add(n);
              expect(total, 6);
              if (n == 3 && !first.isCompleted) first.complete();
            }
          },
        );
        await first.future.timeout(const Duration(seconds: 1));
        expect(received.last, 3);
        release.complete();
        expect(utf8.decode((await read)['bytes'] as List<int>), 'abcdef');
        expect(received.last, 6);
      },
    );
  });
  test('expired authorization has actionable error', () async {
    await withReader(
      (r) async {
        r.response.statusCode = 401;
        await r.response.close();
      },
      (reader) async {
        await expectLater(
          reader.readResource('ref'),
          throwsA(
            isA<StateError>().having(
              (e) => e.message,
              'message',
              contains('授权已失效'),
            ),
          ),
        );
      },
    );
  });
  test('idle timeout identifies waiting stage', () async {
    await withReader(
      (r) async {
        await Future<void>.delayed(const Duration(milliseconds: 300));
        await r.response.close();
      },
      (reader) async {
        await expectLater(
          reader.readResource('ref'),
          throwsA(
            isA<StateError>().having(
              (e) => e.message,
              'message',
              contains('等待 Desktop 读取文件 超时'),
            ),
          ),
        );
      },
      idle: const Duration(milliseconds: 60),
    );
  });
}
