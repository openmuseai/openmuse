import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'remote.dart';

/// Standalone transport for a scoped Host/acceptance runtime. Production paired
/// Hosts normally mount the same dispatcher behind PairedDesktopGateway.
final class ResourceRpcHttpHost {
  ResourceRpcHttpHost({required this.service, required this.authorize});
  final ResourceRpcService service;
  final Future<ResourcePeer?> Function(HttpRequest request) authorize;
  HttpServer? _server;
  int get port => _server!.port;
  Future<void> start({InternetAddress? address, int port = 0}) async {
    _server = await HttpServer.bind(
      address ?? InternetAddress.loopbackIPv4,
      port,
    );
    _server!.listen((request) => unawaited(_handle(request)));
  }

  Future<void> _handle(HttpRequest request) async {
    try {
      if (request.method != 'POST' ||
          request.uri.path != '/openmuse/resource-center/v1') {
        request.response.statusCode = 404;
        await request.response.close();
        return;
      }
      final peer = await authorize(request);
      if (peer == null) throw const ResourceFailure('DENIED');
      final bytes = <int>[];
      await for (final chunk in request.timeout(const Duration(seconds: 30))) {
        if (bytes.length + chunk.length > 24 * 1024 * 1024)
          throw const ResourceFailure('REQUEST_TOO_LARGE');
        bytes.addAll(chunk);
      }
      final body = jsonDecode(utf8.decode(bytes));
      if (body is! Map) throw const ResourceFailure('INVALID_ENVELOPE');
      final result = await service.dispatch(
        Map<String, Object?>.from(body),
        peer,
      );
      request.response.headers.contentType = ContentType.json;
      request.response.write(jsonEncode(result));
      await request.response.close();
    } catch (error) {
      request.response.headers.contentType = ContentType.json;
      request.response.statusCode =
          error is ResourceFailure && error.code == 'DENIED' ? 403 : 400;
      request.response.write(
        jsonEncode({
          'error': error is ResourceFailure ? error.code : 'INVALID_REQUEST',
        }),
      );
      await request.response.close();
    }
  }

  Future<void> close() async {
    await _server?.close(force: true);
    _server = null;
  }
}
