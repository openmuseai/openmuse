import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

/// Application adapter between two existing owners. Neither content domain
/// imports the other; no editor session, network actor or checkpoint is opened.
final class RowBodyMetadataObserver {
  RowBodyMetadataObserver({
    required this.document,
    required this.database,
    required this.viewId,
    required this.rowId,
    required this.documentId,
    required this.canObserve,
    this.authorityChanges,
    this.onFailure,
  }) {
    document.addListener(retry);
    database.addListener(retry);
    authorityChanges?.addListener(retry);
    retry();
  }
  final SmartDocumentController document;
  final db.DatasetController database;
  final String viewId, rowId, documentId;
  final bool Function() canObserve;
  final Listenable? authorityChanges;
  final void Function(Object?)? onFailure;
  bool _disposed = false, _pending = false;
  Future<void>? _task;
  Object? failure;

  Future<void> retryCheckpoint() async {
    if (_disposed || !canObserve() || !document.canWrite || !database.canWrite)
      return;
    try {
      await database.flush();
      retry();
    } on Object catch (error) {
      if (_disposed) return;
      failure = error;
      onFailure?.call(error);
    }
  }

  void retry() {
    if (_disposed) return;
    _pending = true;
    if (_task != null) return;
    _task = Future<void>.microtask(_drain).whenComplete(() {
      _task = null;
      if (_pending && !_disposed) retry();
    });
  }

  Future<void> _drain() async {
    while (!_disposed && _pending) {
      _pending = false;
      try {
        if (!canObserve() ||
            !document.canWrite ||
            !database.canWrite ||
            document.objectId != documentId)
          continue;
        final empty = document.originalContentEmpty;
        if (empty == null) continue;
        await database.updateRowDocumentContentState(
          viewId,
          rowId,
          documentId,
          empty,
        );
        if (!_disposed && failure != null) {
          failure = null;
          onFailure?.call(null);
        }
      } on Object catch (error) {
        if (_disposed) return;
        failure = error;
        onFailure?.call(error);
        // A failed private checkpoint is retried by an explicit retry or a
        // fresh owner/authority notification, never a busy POST/write loop.
        _pending = false;
      }
    }
  }

  void dispose() {
    if (_disposed) return;
    _disposed = true;
    document.removeListener(retry);
    database.removeListener(retry);
    authorityChanges?.removeListener(retry);
  }
}
