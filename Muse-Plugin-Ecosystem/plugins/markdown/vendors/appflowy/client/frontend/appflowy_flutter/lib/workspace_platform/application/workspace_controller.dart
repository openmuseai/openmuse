import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_workspace_bridge.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_persistence.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as p;

typedef MuseWorkspaceDshPublisher = Future<void> Function(
  String accountSpaceRef,
  String title,
  List<MuseWorkspaceMount> mounts,
  String? activeMountRef,
);

final class MuseWorkspaceController extends ChangeNotifier {
  MuseWorkspaceController({
    required this.providers,
    required this.persistence,
    MuseWorkspaceDshPublisher? dshPublisher,
  }) : _dshPublisher = dshPublisher ?? _publishWithHostBridge;

  final MuseWorkspaceProviderRegistry providers;
  final MuseWorkspacePersistence persistence;
  final MuseWorkspaceDshPublisher _dshPublisher;

  static Future<void> _publishWithHostBridge(
    String accountSpaceRef,
    String title,
    List<MuseWorkspaceMount> mounts,
    String? activeMountRef,
  ) =>
      DshWorkspaceBridge.publishProjectWorkspace(
        appflowyWorkspaceId: accountSpaceRef,
        title: title,
        mounts: mounts,
        activeMountRef: activeMountRef,
      );

  String? accountSpaceRef;
  String accountSpaceTitle = 'Workspace';
  bool loading = false;
  String? error;
  int _generation = 0;
  Timer? _watchDebounce;

  final List<MuseWorkspaceMount> mounts = [];
  final Map<String, MuseWorkspaceEntry> roots = {};
  final Map<String, List<MuseWorkspaceEntry>> children = {};
  final Set<String> expandedEntryRefs = {};
  final Set<String> loadingEntryRefs = {};
  final Map<String, StreamSubscription<void>> _watchers = {};
  String? selectedEntryRef;

  /// Mount the next DSH session should start in; null before the first mount.
  String? activeMountRef;

  void select(String entryRef) {
    if (selectedEntryRef == entryRef) return;
    selectedEntryRef = entryRef;
    final mountRef = entryByRef(entryRef)?.mountRef;
    if (mountRef != null && mountRef != activeMountRef) {
      activeMountRef = mountRef;
      unawaited(_persist());
      unawaited(_publishDshBinding());
    }
    notifyListeners();
  }

  /// Marks one Mount as the active Mount for DSH sessions.
  Future<void> setActiveMount(String mountRef) async {
    if (activeMountRef == mountRef) return;
    if (!mounts.any((mount) => mount.mountRef == mountRef)) return;
    activeMountRef = mountRef;
    await _persist();
    await _publishDshBinding();
    notifyListeners();
  }

  Future<void> open({
    required String accountSpaceRef,
    required String title,
  }) async {
    if (this.accountSpaceRef == accountSpaceRef && !loading) return;
    final generation = ++_generation;
    await _cancelWatchers();
    this.accountSpaceRef = accountSpaceRef;
    accountSpaceTitle = title;
    loading = true;
    error = null;
    mounts.clear();
    roots.clear();
    children.clear();
    expandedEntryRefs.clear();
    selectedEntryRef = null;
    activeMountRef = null;
    notifyListeners();
    try {
      final snapshot = await persistence.load(accountSpaceRef);
      if (generation != _generation) return;
      mounts.addAll(snapshot.mounts);
      expandedEntryRefs.addAll(snapshot.expandedEntryRefs);
      activeMountRef = _resolveActiveMount(snapshot.activeMountRef);
      for (final mount in mounts) {
        await _bindMount(mount, generation);
      }
      for (final root in roots.values) {
        if (expandedEntryRefs.contains(root.entryRef)) {
          await _loadChildren(root, generation: generation, persist: false);
        }
      }
      await _publishDshBinding();
    } on Object catch (exception) {
      if (generation == _generation) error = exception.toString();
    } finally {
      if (generation == _generation) {
        loading = false;
        notifyListeners();
      }
    }
  }

