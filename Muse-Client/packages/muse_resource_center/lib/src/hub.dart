import 'dart:async';
import 'model.dart';
import 'provider.dart';

final class RecentResource {
  const RecentResource(this.record, this.openedAtMs, this.handlerId);
  final ResourceRecord record;
  final int openedAtMs;
  final String handlerId;
  Map<String, Object?> toJson() => {
    'record': record.toJson(),
    'openedAtMs': openedAtMs,
    'handlerId': handlerId,
  };
  factory RecentResource.fromJson(Map<String, Object?> m) => RecentResource(
    ResourceRecord.fromJson(m['record']),
    m['openedAtMs'] as int,
    m['handlerId'] as String,
  );
}

/// The same coordinator is consumed by Desktop, Mobile, Web and Agent adapters.
/// It does not depend on widgets, the filesystem, a transport, or file formats.
final class ResourceCenterHub {
  ResourceCenterHub({
    required String scope,
    ResourceStateStore? store,
    int Function()? clock,
  }) : scope = checkedRef(scope),
       store = store ?? MemoryResourceStateStore(),
       _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);
  final String scope;
  final ResourceStateStore store;
  final int Function() _clock;
  final _providers = <String, ResourceProvider>{};
  final _hiddenSources = <String>{};
  final _recent = <RecentResource>[];
  final _defaults = <String, String>{};
  final _events = StreamController<void>.broadcast(sync: true);
  Stream<void> get changes => _events.stream;
  int generation = 1;
  ResourceReference? clipboard;
  bool _disposed = false;
  Future<void> _writes = Future.value();
  List<ResourceProvider> get providers => List.unmodifiable(_providers.values);
  List<ResourceProvider> get visibleProviders => List.unmodifiable(
    _providers.entries
        .where((e) => !_hiddenSources.contains(e.key))
        .map((e) => e.value),
  );
  List<RecentResource> get recent => List.unmodifiable(_recent);
  void changed() {
    if (!_disposed) _events.add(null);
  }

  ResourceAccess access(
    String audience, {
    Set<ResourceOperation>? operations,
    Set<String>? resourceKeys,
    bool allowOfflineReplica = false,
  }) => ResourceAccess(
    scope: scope,
    audience: audience,
    generation: generation,
    expiresAtMs: _clock() + const Duration(minutes: 10).inMilliseconds,
    operations: operations ?? ResourceOperation.values.toSet(),
    resourceKeys: resourceKeys,
    allowOfflineReplica: allowOfflineReplica,
  );
  Future<void> load() async {
    final saved = await store.read();
    if (saved['scope'] != scope) return;
    final cutoff = _clock() - const Duration(days: 90).inMilliseconds;
    for (final item in (saved['recent'] as List? ?? const [])) {
      try {
        final r = RecentResource.fromJson(
          Map<String, Object?>.from(item as Map),
        );
        if (r.record.identity.scope == scope &&
            r.openedAtMs >= cutoff &&
            !_recent.any((x) => x.record.identity == r.record.identity))
          _recent.add(r);
      } on Object {
        /* A corrupt optional history record cannot break resource access. */
      }
    }
    _recent.sort((a, b) => b.openedAtMs.compareTo(a.openedAtMs));
    if (_recent.length > 200) _recent.removeRange(200, _recent.length);
    final defaults = saved['defaults'];
    if (defaults is Map) {
      for (final e in defaults.entries) {
        if (e.key is String && e.value is String)
          _defaults[e.key as String] = e.value as String;
      }
    }
    changed();
  }

  void register(ResourceProvider provider, {bool visible = true}) {
    if (provider.source.scope != scope) throw const ResourceFailure('DENIED');
    final key = provider.source.key;
    if (_providers.containsKey(key) && !identical(_providers[key], provider)) {
      throw const ResourceFailure('SOURCE_ALREADY_REGISTERED');
    }
    _providers[key] = provider;
    if (visible) {
      _hiddenSources.remove(key);
    } else {
      _hiddenSources.add(key);
    }
    changed();
  }

  void unregister(String sourceKey) {
    _providers.remove(sourceKey);
    _hiddenSources.remove(sourceKey);
    changed();
  }

  ResourceProvider provider(ResourceIdentity identity) {
    if (identity.scope != scope) throw const ResourceFailure('DENIED');
    final p = _providers[identity.sourceKey];
    if (p == null) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    return p;
  }

  Future<OpenedResource> resolve(
    ResourceReference ref,
    ResourceAccess access,
  ) async {
    access.check(
      ref.identity,
      ResourceOperation.read,
      currentGeneration: generation,
      nowMs: _clock(),
    );
    if (ref.retained != null && access.allowOfflineReplica) {
      final retained = ref.retained!;
      access.check(
        retained.identity,
        ResourceOperation.read,
        currentGeneration: generation,
        nowMs: _clock(),
      );
      try {
        final p = provider(retained.identity);
        final content = await p.read(
          retained.identity.resource,
          retained.version,
          access,
        );
        if (content.record.version.digest != ref.version!.digest)
          throw const ResourceFailure('DIGEST_MISMATCH');
        return OpenedResource(
          reference: ref,
          content: content,
          fromSnapshot: true,
          sourceOffline: _providers[ref.identity.sourceKey]?.available != true,
        );
      } on ResourceFailure catch (e) {
        if (!{
          'NOT_FOUND',
          'SOURCE_UNAVAILABLE',
          'VERSION_UNAVAILABLE',
        }.contains(e.code))
          rethrow;
      }
    }
    final p = provider(ref.identity);
    if (!p.available) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    final content = await p.read(ref.identity.resource, ref.version, access);
    if (content.record.identity != ref.identity ||
        (ref.version != null && content.record.version != ref.version)) {
      throw const ResourceFailure('VERSION_MISMATCH');
    }
    return OpenedResource(
      reference: ResourceReference(
        identity: ref.identity,
        version: content.record.version,
      ),
      content: content,
    );
  }

  Future<ResourceReference> retain(
    ResourceReference ref,
    ResourceProvider snapshots,
    ResourceAccess access, {
    required String name,
    required String idempotencyKey,
  }) async {
    access.check(
      ref.identity,
      ResourceOperation.retain,
      currentGeneration: generation,
      nowMs: _clock(),
    );
    final opened = await resolve(ref, access);
    final pinned = ResourceReference(
      identity: ref.identity,
      version: opened.content.record.version,
    );
    final copy = await snapshots.create(
      parentRef: 'root',
      name: name,
      mediaType: opened.content.record.mediaType,
      bytes: opened.content.bytes,
      idempotencyKey: idempotencyKey,
      access: access,
      provenance: pinned,
    );
    return ResourceReference(
      identity: pinned.identity,
      version: pinned.version,
      retained: RetainedResource(copy.identity, copy.version),
    );
  }

  Future<void> recordOpened(ResourceRecord record, String handlerId) async {
    if (record.identity.scope != scope) throw const ResourceFailure('DENIED');
    _recent.removeWhere((x) => x.record.identity == record.identity);
    _recent.insert(0, RecentResource(record, _clock(), handlerId));
    if (_recent.length > 200) _recent.removeRange(200, _recent.length);
    changed();
    await _persist();
  }

  Future<void> refreshRecord(ResourceRecord record) async {
    for (var i = 0; i < _recent.length; i++)
      if (_recent[i].record.identity == record.identity) {
        _recent[i] = RecentResource(
          record,
          _recent[i].openedAtMs,
          _recent[i].handlerId,
        );
      }
    changed();
    await _persist();
  }

  Future<void> removeRecent(ResourceIdentity identity) async {
    _recent.removeWhere((x) => x.record.identity == identity);
    changed();
    await _persist();
  }

  void copy(ResourceReference reference) {
    if (reference.identity.scope != scope)
      throw const ResourceFailure('DENIED');
    clipboard = reference;
    changed();
  }

  void clearClipboard() {
    clipboard = null;
    changed();
  }

  String? defaultHandler(String mediaType) => _defaults[mediaType];
  Future<void> setDefault(String mediaType, String id) async {
    _defaults[mediaType] = id;
    await _persist();
  }

  Future<void> _persist() {
    final state = <String, Object?>{
      'scope': scope,
      'recent': _recent.map((r) => r.toJson()).toList(),
      'defaults': Map<String, String>.of(_defaults),
    };
    final next = _writes
        .catchError((Object _) {})
        .then((_) => store.write(state));
    _writes = next;
    return next;
  }

  Future<void> invalidate() async {
    generation++;
    clipboard = null;
    _providers.clear();
    _hiddenSources.clear();
    _recent.clear();
    changed();
    await _persist();
  }

  Future<void> dispose() async {
    _disposed = true;
    generation++;
    clipboard = null;
    await _writes;
    await _events.close();
  }
}
