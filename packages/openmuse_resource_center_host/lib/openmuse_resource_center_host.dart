library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:file_picker/file_picker.dart';
import 'package:mime/mime.dart';
import 'package:path_provider/path_provider.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center/openmuse_resource_center.dart';

export 'package:muse_resource_center_io/muse_resource_center_io.dart';
export 'package:openmuse_resource_center/openmuse_resource_center.dart';

/// One instance per account and application process. A signed-out instance has
/// an isolated local scope; signing in never silently migrates its documents.
final class NativeResourceCenter {
  NativeResourceCenter._(
    this.root,
    this.hub,
    this.documents,
    this.snapshots,
    this.registry,
  );
  final Directory root;
  final ResourceCenterHub hub;
  final LocalResourceProvider documents, snapshots;
  final OpenMusePluginRegistry registry;
  late final ResourceCenterController controller;
  late final ResourceExecutionCoordinator execution;
  late final ResourceRpcService rpc;
  Timer? _janitor;
  late final ResourceConversationDraft draft;
  ResourceHttpTransport? _transport;
  ResourceRpcClient? _remote;
  ResourceOriginBridge? _bridge;
  final _remoteSources = <RemoteResourceProvider>[];
  final _sharesByPeer = <String, Map<String, ResourceRecord>>{};
  String? _peerRef;
  final _mounts = <String, FileTreeResourceProvider>{};
  String? _mountFingerprint;
  Future<void> _mountTail = Future.value();
  String? _connectionKey;
  bool _closed = false;
  Future<void> _connectionTail = Future.value();
  static Future<NativeResourceCenter> open({
    required String scope,
    required String deviceRef,
    required String deviceLabel,
    required OpenMusePluginRegistry registry,
    bool seedWelcomeResources = false,
    String workspace = 'personal',
    String workspaceLabel = 'Documents',
    String documentsLabel = 'Documents',
    Directory? storageRoot,
    ResourceStateStore? historyStore,
    Future<bool> Function(ResourceRecord)? canOpenLegacy,
    Future<bool> Function(ResourceRecord, List<int>)? openLegacy,
  }) async {
    final support =
        storageRoot ??
        Directory(
          '${(await getApplicationSupportDirectory()).path}/resource-center',
        );
    final root = Directory(
      '${support.path}/${resourceDigest(utf8.encode(scope))}',
    );
    await root.create(recursive: true);
    final hub = ResourceCenterHub(
      scope: scope,
      store:
          historyStore ??
          FileResourceStateStore(File('${root.path}/history.json')),
    );
    final documents = await LocalResourceProvider.open(
      Directory('${root.path}/documents'),
      scope: scope,
      label: documentsLabel,
      deviceLabel: deviceLabel,
      deviceRef: deviceRef,
      workspace: workspace,
      workspaceLabel: workspaceLabel,
      mountLabel: documentsLabel,
      generation: () => hub.generation,
    );
    final snapshots = await LocalResourceProvider.open(
      Directory('${root.path}/conversation-versions'),
      scope: scope,
      label: 'Conversation versions',
      deviceLabel: deviceLabel,
      deviceRef: deviceRef,
      workspace: workspace,
      workspaceLabel: workspaceLabel,
      mount: 'conversation-versions',
      mountLabel: 'Conversation versions',
      editable: false,
      uniqueNames: false,
      internal: true,
      generation: () => hub.generation,
    );
    hub.register(documents);
    hub.register(snapshots, visible: false);
    await hub.load();
    final value = NativeResourceCenter._(
      root,
      hub,
      documents,
      snapshots,
      registry,
    );
    value.controller = ResourceCenterController(
      hub: hub,
      registry: registry,
      importFile: value._import,
      refreshSources: value.refreshRemote,
      settleSourceTransition: () => value._connectionTail,
      canOpenLegacy: canOpenLegacy,
      openLegacy: openLegacy,
    );
    value.draft = ResourceConversationDraft(value.controller);
    value.controller.materializeLinkedContent = (parent, name, content) async {
      if (content.bytes.length > LocalResourceProvider.maxBytes)
        throw const ResourceFailure('TRANSFER_TOO_LARGE');
      // Linked documents are immutable snapshots with source provenance. They
      // never become an editable copy of the originating plugin's private file.
      final key = resourceDigest(
        utf8.encode(
          '${parent.identity.key}:${parent.version.digest}:$name:${resourceDigest(content.bytes)}',
        ),
      );
      return snapshots.create(
        parentRef: 'root',
        name: checkedName(name),
        mediaType: content.mediaType,
        bytes: content.bytes,
        idempotencyKey: 'linked-$key',
        provenance: parent.reference(pin: true),
        access: hub.access('resource-link'),
      );
    };
    value.execution = ResourceExecutionCoordinator(
      hub: hub,
      projections: ResourceProjectionManager(
        Directory('${root.path}/execution-projections'),
      ),
      extractText: value._extract,
    );
    value.rpc = ResourceRpcService(
      hub,
      snapshots: snapshots,
      peerStore: FileResourceStateStore(
        File('${root.path}/shared-origins.json'),
      ),
      prepareExecution: value.execution.prepare,
    );
    value._janitor = Timer.periodic(
      const Duration(minutes: 1),
      (_) => unawaited(
        value.execution.projections.sweep().catchError((Object _) {}),
      ),
    );
    await value.execution.projections.sweep();
    await value.rpc.restore();
    final shares = await FileResourceStateStore(
      File('${root.path}/source-shares.json'),
    ).read();
    if (shares['scope'] == scope && shares['peers'] is Map) {
      for (final e in (shares['peers'] as Map).entries) {
        try {
          final peer = checkedRef(e.key), records = <String, ResourceRecord>{};
          for (final entry in e.value as List) {
            final record = ResourceRecord.fromJson(entry);
            if (record.identity.scope == scope)
              records[record.identity.key] = record;
          }
          value._sharesByPeer[peer] = records;
        } on Object {
          /* Optional metadata never authorizes another account. */
        }
      }
    }
    if (seedWelcomeResources) await value.seedWelcome();
    return value;
  }

