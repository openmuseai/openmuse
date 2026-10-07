import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'host.dart';
import 'provider.dart';

final class RemoteSurfaceTimeout implements Exception {
  const RemoteSurfaceTimeout();
}

abstract interface class RemoteSurfaceTransport {
  Future<List<RemoteSurfaceOffer>> discover({
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  });

  Future<RemoteOpenResult> open({
    required String pluginId,
    required String surfaceId,
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  });

  Future<RemoteControlReceipt> submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  });

  Future<RemoteControlReceipt?> lookup({
    required String surfaceSessionRef,
    required int generation,
    String? requestId,
    String? idempotencyKey,
  });

  Future<RemoteSurfaceSnapshot> snapshot({
    required String surfaceSessionRef,
    required int generation,
  });

  Future<List<RemoteSurfaceEvent>> eventsAfter({
    required String surfaceSessionRef,
    required int generation,
    required int afterSeq,
  });
}

final class InMemoryRemoteSurfaceTransport implements RemoteSurfaceTransport {
  InMemoryRemoteSurfaceTransport(this.host);

  final RemoteSurfaceHost host;

  @override
  Future<List<RemoteSurfaceOffer>> discover({
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) async => host.discover(hello: hello, context: context);

  @override
  Future<RemoteOpenResult> open({
    required String pluginId,
    required String surfaceId,
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) async => host.open(
    pluginId: pluginId,
    surfaceId: surfaceId,
    hello: hello,
    context: context,
  );

  @override
  Future<RemoteControlReceipt> submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  }) async => host.submit(request, context: context);

  @override
  Future<RemoteControlReceipt?> lookup({
    required String surfaceSessionRef,
    required int generation,
    String? requestId,
    String? idempotencyKey,
  }) async => host.lookup(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
    requestId: requestId,
    idempotencyKey: idempotencyKey,
  );

  @override
  Future<RemoteSurfaceSnapshot> snapshot({
    required String surfaceSessionRef,
    required int generation,
  }) async => host.readSnapshot(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
  );

  @override
  Future<List<RemoteSurfaceEvent>> eventsAfter({
    required String surfaceSessionRef,
    required int generation,
    required int afterSeq,
  }) async => host.eventsAfter(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
    afterSeq: afterSeq,
  );
}

/// Deterministic test double. Production Mobile does not install it.
final class RemoteSurfaceFaultTransport implements RemoteSurfaceTransport {
  RemoteSurfaceFaultTransport(this.inner);

  final RemoteSurfaceTransport inner;
  var dropSubmitResponse = false;
  var failBeforeSubmit = false;
  var submits = 0;

  @override
  Future<RemoteControlReceipt> submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  }) async {
    if (failBeforeSubmit) {
      failBeforeSubmit = false;
      throw const RemoteSurfaceTimeout();
    }
    submits += 1;
    final receipt = await inner.submit(request, context: context);
    if (dropSubmitResponse) {
      dropSubmitResponse = false;
      throw const RemoteSurfaceTimeout();
    }
    return receipt;
  }

  @override
  Future<List<RemoteSurfaceOffer>> discover({
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) => inner.discover(hello: hello, context: context);

  @override
  Future<RemoteOpenResult> open({
    required String pluginId,
    required String surfaceId,
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) => inner.open(
    pluginId: pluginId,
    surfaceId: surfaceId,
    hello: hello,
    context: context,
  );

  @override
  Future<RemoteControlReceipt?> lookup({
    required String surfaceSessionRef,
    required int generation,
    String? requestId,
    String? idempotencyKey,
  }) => inner.lookup(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
    requestId: requestId,
    idempotencyKey: idempotencyKey,
  );

  @override
  Future<RemoteSurfaceSnapshot> snapshot({
    required String surfaceSessionRef,
    required int generation,
  }) => inner.snapshot(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
  );

  @override
  Future<List<RemoteSurfaceEvent>> eventsAfter({
    required String surfaceSessionRef,
    required int generation,
    required int afterSeq,
  }) => inner.eventsAfter(
    surfaceSessionRef: surfaceSessionRef,
    generation: generation,
    afterSeq: afterSeq,
  );
}
