import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Presentation only. Original Folder writes and outcome review are owned by
/// the account-scoped service, independently of the live document owner.
final class PageRenameController extends ChangeNotifier {
  PageRenameController({
    required this.authentication,
    required this.service,
    required this.page,
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_accountChanged);
    _subscription = service.pageRenameChanges.listen((_) {
      if (active) notifyListeners();
    }, onDone: retire);
    name = pending?.name ?? page.name;
    if (!authentication.snapshot.isAuthenticated || _subject == null)
      retired = true;
  }
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final AppFlowyDocumentPage page;
  final String? _subject;
  late final StreamSubscription<void> _subscription;
  AppFlowyPageRenameTarget? target;
  String name = '', failure = '';
  String? _submitted;
  bool retired = false, _disposed = false, _task = false;
  bool get active => !retired && !_disposed;
  AppFlowyPageRename? get pending =>
      active ? service.pendingPageRename(page.workspaceId, page.viewId) : null;
  bool get busy => _task || pending?.busy == true;
  bool get canSubmit =>
      active &&
      !busy &&
      pending == null &&
      target != null &&
      name.trim().isNotEmpty &&
      name.trim() != target!.page.name;
  void _accountChanged() {
    if (!authentication.snapshot.isAuthenticated ||
        authentication.snapshot.identity?.subject != _subject)
      retire();
  }

  void retire({bool deferNotification = false}) {
    if (retired) return;
    final before = service.pendingPageRename(page.workspaceId, page.viewId);
    if (before != null && before.operationId == _submitted) {
      try {
        service.cancelPageRenamePreflight(before);
      } on Object {
        /* already retired */
      }
    }
    retired = true;
    name = '';
    target = null;
    failure = 'source';
    _task = false;
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
    failure = '';
    notifyListeners();
  }

  Future<T?> _run<T>(Future<T> Function() action) async {
    if (!active || busy) return null;
    _task = true;
    failure = '';
    notifyListeners();
    try {
      final value = await action();
      return active ? value : null;
    } on AppFlowyPageRenameBlocked catch (error) {
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
      target = before.target;
      name = before.name;
      return;
    }
    final current = await service.preparePageRename(
      page.workspaceId,
      page.viewId,
    );
    if (!active) return;
    if (current.page is! AppFlowyDocumentPage)
      throw const AppFlowyPageRenameBlocked('TARGET_CHANGED');
    target = current;
    name = current.page.name;
  }

  Future<void> load() async {
    await _run(_load);
  }

  Future<AppFlowyWorkspacePage?> submit() async {
    if (!canSubmit) return null;
    return _run<AppFlowyWorkspacePage?>(() async {
      final operation = service.renamePage(target!, name);
      _submitted = service
          .pendingPageRename(page.workspaceId, page.viewId)
          ?.operationId;
      final result = await operation;
      return active && result.status == AppFlowyPageRenameStatus.ready
          ? result.observed
          : null;
    });
  }

  Future<void> check(AppFlowyPageRename before) async {
    await _run(() => service.reconcilePageRename(before));
  }

  AppFlowyWorkspacePage? confirmed(AppFlowyPageRename before) {
    final current = pending;
    return active &&
            !busy &&
            current?.operationId == before.operationId &&
            current?.status == AppFlowyPageRenameStatus.ready
        ? current?.observed
        : null;
  }

  void finishConfirmed(AppFlowyPageRename before) {
    if (confirmed(before) != null) service.finishPageRename(before);
  }

  Future<void> finishReview(AppFlowyPageRename before) async {
    await _run(() async {
      service.finishPageRename(before, reviewed: true);
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
