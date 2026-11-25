part of 'appflowy_cloud_workspace_service.dart';

/// Original visible Folder view, bound to the service/account that read it.
final class AppFlowyPageRenameTarget {
  AppFlowyPageRenameTarget._(this._owner, this._account, this.page);
  final AppFlowyCloudWorkspaceService _owner;
  final (String?, int) _account;
  final AppFlowyWorkspacePage page;
}

enum AppFlowyPageRenameStatus {
  preflight,
  sending,
  checking,
  unconfirmed,
  ready,
}

final class AppFlowyPageRenameBlocked implements Exception {
  const AppFlowyPageRenameBlocked(this.code);
  final String code;
}

final class AppFlowyPageRename {
  AppFlowyPageRename._(
    this._owner,
    this._account,
    this.operationId,
    this.target,
    this.name,
    this.status,
    this.serverReturnedSuccess,
    this.observed,
  );
  final AppFlowyCloudWorkspaceService _owner;
  final (String?, int) _account;
  final String operationId, name;
  final AppFlowyPageRenameTarget target;
  final AppFlowyPageRenameStatus status;
  final bool serverReturnedSuccess;
  final AppFlowyWorkspacePage? observed;
  bool get busy => {
    AppFlowyPageRenameStatus.preflight,
    AppFlowyPageRenameStatus.sending,
    AppFlowyPageRenameStatus.checking,
  }.contains(status);
}

/// Only a current original GET may publish a display-name observation. This is
/// metadata, not permission, a content checkpoint, or a compare-and-set token.
final class AppFlowyPageNameObservation {
  AppFlowyPageNameObservation._(
    this._owner,
    this._account,
    this.page,
    this._version,
  );
  final AppFlowyCloudWorkspaceService _owner;
  final (String?, int) _account;
  final AppFlowyWorkspacePage page;
  final int _version;
}

final class _PageRenameIntent {
  _PageRenameIntent(this.owner, this.target, this.name)
    : id = const Uuid().v4();
  final AppFlowyCloudWorkspaceService owner;
  final AppFlowyPageRenameTarget target;
  final String name, id;
  AppFlowyPageRenameStatus status = AppFlowyPageRenameStatus.preflight;
  bool serverReturnedSuccess = false;
  AppFlowyWorkspacePage? observed;
  (String, String) get key => (target.page.workspaceId, target.page.viewId);
  AppFlowyPageRename get snapshot => AppFlowyPageRename._(
    owner,
    target._account,
    id,
    target,
    name,
    status,
    serverReturnedSuccess,
    observed,
  );
}

extension AppFlowyPageRenameApi on AppFlowyCloudWorkspaceService {
  void _publishPageNames(
    String workspaceId,
    List<AppFlowyWorkspacePage> pages,
    (String?, int) account,
    int version,
  ) {
    _checkAccount(account);
    if (_pageNameChanges.isClosed || _pageNameReads[workspaceId] != version)
      return;
    _pageNameSnapshots.remove(workspaceId);
    _pageNameSnapshots[workspaceId] = (
      version: version,
      account: account,
      pages: pages,
    );
    while (_pageNameSnapshots.length > 32) {
      _pageNameSnapshots.remove(_pageNameSnapshots.keys.first);
    }
    for (final page in pages) {
      _pageNameChanges.add(
        AppFlowyPageNameObservation._(this, account, page, version),
      );
    }
  }

  ({List<AppFlowyWorkspacePage> pages, int version})
  _preferObservedPageSnapshot(
    String workspace,
    List<AppFlowyWorkspacePage> pages,
    (String?, int) account,
    int version,
  ) {
    _checkAccount(account);
    final newer = _pageNameSnapshots[workspace];
    return newer != null && newer.account == account && newer.version > version
        ? (pages: newer.pages, version: newer.version)
        : (pages: pages, version: version);
  }

  AppFlowyPageNameObservation? currentPageNameObservation(
    String workspaceId,
    String viewId,
  ) {
    final snapshot = _pageNameSnapshots[workspaceId];
    final page = snapshot?.pages.where((p) => p.viewId == viewId).singleOrNull;
    if (snapshot == null || page == null) return null;
    final value = AppFlowyPageNameObservation._(
      this,
      snapshot.account,
      page,
      snapshot.version,
    );
    return isCurrentPageNameObservation(value) ? value : null;
  }

  void _renameChanged() {
    if (!_renameChanges.isClosed) _renameChanges.add(null);
  }

  bool isCurrentPageNameObservation(AppFlowyPageNameObservation value) =>
      !_pageNameChanges.isClosed &&
      identical(value._owner, this) &&
      value._account == (accountSubject?.call(), _accountEpoch) &&
      _pageNameSnapshots[value.page.workspaceId]?.version == value._version &&
      (accountSubject == null || value._account.$1 != null);

  AppFlowyPageRename? pendingPageRename(String workspaceId, String viewId) {
    final intent = _renameIntents[(workspaceId, viewId)];
    if (intent == null ||
        intent.target._account != (accountSubject?.call(), _accountEpoch))
      return null;
    return intent.snapshot;
  }

