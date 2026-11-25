part of 'appflowy_cloud_workspace_service.dart';

typedef _OwnerCancelKey = (int, String, String, DateTime, int, String);
const _ownerInvitationProfile = 'appflowy.workspace-invitation-management/1';

final class AppFlowyInvitationManagementUnsupported implements Exception {
  const AppFlowyInvitationManagementUnsupported();
}

final class AppFlowyOwnerInvitationCursor {
  const AppFlowyOwnerInvitationCursor(this.createdAt, this.inviteId);
  final DateTime createdAt;
  final String inviteId;
}

int _invitationInteger(
  Object? value,
  String name, {
  int min = -2147483648,
  int max = 2147483647,
}) {
  if (value is! int || value < min || value > max)
    throw _invalid('invalid invitation $name');
  return value;
}

DateTime _invitationTime(Object? value) {
  if (value is! String) throw _invalid('invalid invitation timestamp');
  final time = DateTime.tryParse(value);
  if (time == null) throw _invalid('invalid invitation timestamp');
  return time.toUtc();
}

AppFlowyWorkspaceRole? _invitationRole(int? value) => switch (value) {
  1 => AppFlowyWorkspaceRole.owner,
  2 => AppFlowyWorkspaceRole.member,
  3 => AppFlowyWorkspaceRole.guest,
  _ => null,
};

/// Original invitation data. Unknown role/status stay visible and cannot grant
/// authority or be silently converted to Guest/Pending.
final class AppFlowyOwnerWorkspaceInvitation {
  AppFlowyOwnerWorkspaceInvitation._(
    this.id,
    this.workspaceId,
    this.email,
    this.roleId,
    this.wireStatus,
    this.createdAt,
    this.updatedAt,
    this.memberRoleId,
  );
  final String id, workspaceId, email;
  final int roleId, wireStatus;
  final int? memberRoleId;
  final DateTime createdAt, updatedAt;
  AppFlowyWorkspaceRole? get role => _invitationRole(roleId);
  AppFlowyWorkspaceRole? get memberRole => _invitationRole(memberRoleId);
  AppFlowyWorkspaceInvitationStatus get status => switch (wireStatus) {
    0 => AppFlowyWorkspaceInvitationStatus.pending,
    1 => AppFlowyWorkspaceInvitationStatus.accepted,
    2 => AppFlowyWorkspaceInvitationStatus.rejected,
    _ => AppFlowyWorkspaceInvitationStatus.unsupported,
  };
  bool get canCancel =>
      status == AppFlowyWorkspaceInvitationStatus.pending && role != null;
  factory AppFlowyOwnerWorkspaceInvitation.fromJson(
    Object? raw,
    String workspace,
  ) {
    final v = _object(raw, 'sent invitation');
    final wid = _uuid(_requiredString(v, 'workspace_id')).toLowerCase();
    if (wid != workspace.toLowerCase())
      throw _invalid('invitation workspace changed');
    final email = _requiredString(v, 'invitee_email');
    if (email.length > 254) throw _invalid('invalid invitation target');
    return AppFlowyOwnerWorkspaceInvitation._(
      _uuid(_requiredString(v, 'invite_id')).toLowerCase(),
      wid,
      email,
      _invitationInteger(v['role_id'], 'role'),
      _invitationInteger(v['status'], 'status', min: -32768, max: 32767),
      _invitationTime(v['created_at']),
      _invitationTime(v['updated_at']),
      v['member_role_id'] == null
          ? null
          : _invitationInteger(v['member_role_id'], 'member role'),
    );
  }
}

final class AppFlowyOwnerInvitationPage {
  AppFlowyOwnerInvitationPage._(
    this.workspaceId,
    List<AppFlowyOwnerWorkspaceInvitation> items,
    this.hasMore,
    this.nextCursor,
  ) : items = List.unmodifiable(items);
  final String workspaceId;
  final List<AppFlowyOwnerWorkspaceInvitation> items;
  final bool hasMore;
  final AppFlowyOwnerInvitationCursor? nextCursor;
}

final class AppFlowyOwnerInvitationCancellation {
  const AppFlowyOwnerInvitationCancellation(this.outcome, this.invitation);
  final String outcome;
  final AppFlowyOwnerWorkspaceInvitation invitation;
}

extension AppFlowyOwnerInvitationsApi on AppFlowyCloudWorkspaceService {
  Future<AppFlowyOwnerInvitationPage> ownerWorkspaceInvitations(
    String workspaceId, {
    int limit = 50,
    String? email,
    int? status,
    AppFlowyOwnerInvitationCursor? cursor,
  }) async {
    final wid = _uuid(workspaceId).toLowerCase();
    if (limit < 1 ||
        limit > 100 ||
        (status != null && ![0, 1, 2].contains(status)))
      throw ArgumentError('invalid invitation query');
    if (email != null) _emails([email]);
    final query = <String, String>{
      'limit': '$limit',
      if (email != null) 'email': email.trim(),
      if (status != null) 'status': '$status',
      if (cursor != null)
        'before_created_at': cursor.createdAt.toUtc().toIso8601String(),
      if (cursor != null) 'before_invite_id': _uuid(cursor.inviteId),
    };
    Object? raw;
    try {
      raw = await _envelope(
        'GET',
        Uri(
          path: '/api/workspace/$wid/invite',
          queryParameters: query,
        ).toString(),
        timeout: const Duration(seconds: 20),
      );
    } on CloudServiceException catch (error) {
      if ([404, 405].contains(error.httpStatus))
        throw const AppFlowyInvitationManagementUnsupported();
      rethrow;
    }
    final v = _object(raw, 'sent invitation page');
    if (v['profile'] != _ownerInvitationProfile ||
        v['workspace_id'] != wid ||
        v['items'] is! List ||
        v['has_more'] is! bool)
      throw _invalid('invalid invitation management profile');
    final rows = (v['items'] as List)
        .map((r) => AppFlowyOwnerWorkspaceInvitation.fromJson(r, wid))
        .toList(growable: false);
    if (rows.length > limit ||
        rows.map((r) => r.id).toSet().length != rows.length)
      throw _invalid('invalid invitation page budget or identity');
    final more = v['has_more'] as bool;
    AppFlowyOwnerInvitationCursor? next;
    if (more) {
      final c = _object(v['next_cursor'], 'invitation cursor');
      next = AppFlowyOwnerInvitationCursor(
        _invitationTime(c['created_at']),
        _uuid(_requiredString(c, 'invite_id')).toLowerCase(),
      );
      if (rows.isEmpty ||
          next.inviteId != rows.last.id ||
          next.createdAt != rows.last.createdAt)
        throw _invalid('invitation cursor changed');
    } else if (v['next_cursor'] != null) {
      throw _invalid('unexpected invitation cursor');
    }
    // Only a first page can establish a latest per-target fact. A partial page
    // or absence never proves that an unknown send did not happen.
    _observeOwnerInvitations(rows, firstPage: cursor == null);
    return AppFlowyOwnerInvitationPage._(wid, rows, more, next);
  }

