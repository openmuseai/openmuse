part of 'appflowy_cloud_workspace_service.dart';

/// Delivery state only. These intents confer no access and are not an ACL,
/// durable receipt, distributed CAS, or persistent outbox.
enum AppFlowyAccessMutationStatus { sending, unconfirmed, accepted, observed }

enum AppFlowyAccessMutationKind {
  invite,
  role,
  remove,
  share,
  revoke,
  createInviteCode,
  deleteInviteCode,
  joinInviteCode,
}

final class AppFlowyAccessMutation {
  AppFlowyAccessMutation._(
    this.workspaceId,
    this.viewId,
    this.kind,
    List<String> targets,
    this.value,
    this.status,
  ) : targets = List.unmodifiable(targets);
  final String workspaceId;
  final String? viewId, value;
  final AppFlowyAccessMutationKind kind;
  final List<String> targets;
  final AppFlowyAccessMutationStatus status;
}

final class AppFlowyAccessMutationBlocked implements Exception {
  const AppFlowyAccessMutationBlocked(this.code);
  final String code;
}

final class _AccessIntent {
  _AccessIntent(
    this.workspaceId,
    this.viewId,
    this.kind,
    this.targets,
    this.value,
    this.account,
  );
  final String workspaceId;
  final String? viewId, value;
  final AppFlowyAccessMutationKind kind;
  final List<String> targets;
  final (String?, int) account;
  bool sent = false, rejected = false;
  String? observedInviteId;
  AppFlowyAccessMutationStatus status = AppFlowyAccessMutationStatus.sending;
  AppFlowyAccessMutation get snapshot => AppFlowyAccessMutation._(
    workspaceId,
    viewId,
    kind,
    targets,
    value,
    status,
  );
}

String _accessEmail(String email) => email.trim().toLowerCase();

extension AppFlowyAccessMutationApi on AppFlowyCloudWorkspaceService {
  List<AppFlowyAccessMutation> pendingAccessMutations(
    String workspaceId, {
    String? viewId,
  }) => List.unmodifiable(
    _accessIntents
        .where(
          (i) =>
              i.workspaceId == workspaceId &&
              i.viewId == viewId &&
              i.account.$2 == _accountEpoch &&
              (accountSubject == null || i.account.$1 == accountSubject!()),
        )
        .map((i) => i.snapshot),
  );

  Future<Object?> _accessMutation(
    AppFlowyAccessMutationKind kind,
    String workspaceId,
    String? viewId,
    List<String> targets,
    String? value,
    String method,
    String path,
    Object body, {
    List<_AccessIntent>? registry,
  }) async {
    final intents = registry ?? _accessIntents;
    final account = (accountSubject?.call(), _accountEpoch);
    _checkAccount(account);
    final emails = targets.map(_accessEmail).toList(growable: false);
    if (intents.any(
      (i) =>
          i.account == account &&
          i.workspaceId == workspaceId &&
          i.viewId == viewId &&
          i.targets.any(emails.contains) &&
          (![
                AppFlowyAccessMutationStatus.accepted,
                AppFlowyAccessMutationStatus.observed,
              ].contains(i.status) ||
              (i.kind == kind && i.value == value)),
    ))
      throw const AppFlowyAccessMutationBlocked('RESULT_REQUIRES_REVIEW');
    // An explicitly different command after a confirmed response is a new
    // intent. Unknown/sending targets above remain fenced across all actions.
    intents.removeWhere(
      (i) =>
          i.account == account &&
          i.workspaceId == workspaceId &&
          i.viewId == viewId &&
          i.status == AppFlowyAccessMutationStatus.accepted &&
          i.targets.any(emails.contains),
    );
    // Keep unknown/sending/observed facts; acknowledged history may be dropped
    // when the process-local delivery budget is full. It is not an outbox.
    if (intents.length >= 128) {
      final acknowledged = _accessIntents
          .where((i) => i.status == AppFlowyAccessMutationStatus.accepted)
          .toList();
      for (final old in acknowledged) {
        if (intents.length < 128) break;
        intents.remove(old);
      }
    }
    if (intents.length >= 128)
      throw const AppFlowyAccessMutationBlocked('REVIEW_BUDGET');
    final intent = _AccessIntent(
      workspaceId,
      viewId,
      kind,
      emails,
      value,
      account,
    );
    intents.add(intent);
    _accessChanged();
    try {
      final response = await _envelope(
        method,
        path,
        body: body,
        accessIntent: intent,
        timeout: const Duration(seconds: 20),
      );
      _checkAccount(account);
      intent.status = AppFlowyAccessMutationStatus.accepted;
      _accessChanged();
      return response;
    } on Object {
      final current =
          account.$2 == _accountEpoch &&
          (accountSubject == null || account.$1 == accountSubject!());
      if (!current || !intent.sent || intent.rejected) {
        intents.remove(intent);
      } else {
        intent.status = AppFlowyAccessMutationStatus.unconfirmed;
      }
      _accessChanged();
      rethrow;
    }
  }

  void _observeAccessMembers(
    String workspace,
    List<AppFlowyWorkspaceMember> members,
  ) {
    _accessIntents.removeWhere(
      (i) =>
          i.workspaceId == workspace &&
          i.viewId == null &&
          i.status != AppFlowyAccessMutationStatus.sending &&
          i.targets.every((email) {
            final found = members.where((m) => _accessEmail(m.email) == email);
            if (i.status == AppFlowyAccessMutationStatus.accepted) {
              return i.kind == AppFlowyAccessMutationKind.invite
                  ? found.isNotEmpty
                  : true;
            }
            return switch (i.kind) {
              AppFlowyAccessMutationKind.invite ||
              AppFlowyAccessMutationKind.role => found.any(
                (m) => m.role.wire == i.value,
              ),
              AppFlowyAccessMutationKind.remove => found.isEmpty,
              _ => false,
            };
          }),
    );
    _accessChanged();
  }

  void _observePageAccess(String workspace, AppFlowyPageAccess access) {
    _accessIntents.removeWhere(
      (i) =>
          i.workspaceId == workspace &&
          i.viewId == access.viewId &&
          i.status != AppFlowyAccessMutationStatus.sending &&
          i.targets.every((email) {
            final found = access.people.where(
              (p) =>
                  p.viewId == access.viewId && _accessEmail(p.email) == email,
            );
            if (i.status == AppFlowyAccessMutationStatus.accepted) return true;
            return switch (i.kind) {
              AppFlowyAccessMutationKind.share => found.any(
                (p) => p.access.wire.toString() == i.value,
              ),
              AppFlowyAccessMutationKind.revoke => found.isEmpty,
              _ => false,
            };
          }),
    );
    _accessChanged();
  }
}
