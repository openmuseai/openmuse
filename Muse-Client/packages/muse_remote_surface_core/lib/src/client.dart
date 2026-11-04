import 'dart:typed_data';

import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'host.dart';
import 'provider.dart';
import 'transport.dart';

final class RemoteWorkbenchCommand {
  const RemoteWorkbenchCommand({
    required this.id,
    required this.label,
    required this.invoke,
  });

  final String id;
  final String label;
  final Future<void> Function() invoke;
}

final class RemoteWorkbenchController {
  RemoteWorkbenchController({
    required RemoteSurfaceTransport transport,
    required this.context,
    RemoteClientHello? hello,
    this.notice,
    this.loadMedia,
    this.commands = const [],
  }) : _transport = transport, // ignore: prefer_initializing_formals
       hello = hello ?? RemoteClientHello.mobileV1;

  final RemoteSurfaceTransport _transport;
  final RemoteClientHello hello;
  final RemoteConnectionContext context;
  final String? notice;
  final Future<Uint8List?> Function(String handle)? loadMedia;
  final List<RemoteWorkbenchCommand> commands;

  List<RemoteSurfaceOffer> offers = const [];
  RemoteSurfaceSnapshot? snapshot;
  RemoteSurfaceDescriptor? descriptor;
  String? error;
  String? jobState;
  int lastSeq = 0;
  var busy = false;
  final drafts = <String, Object?>{};
  var _started = false;
  var _disposed = false;
  var _serial = 0;
  final _listeners = <void Function()>[];

  void addListener(void Function() listener) => _listeners.add(listener);

  void removeListener(void Function() listener) => _listeners.remove(listener);

  void dispose() {
    _disposed = true;
    _listeners.clear();
  }

  Future<void> start() async {
    if (_started) return;
    _started = true;
    try {
      await discover();
      final compatible = offers.where((offer) => offer.compatible).toList();
      if (compatible.length == 1) await openOffer(compatible.first);
    } catch (_) {
      error = '远程工作台暂时不可用';
      _notify();
    }
  }

  Future<void> discover() async {
    offers = await _transport.discover(hello: hello, context: context);
    _notify();
  }

  Future<void> openOffer(RemoteSurfaceOffer offer) async {
    if (!offer.compatible) {
      error = offer.unsupportedReason ?? 'UNSUPPORTED_MODE';
      _notify();
      return;
    }
    final result = await _transport.open(
      pluginId: offer.descriptor.pluginId,
      surfaceId: offer.descriptor.surfaceId,
      hello: hello,
      context: context,
    );
    if (result is RemoteOpenRejected) {
      error = result.errorCode;
      snapshot = null;
      descriptor = null;
    } else if (result is RemoteOpenAccepted) {
      error = null;
      jobState = null;
      lastSeq = 0;
      snapshot = result.snapshot;
      descriptor = offer.descriptor;
      _seed(result.snapshot);
    }
    _notify();
  }

  void closeSurface() {
    snapshot = null;
    descriptor = null;
    jobState = null;
    error = null;
    lastSeq = 0;
    _notify();
  }

  void setField(String fieldId, Object? value) {
    drafts[fieldId] = value;
    _notify();
  }

  Future<void> invoke(RemoteSurfaceNode node) async {
    final current = snapshot;
    final description = descriptor;
    final actionId = node.props['actionId'];
    if (busy || current == null || description == null || actionId is! String) {
      return;
    }
    final action = description.action(actionId);
    if (action == null) {
      error = 'UNKNOWN_ACTION';
      _notify();
      return;
    }
    busy = true;
    _serial += 1;
    final request = RemoteControlRequest(
      requestId: 'req-$_serial',
      surfaceSessionRef: current.surfaceSessionRef,
      generation: current.generation,
      actionId: actionId,
      input: _inputFor(node),
      expectedStateRevision: current.stateRevision,
      idempotencyKey: 'submission-$_serial',
      deadlineMs: 10000,
    );
    _notify();
    try {
      final receipt = await _transport.submit(request, context: context);
      await _apply(receipt);
    } on RemoteSurfaceTimeout {
      await _recover(request, action.effect);
    } finally {
      busy = false;
      _notify();
    }
  }

  Future<void> reconnect() async {
    final current = snapshot;
    if (current == null) {
      await discover();
      return;
    }
    try {
      snapshot = await _transport.snapshot(
        surfaceSessionRef: current.surfaceSessionRef,
        generation: current.generation,
      );
      await _takeEvents(current);
      error = null;
    } on RemoteSurfaceClosed {
      snapshot = null;
      descriptor = null;
      jobState = null;
      lastSeq = 0;
      error = '界面已失效，请重新打开';
      offers = await _transport.discover(hello: hello, context: context);
    }
    _notify();
  }

  Future<void> _recover(RemoteControlRequest request, String effect) async {
    if (effect != 'external-side-effect') {
      error = '请求超时';
      return;
    }
    final found = await _transport.lookup(
      surfaceSessionRef: request.surfaceSessionRef,
      generation: request.generation,
      requestId: request.requestId,
      idempotencyKey: request.idempotencyKey,
    );
    if (found == null) {
      error = '结果未知，未重新提交';
      jobState = 'outcome_unknown';
      return;
    }
    await _apply(found);
  }

  Future<void> _apply(RemoteControlReceipt receipt) async {
    final current = snapshot;
    if (receipt.status != 'accepted' || current == null) {
      error = receipt.errorCode ?? receipt.status;
      return;
    }
    error = null;
    snapshot = await _transport.snapshot(
      surfaceSessionRef: current.surfaceSessionRef,
      generation: current.generation,
    );
    _seed(snapshot!);
    await _takeEvents(current);
  }

  Future<void> _takeEvents(RemoteSurfaceSnapshot current) async {
    final events = await _transport.eventsAfter(
      surfaceSessionRef: current.surfaceSessionRef,
      generation: current.generation,
      afterSeq: lastSeq,
    );
    for (final event in events) {
      if (event.generation != current.generation || event.seq <= lastSeq) {
        continue;
      }
      lastSeq = event.seq;
      jobState = event.state;
    }
  }

  void _seed(RemoteSurfaceSnapshot value) {
    for (final node in _flatten(value.nodes)) {
      if (node.type != 'form') continue;
      final fields = node.props['fields'];
      if (fields is! List) continue;
      for (final raw in fields) {
        if (raw is! Map ||
            raw['id'] is! String ||
            drafts.containsKey(raw['id'])) {
          continue;
        }
        if (raw.containsKey('value')) {
          drafts[raw['id'] as String] = raw['value'];
        }
      }
    }
  }

  Map<String, Object?> _inputFor(RemoteSurfaceNode node) {
    final input = <String, Object?>{};
    final fixed = node.props['input'];
    if (fixed is Map) {
      for (final entry in fixed.entries) {
        input[entry.key.toString()] = entry.value;
      }
    }
    final bound = node.props['inputFromFields'];
    if (bound is List) {
      for (final field in bound) {
        if (field is! String) continue;
        input[field] = drafts[field] ?? '';
      }
    }
    return input;
  }

  List<RemoteSurfaceNode> _flatten(List<RemoteSurfaceNode> nodes) => [
    for (final node in nodes) ...[node, ..._flatten(node.children)],
  ];

  void _notify() {
    if (_disposed) return;
    for (final listener in List<void Function()>.of(_listeners)) {
      listener();
    }
  }
}
