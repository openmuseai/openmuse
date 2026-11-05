import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';

import 'package:appflowy/plugins/resource_surface/muse_presentation_dispatch.dart';
import 'package:appflowy_backend/ffi.dart' as backend_ffi;
import 'package:appflowy_backend/log.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';

/// Native entry points `dart_ffi.dll` exposes for the presentation seam.
///
/// `install` is called once per app start with the native port the Host posts
/// dispatches to; `complete` answers the dispatch `requestRef` waits on, with a
/// `null` outcome when the surface must fail closed. `publishMountRoots` hands
/// the Host's granted Mount directories to the Rust core, which is what lets it
/// resolve a partial Mount-relative path by fuzzy matching.
abstract interface class MusePresentationTransport {
  /// True when this build can reach the Host core at all.
  bool get available;

  /// True when the Host accepted `port` and installed a surface dispatcher.
  bool install(int port);

  /// True when the Host accepted the answer for `requestRef`.
  bool complete(String requestRef, String? outcomeJson);

  /// True when the Host accepted the granted Mount catalog.
  bool publishMountRoots(String rootsJson);
}

/// `dart:ffi` transport over the hand-written exports of `dart_ffi.dll`.
///
/// The bindings are looked up lazily and once: a build that predates the
/// exports (or a test host without the library) simply reports "unavailable"
/// instead of failing app start. The miss is remembered too, so an unavailable
/// library is reported once per process rather than once per call.
final class MusePresentationNativeTransport
    implements MusePresentationTransport {
  _MusePresentationNativeBindings? _bindings;
  bool _unavailable = false;

  @override
  bool get available => _load() != null;

  _MusePresentationNativeBindings? _load() {
    if (_unavailable) return null;
    final cached = _bindings;
    if (cached != null) return cached;
    try {
      final library = backend_ffi.dl;
      final bindings = _MusePresentationNativeBindings(
        install: library.lookupFunction<_InstallNative, _InstallDart>(
          'muse_presentation_install_dart_surface',
        ),
        complete: library.lookupFunction<_CompleteNative, _CompleteDart>(
          'muse_presentation_complete_dispatch',
        ),
        publishMountRoots:
            library.lookupFunction<_PublishMountRootsNative, _PublishMountRootsDart>(
          'muse_presentation_publish_mount_roots',
        ),
      );
      _bindings = bindings;
      return bindings;
    } on Object catch (error) {
      _unavailable = true;
      Log.warn('Muse presentation FFI is unavailable: $error');
      return null;
    }
  }

  @override
  bool install(int port) {
    final bindings = _load();
    if (bindings == null) return false;
    try {
      return bindings.install(port) != 0;
    } on Object catch (error) {
      Log.warn('Muse presentation install failed: $error');
      return false;
    }
  }

  @override
  bool complete(String requestRef, String? outcomeJson) {
    final bindings = _load();
    if (bindings == null) return false;
    final requestPtr = requestRef.toNativeUtf8();
    Pointer<Utf8> outcomePtr = nullptr;
    try {
      if (outcomeJson != null) outcomePtr = outcomeJson.toNativeUtf8();
      return bindings.complete(requestPtr, outcomePtr) != 0;
    } on Object catch (error) {
      Log.warn('Muse presentation answer failed: $error');
      return false;
    } finally {
      malloc.free(requestPtr);
      if (outcomePtr != nullptr) malloc.free(outcomePtr);
    }
  }

  @override
  bool publishMountRoots(String rootsJson) {
    final bindings = _load();
    if (bindings == null) return false;
    final rootsPtr = rootsJson.toNativeUtf8();
    try {
      return bindings.publishMountRoots(rootsPtr) != 0;
    } on Object catch (error) {
      Log.warn('Muse presentation Mount catalog failed: $error');
      return false;
    } finally {
      malloc.free(rootsPtr);
    }
  }
}

typedef _InstallNative = Int32 Function(Int64 port);
typedef _InstallDart = int Function(int port);
typedef _CompleteNative = Int32 Function(
  Pointer<Utf8> requestRef,
  Pointer<Utf8> outcomeJson,
);
typedef _CompleteDart = int Function(
  Pointer<Utf8> requestRef,
  Pointer<Utf8> outcomeJson,
);
typedef _PublishMountRootsNative = Int32 Function(Pointer<Utf8> rootsJson);
typedef _PublishMountRootsDart = int Function(Pointer<Utf8> rootsJson);

final class _MusePresentationNativeBindings {
  const _MusePresentationNativeBindings({
    required this.install,
    required this.complete,
    required this.publishMountRoots,
  });

  final _InstallDart install;
  final _CompleteDart complete;
  final _PublishMountRootsDart publishMountRoots;
}

