import 'dart:async';
import 'dart:math';
import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

final class ResourceEditorPresentation {
  const ResourceEditorPresentation(this.choice, this.session);
  final ResourceHandlerChoice choice;
  final ResourceEditorSession session;
}

enum ResourceCenterSection { recent, local, remote }

final class ResourceImport {
  const ResourceImport({
    required this.name,
    required this.bytes,
    this.mediaType = 'application/octet-stream',
  });
  final String name, mediaType;
  final List<int> bytes;
}

final class ResourceHandlerChoice {
  const ResourceHandlerChoice(this.plugin, this.contributor, this.spec);
  final OpenMusePlugin plugin;
  final OpenMuseResourceHandlerContributor contributor;
  final ResourceHandlerSpec spec;
  String get key => '${plugin.descriptor.id}/${spec.id}';
}

final class ResourceCreatorChoice {
  const ResourceCreatorChoice(this.plugin, this.contributor, this.spec);
  final OpenMusePlugin plugin;
  final OpenMuseResourceCreatorContributor contributor;
  final ResourceCreatorSpec spec;
  String get key => '${plugin.descriptor.id}/${spec.id}';
}

final class ResourceBreadcrumb {
  const ResourceBreadcrumb(this.ref, this.name);
  final String ref, name;
}

String resourceOperationKey(String prefix) =>
    '$prefix-${DateTime.now().microsecondsSinceEpoch}-${Random.secure().nextInt(1 << 32)}';

/// Shared state for Home and Desktop. Platform import and compatibility routes
/// are supplied by the Host; every operation is backed by the same core Hub.
final class ResourceCenterController extends ChangeNotifier {
  ResourceCenterController({
    required this.hub,
    required this.registry,
    this.importFile,
    this.refreshSources,
    this.settleSourceTransition,
    this.canOpenLegacy,
    this.openLegacy,
  }) {
    _subscription = hub.changes.listen((_) => _changed());
    registry.addListener(_changed);
  }

  /// Desktop may retain the shared editor session in its workspace tabs.
  /// Mobile leaves this unset and uses the shared resource page.
  Future<void> Function(ResourceEditorPresentation)? presentEditor;
  Future<ResourceRecord> Function(
    ResourceRecord parent,
    String name,
    ResourceSeed content,
  )?
  materializeLinkedContent;
  final ResourceCenterHub hub;
  final OpenMusePluginRegistry registry;
  final Future<ResourceImport?> Function()? importFile;
  final Future<void> Function()? refreshSources;

  /// Waits for a Host's already-started connection transition, if any.
  /// It must not initiate login, broaden a grant, or reconnect an offline peer.
  final Future<void> Function()? settleSourceTransition;
  final Future<bool> Function(ResourceRecord record)? canOpenLegacy;
  final Future<bool> Function(ResourceRecord record, List<int> bytes)?
  openLegacy;
  late final StreamSubscription<void> _subscription;
  bool _disposed = false, loading = false;
  int _epoch = 0;
  Object? failure;
  ResourceCenterSection section = ResourceCenterSection.recent;
  String? selectedSourceKey;
  String query = '';
  final _breadcrumbs = <ResourceBreadcrumb>[];
  final _entries = <ResourceDirectoryEntry>[];
  String? cursor;
  List<ResourceBreadcrumb> get breadcrumbs => List.unmodifiable(_breadcrumbs);
  List<ResourceDirectoryEntry> get entries => List.unmodifiable(_entries);
  ResourceProvider? get selectedSource => hub.visibleProviders
      .where((p) => p.source.key == selectedSourceKey)
      .firstOrNull;
  String get parentRef => _breadcrumbs.lastOrNull?.ref ?? 'root';
  List<ResourceProvider> get sources => hub.visibleProviders
      .where(
        (p) => section == ResourceCenterSection.local
            ? p.source.local
            : !p.source.local,
      )
      .toList();
  List<RecentResource> get recent => hub.recent
      .where((r) => _matches(r.record.name, r.record.location.join(' ')))
      .toList();
  List<ResourceDirectoryEntry> get filteredEntries => _entries
      .where((e) => _matches(e.name, e.resource?.location.join(' ') ?? ''))
      .toList();
  bool _matches(String name, String location) =>
      query.trim().isEmpty ||
      '$name $location'.toLowerCase().contains(query.trim().toLowerCase());
  void _changed() {
    if (!_disposed) notifyListeners();
  }

  void setQuery(String value) {
    query = value;
    _changed();
  }

  void selectSection(ResourceCenterSection value) {
    section = value;
    selectedSourceKey = null;
    _breadcrumbs.clear();
    _entries.clear();
    cursor = null;
    failure = null;
    query = '';
    loading = false;
    _epoch++;
    _changed();
  }

  Future<void> selectSource(ResourceProvider provider) async {
    selectedSourceKey = provider.source.key;
    query = '';
    _breadcrumbs
      ..clear()
      ..add(ResourceBreadcrumb('root', provider.source.mountLabel));
    await refresh();
  }

