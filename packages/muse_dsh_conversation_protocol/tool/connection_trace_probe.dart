// Controlled local probe; not a real-device or WAN latency benchmark.
import 'dart:async';
import 'dart:io';
import 'dart:convert';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

Future<void> main() async {
  ConnectionTrace.sink = print;
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  var requests = 0;
  server.listen((request) async {
    requests++;
    await Future<void>.delayed(const Duration(milliseconds: 20));
    request.response.bufferOutput = false;
    request.response.headers.contentType = ContentType.json;
    request.response.write('{"items":');
    await request.response.flush();
    await Future<void>.delayed(const Duration(milliseconds: 30));
    request.response.write('[]}');
    await request.response.close();
  });
  DshNativeGatewayClient client(ConnectionTrace trace) =>
      DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
        trace: trace,
      );
  final trace = ConnectionTrace();
  final warmup = List.generate(5, (_) => client(trace));
  ConnectionTrace.sink = (_) {};
  await Future.wait(warmup.map((c) => c.listSessions()));
  for (final c in warmup) {
    c.close();
  }
  ConnectionTrace.sink = print;
  Future<void> run(bool shared) async {
    final clients = List.generate(shared ? 1 : 4, (_) => client(trace));
    final before = requests;
    final scenario = shared ? 'shared_inflight' : 'separate_clients';
    try {
      await trace.measure(
        'probe.$scenario',
        (_) => Future.wait(
          List.generate(4, (i) => clients[shared ? 0 : i].listSessions()),
        ),
      );
      print(
        'PROBE ${jsonEncode({'scenario': scenario, 'requests': requests - before, 'injectedHeaderMs': 20, 'injectedBodyMs': 30})}',
      );
    } finally {
      for (final c in clients) {
        c.close();
      }
    }
  }

  try {
    for (var i = 0; i < 10; i++) {
      await run(i.isEven);
      await run(i.isOdd);
    }
  } finally {
    await server.close(force: true);
    ConnectionTrace.sink = null;
  }
}
