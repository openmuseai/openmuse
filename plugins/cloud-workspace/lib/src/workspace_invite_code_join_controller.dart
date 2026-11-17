import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// A preview never grants membership. Only original Cloud acceptance followed
/// by the original role/catalog reads can update the joining presentation.
final class WorkspaceInviteCodeJoinController extends ChangeNotifier {
  WorkspaceInviteCodeJoinController({
    required this.authentication,
    required this.service,
    this.onJoined,
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_accountChanged);
  }
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final Future<void> Function()? onJoined;
  final String? _subject;
  bool _disposed = false, retired = false, busy = false;
  int _generation = 0;
  AppFlowyWorkspaceInviteCodePreview? preview;
  AppFlowyWorkspaceRole? joinedRole;
  String? failure;
  bool get _active => !_disposed && !retired;
  void _accountChanged() {
    if (_active &&
        (!authentication.snapshot.isAuthenticated ||
            _subject == null ||
            authentication.snapshot.identity?.subject != _subject)) {
      retired = true;
      _generation++;
      preview = null;
      joinedRole = null;
      failure = 'account_changed';
      notifyListeners();
    }
  }

  void draftChanged() {
    if (!_active) return;
    _generation++;
    preview = null;
    joinedRole = null;
    failure = null;
    notifyListeners();
  }

  bool canJoin(AppFlowyWorkspaceInviteCodePreview value) =>
      _active &&
      !busy &&
      identical(preview, value) &&
      !value.isMember &&
      joinedRole == null &&
      service
          .pendingInviteCodeMutations(value.workspaceId)
          .where((i) => i.kind == AppFlowyAccessMutationKind.joinInviteCode)
          .isEmpty;
  Future<void> read(String code) async {
    if (!_active || busy) return;
    _accountChanged();
    if (!_active) return;
    final generation = ++_generation;
    busy = true;
    failure = null;
    preview = null;
    joinedRole = null;
    notifyListeners();
    try {
      final value = await service.previewWorkspaceInviteCode(code.trim());
      if (_active && generation == _generation) preview = value;
    } catch (_) {
      if (_active && generation == _generation) failure = 'unavailable';
    } finally {
      busy = false;
      if (!_disposed) notifyListeners();
    }
  }

  Future<void> join(AppFlowyWorkspaceInviteCodePreview value) async {
    if (!canJoin(value)) return;
    final generation = _generation;
    busy = true;
    failure = null;
    notifyListeners();
    try {
      final role = await service.joinWorkspaceByInviteCode(value);
      if (!_active || generation != _generation) return;
      joinedRole = role;
      try {
        await onJoined?.call();
      } catch (_) {
        if (_active) failure = 'catalog_unavailable';
      }
    } catch (_) {
      if (_active && generation == _generation) {
        final pending = service.pendingInviteCodeMutations(value.workspaceId);
        failure =
            pending.any(
              (i) => i.status == AppFlowyAccessMutationStatus.unconfirmed,
            )
            ? 'unconfirmed'
            : 'unavailable';
      }
    } finally {
      busy = false;
      if (!_disposed) notifyListeners();
    }
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    retired = true;
    preview = null;
    joinedRole = null;
    _generation++;
    authentication.removeListener(_accountChanged);
    super.dispose();
  }
}
