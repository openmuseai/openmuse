import 'dart:async';
import 'dart:convert';
import 'dart:math';

import 'package:muse_plugin_facets/muse_plugin_facets.dart';

enum MuseSurfaceState { background, active, focused, closing, closed }

final class MuseSurfaceLease {
  const MuseSurfaceLease._(this.surfaceInstanceRef, this.generation);

  final String surfaceInstanceRef;
  final int generation;
}

final class MuseFacetRegistration {
  const MuseFacetRegistration({
    required this.pluginId,
    required this.pluginVersion,
    required this.facetInstanceRef,
    required this.surfaceKinds,
  });

  final String pluginId;
  final String pluginVersion;
  final String facetInstanceRef;
  final Set<String> surfaceKinds;
}

final class MuseOpenSurface {
  const MuseOpenSurface({
    required this.facetInstanceRef,
    required this.surfaceKind,
    required this.scopeRef,
    required this.resourceRef,
    required this.windowRef,
  });

  final String facetInstanceRef;
  final String surfaceKind;
  final String scopeRef;
  final String resourceRef;
  final String windowRef;
}

abstract interface class MuseUiFacet {
  Future<void> onDomainChange(MuseDomainChangeV1 change);
  Future<void> dispose();
}

abstract interface class MusePresentationIntentHandler {
  Future<MusePresentationIntentResultV1> onPresentationIntent(
    MusePresentationIntentV1 intent,
  );
}

abstract interface class MuseSurfaceContextSink {
  Future<void> publish(MuseContextContributionV1 context);
  Future<void> closeSurface(String surfaceInstanceRef, String scopeRef);
}

final class MuseSurfaceRuntimeException implements Exception {
  MuseSurfaceRuntimeException(this.code);
  final String code;
  @override
  String toString() => 'MuseSurfaceRuntimeException($code)';
}

