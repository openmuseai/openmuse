/// Opaque snapshots only. Content validation, versioning, authorization and
/// conflict resolution belong to the resource/domain owner.
abstract interface class SnapshotStore {
  Future<Map<String, dynamic>?> read();
  Future<void> write(Map<String, dynamic> snapshot);
}

final class SnapshotStoreFailure implements Exception {
  const SnapshotStoreFailure(this.code);
  final String code;
  @override
  String toString() => 'SnapshotStoreFailure($code)';
}
