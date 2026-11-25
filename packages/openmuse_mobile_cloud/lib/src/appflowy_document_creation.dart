part of 'appflowy_cloud_workspace_service.dart';

/// A visible original Folder target, bound to the service/account that read it.
/// This is a selection snapshot, never a server authorization grant.
final class AppFlowyDocumentParent {
  AppFlowyDocumentParent._(
    this._owner,
    this._account,
    this.workspaceId,
    this.viewId,
    this.name,
    this.isSpace,
    List<String> ancestors,
    this.breadcrumb,
  ) : ancestors = List.unmodifiable(ancestors);
  final AppFlowyCloudWorkspaceService _owner;
  final (String?, int) _account;
  final String workspaceId, viewId, name, breadcrumb;
  final bool isSpace;
  final List<String> ancestors;
}

enum AppFlowyDocumentCreationStatus {
  preflight,
  sending,
  checking,
  unconfirmed,
  ready,
}

final class AppFlowyDocumentCreation {
  AppFlowyDocumentCreation._(
    this._owner,
    this._account,
    this.workspaceId,
    this.documentId,
    this.parent,
    this.name,
    this.status,
    this.serverReturnedId,
    this.page,
  );
  final AppFlowyCloudWorkspaceService _owner;
  final (String?, int) _account;
  final String workspaceId, documentId, name;
  final AppFlowyDocumentParent parent;
  final AppFlowyDocumentCreationStatus status;
  final bool serverReturnedId;
  final AppFlowyDocumentPage? page;
  bool get busy => {
    AppFlowyDocumentCreationStatus.preflight,
    AppFlowyDocumentCreationStatus.sending,
    AppFlowyDocumentCreationStatus.checking,
  }.contains(status);
}

final class AppFlowyDocumentCreationBlocked implements Exception {
  const AppFlowyDocumentCreationBlocked(this.code);
  final String code;
}

final class _DocumentCreationIntent {
  _DocumentCreationIntent(this.owner, this.account, this.parent, this.name)
    : documentId = const Uuid().v4();
  final AppFlowyCloudWorkspaceService owner;
  final (String?, int) account;
  final AppFlowyDocumentParent parent;
  final String name, documentId;
  String get workspaceId => parent.workspaceId;
  AppFlowyDocumentCreationStatus status =
      AppFlowyDocumentCreationStatus.preflight;
  bool serverReturnedId = false;
  AppFlowyDocumentPage? page;
  AppFlowyDocumentCreation get snapshot => AppFlowyDocumentCreation._(
    owner,
    account,
    workspaceId,
    documentId,
    parent,
    name,
    status,
    serverReturnedId,
    page,
  );
}

extension AppFlowyDocumentCreationApi on AppFlowyCloudWorkspaceService {
  void _documentChanged() {
    if (!_documentChanges.isClosed) _documentChanges.add(null);
  }

  AppFlowyDocumentCreation? pendingDocumentCreation(String workspaceId) {
    final intent = _documentIntents[workspaceId];
    if (intent == null ||
        intent.account != (accountSubject?.call(), _accountEpoch) ||
        (accountSubject != null && intent.account.$1 == null))
      return null;
    return intent.snapshot;
  }

  Future<List<AppFlowyDocumentParent>> documentCreationParents(
    String workspaceId,
  ) async {
    final workspace = _uuid(workspaceId),
        account = (accountSubject?.call(), _accountEpoch);
    _checkAccount(account);
    final role = await workspaceRole(workspace);
    _checkAccount(account);
    if (role == AppFlowyWorkspaceRole.guest) {
      throw const AppFlowyDocumentCreationBlocked('READ_ONLY');
    }
    final root = await workspaceFolder(workspace, depth: 10);
    _checkAccount(account);
    if (root['view_id'] != workspace) throw _invalid('Folder binding mismatch');
    final found = <AppFlowyDocumentParent>[], ids = <String>{};
    var visited = 0;
    void walk(
      Map<String, Object?> node,
      List<String> ancestors,
      List<String> names,
    ) {
      if (++visited > 10000 || ancestors.length > 12)
        throw _invalid('Folder budget');
      final id = _uuid(_requiredString(node, 'view_id'));
      if (!ids.add(id)) throw _invalid('Duplicate Folder identity');
      final name = _optionalString(node, 'name') ?? 'Untitled';
      final isRoot = id == workspace, isSpace = node['is_space'] == true;
      if (!isRoot && (isSpace || node['layout'] == 0)) {
        found.add(
          AppFlowyDocumentParent._(
            this,
            account,
            workspace,
            id,
            name,
            isSpace,
            ancestors,
            names.join(' / '),
          ),
        );
      }
      final children = node['children'];
      if (children is! List) throw _invalid('Folder children required');
      for (final child in children) {
        walk(_object(child, 'Folder view'), [
          ...ancestors,
          id,
        ], isRoot ? names : [...names, name]);
      }
    }

    walk(root, [], []);
    return List.unmodifiable(found);
  }

  void _currentDocumentIntent(_DocumentCreationIntent intent) {
    _checkAccount(intent.account);
    if (!identical(_documentIntents[intent.workspaceId], intent)) {
      throw const AppFlowyDocumentCreationBlocked('SOURCE_ENDED');
    }
  }

