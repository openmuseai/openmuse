import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Application adapter shared by contributed surfaces. A resource commit is a
/// checkpoint of the independent dataset, never of its containing document.
final class DatasetResourceCheckpoint extends ChangeNotifier
    implements RebindableResourceCheckpoint {
  DatasetResourceCheckpoint(this.controller, ResourceEditorSession session)
    : _session = session {
    // Compare against resource bytes, not just the newer private recovery cache.
    _saved = jsonEncode(jsonDecode(utf8.decode(session.bytes)));
    if (session.allowEditing && !session.opened.fromSnapshot)
      session.enterEditing();
    session.flushPendingChanges = flush;
    controller.addListener(_changed);
    session.reportOpened();
    _changed();
  }
  final DatasetController controller;
  ResourceEditorSession _session;
  ResourceEditorSession get session => _session;
  late String _saved;
  Object? failure;
  String? _failedContent, _deferredContent;
  Timer? _timer;
  Future<void>? _saving;
  bool _disposed = false;
  final _closeBarrier = ResourceCheckpointBarrier();
  @override
  ResourceCheckpointAuthorityBinding prepareAuthorityBinding(
    ResourceEditorSession next,
  ) {
    if (_disposed ||
        !_closeBarrier.isPaused ||
        _saving != null ||
        !next.isPresentationActive ||
        !next.allowEditing ||
        next.opened.fromSnapshot ||
        next.ownershipScope != session.ownershipScope ||
        !identical(next.lifetime, session.lifetime))
      throw const ResourceFailure('RESOURCE_AUTHORITY_TRANSFER_EXPIRED');
    final previous = session, saved = _saved, priorFailure = failure;
    final previousFailedContent = _failedContent,
        previousDeferredContent = _deferredContent;
    final fresh = jsonEncode(jsonDecode(utf8.decode(next.bytes)));
    var applied = false;
    return ResourceCheckpointAuthorityBinding(
      apply: () {
        _session = next;
        _saved = fresh;
        failure = null;
        _failedContent = null;
        _deferredContent = null;
        applied = true;
      },
      cancel: () {
        if (!applied || !identical(_session, next)) return;
        _session = previous;
        _saved = saved;
        failure = priorFailure;
        _failedContent = previousFailedContent;
        _deferredContent = previousDeferredContent;
      },
    );
  }

  @override
  ResourceCheckpointCloseHold pauseForClose() {
    _timer?.cancel();
    return _closeBarrier.pause(
      _saving,
      _changed,
      onPrivateResume: () {
        if (_disposed) return;
        _deferredContent = jsonEncode(controller.bundle);
        _changed();
      },
    );
  }

  @override
  Future<void> flush() => flushIfCurrent(() => true);
  @override
  Future<void> flushIfCurrent(bool Function() isCurrent) async {
    while (_closeBarrier.isPaused) {
      await _closeBarrier.resumed;
    }
    if (_disposed || !isCurrent()) return;
    await flushPrepared();
  }

  bool get dirty => session.dirty;
  void _changed() {
    if (_disposed || !session.isActive || !session.editing) {
      _timer?.cancel();
      return;
    }
    final current = jsonEncode(controller.bundle);
    if (_deferredContent != null && current != _deferredContent)
      _deferredContent = null;
    session.markDirty(current != _saved);
    notifyListeners();
    _timer?.cancel();
    // Resuming a close or receiving a status notification is not an explicit
    // retry of the same rejected payload. New accepted content may autosave;
    // an explicit flush always reaches the original checkpoint port.
    if (!session.dirty ||
        controller.isClosed ||
        _closeBarrier.isPaused ||
        (failure != null && _failedContent == current) ||
        _deferredContent == current)
      return;
    _timer?.cancel();
    _timer = Timer(
      const Duration(milliseconds: 250),
      () => unawaited(flush().catchError((Object _) {})),
    );
  }

  Future<void> flushPrepared() => _saving ??= (() async {
    if (!session.editing) return;
    String? attempted;
    try {
      await controller.flush();
      final bundle = controller.bundle, encoded = jsonEncode(controller.bundle);
      attempted = encoded;
      if (encoded != _saved) {
        if (!await session.commitCheckpointIfActive(
          utf8.encode(jsonEncode(bundle)),
        ))
          return;
        _saved = encoded;
      }
      failure = null;
      _failedContent = null;
      _deferredContent = null;
      _changed();
    } on Object catch (e) {
      failure = e;
      _failedContent = attempted ?? jsonEncode(controller.bundle);
      _timer?.cancel();
      rethrow;
    } finally {
      if (!_disposed) notifyListeners();
    }
  })().whenComplete(() => _saving = null);
  @override
  void dispose() {
    _disposed = true;
    _timer?.cancel();
    controller.removeListener(_changed);
    session.flushPendingChanges = null;
    super.dispose();
  }
}
