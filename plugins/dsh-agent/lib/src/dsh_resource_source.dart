import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart'
    show ConnectionTrace;
import 'package:mime/mime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// DSH owns path interpretation and filesystem access, including sandbox paths.
/// Only opaque file handles leave this plugin's source adapter.
final class DshResourceSource
    implements
        ResourceProvider,
        ResourceLocatorProvider,
        ResourceRelativeLocatorProvider {
  DshResourceSource(
    this.context, {
    required this.endpoint,
    required this.token,
    required this.ready,
    required this.enabled,
  }) : source = ResourceSource(
         scope: context.scope,
         id: 'dsh-files-${context.deviceRef}',
         workspace: context.workspace,
         mount: 'dsh-files',
         label: 'DSH',
         workspaceLabel: context.workspaceLabel,
         mountLabel: 'DSH',
         deviceLabel: context.deviceLabel,
         deviceRef: context.deviceRef,
         local: true,
         exactVersions: false,
         internal: true,
         locatorOwner: 'dsh',
       );
  final ResourceSourceContext context;
  final Uri? Function() endpoint;
  final String Function() token;
  final Future<void> Function() ready;
  final bool Function() enabled;
  final _entries = <String, Map<String, Object?>>{};
  final _bridgeRefs = <String, String>{};
  late final _state = context.state ?? MemoryResourceStateStore();
  late final Future<void> _loaded = _load();
  Future<void> _writes = Future.value();
  Future<void> _load() async {
    final saved = await _state.read();
    if (saved['scope'] != source.scope || saved['entries'] is! Map) return;
    for (final e in (saved['entries'] as Map).entries.take(5000)) {
      if (e.value is! Map) continue;
      final value = Map<String, Object?>.from(e.value as Map);
      if (value['sessionId'] is String && value['target'] is String)
        _entries[checkedRef(e.key)] = value;
    }
  }

  Future<void> _save() {
    final state =
        jsonDecode(jsonEncode({'scope': source.scope, 'entries': _entries}))
            as Map<String, dynamic>;
    return _writes = _writes
        .catchError((Object _) {})
        .then((_) => _state.write(state));
  }

  String _identity(String sessionId, String target) {
    for (final e in _entries.entries) {
      if (e.value['sessionId'] == sessionId && e.value['target'] == target)
        return e.key;
    }
    if (_entries.length >= 5000) throw const ResourceFailure('CATALOG_LIMIT');
    final random = Random.secure();
    final id =
        'dsh-${List.generate(16, (_) => random.nextInt(256).toRadixString(16).padLeft(2, '0')).join()}';
    _entries[id] = {'sessionId': sessionId, 'target': target};
    return id;
  }

  Future<String> _bridgeRef(String resourceRef) async {
    await _loaded;
    final entry = _entries[resourceRef];
    if (entry == null) throw const ResourceFailure('NOT_FOUND');
    final cached = _bridgeRefs[resourceRef];
    if (cached != null) return cached;
    final result = await _request('resolve', {
      'locator': {
        'owner': 'dsh',
        'kind': 'canonical',
        'target': entry['target'],
        'context': {'sessionId': entry['sessionId']},
      },
    });
    final matches = result['matches'] as List;
    if (matches.length != 1 || result['truncated'] == true)
      throw const ResourceFailure('NOT_FOUND');
    return _bridgeRefs[resourceRef] = checkedRef(matches.single['ref']);
  }

  @override
  final ResourceSource source;
  @override
  bool get available => enabled();
  @override
  String get locatorOwner => 'dsh';
  void _check(String ref, ResourceAccess access) {
    if (!available) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    access.check(
      source.identity(ref),
      ResourceOperation.read,
      currentGeneration: context.generation(),
    );
  }

  Future<Map<String, Object?>> _request(
    String operation,
    Map<String, Object?> body,
  ) => ConnectionTrace().measure('resource.owner.$operation', (span) async {
    await ready();
    span.mark('ready');
    if (!available || endpoint() == null)
      throw const ResourceFailure('SOURCE_UNAVAILABLE');
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 5);
    try {
      final request = await client.postUrl(
        endpoint()!.resolve('/openmuse-native/v1/session/files/$operation'),
      );
      request.headers
        ..contentType = ContentType.json
        ..set('x-openmuse-bridge-token', token());
      request.headers.set('traceparent', span.traceparent);
      request.write(jsonEncode(body));
      final response = await request.close().timeout(
        const Duration(seconds: 15),
      );
      span.mark('response', status: response.statusCode);
      final bytes = <int>[];
      await for (final part in response.timeout(const Duration(seconds: 15))) {
        if (bytes.length + part.length > 24 * 1024 * 1024)
          throw const ResourceFailure('TRANSFER_TOO_LARGE');
        bytes.addAll(part);
      }
      final value = jsonDecode(utf8.decode(bytes));
      span.mark('decoded', bytes: bytes.length);
      if (value is! Map) throw const ResourceFailure('INVALID_ENVELOPE');
      if (response.statusCode != 200)
        throw ResourceFailure(
          value['error'] is String
              ? value['error'] as String
              : 'SOURCE_UNAVAILABLE',
        );
      return Map<String, Object?>.from(value);
    } finally {
      client.close(force: true);
    }
  });
  @override
  Future<ResourceLocatorResult> locateRelative(
    ResourceReference base,
    String target,
    ResourceAccess access,
  ) async {
    if (base.identity.sourceKey != source.key ||
        base.identity.scope != source.scope ||
        base.retained != null) {
      throw const ResourceFailure('DENIED');
    }
    final ref = base.identity.resource;
    _check(ref, access);
    await _loaded;
    final entry = _entries[ref];
    if (entry == null) throw const ResourceFailure('NOT_FOUND');
    // Refresh the private handle so an old open document still has a verified
    // source context after the bridge TTL or a Desktop restart.
    _bridgeRefs.remove(ref);
    final bridgeRef = await _bridgeRef(ref);
    return locate(
      ResourceLocator(
        owner: locatorOwner,
        kind: 'relative',
        target: target,
        context: {'sessionId': entry['sessionId'], 'baseRef': bridgeRef},
      ),
      access,
    );
  }

  @override
  Future<ResourceLocatorResult> locate(
    ResourceLocator locator,
    ResourceAccess access,
  ) async {
    _check('root', access);
    await _loaded;
    final value = await _request('resolve', {'locator': locator.toJson()});
    final matches = <ResourceLocatorMatch>[];
    for (final item in value['matches'] as List) {
      final label = item['label'] as String;
      final id = _identity(locator.context['sessionId'] as String, label);
      _bridgeRefs[id] = checkedRef(item['ref']);
      matches.add(
        ResourceLocatorMatch(
          ResourceReference(identity: source.identity(id)),
          label,
        ),
      );
    }
    await _save();
    return ResourceLocatorResult(
      matches,
      truncated: value['truncated'] == true,
      line: value['line'] as int?,
      column: value['column'] as int?,
    );
  }

  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access);
    Map<String, Object?> value;
    try {
      value = await _request('read', {'ref': await _bridgeRef(resourceRef)});
    } on ResourceFailure catch (e) {
      if (e.code != 'EXPIRED') rethrow;
      _bridgeRefs.remove(resourceRef);
      value = await _request('read', {'ref': await _bridgeRef(resourceRef)});
    }
    _check(resourceRef, access);
    final bytes = base64Decode(value['bytes'] as String);
    final digest = resourceDigest(bytes);
    if (digest != value['digest'])
      throw const ResourceFailure('DIGEST_MISMATCH');
    final actual = ResourceVersion(revision: 'sha256-$digest', digest: digest);
    if (version != null && version != actual)
      throw const ResourceFailure('VERSION_UNAVAILABLE');
    final parts = (value['label'] as String).split('/');
    final name = checkedName(parts.last);
    final mime =
        lookupMimeType(name, headerBytes: bytes.take(64).toList()) ??
        'application/octet-stream';
    return ResourceContent(
      ResourceRecord(
        identity: source.identity(resourceRef),
        version: actual,
        sourceLabel: source.deviceLabel,
        location: ['DSH', ...parts.take(parts.length - 1)],
        parentRef: 'root',
        metadataRevision: 1,
        readOnly: true,
        descriptor: MuseResourceDescriptorV1(
          resourceRef: resourceRef,
          revision: actual.revision,
          displayName: name,
          mediaType: mime,
          format: MuseResourceFormat(
            formatId: mime,
            confidence: MuseFormatConfidence.claimed,
          ),
          capabilities: const [
            MuseResourceCapability.describe,
            MuseResourceCapability.materialize,
          ],
          security: const MuseResourceSecurity(
            classification: MuseSecurityClassification.internal,
            activeContent: MuseActiveContent.unknown,
          ),
        ),
      ),
      bytes,
    );
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async => (await read(resourceRef, null, access)).record;
  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    _check(parentRef, access);
    return const ResourcePage([]);
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