  /// One durable seed per local account scope. Later edits and deletions win.
  Future<void> seedWelcome() async {
    final marker = File('${root.path}/welcome-v1.json');
    if (await marker.exists()) {
      await _upgradeWelcomeGuide();
      return;
    }
    for (final sample in [
      (
        'OpenMuse Introduction.pptx',
        'OpenMuse-Introduction.pptx',
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      ),
      (
        'OpenMuse User Guide.sydoc',
        'OpenMuse-User-Guide.sydoc',
        'application/vnd.openmuse.siyuan.document+zip',
      ),
    ]) {
      // A crash may have created a sample before writing the scope marker.
      // Never replay a create with new bytes over an existing starter resource.
      if ((await _rootResources()).any((r) => r.name == sample.$1)) continue;
      final data = await rootBundle.load(
        'packages/openmuse_resource_center_host/assets/welcome/${sample.$2}',
      );
      final record = await documents.create(
        parentRef: 'root',
        name: sample.$1,
        mediaType: sample.$3,
        bytes: data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes),
        idempotencyKey: 'welcome-v1-${sample.$2}',
        access: hub.access('welcome-seed'),
      );
      // The product explicitly places starter resources in Home Recent.
      await hub.recordOpened(record, 'welcome');
    }
    await marker.writeAsString('{"version":1}', flush: true);
    await _upgradeWelcomeGuide();
  }

  Future<List<ResourceRecord>> _rootResources() async {
    final result = <ResourceRecord>[];
    String? cursor;
    do {
      final page = await documents.list(
        'root',
        hub.access('welcome-seed'),
        cursor: cursor,
        limit: 200,
      );
      result.addAll(
        page.entries.map((e) => e.resource).whereType<ResourceRecord>(),
      );
      cursor = page.nextCursor;
    } while (cursor != null);
    return result;
  }

  Future<void> _upgradeWelcomeGuide() async {
    const edition = '2026-11-03-illustrated';
    final marker = File('${root.path}/welcome-guide-$edition.json');
    if (await marker.exists()) return;
    const pristineDigests = {
      '2a3c34022420c3a2a626cd33bab4868d5280393c1fe5d54d59d231b60de67170',
      'c135995a41eb5c5141158bf120b59c7dcc8200266939845588cb4fda8f7f652d',
    };
    final guides = (await _rootResources())
        .where((r) => r.name == 'OpenMuse User Guide.sydoc')
        .toList();
    // Missing/renamed resources express user intent; do not resurrect them.
    if (guides.length == 1) {
      final guide = guides.single;
      final data = await rootBundle.load(
        'packages/openmuse_resource_center_host/assets/welcome/OpenMuse-User-Guide.sydoc',
      );
      final bytes = data.buffer.asUint8List(
        data.offsetInBytes,
        data.lengthInBytes,
      );
      final digest = resourceDigest(bytes);
      if (pristineDigests.contains(guide.version.digest)) {
        try {
          final updated = await documents.commit(
            guide.identity.resource,
            guide.version,
            bytes,
            'welcome-guide-$edition-${guide.identity.resource}',
            hub.access('welcome-seed'),
          );
          await hub.recordOpened(updated, 'welcome');
        } on ResourceFailure catch (error) {
          // A concurrent edit wins. Retry next launch against the latest head.
          if (error.code == 'REVISION_CONFLICT') return;
          rethrow;
        }
      } else if (guide.version.digest != digest) {
        // An edited guide is personal content. Offer the new edition separately.
        const name = 'OpenMuse User Guide — 2026-11.sydoc';
        if (!(await _rootResources()).any((r) => r.name == name)) {
          final updated = await documents.create(
            parentRef: 'root',
            name: name,
            mediaType: 'application/vnd.openmuse.siyuan.document+zip',
            bytes: bytes,
            idempotencyKey: 'welcome-guide-$edition',
            access: hub.access('welcome-seed'),
          );
          await hub.recordOpened(updated, 'welcome');
        }
      }
    }
    await marker.writeAsString('{"edition":"$edition"}', flush: true);
  }

  Future<String?> _extract(ResourceContent content) async {
    for (final d in registry.descriptors) {
      if (registry.stateOf(d.id) != OpenMusePluginState.active) continue;
      final p = registry.plugin(d.id);
      if (p is OpenMuseResourceContextContributor) {
        final text = await (p as OpenMuseResourceContextContributor)
            .resourceConversationText(content);
        if (text != null) return text;
      }
    }
    return null;
  }

  Future<ResourceImport?> _import() async {
    final result = await FilePicker.platform.pickFiles(
      withData: true,
      allowMultiple: false,
    );
    if (result == null) return null;
    final f = result.files.single;
    if (f.size > LocalResourceProvider.maxBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    final bytes =
        f.bytes ?? (f.path == null ? null : await File(f.path!).readAsBytes());
    if (bytes == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    return ResourceImport(
      name: f.name,
      bytes: bytes,
      mediaType:
          lookupMimeType(f.name, headerBytes: bytes.take(64).toList()) ??
          'application/octet-stream',
    );
  }

  /// Only a Host-authenticated pairing is passed here. No plugin controls the
  /// endpoint, cookie or permission to use the existing private-LAN pairing.
  Future<void> connect({
    required String key,
    required String peerRef,
    required Uri endpoint,
    required Map<String, String> Function() headers,
    bool allowPrivateNetwork = false,
  }) {
    final next = _connectionTail.catchError((Object _) {}).then((_) async {
      if (_closed) return;
      if (_connectionKey == key) return;
      await _disconnect();
      if (_closed) return;
      final t = ResourceHttpTransport(
        endpoint,
        headers: headers,
        allowInsecureTesting: allowPrivateNetwork,
      );
      _transport = t;
      _remote = ResourceRpcClient(t.call);
      _connectionKey = key;
      _peerRef = checkedRef(peerRef);
      _bridge = ResourceOriginBridge(client: _remote!, localHub: hub)..start();
      try {
        await refreshRemote();
        for (final r in (_sharesByPeer[_peerRef] ?? {}).values) {
          await _bridge!.share(
            r,
            retain: false,
            idempotencyKey: opaqueResourceId('reconnect'),
          );
        }
      } on Object {
        // Do not cache an unsuccessful connection as an established one.
        // The next lifecycle/pairing event must retry even with the same key.
        await _disconnect();
        rethrow;
      }
    });
    _connectionTail = next;
    return next;
  }

  Future<void> mountDirectories(
    Map<String, String> paths, {
    String workspace = 'openmuse.local.default',
    String workspaceLabel = 'Workspace',
  }) {
    final fingerprint = jsonEncode(paths);
    final next = _mountTail.catchError((Object _) {}).then((_) async {
      if (_closed || fingerprint == _mountFingerprint) return;
      for (final path
          in _mounts.keys.where((p) => !paths.containsKey(p)).toList()) {
        final p = _mounts.remove(path)!;
        p.available = false;
        hub.unregister(p.source.key);
      }
      var complete = true;
      for (final e in paths.entries) {
        if (_mounts.containsKey(e.key)) continue;
        final FileTreeResourceProvider provider;
        try {
          provider = await FileTreeResourceProvider.open(
            root: Directory(e.key),
            index: FileResourceStateStore(
              File(
                '${root.path}/mount-${resourceDigest(utf8.encode(e.key))}.json',
              ),
            ),
            scope: hub.scope,
            workspace: workspace,
            deviceRef: documents.source.deviceRef,
            deviceLabel: documents.source.deviceLabel,
            workspaceLabel: workspaceLabel,
            mountLabel: e.value,
            generation: () => hub.generation,
          );
        } on FileSystemException {
          // A saved mount may be disconnected, removed or inaccessible. It
          // must not prevent personal resources or other mounts from opening.
          // Keep this catalog retryable when the directory becomes available.
          complete = false;
          continue;
        }
        if (_closed) return;
        _mounts[e.key] = provider;
        hub.register(provider);
      }
      _mountFingerprint = complete ? fingerprint : null;
    });
    _mountTail = next;
    return next;
  }

  Future<void> refreshRemote() async {
    if (_remote == null) return;
    final providers = await _remote!.catalog(
      expectedScope: hub.scope,
      generation: () => hub.generation,
    );
    for (final p in _remoteSources) {
      hub.unregister(p.source.key);
      p.available = false;
    }
    _remoteSources
      ..clear()
      ..addAll(providers);
    for (final p in providers) {
      hub.register(p, visible: !p.source.internal);
    }
  }

  Future<String> preparePrompt(
    String text, {
    required String key,
    String? expectedPeer,
  }) async {
    await _connectionTail;
    if (_closed ||
        (expectedPeer != null && (_remote == null || _peerRef != expectedPeer)))
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    if (draft.isEmpty) return text;
    final refs = <ResourceReference>[];
    for (final item in draft.items) {
      var ref = item.reference;
      final p = hub.provider(ref.identity);
      if (_remote != null && p.source.local) {
        final bridge = _bridge;
        if (bridge == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
        final peer = _peerRef;
        if (peer == null) throw const ResourceFailure('DENIED');
        _sharesByPeer.putIfAbsent(peer, () => {})[item.record.identity.key] =
            item.record;
        await FileResourceStateStore(
          File('${root.path}/source-shares.json'),
        ).write({
          'scope': hub.scope,
          'peers': {
            for (final e in _sharesByPeer.entries)
              e.key: e.value.values.map((r) => r.toJson()).toList(),
          },
        });
        ref = await bridge.share(
          item.record,
          retain: draft.keepVersions,
          idempotencyKey: '$key-${refs.length}',
        );
        // A card may pin V1 while the source metadata already advertises V2.
        if (ref.version != item.reference.version)
          throw const ResourceFailure('VERSION_MISMATCH');
      } else if (draft.keepVersions && _remote == null) {
        ref = await hub.retain(
          ref,
          snapshots,
          hub.access('conversation-retain'),
          name: item.record.name,
          idempotencyKey: '$key-${refs.length}',
        );
      } else if (draft.keepVersions && _remote != null) {
        // Local PC resources are retained by the execution Host, never uploaded
        // to a phone and returned as a different identity.
        final response = await _remote!.call('retain', {
          'reference': ref.toJson(),
          'key': '$key-${refs.length}',
        });
        ref = ResourceReference.fromJson(response['reference']);
      }
      refs.add(ref);
    }
    final result = _remote == null
        ? await execution.prepare(refs, key)
        : await _remote!.call('prepare', {
            'references': refs.map((r) => r.toJson()).toList(),
            'key': key,
          });
    final prompt = [
      text,
      result['content'] as String,
    ].where((s) => s.isNotEmpty).join('\n\n');
    if (utf8.encode(jsonEncode({'text': prompt})).length > 56 * 1024)
      throw const ResourceFailure('CONTEXT_TOO_LARGE');
    return prompt;
  }

  Future<void> disconnect() {
    final next = _connectionTail
        .catchError((Object _) {})
        .then((_) => _disconnect());
    _connectionTail = next;
    return next;
  }

  Future<void> _disconnect() async {
    _connectionKey = null;
    _peerRef = null;
    final bridge = _bridge;
    _bridge = null;
    _remote = null;
    final transport = _transport;
    _transport = null;
    // Closing the HTTP client also interrupts any in-flight foreground poll.
    transport?.close();
    await bridge?.stop();
    for (final p in _remoteSources) {
      p.available = false;
    }
    hub.changed();
  }

  Future<void>? _closing;
  Future<void> close() => _closing ??= _close();
  Future<void> _close() async {
    _closed = true;
    _janitor?.cancel();
    await _mountTail.catchError((Object _) {});
    await _connectionTail.catchError((Object _) {});
    await _disconnect();
    draft.dispose();
    controller.dispose();
    await rpc.close();
    await execution.close();
    await documents.close();
    await snapshots.close();
    await hub.dispose();
  }
}
