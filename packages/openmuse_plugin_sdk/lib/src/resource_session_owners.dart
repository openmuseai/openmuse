import 'dart:async';
import 'package:flutter/foundation.dart';
import 'resource_capabilities.dart';

/// Format-independent checkpoint adapter; content knowledge stays in plugins.
abstract interface class ResourceOwnerCheckpoint implements Listenable {
  bool get dirty;
  Object? get failure;
  Future<void> flush();
  void dispose();
}

/// Checkpoints that can stop new application saves while a close tree freezes.
abstract interface class PreparedResourceCheckpoint
    implements ResourceOwnerCheckpoint {
  ResourceCheckpointCloseHold pauseForClose();
  Future<void> flushPrepared();
  Future<void> flushIfCurrent(bool Function() isCurrent);
}

/// A stable checkpoint keeps its subscriptions while a newly resolved grant
/// takes over the resource baseline. Applying the prepared binding is
/// synchronous and must not notify, save, or start editing a presentation.
abstract interface class RebindableResourceCheckpoint
    implements PreparedResourceCheckpoint {
  ResourceCheckpointAuthorityBinding prepareAuthorityBinding(
    ResourceEditorSession next,
  );
}

final class ResourceCheckpointAuthorityBinding {
  ResourceCheckpointAuthorityBinding({
    required this.apply,
    required this.cancel,
  });
  final VoidCallback apply, cancel;
}

/// Content-owned preparation of the current original resource state. The SDK
/// validates the new session; domains retain their own merge and editing rules.
final class ResourceOwnerAuthorityPreparation {
  ResourceOwnerAuthorityPreparation({
    required this.active,
    required this.commit,
    required this.cancel,
    required this.complete,
  });
  final bool Function() active;
  final VoidCallback commit, cancel, complete;
  bool get isActive => active();
}

/// Reference-counted pauses also cover overlapping close trees. Waiting public
/// flushes never enter the checkpoint's in-flight cache before the gate opens.
final class ResourceCheckpointBarrier {
  int _pauses = 0;
  Completer<void>? _gate;
  bool _privateResume = false;
  bool get isPaused => _pauses > 0;
  Future<void> get resumed => _gate?.future ?? Future.value();
  ResourceCheckpointCloseHold pause(
    Future<void>? inFlight,
    VoidCallback onResume, {
    VoidCallback? onPrivateResume,
  }) {
    _pauses++;
    _gate ??= Completer<void>();
    return _CheckpointHold(inFlight, (privateRecovery) {
      _privateResume |= privateRecovery;
      if (--_pauses != 0) return;
      final gate = _gate!;
      _gate = null;
      gate.complete();
      final privately = _privateResume;
      _privateResume = false;
      if (privately && onPrivateResume != null) {
        onPrivateResume();
      } else {
        onResume();
      }
    });
  }
}

final class _CheckpointHold implements ResourcePrivateRecoveryCheckpointHold {
  _CheckpointHold(this.inFlight, this.release);
  final Future<void>? inFlight;
  final void Function(bool privateRecovery) release;
  bool _released = false;
  @override
  Future<void> drain() async {
    try {
      await inFlight;
    } on Object {
      /* An existing rejected save is retried only by normal checkpointing. */
    }
  }

  @override
  void resume() {
    if (_released) return;
    _released = true;
    release(false);
  }

  @override
  void resumeAfterPrivateRecovery() {
    if (_released) return;
    _released = true;
    release(true);
  }
}

/// A content-domain preparation remains independent of the SDK. The existing
/// release adapter consumes the frozen ticket after all checkpoints succeed.
final class ResourceOwnerPreparation {
  ResourceOwnerPreparation({required this.active, required this.cancel});
  final bool Function() active;
  final VoidCallback cancel;
  bool get isActive => active();
}

