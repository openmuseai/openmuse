part of 'resource_capabilities.dart';

/// Content adapters implement this port without importing content types into
/// the SDK. Preparation freezes an owner; checkpoint may fail; commit performs
/// only accepted-content teardown and reports cleanup failures separately.
abstract interface class PreparedResourceClose {
  bool get isActive;
  Future<void> checkpoint();
  Future<void> commit();
  void cancel();
}

/// Optional close path that retains private recovery without a resource commit.
abstract interface class PreparedPrivateRecoveryClose
    implements PreparedResourceClose {
  Future<void> commitPrivateRecovery();
}

abstract interface class ResourceCheckpointCloseHold {
  Future<void> drain();
  void resume();
}

/// Optional surviving-owner resume that preserves a private-close decision.
/// Restoring the same paused payload must not itself schedule another save.
abstract interface class ResourcePrivateRecoveryCheckpointHold
    implements ResourceCheckpointCloseHold {
  void resumeAfterPrivateRecovery();
}

final class ResourceCloseParticipant {
  ResourceCloseParticipant({
    required this.prepare,
    this.supportsPrivateRecovery = false,
    this.holdCheckpoint,
  }) : _order = _nextOrder++;
  final bool supportsPrivateRecovery;
  final ResourceCheckpointCloseHold Function()? holdCheckpoint;
  static int _nextOrder = 0;
  final int _order;
  final Future<PreparedResourceClose> Function(Set<ResourceEditorSession>)
  prepare;
}

Map<ResourceEditorSession, Set<ResourceEditorSession>> _resourceCloseGraph(
  ResourceEditorSession root,
) {
  final graph = <ResourceEditorSession, Set<ResourceEditorSession>>{};
  void visit(ResourceEditorSession session) {
    if (graph.containsKey(session)) return;
    if (graph.length >= 64)
      throw const ResourceFailure('RESOURCE_DEPENDENCY_LIMIT');
    final children = Set<ResourceEditorSession>.of(session._dependents);
    graph[session] = children;
    for (final child in children) {
      visit(child);
    }
  }

  visit(root);
  return graph;
}

final class _PreparedResourceTree {
  _PreparedResourceTree(
    this.root,
    this.graph,
    this.epoch,
    this.owners,
    this.holds,
    this.privateRecoveryOnly,
  );
  final bool privateRecoveryOnly;
  final List<ResourceCheckpointCloseHold> holds;
  final ResourceEditorSession root;
  final Map<ResourceEditorSession, Set<ResourceEditorSession>> graph;
  final int epoch;
  final List<PreparedResourceClose> owners;
  bool _finished = false;
  bool get isCurrent =>
      !_finished &&
      root._closePreparationEpoch == epoch &&
      owners.every((owner) => owner.isActive) &&
      graph.entries.every(
        (e) =>
            !e.key._disposed &&
            e.key._dependents.length == e.value.length &&
            e.key._dependents.containsAll(e.value) &&
            (privateRecoveryOnly || !e.key.isActive || !e.key.dirty),
      );
  void cancel() {
    if (_finished) return;
    _finished = true;
    for (final owner in owners.reversed) {
      owner.cancel();
    }
    for (final hold in holds.reversed) {
      hold.resume();
    }
    for (final session in graph.keys) {
      session._closePrepared = false;
    }
  }

  Future<void> commit() async {
    if (!isCurrent)
      throw const ResourceFailure('RESOURCE_CLOSE_PREPARATION_EXPIRED');
    for (final owner in owners) {
      if (privateRecoveryOnly) {
        await (owner as PreparedPrivateRecoveryClose).commitPrivateRecovery();
      } else {
        await owner.commit();
      }
    }
    _finished = true;
    for (final hold in holds.reversed) {
      if (privateRecoveryOnly &&
          hold is ResourcePrivateRecoveryCheckpointHold) {
        hold.resumeAfterPrivateRecovery();
      } else {
        hold.resume();
      }
    }
    for (final session in graph.keys) {
      // Ended accounts close from their original private recovery checkpoint;
      // this does not advance a resource version or acknowledge a Cloud write.
      if (!session.isActive) session.dirty = false;
      session._closePrepared = true;
      session._closeFinalized = true;
    }
  }
}

Future<_PreparedResourceTree> _prepareResourceTree(
  ResourceEditorSession root,
  Map<ResourceEditorSession, Set<ResourceEditorSession>> graph,
  int epoch, {
  bool privateRecoveryOnly = false,
}) async {
  final groups = <ResourceCloseParticipant, Set<ResourceEditorSession>>{};
  for (final session in graph.keys) {
    session._closePrepared = false;
    final participant = session.closeParticipant;
    if (participant == null) {
      if (privateRecoveryOnly)
        throw const ResourceFailure('RESOURCE_PRIVATE_RECOVERY_UNSUPPORTED');
      if (session.editing || session.releaseForClose != null) {
        throw const ResourceFailure('RESOURCE_CLOSE_PREPARATION_UNSUPPORTED');
      }
    } else {
      if (privateRecoveryOnly && !participant.supportsPrivateRecovery)
        throw const ResourceFailure('RESOURCE_PRIVATE_RECOVERY_UNSUPPORTED');
      (groups[participant] ??= <ResourceEditorSession>{}).add(session);
    }
  }
  final prepared = <PreparedResourceClose>[];
  final holds = <ResourceCheckpointCloseHold>[];
  final tree = _PreparedResourceTree(
    root,
    graph,
    epoch,
    prepared,
    holds,
    privateRecoveryOnly,
  );
  try {
    // Stable per-owner ordering also applies when two trees overlap. Adapters
    // serialize each owner until its preparation is committed or cancelled.
    final participants = groups.keys.toList()
      ..sort((a, b) => a._order.compareTo(b._order));
    // Pause every autosave synchronously before yielding to any domain owner.
    for (final participant in participants) {
      final hold = participant.holdCheckpoint?.call();
      if (hold != null) holds.add(hold);
    }
    for (final participant in participants) {
      prepared.add(
        await participant.prepare(Set.unmodifiable(groups[participant]!)),
      );
      if (root._closePreparationEpoch != epoch)
        throw const ResourceFailure('RESOURCE_CLOSE_CANCELLED');
    }
    // Every owner is frozen before any fallible application checkpoint starts.
    // A changed dependency set requires a fresh preparation, never a partial tree.
    bool graphCurrent() => graph.entries.every(
      (e) =>
          e.key._dependents.length == e.value.length &&
          e.key._dependents.containsAll(e.value),
    );
    if (!graphCurrent())
      throw const ResourceFailure('RESOURCE_DEPENDENCIES_CHANGED');
    for (final hold in holds) {
      await hold.drain();
    }
    if (privateRecoveryOnly &&
        prepared.any((p) => p is! PreparedPrivateRecoveryClose))
      throw const ResourceFailure('RESOURCE_PRIVATE_RECOVERY_UNSUPPORTED');
    if (!privateRecoveryOnly)
      for (final owner in prepared) {
        await owner.checkpoint();
      }
    if (!privateRecoveryOnly)
      for (final session in graph.keys.where(
        (s) => s.closeParticipant == null,
      )) {
        await session.flushPendingChanges?.call();
      }
    if (!tree.isCurrent) throw const ResourceFailure('UNSAVED_CHILD_CHANGES');
    for (final session in graph.keys) {
      session._closePrepared = true;
    }
    return tree;
  } on Object {
    tree.cancel();
    rethrow;
  }
}
