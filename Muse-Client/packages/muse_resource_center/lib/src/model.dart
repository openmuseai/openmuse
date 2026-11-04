import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'package:muse_resource_contract/muse_resource_contract.dart';

export 'package:muse_resource_contract/muse_resource_contract.dart'
    show
        MuseResourceDescriptorV1,
        MuseResourceFormat,
        MuseFormatConfidence,
        MuseResourceSecurity,
        MuseSecurityClassification,
        MuseActiveContent,
        MuseResourceCapability;

final class ResourceFailure implements Exception {
  const ResourceFailure(this.code);
  final String code;
  @override
  String toString() => 'ResourceFailure($code)';
}

String resourceDigest(List<int> bytes) => sha256.convert(bytes).toString();

String checkedRef(Object? value) {
  if (value is! String ||
      value.isEmpty ||
      value.length > 256 ||
      value.codeUnits.any((c) => c <= 32 || c == 47 || c == 92) ||
      RegExp(r'^[a-zA-Z]:').hasMatch(value)) {
    throw const ResourceFailure('INVALID_REFERENCE');
  }
  return value;
}

Map<String, Object?> resourceObject(Object? value, Set<String> keys) {
  if (value is! Map ||
      value.keys.any((k) => k is! String || !keys.contains(k))) {
    throw const ResourceFailure('INVALID_ENVELOPE');
  }
  return Map<String, Object?>.from(value);
}

String checkedName(Object? value) {
  if (value is! String ||
      value.trim().isEmpty ||
      value.length > 240 ||
      value == '.' ||
      value == '..' ||
      value.codeUnits.any((c) => c < 32 || c == 47 || c == 92)) {
    throw const ResourceFailure('INVALID_NAME');
  }
  return value.trim();
}

/// Provider-scoped opaque identity; never a transport address or file path.
final class ResourceIdentity {
  ResourceIdentity({
    required String scope,
    required String source,
    required String workspace,
    required String mount,
    required String resource,
  }) : scope = checkedRef(scope),
       source = checkedRef(source),
       workspace = checkedRef(workspace),
       mount = checkedRef(mount),
       resource = checkedRef(resource);
  final String scope, source, workspace, mount, resource;
  String get sourceKey => jsonEncode([scope, source, workspace, mount]);
  String get key => jsonEncode([scope, source, workspace, mount, resource]);
  Map<String, Object?> toJson() => {
    'scope': scope,
    'source': source,
    'workspace': workspace,
    'mount': mount,
    'resource': resource,
  };
  factory ResourceIdentity.fromJson(Object? value) {
    final m = resourceObject(value, {
      'scope',
      'source',
      'workspace',
      'mount',
      'resource',
    });
    return ResourceIdentity(
      scope: checkedRef(m['scope']),
      source: checkedRef(m['source']),
      workspace: checkedRef(m['workspace']),
      mount: checkedRef(m['mount']),
      resource: checkedRef(m['resource']),
    );
  }
  @override
  bool operator ==(Object other) =>
      other is ResourceIdentity && other.key == key;
  @override
  int get hashCode => key.hashCode;
}

final class ResourceVersion {
  ResourceVersion({required String revision, required String digest})
    : revision = checkedRef(revision),
      digest = _digest(digest);
  final String revision, digest;
  static String _digest(String value) {
    if (!RegExp(r'^[a-f0-9]{64}$').hasMatch(value)) {
      throw const ResourceFailure('INVALID_DIGEST');
    }
    return value;
  }

  Map<String, Object?> toJson() => {'revision': revision, 'digest': digest};
  factory ResourceVersion.fromJson(Object? value) {
    final m = resourceObject(value, {'revision', 'digest'});
    return ResourceVersion(
      revision: checkedRef(m['revision']),
      digest: m['digest'] as String? ?? '',
    );
  }
  @override
  bool operator ==(Object other) =>
      other is ResourceVersion &&
      other.revision == revision &&
      other.digest == digest;
  @override
  int get hashCode => Object.hash(revision, digest);
}

final class RetainedResource {
  const RetainedResource(this.identity, this.version);
  final ResourceIdentity identity;
  final ResourceVersion version;
  Map<String, Object?> toJson() => {
    'identity': identity.toJson(),
    'version': version.toJson(),
  };
  factory RetainedResource.fromJson(Object? value) {
    final m = resourceObject(value, {'identity', 'version'});
    return RetainedResource(
      ResourceIdentity.fromJson(m['identity']),
      ResourceVersion.fromJson(m['version']),
    );
  }
}

