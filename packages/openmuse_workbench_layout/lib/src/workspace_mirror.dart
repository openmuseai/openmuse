import 'package:flutter/foundation.dart';

/// Browser-visible references are scoped to one authorized Desktop workspace.
/// Implementations must never return Desktop absolute file paths.
final class WorkspaceMirrorMount {
  const WorkspaceMirrorMount({
    required this.mountRef,
    required this.rootRef,
    required this.title,
  });

  final String mountRef;
  final String rootRef;
  final String title;
}

final class WorkspaceMirrorEntry {
  const WorkspaceMirrorEntry({
    required this.nodeRef,
    required this.name,
    required this.isDirectory,
    this.resourceRef,
    this.mediaType,
  });

  final String nodeRef;
  final String name;
  final bool isDirectory;
  final String? resourceRef;
  final String? mediaType;
}

final class WorkspaceMirrorPage {
  const WorkspaceMirrorPage({required this.entries, this.nextCursor});

  final List<WorkspaceMirrorEntry> entries;
  final String? nextCursor;
}

abstract interface class WorkspaceMirrorPort {
  Future<List<WorkspaceMirrorMount>> listMounts(String workspaceRef);

  Future<WorkspaceMirrorPage> listChildren({
    required String workspaceRef,
    required String mountRef,
    required String parentRef,
    required int limit,
    String? cursor,
  });
}

abstract interface class WorkspaceResourcePort {
  Future<Uint8List> readResource({
    required String workspaceRef,
    required String resourceRef,
  });
}

final class WorkspaceMirrorNode {
  WorkspaceMirrorNode({
    required this.mountRef,
    required this.nodeRef,
    required this.name,
    required this.isDirectory,
    required int generation,
    this.resourceRef,
    this.mediaType,
  }) : _generation = generation;

  final String mountRef;
  final String nodeRef;
  final String name;
  final bool isDirectory;
  final String? resourceRef;
  final String? mediaType;
  final int _generation;
  final List<WorkspaceMirrorNode> children = [];
  bool expanded = false;
  bool loaded = false;
  bool loading = false;
  Object? error;
  String? nextCursor;
  int _requestEpoch = 0;
}

/// Loads mounts on connection, but requests one directory only when expanded.
/// Switching Desktop/workspace invalidates all outstanding page responses.
final class WorkspaceMirrorController extends ChangeNotifier {
  WorkspaceMirrorController({this.pageSize = 100}) {
    if (pageSize < 1 || pageSize > 200) {
      throw ArgumentError.value(pageSize, 'pageSize', 'Must be 1–200.');
    }
  }

  final int pageSize;
  WorkspaceMirrorPort? _port;
  String? _workspaceRef;
  int _generation = 0;
  List<WorkspaceMirrorNode> _mounts = [];
  Object? _mountError;
  bool _mountsLoading = false;

  List<WorkspaceMirrorNode> get mounts => List.unmodifiable(_mounts);
  Object? get mountError => _mountError;
  bool get mountsLoading => _mountsLoading;
  String? get workspaceRef => _workspaceRef;

  Future<void> connect(String workspaceRef, WorkspaceMirrorPort port) async {
    final generation = ++_generation;
    _workspaceRef = workspaceRef;
    _port = port;
    _mounts = [];
    _mountError = null;
    _mountsLoading = true;
    notifyListeners();
    try {
      final mounts = await port.listMounts(workspaceRef);
      if (generation != _generation) return;
      _mounts = [
        for (final mount in mounts)
          WorkspaceMirrorNode(
            mountRef: mount.mountRef,
            nodeRef: mount.rootRef,
            name: mount.title,
            isDirectory: true,
            generation: generation,
          ),
      ];
    } on Object catch (error) {
      if (generation != _generation) return;
      _mountError = error;
    } finally {
      if (generation == _generation) {
        _mountsLoading = false;
        notifyListeners();
      }
    }
  }

  void disconnect() {
    _generation++;
    _port = null;
    _workspaceRef = null;
    _mounts = [];
    _mountError = null;
    _mountsLoading = false;
    notifyListeners();
  }

  Future<void> expand(WorkspaceMirrorNode node) async {
    if (!node.isDirectory || node._generation != _generation) return;
    node.expanded = true;
    notifyListeners();
    if (!node.loaded && !node.loading) await _load(node, cursor: null);
  }

  void collapse(WorkspaceMirrorNode node) {
    if (!node.isDirectory || node._generation != _generation || !node.expanded)
      return;
    node.expanded = false;
    notifyListeners();
  }

  Future<void> loadMore(WorkspaceMirrorNode node) async {
    final cursor = node.nextCursor;
    if (!node.isDirectory ||
        node._generation != _generation ||
        !node.loaded ||
        node.loading ||
        cursor == null) {
      return;
    }
    await _load(node, cursor: cursor);
  }

  Future<void> refresh(WorkspaceMirrorNode node) async {
    if (!node.isDirectory || node._generation != _generation) return;
    node._requestEpoch++;
    node.children.clear();
    node.loaded = false;
    node.loading = false;
    node.nextCursor = null;
    node.error = null;
    notifyListeners();
    if (node.expanded) await _load(node, cursor: null);
  }

  Future<void> _load(
    WorkspaceMirrorNode node, {
    required String? cursor,
  }) async {
    final port = _port;
    final workspaceRef = _workspaceRef;
    if (port == null || workspaceRef == null) return;
    final generation = _generation;
    final epoch = ++node._requestEpoch;
    node.loading = true;
    node.error = null;
    notifyListeners();
    try {
      final page = await port.listChildren(
        workspaceRef: workspaceRef,
        mountRef: node.mountRef,
        parentRef: node.nodeRef,
        limit: pageSize,
        cursor: cursor,
      );
      if (generation != _generation || epoch != node._requestEpoch) return;
      final known = {for (final child in node.children) child.nodeRef};
      for (final entry in page.entries) {
        if (!known.add(entry.nodeRef)) continue;
        node.children.add(
          WorkspaceMirrorNode(
            mountRef: node.mountRef,
            nodeRef: entry.nodeRef,
            name: entry.name,
            isDirectory: entry.isDirectory,
            generation: generation,
            resourceRef: entry.resourceRef,
            mediaType: entry.mediaType,
          ),
        );
      }
      node.loaded = true;
      node.nextCursor = page.nextCursor;
    } on Object catch (error) {
      if (generation != _generation || epoch != node._requestEpoch) return;
      node.error = error;
    } finally {
      if (generation == _generation && epoch == node._requestEpoch) {
        node.loading = false;
        notifyListeners();
      }
    }
  }

  @override
  void dispose() {
    _generation++;
    super.dispose();
  }
}
