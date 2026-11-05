enum MuseWorkspaceEntryKind { directory, file, symlink, unknown }

enum MuseWorkspaceCapability {
  metadataRead,
  childrenList,
  contentRead,
  contentWrite,
  createFile,
  createDirectory,
  rename,
  delete,
  import,
  nativeReveal,
  changesWatch,
}

final class MuseWorkspaceMount {
  const MuseWorkspaceMount({
    required this.mountRef,
    required this.providerId,
    required this.bindingKey,
    required this.displayName,
    required this.rootLocator,
    required this.readOnly,
    required this.order,
  });

  factory MuseWorkspaceMount.fromJson(Map<String, dynamic> json) =>
      MuseWorkspaceMount(
        mountRef: json['mountRef'] as String,
        providerId: json['providerId'] as String,
        bindingKey: json['bindingKey'] as String,
        displayName: json['displayName'] as String,
        rootLocator: json['rootLocator'] as String,
        readOnly: json['readOnly'] as bool? ?? false,
        order: json['order'] as int? ?? 0,
      );

  final String mountRef;
  final String providerId;
  final String bindingKey;
  final String displayName;
  final String rootLocator;
  final bool readOnly;
  final int order;

  Map<String, Object?> toJson() => {
        'mountRef': mountRef,
        'providerId': providerId,
        'bindingKey': bindingKey,
        'displayName': displayName,
        'rootLocator': rootLocator,
        'readOnly': readOnly,
        'order': order,
      };
}

final class MuseWorkspaceEntry {
  const MuseWorkspaceEntry({
    required this.entryRef,
    required this.resourceRef,
    required this.mountRef,
    required this.name,
    required this.kind,
    required this.locator,
    required this.capabilities,
    this.parentEntryRef,
    this.size,
    this.modifiedAt,
  });

  final String entryRef;
  final String resourceRef;
  final String mountRef;
  final String? parentEntryRef;
  final String name;
  final MuseWorkspaceEntryKind kind;
  final String locator;
  final Set<MuseWorkspaceCapability> capabilities;
  final int? size;
  final DateTime? modifiedAt;

  bool get isDirectory => kind == MuseWorkspaceEntryKind.directory;
  bool get isFile => kind == MuseWorkspaceEntryKind.file;
}

final class MuseWorkspaceSnapshot {
  const MuseWorkspaceSnapshot({
    required this.accountSpaceRef,
    required this.mounts,
    required this.expandedEntryRefs,
    this.activeMountRef,
  });

  final String accountSpaceRef;
  final List<MuseWorkspaceMount> mounts;
  final Set<String> expandedEntryRefs;

  /// Mount the next DSH session should start in; null means the lowest order.
  final String? activeMountRef;
}
