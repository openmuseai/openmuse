import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'workspace_access_controller.dart';

/// Read/cancel presentation over original invitation rows. No membership or
/// access permission is inferred from this list or written locally.
final class OwnerInvitationsController extends ChangeNotifier {
  OwnerInvitationsController(this.access) {
    _accessRevision = access.confirmedRevision;
    access.addListener(_authorityChanged);
  }
  final WorkspaceAccessController access;
  bool _disposed = false,
      retired = false,
      busy = false,
      unsupported = false,
      fresh = false;
  int _generation = 0;
  late int _accessRevision;
  bool _readAgain = false;
  List<AppFlowyOwnerWorkspaceInvitation> items = const [];
  AppFlowyOwnerInvitationCursor? cursor;
  String? email, failure, notice;
  bool get canLoadMore =>
      !_disposed && !busy && !retired && cursor != null && items.length < 500;
  bool canCancel(AppFlowyOwnerWorkspaceInvitation row) =>
      !busy &&
      !retired &&
      fresh &&
      access.canManageMembers &&
      row.canCancel &&
      items.any(
        (r) =>
            r.id == row.id &&
            r.updatedAt == row.updatedAt &&
            r.roleId == row.roleId &&
            r.email == row.email &&
            r.wireStatus == row.wireStatus,
      ) &&
      !access.service.ownerInvitationCancellationPending(
        access.workspaceId,
        row.id,
      );
  void _authorityChanged() {
    if (_disposed || retired) return;
    if (!access.canManageMembers) {
      retired = true;
      _generation++;
      items = const [];
      cursor = null;
      fresh = false;
      if (!_disposed) notifyListeners();
    }
    if (!retired && _accessRevision != access.confirmedRevision) {
      _accessRevision = access.confirmedRevision;
      if (busy) {
        _readAgain = true;
      } else {
        unawaited(refresh(filter: email));
      }
    }
  }

  bool _current(int generation) =>
      !_disposed &&
      !retired &&
      generation == _generation &&
      access.canManageMembers;
  Future<void> refresh({String? filter}) => _read(filter: filter);
  Future<void> loadMore() =>
      canLoadMore ? _read(more: true, filter: email) : Future.value();
  Future<void> _read({bool more = false, String? filter}) async {
    if (_disposed || retired || busy) return;
    _authorityChanged();
    if (retired) return;
    final generation = _generation;
    busy = true;
    failure = null;
    notice = null;
    notifyListeners();
    try {
      final nextFilter = filter?.trim();
      final page = await access.service.ownerWorkspaceInvitations(
        access.workspaceId,
        email: nextFilter == null || nextFilter.isEmpty ? null : nextFilter,
        cursor: more ? cursor : null,
      );
      if (!_current(generation)) return;
      unsupported = false;
      fresh = true;
      if (more) {
        final byId = {for (final row in items) row.id: row};
        for (final row in page.items) {
          byId[row.id] = row;
        }
        items = List.unmodifiable(byId.values.take(500));
      } else {
        items = page.items;
        email = nextFilter;
      }
      cursor = page.nextCursor;
    } on AppFlowyInvitationManagementUnsupported {
      if (_current(generation)) {
        unsupported = true;
        items = const [];
        cursor = null;
        fresh = false;
      }
    } on Object catch (error) {
      if (_current(generation)) {
        fresh = false;
        failure = error is ArgumentError ? 'invalid_filter' : 'unavailable';
        if (error is CloudServiceException &&
            error.code == CloudServiceErrorCode.unauthorized) {
          items = const [];
          cursor = null;
          access.managementReadDenied();
        }
      }
    } finally {
      busy = false;
      if (!_disposed) {
        notifyListeners();
        if (_readAgain && !retired) {
          _readAgain = false;
          unawaited(refresh(filter: email));
        }
      }
    }
  }

  Future<void> cancel(AppFlowyOwnerWorkspaceInvitation observed) async {
    if (!canCancel(observed)) return;
    final generation = _generation;
    busy = true;
    failure = null;
    notice = null;
    notifyListeners();
    try {
      final result = await access.service.cancelOwnerWorkspaceInvitation(
        observed,
      );
      if (!_current(generation)) return;
      items = List.unmodifiable(
        items.map((r) => r.id == observed.id ? result.invitation : r),
      );
      notice = result.outcome;
      if (['changed', 'unsupported'].contains(result.outcome)) fresh = false;
    } on Object catch (error) {
      if (_current(generation)) {
        fresh = false;
        failure = 'unconfirmed';
        if (error is CloudServiceException &&
            error.code == CloudServiceErrorCode.unauthorized) {
          items = const [];
          cursor = null;
          access.managementReadDenied();
        }
      }
    } finally {
      busy = false;
      if (!_disposed) {
        notifyListeners();
        if (_readAgain && !retired) {
          _readAgain = false;
          unawaited(refresh(filter: email));
        }
      }
    }
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    retired = true;
    fresh = false;
    cursor = null;
    _generation++;
    items = const [];
    access.removeListener(_authorityChanged);
    super.dispose();
  }
}
