import 'dart:io';
import 'package:mime/mime.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'local_provider.dart';

/// Explicit Host-mounted filesystem adapter. Opaque references are persisted in
/// private storage, never derived from public paths. External files remain
/// read-only until a platform supplies a transactional writer. Exact old bytes
/// are not invented when another process changes the file.
final class FileTreeResourceProvider implements ResourceProvider {
  FileTreeResourceProvider._(
    this.root,
    this.index,
    this._state,
    this.source,
    this.generation,
  );
  final Directory root;
  final ResourceStateStore index;
  final Map<String, Object?> _state;
  @override
  final ResourceSource source;
  final int Function() generation;
  Future<void> _writes = Future.value();
  @override
  bool available = true;
  static Future<FileTreeResourceProvider> open({
    required Directory root,
    required ResourceStateStore index,
    required String scope,
    required String workspace,
    required String deviceRef,
    required String deviceLabel,
    required String workspaceLabel,
    required String mountLabel,
    required int Function() generation,
  }) async {
    final canonical = Directory(await root.resolveSymbolicLinks());
    var state = await index.read();
    if (state.isEmpty) {
      state = {
        'scope': scope,
        'root': canonical.path,
        'source': opaqueResourceId('tree'),
        'paths': <String, Object?>{'root': <String>[]},
      };
    }
    if (state['scope'] != scope || state['root'] != canonical.path)
      throw const ResourceFailure('STORAGE_SCOPE_MISMATCH');
    final id = checkedRef(state['source']);
    final value = FileTreeResourceProvider._(
      canonical,
      index,
      state,
      ResourceSource(
        scope: scope,
        id: id,
        workspace: workspace,
        mount: id,
        label: mountLabel,
        workspaceLabel: workspaceLabel,
        mountLabel: mountLabel,
        deviceLabel: deviceLabel,
        deviceRef: deviceRef,
        local: true,
        exactVersions: false,
      ),
      generation,
    );
    await value._save();
    return value;
  }

  Map<String, Object?> get _paths =>
      Map<String, Object?>.from(_state['paths'] as Map);
  Future<void> _save() {
    final copy = Map<String, Object?>.from(_state);
    final next = _writes
        .catchError((Object _) {})
        .then((_) => index.write(copy));
    _writes = next;
    return next;
  }

  String _ref(List<String> parts) {
    final paths = _paths;
    for (final e in paths.entries) {
      if ((e.value as List).join('/') == parts.join('/')) return e.key;
    }
    if (paths.length >= 20000) throw const ResourceFailure('CATALOG_LIMIT');
    final ref = opaqueResourceId('node');
    paths[ref] = List<String>.of(parts);
    _state['paths'] = paths;
    return ref;
  }

  List<String> _parts(String ref) {
    final value = _paths[checkedRef(ref)];
    if (value is! List) throw const ResourceFailure('NOT_FOUND');
    return value.map(checkedName).toList();
  }

  void _check(String ref, ResourceAccess access, ResourceOperation operation) {
    if (!available) throw const ResourceFailure('SOURCE_UNAVAILABLE');
    access.check(
      source.identity(ref),
      operation,
      currentGeneration: generation(),
    );
  }

  Future<String> _path(List<String> parts) async {
    var path = root.path;
    for (final part in parts) {
      path = '$path/${checkedName(part)}';
      if (await FileSystemEntity.isLink(path))
        throw const ResourceFailure('UNSAFE_STORAGE');
    }
    final type = await FileSystemEntity.type(path, followLinks: false);
    if (type == FileSystemEntityType.notFound)
      throw const ResourceFailure('NOT_FOUND');
    final resolved = type == FileSystemEntityType.directory
        ? await Directory(path).resolveSymbolicLinks()
        : await File(path).resolveSymbolicLinks();
    if (resolved != root.path && !resolved.startsWith('${root.path}/'))
      throw const ResourceFailure('DENIED');
    return resolved;
  }

  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) async {
    _check(parentRef, access, ResourceOperation.describe);
    if (limit < 1 || limit > 200)
      throw const ResourceFailure('INVALID_REQUEST');
    final parts = _parts(parentRef), path = await _path(parts);
    if (await FileSystemEntity.type(path, followLinks: false) !=
        FileSystemEntityType.directory)
      throw const ResourceFailure('NOT_FOUND');
    final entries = await Directory(path)
        .list(followLinks: false)
        .where((e) => e is File || e is Directory)
        .take(10001)
        .toList();
    if (entries.length > 10000) throw const ResourceFailure('CATALOG_LIMIT');
    entries.sort((a, b) => a.path.compareTo(b.path));
    final offset = cursor == null ? 0 : int.tryParse(cursor) ?? -1;
    if (offset < 0 || offset > entries.length)
      throw const ResourceFailure('INVALID_CURSOR');
    final page = <ResourceDirectoryEntry>[];
    for (final entry in entries.skip(offset).take(limit)) {
      final name = entry.path.split(Platform.pathSeparator).last;
      try {
        checkedName(name);
      } on ResourceFailure {
        continue;
      }
      final ref = _ref([...parts, name]), file = entry is File;
      page.add(
        ResourceDirectoryEntry(
          nodeRef: ref,
          name: name,
          deferredIdentity: file ? source.identity(ref) : null,
          mediaType: file
              ? lookupMimeType(name) ?? 'application/octet-stream'
              : null,
        ),
      );
    }
    await _save();
    return ResourcePage(
      page,
      nextCursor: offset + limit < entries.length ? '${offset + limit}' : null,
    );
  }

  Future<ResourceContent> _read(
    String ref,
    ResourceAccess access,
    ResourceVersion? version, {
    ResourceOperation operation = ResourceOperation.read,
  }) async {
    _check(ref, access, operation);
    final parts = _parts(ref), path = await _path(parts);
    if (parts.isEmpty ||
        await FileSystemEntity.type(path, followLinks: false) !=
            FileSystemEntityType.file)
      throw const ResourceFailure('NOT_FOUND');
    final file = File(path);
    if (await file.length() > LocalResourceProvider.maxBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    final bytes = await file.readAsBytes();
    await _path(parts);
    if (bytes.length > LocalResourceProvider.maxBytes)
      throw const ResourceFailure('TRANSFER_TOO_LARGE');
    final digest = resourceDigest(bytes),
        actual = ResourceVersion(revision: 'sha256-$digest', digest: digest);
    if (version != null && version != actual)
      throw const ResourceFailure('VERSION_UNAVAILABLE');
    final mime =
        lookupMimeType(parts.last, headerBytes: bytes.take(64).toList()) ??
        'application/octet-stream';
    final record = ResourceRecord(
      identity: source.identity(ref),
      version: actual,
      sourceLabel: source.deviceLabel,
      location: [
        source.workspaceLabel,
        source.mountLabel,
        ...parts.take(parts.length - 1),
      ],
      parentRef: _ref(parts.take(parts.length - 1).toList()),
      readOnly: true,
      metadataRevision: 1,
      descriptor: MuseResourceDescriptorV1(
        resourceRef: ref,
        revision: actual.revision,
        displayName: parts.last,
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
        sizeBytes: bytes.length,
      ),
    );
    return ResourceContent(record, bytes);
  }

  @override
  Future<ResourceRecord> describe(
    String resourceRef,
    ResourceAccess access,
  ) async {
    _check(resourceRef, access, ResourceOperation.describe);
    // Metadata includes a digest, so describe is explicitly bounded file IO.
    return (await _read(
      resourceRef,
      access,
      null,
      operation: ResourceOperation.describe,
    )).record;
  }

  @override
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  ) => _read(resourceRef, access, version);
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
