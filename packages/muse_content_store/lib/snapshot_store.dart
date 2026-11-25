/// Opaque snapshots only. Content validation, versioning, authorization and
/// conflict resolution belong to the resource/domain owner.
abstract interface class SnapshotStore {
  Future<Map<String, dynamic>?> read();
  Future<void> write(Map<String, dynamic> snapshot);
}

/// Optional retirement of an opaque record. Domain owners serialize removal
/// against their own writes; unrelated records and namespaces are untouched.
abstract interface class RemovableSnapshotStore implements SnapshotStore {
  Future<void> remove();
}

final class SnapshotStoreFailure implements Exception {
  const SnapshotStoreFailure(this.code);
  final String code;
  @override
  String toString() => 'SnapshotStoreFailure($code)';
}
