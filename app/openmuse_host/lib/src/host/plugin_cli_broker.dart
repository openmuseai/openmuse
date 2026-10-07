import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'plugin_command_registry.dart';

/// Narrow loopback execution seam for DSH. The agent can invoke only a CLI
/// contribution in a verified installed package; it cannot submit a shell.
final class PluginCliBroker {
  PluginCliBroker({required this.installRoot});

  final Directory installRoot;
  final String token = _newToken();
  HttpServer? _server;

  Uri get origin {
    final server = _server;
    if (server == null) throw StateError('CLI broker is not running');
    return Uri(scheme: 'http', host: '127.0.0.1', port: server.port);
  }

  Future<void> start() async {
    if (_server != null) return;
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    _server = server;
    unawaited(_serve(server));
  }

  Future<void> close() async {
    final server = _server;
    _server = null;
    await server?.close(force: true);
  }

  Future<void> _serve(HttpServer server) async {
    await for (final request in server) {
      unawaited(_handle(request));
    }
  }

  Future<void> _handle(HttpRequest request) async {
    final response = request.response;
    try {
      if (request.method != 'POST' || request.uri.path != '/v1/execute') {
        response.statusCode = HttpStatus.notFound;
        await response.close();
        return;
      }
      if (request.headers.value(HttpHeaders.authorizationHeader) !=
          'Bearer $token') {
        response.statusCode = HttpStatus.unauthorized;
        await response.close();
        return;
      }
      final bytes = await request.fold<List<int>>(<int>[], (collected, chunk) {
        if (collected.length + chunk.length > 16384) {
          throw const FormatException('CLI request too large');
        }
        return collected..addAll(chunk);
      });
      final decoded = jsonDecode(utf8.decode(bytes));
      if (decoded is! Map ||
          decoded['protocol'] != 'openmuse.cli-broker/v1' ||
          decoded['identity'] is! String ||
          decoded['args'] is! List ||
          (decoded['args'] as List).any((value) => value is! String)) {
        throw const FormatException('Invalid CLI request');
      }
      final identity = decoded['identity'] as String;
      if (!RegExp(
        r'^[a-z0-9][a-z0-9-]*/[a-z0-9][a-z0-9-]*/[a-z0-9][a-z0-9-]*$',
      ).hasMatch(identity)) {
        throw const FormatException('Invalid command identity');
      }
      final discovery = discoverInstalledCliCommands(installRoot);
      final command = discovery.commands[identity];
      if (command == null) {
        response.statusCode = HttpStatus.notFound;
        await response.close();
        return;
      }
      final args = (decoded['args'] as List).cast<String>();
      // Validate against the signed contribution before starting the stream.
      command.argv(args);
      response.headers.contentType = ContentType('application', 'x-ndjson');
      response.headers.set(HttpHeaders.cacheControlHeader, 'no-store');
      response.bufferOutput = false;
      void frame(String stream, List<int> data) {
        response.write(
          jsonEncode({
            'protocol': 'openmuse.cli-stream/v1',
            'stream': stream,
            'data': base64Encode(data),
          }),
        );
        response.write('\n');
      }

      try {
        final exitCode = await invokeInstalledCliCommand(
          command,
          args,
          onStdout: (bytes) => frame('stdout', bytes),
          onStderr: (bytes) => frame('stderr', bytes),
        );
        response.write(
          jsonEncode({
            'protocol': 'openmuse.cli-stream/v1',
            'stream': 'exit',
            'exitCode': exitCode,
          }),
        );
        response.write('\n');
      } catch (_) {
        response.write(
          jsonEncode({
            'protocol': 'openmuse.cli-stream/v1',
            'stream': 'exit',
            'exitCode': 1,
          }),
        );
        response.write('\n');
      }
      await response.close();
    } catch (_) {
      try {
        response.statusCode = HttpStatus.badRequest;
      } on StateError {
        // The streaming response has already started.
      }
      try {
        await response.close();
      } catch (_) {}
    }
  }

  static String _newToken() {
    final random = Random.secure();
    return base64UrlEncode(List.generate(32, (_) => random.nextInt(256)));
  }
}

Future<int> invokeBrokeredCliCommand({
  required Uri origin,
  required String token,
  required String identity,
  required List<String> args,
}) async {
  final client = HttpClient()
    ..connectionTimeout = const Duration(seconds: 5)
    ..findProxy = (_) => 'DIRECT';
  try {
    final request = await client.postUrl(origin.replace(path: '/v1/execute'));
    request.headers.set(HttpHeaders.authorizationHeader, 'Bearer $token');
    request.headers.contentType = ContentType.json;
    request.write(
      jsonEncode({
        'protocol': 'openmuse.cli-broker/v1',
        'identity': identity,
        'args': args,
      }),
    );
    final response = await request.close();
    if (response.statusCode != HttpStatus.ok) {
      await response.drain<void>();
      throw StateError('CLI broker rejected command (${response.statusCode})');
    }
    int? exitCode;
    await for (final line
        in response.transform(utf8.decoder).transform(const LineSplitter())) {
      final frame = jsonDecode(line);
      if (frame is! Map || frame['protocol'] != 'openmuse.cli-stream/v1') {
        throw const FormatException('Invalid CLI broker stream');
      }
      switch (frame['stream']) {
        case 'stdout':
          stdout.add(base64Decode(frame['data'] as String));
        case 'stderr':
          stderr.add(base64Decode(frame['data'] as String));
        case 'exit':
          exitCode = frame['exitCode'] as int;
        default:
          throw const FormatException('Unknown CLI stream frame');
      }
    }
    if (exitCode == null) throw const FormatException('Missing CLI exit frame');
    return exitCode;
  } finally {
    client.close(force: true);
  }
}
