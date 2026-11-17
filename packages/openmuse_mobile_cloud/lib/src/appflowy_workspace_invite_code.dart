part of 'appflowy_cloud_workspace_service.dart';

String _workspaceInviteCode(Object? value) {
  if (value is! String || !RegExp(r'^[A-Za-z0-9]{16}$').hasMatch(value)) {
    throw _invalid('invalid workspace invitation code');
  }
  return value;
}

/// Original code observation, scoped to this account/service/read. The code is
/// a bearer credential; callers must not log, persist, or preview it remotely.
final class AppFlowyWorkspaceInviteCode {
  AppFlowyWorkspaceInviteCode._(
    this.workspaceId,
    this.code,
    this._service,
    this._account,
    this._version,
  );
  final String workspaceId;
  final String? code;
  final AppFlowyCloudWorkspaceService _service;
  final (String?, int) _account;
  final int _version;
  // The original GET does not return an expiry timestamp. Existence alone
  // must never be presented as proof of current validity or unlimited expiry.
}

final class AppFlowyWorkspaceInviteCodePreview {
  AppFlowyWorkspaceInviteCodePreview._(
    this.workspaceId,
    this.workspaceName,
    this.ownerName,
    this.memberCount,
    this.isMember,
    this._code,
    this._service,
    this._account,
    this._version,
  );
  final String workspaceId, workspaceName, ownerName;
  final int memberCount;
  final bool isMember;
  final String _code;
  final AppFlowyCloudWorkspaceService _service;
  final (String?, int) _account;
  final int _version;
}

extension AppFlowyWorkspaceInviteCodeApi on AppFlowyCloudWorkspaceService {
  List<AppFlowyAccessMutation> pendingInviteCodeMutations(String workspaceId) =>
      List.unmodifiable(
        _codeIntents
            .where(
              (i) =>
                  i.workspaceId == workspaceId &&
                  i.account == (accountSubject?.call(), _accountEpoch),
            )
            .map((i) => i.snapshot),
      );

  AppFlowyWorkspaceInviteCode _rememberCode(
    String workspace,
    String? code,
    (String?, int) account,
  ) {
    _checkAccount(account);
    if (_codeReads.length >= 128 && !_codeReads.containsKey(workspace)) {
      _codeReads.remove(_codeReads.keys.first);
    }
    final value = AppFlowyWorkspaceInviteCode._(
      workspace,
      code,
      this,
      account,
      ++_codeReadVersion,
    );
    _codeReads[workspace] = value;
    return value;
  }

  void _currentCode(AppFlowyWorkspaceInviteCode observed) {
    _checkAccount(observed._account);
    if (!identical(observed._service, this) ||
        !identical(_codeReads[observed.workspaceId], observed)) {
      throw const AppFlowyAccessMutationBlocked('STALE_CODE_OBSERVATION');
    }
  }

  bool workspaceInviteCodeIsCurrent(AppFlowyWorkspaceInviteCode value) {
    try {
      _currentCode(value);
      return true;
    } on Object {
      return false;
    }
  }

  Future<AppFlowyWorkspaceInviteCode> workspaceInviteCode(
    String workspaceId,
  ) async {
    final wid = _uuid(workspaceId).toLowerCase();
    final account = (accountSubject?.call(), _accountEpoch);
    final raw = _object(
      await _envelope(
        'GET',
        '/api/workspace/$wid/invite-code',
        timeout: const Duration(seconds: 20),
      ),
      'workspace invitation code',
    );
    if (!raw.containsKey('code'))
      throw _invalid('missing invitation code state');
    final code = raw['code'] == null ? null : _workspaceInviteCode(raw['code']);
    final value = _rememberCode(wid, code, account);
    // A known acknowledgement can be reviewed against a current fact. A read
    // cannot prove the causal outcome of an unknown generation/reset/delete.
    _codeIntents.removeWhere(
      (i) =>
          i.workspaceId == wid &&
          i.account == account &&
          i.status == AppFlowyAccessMutationStatus.accepted,
    );
    _accessChanged();
    return value;
  }