/// The dataset's qualified ownership/lease state machine, shared with other
/// content editors without adding format knowledge to the Host or SDK.
class ResourceSessionOwners<T, C extends ResourceOwnerCheckpoint, M> {
  ResourceSessionOwners({
    required this.open,
    required this.release,
    required this.objectId,
    required this.realtime,
    required this.createCheckpoint,
    required this.controllerObjectId,
    required this.controllerEditable,
    required this.authorityEnded,
    this.prepareClose,
    this.supportsPrivateRecovery = false,
    this.prepareAuthority,
  });
  final Future<ResourceOwnerAuthorityPreparation> Function(
    T,
    ResourceEditorSession,
  )?
  prepareAuthority;
  final bool supportsPrivateRecovery;
  final Future<ResourceOwnerPreparation> Function(T)? prepareClose;
  final C Function(T, ResourceEditorSession) createCheckpoint;
  final String Function(T) controllerObjectId;
  final bool Function(T) controllerEditable;
  final void Function(T, M) authorityEnded;
  final Future<T> Function(ResourceEditorSession) open;
  final Future<void> Function(
    ResourceEditorSession,
    T, {
    Future<void> Function()? beforeRelease,
  })
  release;
  final String Function(List<int>) objectId;
  final M Function(ResourceSessionScope) realtime;
  final _entries = <ResourceSessionScope, _Entry<T, C, M>>{};
  bool _accepting = true;
  bool owns(ResourceSessionScope scope) => _entries.containsKey(scope);

  /// Owner lifetime only. Content access still requires its current provider
  /// and domain permission checks; this is not an authorization capability.
  bool hasActiveOwner(ResourceSessionScope scope) {
    final owner = _entries[scope]?.owner;
    return owner != null && !owner._disposed && owner.session.isActive;
  }

  void activate() => _accepting = true;

  Future<_Owner<T, C, M>> _open(ResourceEditorSession session) async {
    final retention = session.retain();
    T? controller;
    try {
      final opened = await open(session);
      controller = opened;
      final checkpoint = createCheckpoint(opened, session);
      return _Owner<T, C, M>(
        opened,
        session,
        checkpoint,
        retention,
        realtime(session.ownershipScope),
        authorityEnded,
      );
    } on Object {
      if (controller != null) await release(session, controller);
      retention.release();
      rethrow;
    }
  }

  Future<ResourceSessionLease<T, C, M>> acquire(
    ResourceEditorSession session,
  ) async {
    if (!_accepting || !session.isPresentationActive)
      throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
    final scope = session.ownershipScope;
    var entry = _entries[scope];
    if (entry?.closing != null) {
      await _waitClosing(entry!, session);
      return acquire(session);
    }
    if (entry == null) {
      final created = _Entry<T, C, M>();
      entry = created;
      _entries[scope] = created;
      created.ready = _open(session).then((owner) {
        created.owner = owner;
        return owner;
      });
    }
    entry.users++;
    var reserved = true;
    _Owner<T, C, M>? owner;
    ResourceSessionLease<T, C, M>? createdLease;
    VoidCallback? finishAuthority;
    try {
      owner = await entry.ready;
      // An acquisition reserved before preparation keeps the owner alive, but
      // must not mount a new editor while that owner is frozen.
      while (entry.closing != null) {
        await _waitClosing(entry, session);
        if (!identical(_entries[scope], entry)) {
          if (entry.users > 0) entry.users--;
          reserved = false;
          return acquire(session);
        }
      }
      if (!_accepting || !session.isPresentationActive)
        throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      if (!owner.session.isActive &&
          !identical(owner.session.lifetime, session.lifetime)) {
        // A fresh, separately authorized lifetime must never reuse the ended
        // account's live handle. Preserve its private state, release old leases
        // without another resource commit, then open from the new binding.
        await _closeEntry(scope, entry, owner);
        for (final old in entry.leases.toList()) {
          old._finish();
        }
        entry.leases.clear();
        entry.users = 0;
        reserved = false;
        return acquire(session);
      }
      if (!owner.session.isActive)
        throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      if (!identical(owner.session.lifetime, session.lifetime))
        throw const ResourceFailure('RESOURCE_SESSION_LIFETIME_MISMATCH');
      if (entry.authorityRecoveryRequired)
        throw const ResourceFailure('RESOURCE_AUTHORITY_RECOVERY_REQUIRED');
      if (controllerObjectId(owner.controller) != objectId(session.bytes))
        throw const ResourceFailure('RESOURCE_CONTENT_CHANGED');
      if (session.allowEditing &&
          !session.opened.fromSnapshot &&
          !controllerEditable(owner.controller)) {
        finishAuthority = await _adoptAuthority(scope, entry, owner, session);
        if (!_accepting || !session.isPresentationActive)
          throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      }
      if (prepareClose != null) {
        entry.participant ??= ResourceCloseParticipant(
          supportsPrivateRecovery:
              supportsPrivateRecovery &&
              owner.checkpoint is PreparedResourceCheckpoint,
          holdCheckpoint: owner.checkpoint is PreparedResourceCheckpoint
              ? (owner.checkpoint as PreparedResourceCheckpoint).pauseForClose
              : null,
          prepare: (sessions) => _prepareEntry(scope, entry!, owner!, sessions),
        );
      }
      late final ResourceSessionLease<T, C, M> lease;
      lease = ResourceSessionLease<T, C, M>._(owner, session, () async {
        await _release(scope, entry!, owner!, lease);
        entry.leases.remove(lease);
      }, participant: entry.participant);
      entry.leases.add(lease);
      createdLease = lease;
      lease._activate();
      finishAuthority?.call();
      finishAuthority = null;
      return lease;
    } on Object {
      finishAuthority?.call();
      if (!reserved) rethrow;
      if (createdLease != null) {
        if (entry.leases.remove(createdLease) && entry.users > 0) entry.users--;
        createdLease._finish();
      } else if (entry.users > 0) {
        entry.users--;
      }
      if (entry.users == 0) {
        if (owner != null && _accepting) {
          await _closeEntry(scope, entry, owner);
        } else if (owner == null && identical(_entries[scope], entry)) {
          _entries.remove(scope);
        }
      }
      rethrow;
    }
  }

