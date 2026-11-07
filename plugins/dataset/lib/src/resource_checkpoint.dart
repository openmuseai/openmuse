import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Application adapter shared by contributed surfaces. A resource commit is a
/// checkpoint of the independent dataset, never of its containing document.
final class DatasetResourceCheckpoint extends ChangeNotifier {
  DatasetResourceCheckpoint(this.controller, this.session) {
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
  final ResourceEditorSession session;
  late String _saved;
  Object? failure;
  Timer? _timer;
  Future<void>? _saving;
  bool _disposed = false;
  void _changed() {
    if (_disposed || !session.editing) return;
    session.markDirty(jsonEncode(controller.bundle) != _saved);
    if (!session.dirty) return;
    _timer?.cancel();
    _timer = Timer(
      const Duration(milliseconds: 250),
      () => unawaited(flush().catchError((Object _) {})),
    );
  }

  Future<void> flush() => _saving ??= (() async {
    if (!session.editing) return;
    try {
      await controller.flush();
      final bundle = controller.bundle, encoded = jsonEncode(controller.bundle);
      if (encoded != _saved) {
        await session.commit(utf8.encode(jsonEncode(bundle)));
        _saved = encoded;
      }
      failure = null;
      _changed();
    } on Object catch (e) {
      failure = e;
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
