enum CloudFlowState {
  signedOut,
  catalog,
  queued,
  binding,
  ready,
  proposing,
  awaitingApproval,
  applied,
  storageUnavailable,
  degraded,
}

final class CloudWorkspaceFlow {
  CloudFlowState state = CloudFlowState.signedOut;
  String? workspaceRef;
  String? revision;
  int generation = 0;
  void login() => state = CloudFlowState.catalog;
  void select(String workspace, String currentRevision) {
    workspaceRef = workspace;
    revision = currentRevision;
    generation++;
    state = CloudFlowState.queued;
  }

  void sessionOpened(int value) {
    if (value == generation) state = CloudFlowState.binding;
  }

  void bound(int value, String workspace) {
    if (value == generation && workspace == workspaceRef)
      state = CloudFlowState.ready;
  }

  void propose(String expectedRevision) {
    if (state != CloudFlowState.ready || expectedRevision != revision)
      throw StateError('stale revision');
    state = CloudFlowState.awaitingApproval;
  }

  void approve(String newRevision) {
    if (state != CloudFlowState.awaitingApproval)
      throw StateError('approval unavailable');
    revision = newRevision;
    state = CloudFlowState.applied;
  }

  void storageFailed() => state = CloudFlowState.storageUnavailable;
  void degraded() => state = CloudFlowState.degraded;
}