  Future<AppFlowyPageRenameTarget> preparePageRename(
    String workspaceId,
    String viewId,
  ) async {
    final workspace = _uuid(workspaceId),
        id = _uuid(viewId),
        account = (accountSubject?.call(), _accountEpoch);
    _checkAccount(account);
    final role = await workspaceRole(workspace).timeout(pageRenameTimeout);
    _checkAccount(account);
    if (role == AppFlowyWorkspaceRole.guest)
      throw const AppFlowyPageRenameBlocked('READ_ONLY');
    final snapshot = await _workspacePageSnapshot(
      workspace,
    ).timeout(pageRenameTimeout);
    _checkAccount(account);
    if (_pageNameReads[workspace] != snapshot.version)
      throw const AppFlowyPageRenameBlocked('TARGET_CHANGED');
    final page = snapshot.pages.where((p) => p.viewId == id).singleOrNull;
    if (page == null) throw const AppFlowyPageRenameBlocked('TARGET_CHANGED');
    return AppFlowyPageRenameTarget._(this, account, page);
  }

  void _currentRename(_PageRenameIntent intent) {
    _checkAccount(intent.target._account);
    if (!identical(_renameIntents[intent.key], intent))
      throw const AppFlowyPageRenameBlocked('SOURCE_ENDED');
  }

  _PageRenameIntent _renameIntent(AppFlowyPageRename before) {
    if (!identical(before._owner, this))
      throw const AppFlowyPageRenameBlocked('SOURCE_ENDED');
    _checkAccount(before._account);
    final intent =
        _renameIntents[(
          before.target.page.workspaceId,
          before.target.page.viewId,
        )];
    if (intent == null || intent.id != before.operationId)
      throw const AppFlowyPageRenameBlocked('SOURCE_ENDED');
    _currentRename(intent);
    return intent;
  }

  bool _sameRenameTarget(AppFlowyWorkspacePage a, AppFlowyWorkspacePage b) =>
      a.runtimeType == b.runtimeType &&
      a.workspaceId == b.workspaceId &&
      a.viewId == b.viewId &&
      jsonEncode(a.ancestors) == jsonEncode(b.ancestors) &&
      (a is! AppFlowyDatabasePage ||
          (b is AppFlowyDatabasePage && a.databaseId == b.databaseId));

  Future<AppFlowyPageRename> renamePage(
    AppFlowyPageRenameTarget target,
    String name,
  ) async {
    if (!identical(target._owner, this))
      throw const AppFlowyPageRenameBlocked('SOURCE_ENDED');
    _checkAccount(target._account);
    final title = _checkedPageTitle(name);
    final key = (target.page.workspaceId, target.page.viewId);
    if (_renameIntents.containsKey(key))
      throw const AppFlowyPageRenameBlocked('REVIEW_REQUIRED');
    final intent = _PageRenameIntent(this, target, title);
    _renameIntents[key] = intent;
    _renameChanged();
    try {
      final fresh = await preparePageRename(key.$1, key.$2);
      _currentRename(intent);
      if (!_sameRenameTarget(target.page, fresh.page) ||
          target.page.name != fresh.page.name) {
        throw const AppFlowyPageRenameBlocked('TARGET_CHANGED');
      }
    } on Object {
      if (identical(_renameIntents[key], intent)) {
        _renameIntents.remove(key);
        _renameChanged();
      }
      rethrow;
    }
    _currentRename(intent);
    intent.status = AppFlowyPageRenameStatus.sending;
    _renameChanged();
    try {
      await _envelope(
        'POST',
        '/api/workspace/${key.$1}/page-view/${key.$2}/update-name',
        body: {'name': title},
      ).timeout(pageRenameTimeout);
      _currentRename(intent);
      intent.serverReturnedSuccess = true;
    } on Object {
      _currentRename(intent);
      intent.status = AppFlowyPageRenameStatus.unconfirmed;
      _renameChanged();
      return intent.snapshot;
    }
    return _checkPageRename(intent);
  }

  Future<AppFlowyPageRename> _checkPageRename(_PageRenameIntent intent) async {
    _currentRename(intent);
    intent.status = AppFlowyPageRenameStatus.checking;
    _renameChanged();
    try {
      final snapshot = await _workspacePageSnapshot(
        intent.key.$1,
      ).timeout(pageRenameTimeout);
      _currentRename(intent);
      final page = snapshot.pages
          .where((p) => p.viewId == intent.key.$2)
          .singleOrNull;
      intent.observed = page;
      intent.status =
          _pageNameReads[intent.key.$1] == snapshot.version &&
              page != null &&
              page.name == intent.name &&
              _sameRenameTarget(intent.target.page, page)
          ? AppFlowyPageRenameStatus.ready
          : AppFlowyPageRenameStatus.unconfirmed;
    } on Object {
      _currentRename(intent);
      intent.observed = null;
      intent.status = AppFlowyPageRenameStatus.unconfirmed;
    }
    _renameChanged();
    return intent.snapshot;
  }

  Future<AppFlowyPageRename> reconcilePageRename(AppFlowyPageRename before) {
    final intent = _renameIntent(before);
    if (intent.snapshot.busy) throw const AppFlowyPageRenameBlocked('BUSY');
    return _checkPageRename(intent);
  }

  bool cancelPageRenamePreflight(AppFlowyPageRename before) {
    final intent = _renameIntent(before);
    if (intent.status != AppFlowyPageRenameStatus.preflight) return false;
    _renameIntents.remove(intent.key);
    _renameChanged();
    return true;
  }

  void finishPageRename(AppFlowyPageRename before, {bool reviewed = false}) {
    final intent = _renameIntent(before);
    if (intent.snapshot.busy ||
        (!reviewed && intent.status != AppFlowyPageRenameStatus.ready))
      throw const AppFlowyPageRenameBlocked('REVIEW_REQUIRED');
    _renameIntents.remove(intent.key);
    _renameChanged();
  }
}
