import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'local_provider.dart';

/// Produced by the authenticated transport (e.g. PairedDesktopGateway), never
/// decoded from an RPC body. Its workspace is an authorization boundary.
final class ResourcePeer {
  const ResourcePeer({
    required this.scope,
    required this.deviceRef,
    required this.workspaceRef,
    required this.expiresAtMs,
  });
  final String scope, deviceRef, workspaceRef;
  final int expiresAtMs;
}

typedef ResourceRpcTransport =
    Future<Map<String, Object?>> Function(Map<String, Object?> request);

/// Versioned, transport-neutral dispatch behind the existing paired grant.
/// Public operations never return a storage path or an execution projection.
final class ResourceRpcService {
  ResourceRpcService(
    this.hub, {
    required this.snapshots,
    this.peerStore,
    this.readTimeout = const Duration(seconds: 12),
    this.heartbeatTtl = const Duration(seconds: 10),
    this.prepareExecution,
  });
  static const protocol = 'openmuse.resource-center/rpc/v1';
  static const maxTransferBytes = 16 * 1024 * 1024;
  final ResourceCenterHub hub;
  final ResourceProvider snapshots;
  final ResourceStateStore? peerStore;
  final Duration readTimeout, heartbeatTtl;

  /// Trusted Host adapter. Only references authorized below reach it.
  final Future<Map<String, Object?>> Function(
    List<ResourceReference> refs,
    String key,
  )?
  prepareExecution;
  final _peerSources = <String, _PeerSource>{};
  final _pending = <String, _SourceRead>{};
  final _peers = <String, ResourcePeer>{};
  final _lastSeen = <String, int>{};
  Future<void> _persistTail = Future.value();
  bool _closed = false;

  Future<void> restore() async {
    final data = await peerStore?.read();
    if (data == null || data['scope'] != hub.scope) return;
    for (final value in (data['sources'] as List? ?? const [])) {
      try {
        final m = Map<String, Object?>.from(value as Map),
            source = ResourceSource.fromJson(
              Map<String, Object?>.from(m['source'] as Map),
            );
        if (source.scope != hub.scope) continue;
        final p = _PeerSource(this, source, checkedRef(m['peer']));
        for (final item in m['records'] as List) {
          final r = ResourceRecord.fromJson(item);
          if (r.identity.sourceKey == source.key)
            p.records[r.identity.resource] = r;
        }
        _peerSources[source.key] = p;
        hub.register(p);
      } on Object {
        /* Ignore a corrupt optional origin catalog, never trust it for bytes. */
      }
    }
  }

  ResourceAccess _access(ResourcePeer peer) =>
      hub.access('paired:${peer.deviceRef}');
  void _authorize(ResourcePeer peer) {
    if (_closed ||
        peer.scope != hub.scope ||
        peer.deviceRef.isEmpty ||
        peer.expiresAtMs <= DateTime.now().millisecondsSinceEpoch)
      throw const ResourceFailure('DENIED');
  }

  ResourceProvider _local(
    ResourcePeer peer,
    Object? value, {
    bool allowSnapshotRead = false,
  }) {
    final identity = ResourceIdentity.fromJson(value);
    if (identity.scope != peer.scope || identity.workspace != peer.workspaceRef)
      throw const ResourceFailure('DENIED');
    final p = hub.provider(identity);
    if (!p.source.local || (identical(p, snapshots) && !allowSnapshotRead))
      throw const ResourceFailure('DENIED');
    return p;
  }

