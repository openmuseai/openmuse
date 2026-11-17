import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

/// Application persistence for one content owner. Widgets and projections do
/// not own a separate provider commit baseline or a private save timer.
final class SmartDocumentResourceCheckpoint extends ChangeNotifier
    implements RebindableResourceCheckpoint {
  SmartDocumentResourceCheckpoint(
    this.controller,
    ResourceEditorSession session, {
    required String savedState,
    required this.encode,
  }) : _session = session,
       _savedState = savedState {
    if (session.allowEditing && !session.opened.fromSnapshot)
      session.enterEditing();
    session.flushPendingChanges = flush;
    controller.addListener(_changed);
    session.reportOpened();
    _changed();
  }
  final SmartDocumentController controller;
  ResourceEditorSession _session;
  ResourceEditorSession get session => _session;
  final List<int> Function(Map<String, dynamic>) encode;
  String _savedState;
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
    final previous = session, saved = _savedState, priorFailure = failure;
    final previousFailedContent = _failedContent,
        previousDeferredContent = _deferredContent;
    final fresh =
        (jsonDecode(utf8.decode(next.bytes)) as Map)['docState'] as String;
    var applied = false;
    return ResourceCheckpointAuthorityBinding(
      apply: () {
        _session = next;
        _savedState = fresh;
        failure = null;
        _failedContent = null;
        _deferredContent = null;
        applied = true;
      },
      cancel: () {
        if (!applied || !identical(_session, next)) return;
        _session = previous;
        _savedState = saved;
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
        _deferredContent = controller.snapshot['docState'] as String;
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

  @override
  bool get dirty => session.dirty;
  void _changed() {
    if (_disposed || !session.isActive || !session.editing) {
      _timer?.cancel();
      return;
    }
    final current = controller.snapshot['docState'] as String;
    if (_deferredContent != null && current != _deferredContent)
      _deferredContent = null;
    session.markDirty(
      controller.contentFailure != null || current != _savedState,
    );
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

  @override
  Future<void> flushPrepared() => _saving ??= (() async {
    if (!session.editing) return;
    String? attempted;
    try {
      await controller.flush();
      final state = controller.snapshot;
      attempted = state['docState'] as String;
      if (state['docState'] != _savedState) {
        if (!await session.commitCheckpointIfActive(encode(state))) return;
        _savedState = state['docState'] as String;
      }
      failure = null;
      _failedContent = null;
      _deferredContent = null;
      _changed();
    } on Object catch (error) {
      failure = error;
      _failedContent = attempted ?? controller.snapshot['docState'] as String;
      _timer?.cancel();
      rethrow;
    } finally {
      if (!_disposed) notifyListeners();
    }
  })().whenComplete(() => _saving = null);
  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    _timer?.cancel();
    controller.removeListener(_changed);
    session.flushPendingChanges = null;
    super.dispose();
  }
}