final class MuseUiSurfaceRuntime {
  MuseUiSurfaceRuntime({
    required MuseSurfaceContextSink sink,
    Duration stateDebounce = const Duration(milliseconds: 50),
    int maxPayloadBytes = 64 * 1024,
    int Function()? clock,
  })  : _sink = sink,
        _stateDebounce = stateDebounce,
        _maxPayloadBytes = maxPayloadBytes,
        _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);

  final MuseSurfaceContextSink _sink;
  final Duration _stateDebounce;
  final int _maxPayloadBytes;
  final int Function() _clock;
  final _registrations = <String, MuseFacetRegistration>{};
  final _surfaces = <String, _SurfaceRecord>{};
  final _focusedByWindow = <String, String>{};
  final _pendingState = <String, MuseContextContributionV1>{};
  final _timers = <String, Timer>{};
  final _random = Random.secure();
  var _generation = 0;
  var _disposed = false;

  int get surfaceCount => _surfaces.length;

  Map<String, MuseSurfaceState> get inventory => Map.unmodifiable(
        _surfaces.map((key, value) => MapEntry(key, value.state)),
      );

  void registerFacet(MuseFacetRegistration registration) {
    _ensureRunning();
    if (_registrations.containsKey(registration.facetInstanceRef)) {
      throw MuseSurfaceRuntimeException('FACET_ALREADY_REGISTERED');
    }
    if (registration.surfaceKinds.isEmpty) {
      throw MuseSurfaceRuntimeException('SURFACE_KINDS_EMPTY');
    }
    _registrations[registration.facetInstanceRef] = registration;
  }

  Future<void> unregisterFacet(String facetInstanceRef) async {
    final refs = _surfaces.values
        .where((record) => record.input.facetInstanceRef == facetInstanceRef)
        .map((record) => record.lease)
        .toList();
    for (final lease in refs) {
      await closeSurface(lease);
    }
    _registrations.remove(facetInstanceRef);
  }

  MuseSurfaceLease openSurface(MuseOpenSurface input, MuseUiFacet facet) {
    _ensureRunning();
    final registration = _registrations[input.facetInstanceRef];
    if (registration == null) {
      throw MuseSurfaceRuntimeException('FACET_NOT_REGISTERED');
    }
    if (!registration.surfaceKinds.contains(input.surfaceKind)) {
      throw MuseSurfaceRuntimeException('SURFACE_KIND_NOT_DECLARED');
    }
    final ref = _opaqueRef('surface');
    final lease = MuseSurfaceLease._(ref, ++_generation);
    _surfaces[ref] = _SurfaceRecord(
      lease: lease,
      input: input,
      registration: registration,
      facet: facet,
    );
    return lease;
  }

  void setActive(MuseSurfaceLease lease, {required bool active}) {
    final record = _record(lease);
    if (!active && record.state == MuseSurfaceState.focused) {
      _focusedByWindow.remove(record.input.windowRef);
    }
    record.state =
        active ? MuseSurfaceState.active : MuseSurfaceState.background;
  }

  void setFocused(MuseSurfaceLease lease, {required bool focused}) {
    final record = _record(lease);
    if (focused) {
      final previousRef = _focusedByWindow[record.input.windowRef];
      if (previousRef != null &&
          previousRef != record.lease.surfaceInstanceRef) {
        final previous = _surfaces[previousRef];
        if (previous != null && previous.state == MuseSurfaceState.focused) {
          previous.state = MuseSurfaceState.active;
        }
      }
      _focusedByWindow[record.input.windowRef] =
          record.lease.surfaceInstanceRef;
      record.state = MuseSurfaceState.focused;
    } else {
      _focusedByWindow.remove(record.input.windowRef);
      record.state = MuseSurfaceState.active;
    }
  }

  Future<void> publishContext(
    MuseSurfaceLease lease,
    MuseContextContributionV1 context,
  ) async {
    final record = _record(lease);
    _validateContextOwner(record, context);
    validateMuseFacetValue(
      MuseFacetSchemaKind.contextContribution,
      context.toJson(),
    );
    if (context.expiresAt <= _clock()) {
      throw MuseSurfaceRuntimeException('CONTEXT_EXPIRED');
    }
    if (utf8.encode(jsonEncode(context.toJson())).length > _maxPayloadBytes) {
      throw MuseSurfaceRuntimeException('CONTEXT_TOO_LARGE');
    }
    final revision = int.parse(context.contextRevision);
    final revisionKey = '${context.surfaceInstanceRef}:${context.contextType}';
    if (revision <= (record.revisions[revisionKey] ?? -1)) {
      throw MuseSurfaceRuntimeException('STALE_CONTEXT_REVISION');
    }
    record.revisions[revisionKey] = revision;
    if (context.lane == MuseContextLane.control) {
      await _sink.publish(context);
      return;
    }
    if (record.state == MuseSurfaceState.background) return;
    _pendingState[revisionKey] = context;
    _timers[revisionKey]?.cancel();
    _timers[revisionKey] = Timer(_stateDebounce, () async {
      _timers.remove(revisionKey);
      final pending = _pendingState.remove(revisionKey);
      final live = _surfaces[lease.surfaceInstanceRef];
      if (pending != null &&
          live != null &&
          live.state != MuseSurfaceState.closed) {
        await _sink.publish(pending);
      }
    });
  }

  Future<void> routeDomainChange(MuseDomainChangeV1 change) async {
    validateMuseFacetValue(MuseFacetSchemaKind.domainChange, change.toJson());
    // Route by plugin + resource. Host scopeRef is an opaque authority token and
    // is not the Flutter surface scope (document/view id).
    final targets = _surfaces.values.where(
      (record) =>
          record.registration.pluginId == change.pluginId &&
          record.input.resourceRef == change.resourceRef &&
          record.state != MuseSurfaceState.closing &&
          record.state != MuseSurfaceState.closed,
    );
    for (final target in targets.toList()) {
      try {
        await target.facet.onDomainChange(change);
      } catch (_) {
        target.handlerFailures++;
      }
    }
  }

  Future<MusePresentationIntentResultV1> routePresentationIntent(
    MusePresentationIntentV1 intent, {
    Duration timeout = const Duration(seconds: 2),
  }) async {
    validateMuseFacetValue(
      MuseFacetSchemaKind.presentationIntent,
      intent.toJson(),
    );
    final now = _clock();
    if (intent.expiresAt <= now) {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.stale,
        'INTENT_EXPIRED',
      );
    }
    final candidates = _surfaces.values.where((record) {
      return record.registration.pluginId == intent.pluginId &&
          record.input.scopeRef == intent.scopeRef &&
          record.state != MuseSurfaceState.closing &&
          record.state != MuseSurfaceState.closed &&
          (intent.targetSurfaceInstanceRef == null ||
              record.lease.surfaceInstanceRef ==
                  intent.targetSurfaceInstanceRef);
    }).toList();
    if (candidates.isEmpty) {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.notFound,
        'SURFACE_NOT_FOUND',
      );
    }
    final focused = candidates
        .where((record) => record.state == MuseSurfaceState.focused)
        .toList();
    final target = intent.targetSurfaceInstanceRef != null
        ? candidates.single
        : focused.length == 1
            ? focused.single
            : null;
    if (target == null) {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.rejected,
        'SURFACE_AMBIGUOUS',
      );
    }
    final facet = target.facet;
    if (facet is! MusePresentationIntentHandler) {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.notSupported,
        'INTENT_NOT_SUPPORTED',
      );
    }
    final handler = facet as MusePresentationIntentHandler;
    try {
      return await handler.onPresentationIntent(intent).timeout(timeout);
    } on TimeoutException {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.timedOut,
        'HANDLER_TIMEOUT',
      );
    } catch (_) {
      return _intentResult(
        intent,
        MusePresentationIntentStatus.rejected,
        'HANDLER_FAILED',
      );
    }
  }

  Future<void> closeSurface(MuseSurfaceLease lease) async {
    final record = _record(lease);
    record.state = MuseSurfaceState.closing;
    _focusedByWindow.remove(record.input.windowRef);
    final prefix = '${lease.surfaceInstanceRef}:';
    for (final key
        in _timers.keys.where((key) => key.startsWith(prefix)).toList()) {
      _timers.remove(key)?.cancel();
      _pendingState.remove(key);
    }
    await _sink.closeSurface(lease.surfaceInstanceRef, record.input.scopeRef);
    try {
      await record.facet.dispose();
    } finally {
      record.state = MuseSurfaceState.closed;
      _surfaces.remove(lease.surfaceInstanceRef);
    }
  }

  Future<void> dispose() async {
    if (_disposed) return;
    for (final lease in _surfaces.values.map((value) => value.lease).toList()) {
      await closeSurface(lease);
    }
    for (final timer in _timers.values) {
      timer.cancel();
    }
    _timers.clear();
    _pendingState.clear();
    _registrations.clear();
    _disposed = true;
  }

  _SurfaceRecord _record(MuseSurfaceLease lease) {
    _ensureRunning();
    final record = _surfaces[lease.surfaceInstanceRef];
    if (record == null || record.lease.generation != lease.generation) {
      throw MuseSurfaceRuntimeException('INVALID_SURFACE_LEASE');
    }
    if (record.state == MuseSurfaceState.closing ||
        record.state == MuseSurfaceState.closed) {
      throw MuseSurfaceRuntimeException('SURFACE_CLOSED');
    }
    return record;
  }

  void _validateContextOwner(
    _SurfaceRecord record,
    MuseContextContributionV1 context,
  ) {
    if (context.pluginId != record.registration.pluginId ||
        context.pluginVersion != record.registration.pluginVersion ||
        context.facetInstanceRef != record.registration.facetInstanceRef ||
        context.surfaceInstanceRef != record.lease.surfaceInstanceRef ||
        context.surfaceKind != record.input.surfaceKind ||
        context.scopeRef != record.input.scopeRef) {
      throw MuseSurfaceRuntimeException('CONTEXT_OWNER_MISMATCH');
    }
  }

  void _ensureRunning() {
    if (_disposed) throw MuseSurfaceRuntimeException('RUNTIME_DISPOSED');
  }

  String _opaqueRef(String prefix) {
    final parts = List.generate(4, (_) => _random.nextInt(0x100000000));
    return '$prefix.${parts.map((part) => part.toRadixString(16).padLeft(8, '0')).join()}';
  }

  MusePresentationIntentResultV1 _intentResult(
    MusePresentationIntentV1 intent,
    MusePresentationIntentStatus status,
    String reasonCode,
  ) =>
      MusePresentationIntentResultV1(
        intentRef: intent.intentRef,
        status: status,
        reasonCode: reasonCode,
        completedAt: _clock(),
      );
}

final class _SurfaceRecord {
  _SurfaceRecord({
    required this.lease,
    required this.input,
    required this.registration,
    required this.facet,
  });

  final MuseSurfaceLease lease;
  final MuseOpenSurface input;
  final MuseFacetRegistration registration;
  final MuseUiFacet facet;
  final revisions = <String, int>{};
  MuseSurfaceState state = MuseSurfaceState.background;
  int handlerFailures = 0;
}