  Future<Map<String, Object?>> dispatch(
    Map<String, Object?> request,
    ResourcePeer peer,
  ) async {
    _authorize(peer);
    final envelope = resourceObject(request, {
      'protocol',
      'operation',
      'arguments',
    });
    if (envelope['protocol'] != protocol)
      throw const ResourceFailure('UNSUPPORTED_PROTOCOL');
    final op = envelope['operation'];
    final args = Map<String, Object?>.from(envelope['arguments'] as Map? ?? {});
    final access = _access(peer);
    switch (op) {
      case 'retain':
        final ref = ResourceReference.fromJson(args['reference']);
        _local(peer, ref.identity.toJson());
        final retained = await hub.retain(
          ref,
          snapshots,
          access,
          name:
              (await hub
                      .provider(ref.identity)
                      .describe(ref.identity.resource, access))
                  .name,
          idempotencyKey: checkedRef(args['key']),
        );
        return {'reference': retained.toJson()};
      case 'prepare':
        final callback = prepareExecution;
        if (callback == null)
          throw const ResourceFailure('CONTEXT_UNAVAILABLE');
        final values = args['references'];
        if (values is! List || values.isEmpty || values.length > 10)
          throw const ResourceFailure('INVALID_REFERENCE');
        final refs = values.map(ResourceReference.fromJson).toList();
        for (final ref in refs) {
          if (!ref.pinned || ref.identity.scope != peer.scope)
            throw const ResourceFailure('DENIED');
          final p = hub.provider(ref.identity);
          final ownPhone =
              _peerSources[ref.identity.sourceKey]?.peerDevice ==
              peer.deviceRef;
          if (!ownPhone &&
              (!p.source.local ||
                  p.source.workspace != peer.workspaceRef ||
                  identical(p, snapshots))) {
            throw const ResourceFailure('DENIED');
          }
          if (ref.retained != null &&
              ref.retained!.identity.sourceKey != snapshots.source.key)
            throw const ResourceFailure('DENIED');
        }
        return callback(refs, checkedRef(args['key']));
      case 'catalog':
        return {
          'sources': [
            for (final p in hub.providers)
              if (p.source.local &&
                  !identical(p, snapshots) &&
                  p.source.workspace == peer.workspaceRef)
                p.source.toJson(),
            if (snapshots.source.workspace == peer.workspaceRef)
              {
                ...snapshots.source.toJson(),
                'internal': true,
                'canCreate': false,
                'canRename': false,
                'canEdit': false,
              },
          ],
          'maxTransferBytes': maxTransferBytes,
        };
      case 'list':
        final p = _local(peer, args['identity']);
        final page = await p.list(
          checkedRef(args['parent']),
          access,
          cursor: args['cursor'] as String?,
          limit: (args['limit'] as int?) ?? 100,
        );
        return {
          'entries': page.entries.map((e) => e.toJson()).toList(),
          if (page.nextCursor != null) 'nextCursor': page.nextCursor,
        };
      case 'describe':
        final p = _local(peer, args['identity'], allowSnapshotRead: true);
        final identity = ResourceIdentity.fromJson(args['identity']);
        return {
          'record': (await p.describe(identity.resource, access)).toJson(),
        };
      case 'read':
        final p = _local(peer, args['identity'], allowSnapshotRead: true);
        final identity = ResourceIdentity.fromJson(args['identity']);
        final content = await p.read(
          identity.resource,
          args['version'] == null
              ? null
              : ResourceVersion.fromJson(args['version']),
          access,
        );
        return _content(content);
      case 'create':
        final p = _local(peer, args['identity']);
        final bytes = _bytes(args['bytes']);
        final record = await p.create(
          parentRef: checkedRef(args['parent']),
          name: checkedName(args['name']),
          mediaType: args['mediaType'] as String,
          bytes: bytes,
          idempotencyKey: checkedRef(args['key']),
          access: access,
        );
        hub.changed();
        return {'record': record.toJson()};
      case 'folder':
        final p = _local(peer, args['identity']);
        final node = await p.createDirectory(
          checkedRef(args['parent']),
          checkedName(args['name']),
          access,
        );
        hub.changed();
        return {'nodeRef': node};
      case 'rename':
        final p = _local(peer, args['identity']);
        final identity = ResourceIdentity.fromJson(args['identity']);
        final record = await p.rename(
          identity.resource,
          checkedName(args['name']),
          args['expectedMetadataRevision'] as int,
          access,
        );
        await hub.refreshRecord(record);
        return {'record': record.toJson()};
      case 'commit':
        final p = _local(peer, args['identity']);
        final identity = ResourceIdentity.fromJson(args['identity']);
        final record = await p.commit(
          identity.resource,
          ResourceVersion.fromJson(args['base']),
          _bytes(args['bytes']),
          checkedRef(args['key']),
          access,
        );
        await hub.refreshRecord(record);
        return {'record': record.toJson()};
      case 'share':
        final record = ResourceRecord.fromJson(args['record']);
        final source = ResourceSource.fromJson(
          Map<String, Object?>.from(args['source'] as Map),
        );
        if (record.identity.scope != peer.scope ||
            source.scope != peer.scope ||
            record.identity.sourceKey != source.key)
          throw const ResourceFailure('DENIED');
        final existing = _peerSources[source.key];
        if (existing != null && existing.peerDevice != peer.deviceRef)
          throw const ResourceFailure('DENIED');
        if (existing == null &&
            hub.providers.any((p) => p.source.key == source.key))
          throw const ResourceFailure('SOURCE_ALREADY_REGISTERED');
        final readOnly = ResourceSource(
          scope: source.scope,
          id: source.id,
          workspace: source.workspace,
          mount: source.mount,
          label: source.label,
          workspaceLabel: source.workspaceLabel,
          mountLabel: source.mountLabel,
          deviceLabel: source.deviceLabel,
          deviceRef: source.deviceRef,
          local: false,
          exactVersions: source.exactVersions,
        );
        final p = existing ?? _PeerSource(this, readOnly, peer.deviceRef);
        p.records[record.identity.resource] = record;
        _peers[peer.deviceRef] = peer;
        _lastSeen[peer.deviceRef] = DateTime.now().millisecondsSinceEpoch;
        if (existing == null) {
          _peerSources[source.key] = p;
          hub.register(p);
        }
        await _savePeers();
        var reference = record.reference(pin: true);
        if (args['retain'] == true) {
          reference = await hub.retain(
            reference,
            snapshots,
            access,
            name: record.name,
            idempotencyKey: checkedRef(args['key']),
          );
        }
        hub.changed();
        return {'reference': reference.toJson(), 'record': record.toJson()};
      case 'poll':
        _peers[peer.deviceRef] = peer;
        _lastSeen[peer.deviceRef] = DateTime.now().millisecondsSinceEpoch;
        return {
          'requests': [
            for (final e in _pending.entries)
              if (e.value.peerDevice == peer.deviceRef)
                {
                  'id': e.key,
                  'identity': e.value.identity.toJson(),
                  if (e.value.version != null)
                    'version': e.value.version!.toJson(),
                },
          ],
        };
      case 'reply':
        final id = checkedRef(args['id']), pending = _pending[id];
        if (pending == null) throw const ResourceFailure('REQUEST_EXPIRED');
        if (pending.peerDevice != peer.deviceRef)
          throw const ResourceFailure('DENIED');
        if (!pending.completion.isCompleted) {
          final error = args['error'];
          if (error != null) {
            pending.completion.completeError(
              ResourceFailure(
                {
                      'DENIED',
                      'NOT_FOUND',
                      'VERSION_UNAVAILABLE',
                      'VERSION_MISMATCH',
                    }.contains(error)
                    ? error as String
                    : 'SOURCE_UNAVAILABLE',
              ),
            );
          } else {
            final content = _decodeContent(args);
            if (content.record.identity != pending.identity ||
                (pending.version != null &&
                    content.record.version != pending.version)) {
              throw const ResourceFailure('VERSION_MISMATCH');
            }
            pending.completion.complete(content);
          }
        }
        return {'accepted': true};
      case 'disconnect':
        _lastSeen.remove(peer.deviceRef);
        _peers.remove(peer.deviceRef);
        for (final p in _pending.values.where(
          (x) => x.peerDevice == peer.deviceRef,
        )) {
          if (!p.completion.isCompleted)
            p.completion.completeError(
              const ResourceFailure('SOURCE_UNAVAILABLE'),
            );
        }
        hub.changed();
        return {'disconnected': true};
      default:
        throw const ResourceFailure('UNSUPPORTED_OPERATION');
    }
  }

