import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:muse_resource_center/muse_resource_center.dart';

String opaqueResourceId([String prefix = 'r']) =>
    '$prefix-${base64Url.encode(List.generate(18, (_) => Random.secure().nextInt(256))).replaceAll('=', '')}';

/// App-owned virtual filesystem: names and hierarchy live in a private ledger,
/// immutable bytes in digest blobs. Display names never become physical paths.
final class LocalResourceProvider implements ResourceProvider {
  LocalResourceProvider._(
    this._root,
    this._ledger,
    this.source,
    this.generation,
    this.uniqueNames,
  );
  final Directory _root;
  Map<String, Object?> _ledger;
  @override
  final ResourceSource source;
  final int Function() generation;
  final bool uniqueNames;
  Future<void> _tail = Future.value();
  @override
  bool available = true;
  bool _closed = false;
  static const maxBytes = 64 * 1024 * 1024;

  static Future<LocalResourceProvider> open(
    Directory root, {
    required String scope,
    required String label,
    required String deviceLabel,
    String? deviceRef,
    String workspace = 'personal',
    String workspaceLabel = 'Documents',
    String mount = 'documents',
    String mountLabel = 'Documents',
    bool editable = true,
    bool uniqueNames = true,
    bool internal = false,
    int Function()? generation,
  }) async {
    await root.create(recursive: true);
    final canonical = Directory(await root.resolveSymbolicLinks());
    final ledgerFile = File('${canonical.path}/ledger.json');
    if (await FileSystemEntity.isLink(ledgerFile.path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    Map<String, Object?> ledger;
    if (await ledgerFile.exists()) {
      if (await ledgerFile.length() > 16 * 1024 * 1024)
        throw const ResourceFailure('INVALID_STORAGE');
      ledger = Map<String, Object?>.from(
        jsonDecode(await ledgerFile.readAsString()) as Map,
      );
      if (ledger['schema'] != 1 ||
          ledger['scope'] != scope ||
          ledger['workspace'] != workspace ||
          ledger['mount'] != mount) {
        throw const ResourceFailure('STORAGE_SCOPE_MISMATCH');
      }
    } else {
      ledger = {
        'schema': 1,
        'scope': scope,
        'source': opaqueResourceId('source'),
        'workspace': workspace,
        'mount': mount,
        'folders': {
          'root': {'name': mountLabel, 'parent': null},
        },
        'items': <String, Object?>{},
        'operations': <String, Object?>{},
      };
    }
    final blobs = Directory('${canonical.path}/blobs');
    if (await FileSystemEntity.isLink(blobs.path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    await blobs.create();
    final provider = LocalResourceProvider._(
      canonical,
      ledger,
      ResourceSource(
        scope: scope,
        id: checkedRef(ledger['source']),
        workspace: workspace,
        mount: mount,
        label: label,
        workspaceLabel: workspaceLabel,
        mountLabel: mountLabel,
        deviceLabel: deviceLabel,
        deviceRef: deviceRef,
        local: true,
        canCreate: true,
        canRename: editable,
        canEdit: editable,
        internal: internal,
      ),
      generation ?? (() => 1),
      uniqueNames,
    );
    await provider._save();
    return provider;
  }

  Map<String, Object?> get _items =>
      Map<String, Object?>.from(_ledger['items'] as Map);
  Map<String, Object?> get _folders =>
      Map<String, Object?>.from(_ledger['folders'] as Map);
  Map<String, Object?> get _operations =>
      Map<String, Object?>.from(_ledger['operations'] as Map);
  void _check(ResourceAccess access, ResourceOperation operation, String ref) {
    if (_closed || !available)
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    access.check(
      source.identity(checkedRef(ref)),
      operation,
      currentGeneration: generation(),
    );
  }

  Map<String, Object?> _item(String ref) {
    final value = _items[checkedRef(ref)];
    if (value is! Map) throw const ResourceFailure('NOT_FOUND');
    return Map<String, Object?>.from(value);
  }

  void _parent(String ref) {
    if (!_folders.containsKey(checkedRef(ref)))
      throw const ResourceFailure('NOT_FOUND');
  }

  List<String> _location(String parent) {
    final list = <String>[];
    final seen = <String>{};
    var current = parent;
    while (true) {
      if (!seen.add(current) || seen.length > 32)
        throw const ResourceFailure('INVALID_STORAGE');
      final f = _folders[current];
      if (f is! Map) throw const ResourceFailure('NOT_FOUND');
      list.insert(0, checkedName(f['name']));
      if (current == 'root') break;
      current = checkedRef(f['parent']);
    }
    return list;
  }

  ResourceRecord _record(
    String ref,
    Map<String, Object?> item, [
    String? revision,
  ]) {
    final rev = revision ?? checkedRef(item['head']);
    final v = (item['versions'] as Map)[rev];
    if (v is! Map) throw const ResourceFailure('VERSION_UNAVAILABLE');
    final version = ResourceVersion(
      revision: rev,
      digest: v['digest'] as String,
    );
    return ResourceRecord(
      identity: source.identity(ref),
      descriptor: MuseResourceDescriptorV1(
        resourceRef: ref,
        revision: rev,
        displayName: checkedName(item['name']),
        mediaType: item['mediaType'] as String,
        format: MuseResourceFormat(
          formatId: item['mediaType'] as String,
          confidence: MuseFormatConfidence.claimed,
        ),
        capabilities: [
          MuseResourceCapability.describe,
          MuseResourceCapability.materialize,
          if (source.canEdit) MuseResourceCapability.commit,
        ],
        security: const MuseResourceSecurity(
          classification: MuseSecurityClassification.internal,
          activeContent: MuseActiveContent.unknown,
        ),
        sizeBytes: v['size'] as int,
      ),
      version: version,
      sourceLabel: source.label,
      location: _location(checkedRef(item['parent'])),
      parentRef: checkedRef(item['parent']),
      metadataRevision: item['metadataRevision'] as int,
      readOnly: !source.canEdit,
      provenance: item['provenance'] == null
          ? null
          : ResourceReference.fromJson(item['provenance']),
    );
  }

  Future<T> _serial<T>(Future<T> Function() action) {
    final next = _tail.catchError((Object _) {}).then((_) => action());
    _tail = next.then<void>((_) {}, onError: (Object _, StackTrace stack) {});
    return next;
  }

  Future<void> _save() async {
    final temp = File('${_root.path}/${opaqueResourceId('ledger')}.tmp');
    await temp.writeAsString(jsonEncode(_ledger), flush: true);
    await temp.rename('${_root.path}/ledger.json');
  }

  Future<void> _blob(String digest, List<int> bytes) async {
    if (bytes.length > maxBytes || bytes.any((v) => v < 0 || v > 255))
      throw const ResourceFailure('RESOURCE_TOO_LARGE');
    final path = '${_root.path}/blobs/$digest';
    if (await FileSystemEntity.isLink(path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    final f = File(path);
    if (await f.exists()) {
      if (resourceDigest(await f.readAsBytes()) != digest)
        throw const ResourceFailure('DIGEST_MISMATCH');
      return;
    }
    final temp = File('${_root.path}/blobs/${opaqueResourceId('write')}.tmp');
    await temp.writeAsBytes(bytes, flush: true);
    await temp.rename(path);
  }

  void _unique(String parent, String name, {String? excluding}) {
    if (!uniqueNames) return;
    for (final e in _items.entries) {
      final item = e.value as Map;
      if (e.key != excluding &&
          item['parent'] == parent &&
          (item['name'] as String).toLowerCase() == name.toLowerCase())
        throw const ResourceFailure('NAME_CONFLICT');
    }
    for (final e in _folders.entries) {
      final folder = e.value as Map;
      if (folder['parent'] == parent &&
          (folder['name'] as String).toLowerCase() == name.toLowerCase()) {
        throw const ResourceFailure('NAME_CONFLICT');
      }
    }
  }

  String _fingerprint(List<Object?> value) =>
      resourceDigest(utf8.encode(jsonEncode(value)));
  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    _check(access, ResourceOperation.describe, parentRef);
    _parent(parentRef);
    if (limit < 1 || limit > 200) throw const ResourceFailure('INVALID_LIMIT');
    final offset = cursor == null ? 0 : int.tryParse(cursor);
    if (offset == null || offset < 0)
      throw const ResourceFailure('INVALID_CURSOR');
    final all = <ResourceDirectoryEntry>[
      for (final e in _folders.entries)
        if ((e.value as Map)['parent'] == parentRef)
          ResourceDirectoryEntry(
            nodeRef: e.key,
            name: (e.value as Map)['name'] as String,
          ),
      for (final e in _items.entries)
        if ((e.value as Map)['parent'] == parentRef)
          ResourceDirectoryEntry(
            nodeRef: e.key,
            name: (e.value as Map)['name'] as String,
            resource: _record(e.key, Map<String, Object?>.from(e.value as Map)),
          ),
    ];
    all.sort((a, b) {
      if (a.isDirectory != b.isDirectory) return a.isDirectory ? -1 : 1;
      final n = a.name.toLowerCase().compareTo(b.name.toLowerCase());
      return n != 0 ? n : a.nodeRef.compareTo(b.nodeRef);
    });
    final end = min(offset + limit, all.length);
    return ResourcePage(
      all.sublist(min(offset, all.length), end),
      nextCursor: end < all.length ? '$end' : null,
    );
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async {
    _check(access, ResourceOperation.describe, resourceRef);
    return _record(resourceRef, _item(resourceRef));
  }

  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    _check(access, ResourceOperation.read, resourceRef);
    final item = _item(resourceRef);
    final record = _record(resourceRef, item, version?.revision);
    if (version != null && record.version != version)
      throw const ResourceFailure('VERSION_MISMATCH');
    final path = '${_root.path}/blobs/${record.version.digest}';
    if (await FileSystemEntity.isLink(path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    final file = File(path);
    if (!await file.exists())
      throw const ResourceFailure('VERSION_UNAVAILABLE');
    if (await file.length() > maxBytes)
      throw const ResourceFailure('RESOURCE_TOO_LARGE');
    return ResourceContent(record, await file.readAsBytes());
  }

  @override
  Future<ResourceRecord> create({
    required String parentRef,
    required String name,
    required String mediaType,
    required List<int> bytes,
    required String idempotencyKey,
    required ResourceAccess access,
    ResourceReference? provenance,
  }) => _serial(() async {
    _check(access, ResourceOperation.create, parentRef);
    _parent(parentRef);
    name = checkedName(name);
    checkedRef(idempotencyKey);
    if (mediaType.isEmpty || mediaType.length > 240 || !mediaType.contains('/'))
      throw const ResourceFailure('INVALID_MEDIA_TYPE');
    if (provenance != null) {
      access.check(
        provenance.identity,
        ResourceOperation.retain,
        currentGeneration: generation(),
      );
    }
    final digest = resourceDigest(bytes),
        fingerprint = _fingerprint([
          'create',
          parentRef,
          name,
          mediaType,
          digest,
          provenance?.toJson(),
        ]);
    final prior = _operations[idempotencyKey];
    if (prior is Map) {
      if (prior['fingerprint'] != fingerprint)
        throw const ResourceFailure('IDEMPOTENCY_CONFLICT');
      return _record(
        prior['resource'] as String,
        _item(prior['resource'] as String),
        prior['revision'] as String,
      );
    }
    _unique(parentRef, name);
    await _blob(digest, bytes);
    final ref = opaqueResourceId(), revision = opaqueResourceId('v');
    final item = <String, Object?>{
      'name': name,
      'parent': parentRef,
      'mediaType': mediaType,
      'metadataRevision': 1,
      'head': revision,
      'versions': {
        revision: {'digest': digest, 'size': bytes.length},
      },
      if (provenance != null) 'provenance': provenance.toJson(),
    };
    final before = _ledger;
    _ledger = {
      ..._ledger,
      'items': {..._items, ref: item},
      'operations': {
        ..._operations,
        idempotencyKey: {
          'fingerprint': fingerprint,
          'resource': ref,
          'revision': revision,
        },
      },
    };
    try {
      await _save();
    } on Object {
      _ledger = before;
      rethrow;
    }
    return _record(ref, item);
  });
  @override
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  ) => _serial(() async {
    _check(access, ResourceOperation.create, parentRef);
    _parent(parentRef);
    name = checkedName(name);
    _unique(parentRef, name);
    final ref = opaqueResourceId('directory'), before = _ledger;
    _ledger = {
      ..._ledger,
      'folders': {
        ..._folders,
        ref: {'name': name, 'parent': parentRef},
      },
    };
    try {
      await _save();
    } on Object {
      _ledger = before;
      rethrow;
    }
    return ref;
  });
  @override
  Future<ResourceRecord> rename(
    String resourceRef,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  ) => _serial(() async {
    _check(access, ResourceOperation.rename, resourceRef);
    if (!source.canRename) throw const ResourceFailure('READ_ONLY');
    name = checkedName(name);
    final item = _item(resourceRef);
    if (item['name'] == name) return _record(resourceRef, item);
    if (item['metadataRevision'] != expectedMetadataRevision)
      throw const ResourceFailure('METADATA_CONFLICT');
    _unique(item['parent'] as String, name, excluding: resourceRef);
    final updated = {
          ...item,
          'name': name,
          'metadataRevision': expectedMetadataRevision + 1,
        },
        before = _ledger;
    _ledger = {
      ..._ledger,
      'items': {..._items, resourceRef: updated},
    };
    try {
      await _save();
    } on Object {
      _ledger = before;
      rethrow;
    }
    return _record(resourceRef, updated);
  });
  @override
  Future<ResourceRecord> commit(
    String resourceRef,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) => _serial(() async {
    _check(access, ResourceOperation.edit, resourceRef);
    if (!source.canEdit) throw const ResourceFailure('READ_ONLY');
    checkedRef(idempotencyKey);
    final item = _item(resourceRef);
    final digest = resourceDigest(bytes),
        fingerprint = _fingerprint([
          'edit',
          resourceRef,
          base.toJson(),
          digest,
        ]);
    final prior = _operations[idempotencyKey];
    if (prior is Map) {
      if (prior['fingerprint'] != fingerprint)
        throw const ResourceFailure('IDEMPOTENCY_CONFLICT');
      return _record(resourceRef, item, prior['revision'] as String);
    }
    if (_record(resourceRef, item).version != base)
      throw const ResourceFailure('REVISION_CONFLICT');
    await _blob(digest, bytes);
    final revision = base.digest == digest
        ? base.revision
        : opaqueResourceId('v');
    final updated = {
          ...item,
          'head': revision,
          'versions': {
            ...Map<String, Object?>.from(item['versions'] as Map),
            revision: {'digest': digest, 'size': bytes.length},
          },
        },
        before = _ledger;
    _ledger = {
      ..._ledger,
      'items': {..._items, resourceRef: updated},
      'operations': {
        ..._operations,
        idempotencyKey: {
          'fingerprint': fingerprint,
          'resource': resourceRef,
          'revision': revision,
        },
      },
    };
    try {
      await _save();
    } on Object {
      _ledger = before;
      rethrow;
    }
    return _record(resourceRef, updated);
  });
  Future<void> close() async {
    await _tail.catchError((Object _) {});
    _closed = true;
    available = false;
  }
}

final class FileResourceStateStore implements ResourceStateStore {
  const FileResourceStateStore(this.file);
  final File file;
  @override
  Future<Map<String, Object?>> read() async {
    if (!await file.exists()) return {};
    if (await FileSystemEntity.isLink(file.path) ||
        await file.length() > 4 * 1024 * 1024)
      throw const ResourceFailure('UNSAFE_STORAGE');
    return Map<String, Object?>.from(
      jsonDecode(await file.readAsString()) as Map,
    );
  }

  @override
  Future<void> write(Map<String, Object?> state) async {
    await file.parent.create(recursive: true);
    if (await FileSystemEntity.isLink(file.path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    final temp = File('${file.parent.path}/${opaqueResourceId('state')}.tmp');
    await temp.writeAsString(jsonEncode(state), flush: true);
    await temp.rename(file.path);
  }
}