  Future<VoidCallback> _adoptAuthority(
    ResourceSessionScope scope,
    _Entry<T, C, M> entry,
    _Owner<T, C, M> owner,
    ResourceEditorSession next,
  ) async {
    final checkpoint = owner.checkpoint;
    if (prepareAuthority == null || checkpoint is! RebindableResourceCheckpoint)
      throw const ResourceFailure('RESOURCE_READ_ONLY_OWNER');
    if (entry.closing != null || !identical(_entries[scope], entry))
      throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
    final gate = Completer<void>();
    entry.closing = gate.future;
    final hold = checkpoint.pauseForClose();
    ResourceOwnerAuthorityPreparation? preparation;
    ResourceCheckpointAuthorityBinding? binding;
    ResourceSessionRetention? retention;
    var finished = false;
    void finish() {
      if (finished) return;
      finished = true;
      entry.closing = null;
      hold.resume();
      if (!gate.isCompleted) gate.complete();
    }

    try {
      await hold.drain();
      if (!_accepting || !next.isPresentationActive || !owner.session.isActive)
        throw const ResourceFailure('RESOURCE_AUTHORITY_TRANSFER_EXPIRED');
      preparation = await prepareAuthority!(owner.controller, next);
      if (!_accepting ||
          !next.isPresentationActive ||
          !owner.session.isActive ||
          !identical(owner.session.lifetime, next.lifetime) ||
          owner.session.ownershipScope != next.ownershipScope ||
          !next.allowEditing ||
          next.opened.fromSnapshot ||
          !preparation.isActive)
        throw const ResourceFailure('RESOURCE_AUTHORITY_TRANSFER_EXPIRED');
      binding = checkpoint.prepareAuthorityBinding(next);
      retention = next.retain();
      preparation.commit();
      binding.apply();
      owner.replaceAuthority(next, retention);
      retention = null;
      preparation.complete();
      // The caller registers and activates the complete lease before resuming
      // autosave or releasing opens/closes queued on this gate.
      return finish;
    } on Object {
      // A faulty adapter must not leave acquisition/autosave permanently held.
      // If rollback itself fails, refuse further opens on this entry until its
      // existing owner is released rather than publish an uncertain authority.
      for (final cancel in <VoidCallback?>[
        binding?.cancel,
        preparation?.cancel,
        retention?.release,
      ]) {
        try {
          cancel?.call();
        } on Object {
          entry.authorityRecoveryRequired = true;
        }
      }
      finish();
      rethrow;
    }
  }

