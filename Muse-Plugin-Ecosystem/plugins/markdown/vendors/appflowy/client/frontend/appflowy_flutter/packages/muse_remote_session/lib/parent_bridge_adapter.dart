import 'dart:async';

import 'package:muse_plugin_facets/muse_plugin_facets.dart';

import 'facet_transport.dart';

/// Compatibility codec for the deployed v1 carrier. Business payloads remain
/// the existing Facet envelopes; this is NOT a new Muse Bridge wire version.
final class MuseParentBridgeAdapter {
  MuseParentBridgeAdapter({
    required this.transport,
    required this.inbox,
    required this.workspaceId,
    required this.workspaceTitle,
    required this.deviceId,
  });
  final MuseJsonTransport transport;
  final MuseIntentInbox inbox;
  final String workspaceId;
  final String workspaceTitle;
  final String deviceId;
  StreamSubscription<Map<String, Object?>>? _subscription;
  Timer? _heartbeat;
  final _ready = Completer<void>();
  bool _closed = false;
  bool _bound = false;
  Future<void> _outbound = Future.value();
  int _pending = 0;
  void Function(Object error)? onDisconnected;

  Future<void> connect() async {
    final Map<String, Object?> capabilities;
    try {
      capabilities = await transport.get('capabilities');
    } on MuseTransportException catch (error) {
      // Legacy SPA hosts return HTML with HTTP 200 for unknown API routes.
      if (error.code == 'INVALID_RESPONSE' || error.status == 404) {
        throw const MuseTransportException('HOST_UPGRADE_REQUIRED');
      }
      rethrow;
    }
    if (capabilities['nativeHttpSse'] != true ||
        capabilities['mode'] != 'exclusive-test') {
      throw const MuseTransportException('HOST_UPGRADE_REQUIRED');
    }
    if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
    final bound = await _send({
      'type': 'parent-hello',
      'workspaceRef': workspaceId,
      'workspaceTitle': workspaceTitle,
      'deviceId': deviceId,
    });
    if (bound['bound'] != workspaceId) {
      throw const MuseTransportException('WORKSPACE_NOT_BOUND');
    }
    _bound = true;
    if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
    _subscription = transport.events().listen(
          _receive,
          onError: _lost,
          onDone: () => _lost(const MuseTransportException('STREAM_CLOSED')),
        );
    await _ready.future.timeout(const Duration(seconds: 12));
    if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
    _heartbeat = Timer.periodic(const Duration(seconds: 20), (_) async {
      try {
        await _send({'type': 'peer.ping'});
      } catch (error) {
        _lost(error);
      }
    });
  }

  void _receive(Map<String, Object?> message) {
    if (_closed || message['source'] != 'muse.dsh-web') return;
    if (message['type'] == 'bridge.ready' &&
        message['workspaceRef'] == workspaceId) {
      if (!_ready.isCompleted) _ready.complete();
      return;
    }
    if (message['type'] == 'intent.dispatch') {
      unawaited(_intent(message['intent']));
    }
  }

  Future<void> _intent(Object? raw) async {
    try {
      validateMuseFacetValue(MuseFacetSchemaKind.presentationIntent, raw);
      final intent = MusePresentationIntentV1.fromJson(
        (raw as Map).cast<String, Object?>(),
      );
      final receipt = await inbox.dispatch(intent);
      if (!_closed) {
        await _send({'type': 'intent.receipt', 'result': receipt.toJson()});
      }
    } catch (error) {
      _lost(error);
    }
  }

  Future<void> contribute(MuseContextContributionV1 envelope) async {
    validateMuseFacetValue(
      MuseFacetSchemaKind.contextContribution,
      envelope.toJson(),
    );
    await _send({'type': 'context.contribute', 'envelope': envelope.toJson()});
  }

  Future<void> closeSurface(String ref) async =>
      _send({'type': 'surface.closed', 'surfaceInstanceRef': ref});

  Future<Map<String, Object?>> _send(Map<String, Object?> body) async {
    if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
    if (_pending >= 32) throw const MuseTransportException('BACKPRESSURE');
    _pending++;
    final next = _outbound.then((_) async {
      if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
      final result =
          await transport.send({'source': 'muse.appflowy-mobile', ...body});
      if (result['ok'] != true) {
        throw const MuseTransportException('HOST_REJECTED');
      }
      return result;
    });
    _outbound = next.then<void>((_) {}, onError: (Object _) {});
    try {
      return await next;
    } finally {
      _pending--;
    }
  }

  void _lost(Object error) {
    if (_closed) return;
    _heartbeat?.cancel();
    if (!_ready.isCompleted) _ready.completeError(error);
    onDisconnected?.call(error);
  }

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    _heartbeat?.cancel();
    inbox.close();
    await _subscription?.cancel();
    try {
      if (_bound) {
        await transport.send({
          'source': 'muse.appflowy-mobile',
          'type': 'peer.close',
        }).timeout(const Duration(seconds: 2));
      }
    } catch (_) {/* Host lease expiry handles abrupt disconnects. */}
    transport.close();
  }
}
