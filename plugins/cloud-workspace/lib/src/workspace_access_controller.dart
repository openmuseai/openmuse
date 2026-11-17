import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Presentation state over the original Cloud membership/sharing domain.
/// A confirmed response is the only source of access details. Account changes
/// retire this controller permanently; late operations cannot populate a new UI.
final class WorkspaceAccessController extends ChangeNotifier {
  WorkspaceAccessController({
    required this.authentication,
    required this.service,
    required this.workspaceId,
    this.viewId,
    this.ancestors = const [],
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_authenticationChanged);
    _deliverySubscription = service.accessMutationChanges.listen((_) {
      if (_disposed || retired) return;
      if (failure == 'unconfirmed' &&
          !pendingMutations.any(
            (i) => i.status == AppFlowyAccessMutationStatus.unconfirmed,
          ))
        failure = null;
      notifyListeners();
    });
  }

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final String workspaceId;
  final String? viewId;
  final List<String> ancestors;
  final String? _subject;
  late final StreamSubscription<void> _deliverySubscription;
  bool _disposed = false, retired = false, busy = false;
  String? failure;
  AppFlowyWorkspaceRole? role;
  int _confirmedRevision = 0;
  int get confirmedRevision => _confirmedRevision;
  List<AppFlowyWorkspaceMember> members = const [];
  List<AppFlowySharedPerson> people = const [];

  bool get canManageMembers =>
      !_disposed && !retired && role == AppFlowyWorkspaceRole.owner;
  bool get canShare =>
      !_disposed &&
      !retired &&
      (role == AppFlowyWorkspaceRole.owner ||
          role == AppFlowyWorkspaceRole.member);

  void _authenticationChanged() {
    if (!authentication.snapshot.isAuthenticated ||
        authentication.snapshot.identity?.subject != _subject) {
      retired = true;
      role = null;
      members = const [];
      people = const [];
      failure = 'account_changed';
      if (!_disposed) notifyListeners();
    }
  }

  Future<void> refresh() => _run(_load);

  Future<void> _load() async {
    if (_disposed || retired) return;
    final loadedRole = await service.workspaceRole(workspaceId);
    if (_disposed || retired) return;
    // Original sharing management is limited to workspace members. A Guest
    // grant, including FullAccess, does not confer membership administration.
    final loadedMembers = loadedRole == AppFlowyWorkspaceRole.guest
        ? const <AppFlowyWorkspaceMember>[]
        : await service.workspaceMembers(workspaceId);
    if (_disposed || retired) return;
    final loadedPeople =
        viewId == null || loadedRole == AppFlowyWorkspaceRole.guest
        ? const <AppFlowySharedPerson>[]
        : (await service.pageAccess(
            workspaceId,
            viewId!,
            ancestors: ancestors,
          )).people;
    if (_disposed || retired) return;
    role = loadedRole;
    members = loadedMembers;
    people = loadedPeople;
    _confirmedRevision++;
  }

  Future<void> invite(String email, AppFlowyWorkspaceRole newRole) => _run(
    () async {
      if (!canManageMembers) throw StateError('owner required');
      await service.inviteWorkspaceMembers(workspaceId, [email], role: newRole);
    },
    mutation: true,
  );

  Future<void> changeRole(String email, AppFlowyWorkspaceRole newRole) =>
      _run(() async {
        if (!canManageMembers) throw StateError('owner required');
        await service.updateWorkspaceMember(workspaceId, email, newRole);
      }, mutation: true);

  Future<void> removeMember(String email) => _run(() async {
    if (!canManageMembers) throw StateError('owner required');
    await service.removeWorkspaceMembers(workspaceId, [email]);
  }, mutation: true);

  Future<void> share(String email, AppFlowyAccessLevel access) =>
      _run(() async {
        if (!canShare || viewId == null) throw StateError('member required');
        await service.shareView(workspaceId, viewId!, [email], access);
      }, mutation: true);

  Future<void> revoke(AppFlowySharedPerson person) => _run(() async {
    if (!canShare || viewId == null || person.viewId != viewId) {
      throw StateError('inherited grants must be managed at their source');
    }
    await service.revokeViewAccess(workspaceId, viewId!, [person.email]);
  }, mutation: true);

  Future<void> _run(
    Future<void> Function() action, {
    bool mutation = false,
  }) async {
    if (_disposed || retired || busy) return;
    _authenticationChanged();
    if (retired) return;
    busy = true;
    failure = null;
    notifyListeners();
    try {
      await action();
      if (mutation && !_disposed && !retired) {
        try {
          await _load();
        } on Object catch (error) {
          _readFailure(error);
          if (!_disposed && !retired) failure = 'refresh_unavailable';
        }
      }
      if (!_disposed &&
          !retired &&
          failure == null &&
          pendingMutations.any(
            (i) => i.status == AppFlowyAccessMutationStatus.unconfirmed,
          ))
        failure = 'unconfirmed';
    } on Object catch (error) {
      if (!_disposed && !retired) {
        if (mutation) {
          failure =
              pendingMutations.any(
                (i) => i.status == AppFlowyAccessMutationStatus.unconfirmed,
              )
              ? 'unconfirmed'
              : error is AppFlowyAccessMutationBlocked
              ? 'review_required'
              : 'rejected';
        } else {
          _readFailure(error);
          failure = 'unavailable';
        }
      }
    } finally {
      busy = false;
      if (!_disposed) notifyListeners();
    }
  }

  List<AppFlowyAccessMutation> get pendingMutations =>
      _disposed || retired || role == null
      ? const []
      : service.pendingAccessMutations(workspaceId, viewId: viewId);
  bool canMutateTarget(String email) =>
      !retired &&
      !busy &&
      !pendingMutations.any(
        (i) => i.targets.contains(email.trim().toLowerCase()),
      );
  void managementReadDenied() {
    if (_disposed || retired) return;
    role = null;
    members = const [];
    people = const [];
    failure = 'unavailable';
    notifyListeners();
  }

  void _readFailure(Object error) {
    if (error is CloudServiceException &&
        error.code == CloudServiceErrorCode.unauthorized) {
      role = null;
      members = const [];
      people = const [];
    }
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    authentication.removeListener(_authenticationChanged);
    unawaited(_deliverySubscription.cancel());
    super.dispose();
  }
}
