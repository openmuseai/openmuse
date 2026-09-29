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
  });
  final String sessionRef;
  final String origin;
  final String path;
  final int generation;
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
    return uri != null &&
        uri.scheme == 'https' &&
        uri.host.isNotEmpty &&
        value.path.startsWith('/session/') &&
        !value.path.contains('..');
  }
}
