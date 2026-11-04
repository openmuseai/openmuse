import 'dart:async';
import 'dart:js_interop';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:web/web.dart' as web;

/// IndexedDB commits the signed snapshot and its verified resource bytes in one
/// transaction. Different deployment identities never share a record.
final class BrowserConfigStore implements ConfigStore {
  BrowserConfigStore(this.identity);
  final String identity;
  Future<web.IDBDatabase> _open() {
    final completed = Completer<web.IDBDatabase>();
    final request = web.window.indexedDB.open('openmuse.runtime-config.v1', 1);
    request.onupgradeneeded = ((web.Event _) {
      (request.result as web.IDBDatabase).createObjectStore('records');
    }).toJS;
    request.onsuccess = ((web.Event _) {
      final database = request.result as web.IDBDatabase;
      database.onversionchange = ((web.Event _) => database.close()).toJS;
      if (!completed.isCompleted) completed.complete(database);
    }).toJS;
    request.onerror = ((web.Event _) {
      if (!completed.isCompleted) {
        completed.completeError(
          StateError('Runtime cache could not be opened'),
        );
      }
    }).toJS;
    request.onblocked = ((web.Event _) {
      if (!completed.isCompleted) {
        completed.completeError(StateError('Runtime cache upgrade is blocked'));
      }
    }).toJS;
    return completed.future;
  }

  Future<T> _transaction<T>(
    String mode,
    void Function(web.IDBObjectStore, void Function(T)) action,
  ) async {
    final database = await _open();
    final completed = Completer<T>();
    final transaction = database.transaction('records'.toJS, mode);
    T? result;
    transaction.oncomplete = ((web.Event _) {
      database.close();
      if (!completed.isCompleted) completed.complete(result as T);
    }).toJS;
    void failed(web.Event _) {
      database.close();
      if (!completed.isCompleted) {
        completed.completeError(StateError('Runtime cache transaction failed'));
      }
    }

    transaction.onerror = failed.toJS;
    transaction.onabort = failed.toJS;
    try {
      action(transaction.objectStore('records'), (value) => result = value);
    } catch (error) {
      transaction.abort();
      database.close();
      rethrow;
    }
    return completed.future;
  }

  @override
  Future<String?> read() => _transaction<String?>('readonly', (store, finish) {
    final request = store.get(identity.toJS);
    request.onsuccess = ((web.Event _) {
      final value = request.result;
      finish(
        value == null || value.isUndefined ? null : (value as JSString).toDart,
      );
    }).toJS;
  });
  @override
  Future<void> commit(String record) =>
      _transaction<bool>('readwrite', (store, finish) {
        if (record.length > 8 * 1024 * 1024) {
          throw const FormatException('Runtime cache is too large');
        }
        store.put(record.toJS, identity.toJS);
        finish(true);
      }).then((_) {});
}
