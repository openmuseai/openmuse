import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'audit.dart';
import 'provider.dart';

final class RemoteSurfaceOffer {
  const RemoteSurfaceOffer({
    required this.descriptor,
    required this.compatible,
    this.unsupportedReason,
    this.mode,
  });

  final RemoteSurfaceDescriptor descriptor;
  final bool compatible;
  final String? unsupportedReason;
  final String? mode;
}

sealed class RemoteOpenResult {
  const RemoteOpenResult();
}

final class RemoteOpenAccepted extends RemoteOpenResult {
  RemoteOpenAccepted(this.snapshot);

  final RemoteSurfaceSnapshot snapshot;
}

final class RemoteOpenRejected extends RemoteOpenResult {
  const RemoteOpenRejected({required this.status, required this.errorCode});

  final String status;
  final String errorCode;
}

final class RemoteSurfaceClosed implements Exception {
  const RemoteSurfaceClosed(this.code);

  final String code;

  @override
  String toString() => 'RemoteSurfaceClosed: $code';
}

final class RemoteSurfaceHost {
  RemoteSurfaceHost({
    DateTime Function()? clock,
    RemoteAuditLog? audit,
    RemoteJobLedger? jobs,
  }) : _clock = clock ?? DateTime.now,
       _audit = audit, // ignore: prefer_initializing_formals
       _jobs = jobs; // ignore: prefer_initializing_formals

  final DateTime Function() _clock;
  final RemoteAuditLog? _audit;
  final RemoteJobLedger? _jobs;

  Iterable<String> get registeredPluginIds => _providers.keys;
  final _providers = <String, RemoteSurfaceProvider>{};
  final _epochs = <String, int>{};
  final _sessions = <String, _Session>{};
  var _hostEpoch = 1;
  var _sessionSerial = 0;
  var _decisionSerial = 0;

  void register(RemoteSurfaceProvider provider) {
    final id = provider.pluginId;
    if (_providers.containsKey(id)) {
      throw StateError('Plugin already registered: $id');
    }
    _providers[id] = provider;
    _epochs.putIfAbsent(id, () => 1);
  }

  void unregister(String pluginId) {
    _providers.remove(pluginId);
    _epochs[pluginId] = (_epochs[pluginId] ?? 0) + 1;
  }

  void invalidateAll() {
    _hostEpoch += 1;
    for (final id in _epochs.keys.toList()) {
      _epochs[id] = _epochs[id]! + 1;
    }
  }

