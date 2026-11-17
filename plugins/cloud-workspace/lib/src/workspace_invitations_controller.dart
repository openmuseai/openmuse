import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Account-lifetime presentation over original Cloud invitations. Server
/// membership and invitation state remain authoritative; no local grant exists.
final class WorkspaceInvitationsController extends ChangeNotifier {
  WorkspaceInvitationsController({
    required this.authentication,
    required this.service,
    this.onJoined,
    this.requestTimeout = const Duration(seconds: 20),
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_authenticationChanged);
  }
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final Future<void> Function()? onJoined;
  final Duration requestTimeout;
  final String? _subject;
  bool _disposed = false, retired = false, busy = false;
  String? failure;
  List<AppFlowyWorkspaceInvitation> _invitations = const [];
  List<AppFlowyWorkspaceInvitation> get invitations => _invitations;
  final _unconfirmed = <String>{};
  Set<String> get unconfirmedIds => Set.unmodifiable(_unconfirmed);
  bool get _active => !_disposed && !retired;

  void _authenticationChanged() {
    if (!authentication.snapshot.isAuthenticated ||
        _subject == null ||
        authentication.snapshot.identity?.subject != _subject) {
      retired = true;
      _invitations = const [];
      _unconfirmed.clear();
      failure = 'account_changed';
      if (!_disposed) notifyListeners();
    }
  }

  bool canAccept(AppFlowyWorkspaceInvitation invitation) =>
      _active &&
      !busy &&
      !_unconfirmed.contains(invitation.id) &&
      _invitations.any(
        (e) =>
            e.id == invitation.id &&
            e.workspaceId == invitation.workspaceId &&
            e.status == AppFlowyWorkspaceInvitationStatus.pending,
      );

  Future<void> _load() async {
    final loaded = await service.workspaceInvitations(timeout: requestTimeout);
    if (!_active) return;
    var joined = false;
    for (final invitation in loaded) {
      if (_unconfirmed.contains(invitation.id) &&
          {
            AppFlowyWorkspaceInvitationStatus.accepted,
            AppFlowyWorkspaceInvitationStatus.rejected,
          }.contains(invitation.status)) {
        joined |=
            invitation.status == AppFlowyWorkspaceInvitationStatus.accepted;
        _unconfirmed.remove(invitation.id);
      }
    }
    _invitations = loaded;
    if (joined && _active) await onJoined?.call();
  }

  Future<void> refresh() => _run(_load);

  Future<void> accept(String invitationId) => _run(() async {
    final opening = _invitations.where((e) => e.id == invitationId).firstOrNull;
    if (opening == null ||
        _unconfirmed.contains(invitationId) ||
        opening.status != AppFlowyWorkspaceInvitationStatus.pending)
      return;
    final current = await service.workspaceInvitation(
      invitationId,
      timeout: requestTimeout,
    );
    if (!_active) return;
    if (current.workspaceId != opening.workspaceId) {
      failure = 'invitation_changed';
      return;
    }
    if (current.status != AppFlowyWorkspaceInvitationStatus.pending) {
      await _load();
      if (_active &&
          current.status == AppFlowyWorkspaceInvitationStatus.accepted) {
        await onJoined?.call();
      }
      return;
    }
    // Mark before dispatch: even a successful server mutation can lose its
    // response. Refresh only reads; it never resubmits this unknown POST.
    _unconfirmed.add(invitationId);
    await service.acceptWorkspaceInvitation(
      invitationId,
      timeout: requestTimeout,
    );
    if (_active) await _load();
  });

  Future<void> _run(Future<void> Function() action) async {
    if (!_active || busy) return;
    _authenticationChanged();
    if (!_active) return;
    busy = true;
    failure = null;
    notifyListeners();
    try {
      await action();
    } catch (_) {
      if (_active)
        failure = _unconfirmed.isEmpty ? 'unavailable' : 'unconfirmed';
    } finally {
      busy = false;
      if (_active && _unconfirmed.isNotEmpty) failure = 'unconfirmed';
      if (!_disposed) notifyListeners();
    }
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    authentication.removeListener(_authenticationChanged);
    super.dispose();
  }
}
