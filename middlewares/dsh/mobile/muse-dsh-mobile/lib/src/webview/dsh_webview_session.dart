enum DshWebViewPhase {
  idle,
  creating,
  loadingDocument,
  documentReady,
  disposing,
  fatalError,
}

class DshWebViewSession {
  int generation = 0;
  DshWebViewPhase phase = DshWebViewPhase.idle;

  int bump() {
    phase = DshWebViewPhase.idle;
    return ++generation;
  }

  bool isLive(int observed) => observed == generation;

  void enter(DshWebViewPhase next) {
    if (phase == DshWebViewPhase.disposing ||
        phase == DshWebViewPhase.fatalError) {
      return;
    }
    phase = next;
  }
}
