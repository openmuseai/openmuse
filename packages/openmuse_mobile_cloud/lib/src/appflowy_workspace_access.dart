part of 'appflowy_cloud_workspace_service.dart';

/// A definitive preflight/server rejection; other failures can be ambiguous.
final class AppFlowyRowCreationRejected implements Exception {
  const AppFlowyRowCreationRejected(this.code);
  final String code;
}

final class AppFlowyDatabaseRowCreation {
  AppFlowyDatabaseRowCreation._(
    this.rootId,
    this.rowId,
    this.beforeRootState,
    this.rootUpdate,
    this.rowState,
  );
  final String rootId, rowId, beforeRootState, rootUpdate, rowState;
  factory AppFlowyDatabaseRowCreation.fromJson(
    Object? value,
    String expectedRoot,
  ) {
    final data = _object(value, 'row creation');
    if (data['profile'] != 'appflowy.database-row-creation/1' ||
        data['rootId'] != expectedRoot) {
      throw _invalid('row creation binding mismatch');
    }
    final row = _uuid(_requiredString(data, 'rowId'));
    if (row == expectedRoot) throw _invalid('row creation object mismatch');
    final values = [
      for (final key in ['beforeRootState', 'rootUpdate', 'rowState'])
        _requiredString(data, key),
    ];
    var total = 0;
    for (final value in values) {
      List<int> bytes;
      try {
        bytes = base64Decode(value);
      } on FormatException {
        throw _invalid('invalid creation state');
      }
      if (bytes.isEmpty || bytes.length > 8 * 1024 * 1024)
        throw _invalid('creation object budget');
      total += bytes.length;
    }
    if (total > 16 * 1024 * 1024) throw _invalid('creation state budget');
    return AppFlowyDatabaseRowCreation._(
      expectedRoot,
      row,
      values[0],
      values[1],
      values[2],
    );
  }
}

/// Original AFRole/AFAccessLevel wire values. No independent authorization
/// table is kept here: every mutation is decided by AppFlowy Cloud.
enum AppFlowyWorkspaceRole {
  owner('Owner'),
  member('Member'),
  guest('Guest');

  const AppFlowyWorkspaceRole(this.wire);
  final String wire;
  static AppFlowyWorkspaceRole parse(Object? value) => switch (value) {
    'Owner' || 1 => owner,
    'Member' || 2 => member,
    'Guest' || 3 => guest,
    _ => throw _invalid('unknown workspace role'),
  };
}

enum AppFlowyAccessLevel {
  readOnly(10),
  comment(20),
  edit(30),
  full(50);

  const AppFlowyAccessLevel(this.wire);
  final int wire;
  static AppFlowyAccessLevel parse(Object? value) => values.firstWhere(
    (v) => v.wire == value,
    orElse: () => throw _invalid('unknown access level'),
  );
}

final class AppFlowyWorkspaceMember {
  const AppFlowyWorkspaceMember({
    required this.email,
    required this.name,
    required this.role,
  });
  final String email, name;
  final AppFlowyWorkspaceRole role;
  factory AppFlowyWorkspaceMember.fromJson(Object? data) {
    final value = _object(data, 'member');
    return AppFlowyWorkspaceMember(
      email: _requiredString(value, 'email'),
      name: _optionalString(value, 'name') ?? '',
      role: AppFlowyWorkspaceRole.parse(value['role']),
    );
  }
}

final class AppFlowySharedPerson {
  const AppFlowySharedPerson({
    required this.viewId,
    required this.email,
    required this.name,
    required this.role,
    required this.access,
    required this.pending,
    this.sourceViewName,
  });
  final String viewId, email, name;
  final String? sourceViewName;
  final AppFlowyWorkspaceRole role;
  final AppFlowyAccessLevel access;
  final bool pending;
  factory AppFlowySharedPerson.fromJson(Object? data) {
    final value = _object(data, 'shared person');
    return AppFlowySharedPerson(
      viewId: _uuid(_requiredString(value, 'view_id')),
      email: _requiredString(value, 'email'),
      name: _optionalString(value, 'name') ?? '',
      role: AppFlowyWorkspaceRole.parse(value['role']),
      access: AppFlowyAccessLevel.parse(value['access_level']),
      pending: value['pending_invitation'] == true,
      sourceViewName: _optionalString(value, 'source_view_name'),
    );
  }
}

