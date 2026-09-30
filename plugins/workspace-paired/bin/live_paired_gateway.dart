import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:openmuse_workspace_paired/src/paired_desktop_gateway.dart';

Future<void> main() async {
  final accountRef = _requiredEnvironment('OPENMUSE_DESKTOP_ACCOUNT_REF');
  final gotrueOrigin = Uri.parse(
    _requiredEnvironment('OPENMUSE_GOTRUE_ORIGIN'),
  );
  final dshLog = File(_requiredEnvironment('OPENMUSE_DSH_ENDPOINT_LOG'));
  final stopFile = File(_requiredEnvironment('OPENMUSE_GATEWAY_STOP_FILE'));
  final gateway = PairedDesktopGateway(
    currentAccountRef: () => accountRef,
    validateToken: (token) =>
        _validateGoTrueToken(origin: gotrueOrigin, accessToken: token),
    dshEndpoint: () => _readDshEndpoint(dshLog),
    workspaceRef: 'openmuse.local.default',
    workspaceTitle: 'Project Workspace',
    port: 13180,
    bindAddress: Platform.environment['OPENMUSE_GATEWAY_BIND_ALL'] == '1'
        ? InternetAddress.anyIPv4
        : InternetAddress.loopbackIPv4,
    fixedPairingCode: _requiredEnvironment(
      'OPENMUSE_PAIRED_DESKTOP_PAIRING_CODE',
    ),
  );
  await gateway.start();
  stdout.writeln('PAIRED_DESKTOP_GATEWAY_READY');
  while (!stopFile.existsSync()) {
    await Future<void>.delayed(const Duration(seconds: 1));
  }
  await gateway.stop();
}

String _requiredEnvironment(String name) {
  final value = Platform.environment[name];
  if (value == null || value.isEmpty) throw StateError('$name is required');
  return value;
}

Future<Uri> _readDshEndpoint(File log) async {
  final deadline = DateTime.now().add(const Duration(seconds: 30));
  while (DateTime.now().isBefore(deadline)) {
    if (await log.exists()) {
      final contents = await log.readAsString();
      final match = RegExp(
        r'https?://127\.0\.0\.1:[0-9]+(?:/\?[^\s]+)?',
      ).firstMatch(contents);
      if (match != null) return Uri.parse(match.group(0)!);
    }
    await Future<void>.delayed(const Duration(milliseconds: 200));
  }
  throw TimeoutException('DSH did not publish its local endpoint');
}

Future<String> _validateGoTrueToken({
  required Uri origin,
  required String accessToken,
}) async {
  final client = HttpClient();
  try {
    final request = await client.getUrl(origin.resolve('/user'));
    request.headers.set(HttpHeaders.authorizationHeader, 'Bearer $accessToken');
    final response = await request.close();
    final body = await utf8.decodeStream(response);
    if (response.statusCode != HttpStatus.ok) {
      throw const HttpException('GoTrue token rejected');
    }
    final decoded = jsonDecode(body);
    if (decoded is! Map || decoded['id'] is! String) {
      throw const FormatException('invalid GoTrue user');
    }
    return decoded['id'] as String;
  } finally {
    client.close(force: true);
  }
}