  Future<void> mountLocalDirectory(String path) async {
    final workspace = accountSpaceRef;
    if (workspace == null) throw StateError('No project workspace is open');
    final canonical = await Directory(path).resolveSymbolicLinks();
    if (mounts.any((mount) => mount.rootLocator == canonical)) return;
    final digest = sha256.convert(utf8.encode('$workspace:$canonical'));
    final mount = MuseWorkspaceMount(
      mountRef: 'mount:$digest',
      providerId: MuseLocalWorkspaceProvider.providerId,
      bindingKey: 'device-local:$digest',
      displayName: p.basename(canonical),
      rootLocator: canonical,
      readOnly: false,
      order: mounts.length,
    );
    mounts.add(mount);
    activeMountRef ??= mount.mountRef;
    await _bindMount(mount, _generation);
    final root = roots[mount.mountRef];
    if (root != null) {
      expandedEntryRefs.add(root.entryRef);
      await _loadChildren(root, generation: _generation, persist: false);
    }
    await _persist();
    await _publishDshBinding();
    notifyListeners();
  }

  Future<void> unmount(String mountRef) async {
    await _watchers.remove(mountRef)?.cancel();
    final root = roots.remove(mountRef);
    if (root != null) _dropSubtree(root.entryRef);
    mounts.removeWhere((mount) => mount.mountRef == mountRef);
    for (var index = 0; index < mounts.length; index++) {
      final mount = mounts[index];
      if (mount.order == index) continue;
      mounts[index] = MuseWorkspaceMount(
        mountRef: mount.mountRef,
        providerId: mount.providerId,
        bindingKey: mount.bindingKey,
        displayName: mount.displayName,
        rootLocator: mount.rootLocator,
        readOnly: mount.readOnly,
        order: index,
      );
    }
    if (activeMountRef == mountRef) activeMountRef = _resolveActiveMount(null);
    await _persist();
    await _publishDshBinding();
    notifyListeners();
  }

  Future<void> toggleExpanded(MuseWorkspaceEntry entry) async {
    if (!entry.isDirectory) return;
    if (expandedEntryRefs.remove(entry.entryRef)) {
      await _persist();
      notifyListeners();
      return;
    }
    expandedEntryRefs.add(entry.entryRef);
    notifyListeners();
    await _loadChildren(entry, generation: _generation);
  }

  Future<void> refresh(MuseWorkspaceEntry entry) =>
      _loadChildren(entry, generation: _generation, force: true);

  Future<MuseWorkspaceEntry> createFile(
    MuseWorkspaceEntry parent,
    String name,
  ) async {
    final mount = requireMount(parent.mountRef);
    final provider = providers.require(mount.providerId);
    final created = await provider.createFile(
      mount: mount,
      parent: parent,
      name: name,
    );
    await refresh(parent);
    return created;
  }

  Future<MuseWorkspaceEntry> createDirectory(
    MuseWorkspaceEntry parent,
    String name,
  ) async {
    final mount = requireMount(parent.mountRef);
    final provider = providers.require(mount.providerId);
    final created = await provider.createDirectory(
      mount: mount,
      parent: parent,
      name: name,
    );
    await refresh(parent);
    return created;
  }

  Future<MuseWorkspaceEntry> rename(
    MuseWorkspaceEntry entry,
    String name,
  ) async {
    final mount = requireMount(entry.mountRef);
    final provider = providers.require(mount.providerId);
    final renamed = await provider.rename(
      mount: mount,
      entry: entry,
      newName: name,
    );
    _dropSubtree(entry.entryRef);
    final parent =
        entry.parentEntryRef == null ? null : entryByRef(entry.parentEntryRef!);
    if (parent == null) {
      roots[entry.mountRef] = renamed;
    } else {
      await refresh(parent);
    }
    notifyListeners();
    return renamed;
  }

  Future<void> delete(MuseWorkspaceEntry entry) async {
    final mount = requireMount(entry.mountRef);
    final provider = providers.require(mount.providerId);
    final parent =
        entry.parentEntryRef == null ? null : entryByRef(entry.parentEntryRef!);
    await provider.delete(mount: mount, entry: entry);
    _dropSubtree(entry.entryRef);
    if (parent != null) await refresh(parent);
    notifyListeners();
  }