/// Live references omit version; conversation references pin revision + digest.
/// Retained snapshots are separate identities, with explicit provenance.
final class ResourceReference {
  ResourceReference({required this.identity, this.version, this.retained}) {
    if (retained != null &&
        (version == null ||
            retained!.identity.scope != identity.scope ||
            retained!.version.digest != version!.digest)) {
      throw const ResourceFailure('INVALID_SNAPSHOT_BINDING');
    }
  }
  final ResourceIdentity identity;
  final ResourceVersion? version;
  final RetainedResource? retained;
  bool get pinned => version != null;
  String get key => '${identity.key}:${version?.revision ?? 'latest'}';
  Map<String, Object?> toJson() => {
    'protocol': 'openmuse.resource-reference/v1',
    'identity': identity.toJson(),
    if (version != null) 'version': version!.toJson(),
    if (retained != null) 'retained': retained!.toJson(),
  };
  factory ResourceReference.fromJson(Object? value) {
    final m = resourceObject(value, {
      'protocol',
      'identity',
      'version',
      'retained',
    });
    if (m['protocol'] != 'openmuse.resource-reference/v1') {
      throw const ResourceFailure('UNSUPPORTED_PROTOCOL');
    }
    return ResourceReference(
      identity: ResourceIdentity.fromJson(m['identity']),
      version: m['version'] == null
          ? null
          : ResourceVersion.fromJson(m['version']),
      retained: m['retained'] == null
          ? null
          : RetainedResource.fromJson(m['retained']),
    );
  }
  Uri toUri() => Uri(
    scheme: 'openmuse-resource',
    host: 'v1',
    path:
        '/${base64Url.encode(utf8.encode(jsonEncode(toJson()))).replaceAll('=', '')}',
  );
  factory ResourceReference.fromUri(String value) {
    if (value.length > 8192) throw const ResourceFailure('REFERENCE_TOO_LARGE');
    try {
      final u = Uri.parse(value);
      if (u.scheme != 'openmuse-resource' ||
          u.host != 'v1' ||
          u.hasQuery ||
          u.hasFragment ||
          u.userInfo.isNotEmpty ||
          u.hasPort ||
          u.pathSegments.length != 1) {
        throw const ResourceFailure('INVALID_REFERENCE');
      }
      final bytes = base64Url.decode(
        base64Url.normalize(u.pathSegments.single),
      );
      return ResourceReference.fromJson(jsonDecode(utf8.decode(bytes)));
    } on ResourceFailure {
      rethrow;
    } on Object {
      throw const ResourceFailure('INVALID_REFERENCE');
    }
  }
}

enum ResourceOperation { describe, read, create, rename, edit, retain }

/// Constructed by the trusted Host, never accepted as client-supplied authority.
final class ResourceAccess {
  const ResourceAccess({
    required this.scope,
    required this.audience,
    required this.generation,
    required this.expiresAtMs,
    required this.operations,
    this.resourceKeys,
    this.allowOfflineReplica = false,
  });
  final String scope, audience;
  final int generation, expiresAtMs;
  final Set<ResourceOperation> operations;
  final Set<String>? resourceKeys;
  final bool allowOfflineReplica;
  void check(
    ResourceIdentity identity,
    ResourceOperation op, {
    required int currentGeneration,
    int? nowMs,
  }) {
    if (scope != identity.scope ||
        audience.isEmpty ||
        !operations.contains(op) ||
        (resourceKeys != null && !resourceKeys!.contains(identity.key))) {
      throw const ResourceFailure('DENIED');
    }
    if (generation != currentGeneration)
      throw const ResourceFailure('STALE_GENERATION');
    if ((nowMs ?? DateTime.now().millisecondsSinceEpoch) >= expiresAtMs) {
      throw const ResourceFailure('EXPIRED');
    }
  }
}