/// Owns the Host→Flutter dispatch port of the running app.
///
/// The Host posts one `muse.presentation/dispatch/v1` JSON document per request
/// on the port handed to [install] and blocks until [MusePresentationTransport]
/// [complete] answers that `requestRef`. One host per app: the seam is
/// process-wide and installed exactly once for the app's lifetime.
final class MusePresentationChannelHost {
  MusePresentationChannelHost({
    MusePresentationTransport? transport,
    MuseResourceSurfaceDispatcher? dispatcher,
    void Function(String message)? onRefused,
    RawReceivePort Function()? portFactory,
  })  : _transport = transport ?? MusePresentationNativeTransport(),
        dispatcher = dispatcher ?? MuseResourceSurfaceDispatcher(),
        _onRefused = onRefused ?? Log.warn,
        _portFactory = portFactory ?? RawReceivePort.new;

  /// The channel the app installs at startup.
  static final MusePresentationChannelHost instance =
      MusePresentationChannelHost();

  final MusePresentationTransport _transport;
  final MuseResourceSurfaceDispatcher dispatcher;
  final void Function(String message) _onRefused;
  final RawReceivePort Function() _portFactory;

  RawReceivePort? _port;
  bool _installed = false;

  /// True once the Host accepted this app's dispatch port.
  bool get installed => _installed;

  /// Port the Host posts dispatches to, once installed.
  @visibleForTesting
  RawReceivePort? get port => _port;

  /// Publishes the directories this Host granted for its Mounts.
  ///
  /// The Rust core treats a `mountRef` as opaque, so it can only resolve a
  /// **partial** Mount-relative path by fuzzy matching once it knows the
  /// directory each Mount was granted. An empty map is a valid, meaningful
  /// catalog: it says this Host currently grants no searchable Mount, which is
  /// exactly what the core needs to hear when the last Mount is unbound.
  ///
  /// Returns true when the core accepted the catalog. A build whose
  /// `dart_ffi.dll` predates the export reports false without a warning: there
  /// is nothing to publish, and the Host keeps forwarding locator paths
  /// verbatim.
  bool publishMountRoots(Map<String, String> roots) {
    final usable = <String, String>{
      for (final entry in roots.entries)
        if (entry.key.isNotEmpty && entry.value.trim().isNotEmpty)
          entry.key: entry.value,
    };
    if (!_transport.available) return false;
    if (!_transport.publishMountRoots(jsonEncode(usable))) {
      _onRefused(
        'muse presentation could not publish the granted Mount catalog',
      );
      return false;
    }
    return true;
  }

  /// Installs the Host dispatch channel exactly once and keeps it for the
  /// lifetime of the app.
  ///
  /// Returns true when this call installed it, false when it was already
  /// installed or when `dart_ffi.dll` exposes no surface install entry point.
  bool install() {
    if (_installed) return false;
    final port = _portFactory();
    if (!_transport.install(port.sendPort.nativePort)) {
      port.close();
      _onRefused(
        'muse presentation channel unavailable: the Host exposed no surface '
        'dispatcher install entry point',
      );
      return false;
    }
    port.handler = (message) => unawaited(handleMessage(message));
    _port = port;
    _installed = true;
    return true;
  }

  /// Decodes and answers one Host dispatch. Public so the seam can be driven
  /// without a live Rust host.
  Future<MusePresentationOutcome> handleMessage(Object? message) async {
    final decoded = decodeMusePresentationWire(message);
    final requestRef = _requestRefOf(decoded);
    final outcome = await dispatcher.handleWire(decoded);
    if (outcome.isFailure) {
      _onRefused(
        'muse presentation refused ${outcome.reasonCode}: ${outcome.detail}',
      );
    }
    if (requestRef == null) {
      _onRefused(
        'muse presentation could not answer a dispatch without a requestRef',
      );
      return outcome;
    }
    if (!_transport.complete(requestRef, outcome.toWireJson())) {
      _onRefused('muse presentation could not answer $requestRef');
    }
    return outcome;
  }

  /// Releases the dispatch port. The app keeps the channel installed for its
  /// whole lifetime, so this exists for tests only.
  void dispose() {
    _port?.close();
    _port = null;
    _installed = false;
  }

  static String? _requestRefOf(Object? decoded) {
    if (decoded is! Map) return null;
    final requestRef = decoded['requestRef'];
    return requestRef is String && requestRef.isNotEmpty ? requestRef : null;
  }
}

/// Publishes the Host's granted Mount directories through the app's one
/// presentation channel, so the Rust core can resolve a partial Mount-relative
/// path (`muse_partial_path`) instead of forwarding a path that does not exist.
///
/// Called by `DshWorkspaceBridge` whenever it publishes the binding, because
/// that is the moment the Host decides which directories it grants.
bool publishMuseMountRoots(Map<String, String> roots) =>
    MusePresentationChannelHost.instance.publishMountRoots(roots);
