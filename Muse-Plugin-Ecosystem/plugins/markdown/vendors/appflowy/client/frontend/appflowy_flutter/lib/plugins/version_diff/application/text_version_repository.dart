import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

typedef MuseVersionRootResolver = Future<Directory> Function();

/// Small, provider-neutral local backend used by the text/code vertical slice.
/// Content is immutable and content-addressed; version metadata and audit events
/// are append-only. A remote/SSH/cloud backend can implement the same contract.
final class MuseTextVersionRepository implements MuseContentResolver {
  MuseTextVersionRepository({MuseVersionRootResolver? rootResolver})
      : _rootResolver = rootResolver ?? _defaultRoot;

  static const repository = MuseRepositoryRef(
    id: 'host.local-version-store',
    providerId: 'muse.version-store.local.v1',
  );

  static const defaultActor = MuseActorRef(
    id: 'host.current-user',
    displayName: 'Current user',
  );

  final MuseVersionRootResolver _rootResolver;

  static Future<Directory> _defaultRoot() async {
    final support = await getApplicationSupportDirectory();
    return Directory(p.join(support.path, 'OpenMuse', 'version-diff-v1'));
  }

  Future<MuseResourceRef> resourceFor(File file) async {
    final canonical = file.absolute.path;
    final id = 'resource:${sha256.convert(utf8.encode(canonical))}';
    return MuseResourceRef(
      id: id,
      repository: repository,
      locator: canonical,
      mediaType: _mediaType(canonical),
      displayName: p.basename(canonical),
    );
  }

  Future<MuseVersion> capture(
    File file, {
    required MuseVersionKind kind,
    MuseActorRef actor = defaultActor,
    String? message,
    bool persistMetadata = true,
  }) async {
    final bytes = await file.readAsBytes();
    final resource = await resourceFor(file);
    final digest = sha256.convert(bytes).toString();
    final createdAt = DateTime.now().toUtc();
    final committed = await listVersions(file);
    final parent = committed.isEmpty ? null : committed.last.ref;
    final idSource = '${resource.id}:$digest:${kind.name}:'
        '${createdAt.microsecondsSinceEpoch}:${parent?.id ?? ''}';
    final version = MuseVersion(
      ref: MuseVersionRef('version:${sha256.convert(utf8.encode(idSource))}'),
      resource: resource,
      kind: kind,
      contentRef: 'sha256:$digest',
      contentDigest: digest,
      byteLength: bytes.length,
      createdAt: createdAt,
      actor: actor,
      parents: parent == null ? const [] : [parent],
      message: message,
    );
    await _writeBlob(digest, bytes);
    if (persistMetadata) {
      await _writeVersion(version);
      await appendAudit(
        MuseAuditEvent(
          id: _auditId('version.capture', version.ref.id, createdAt),
          type: 'version.capture',
          repositoryId: repository.id,
          resourceId: resource.id,
          actor: actor,
          occurredAt: createdAt,
          subjectId: version.ref.id,
          metadata: {
            'kind': kind.name,
            'digest': digest,
            if (message != null) 'message': message,
          },
        ),
      );
    }
    return version;
  }

  Future<List<MuseVersion>> listVersions(File file) async {
    final resource = await resourceFor(file);
    final root = await _root();
    final directory =
        Directory(p.join(root.path, 'versions', _safe(resource.id)));
    if (!await directory.exists()) return const [];
    final versions = <MuseVersion>[];
    await for (final entity in directory.list()) {
      if (entity is! File || p.extension(entity.path) != '.json') continue;
      try {
        final map =
            jsonDecode(await entity.readAsString()) as Map<String, dynamic>;
        versions.add(_versionFromJson(map));
      } on Object {
        // Ignore an incomplete/corrupt sidecar; immutable blobs remain intact.
      }
    }
    versions.sort((a, b) => a.createdAt.compareTo(b.createdAt));
    return versions;
  }

  Future<List<MuseVersion>> listCommitted(File file) async {
    return (await listVersions(file))
        .where((version) => version.kind == MuseVersionKind.committed)
        .toList(growable: false);
  }

