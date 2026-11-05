import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_appflowy_facets/muse_appflowy_facets.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_remote_session/facet_transport.dart';
import 'package:muse_remote_session/parent_bridge_adapter.dart';

Map<String, dynamic> fixture() {
  var dir = Directory.current;
  while (true) {
    final file = File(
      '${dir.path}/middlewares/dsh/plugins/dsh-appflowy/fixtures/presentation-v1.json',
    );
    if (file.existsSync()) {
      return jsonDecode(file.readAsStringSync()) as Map<String, dynamic>;
    }
    if (dir.parent.path == dir.path) {
      throw StateError('Shared fixture not found');
    }
    dir = dir.parent;
  }
}

void main() {
  final golden = fixture();
  MusePresentationIntentV1 intent([Map<String, Object?> extra = const {}]) =>
      MusePresentationIntentV1.fromJson(
        {...golden['intent'] as Map<String, dynamic>, ...extra},
      );

  test(
      'native facet uses shared Web/Host contract; awaits navigation and deduplicates',
      () async {
    final done = Completer<MusePresentationIntentStatus>();
    var calls = 0;
    final inbox =
        MuseIntentInbox(scopeRef: 'workspace.ws-1', clock: () => 2000);
    MuseWorkspaceUiFacet(
      workspaceId: 'ws-1',
      title: 'Docs',
      instanceRef: 'fixture',
      clock: () => 2000,
      navigate: (view, block) {
        expect(view, 'view-1');
        calls++;
        return done.future;
      },
    ).register(inbox);
    final first = inbox.dispatch(intent());
    final repeated = inbox.dispatch(intent());
    expect(calls, 1);
    var completed = false;
    unawaited(first.then((_) => completed = true));
    await Future<void>.delayed(Duration.zero);
    expect(completed, false);
    done.complete(MusePresentationIntentStatus.applied);
    expect((await first).status, MusePresentationIntentStatus.applied);
    expect((await repeated).toJson(), (await first).toJson());
    inbox.close();
  });

  test('wrong plugin, digest, workspace, stale and reused IDs never navigate',
      () async {
    var calls = 0;
    final inbox =
        MuseIntentInbox(scopeRef: 'workspace.ws-1', clock: () => 2000);
    MuseWorkspaceUiFacet(
      workspaceId: 'ws-1',
      title: 'Docs',
      instanceRef: 'fixture',
      navigate: (_, __) async {
        calls++;
        return MusePresentationIntentStatus.applied;
      },
    ).register(inbox);
    for (final change in <Map<String, Object?>>[
      {'pluginId': 'foreign.plugin'},
      {'intentSchemaDigest': 'sha256:${'0' * 64}'},
      {'scopeRef': 'workspace.other'},
      {'expiresAt': 1000},
    ]) {
      final result = await inbox.dispatch(
        intent({...change, 'intentRef': 'intent.${change.keys.first}'}),
      );
      expect(result.status, isNot(MusePresentationIntentStatus.applied));
    }
    expect(calls, 0);
    await inbox.dispatch(intent());
    final reused = await inbox.dispatch(
      intent({
        'payload': {'viewId': 'another'},
      }),
    );
    expect(reused.reasonCode, 'INTENT_ID_REUSED');
    expect(calls, 1);
    inbox.close();
    expect(
      (await inbox.dispatch(intent())).status,
      MusePresentationIntentStatus.surfaceClosed,
    );
  });

  test(
      'HTTP compatibility adapter preserves facet envelopes and handles real ready event',
      () async {
    final transport = _FakeTransport();
    final inbox =
        MuseIntentInbox(scopeRef: 'workspace.ws-1', clock: () => 2000);
    MuseWorkspaceUiFacet(
      workspaceId: 'ws-1',
      title: 'Docs',
      instanceRef: 'fixture',
      clock: () => 2000,
      navigate: (_, __) async => MusePresentationIntentStatus.applied,
    ).register(inbox);
    final adapter = MuseParentBridgeAdapter(
      transport: transport,
      inbox: inbox,
      workspaceId: 'ws-1',
      workspaceTitle: 'Docs',
      deviceId: 'device.1',
    );
    await adapter.connect();
    await adapter
        .contribute(MuseContextContributionV1.fromJson(golden['context']));
    expect(transport.sent.last['envelope'], golden['context']);
    transport.controller.add({
      'source': 'muse.dsh-web',
      'type': 'intent.dispatch',
      'intent': golden['intent'],
    });
    await Future<void>.delayed(Duration.zero);
    expect(transport.sent.last['type'], 'intent.receipt');
    expect((transport.sent.last['result'] as Map)['status'], 'applied');
    expect(jsonEncode(transport.sent), isNot(contains('deviceToken')));
    await adapter.close();
    expect(transport.closed, true);
  });

  test(
      'SSE handles fragmented UTF8, CRLF, comments, multiline JSON and truncation',
      () async {
    final bytes = utf8.encode(
      ': heartbeat\r\ndata: {"text":\r\ndata: "你好😀"}\r\n\r\ndata: {"incomplete":',
    );
    final events =
        await decodeMuseSse(Stream.fromIterable(bytes.map((byte) => [byte])))
            .toList();
    expect(events, [
      {'text': '你好😀'},
    ]);
    await expectLater(
      decodeMuseSse(Stream.value(utf8.encode('data: ${'x' * 32769}'))).toList(),
      throwsA(isA<MuseTransportException>()),
    );
  });

  test('HTTP transport sends auth headers, refuses redirects, and closes SSE',
      () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final requests = <HttpRequest>[];
    server.listen((request) async {
      requests.add(request);
      if (request.uri.path.endsWith('/redirect')) {
        request.response.statusCode = 302;
        request.response.headers.set('location', 'https://foreign.invalid/');
      } else if (request.uri.path.endsWith('/capabilities')) {
        request.response.headers.contentType = ContentType.html;
        request.response.write('<html>legacy SPA fallback</html>');
      } else {
        request.response.headers.contentType = ContentType.json;
        request.response.write('{"ok":true}');
      }
      await request.response.close();
    });
    final transport = MuseHttpSseTransport(
      endpoint: Uri.parse('http://127.0.0.1:${server.port}/bridge'),
      allowLoopbackForTests: true,
      headers: {'Authorization': 'Bearer fixture'},
    );
    try {
      expect((await transport.send({'hello': 'test'}))['ok'], true);
      expect(requests.single.headers.value('authorization'), 'Bearer fixture');
      await expectLater(
        transport.get('redirect'),
        throwsA(isA<MuseTransportException>()),
      );
      expect(requests.length, 2);
      final adapter = MuseParentBridgeAdapter(
        transport: transport,
        inbox: MuseIntentInbox(scopeRef: 'workspace.ws-1'),
        workspaceId: 'ws-1',
        workspaceTitle: 'Workspace',
        deviceId: 'fixture',
      );
      await expectLater(
        adapter.connect(),
        throwsA(
          isA<MuseTransportException>()
              .having((e) => e.code, 'code', 'HOST_UPGRADE_REQUIRED'),
        ),
      );
      await adapter.close();
      expect(
        () => MuseHttpSseTransport(
          endpoint: Uri.parse('http://public.invalid/'),
        ),
        throwsA(isA<MuseTransportException>()),
      );
    } finally {
      transport.close();
      await server.close(force: true);
    }
  });
}

class _FakeTransport implements MuseJsonTransport {
  final sent = <Map<String, Object?>>[];
  final controller = StreamController<Map<String, Object?>>();
  bool closed = false;
  @override
  Future<Map<String, Object?>> get(String suffix) async =>
      {'nativeHttpSse': true, 'mode': 'exclusive-test'};
  @override
  Future<Map<String, Object?>> send(Map<String, Object?> message) async {
    sent.add(message);
    return {'ok': true, if (message['type'] == 'parent-hello') 'bound': 'ws-1'};
  }

  @override
  Stream<Map<String, Object?>> events() {
    controller.add({
      'source': 'muse.dsh-web',
      'type': 'bridge.ready',
      'workspaceRef': 'ws-1',
    });
    return controller.stream;
  }

  @override
  void close() {
    closed = true;
    unawaited(controller.close());
  }
}