  Future<void> _waitClosing(
    _Entry<T, C, M> entry,
    ResourceEditorSession session,
  ) async {
    try {
      await entry.closing;
    } on Object {
      // A rejected checkpoint may have restored the same owner. The normal
      // acquisition checks below still enforce its authority and lifetime.
      if (!_accepting) rethrow;
    }
    final owner = await entry.ready;
    if (identical(owner.session.lifetime, session.lifetime) &&
        session.isPresentationActive) {
      // A waiting open resolved before the final checkpoint. Carry forward its
      // verified baseline before reopening a released owner; never recommit the
      // same content against that open's stale resource revision.
      session.observeSharedCheckpoint(owner.session);
    }
  }

  Future<_PreparedOwnerClose<T, C, M>> _prepareEntry(
    ResourceSessionScope scope,
    _Entry<T, C, M> entry,
    _Owner<T, C, M> owner,
    Set<ResourceEditorSession> sessions,
  ) async {
    if (entry.closing != null) {
      await entry.closing;
      return _prepareEntry(scope, entry, owner, sessions);
    }
    if (!_accepting || !identical(_entries[scope], entry))
      throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
    final leases = entry.leases
        .where((lease) => sessions.contains(lease.session))
        .toSet();
    if (leases.isEmpty ||
        sessions.any(
          (session) =>
              !leases.any((lease) => identical(lease.session, session)),
        ))
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    final gate = Completer<void>();
    entry.closing = gate.future;
    final prepared = _PreparedOwnerClose<T, C, M>(
      this,
      scope,
      entry,
      owner,
      leases,
      gate,
    );
    return entry.preparation = (() async {
      try {
        prepared.domain = await prepareClose!(owner.controller);
        if (!_accepting) throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
        return prepared;
      } on Object {
        prepared.cancel();
        rethrow;
      }
    })();
  }

  Future<void> _destroy(
    ResourceSessionScope scope,
    _Entry<T, C, M> entry,
    _Owner<T, C, M> owner, {
    bool flushCheckpoint = true,
  }) async {
    await release(
      owner.session,
      owner.controller,
      beforeRelease: () async {
        // The content adapter flushes its private recovery state before this
        // callback. A departed account must not commit another resource version.
        if (flushCheckpoint && owner.session.isActive) {
          final checkpoint = owner.checkpoint;
          if (checkpoint is PreparedResourceCheckpoint) {
            await checkpoint.flushPrepared();
          } else {
            await checkpoint.flush();
          }
        }
      },
    );
    owner.dispose();
    if (identical(_entries[scope], entry)) _entries.remove(scope);
  }

  Future<void> _closeEntry(
    ResourceSessionScope scope,
    _Entry<T, C, M> entry,
    _Owner<T, C, M> owner,
  ) {
    if (!identical(_entries[scope], entry)) return Future.value();
    return entry.closing ??= (() async {
      try {
        await _destroy(scope, entry, owner);
      } on Object {
        entry.closing = null;
        rethrow;
      }
    })();
  }

  Future<void> _release(
    ResourceSessionScope scope,
    _Entry<T, C, M> entry,
    _Owner<T, C, M> owner,
    ResourceSessionLease<T, C, M> lease,
  ) async {
    final preparing = entry.preparation;
    if (preparing != null) {
      await entry.closing;
      if (!identical(_entries[scope], entry)) return;
    }
    // A prepared tree may already have finalized this lease while its Widget
    // disposal was waiting on the gate. Do not consume a surviving sibling.
    if (!entry.leases.contains(lease)) return;
    // Do not drop a lease or its bounded callback when its checkpoint fails.
    if (owner.session.isActive) {
      final checkpoint = owner.checkpoint;
      if (checkpoint is PreparedResourceCheckpoint) {
        await checkpoint.flushIfCurrent(() => entry.leases.contains(lease));
      } else {
        await checkpoint.flush();
      }
    }
    // A tree can acquire its gate while this public flush is paused. It may
    // finalize this exact lease before the wait resumes; never consume a sibling.
    if (!entry.leases.contains(lease)) return;
    if (entry.users == 1) {
      await _closeEntry(scope, entry, owner);
    }
    if (entry.users > 0) entry.users--;
  }