final class ResourceRecord {
  ResourceRecord({
    required this.identity,
    required this.descriptor,
    required this.version,
    required this.sourceLabel,
    required List<String> location,
    required this.parentRef,
    required this.metadataRevision,
    this.readOnly = false,
    this.provenance,
  }) : location = List.unmodifiable(location) {
    if (identity.resource != descriptor.resourceRef ||
        version.revision != descriptor.revision) {
      throw const ResourceFailure('RESOURCE_MISMATCH');
    }
  }
  final ResourceIdentity identity;
  final MuseResourceDescriptorV1 descriptor;
  final ResourceVersion version;
  final String sourceLabel, parentRef;
  final List<String> location;
  final int metadataRevision;
  final bool readOnly;
  final ResourceReference? provenance;
  String get name => descriptor.displayName;
  String get mediaType => descriptor.mediaType;
  ResourceReference reference({bool pin = false}) =>
      ResourceReference(identity: identity, version: pin ? version : null);
  Map<String, Object?> toJson() => {
    'identity': identity.toJson(),
    'descriptor': descriptor.toJson(),
    'version': version.toJson(),
    'sourceLabel': sourceLabel,
    'location': location,
    'parentRef': parentRef,
    'metadataRevision': metadataRevision,
    'readOnly': readOnly,
    if (provenance != null) 'provenance': provenance!.toJson(),
  };
  factory ResourceRecord.fromJson(Object? value) {
    final m = resourceObject(value, {
      'identity',
      'descriptor',
      'version',
      'sourceLabel',
      'location',
      'parentRef',
      'metadataRevision',
      'readOnly',
      'provenance',
    });
    final locations = m['location'];
    if (locations is! List ||
        locations.length > 32 ||
        locations.any((x) => x is! String)) {
      throw const ResourceFailure('INVALID_LOCATION');
    }
    return ResourceRecord(
      identity: ResourceIdentity.fromJson(m['identity']),
      descriptor: MuseResourceDescriptorV1.fromJson(
        Map<String, Object?>.from(m['descriptor'] as Map),
      ),
      version: ResourceVersion.fromJson(m['version']),
      sourceLabel: m['sourceLabel'] as String,
      location: locations.cast<String>(),
      parentRef: checkedRef(m['parentRef']),
      metadataRevision: m['metadataRevision'] as int,
      readOnly: m['readOnly'] == true,
      provenance: m['provenance'] == null
          ? null
          : ResourceReference.fromJson(m['provenance']),
    );
  }
}

final class ResourceDirectoryEntry {
  const ResourceDirectoryEntry({
    required this.nodeRef,
    required this.name,
    this.resource,
    this.deferredIdentity,
    this.mediaType,
  });
  final String nodeRef, name;
  final ResourceRecord? resource;

  /// A directory listing need not read/hash every file just to render a row.
  /// This identity must be described before opening or copying a version.
  final ResourceIdentity? deferredIdentity;
  final String? mediaType;
  bool get isDirectory => resource == null && deferredIdentity == null;
  Map<String, Object?> toJson() => {
    'nodeRef': nodeRef,
    'name': name,
    if (resource != null) 'resource': resource!.toJson(),
    if (deferredIdentity != null) 'identity': deferredIdentity!.toJson(),
    if (mediaType != null) 'mediaType': mediaType,
  };
  factory ResourceDirectoryEntry.fromJson(Map<String, Object?> m) =>
      ResourceDirectoryEntry(
        nodeRef: checkedRef(m['nodeRef']),
        name: checkedName(m['name']),
        deferredIdentity: m['identity'] == null
            ? null
            : ResourceIdentity.fromJson(m['identity']),
        mediaType: m['mediaType'] as String?,
        resource: m['resource'] == null
            ? null
            : ResourceRecord.fromJson(m['resource']),
      );
}

final class ResourcePage {
  const ResourcePage(this.entries, {this.nextCursor});
  final List<ResourceDirectoryEntry> entries;
  final String? nextCursor;
}

final class ResourceContent {
  ResourceContent(this.record, List<int> bytes)
    : bytes = List.unmodifiable(bytes) {
    if (resourceDigest(bytes) != record.version.digest)
      throw const ResourceFailure('DIGEST_MISMATCH');
  }
  final ResourceRecord record;
  final List<int> bytes;
}

final class OpenedResource {
  const OpenedResource({
    required this.reference,
    required this.content,
    this.fromSnapshot = false,
    this.sourceOffline = false,
  });
  final ResourceReference reference;
  final ResourceContent content;
  final bool fromSnapshot, sourceOffline;
}