  Future<MuseVersion?> latestCommitted(File file) async {
    final versions = await listCommitted(file);
    return versions.isEmpty ? null : versions.last;
  }

  Future<bool> hasUncommittedChanges(File file) async {
    final latest = await latestCommitted(file);
    if (latest == null) return true;
    final digest = sha256.convert(await file.readAsBytes()).toString();
    return digest != latest.contentDigest;
  }

  Future<MuseVersion?> captureIfDirty(
    File file, {
    String message = '自动保存',
    MuseActorRef actor = defaultActor,
  }) async {
    if (!await hasUncommittedChanges(file)) return null;
    return capture(
      file,
      kind: MuseVersionKind.committed,
      message: message,
      actor: actor,
    );
  }

  Future<MuseVersion?> versionById(File file, String id) async {
    final versions = await listVersions(file);
    for (final version in versions) {
      if (version.ref.id == id) return version;
    }
    return null;
  }

  @override
  Future<Uint8List> resolve(MuseVersion version) async {
    final root = await _root();
    final blob = File(p.join(root.path, 'blobs', version.contentDigest));
    if (!await blob.exists()) {
      throw StateError('Missing immutable blob ${version.contentDigest}');
    }
    return blob.readAsBytes();
  }

  Future<void> recordComparison(
    MuseComparison comparison, {
    required MuseVersion base,
    required MuseVersion target,
    Map<String, Object?> changeMetadata = const {},
  }) async {
    // Keep both immutable sides so a Comparison deep link can be reopened
    // after a process restart. This internal working snapshot does not emit a
    // separate version.capture event.
    await _writeVersion(base);
    await _writeVersion(target);
    await _writeComparison(comparison);
    await appendAudit(
      MuseAuditEvent(
        id: _auditId('comparison.create', comparison.id, comparison.createdAt),
        type: 'comparison.create',
        repositoryId: comparison.resource.repository.id,
        resourceId: comparison.resource.id,
        actor: comparison.actor,
        occurredAt: comparison.createdAt,
        subjectId: comparison.id,
        metadata: {
          'base': comparison.base.id,
          'target': comparison.target.id,
          'rendererType': comparison.rendererType,
          ...changeMetadata,
        },
      ),
    );
  }

  Future<MuseComparison?> comparisonById(File file, String id) async {
    final resource = await resourceFor(file);
    final root = await _root();
    final comparisonFile = File(
      p.join(
        root.path,
        'comparisons',
        _safe(resource.id),
        '${_safe(id)}.json',
      ),
    );
    if (!await comparisonFile.exists()) return null;
    try {
      return _comparisonFromJson(
        jsonDecode(await comparisonFile.readAsString()) as Map<String, dynamic>,
      );
    } on Object {
      return null;
    }
  }

  Future<void> appendAudit(MuseAuditEvent event) async {
    final root = await _root();
    final directory = Directory(p.join(root.path, 'audit'));
    await directory.create(recursive: true);
    final file =
        File(p.join(directory.path, '${_safe(event.resourceId)}.jsonl'));
    await file.writeAsString(
      '${jsonEncode(_auditToJson(event))}\n',
      mode: FileMode.append,
      flush: true,
    );
  }

  Future<List<MuseAuditEvent>> listAudit(File file) async {
    final resource = await resourceFor(file);
    final root = await _root();
    final auditFile = File(
      p.join(root.path, 'audit', '${_safe(resource.id)}.jsonl'),
    );
    if (!await auditFile.exists()) return const [];
    final events = <MuseAuditEvent>[];
    for (final line in await auditFile.readAsLines()) {
      if (line.trim().isEmpty) continue;
      try {
        events.add(
          _auditFromJson(jsonDecode(line) as Map<String, dynamic>),
        );
      } on Object {
        // Audit is append-only. A malformed tail does not hide older events.
      }
    }
    events.sort((a, b) => b.occurredAt.compareTo(a.occurredAt));
    return events;
  }