  Future<void> close() async {
    _accepting = false;
    for (final pair in _entries.entries.toList()) {
      final entry = pair.value;
      final pending = entry.preparation;
      if (pending != null) {
        try {
          final prepared = await pending;
          if (prepared._committing != null) {
            await prepared._committing;
          } else {
            try {
              await prepared._checkpoint;
            } on Object {
              /* Recovery stays private. */
            }
            prepared.cancel();
          }
        } on Object {
          /* A failed preparation already released its gate. */
        }
      }
      if (pending == null && entry.closing != null) {
        await entry.closing;
      }
      final owner = await entry.ready;
      if (!identical(_entries[pair.key], entry)) continue;
      await _closeEntry(pair.key, entry, owner);
      for (final lease in entry.leases.toList()) {
        lease._finish();
      }
      entry.users = 0;
    }
  }
}

final class _Entry<T, C extends ResourceOwnerCheckpoint, M> {
  late Future<_Owner<T, C, M>> ready;
  _Owner<T, C, M>? owner;
  bool authorityRecoveryRequired = false;
  int users = 0;
  Future<void>? closing;
  ResourceCloseParticipant? participant;
  Future<_PreparedOwnerClose<T, C, M>>? preparation;
  final leases = <ResourceSessionLease<T, C, M>>{};
}

final class _Owner<T, C extends ResourceOwnerCheckpoint, M> {
  _Owner(
    this.controller,
    this.session,
    this.checkpoint,
    this.retention,
    this.realtime,
    this.authorityEnded,
  ) {
    session.lifetime?.addListener(_authorityChanged);
    _authorityChanged();
  }
  final T controller;
  ResourceEditorSession session;
  final C checkpoint;
  ResourceSessionRetention retention;
  final M realtime;
  final void Function(T, M) authorityEnded;
  bool _disposed = false;
  void replaceAuthority(
    ResourceEditorSession next,
    ResourceSessionRetention nextRetention,
  ) {
    if (!identical(session.lifetime, next.lifetime))
      throw const ResourceFailure('RESOURCE_SESSION_LIFETIME_MISMATCH');
    final previous = retention;
    session = next;
    retention = nextRetention;
    previous.release();
  }

  void _authorityChanged() {
    if (session.isActive) return;
    authorityEnded(controller, realtime);
  }

  void dispose() {
    if (_disposed) return;
    _disposed = true;
    session.lifetime?.removeListener(_authorityChanged);
    checkpoint.dispose();
    retention.release();
  }
}

/// A presentation has a local capability and close barrier, while the shared
/// owner retains one controller, realtime connection and commit version stream.
final class ResourceSessionLease<T, C extends ResourceOwnerCheckpoint, M> {
  ResourceSessionLease._(
    this._owner,
    this.session,
    this._release, {
    ResourceCloseParticipant? participant,
  }) : _participant = participant,
       _retention = session.retain() {
    _flush = flush;
    _closeBarrier = close;
  }

  // Register a complete lease in its owner before exposing callbacks or
  // notifying listeners; a listener is allowed to request closing immediately.
  void _activate() {
    session.flushPendingChanges = _flush;
    session.releaseForClose = _closeBarrier;
    session.closeParticipant = _participant;
    _owner.checkpoint.addListener(_observe);
    if (allowEditing && !session.editing) session.enterEditing();
    session.reportOpened();
    _observe();
  }

  final _Owner<T, C, M> _owner;
  final ResourceCloseParticipant? _participant;
  final ResourceEditorSession session;
  final ResourceSessionRetention _retention;
  final Future<void> Function() _release;
  late final Future<void> Function() _flush;
  late final Future<void> Function() _closeBarrier;
  Future<void> flush() {
    final checkpoint = _owner.checkpoint;
    bool current() => !_closed && session.isPresentationActive;
    if (checkpoint is PreparedResourceCheckpoint)
      return checkpoint.flushIfCurrent(current);
    return current() ? checkpoint.flush() : Future.value();
  }

