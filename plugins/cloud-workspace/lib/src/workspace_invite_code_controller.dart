import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'workspace_access_controller.dart';

/// Original code management is independent from email invitation and member
/// mutation presentation. All authority comes from existing access/server roles.
final class WorkspaceInviteCodeController extends ChangeNotifier {
  WorkspaceInviteCodeController(this.access) {
    access.addListener(_authorityChanged);
  }
  final WorkspaceAccessController access;
  bool _disposed = false, retired = false, busy = false, fresh = false;
  AppFlowyWorkspaceInviteCode? observation;
  String? failure, notice;
  bool get unknown => access.service
      .pendingInviteCodeMutations(access.workspaceId)
      .any((i) => i.status == AppFlowyAccessMutationStatus.unconfirmed);
  bool current(AppFlowyWorkspaceInviteCode value) =>
      !_disposed &&
      !retired &&
      access.canManageMembers &&
      fresh &&
      identical(value, observation) &&
      access.service.workspaceInviteCodeIsCurrent(value);
  bool canWrite(AppFlowyWorkspaceInviteCode value) =>
      current(value) && !busy && !unknown;
  bool canCopy(AppFlowyWorkspaceInviteCode value) =>
      current(value) && !busy && value.code != null;
  void _authorityChanged() {
    if (_disposed || retired) return;
    if (!access.canManageMembers) {
      retired = true;
      fresh = false;
      observation = null;
      failure = 'account_changed';
      notifyListeners();
    }
  }

  Future<void> refresh() async {
    if (_disposed || retired || busy) return;
    _authorityChanged();
    if (retired) return;
    busy = true;
    failure = null;
    notice = null;
    notifyListeners();
    try {
      final read = await access.service.workspaceInviteCode(access.workspaceId);
      if (!_disposed && !retired) {
        observation = read;
        fresh = true;
      }
    } catch (error) {
      if (!_disposed && !retired) {
        fresh = false;
        failure = 'unavailable';
        _denied(error);
      }
    } finally {
      busy = false;
      if (!_disposed) notifyListeners();
    }
  }

  Future<void> generate(AppFlowyWorkspaceInviteCode value, int? hours) =>
      _write(value, () async {
        final next = await access.service.createWorkspaceInviteCode(
          value,
          validityPeriodHours: hours,
        );
        if (!_disposed && !retired) {
          observation = next;
          fresh = true;
          notice = 'generated';
        }
      });
  Future<void> disable(AppFlowyWorkspaceInviteCode value) =>
      _write(value, () async {
        final next = await access.service.deleteWorkspaceInviteCode(value);
        if (!_disposed && !retired) {
          observation = next;
          fresh = true;
          notice = 'disabled';
        }
      });
  Future<void> _write(
    AppFlowyWorkspaceInviteCode value,
    Future<void> Function() action,
  ) async {
    if (!canWrite(value)) return;
    busy = true;
    failure = null;
    notice = null;
    notifyListeners();
    try {
      await action();
    } catch (error) {
      if (!_disposed && !retired) {
        fresh = false;
        failure = unknown ? 'unconfirmed' : 'unavailable';
        _denied(error);
      }
    } finally {
      busy = false;
      if (!_disposed) notifyListeners();
    }
  }

  void _denied(Object error) {
    if (error is CloudServiceException &&
        error.code == CloudServiceErrorCode.unauthorized) {
      observation = null;
      fresh = false;
      access.managementReadDenied();
    }
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    retired = true;
    fresh = false;
    observation = null;
    access.removeListener(_authorityChanged);
    super.dispose();
  }
}
