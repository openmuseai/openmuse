import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'host.dart';
import 'provider.dart';
import 'transport.dart';

/// Talks to a paired Desktop gateway. The connection context stored on the
/// controller is not sent; the gateway builds it from the paired grant.
final class PairedRemoteSurfaceTransport implements RemoteSurfaceTransport {
  PairedRemoteSurfaceTransport({
    required this.origin,
    required this.grantRef,
    HttpClient? httpClient,
  }) : _httpClient = httpClient,
       _ownsClient = httpClient == null;

  final Uri origin;
  final String grantRef;
  final HttpClient? _httpClient;
  final bool _ownsClient;

  HttpClient _client() => _httpClient ?? HttpClient();

  void _finish(HttpClient client) {
    if (_ownsClient) client.close(force: true);
  }

  @override
  Future<List<RemoteSurfaceOffer>> discover({
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) async {
    final response = await _post('discover', {'hello': hello.toJson()});
    final offers = response['offers'];
    if (offers is! List) {
      throw const FormatException('remote surface offers are invalid');
    }
    return [
      for (final item in offers)
        if (item is Map)
          RemoteSurfaceOffer(
            descriptor: RemoteSurfaceDescriptor.fromJson(
              _map(item['descriptor']),
            ),
            compatible: item['compatible'] == true,
            mode: item['mode'] as String?,
            unsupportedReason: item['unsupportedReason'] as String?,
          ),
    ];
  }

  @override
  Future<RemoteOpenResult> open({
    required String pluginId,
    required String surfaceId,
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) async {
    final response = await _post('open', {
      'pluginId': pluginId,
      'surfaceId': surfaceId,
      'hello': hello.toJson(),
    });
    if (response['status'] == 'accepted') {
      return RemoteOpenAccepted(
        RemoteSurfaceSnapshot.fromJson(_map(response['snapshot'])),
      );
    }
    return RemoteOpenRejected(
      status: response['status']! as String,
      errorCode: response['errorCode']! as String,
    );
  }

  @override
  Future<RemoteControlReceipt> submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  }) async {
    final response = await _post('submit', {'request': request.toJson()});
    return RemoteControlReceipt.fromJson(response);
  }

  @override
  Future<RemoteControlReceipt?> lookup({
    required String surfaceSessionRef,
    required int generation,
    String? requestId,
    String? idempotencyKey,
  }) async {
    final response = await _post('lookup', {
      'surfaceSessionRef': surfaceSessionRef,
      'generation': generation,
      'requestId': ?requestId,
      'idempotencyKey': ?idempotencyKey,
    });
    final receipt = response['receipt'];
    if (receipt == null) return null;
    return RemoteControlReceipt.fromJson(_map(receipt));
  }

  @override
  Future<RemoteSurfaceSnapshot> snapshot({
    required String surfaceSessionRef,
    required int generation,
  }) async {
    final response = await _post('snapshot', {
      'surfaceSessionRef': surfaceSessionRef,
      'generation': generation,
    });
    return RemoteSurfaceSnapshot.fromJson(response);
  }

  @override
  Future<List<RemoteSurfaceEvent>> eventsAfter({
    required String surfaceSessionRef,
    required int generation,
    required int afterSeq,
  }) async {
    final response = await _post('events', {
      'surfaceSessionRef': surfaceSessionRef,
      'generation': generation,
      'afterSeq': afterSeq,
    });
    final events = response['events'];
    if (events is! List) {
      throw const FormatException('remote surface events are invalid');
    }
    return [
      for (final item in events)
        if (item is Map) RemoteSurfaceEvent.fromJson(_map(item)),
    ];
  }

  Future<Uint8List?> readMedia(String handle) async {
    final client = _client();
    try {
      final request = await client.getUrl(
        origin.replace(
          path: '/openmuse/remote-surface/media/v1/$handle',
          query: '',
        ),
      );
      request.cookies.add(Cookie('OpenMuse-Paired', grantRef));
      request.headers.set(HttpHeaders.rangeHeader, 'bytes=0-');
      final response = await request.close();
      if (response.statusCode == HttpStatus.notFound) {
        await response.drain<void>();
        return null;
      }
      if (response.statusCode != HttpStatus.partialContent &&
          response.statusCode != HttpStatus.ok) {
        throw StateError(
          'remote media failed with HTTP ${response.statusCode}',
        );
      }
      return Uint8List.fromList(
        await response.fold<List<int>>(
          <int>[],
          (bytes, chunk) => bytes..addAll(chunk),
        ),
      );
    } finally {
      _finish(client);
    }
  }

  Future<Map<String, Object?>> _post(
    String operation,
    Map<String, Object?> body,
  ) async {
    final client = _client();
    try {
      final request = await client.postUrl(
        origin.replace(path: '/openmuse/remote-surface/v1', query: ''),
      );
      request.cookies.add(Cookie('OpenMuse-Paired', grantRef));
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode({'operation': operation, 'body': body}));
      final response = await request.close();
      final text = await utf8.decodeStream(response);
      if (response.statusCode != HttpStatus.ok) {
        throw StateError(
          'remote surface $operation failed with HTTP ${response.statusCode}',
        );
      }
      final decoded = jsonDecode(text);
      final normalized = _normalize(decoded);
      return _map(normalized);
    } finally {
      _finish(client);
    }
  }

  Map<String, Object?> _map(Object? value) {
    final normalized = _normalize(value);
    if (normalized is! Map) {
      throw const FormatException('remote surface response is invalid');
    }
    return normalized.map((key, item) => MapEntry(key.toString(), item));
  }

  Object? _normalize(Object? value) {
    if (value is Map) {
      return <String, Object?>{
        for (final entry in value.entries)
          entry.key.toString(): _normalize(entry.value),
      };
    }
    if (value is List) {
      return [for (final item in value) _normalize(item)];
    }
    return value;
  }
}
