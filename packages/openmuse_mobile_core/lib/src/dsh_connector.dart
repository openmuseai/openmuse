enum DshPlacement { localSidecar, cloudRemote, pairedDesktop }

enum DshPresentationState {
  idle,
  queued,
  loading,
  bridgeBound,
  workspaceAttached,
  ready,
  reconnecting,
  closed,
  failed,
}

final class DshSessionDescriptor {
  const DshSessionDescriptor({
    required this.sessionRef,
    required this.origin,
    required this.path,
    required this.generation,
    this.allowInsecureLoopback = false,
  });
  final String sessionRef;
  final String origin;
  final String path;
  final int generation;
  final bool allowInsecureLoopback;
}

final class DshSessionSummary {
  const DshSessionSummary({
    required this.sessionRef,
    required this.workspaceRef,
    required this.state,
    required this.nodeId,
    required this.createdAtMs,
    required this.lastActiveAtMs,
    required this.attachedDeviceCount,
    this.instanceRef,
    this.queuePosition,
  });

  final String sessionRef;
  final String workspaceRef;
  final String state;
  final String nodeId;
  final int createdAtMs;
  final int lastActiveAtMs;
  final int attachedDeviceCount;
  final String? instanceRef;
  final int? queuePosition;

  bool get isRunning =>
      state == 'starting' || state == 'ready' || state == 'idle';
}

abstract interface class DshSessionCatalogPort {
  Future<List<DshSessionSummary>> listSessions();
}

abstract interface class DshRuntimeConnector {
  DshPlacement get placement;
  Future<DshSessionDescriptor> open(String workspaceRef, int generation);
  Future<void> close(String sessionRef);
}

final class DshPresentationController {
  DshPresentationState state = DshPresentationState.idle;
  int generation = 0;
  DshSessionDescriptor? session;

  Future<void> open(DshRuntimeConnector connector, String workspaceRef) async {
    state = DshPresentationState.queued;
    final requested = ++generation;
    final opened = await connector.open(workspaceRef, requested);
    if (opened.generation != generation || !_safe(opened)) return;
    session = opened;
    state = DshPresentationState.loading;
  }

  void pageLoaded(int callbackGeneration) {
    if (_current(callbackGeneration)) state = DshPresentationState.bridgeBound;
  }

  void bridgeBound(int callbackGeneration) {
    if (_current(callbackGeneration))
      state = DshPresentationState.workspaceAttached;
  }

  void workspaceAttached(int callbackGeneration) {
    if (_current(callbackGeneration)) state = DshPresentationState.ready;
  }

  void disconnected() {
    if (state != DshPresentationState.closed)
      state = DshPresentationState.reconnecting;
  }

  void close() {
    generation++;
    session = null;
    state = DshPresentationState.closed;
  }

  bool _current(int value) => value == generation && session != null;
  bool _safe(DshSessionDescriptor value) {
    final uri = Uri.tryParse(value.origin);
    final loopback =
        uri != null &&
        (uri.host == 'localhost' ||
            uri.host == '127.0.0.1' ||
            uri.host == '::1' ||
            uri.host == '10.0.2.2');
    final safeOrigin =
        uri != null &&
        uri.host.isNotEmpty &&
        (uri.scheme == 'https' ||
            (value.allowInsecureLoopback && uri.scheme == 'http' && loopback));
    final safePath =
        value.path.startsWith('/session/') || value.path.startsWith('/u/');
    return safeOrigin && safePath && !value.path.contains('..');
  }
}