  List<RemoteSurfaceOffer> discover({
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) {
    final offers = <RemoteSurfaceOffer>[];
    for (final provider in _providers.values) {
      for (final descriptor in provider.descriptors(context)) {
        if (!_visible(descriptor, context)) continue;
        offers.add(_offer(provider, descriptor, hello));
      }
    }
    return List<RemoteSurfaceOffer>.unmodifiable(offers);
  }

  RemoteOpenResult open({
    required String pluginId,
    required String surfaceId,
    required RemoteClientHello hello,
    required RemoteConnectionContext context,
  }) {
    final provider = _providers[pluginId];
    if (provider == null) {
      return const RemoteOpenRejected(
        status: 'unsupported',
        errorCode: 'SESSION_CLOSED',
      );
    }
    final descriptor = provider
        .descriptors(context)
        .where((item) => item.surfaceId == surfaceId)
        .firstOrNull;
    if (descriptor == null || !_visible(descriptor, context)) {
      return const RemoteOpenRejected(status: 'denied', errorCode: 'DENIED');
    }
    final offer = _offer(provider, descriptor, hello);
    if (!offer.compatible || offer.mode == null) {
      return RemoteOpenRejected(
        status: 'unsupported',
        errorCode: offer.unsupportedReason ?? 'UNSUPPORTED_MODE',
      );
    }
    final sealed = _seal(
      provider,
      descriptor,
      hello,
      offer.mode!,
      'sess.${++_sessionSerial}',
      provider.initialRevision(surfaceId),
      provider.initialNodes(surfaceId),
    );
    if (sealed is RemoteOpenRejected) return sealed;
    final snapshot = (sealed as RemoteOpenAccepted).snapshot;
    _sessions[snapshot.surfaceSessionRef] = _Session(
      pluginId: pluginId,
      surfaceId: surfaceId,
      generation: _epochs[pluginId]!,
      pluginEpoch: _epochs[pluginId]!,
      hostEpoch: _hostEpoch,
      descriptor: descriptor,
      components: hello.components,
      stateRevision: snapshot.stateRevision,
      nodes: snapshot.nodes,
      mode: snapshot.mode,
    );
    return sealed;
  }

  RemoteControlReceipt submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  }) {
    final receipt = _submit(request, context: context);
    _audit?.record(
      RemoteAuditEntry(
        atMs: _clock().millisecondsSinceEpoch,
        actorRef: context.actorRef,
        mobileDeviceRef: context.mobileDeviceRef,
        desktopDeviceRef: context.desktopDeviceRef,
        workspaceRef: context.workspaceRef,
        actionId: request.actionId,
        status: receipt.status,
        errorCode: receipt.errorCode,
        decisionRef: receipt.decisionRef,
        idempotencyKey: request.idempotencyKey,
      ),
    );
    return receipt;
  }

  RemoteControlReceipt _submit(
    RemoteControlRequest request, {
    required RemoteConnectionContext context,
  }) {
    final session = _sessions[request.surfaceSessionRef];
    if (session == null) {
      return _denied(request, 'SESSION_CLOSED');
    }
    final stale = _stale(session);
    if (stale != null) {
      return _remember(session, request, _unsupported(request, stale));
    }
    final existing = session.ledger[request.idempotencyKey];
    if (existing != null) {
      if (existing.actionId != request.actionId ||
          existing.canonical != canonicalJson(request.input)) {
        return _conflict(request, 'IDEMPOTENCY_CONFLICT');
      }
      return existing.receipt;
    }
    if (session.descriptor.workspaceRef != context.workspaceRef) {
      return _remember(session, request, _denied(request, 'DENIED'));
    }
    final action = session.descriptor.action(request.actionId);
    if (action == null) {
      return _remember(
        session,
        request,
        _unsupported(request, 'UNKNOWN_ACTION'),
      );
    }
    if (!context.permissions.containsAll(action.requiredPermissions)) {
      return _remember(session, request, _denied(request, 'DENIED'));
    }
    if (!inputMatchesSchema(action.inputSchema, request.input)) {
      return _remember(session, request, _denied(request, 'SCHEMA_REJECTED'));
    }
    if (action.effect == 'external-side-effect') {
      final stored = _jobs?.find(request.idempotencyKey);
      if (stored != null) {
        if (stored.actionId != request.actionId ||
            stored.canonical != canonicalJson(request.input)) {
          return _conflict(request, 'IDEMPOTENCY_CONFLICT');
        }
        return stored.receipt;
      }
    }
    if (request.expectedStateRevision != session.stateRevision) {
      return _remember(session, request, _conflict(request, 'STATE_CONFLICT'));
    }
    final provider = _providers[session.pluginId];
    if (provider == null) {
      return _remember(
        session,
        request,
        _unsupported(request, 'STALE_GENERATION'),
      );
    }
    final RemoteProviderResult result;
    try {
      result = provider.execute(
        RemoteProviderRequest(
          surfaceId: session.surfaceId,
          actionId: action.id,
          effect: action.effect,
          input: request.input,
          stateRevision: session.stateRevision,
        ),
      );
    } catch (_) {
      return _remember(session, request, _denied(request, 'PROVIDER_FAILED'));
    }
    if (result is RemoteProviderReject) {
      return _remember(
        session,
        request,
        _denied(request, _safeCode(result.errorCode)),
      );
    }
    if (result is! RemoteProviderUpdate) {
      return _remember(session, request, _denied(request, 'PROVIDER_FAILED'));
    }
    if (result.eventState != null &&
        !remoteEventStates.contains(result.eventState)) {
      return _remember(session, request, _denied(request, 'PROVIDER_FAILED'));
    }
    final sealed = _seal(
      provider,
      session.descriptor,
      RemoteClientHello(
        protocolMajor: 1,
        protocolMinor: 0,
        components: session.components,
        mediaFormats: const ['image/jpeg'],
        webSnapshot: true,
        webInteractive: false,
        maxControlBytes: RemoteSurfaceLimits.maxControlBytes,
      ),
      session.mode,
      request.surfaceSessionRef,
      result.stateRevision,
      result.nodes,
    );
    if (sealed is! RemoteOpenAccepted) {
      return _remember(session, request, _denied(request, 'PROVIDER_FAILED'));
    }
    session
      ..stateRevision = sealed.snapshot.stateRevision
      ..nodes = sealed.snapshot.nodes;
    if (result.eventState != null) {
      session.events.add(
        RemoteSurfaceEvent(
          surfaceSessionRef: request.surfaceSessionRef,
          generation: session.generation,
          seq: session.events.length + 1,
          jobRef: result.jobRef,
          state: result.eventState!,
          occurredAtMs: _clock().millisecondsSinceEpoch,
          stateRevision: session.stateRevision,
        ),
      );
    }
    final receipt = RemoteControlReceipt(
      requestId: request.requestId,
      idempotencyKey: request.idempotencyKey,
      status: 'accepted',
      jobRef: result.jobRef,
      stateRevision: session.stateRevision,
      decisionRef: 'decision-${++_decisionSerial}',
    );
    if (action.effect == 'external-side-effect') {
      _jobs?.put(
        idempotencyKey: request.idempotencyKey,
        actionId: request.actionId,
        canonical: canonicalJson(request.input),
        receipt: receipt,
      );
    }
    return _remember(session, request, receipt);
  }

  RemoteControlReceipt? lookup({
    required String surfaceSessionRef,
    required int generation,
    String? requestId,
    String? idempotencyKey,
  }) {
    final session = _sessions[surfaceSessionRef];
    if (session == null ||
        session.generation != generation ||
        _stale(session) != null) {
      return null;
    }
    if (requestId != null) {
      final byRequest = session.byRequest[requestId];
      if (byRequest != null) return byRequest;
    }
    return idempotencyKey == null
        ? null
        : session.ledger[idempotencyKey]?.receipt;
  }

  RemoteSurfaceSnapshot readSnapshot({
    required String surfaceSessionRef,
    required int generation,
  }) {
    final session = _live(surfaceSessionRef, generation);
    return _snapshot(session, surfaceSessionRef);
  }

  List<RemoteSurfaceEvent> eventsAfter({
    required String surfaceSessionRef,
    required int generation,
    required int afterSeq,
  }) {
    final session = _live(surfaceSessionRef, generation);
    return session.events.where((event) => event.seq > afterSeq).toList();
  }

  bool _visible(
    RemoteSurfaceDescriptor descriptor,
    RemoteConnectionContext context,
  ) =>
      descriptor.workspaceRef == context.workspaceRef &&
      context.permissions.containsAll(descriptor.readPermissions);

  RemoteSurfaceOffer _offer(
    RemoteSurfaceProvider provider,
    RemoteSurfaceDescriptor descriptor,
    RemoteClientHello hello,
  ) {
    final mode = selectRemoteSurfaceMode(descriptor.modes, hello);
    if (mode == null) {
      return RemoteSurfaceOffer(
        descriptor: descriptor,
        compatible: false,
        unsupportedReason: 'UNSUPPORTED_MODE',
      );
    }
    final sealed = _seal(
      provider,
      descriptor,
      hello,
      mode,
      'sess.preview',
      provider.initialRevision(descriptor.surfaceId),
      provider.initialNodes(descriptor.surfaceId),
    );
    if (sealed is RemoteOpenRejected) {
      return RemoteSurfaceOffer(
        descriptor: descriptor,
        compatible: false,
        unsupportedReason: sealed.errorCode,
      );
    }
    return RemoteSurfaceOffer(
      descriptor: descriptor,
      compatible: true,
      mode: mode,
    );
  }

  RemoteOpenResult _seal(
    RemoteSurfaceProvider provider,
    RemoteSurfaceDescriptor descriptor,
    RemoteClientHello hello,
    String mode,
    String sessionRef,
    String stateRevision,
    List<RemoteSurfaceNode> nodes,
  ) {
    try {
      for (final capability in descriptor.requiredCapabilities) {
        if (!hello.components.contains(capability)) {
          throw const RemoteSurfaceTreeRejection('UNSUPPORTED_COMPONENT');
        }
      }
      final parsed = parseNodeList(nodes.map((node) => node.toJson()).toList());
      validateNodesForClient(parsed, hello.components);
      final snapshot = RemoteSurfaceSnapshot.fromJson({
        'protocol': remoteSurfaceSnapshotProtocol,
        'pluginId': provider.pluginId,
        'surfaceId': descriptor.surfaceId,
        'surfaceSessionRef': sessionRef,
        'generation': _epochs[provider.pluginId] ?? 1,
        'stateRevision': stateRevision,
        'mode': mode,
        'nodes': parsed.map((node) => node.toJson()).toList(),
      });
      return RemoteOpenAccepted(snapshot);
    } on RemoteSurfaceTreeRejection catch (error) {
      return RemoteOpenRejected(status: 'unsupported', errorCode: error.code);
    } on RemoteSurfaceFormatException catch (error) {
      if (error.code == 'UNSUPPORTED_COMPONENT') {
        return const RemoteOpenRejected(
          status: 'unsupported',
          errorCode: 'UNSUPPORTED_COMPONENT',
        );
      }
      return const RemoteOpenRejected(
        status: 'denied',
        errorCode: 'PROVIDER_FAILED',
      );
    }
  }

  _Session _live(String surfaceSessionRef, int generation) {
    final session = _sessions[surfaceSessionRef];
    if (session == null) throw const RemoteSurfaceClosed('SESSION_CLOSED');
    if (session.generation != generation || _stale(session) != null) {
      throw const RemoteSurfaceClosed('STALE_GENERATION');
    }
    return session;
  }

  String? _stale(_Session session) {
    if (session.hostEpoch != _hostEpoch) return 'STALE_GENERATION';
    if (!_providers.containsKey(session.pluginId)) return 'STALE_GENERATION';
    if (session.pluginEpoch != _epochs[session.pluginId]) {
      return 'STALE_GENERATION';
    }
    return null;
  }

  RemoteSurfaceSnapshot _snapshot(_Session session, String sessionRef) {
    return RemoteSurfaceSnapshot(
      pluginId: session.pluginId,
      surfaceId: session.surfaceId,
      surfaceSessionRef: sessionRef,
      generation: session.generation,
      stateRevision: session.stateRevision,
      mode: session.mode,
      nodes: session.nodes,
    );
  }

  RemoteControlReceipt _remember(
    _Session session,
    RemoteControlRequest request,
    RemoteControlReceipt receipt,
  ) {
    session.ledger.putIfAbsent(
      request.idempotencyKey,
      () => _LedgerEntry(
        actionId: request.actionId,
        canonical: canonicalJson(request.input),
        receipt: receipt,
      ),
    );
    session.byRequest.putIfAbsent(request.requestId, () => receipt);
    return session.ledger[request.idempotencyKey]!.receipt;
  }

  RemoteControlReceipt _denied(RemoteControlRequest request, String code) =>
      RemoteControlReceipt(
        requestId: request.requestId,
        idempotencyKey: request.idempotencyKey,
        status: 'denied',
        errorCode: _safeCode(code),
      );

  RemoteControlReceipt _conflict(RemoteControlRequest request, String code) =>
      RemoteControlReceipt(
        requestId: request.requestId,
        idempotencyKey: request.idempotencyKey,
        status: 'conflict',
        errorCode: _safeCode(code),
      );

  RemoteControlReceipt _unsupported(
    RemoteControlRequest request,
    String code,
  ) => RemoteControlReceipt(
    requestId: request.requestId,
    idempotencyKey: request.idempotencyKey,
    status: 'unsupported',
    errorCode: _safeCode(code),
  );

  String _safeCode(String code) =>
      RegExp(r'^[A-Z][A-Z0-9_]{1,63}$').hasMatch(code)
      ? code
      : 'PROVIDER_FAILED';
}

final class _Session {
  _Session({
    required this.pluginId,
    required this.surfaceId,
    required this.generation,
    required this.pluginEpoch,
    required this.hostEpoch,
    required this.descriptor,
    required this.components,
    required this.stateRevision,
    required this.nodes,
    required this.mode,
  });

  final String pluginId;
  final String surfaceId;
  final int generation;
  final int pluginEpoch;
  final int hostEpoch;
  final RemoteSurfaceDescriptor descriptor;
  final List<String> components;
  final String mode;
  String stateRevision;
  List<RemoteSurfaceNode> nodes;
  final events = <RemoteSurfaceEvent>[];
  final ledger = <String, _LedgerEntry>{};
  final byRequest = <String, RemoteControlReceipt>{};
}

final class _LedgerEntry {
  const _LedgerEntry({
    required this.actionId,
    required this.canonical,
    required this.receipt,
  });

  final String actionId;
  final String canonical;
  final RemoteControlReceipt receipt;
}