  Future<Directory> _root() async {
    final root = await _rootResolver();
    await root.create(recursive: true);
    return root;
  }

  Future<void> _writeBlob(String digest, List<int> bytes) async {
    final root = await _root();
    final directory = Directory(p.join(root.path, 'blobs'));
    await directory.create(recursive: true);
    final target = File(p.join(directory.path, digest));
    if (await target.exists()) return;
    final temporary =
        File('${target.path}.tmp-${DateTime.now().microsecondsSinceEpoch}');
    await temporary.writeAsBytes(bytes, flush: true);
    if (await target.exists()) {
      await temporary.delete();
    } else {
      await temporary.rename(target.path);
    }
  }

  Future<void> _writeVersion(MuseVersion version) async {
    final root = await _root();
    final directory = Directory(
      p.join(root.path, 'versions', _safe(version.resource.id)),
    );
    await directory.create(recursive: true);
    final target =
        File(p.join(directory.path, '${_safe(version.ref.id)}.json'));
    final temporary = File('${target.path}.tmp');
    await temporary.writeAsString(
      const JsonEncoder.withIndent('  ').convert(_versionToJson(version)),
      flush: true,
    );
    if (await target.exists()) await target.delete();
    await temporary.rename(target.path);
  }

  Future<void> _writeComparison(MuseComparison comparison) async {
    final root = await _root();
    final directory = Directory(
      p.join(root.path, 'comparisons', _safe(comparison.resource.id)),
    );
    await directory.create(recursive: true);
    final target = File(
      p.join(directory.path, '${_safe(comparison.id)}.json'),
    );
    final temporary = File('${target.path}.tmp');
    await temporary.writeAsString(
      const JsonEncoder.withIndent('  ').convert({
        'id': comparison.id,
        'resource': {
          'id': comparison.resource.id,
          'repositoryId': comparison.resource.repository.id,
          'providerId': comparison.resource.repository.providerId,
          'locator': comparison.resource.locator,
          'mediaType': comparison.resource.mediaType,
          'displayName': comparison.resource.displayName,
        },
        'base': comparison.base.id,
        'target': comparison.target.id,
        'createdAt': comparison.createdAt.toIso8601String(),
        'actor': {
          'id': comparison.actor.id,
          'displayName': comparison.actor.displayName,
          'kind': comparison.actor.kind.name,
        },
        'rendererType': comparison.rendererType,
        'proposalId': comparison.proposalId,
      }),
      flush: true,
    );
    if (await target.exists()) await target.delete();
    await temporary.rename(target.path);
  }

  String _safe(String value) =>
      value.replaceAll(RegExp('[^a-zA-Z0-9._-]'), '_');

  String _auditId(String type, String subject, DateTime time) =>
      'audit:${sha256.convert(utf8.encode('$type:$subject:${time.microsecondsSinceEpoch}'))}';

  String _mediaType(String path) {
    final extension = p.extension(path).toLowerCase();
    return switch (extension) {
      '.md' || '.markdown' => 'text/markdown',
      '.html' || '.htm' => 'text/html',
      '.json' => 'application/json',
      '.xml' => 'application/xml',
      '.yaml' || '.yml' => 'application/yaml',
      '.png' => 'image/png',
      '.jpg' || '.jpeg' => 'image/jpeg',
      '.gif' => 'image/gif',
      '.webp' => 'image/webp',
      _ => 'text/plain',
    };
  }

  Map<String, Object?> _versionToJson(MuseVersion version) => {
        'id': version.ref.id,
        'resource': {
          'id': version.resource.id,
          'repositoryId': version.resource.repository.id,
          'providerId': version.resource.repository.providerId,
          'locator': version.resource.locator,
          'mediaType': version.resource.mediaType,
          'displayName': version.resource.displayName,
        },
        'kind': version.kind.name,
        'contentRef': version.contentRef,
        'contentDigest': version.contentDigest,
        'byteLength': version.byteLength,
        'createdAt': version.createdAt.toIso8601String(),
        'actor': {
          'id': version.actor.id,
          'displayName': version.actor.displayName,
          'kind': version.actor.kind.name,
        },
        'parents': version.parents.map((parent) => parent.id).toList(),
        'message': version.message,
      };

