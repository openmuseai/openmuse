part of 'appflowy_cloud_workspace_service.dart';

/// Original AFWorkspaceInvitation status. Unknown wire states remain visible
/// and cannot be accepted; a display string never confers workspace authority.
enum AppFlowyWorkspaceInvitationStatus {
  pending,
  accepted,
  rejected,
  unsupported,
}

final class AppFlowyWorkspaceInvitation {
  const AppFlowyWorkspaceInvitation._({
    required this.id,
    required this.workspaceId,
    required this.wireStatus,
    required this.updatedAt,
    this.workspaceName,
    this.inviterName,
    this.inviterEmail,
  });
  final String id, workspaceId, wireStatus;
  final String? workspaceName, inviterName, inviterEmail;
  final DateTime updatedAt;
  AppFlowyWorkspaceInvitationStatus get status => switch (wireStatus) {
    'Pending' => AppFlowyWorkspaceInvitationStatus.pending,
    'Accepted' => AppFlowyWorkspaceInvitationStatus.accepted,
    'Rejected' => AppFlowyWorkspaceInvitationStatus.rejected,
    _ => AppFlowyWorkspaceInvitationStatus.unsupported,
  };
  factory AppFlowyWorkspaceInvitation.fromJson(Object? value) {
    final data = _object(value, 'workspace invitation');
    final id = _uuid(_requiredString(data, 'invite_id')).toLowerCase();
    final workspace = _uuid(
      _requiredString(data, 'workspace_id'),
    ).toLowerCase();
    final status = _requiredString(data, 'status');
    if (status.length > 64) throw _invalid('invitation status too long');
    final updated = DateTime.tryParse(_requiredString(data, 'updated_at'));
    if (updated == null) throw _invalid('invalid invitation timestamp');
    String? text(String key) {
      final value = data[key];
      if (value == null || value == '') return null;
      if (value is! String) throw _invalid('invalid invitation $key');
      return value;
    }

    return AppFlowyWorkspaceInvitation._(
      id: id,
      workspaceId: workspace,
      wireStatus: status,
      updatedAt: updated.toUtc(),
      workspaceName: text('workspace_name'),
      inviterName: text('inviter_name'),
      inviterEmail: text('inviter_email'),
    );
  }
}

Duration _invitationDeadline(Duration timeout) {
  if (timeout <= Duration.zero || timeout > const Duration(minutes: 2)) {
    throw ArgumentError('invalid invitation request timeout');
  }
  return timeout;
}

/// Original client-api http_member contracts, using the same authenticated
/// request/epoch/response-budget implementation. Ambiguous POSTs never replay.
extension AppFlowyWorkspaceInvitationApi on AppFlowyCloudWorkspaceService {
  Future<List<AppFlowyWorkspaceInvitation>> workspaceInvitations({
    Duration timeout = const Duration(seconds: 20),
  }) async {
    final data = await _envelope(
      'GET',
      '/api/workspace/invite',
      timeout: _invitationDeadline(timeout),
    );
    if (data is! List) throw _invalid('invitation list required');
    final invitations = data
        .map(AppFlowyWorkspaceInvitation.fromJson)
        .toList(growable: false);
    if (invitations.map((e) => e.id).toSet().length != invitations.length) {
      throw _invalid('duplicate invitation identity');
    }
    return List.unmodifiable(invitations);
  }

  Future<AppFlowyWorkspaceInvitation> workspaceInvitation(
    String invitationId, {
    Duration timeout = const Duration(seconds: 20),
  }) async {
    final id = _uuid(invitationId).toLowerCase();
    final data = await _envelope(
      'GET',
      '/api/workspace/invite/$id',
      timeout: _invitationDeadline(timeout),
    );
    final invitation = AppFlowyWorkspaceInvitation.fromJson(data);
    if (invitation.id != id)
      throw _invalid('invitation response identity changed');
    return invitation;
  }

  Future<void> acceptWorkspaceInvitation(
    String invitationId, {
    Duration timeout = const Duration(seconds: 20),
  }) async {
    final id = _uuid(invitationId).toLowerCase();
    // The original Actix endpoint accepts a unit request; authority and Pending
    // status are checked atomically by its existing invitation transaction.
    await _envelope(
      'POST',
      '/api/workspace/accept-invite/$id',
      timeout: _invitationDeadline(timeout),
    );
  }
}