  Future<void> enter(ResourceDirectoryEntry entry) async {
    if (!entry.isDirectory) return;
    query = '';
    _breadcrumbs.add(ResourceBreadcrumb(entry.nodeRef, entry.name));
    await refresh();
  }

  Future<void> breadcrumb(int index) async {
    if (index < 0) {
      selectedSourceKey = null;
      _breadcrumbs.clear();
      _entries.clear();
      _epoch++;
      _changed();
      return;
    }
    if (index >= _breadcrumbs.length) return;
    _breadcrumbs.removeRange(index + 1, _breadcrumbs.length);
    query = '';
    await refresh();
  }

  Future<void> refresh({bool more = false}) async {
    final epoch = ++_epoch;
    failure = null;
    final source = selectedSource;
    if (source == null) {
      loading = true;
      _changed();
      try {
        // Local documents and recent history remain usable when a peer is
        // offline or its pairing session has expired.
        if (section == ResourceCenterSection.remote) {
          await refreshSources?.call();
        }
      } on Object catch (e) {
        if (epoch == _epoch) failure = e;
      }
      if (epoch == _epoch) {
        loading = false;
        _changed();
      }
      return;
    }
    if (more && cursor == null) return;
    loading = true;
    _changed();
    try {
      final parent = parentRef, next = more ? cursor : null;
      final page = await _readCurrentSource(
        source.source.identity('root'),
        () => hub
            .provider(source.source.identity('root'))
            .list(parent, hub.access('resource-center'), cursor: next),
        stillCurrent: () => epoch == _epoch && !_disposed,
      );
      if (epoch != _epoch || _disposed) return;
      if (!more) _entries.clear();
      final known = _entries.map((e) => e.nodeRef).toSet();
      _entries.addAll(page.entries.where((e) => known.add(e.nodeRef)));
      cursor = page.nextCursor;
    } on Object catch (e) {
      if (epoch == _epoch) failure = e;
    } finally {
      if (epoch == _epoch) {
        loading = false;
        _changed();
      }
    }
  }

  List<ResourceCreatorChoice> get creators {
    final result = <ResourceCreatorChoice>[];
    for (final descriptor in registry.descriptors) {
      if (registry.stateOf(descriptor.id) != OpenMusePluginState.active)
        continue;
      final plugin = registry.plugin(descriptor.id);
      if (plugin is OpenMuseResourceCreatorContributor) {
        for (final spec
            in (plugin as OpenMuseResourceCreatorContributor)
                .resourceCreators) {
          result.add(
            ResourceCreatorChoice(
              plugin!,
              plugin as OpenMuseResourceCreatorContributor,
              spec,
            ),
          );
        }
      }
    }
    result.sort((a, b) {
      final n = a.spec.title.compareTo(b.spec.title);
      return n != 0 ? n : a.key.compareTo(b.key);
    });
    return result;
  }

  bool creatorAvailable(ResourceCreatorChoice choice) =>
      registry.stateOf(choice.plugin.descriptor.id) ==
          OpenMusePluginState.active &&
      identical(registry.plugin(choice.plugin.descriptor.id), choice.plugin) &&
      creators.any((c) => c.key == choice.key);
  Future<List<ResourceHandlerChoice>> handlers(
    ResourceRecord record, {
    ResourceOpenIntent intent = ResourceOpenIntent.view,
  }) async {
    final result = <ResourceHandlerChoice>[];
    for (final descriptor in registry.descriptors) {
      if (registry.stateOf(descriptor.id) != OpenMusePluginState.active)
        continue;
      final plugin = registry.plugin(descriptor.id);
      if (plugin is! OpenMuseResourceHandlerContributor) continue;
      final contributor = plugin as OpenMuseResourceHandlerContributor;
      for (final spec in contributor.resourceHandlers) {
        if (!spec.accepts(record) ||
            (intent == ResourceOpenIntent.edit && !spec.editable))
          continue;
        if (await contributor.probeResource(record, intent) &&
            registry.stateOf(descriptor.id) == OpenMusePluginState.active) {
          result.add(ResourceHandlerChoice(plugin!, contributor, spec));
        }
      }
    }
    result.sort((a, b) {
      final p = b.spec.priority.compareTo(a.spec.priority);
      return p != 0 ? p : a.key.compareTo(b.key);
    });
    return result;
  }

  String preferenceKey(ResourceRecord record) =>
      '${record.mediaType}:${record.descriptor.format.formatId}:${record.mediaType == 'application/octet-stream' ? record.name.split('.').last.toLowerCase() : ''}:view';
  // A grant/catalog refresh replaces remote providers and closes their HTTP
  // clients. A read already in flight may fail on that obsolete client. Retry
  // once only when the same source identity now has a different live provider;
  // the new attempt performs all authorization/version checks again.
  Future<T> _readCurrentSource<T>(
    ResourceIdentity identity,
    Future<T> Function() read, {
    bool Function()? stillCurrent,
  }) async {
    ResourceProvider? provider() {
      try {
        return hub.provider(identity);
      } on ResourceFailure {
        return null;
      }
    }

    final original = provider();
    try {
      return await read();
    } on Object {
      if (!_disposed &&
          original != null &&
          !original.source.local &&
          stillCurrent?.call() != false) {
        try {
          await settleSourceTransition?.call();
        } on Object {
          // Preserve the original read failure if the new connection failed.
        }
      }
      final replacement = provider();
      if (_disposed ||
          stillCurrent?.call() == false ||
          original == null ||
          replacement == null ||
          identical(original, replacement) ||
          !replacement.available) {
        rethrow;
      }
      return read();
    }
  }