  Future<AppFlowyWorkspaceInviteCode> createWorkspaceInviteCode(
    AppFlowyWorkspaceInviteCode observed, {
    int? validityPeriodHours = 168,
  }) async {
    _currentCode(observed);
    if (validityPeriodHours != null &&
        (validityPeriodHours < 1 || validityPeriodHours > 8760)) {
      throw ArgumentError('invalid invitation validity');
    }
    final raw = await _accessMutation(
      AppFlowyAccessMutationKind.createInviteCode,
      observed.workspaceId,
      null,
      [observed.workspaceId],
      'create:${observed._version}:${validityPeriodHours ?? 'unlimited'}',
      'POST',
      '/api/workspace/${observed.workspaceId}/invite-code',
      {'validity_period_hours': validityPeriodHours},
      registry: _codeIntents,
    );
    _currentCode(observed);
    return _rememberCode(
      observed.workspaceId,
      _workspaceInviteCode(_object(raw, 'created invitation code')['code']),
      observed._account,
    );
  }

  Future<AppFlowyWorkspaceInviteCode> deleteWorkspaceInviteCode(
    AppFlowyWorkspaceInviteCode observed,
  ) async {
    _currentCode(observed);
    await _accessMutation(
      AppFlowyAccessMutationKind.deleteInviteCode,
      observed.workspaceId,
      null,
      [observed.workspaceId],
      'delete:${observed._version}',
      'DELETE',
      '/api/workspace/${observed.workspaceId}/invite-code',
      const {},
      registry: _codeIntents,
    );
    _currentCode(observed);
    return _rememberCode(observed.workspaceId, null, observed._account);
  }

  Future<AppFlowyWorkspaceInviteCodePreview> previewWorkspaceInviteCode(
    String code,
  ) async {
    _workspaceInviteCode(code);
    final account = (accountSubject?.call(), _accountEpoch);
    final raw = _object(
      await _envelope(
        'GET',
        Uri(
          path: '/api/invite-code-info',
          queryParameters: {'code': code},
        ).toString(),
        timeout: const Duration(seconds: 20),
      ),
      'invited workspace',
    );
    final workspace = _uuid(_requiredString(raw, 'workspace_id')).toLowerCase();
    final name = _requiredString(raw, 'workspace_name');
    final owner = _requiredString(raw, 'owner_name');
    final count = raw['member_count'];
    if (name.length > 4096 ||
        owner.length > 4096 ||
        count is! int ||
        count < 1 ||
        count > 10000000 ||
        raw['is_member'] is! bool) {
      throw _invalid('invalid invitation preview');
    }
    _checkAccount(account);
    if (_codePreviews.length >= 128 && !_codePreviews.containsKey(code)) {
      _codePreviews.remove(_codePreviews.keys.first);
    }
    final value = AppFlowyWorkspaceInviteCodePreview._(
      workspace,
      name,
      owner,
      count,
      raw['is_member'] as bool,
      code,
      this,
      account,
      ++_codeReadVersion,
    );
    _codePreviews[code] = value;
    return value;
  }

  Future<AppFlowyWorkspaceRole> joinWorkspaceByInviteCode(
    AppFlowyWorkspaceInviteCodePreview preview,
  ) async {
    _checkAccount(preview._account);
    if (!identical(preview._service, this) ||
        !identical(_codePreviews[preview._code], preview)) {
      throw const AppFlowyAccessMutationBlocked('STALE_CODE_PREVIEW');
    }
    final raw = _object(
      await _accessMutation(
        AppFlowyAccessMutationKind.joinInviteCode,
        preview.workspaceId,
        null,
        [preview.workspaceId],
        'join:${preview._version}',
        'POST',
        '/api/workspace/join-by-invite-code',
        {'code': preview._code},
        registry: _codeIntents,
      ),
      'joined workspace',
    );
    if (_uuid(_requiredString(raw, 'workspace_id')).toLowerCase() !=
        preview.workspaceId) {
      throw _invalid('joined workspace identity changed');
    }
    // Original server membership remains the authority, including an existing
    // Guest/Owner role. The returned workspace id does not grant a local role.
    final role = await workspaceRole(preview.workspaceId);
    _checkAccount(preview._account);
    _codePreviews.remove(preview._code);
    _codeIntents.removeWhere(
      (i) =>
          i.workspaceId == preview.workspaceId &&
          i.account == preview._account &&
          i.kind == AppFlowyAccessMutationKind.joinInviteCode &&
          i.status == AppFlowyAccessMutationStatus.accepted,
    );
    _accessChanged();
    return role;
  }
}