  MuseVersion _versionFromJson(Map<String, dynamic> json) {
    final resourceJson = json['resource'] as Map<String, dynamic>;
    final actorJson = json['actor'] as Map<String, dynamic>;
    return MuseVersion(
      ref: MuseVersionRef(json['id'] as String),
      resource: MuseResourceRef(
        id: resourceJson['id'] as String,
        repository: MuseRepositoryRef(
          id: resourceJson['repositoryId'] as String,
          providerId: resourceJson['providerId'] as String,
        ),
        locator: resourceJson['locator'] as String,
        mediaType: resourceJson['mediaType'] as String,
        displayName: resourceJson['displayName'] as String,
      ),
      kind: MuseVersionKind.values.byName(json['kind'] as String),
      contentRef: json['contentRef'] as String,
      contentDigest: json['contentDigest'] as String,
      byteLength: json['byteLength'] as int,
      createdAt: DateTime.parse(json['createdAt'] as String),
      actor: MuseActorRef(
        id: actorJson['id'] as String,
        displayName: actorJson['displayName'] as String,
        kind: MuseActorKind.values.byName(
          actorJson['kind'] as String? ?? MuseActorKind.user.name,
        ),
      ),
      parents: (json['parents'] as List<dynamic>)
          .cast<String>()
          .map(MuseVersionRef.new)
          .toList(growable: false),
      message: json['message'] as String?,
    );
  }

  MuseComparison _comparisonFromJson(Map<String, dynamic> json) {
    final resourceJson = json['resource'] as Map<String, dynamic>;
    final actorJson = json['actor'] as Map<String, dynamic>;
    return MuseComparison(
      id: json['id'] as String,
      resource: MuseResourceRef(
        id: resourceJson['id'] as String,
        repository: MuseRepositoryRef(
          id: resourceJson['repositoryId'] as String,
          providerId: resourceJson['providerId'] as String,
        ),
        locator: resourceJson['locator'] as String,
        mediaType: resourceJson['mediaType'] as String,
        displayName: resourceJson['displayName'] as String,
      ),
      base: MuseVersionRef(json['base'] as String),
      target: MuseVersionRef(json['target'] as String),
      createdAt: DateTime.parse(json['createdAt'] as String),
      actor: MuseActorRef(
        id: actorJson['id'] as String,
        displayName: actorJson['displayName'] as String,
        kind: MuseActorKind.values.byName(
          actorJson['kind'] as String? ?? MuseActorKind.user.name,
        ),
      ),
      rendererType: json['rendererType'] as String,
      proposalId: json['proposalId'] as String?,
    );
  }

  Map<String, Object?> _auditToJson(MuseAuditEvent event) => {
        'id': event.id,
        'type': event.type,
        'repositoryId': event.repositoryId,
        'resourceId': event.resourceId,
        'actor': {
          'id': event.actor.id,
          'displayName': event.actor.displayName,
          'kind': event.actor.kind.name,
        },
        'occurredAt': event.occurredAt.toIso8601String(),
        'subjectId': event.subjectId,
        'metadata': event.metadata,
      };

  MuseAuditEvent _auditFromJson(Map<String, dynamic> json) {
    final actor = json['actor'] as Map<String, dynamic>;
    return MuseAuditEvent(
      id: json['id'] as String,
      type: json['type'] as String,
      repositoryId: json['repositoryId'] as String,
      resourceId: json['resourceId'] as String,
      actor: MuseActorRef(
        id: actor['id'] as String,
        displayName: actor['displayName'] as String,
        kind: MuseActorKind.values.byName(
          actor['kind'] as String? ?? MuseActorKind.user.name,
        ),
      ),
      occurredAt: DateTime.parse(json['occurredAt'] as String),
      subjectId: json['subjectId'] as String?,
      metadata:
          (json['metadata'] as Map<String, dynamic>).cast<String, Object?>(),
    );
  }
}