final class AppFlowyPageAccess {
  const AppFlowyPageAccess(this.viewId, this.people);
  final String viewId;
  final List<AppFlowySharedPerson> people;
}

abstract interface class AppFlowyWorkspacePage {
  String get workspaceId;
  String get viewId;
  String get name;
  List<String> get ancestors;
}

final class AppFlowyDocumentPage implements AppFlowyWorkspacePage {
  const AppFlowyDocumentPage({
    required this.workspaceId,
    required this.viewId,
    required this.name,
    this.ancestors = const [],
  });
  final String workspaceId, viewId, name;
  final List<String> ancestors;
}

final class AppFlowyDatabasePage implements AppFlowyWorkspacePage {
  const AppFlowyDatabasePage({
    required this.workspaceId,
    required this.viewId,
    required this.databaseId,
    required this.name,
    required this.layout,
    this.ancestors = const [],
  });
  @override
  final String workspaceId, viewId, name;
  final String databaseId;
  final int layout;
  @override
  final List<String> ancestors;
}

String _uuid(String value) {
  if (!RegExp(
    r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
  ).hasMatch(value))
    throw ArgumentError('invalid workspace/view identifier');
  return value;
}

List<String> _emails(List<String> emails) {
  if (emails.isEmpty || emails.length > 100)
    throw ArgumentError('invalid recipient count');
  final result = emails.map((e) => e.trim()).toSet().toList(growable: false);
  if (result.any(
    (e) => e.length > 254 || !RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]+$').hasMatch(e),
  ))
    throw ArgumentError('invalid email');
  return result;
}

/// Thin mapping of the existing client-api http_member/http_guest contracts.
/// Uses the account adapter's same refresh/HTTP/envelope implementation.
extension AppFlowyWorkspaceAccessApi on AppFlowyCloudWorkspaceService {
  Future<bool> supportsDatabaseRowCreation(
    String workspaceId,
    String rootId,
  ) async {
    try {
      return await _databaseRowCreationCapability(workspaceId, rootId);
    } on Exception {
      return false;
    }
  }

  Future<bool> _databaseRowCreationCapability(
    String workspaceId,
    String rootId,
  ) async {
    final root = _uuid(rootId), workspace = _uuid(workspaceId);
    final data = _object(
      await _envelope(
        'GET',
        '/api/workspace/$workspace/database/$root/row/creation-capabilities',
        timeout: rowCreationTimeout,
      ),
      'creation capabilities',
    );
    return data['profile'] == 'appflowy.database-row-creation/1' &&
        data['rootId'] == root &&
        data['maxStateBytes'] == 16 * 1024 * 1024;
  }

  Future<AppFlowyDatabaseRowCreation> createDatabaseRow(
    String workspaceId,
    String rootId,
    Map<String, Object?> cells,
  ) async {
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    final root = _uuid(rootId), workspace = _uuid(workspaceId);
    if (cells.length > 128 ||
        utf8.encode(jsonEncode(cells)).length > 1024 * 1024) {
      throw ArgumentError('cell creation budget');
    }
    // Everything in this block precedes POST. Its failure cannot be an
    // unknown creation outcome and must not freeze an editor's creation queue.
    try {
      if (!await _databaseRowCreationCapability(workspace, root)) {
        throw const AppFlowyRowCreationRejected('ROW_CREATION_UNSUPPORTED');
      }
      _checkAccount(subject);
    } on AppFlowyRowCreationRejected {
      rethrow;
    } on CloudServiceException catch (error) {
      if ({404, 501}.contains(error.httpStatus)) {
        throw const AppFlowyRowCreationRejected('ROW_CREATION_UNSUPPORTED');
      }
      throw const AppFlowyRowCreationRejected('ROW_CREATION_PREFLIGHT_FAILED');
    } on Exception {
      throw const AppFlowyRowCreationRejected('ROW_CREATION_PREFLIGHT_FAILED');
    }
    final value = await _envelope(
      'POST',
      '/api/workspace/$workspace/database/$root/row?include_creation_state=true',
      body: {'cells': cells},
      responseBudget: 24 * 1024 * 1024,
      timeout: rowCreationTimeout,
      authorizationRejection: () =>
          const AppFlowyRowCreationRejected('READ_ONLY'),
    );
    _checkAccount(subject);
    return AppFlowyDatabaseRowCreation.fromJson(value, root);
  }