  Future<List<MuseWorkspaceEntry>> importFiles(
    MuseWorkspaceEntry parent,
    List<File> files,
  ) async {
    final mount = requireMount(parent.mountRef);
    final provider = providers.require(mount.providerId);
    final imported = await provider.importFiles(
      mount: mount,
      parent: parent,
      files: files,
    );
    await refresh(parent);
    return imported;
  }

  MuseWorkspaceMount requireMount(String mountRef) =>
      mounts.firstWhere((mount) => mount.mountRef == mountRef);

  /// Active Mount, preferring the persisted choice, then a local Mount.
  String? _resolveActiveMount(String? preferred) {
    if (mounts.isEmpty) return null;
    if (preferred != null && mounts.any((mount) => mount.mountRef == preferred)) {
      return preferred;
    }
    for (final mount in mounts) {
      if (mount.providerId == MuseLocalWorkspaceProvider.providerId) {
        return mount.mountRef;
      }
    }
    return mounts.first.mountRef;
  }

  MuseWorkspaceEntry? entryByRef(String entryRef) {
    for (final root in roots.values) {
      if (root.entryRef == entryRef) return root;
    }
    for (final entries in children.values) {
      for (final entry in entries) {
        if (entry.entryRef == entryRef) return entry;
      }
    }
    return null;
  }

  Future<void> _bindMount(MuseWorkspaceMount mount, int generation) async {
    final provider = providers.require(mount.providerId);
    final root = await provider.bind(mount);
    if (generation != _generation) return;
    roots[mount.mountRef] = root;
    _watchers[mount.mountRef] = provider.watch(mount).listen(
          (_) => _scheduleWatchRefresh(),
          onError: (_) => _scheduleWatchRefresh(),
        );
  }

  Future<void> _loadChildren(
    MuseWorkspaceEntry parent, {
    required int generation,
    bool force = false,
    bool persist = true,
  }) async {
    if (!parent.isDirectory) return;
    if (!force && children.containsKey(parent.entryRef)) return;
    if (!loadingEntryRefs.add(parent.entryRef)) return;
    notifyListeners();
    try {
      final mount = requireMount(parent.mountRef);
      final provider = providers.require(mount.providerId);
      final loaded = await provider.listChildren(mount: mount, parent: parent);
      if (generation != _generation) return;
      children[parent.entryRef] = loaded;
      if (persist) await _persist();
    } on Object catch (exception) {
      if (generation == _generation) error = exception.toString();
    } finally {
      loadingEntryRefs.remove(parent.entryRef);
      if (generation == _generation) notifyListeners();
    }
  }

  void _dropSubtree(String entryRef) {
    final nested = children.remove(entryRef) ?? const [];
    expandedEntryRefs.remove(entryRef);
    for (final child in nested) {
      _dropSubtree(child.entryRef);
    }
  }

  void _scheduleWatchRefresh() {
    _watchDebounce?.cancel();
    _watchDebounce = Timer(const Duration(milliseconds: 250), () async {
      final visibleDirectories = <MuseWorkspaceEntry>[
        ...roots.values,
        for (final ref in expandedEntryRefs)
          if (entryByRef(ref) case final entry?) entry,
      ];
      for (final entry in visibleDirectories) {
        await _loadChildren(entry, generation: _generation, force: true);
      }
    });
  }

  Future<void> _persist() async {
    final workspace = accountSpaceRef;
    if (workspace == null) return;
    await persistence.save(
      MuseWorkspaceSnapshot(
        accountSpaceRef: workspace,
        mounts: List.unmodifiable(mounts),
        expandedEntryRefs: Set.unmodifiable(expandedEntryRefs),
        activeMountRef: activeMountRef,
      ),
    );
  }

  Future<void> _publishDshBinding() async {
    final workspace = accountSpaceRef;
    if (workspace == null) return;
    await _dshPublisher(
      workspace,
      accountSpaceTitle,
      List.unmodifiable(mounts),
      activeMountRef,
    );
  }

  Future<void> _cancelWatchers() async {
    _watchDebounce?.cancel();
    for (final subscription in _watchers.values) {
      await subscription.cancel();
    }
    _watchers.clear();
  }

  @override
  void dispose() {
    _generation++;
    _watchDebounce?.cancel();
    for (final subscription in _watchers.values) {
      unawaited(subscription.cancel());
    }
    _watchers.clear();
    super.dispose();
  }
}