  bool ownerInvitationCancellationPending(String workspace, String invite) =>
      _ownerCancellations.keys.any(
        (key) =>
            key.$1 == _accountEpoch &&
            key.$2 == workspace.toLowerCase() &&
            key.$3 == invite.toLowerCase(),
      );

  Future<AppFlowyOwnerInvitationCancellation> cancelOwnerWorkspaceInvitation(
    AppFlowyOwnerWorkspaceInvitation observed,
  ) async {
    if (!observed.canCancel)
      throw ArgumentError('original Pending invitation observation required');
    _checkAccount((accountSubject?.call(), _accountEpoch));
    final key = (
      _accountEpoch,
      observed.workspaceId,
      observed.id,
      observed.updatedAt,
      observed.roleId,
      _accessEmail(observed.email),
    );
    final existing = _ownerCancellations[key];
    if (existing != null) return existing;
    if (_ownerCancellations.length >= 128)
      throw const AppFlowyAccessMutationBlocked('CANCELLATION_BUDGET');
    late final Future<AppFlowyOwnerInvitationCancellation> task;
    task = _cancelOwnerInvitationOnce(observed).whenComplete(() {
      if (identical(_ownerCancellations[key], task))
        _ownerCancellations.remove(key);
      _accessChanged();
    });
    _ownerCancellations[key] = task;
    _accessChanged();
    return task;
  }

  Future<AppFlowyOwnerInvitationCancellation> _cancelOwnerInvitationOnce(
    AppFlowyOwnerWorkspaceInvitation observed,
  ) async {
    // Original invitation status is updated conditionally. A repeated DELETE
    // can return Ended/Accepted, but cannot resend mail or remove membership.
    // Do not put this known-row transition behind the uncertain-send budget.
    final raw = await _envelope(
      'DELETE',
      '/api/workspace/${observed.workspaceId}/invite/${observed.id}',
      body: {
        'expected_updated_at': observed.updatedAt.toUtc().toIso8601String(),
        'expected_role_id': observed.roleId,
        'expected_invitee_email': observed.email,
      },
      timeout: const Duration(seconds: 20),
    );
    final v = _object(raw, 'invitation cancellation');
    if (v['profile'] != _ownerInvitationProfile ||
        ![
          'cancelled',
          'accepted',
          'ended',
          'changed',
          'unsupported',
        ].contains(v['outcome']))
      throw _invalid('invalid invitation cancellation profile');
    final current = AppFlowyOwnerWorkspaceInvitation.fromJson(
      v['invitation'],
      observed.workspaceId,
    );
    if (current.id != observed.id)
      throw _invalid('invitation cancellation identity changed');
    final outcome = v['outcome'] as String;
    if ((['cancelled', 'ended'].contains(outcome) && current.wireStatus != 2) ||
        (outcome == 'accepted' && current.wireStatus != 1))
      throw _invalid('invitation outcome and state disagree');
    _observeOwnerInvitations([current], firstPage: false);
    _accessChanged();
    return AppFlowyOwnerInvitationCancellation(outcome, current);
  }

  void _observeOwnerInvitations(
    List<AppFlowyOwnerWorkspaceInvitation> rows, {
    required bool firstPage,
  }) {
    final latest = <String, AppFlowyOwnerWorkspaceInvitation>{};
    if (firstPage) {
      for (final row in rows) {
        latest.putIfAbsent(_accessEmail(row.email), () => row);
      }
    }
    _accessIntents.removeWhere((i) {
      if (i.status == AppFlowyAccessMutationStatus.sending) return false;
      if (i.kind != AppFlowyAccessMutationKind.invite || i.targets.length != 1)
        return false;
      if (i.observedInviteId != null &&
          rows.any(
            (r) =>
                r.workspaceId == i.workspaceId &&
                r.id == i.observedInviteId &&
                (r.wireStatus == 2 || r.wireStatus == 1),
          ))
        return true;
      final row = latest[i.targets.single];
      if (row == null || row.workspaceId != i.workspaceId) return false;
      if (row.wireStatus == 0 && row.role?.wire == i.value) {
        i.observedInviteId = row.id;
        if (i.status == AppFlowyAccessMutationStatus.unconfirmed)
          i.status = AppFlowyAccessMutationStatus.observed;
      }
      if (row.wireStatus == 1 && row.memberRole?.wire == i.value) return true;
      return row.wireStatus == 2 && row.id == i.observedInviteId;
    });
    _accessChanged();
  }
}
