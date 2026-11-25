import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Presentation state only; original API delivery/review stays with the
/// account-scoped service, and content editors remain owned by their handlers.
final class DocumentCreationController extends ChangeNotifier {
  DocumentCreationController({
    required this.authentication,
    required this.service,
    required this.workspaceId,
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_accountChanged);
    _subscription = service.documentCreationChanges.listen((_) {
      if (active) notifyListeners();
    }, onDone: retire);
    name = pending?.name ?? '';
    if (!authentication.snapshot.isAuthenticated || _subject == null)
      retired = true;
  }

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final String workspaceId;
  final String? _subject;
  late final StreamSubscription<void> _subscription;
  bool _disposed = false, retired = false, _task = false;
  String name = '';
  String? parentId, failure, _submittedId;
  List<AppFlowyDocumentParent> parents = const [];
  bool get active => !_disposed && !retired;
  AppFlowyDocumentCreation? get pending =>
      active ? service.pendingDocumentCreation(workspaceId) : null;
  bool get busy => _task || pending?.busy == true;
  AppFlowyDocumentParent? get parent =>
      parents.where((p) => p.viewId == parentId).firstOrNull;
  bool get canSubmit =>
      active &&
      !busy &&
      pending == null &&
      parent != null &&
      name.trim().isNotEmpty;

  void _accountChanged() {
    if (!authentication.snapshot.isAuthenticated ||
        authentication.snapshot.identity?.subject != _subject)
      retire();
  }

  void retire({bool deferNotification = false}) {
    if (retired) return;
    final before = service.pendingDocumentCreation(workspaceId);
    if (before != null && before.documentId == _submittedId) {
      try {
        service.cancelDocumentCreationPreflight(before);
      } on Object {
        /* already retired */
      }
    }
    retired = true;
    name = '';
    parentId = null;
    parents = const [];
    failure = 'source';
    _task = false;
    // A source can retire synchronously from its parent's didUpdateWidget.
    // Fence commands immediately; publish to the separate modal after that
    // frame's build has unwound.
    if (deferNotification) {
      scheduleMicrotask(() {
        if (!_disposed) notifyListeners();
      });
    } else if (!_disposed) {
      notifyListeners();
    }
  }

  void setName(String value) {
    if (!active || busy || pending != null) return;
    name = value;
    failure = null;
    notifyListeners();
  }

  void selectParent(String? value) {
    if (!active ||
        busy ||
        pending != null ||
        !parents.any((p) => p.viewId == value))
      return;
    parentId = value;
    failure = null;
    notifyListeners();
  }

  Future<T?> _run<T>(Future<T> Function() action) async {
    if (!active || busy) return null;
    _task = true;
    failure = null;
    notifyListeners();
    try {
      final result = await action();
      return active ? result : null;
    } on AppFlowyDocumentCreationBlocked catch (error) {
      if (active)
        failure = switch (error.code) {
          'READ_ONLY' => 'readonly',
          'TARGET_CHANGED' || 'SOURCE_ENDED' => 'changed',
          'REVIEW_REQUIRED' || 'BUSY' => 'review',
          _ => 'load',
        };
    } on ArgumentError {
      if (active) failure = 'name';
    } on CloudServiceException catch (error) {
      if (active)
        failure = error.code == CloudServiceErrorCode.unauthorized
            ? 'readonly'
            : 'load';
    } on Object {
      if (active) failure = 'load';
    } finally {
      if (active) {
        _task = false;
        notifyListeners();
      }
    }
    return null;
  }

  Future<void> _load() async {
    final before = pending;
    if (before != null) {
      parents = [before.parent];
      parentId = before.parent.viewId;
      return;
    }
    final targets = await service.documentCreationParents(workspaceId);
    if (!active) return;
    parents = targets;
    if (!parents.any((p) => p.viewId == parentId)) {
      parentId =
          parents.where((p) => p.isSpace).firstOrNull?.viewId ??
          parents.firstOrNull?.viewId;
    }
  }

  Future<void> load() async {
    await _run(_load);
  }

  Future<AppFlowyDocumentPage?> submit() async {
    if (!canSubmit) return null;
    return _run<AppFlowyDocumentPage?>(() async {
      final future = service.createDocument(parent!, name);
      _submittedId = service.pendingDocumentCreation(workspaceId)?.documentId;
      final result = await future;
      if (!active) return null;
      return result.status == AppFlowyDocumentCreationStatus.ready
          ? result.page
          : null;
    });
  }

  Future<void> check(AppFlowyDocumentCreation before) async {
    await _run(() => service.reconcileDocumentCreation(before));
  }

  AppFlowyDocumentPage? readyPage(AppFlowyDocumentCreation before) {
    final current = pending;
    return active &&
            !busy &&
            current?.documentId == before.documentId &&
            current?.status == AppFlowyDocumentCreationStatus.ready
        ? current?.page
        : null;
  }

  Future<void> finishReview(AppFlowyDocumentCreation before) async {
    await _run(() async {
      service.finishDocumentCreation(before, reviewed: true);
      name = '';
      await _load();
    });
  }

  @override
  void dispose() {
    _disposed = true;
    retire();
    authentication.removeListener(_accountChanged);
    unawaited(_subscription.cancel());
    super.dispose();
  }
}