  T get controller => _owner.controller;
  C get checkpoint => _owner.checkpoint;
  M get realtime => _owner.realtime;
  bool get allowEditing =>
      !_closed &&
      session.allowEditing &&
      !session.opened.fromSnapshot &&
      session.isPresentationActive;
  bool get isClosed => _closed;
  bool _closed = false;
  Future<void>? _closing;
  void _observe() {
    if (_closed || !session.isActive) return;
    if (!identical(session, _owner.session))
      session.observeSharedCheckpoint(_owner.session);
    session.markDirty(checkpoint.dirty);
  }

  Future<void> close() {
    if (_closed) return Future.value();
    return _closing ??= (() async {
      try {
        await _release();
        _finish();
      } on Object {
        _closing = null;
        rethrow;
      }
    })();
  }

  void _finish() {
    if (_closed) return;
    _closed = true;
    checkpoint.removeListener(_observe);
    if (identical(session.closeParticipant, _participant))
      session.closeParticipant = null;
    if (identical(session.releaseForClose, _closeBarrier))
      session.releaseForClose = null;
    if (!identical(session, _owner.session) &&
        identical(session.flushPendingChanges, _flush))
      session.flushPendingChanges = null;
    _retention.release();
  }
}

/// One tree preparation holds the entry's acquisition/release gate. Its selected
/// leases can close without destroying a sibling presentation outside the tree.
final class _PreparedOwnerClose<T, C extends ResourceOwnerCheckpoint, M>
    implements PreparedPrivateRecoveryClose {
  _PreparedOwnerClose(
    this.pool,
    this.scope,
    this.entry,
    this.owner,
    this.leases,
    this.gate,
  );
  final ResourceSessionOwners<T, C, M> pool;
  final ResourceSessionScope scope;
  final _Entry<T, C, M> entry;
  final _Owner<T, C, M> owner;
  final Set<ResourceSessionLease<T, C, M>> leases;
  final Completer<void> gate;
  ResourceOwnerPreparation? domain;
  Future<void>? _checkpoint, _committing;
  bool _finished = false;
  @override
  bool get isActive =>
      !_finished &&
      domain?.isActive == true &&
      identical(pool._entries[scope], entry);
  @override
  Future<void> checkpoint() => _checkpoint ??= (() async {
    if (!isActive)
      throw const ResourceFailure('RESOURCE_CLOSE_PREPARATION_EXPIRED');
    if (owner.session.isActive) {
      final checkpoint = owner.checkpoint;
      if (checkpoint is PreparedResourceCheckpoint) {
        await checkpoint.flushPrepared();
      } else {
        await checkpoint.flush();
      }
    }
  })();
  void _finishGate() {
    _finished = true;
    entry.preparation = null;
    entry.closing = null;
    if (!gate.isCompleted) gate.complete();
  }

  @override
  void cancel() {
    if (_committing != null) return;
    _cancel();
  }

  void _cancel() {
    if (_finished) return;
    if (domain?.isActive == true) domain!.cancel();
    _finishGate();
  }

  @override
  Future<void> commit() => _committing ??= _commit();
  @override
  Future<void> commitPrivateRecovery() {
    if (!pool.supportsPrivateRecovery)
      throw const ResourceFailure('RESOURCE_PRIVATE_RECOVERY_UNSUPPORTED');
    return _committing ??= _commit(privateRecoveryOnly: true);
  }

  Future<void> _commit({bool privateRecoveryOnly = false}) async {
    if (!isActive)
      throw const ResourceFailure('RESOURCE_CLOSE_PREPARATION_EXPIRED');
    try {
      if (entry.users == leases.length) {
        // Existing typed release consumes the prepared domain ticket. Its final
        // checkpoint sees the frozen, already accepted state and cannot race input.
        await pool._destroy(
          scope,
          entry,
          owner,
          flushCheckpoint: !privateRecoveryOnly,
        );
      } else {
        domain!.cancel();
      }
      for (final lease in leases) {
        entry.leases.remove(lease);
        entry.users--;
        lease._finish();
      }
      _finishGate();
    } on Object {
      _cancel();
      rethrow;
    }
  }
}
