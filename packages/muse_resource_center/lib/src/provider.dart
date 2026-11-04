import 'model.dart';

final class ResourceSource {
  ResourceSource({
    required String scope,
    required String id,
    required String workspace,
    required String mount,
    required this.label,
    required this.workspaceLabel,
    required this.mountLabel,
    required this.deviceLabel,
    String? deviceRef,
    required this.local,
    this.canCreate = false,
    this.canRename = false,
    this.canEdit = false,
    this.exactVersions = true,
    this.internal = false,
  }) : deviceRef = checkedRef(deviceRef ?? id),
       scope = checkedRef(scope),
       id = checkedRef(id),
       workspace = checkedRef(workspace),
       mount = checkedRef(mount);
  final String scope,
      id,
      workspace,
      mount,
      label,
      workspaceLabel,
      mountLabel,
      deviceLabel;
  final String deviceRef;
  final bool local, canCreate, canRename, canEdit, exactVersions;

  /// An addressable Host-owned replica volume, excluded from browse/create UI.
  final bool internal;
  ResourceIdentity identity(String ref) => ResourceIdentity(
    scope: scope,
    source: id,
    workspace: workspace,
    mount: mount,
    resource: ref,
  );
  String get key => identity('root').sourceKey;
  Map<String, Object?> toJson() => {
    'scope': scope,
    'id': id,
    'workspace': workspace,
    'mount': mount,
    'label': label,
    'workspaceLabel': workspaceLabel,
    'mountLabel': mountLabel,
    'deviceLabel': deviceLabel,
    'deviceRef': deviceRef,
    'local': local,
    'canCreate': canCreate,
    'canRename': canRename,
    'canEdit': canEdit,
    'exactVersions': exactVersions,
    'internal': internal,
  };
  factory ResourceSource.fromJson(Map<String, Object?> m) => ResourceSource(
    scope: checkedRef(m['scope']),
    id: checkedRef(m['id']),
    workspace: checkedRef(m['workspace']),
    mount: checkedRef(m['mount']),
    label: m['label'] as String,
    workspaceLabel: m['workspaceLabel'] as String,
    mountLabel: m['mountLabel'] as String,
    deviceLabel: m['deviceLabel'] as String,
    deviceRef: m['deviceRef'] as String?,
    local: m['local'] == true,
    canCreate: m['canCreate'] == true,
    canRename: m['canRename'] == true,
    canEdit: m['canEdit'] == true,
    exactVersions: m['exactVersions'] == true,
    internal: m['internal'] == true,
  );
}

/// Provider methods receive Host-owned access. Every remote adapter must enforce
/// the same authorization again at the source; local checks never replace it.
abstract interface class ResourceProvider {
  ResourceSource get source;
  bool get available;
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  });
  Future<ResourceRecord> describe(String resourceRef, ResourceAccess access);
  Future<ResourceContent> read(
    String resourceRef,
    ResourceVersion? version,
    ResourceAccess access,
  );
  Future<ResourceRecord> create({
    required String parentRef,
    required String name,
    required String mediaType,
    required List<int> bytes,
    required String idempotencyKey,
    required ResourceAccess access,
    ResourceReference? provenance,
  });
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  );
  Future<ResourceRecord> rename(
    String resourceRef,
    String name,
    int expectedMetadataRevision,
    ResourceAccess access,
  );
  Future<ResourceRecord> commit(
    String resourceRef,
    ResourceVersion base,
    List<int> bytes,
    String idempotencyKey,
    ResourceAccess access,
  );
}

enum ResourceOpenIntent { view, edit }

final class ResourceHandlerSpec {
  const ResourceHandlerSpec({
    required this.id,
    required this.title,
    required this.mediaTypes,
    this.extensions = const {},
    this.editable = false,
    this.priority = 0,
  });
  final String id, title;
  final Set<String> mediaTypes, extensions;
  final bool editable;
  final int priority;
  bool accepts(ResourceRecord record) =>
      mediaTypes.any(
        (m) =>
            m == record.mediaType ||
            (m.endsWith('/*') &&
                record.mediaType.startsWith(m.substring(0, m.length - 1))),
      ) ||
      (record.mediaType == 'application/octet-stream' &&
          extensions.contains(record.name.split('.').last.toLowerCase()));
}

final class ResourceCreatorSpec {
  const ResourceCreatorSpec({
    required this.id,
    required this.title,
    required this.mediaType,
    required this.extension,
    required this.defaultName,
    this.localOnly = false,
    this.inputSchema = const {},
  });
  final String id, title, mediaType, extension, defaultName;
  final bool localOnly;
  final Map<String, Object?> inputSchema;
}

final class ResourceSeed {
  const ResourceSeed({required this.bytes, required this.mediaType});
  final List<int> bytes;
  final String mediaType;
}

abstract interface class ResourceStateStore {
  Future<Map<String, Object?>> read();
  Future<void> write(Map<String, Object?> state);
}

final class MemoryResourceStateStore implements ResourceStateStore {
  Map<String, Object?> _value = {};
  @override
  Future<Map<String, Object?>> read() async => Map.of(_value);
  @override
  Future<void> write(Map<String, Object?> state) async {
    _value = Map.of(state);
  }
}
