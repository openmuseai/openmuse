import 'package:flutter/foundation.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

@immutable
final class WorkspaceCatalogEntry {
  const WorkspaceCatalogEntry({
    required this.page,
    required this.depth,
    required this.breadcrumb,
    required this.hasChildren,
    required this.expanded,
  });
  final AppFlowyWorkspacePage page;
  final int depth;
  final String breadcrumb;
  final bool hasChildren, expanded;
}

/// An account/workspace-scoped presentation of the original Folder catalog.
/// Expansion and search never mutate source objects or grant access.
final class WorkspaceCatalogController extends ChangeNotifier {
  WorkspaceCatalogController({
    required this.authentication,
    required this.service,
    required this.workspaceId,
  }) : _subject = authentication.snapshot.identity?.subject {
    authentication.addListener(_accountChanged);
    if (!authentication.snapshot.isAuthenticated || _subject == null) {
      retired = true;
    }
  }

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final String workspaceId;
  final String? _subject;
  List<AppFlowyWorkspacePage> _pages = const [];
  final _collapsed = <String>{};
  String _query = '';
  int _generation = 0;
  bool _disposed = false;
  bool busy = false, retired = false;
  String? failure, openingId;
  AppFlowyWorkspaceRole? role;
  String get query => _query;
  int get pageCount => _pages.length;
  bool get active => !_disposed && !retired;
  bool owns(AppFlowyWorkspacePage page) =>
      active && _pages.any((value) => identical(value, page));
  bool get canCreateDocument =>
      active &&
      !busy &&
      openingId == null &&
      {
        AppFlowyWorkspaceRole.owner,
        AppFlowyWorkspaceRole.member,
      }.contains(role);
  AppFlowyWorkspacePage? pageForId(String id) =>
      active ? _pages.where((page) => page.viewId == id).firstOrNull : null;

  void _accountChanged() {
    if (retired || _disposed) return;
    if (!authentication.snapshot.isAuthenticated ||
        authentication.snapshot.identity?.subject != _subject) {
      retire(reason: 'account');
    }
  }

  void retire({String reason = 'source'}) {
    if (!retired && !_disposed) {
      retired = true;
      _generation++;
      _pages = const [];
      role = null;
      _collapsed.clear();
      _query = '';
      busy = false;
      openingId = null;
      failure = reason;
      notifyListeners();
    }
  }

  void reportCreatedOpenFailure() {
    if (active) {
      failure = 'created_open';
      notifyListeners();
    }
  }

  Future<void> refresh() async {
    if (!active || busy || openingId != null) return;
    final generation = ++_generation;
    busy = true;
    failure = null;
    _pages = const [];
    role = null;
    notifyListeners();
    try {
      final currentRole = await service.workspaceRole(workspaceId);
      if (!active || generation != _generation) return;
      final pages = await service.workspacePages(workspaceId);
      if (!active || generation != _generation) return;
      final ids = <String>{};
      for (final page in pages) {
        if (page.workspaceId != workspaceId ||
            !ids.add(page.viewId) ||
            page.ancestors.length > 12 ||
            page.ancestors.contains(page.viewId)) {
          throw const FormatException('Invalid original catalog binding');
        }
      }
      _pages = List.unmodifiable(pages);
      role = currentRole;
      _collapsed.retainAll(ids);
    } on Object {
      if (active && generation == _generation) failure = 'load';
    } finally {
      if (active && generation == _generation) {
        busy = false;
        notifyListeners();
      }
    }
  }

  void search(String value) {
    if (!active) return;
    _query = value.trim().toLowerCase();
    notifyListeners();
  }

  void toggle(String viewId) {
    if (!active ||
        _query.isNotEmpty ||
        !_pages.any((page) => page.viewId == viewId))
      return;
    if (!_collapsed.add(viewId)) _collapsed.remove(viewId);
    notifyListeners();
  }

  List<WorkspaceCatalogEntry> get entries {
    final byId = {for (final page in _pages) page.viewId: page};
    final parents = <String>{};
    final matches = <String>{};
    for (final page in _pages) {
      final parent = page.ancestors.reversed
          .where(byId.containsKey)
          .firstOrNull;
      if (parent != null) parents.add(parent);
      if (_query.isNotEmpty && page.name.toLowerCase().contains(_query)) {
        matches.add(page.viewId);
        matches.addAll(page.ancestors.where(byId.containsKey));
      }
    }
    return List.unmodifiable([
      for (final page in _pages)
        if (_query.isNotEmpty
            ? matches.contains(page.viewId)
            : !page.ancestors.any(_collapsed.contains))
          WorkspaceCatalogEntry(
            page: page,
            depth: page.ancestors.where(byId.containsKey).length,
            breadcrumb: page.ancestors
                .where(byId.containsKey)
                .map((id) => byId[id]!.name)
                .join(' / '),
            hasChildren: parents.contains(page.viewId),
            expanded: _query.isNotEmpty || !_collapsed.contains(page.viewId),
          ),
    ]);
  }

  /// True only if the same presentation still owns the completed navigation.
  Future<bool> open(
    AppFlowyWorkspacePage page,
    Future<void> Function(AppFlowyWorkspacePage) onOpen,
  ) async {
    if (!active || busy || openingId != null || !owns(page)) return false;
    final generation = _generation;
    openingId = page.viewId;
    failure = null;
    notifyListeners();
    try {
      await onOpen(page);
      return active && generation == _generation;
    } on Object {
      if (active && generation == _generation) failure = 'open';
      return false;
    } finally {
      if (active && generation == _generation) {
        openingId = null;
        notifyListeners();
      }
    }
  }

  @override
  void dispose() {
    _disposed = true;
    _generation++;
    authentication.removeListener(_accountChanged);
    super.dispose();
  }
}
