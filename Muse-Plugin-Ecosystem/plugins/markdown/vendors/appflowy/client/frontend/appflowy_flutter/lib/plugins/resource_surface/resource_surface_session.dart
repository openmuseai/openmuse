import 'dart:io';

typedef MuseResourceSurfaceFlush = Future<void> Function();

/// Lets the Host version actions flush an engine's in-memory buffer back to
/// the original file before capturing or switching snapshots.
final class MuseResourceSurfaceSession {
  MuseResourceSurfaceSession._();

  static final MuseResourceSurfaceSession instance =
      MuseResourceSurfaceSession._();

  final Map<String, MuseResourceSurfaceFlush> _flushers = {};
  final Map<String, bool> _bufferDirty = {};

  void attach(File file, {required MuseResourceSurfaceFlush flush}) {
    _flushers[_key(file)] = flush;
  }

  void detach(File file) {
    final key = _key(file);
    _flushers.remove(key);
    _bufferDirty.remove(key);
  }

  void markDirty(File file) {
    _bufferDirty[_key(file)] = true;
  }

  void clearDirty(File file) {
    _bufferDirty[_key(file)] = false;
  }

  bool isBufferDirty(File file) => _bufferDirty[_key(file)] ?? false;

  Future<void> flush(File file) async {
    final flusher = _flushers[_key(file)];
    if (flusher != null) await flusher();
  }

  String _key(File file) => file.absolute.path;
}
