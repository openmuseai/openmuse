/// A point-in-time description of the original Cloud object, not a write permit.
/// Every write still goes through original server authorization and lifecycle checks.
final class AppFlowyWriteCapabilities {
  AppFlowyWriteCapabilities({
    required this.workspaceId,
    required this.objectId,
    required this.collabType,
    required this.canWrite,
    required this.qualifiedEditing,
    required this.maxFullSyncBytes,
    required this.maxLegacyRealtimeMessageBytes,
    required this.maxBatchBytes,
    required this.maxBatchObjects,
    List<String> snapshotValidationProfiles = const [],
  }) : snapshotValidationProfiles = List.unmodifiable(
         snapshotValidationProfiles,
       );

  final String workspaceId, objectId;
  final int collabType;
  final bool canWrite;

  /// Server-reported integration qualification, independent of write authority.
  /// The transport does not interpret content kinds or imply full feature parity.
  final bool qualifiedEditing;

  /// Opaque server profiles for complete snapshot ingress only. A content
  /// binding must recognize a profile before relying on its rules. This never
  /// implies incremental validation, a write grant, fencing, or a durable ACK.
  final List<String> snapshotValidationProfiles;
  final int maxFullSyncBytes, maxLegacyRealtimeMessageBytes;
  final int maxBatchBytes, maxBatchObjects;

  /// No epoch, writer fencing, or semantic extensions are enabled by this profile.
  static const profile = 'appflowy.original-collab/1';
  static const documentActorBits = 32;
}