  Future<List<AppFlowyDocumentPage>> documentPages(String workspaceId) async {
    return (await _workspaceViews(workspaceId))
        .where((node) => node.layout == 0)
        .map(
          (node) => AppFlowyDocumentPage(
            workspaceId: workspaceId,
            viewId: node.id,
            name: node.name,
            ancestors: node.ancestors,
          ),
        )
        .toList(growable: false);
  }

  /// Resolve a known document through the original authorized page route.
  /// Row documents are not Folder entries. This metadata is never an ACL or
  /// a substitute for the subsequent current-account Document0 content read.
  Future<AppFlowyDocumentPage?> documentPageForReference(
    String workspaceId,
    String documentId,
  ) async {
    final workspace = _uuid(workspaceId), document = _uuid(documentId);
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    Object? data;
    try {
      data = await _envelope(
        'GET',
        '/api/workspace/$workspace/page-view/$document',
      );
      _checkAccount(subject);
    } on CloudServiceException catch (error) {
      _checkAccount(subject);
      if ({404, 501}.contains(error.httpStatus)) return null;
      rethrow;
    }
    final view = _object(_object(data, 'page collab')['view'], 'page view');
    if (_uuid(_requiredString(view, 'view_id')) != document ||
        view['layout'] != 0 ||
        view['is_space'] != false ||
        view['name'] is! String) {
      throw _invalid('document page identity required');
    }
    return AppFlowyDocumentPage(
      workspaceId: workspace,
      viewId: document,
      name: (view['name'] as String).isEmpty ? '记录正文' : view['name'] as String,
    );
  }

  /// Join the original Folder and WorkspaceDatabase projection. The view ID
  /// remains the sharing target; the database ID is the independent Root1.
  Future<List<AppFlowyWorkspacePage>> workspacePages(
    String workspaceId,
  ) async => (await _workspacePageSnapshot(workspaceId)).pages;

  Future<({List<AppFlowyWorkspacePage> pages, int version})>
  _workspacePageSnapshot(String workspaceId) async {
    _uuid(workspaceId);
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    final version = ++_pageNameReadSerial;
    _pageNameReads[workspaceId] = version;
    final views = await _workspaceViews(workspaceId);
    _checkAccount(subject);
    Object? data;
    try {
      data = await _envelope(
        'GET',
        '/api/workspace/${_uuid(workspaceId)}/database',
      );
      _checkAccount(subject);
    } on CloudServiceException catch (error) {
      _checkAccount(subject);
      if (!{404, 501}.contains(error.httpStatus)) rethrow;
      // Database catalog availability is independent of the original Folder
      // documents. A down-level server must not disable existing document UI.
      final pages = List<AppFlowyWorkspacePage>.unmodifiable([
        for (final node in views.where((n) => n.layout == 0))
          AppFlowyDocumentPage(
            workspaceId: workspaceId,
            viewId: node.id,
            name: node.name,
            ancestors: node.ancestors,
          ),
      ]);
      _publishPageNames(workspaceId, pages, subject, version);
      return _preferObservedPageSnapshot(workspaceId, pages, subject, version);
    }
    if (data is! List) throw _invalid('database catalog required');
    final roots = <String, String>{};
    for (final value in data) {
      final database = _object(value, 'database');
      final id = _uuid(_requiredString(database, 'id'));
      if (database['views'] is! List) throw _invalid('database views required');
      for (final value in database['views'] as List) {
        final view = _uuid(
          _requiredString(_object(value, 'database view'), 'view_id'),
        );
        if (roots.containsKey(view) && roots[view] != id)
          throw _invalid('ambiguous database view');
        roots[view] = id;
      }
    }
    final pages = List<AppFlowyWorkspacePage>.unmodifiable([
      for (final node in views)
        if (node.layout == 0)
          AppFlowyDocumentPage(
            workspaceId: workspaceId,
            viewId: node.id,
            name: node.name,
            ancestors: node.ancestors,
          )
        else if ({1, 2, 3}.contains(node.layout) && roots.containsKey(node.id))
          AppFlowyDatabasePage(
            workspaceId: workspaceId,
            viewId: node.id,
            databaseId: roots[node.id]!,
            name: node.name,
            layout: node.layout,
            ancestors: node.ancestors,
          ),
    ]);
    _publishPageNames(workspaceId, pages, subject, version);
    return _preferObservedPageSnapshot(workspaceId, pages, subject, version);
  }

