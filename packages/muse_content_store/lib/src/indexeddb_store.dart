import 'dart:async';
import 'dart:convert';
import 'dart:js_interop';
import 'package:web/web.dart' as web;
import '../snapshot_store.dart';

/// A successful write means the IndexedDB transaction completed, not just put().
/// Browser eviction/power-loss durability is a separate capability and is not promised.
final class IndexedDbSnapshotStore implements RemovableSnapshotStore {
  IndexedDbSnapshotStore({
    required this.namespace,
    required this.objectId,
    this.databasePrefix = 'muse-snapshots:',
  });
  final String namespace, objectId;
  final String databasePrefix;

  Future<web.IDBDatabase> _open() async {
    if (namespace.isEmpty || objectId.isEmpty) {
      throw const SnapshotStoreFailure('INVALID_STORAGE_SCOPE');
    }
    final complete = Completer<web.IDBDatabase>();
    final request = web.window.indexedDB.open('$databasePrefix$namespace', 1);
    request.onupgradeneeded = ((web.Event _) {
      final db = request.result as web.IDBDatabase;
      db.createObjectStore('snapshots');
    }).toJS;
    request.onsuccess = ((web.Event _) => complete.complete(
      request.result as web.IDBDatabase,
    )).toJS;
    request.onerror = ((web.Event _) => complete.completeError(
      const SnapshotStoreFailure('BROWSER_STORAGE_UNAVAILABLE'),
    )).toJS;
    request.onblocked = ((web.Event _) => complete.completeError(
      const SnapshotStoreFailure('BROWSER_STORAGE_BLOCKED'),
    )).toJS;
    return complete.future.timeout(const Duration(seconds: 15));
  }

  @override
  Future<Map<String, dynamic>?> read() async {
    final db = await _open();
    try {
      final done = Completer<Map<String, dynamic>?>();
      final transaction = db.transaction('snapshots'.toJS, 'readonly');
      final request = transaction.objectStore('snapshots').get(objectId.toJS);
      request.onsuccess = ((web.Event _) {
        try {
          final value = request.result;
          done.complete(
            value == null || value.isUndefinedOrNull
                ? null
                : (jsonDecode((value as JSString).toDart) as Map)
                      .cast<String, dynamic>(),
          );
        } on Object catch (error) {
          done.completeError(error);
        }
      }).toJS;
      request.onerror = ((web.Event _) => done.completeError(
        const SnapshotStoreFailure('BROWSER_STORAGE_READ_FAILED'),
      )).toJS;
      return await done.future;
    } finally {
      db.close();
    }
  }

  @override
  Future<void> write(Map<String, dynamic> snapshot) => _change(
    (store) => store.put(jsonEncode(snapshot).toJS, objectId.toJS),
    'BROWSER_STORAGE_WRITE_FAILED',
  );

  @override
  Future<void> remove() => _change(
    (store) => store.delete(objectId.toJS),
    'BROWSER_STORAGE_REMOVE_FAILED',
  );

  Future<void> _change(
    void Function(web.IDBObjectStore) change,
    String failureCode,
  ) async {
    final db = await _open();
    try {
      final done = Completer<void>();
      final transaction = db.transaction('snapshots'.toJS, 'readwrite');
      transaction.oncomplete = ((web.Event _) {
        if (!done.isCompleted) done.complete();
      }).toJS;
      void fail(web.Event _) {
        if (!done.isCompleted)
          done.completeError(SnapshotStoreFailure(failureCode));
      }

      transaction.onerror = fail.toJS;
      transaction.onabort = fail.toJS;
      change(transaction.objectStore('snapshots'));
      await done.future;
    } finally {
      db.close();
    }
  }
}