  _DocumentCreationIntent _documentIntent(AppFlowyDocumentCreation before) {
    if (!identical(before._owner, this)) {
      throw const AppFlowyDocumentCreationBlocked('SOURCE_ENDED');
    }
    _checkAccount(before._account);
    final intent = _documentIntents[before.workspaceId];
    if (intent == null ||
        intent.documentId != before.documentId ||
        intent.account != before._account) {
      throw const AppFlowyDocumentCreationBlocked('SOURCE_ENDED');
    }
    _currentDocumentIntent(intent);
    return intent;
  }

  /// One original POST per intent. After any attempted delivery failure the
  /// retained input can only be checked with GETs, never silently replayed.
  Future<AppFlowyDocumentCreation> createDocument(
    AppFlowyDocumentParent parent,
    String name,
  ) async {
    if (!identical(parent._owner, this)) {
      throw const AppFlowyDocumentCreationBlocked('SOURCE_ENDED');
    }
    _checkAccount(parent._account);
    final title = _checkedPageTitle(name);
    if (_documentIntents.containsKey(parent.workspaceId)) {
      throw const AppFlowyDocumentCreationBlocked('REVIEW_REQUIRED');
    }
    final intent = _DocumentCreationIntent(
      this,
      parent._account,
      parent,
      title,
    );
    _documentIntents[parent.workspaceId] = intent;
    _documentChanged();
    try {
      final targets = await documentCreationParents(
        parent.workspaceId,
      ).timeout(documentCreationTimeout);
      _currentDocumentIntent(intent);
      final fresh = targets.where((p) => p.viewId == parent.viewId).firstOrNull;
      if (fresh == null ||
          fresh.name != parent.name ||
          fresh.isSpace != parent.isSpace ||
          jsonEncode(fresh.ancestors) != jsonEncode(parent.ancestors)) {
        throw const AppFlowyDocumentCreationBlocked('TARGET_CHANGED');
      }
    } on Object {
      if (identical(_documentIntents[parent.workspaceId], intent)) {
        _documentIntents.remove(parent.workspaceId);
        _documentChanged();
      }
      rethrow;
    }
    _currentDocumentIntent(intent);
    intent.status = AppFlowyDocumentCreationStatus.sending;
    _documentChanged();
    try {
      final id = await createDocumentPage(
        parent.workspaceId,
        parent.viewId,
        title,
        documentId: intent.documentId,
      );
      _currentDocumentIntent(intent);
      if (id != intent.documentId) throw _invalid('Creation identity mismatch');
      intent.serverReturnedId = true;
    } on Object {
      _currentDocumentIntent(intent);
      intent.status = AppFlowyDocumentCreationStatus.unconfirmed;
      _documentChanged();
      return intent.snapshot;
    }
    return _checkDocumentCreation(intent);
  }

  Future<AppFlowyDocumentCreation> _checkDocumentCreation(
    _DocumentCreationIntent intent,
  ) async {
    _currentDocumentIntent(intent);
    intent.status = AppFlowyDocumentCreationStatus.checking;
    _documentChanged();
    try {
      final snapshot = await _workspacePageSnapshot(
        intent.workspaceId,
      ).timeout(documentCreationTimeout);
      _currentDocumentIntent(intent);
      final page = snapshot.pages
          .whereType<AppFlowyDocumentPage>()
          .where((p) => p.viewId == intent.documentId)
          .firstOrNull;
      intent.page = page;
      intent.status =
          _pageNameReads[intent.workspaceId] == snapshot.version &&
              page != null &&
              page.name == intent.name &&
              page.ancestors.lastOrNull == intent.parent.viewId
          ? AppFlowyDocumentCreationStatus.ready
          : AppFlowyDocumentCreationStatus.unconfirmed;
    } on Object {
      _currentDocumentIntent(intent);
      intent.page = null;
      intent.status = AppFlowyDocumentCreationStatus.unconfirmed;
    }
    _documentChanged();
    return intent.snapshot;
  }

  Future<AppFlowyDocumentCreation> reconcileDocumentCreation(
    AppFlowyDocumentCreation before,
  ) {
    final intent = _documentIntent(before);
    if (intent.snapshot.busy)
      throw const AppFlowyDocumentCreationBlocked('BUSY');
    return _checkDocumentCreation(intent);
  }

  /// Retiring a presentation may cancel its own unsent preflight. Once a POST
  /// has been attempted its outcome must remain available for read-only review.
  bool cancelDocumentCreationPreflight(AppFlowyDocumentCreation before) {
    final intent = _documentIntent(before);
    if (intent.status != AppFlowyDocumentCreationStatus.preflight) return false;
    _documentIntents.remove(intent.workspaceId);
    _documentChanged();
    return true;
  }

  /// Ends local handling only. This does not retry, roll back, delete a server
  /// object, confirm durability or persist an outbox across process restarts.
  void finishDocumentCreation(
    AppFlowyDocumentCreation before, {
    bool reviewed = false,
  }) {
    final intent = _documentIntent(before);
    if (intent.snapshot.busy ||
        (!reviewed && intent.status != AppFlowyDocumentCreationStatus.ready)) {
      throw const AppFlowyDocumentCreationBlocked('REVIEW_REQUIRED');
    }
    _documentIntents.remove(intent.workspaceId);
    _documentChanged();
  }
}