  Future<List<({String id, String name, int layout, List<String> ancestors})>>
  _workspaceViews(String workspaceId) async {
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    final role = await workspaceRole(workspaceId);
    _checkAccount(subject);
    final folder = role == AppFlowyWorkspaceRole.guest
        ? _object(
            await _envelope(
              'GET',
              '/api/sharing/workspace/${_uuid(workspaceId)}/view/$workspaceId?depth=10',
            ),
            'shared folder',
          )
        : await workspaceFolder(workspaceId, depth: 10);
    _checkAccount(subject);
    if (folder['view_id'] != workspaceId)
      throw _invalid('Folder binding mismatch');
    final pages =
        <({String id, String name, int layout, List<String> ancestors})>[];
    final ids = <String>{};
    var visited = 0;
    void walk(Map<String, Object?> node, List<String> ancestors) {
      if (++visited > 10000 || ancestors.length > 12)
        throw _invalid('folder budget');
      final id = _uuid(_requiredString(node, 'view_id'));
      if (!ids.add(id)) throw _invalid('Duplicate Folder identity');
      if (node['layout'] is int &&
          node['is_space'] != true &&
          id != workspaceId) {
        pages.add((
          id: id,
          layout: node['layout'] as int,
          name: _optionalString(node, 'name') ?? 'Untitled',
          ancestors: List.unmodifiable(ancestors),
        ));
      }
      final children = node['children'];
      if (children is! List) throw _invalid('folder children required');
      for (final child in children) {
        walk(_object(child, 'view'), [...ancestors, id]);
      }
    }

    walk(folder, []);
    return List.unmodifiable(pages);
  }

  Future<AppFlowyWorkspaceRole> workspaceRole(String workspaceId) async {
    _uuid(workspaceId);
    final data = await _envelope('GET', '/api/workspace?include_role=true');
    if (data is! List) throw _invalid('workspace list required');
    final matches = data
        .map((d) => _object(d, 'workspace'))
        .where((d) => d['workspace_id'] == workspaceId)
        .toList();
    if (matches.length != 1)
      throw const CloudServiceException(
        CloudServiceErrorCode.unauthorized,
        'workspace not available',
      );
    return AppFlowyWorkspaceRole.parse(matches.single['role']);
  }

  Future<List<AppFlowyWorkspaceMember>> workspaceMembers(
    String workspaceId,
  ) async {
    final data = await _envelope(
      'GET',
      '/api/workspace/${_uuid(workspaceId)}/member',
    );
    if (data is! List) throw _invalid('member list required');
    final members = List<AppFlowyWorkspaceMember>.unmodifiable(
      data.map(AppFlowyWorkspaceMember.fromJson),
    );
    _observeAccessMembers(workspaceId, members);
    return members;
  }

  Future<void> inviteWorkspaceMembers(
    String workspaceId,
    List<String> emails, {
    AppFlowyWorkspaceRole role = AppFlowyWorkspaceRole.member,
    bool skipEmailSend = false,
  }) async {
    if (role == AppFlowyWorkspaceRole.owner)
      throw ArgumentError('ownership transfer uses its own workflow');
    final workspace = _uuid(workspaceId), targets = _emails(emails);
    await _accessMutation(
      AppFlowyAccessMutationKind.invite,
      workspace,
      null,
      targets,
      role.wire,
      'POST',
      '/api/workspace/$workspace/invite',
      [
        for (final email in targets)
          {'email': email, 'role': role.wire, 'skip_email_send': skipEmailSend},
      ],
    );
  }

