import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'host.dart';
import 'provider.dart';

/// Serves one paired remote-surface operation.
///
/// [context] comes from the paired grant. Fields in [body] never override the
/// actor, device, or workspace.
Future<Map<String, Object?>> dispatchRemoteSurface({
  required RemoteSurfaceHost host,
  required String operation,
  required Map<String, Object?> body,
  required RemoteConnectionContext context,
}) async {
  final payload = _jsonMap(body);
  return switch (operation) {
    'discover' => {
      'offers': host
          .discover(
            hello: RemoteClientHello.fromJson(_object(payload, 'hello')),
            context: context,
          )
          .map(_offerJson)
          .toList(),
    },
    'open' => _openJson(
      host.open(
        pluginId: _string(payload, 'pluginId'),
        surfaceId: _string(payload, 'surfaceId'),
        hello: RemoteClientHello.fromJson(_object(payload, 'hello')),
        context: context,
      ),
    ),
    'submit' =>
      host
          .submit(
            RemoteControlRequest.fromJson(_object(payload, 'request')),
            context: context,
          )
          .toJson(),
    'lookup' => {
      'receipt': host
          .lookup(
            surfaceSessionRef: _string(payload, 'surfaceSessionRef'),
            generation: _int(payload, 'generation'),
            requestId: _optionalString(payload, 'requestId'),
            idempotencyKey: _optionalString(payload, 'idempotencyKey'),
          )
          ?.toJson(),
    },
    'snapshot' =>
      host
          .readSnapshot(
            surfaceSessionRef: _string(payload, 'surfaceSessionRef'),
            generation: _int(payload, 'generation'),
          )
          .toJson(),
    'events' => {
      'events': host
          .eventsAfter(
            surfaceSessionRef: _string(payload, 'surfaceSessionRef'),
            generation: _int(payload, 'generation'),
            afterSeq: _int(payload, 'afterSeq'),
          )
          .map((event) => event.toJson())
          .toList(),
    },
    _ => throw const FormatException('remote surface operation is unsupported'),
  };
}

Map<String, Object?> _offerJson(RemoteSurfaceOffer offer) => {
  'descriptor': offer.descriptor.toJson(),
  'compatible': offer.compatible,
  if (offer.mode != null) 'mode': offer.mode,
  if (offer.unsupportedReason != null)
    'unsupportedReason': offer.unsupportedReason,
};

Map<String, Object?> _openJson(RemoteOpenResult result) {
  if (result is RemoteOpenAccepted) {
    return {'status': 'accepted', 'snapshot': result.snapshot.toJson()};
  }
  final rejected = result as RemoteOpenRejected;
  return {'status': rejected.status, 'errorCode': rejected.errorCode};
}

Map<String, Object?> _jsonMap(Map<String, Object?> body) {
  final normalized = _normalize(body);
  if (normalized is! Map) {
    throw const FormatException('remote surface body is invalid');
  }
  return normalized.map((key, value) => MapEntry(key.toString(), value));
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

Map<String, Object?> _object(Map<String, Object?> body, String key) {
  final value = body[key];
  if (value is! Map) {
    throw FormatException('remote surface $key is invalid');
  }
  return value.map((itemKey, item) => MapEntry(itemKey.toString(), item));
}

String _string(Map<String, Object?> body, String key) {
  final value = body[key];
  if (value is! String || value.isEmpty) {
    throw FormatException('remote surface $key is invalid');
  }
  return value;
}

String? _optionalString(Map<String, Object?> body, String key) {
  if (!body.containsKey(key) || body[key] == null) return null;
  return _string(body, key);
}

int _int(Map<String, Object?> body, String key) {
  final value = body[key];
  if (value is! int) throw FormatException('remote surface $key is invalid');
  return value;
}