  bool _available(String peer) {
    final principal = _peers[peer], seen = _lastSeen[peer];
    final now = DateTime.now().millisecondsSinceEpoch;
    return !_closed &&
        principal != null &&
        principal.scope == hub.scope &&
        principal.expiresAtMs > now &&
        seen != null &&
        now - seen < heartbeatTtl.inMilliseconds;
  }

  Future<ResourceContent> _requestRead(
    String peer,
    ResourceIdentity id,
    ResourceVersion? version,
  ) async {
    if (!_available(peer)) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    if (_pending.length >= 64) throw const ResourceFailure('BUSY');
    final key = opaqueResourceId('read');
    final read = _SourceRead(peer, id, version);
    _pending[key] = read;
    try {
      return await read.completion.future.timeout(readTimeout);
    } on TimeoutException {
      _lastSeen.remove(peer);
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    } finally {
      _pending.remove(key);
    }
  }

  Future<void> _savePeers() {
    final store = peerStore;
    if (store == null) return Future.value();
    final data = <String, Object?>{
      'scope': hub.scope,
      'sources': [
        for (final p in _peerSources.values)
          {
            'source': p.source.toJson(),
            'peer': p.peerDevice,
            'records': p.records.values.map((r) => r.toJson()).toList(),
          },
      ],
    };
    final next = _persistTail
        .catchError((Object _) {})
        .then((_) => store.write(data));
    _persistTail = next;
    return next;
  }

