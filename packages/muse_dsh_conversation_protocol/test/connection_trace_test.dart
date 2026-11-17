import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

void main() {
  test(
    'trace propagation, bounded storage and failing sinks are safe',
    () async {
      final trace = ConnectionTrace();
      final root = trace.start('test.root');
      final child = ConnectionTrace.fromTraceparent(
        root.traceparent,
      ).start('test.child');
      expect(child.owner.traceId, trace.traceId);
      expect(child.owner.parentSpanId, root.spanId);
      child.end();
      root.end();
      root.end();
      expect(
        ConnectionTrace.fromTraceparent(
          '00-${'0' * 32}-${'0' * 16}-01',
        ).traceId,
        isNot('0' * 32),
      );
      ConnectionTrace.sink = (_) => throw StateError('sink failed');
      addTearDown(() => ConnectionTrace.sink = null);
      for (var i = 0; i < 300; i++) {
        trace.start('test.bounded').end();
      }
      expect(ConnectionTrace.recent.length, 512);
      await expectLater(
        trace.measure<void>(
          'test.failure',
          (_) async => throw TimeoutException('secret'),
        ),
        throwsA(isA<TimeoutException>()),
      );
      expect(ConnectionTrace.recent.last['ok'], false);
      expect(jsonEncode(ConnectionTrace.recent), isNot(contains('secret')));
    },
  );

  test(
    'HTTP phases identify delayed headers/body and coalesce concurrent catalog reads',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final traces = <String>[];
      ConnectionTrace.sink = traces.add;
      addTearDown(() => ConnectionTrace.sink = null);
      var calls = 0;
      server.listen((request) async {
        calls++;
        expect(
          request.headers.value('traceparent'),
          matches(r'^00-[a-f0-9]{32}-[a-f0-9]{16}-01$'),
        );
        await Future<void>.delayed(const Duration(milliseconds: 20));
        request.response.bufferOutput = false;
        request.response.headers.contentType = ContentType.json;
        request.response.write('{"items":');
        await request.response.flush();
        await Future<void>.delayed(const Duration(milliseconds: 30));
        request.response.write('[]}');
        await request.response.close();
      });
      final client = DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
      );
      try {
        await Future.wait(List.generate(4, (_) => client.listSessions()));
        expect(calls, 1);
        await client.listSessions();
        expect(calls, 2); // No stale catalog TTL cache.
        final events = traces
            .map((line) => jsonDecode(line.substring(8)) as Map)
            .toList();
        expect(events.where((e) => e['phase'] == 'end').length, 2);
        final headers =
            events.firstWhere((e) => e['phase'] == 'headers')['elapsedUs']
                as int;
        final body =
            events.firstWhere((e) => e['phase'] == 'body')['elapsedUs'] as int;
        expect(body, greaterThan(headers));
        expect(traces.join(), isNot(contains('127.0.0.1')));
      } finally {
        client.close();
        await server.close(force: true);
      }
    },
  );
}