  Future<void> updateWorkspaceMember(
    String workspaceId,
    String email,
    AppFlowyWorkspaceRole role,
  ) async {
    if (role == AppFlowyWorkspaceRole.owner)
      throw ArgumentError('ownership transfer uses its own workflow');
    final workspace = _uuid(workspaceId), targets = _emails([email]);
    await _accessMutation(
      AppFlowyAccessMutationKind.role,
      workspace,
      null,
      targets,
      role.wire,
      'PUT',
      '/api/workspace/$workspace/member',
      {'email': targets.single, 'role': role.wire},
    );
  }

  Future<void> removeWorkspaceMembers(
    String workspaceId,
    List<String> emails,
  ) async {
    final workspace = _uuid(workspaceId), targets = _emails(emails);
    await _accessMutation(
      AppFlowyAccessMutationKind.remove,
      workspace,
      null,
      targets,
      null,
      'DELETE',
      '/api/workspace/$workspace/member',
      targets,
    );
  }

  Future<AppFlowyPageAccess> pageAccess(
    String workspaceId,
    String viewId, {
    List<String> ancestors = const [],
  }) async {
    _uuid(viewId);
    if (ancestors.length > 64) throw ArgumentError('ancestor budget');
    final data = _object(
      await _envelope(
        'POST',
        '/api/sharing/workspace/${_uuid(workspaceId)}/view/$viewId/access-details',
        body: {'ancestor_view_ids': ancestors.map(_uuid).toList()},
      ),
      'access details',
    );
    if (data['view_id'] != viewId || data['shared_with'] is! List)
      throw _invalid('view access mismatch');
    final access = AppFlowyPageAccess(
      viewId,
      List.unmodifiable(
        (data['shared_with'] as List).map(AppFlowySharedPerson.fromJson),
      ),
    );
    _observePageAccess(workspaceId, access);
    return access;
  }

  Future<void> shareView(
    String workspaceId,
    String viewId,
    List<String> emails,
    AppFlowyAccessLevel access,
  ) async {
    final workspace = _uuid(workspaceId),
        view = _uuid(viewId),
        targets = _emails(emails);
    await _accessMutation(
      AppFlowyAccessMutationKind.share,
      workspace,
      view,
      targets,
      access.wire.toString(),
      'PUT',
      '/api/sharing/workspace/$workspace/view',
      {'view_id': view, 'emails': targets, 'access_level': access.wire},
    );
  }

  Future<void> revokeViewAccess(
    String workspaceId,
    String viewId,
    List<String> emails,
  ) async {
    final workspace = _uuid(workspaceId),
        view = _uuid(viewId),
        targets = _emails(emails);
    await _accessMutation(
      AppFlowyAccessMutationKind.revoke,
      workspace,
      view,
      targets,
      null,
      'POST',
      '/api/sharing/workspace/$workspace/view/$view/revoke-access',
      {'emails': targets},
    );
  }

  /// Original authenticated page creation, not a second resource directory.
  Future<String> createDocumentPage(
    String workspaceId,
    String parentViewId,
    String name, {
    String? documentId,
  }) async {
    final title = _checkedPageTitle(name);
    final data = _object(
      await _envelope(
        'POST',
        '/api/workspace/${_uuid(workspaceId)}/page-view',
        body: {
          'parent_view_id': _uuid(parentViewId),
          'layout': 0,
          'name': title,
          if (documentId != null) ...{
            'view_id': _uuid(documentId),
            'collab_id': _uuid(documentId),
          },
        },
        timeout: documentId == null ? null : documentCreationTimeout,
      ),
      'page',
    );
    return _uuid(_requiredString(data, 'view_id'));
  }

  Future<Map<String, Object?>> workspaceFolder(
    String workspaceId, {
    int depth = 2,
  }) async {
    if (depth < 1 || depth > 10) throw ArgumentError('folder depth');
    return _object(
      await _envelope(
        'GET',
        '/api/workspace/${_uuid(workspaceId)}/folder?depth=$depth',
      ),
      'folder',
    );
  }
}