  Future<ResourceRecord> describe(ResourceIdentity identity) =>
      _readCurrentSource(
        identity,
        () => hub
            .provider(identity)
            .describe(identity.resource, hub.access('resource-row')),
      );

  Future<OpenedResource> resolve(ResourceReference reference) =>
      _readCurrentSource(
        reference.identity,
        () => hub.resolve(
          reference,
          hub.access('resource-center', allowOfflineReplica: true),
        ),
      );
  ResourceProvider? get creationTarget {
    final current = selectedSource;
    if (current != null && current.available && current.source.canCreate)
      return current;
    return hub.visibleProviders
        .where((p) => p.source.local && p.available && p.source.canCreate)
        .firstOrNull;
  }

  String targetParent(ResourceProvider target) =>
      identical(target, selectedSource) ? parentRef : 'root';
  Future<ResourceRecord> create(
    ResourceCreatorChoice creator,
    ResourceProvider target,
    String parent,
    String name,
    Map<String, Object?> values,
    String key,
  ) async {
    if (!creatorAvailable(creator))
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    if (!target.available ||
        !target.source.canCreate ||
        (!target.source.local && creator.spec.localOnly))
      throw const ResourceFailure('READ_ONLY');
    final checked = checkedName(name);
    if (creator.spec.extension.isNotEmpty &&
        !checked.toLowerCase().endsWith(
          '.${creator.spec.extension.toLowerCase()}',
        ))
      throw const ResourceFailure('EXTENSION_REQUIRED');
    final seed = await creator.contributor.createResource(creator.spec.id, {
      'title': checked,
      ...values,
    });
    if (!creatorAvailable(creator))
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    if (seed.mediaType != creator.spec.mediaType)
      throw const ResourceFailure('MEDIA_TYPE_MISMATCH');
    final record = await target.create(
      parentRef: parent,
      name: checked,
      mediaType: seed.mediaType,
      bytes: seed.bytes,
      idempotencyKey: key,
      access: hub.access('resource-create'),
    );
    await refresh();
    return record;
  }

  Future<ResourceRecord?> importResource({
    ResourceProvider? destination,
  }) async {
    final target = destination ?? creationTarget;
    if (target == null || importFile == null)
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    final imported = await importFile!();
    if (imported == null) return null;
    final record = await target.create(
      parentRef: targetParent(target),
      name: checkedName(imported.name),
      mediaType: imported.mediaType,
      bytes: imported.bytes,
      idempotencyKey: resourceOperationKey('import'),
      access: hub.access('resource-import'),
    );
    await refresh();
    return record;
  }

  Future<ResourceRecord> rename(ResourceRecord record, String name) async {
    final next = checkedName(name);
    final oldExtension = RegExp(
      r'\.([a-zA-Z0-9]{1,10})$',
    ).firstMatch(record.name)?.group(1);
    if (oldExtension != null &&
        !next.toLowerCase().endsWith('.${oldExtension.toLowerCase()}'))
      throw const ResourceFailure('EXTENSION_REQUIRED');
    final result = await hub
        .provider(record.identity)
        .rename(
          record.identity.resource,
          next,
          record.metadataRevision,
          hub.access('resource-rename'),
        );
    await hub.refreshRecord(result);
    await refresh();
    return result;
  }

  Future<void> directory(String name, {ResourceProvider? destination}) async {
    final target = destination ?? creationTarget;
    if (target == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    await target.createDirectory(
      targetParent(target),
      checkedName(name),
      hub.access('resource-create'),
    );
    if (selectedSourceKey == target.source.key) {
      await refresh();
    } else {
      await selectSource(target);
    }
  }

  Future<void> saveCopy(ResourceReference reference) async {
    final source = hub.visibleProviders
        .where((p) => p.source.local && p.source.canCreate)
        .firstOrNull;
    if (source == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    final opened = await resolve(reference);
    await source.create(
      parentRef: 'root',
      name: opened.content.record.name,
      mediaType: opened.content.record.mediaType,
      bytes: opened.content.bytes,
      idempotencyKey: resourceOperationKey('copy'),
      access: hub.access('resource-copy'),
    );
    await refresh();
  }

  Future<void> opened(ResourceRecord record, String handler) async {
    try {
      await hub.recordOpened(record, handler);
    } on Object {
      /* History failure does not revoke an already-opened session. */
    }
  }

  @override
  void dispose() {
    _disposed = true;
    _epoch++;
    _subscription.cancel();
    registry.removeListener(_changed);
    super.dispose();
  }
}