  Future<void> close() async {
    _closed = true;
    for (final p in _pending.values) {
      if (!p.completion.isCompleted)
        p.completion.completeError(const ResourceFailure('SOURCE_UNAVAILABLE'));
    }
    _peers.clear();
    _lastSeen.clear();
    await _persistTail;
  }

  static Map<String, Object?> _content(ResourceContent content) {
    if (content.bytes.length > maxTransferBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    return {
      'record': content.record.toJson(),
      'bytes': base64Encode(content.bytes),
    };
  }

  static List<int> _bytes(Object? value) {
    if (value is! String ||
        value.length > (maxTransferBytes * 4 / 3).ceil() + 4)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    try {
      final bytes = base64Decode(value);
      if (bytes.length > maxTransferBytes)
        throw const ResourceFailure('TRANSFER_TOO_LARGE');
      return bytes;
    } on FormatException {
      throw const ResourceFailure('INVALID_BYTES');
    }
  }

  static ResourceContent _decodeContent(Map<String, Object?> m) =>
      ResourceContent(ResourceRecord.fromJson(m['record']), _bytes(m['bytes']));
}

final class _SourceRead {
  _SourceRead(this.peerDevice, this.identity, this.version);
  final String peerDevice;
  final ResourceIdentity identity;
  final ResourceVersion? version;
  final completion = Completer<ResourceContent>();
}

final class _PeerSource implements ResourceProvider {
  _PeerSource(this.service, this.source, this.peerDevice);
  final ResourceRpcService service;
  @override
  final ResourceSource source;
  final String peerDevice;
  final records = <String, ResourceRecord>{};
  @override
  bool get available => service._available(peerDevice);
  void _check(String ref, ResourceAccess access, ResourceOperation op) {
    access.check(
      source.identity(ref),
      op,
      currentGeneration: service.hub.generation,
    );
    if (!records.containsKey(ref)) throw const ResourceFailure('DENIED');
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access, ResourceOperation.describe);
    return records[resourceRef]!;
  }

  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access, ResourceOperation.read);
    return service._requestRead(
      peerDevice,
      source.identity(resourceRef),
      version,
    );
  }

  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    access.check(
      source.identity(parentRef),
      ResourceOperation.describe,
      currentGeneration: service.hub.generation,
    );
    if (parentRef != 'root') throw const ResourceFailure('NOT_FOUND');
    return ResourcePage(
      records.values
          .map(
            (r) => ResourceDirectoryEntry(
              nodeRef: r.identity.resource,
              name: r.name,
              resource: r,
            ),
          )
          .toList(),
    );
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
  }) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<ResourceRecord> rename(
    String resourceRef,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<ResourceRecord> commit(
    String resourceRef,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
}

/// HTTP is an adapter; production callers supply the existing paired cookie.
/// Insecure LAN is opt-in for an explicit acceptance build, never inferred.
final class ResourceHttpTransport {
  ResourceHttpTransport(
    this.endpoint, {
    required this.headers,
    bool allowInsecureTesting = false,
  }) {
    if (endpoint.userInfo.isNotEmpty ||
        endpoint.hasFragment ||
        endpoint.hasQuery ||
        (endpoint.scheme != 'https' &&
            !(endpoint.scheme == 'http' &&
                (endpoint.host == '127.0.0.1' ||
                    endpoint.host == 'localhost' ||
                    allowInsecureTesting)))) {
      throw const ResourceFailure('HTTPS_REQUIRED');
    }
    _client.maxConnectionsPerHost = 6;
    _client.connectionTimeout = const Duration(seconds: 10);
  }
  final Uri endpoint;
  final Map<String, String> Function() headers;
  final HttpClient _client = HttpClient();
  Future<Map<String, Object?>> call(Map<String, Object?> envelope) async {
    final request = await _client.postUrl(endpoint);
    request.followRedirects = false;
    request.headers.contentType = ContentType.json;
    for (final e in headers().entries) {
      request.headers.set(e.key, e.value);
    }
    request.write(jsonEncode(envelope));
    final response = await request.close().timeout(const Duration(seconds: 25));
    final bytes = <int>[];
    await for (final chunk in response.timeout(const Duration(seconds: 25))) {
      if (bytes.length + chunk.length > 24 * 1024 * 1024)
        throw const ResourceFailure('RESPONSE_TOO_LARGE');
      bytes.addAll(chunk);
    }
    if (response.statusCode >= 300 && response.statusCode < 400)
      throw const ResourceFailure('REDIRECT_DENIED');
    final m = Map<String, Object?>.from(jsonDecode(utf8.decode(bytes)) as Map);
    if (response.statusCode != 200 || m['error'] != null)
      throw ResourceFailure(
        (m['error'] is String ? m['error'] as String : null) ??
            'TRANSPORT_UNAVAILABLE',
      );
    return m;
  }

  void close() => _client.close(force: true);
}

final class ResourceRpcClient {
  const ResourceRpcClient(this.transport);
  final ResourceRpcTransport transport;
  Future<Map<String, Object?>> call(
    String operation, [
    Map<String, Object?> arguments = const {},
  ]) => transport({
    'protocol': ResourceRpcService.protocol,
    'operation': operation,
    'arguments': arguments,
  });
  Future<List<RemoteResourceProvider>> catalog({
    required String expectedScope,
    required int Function() generation,
  }) async {
    final reply = await call('catalog');
    final result = <RemoteResourceProvider>[];
    for (final value in reply['sources'] as List) {
      final source = ResourceSource.fromJson(
        Map<String, Object?>.from(value as Map),
      );
      if (source.scope != expectedScope) throw const ResourceFailure('DENIED');
      result.add(RemoteResourceProvider(this, source, generation));
    }
    return result;
  }
}

final class RemoteResourceProvider implements ResourceProvider {
  RemoteResourceProvider(
    this.client,
    ResourceSource advertised,
    this.generation,
  ) : source = ResourceSource(
        scope: advertised.scope,
        id: advertised.id,
        workspace: advertised.workspace,
        mount: advertised.mount,
        label: advertised.label,
        workspaceLabel: advertised.workspaceLabel,
        mountLabel: advertised.mountLabel,
        deviceLabel: advertised.deviceLabel,
        deviceRef: advertised.deviceRef,
        local: false,
        canCreate: advertised.canCreate,
        canRename: advertised.canRename,
        canEdit: advertised.canEdit,
        exactVersions: advertised.exactVersions,
        internal: advertised.internal,
      );
  final ResourceRpcClient client;
  @override
  final ResourceSource source;
  final int Function() generation;
  @override
  bool available = true;
  Future<Map<String, Object?>> _call(
    String op,
    String ref,
    ResourceAccess access,
    ResourceOperation operation, [
    Map<String, Object?> args = const {},
  ]) async {
    access.check(
      source.identity(ref),
      operation,
      currentGeneration: generation(),
    );
    try {
      final r = await client.call(op, {
        'identity': source.identity(ref).toJson(),
        ...args,
      });
      available = true;
      return r;
    } on SocketException {
      available = false;
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    } on TimeoutException {
      available = false;
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    }
  }

  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    final m = await _call(
      'list',
      parentRef,
      access,
      ResourceOperation.describe,
      {
        'parent': parentRef,
        'limit': limit,
        if (cursor != null) 'cursor': cursor,
      },
    );
    return ResourcePage(
      (m['entries'] as List)
          .map(
            (e) => ResourceDirectoryEntry.fromJson(
              Map<String, Object?>.from(e as Map),
            ),
          )
          .toList(),
      nextCursor: m['nextCursor'] as String?,
    );
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async => ResourceRecord.fromJson(
    (await _call(
      'describe',
      resourceRef,
      access,
      ResourceOperation.describe,
    ))['record'],
  );
  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) async => ResourceRpcService._decodeContent(
    await _call('read', resourceRef, access, ResourceOperation.read, {
      if (version != null) 'version': version.toJson(),
    }),
  );
  @override
  Future<ResourceRecord> create({
    required String parentRef,
    required String name,
    required String mediaType,
    required List<int> bytes,
    required String idempotencyKey,
    required ResourceAccess access,
    ResourceReference? provenance,
  }) async {
    if (provenance != null)
      throw const ResourceFailure('UNSUPPORTED_OPERATION');
    if (bytes.length > ResourceRpcService.maxTransferBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    return ResourceRecord.fromJson(
      (await _call('create', parentRef, access, ResourceOperation.create, {
        'parent': parentRef,
        'name': name,
        'mediaType': mediaType,
        'bytes': base64Encode(bytes),
        'key': idempotencyKey,
      }))['record'],
    );
  }

  @override
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  ) async => checkedRef(
    (await _call('folder', parentRef, access, ResourceOperation.create, {
      'parent': parentRef,
      'name': name,
    }))['nodeRef'],
  );
  @override
  Future<ResourceRecord> rename(
    String resourceRef,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  ) async => ResourceRecord.fromJson(
    (await _call('rename', resourceRef, access, ResourceOperation.rename, {
      'name': name,
      'expectedMetadataRevision': expectedMetadataRevision,
    }))['record'],
  );
  @override
  Future<ResourceRecord> commit(
    String resourceRef,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) async {
    if (bytes.length > ResourceRpcService.maxTransferBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    return ResourceRecord.fromJson(
      (await _call('commit', resourceRef, access, ResourceOperation.edit, {
        'base': base.toJson(),
        'bytes': base64Encode(bytes),
        'key': idempotencyKey,
      }))['record'],
    );
  }
}

/// While foreground/paired, answers only reads of resources explicitly shared
/// by this user. No server socket or filesystem is exposed on the phone.
final class ResourceOriginBridge {
  ResourceOriginBridge({
    required this.client,
    required this.localHub,
    this.pollInterval = const Duration(seconds: 1),
  });
  final ResourceRpcClient client;
  final ResourceCenterHub localHub;
  final Duration pollInterval;
  final _shared = <String, ResourceRecord>{};
  bool _running = false;
  Future<void>? _loop;
  void start() {
    if (_running) return;
    _running = true;
    _loop = _poll();
  }

  Future<ResourceReference> share(
    ResourceRecord record, {
    required bool retain,
    required String idempotencyKey,
  }) async {
    if (record.identity.scope != localHub.scope)
      throw const ResourceFailure('DENIED');
    final provider = localHub.provider(record.identity);
    _shared[record.identity.key] = record;
    start();
    final m = await client.call('share', {
      'record': record.toJson(),
      'source': provider.source.toJson(),
      'retain': retain,
      'key': idempotencyKey,
    });
    return ResourceReference.fromJson(m['reference']);
  }

  Future<void> _poll() async {
    while (_running) {
      try {
        final response = await client.call('poll');
        for (final value in response['requests'] as List? ?? const []) {
          if (!_running) break;
          final request = Map<String, Object?>.from(value as Map);
          final id = checkedRef(request['id']);
          final identity = ResourceIdentity.fromJson(request['identity']);
          Map<String, Object?> reply;
          try {
            if (!_shared.containsKey(identity.key))
              throw const ResourceFailure('DENIED');
            final record = _shared[identity.key]!;
            final content = await localHub
                .provider(identity)
                .read(
                  record.identity.resource,
                  request['version'] == null
                      ? null
                      : ResourceVersion.fromJson(request['version']),
                  localHub.access(
                    'source-bridge',
                    operations: {ResourceOperation.read},
                    resourceKeys: {identity.key},
                  ),
                );
            reply = {'id': id, ...ResourceRpcService._content(content)};
          } on ResourceFailure catch (e) {
            reply = {'id': id, 'error': e.code};
          }
          if (_running) await client.call('reply', reply);
        }
      } on Object {
        /* A lost grant or connection must not expose or widen source access. */
      }
      if (_running) await Future<void>.delayed(pollInterval);
    }
  }

  Future<void> stop() async {
    _running = false;
    _shared.clear();
    try {
      await client.call('disconnect');
    } on Object {
      /* Already disconnected. */
    }
    await _loop;
    _shared.clear();
  }
}
