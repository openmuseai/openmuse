import 'dart:convert';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'remote_app.dart';

/// Digest-pinned MCP data resources. UI HTML still follows the reviewed UI path.
/// Resource URIs never select an endpoint, account, token or write capability.
ResourceReference checkedMcpResourceLink(
  Object? value,
  String account,
  String appId,
) {
  if (value is! String || account == 'signed-out-local')
    throw StateError('APP_RESOURCE_LINK_INVALID');
  final u = Uri.parse(value);
  if (u.scheme != 'mcp-resource' ||
      u.host != appId ||
      u.userInfo.isNotEmpty ||
      u.hasQuery ||
      u.hasFragment ||
      u.pathSegments.length != 1)
    throw StateError('APP_RESOURCE_LINK_INVALID');
  final key = u.pathSegments.single;
  decodePinnedMcpResource(key);
  final hash = key.substring(key.lastIndexOf('.') + 1);
  return ResourceReference(
    identity: ResourceIdentity(
      scope: account,
      source: 'mcp-$appId',
      workspace: 'installed-apps',
      mount: 'resources',
      resource: key,
    ),
    version: ResourceVersion(revision: 'sha256-$hash', digest: hash),
  );
}

({String uri, String sha256}) decodePinnedMcpResource(String key) {
  checkedRef(key);
  final i = key.lastIndexOf('.');
  if (i < 1 || !RegExp(r'^[a-f0-9]{64}$').hasMatch(key.substring(i + 1)))
    throw const ResourceFailure('INVALID_REFERENCE');
  try {
    final uri = utf8.decode(
      base64Url.decode(base64Url.normalize(key.substring(0, i))),
    );
    if (uri.length > 512 || !Uri.parse(uri).hasScheme || uri.startsWith('ui:'))
      throw const FormatException();
    return (uri: uri, sha256: key.substring(i + 1));
  } on Object {
    throw const ResourceFailure('INVALID_REFERENCE');
  }
}

final class McpDataResourceSource implements ResourceProvider {
  McpDataResourceSource(this.app, this.context)
    : source = ResourceSource(
        scope: context.scope,
        id: 'mcp-${app.id}',
        workspace: 'installed-apps',
        mount: 'resources',
        label: app.title,
        workspaceLabel: 'Installed Apps',
        mountLabel: 'Resources',
        deviceLabel: 'MCP',
        local: false,
      );
  final RemoteMcpApp app;
  final ResourceSourceContext context;
  @override
  final ResourceSource source;
  @override
  bool get available =>
      app.scope() == context.scope &&
      app.definition['owner'] == resourceDigest(utf8.encode(context.scope));
  void _check(String ref, ResourceAccess access, ResourceOperation op) {
    if (!available) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    access.check(
      source.identity(ref),
      op,
      currentGeneration: context.generation(),
    );
  }

  Future<List<Map>> _declared() async {
    final result = await app.request('resources/list', {});
    return (result['resources'] as List? ?? [])
        .whereType<Map>()
        .where(
          (r) => r['uri'] is String && !(r['uri'] as String).startsWith('ui:'),
        )
        .toList();
  }

  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    _check(parentRef, access, ResourceOperation.describe);
    if (parentRef != 'root' || limit < 1 || limit > 1000)
      throw const ResourceFailure('INVALID_REFERENCE');
    final offset = cursor == null ? 0 : int.tryParse(cursor);
    if (offset == null || offset < 0)
      throw const ResourceFailure('INVALID_REFERENCE');
    final entries = <ResourceDirectoryEntry>[];
    for (final r in await _declared()) {
      final hash = (r['_meta'] as Map?)?['sha256'];
      if (hash is! String || !RegExp(r'^[a-f0-9]{64}$').hasMatch(hash))
        continue;
      final key =
          '${base64Url.encode(utf8.encode(r['uri'] as String)).replaceAll('=', '')}.$hash';
      if (key.length > 256) continue;
      entries.add(
        ResourceDirectoryEntry(
          nodeRef: key,
          name: checkedName(r['name']),
          deferredIdentity: source.identity(key),
          mediaType: r['mimeType'] as String?,
        ),
      );
    }
    _check(parentRef, access, ResourceOperation.describe);
    return ResourcePage(
      entries.skip(offset).take(limit).toList(),
      nextCursor: offset + limit < entries.length ? '${offset + limit}' : null,
    );
  }

  Future<ResourceContent> _read(
    String ref,
    ResourceAccess access,
    ResourceOperation op,
  ) async {
    _check(ref, access, op);
    final target = decodePinnedMcpResource(ref);
    final metadata = (await _declared())
        .where((r) => r['uri'] == target.uri)
        .singleOrNull;
    if (metadata == null) throw const ResourceFailure('DENIED');
    final result = await app.readDataResource(target.uri);
    _check(ref, access, op);
    final contents = (result['contents'] as List).cast<Map>();
    if (contents.length != 1 || contents.single['uri'] != target.uri)
      throw const ResourceFailure('RESOURCE_MISMATCH');
    final c = contents.single;
    final bytes = c['text'] is String
        ? utf8.encode(c['text'] as String)
        : base64.decode(c['blob'] as String);
    if (bytes.length > 8 * 1024 * 1024)
      throw const ResourceFailure('RESOURCE_TOO_LARGE');
    final hash = resourceDigest(bytes);
    if (hash != target.sha256) throw const ResourceFailure('DIGEST_MISMATCH');
    final mime = metadata['mimeType'] as String? ?? 'application/octet-stream';
    if (c['mimeType'] != null && c['mimeType'] != mime)
      throw const ResourceFailure('RESOURCE_MISMATCH');
    final name = checkedName(metadata['name']);
    final revision = 'sha256-$hash';
    final record = ResourceRecord(
      identity: source.identity(ref),
      version: ResourceVersion(revision: revision, digest: hash),
      descriptor: MuseResourceDescriptorV1(
        resourceRef: ref,
        revision: revision,
        displayName: name,
        mediaType: mime,
        format: const MuseResourceFormat(
          formatId: 'mcp-data',
          confidence: MuseFormatConfidence.claimed,
        ),
        capabilities: const [
          MuseResourceCapability.describe,
          MuseResourceCapability.snapshot,
        ],
        security: const MuseResourceSecurity(
          classification: MuseSecurityClassification.confidential,
          activeContent: MuseActiveContent.unknown,
        ),
      ),
      sourceLabel: app.title,
      location: [name],
      parentRef: 'root',
      metadataRevision: 0,
      readOnly: true,
    );
    return ResourceContent(record, bytes);
  }

  @override
  Future<ResourceRecord> describe(String ref, ResourceAccess access) async =>
      (await _read(ref, access, ResourceOperation.describe)).record;
  @override
  Future<ResourceContent> read(
    String ref,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    final result = await _read(ref, access, ResourceOperation.read);
    if (version != null && version.digest != result.record.version.digest)
      throw const ResourceFailure('VERSION_MISMATCH');
    return result;
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
    String ref,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
  @override
  Future<ResourceRecord> commit(
    String ref,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  ) async => throw const ResourceFailure('READ_ONLY');
}
