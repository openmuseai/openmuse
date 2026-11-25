/// Point-in-time persisted model metadata, never a writer lease or write grant.
/// Model profiles remain opaque to this transport; a domain binding must know
/// their semantics and negotiate admission before applying content operations.
final class AppFlowyModelContract {
  const AppFlowyModelContract({
    required this.workspaceId,
    required this.objectId,
    required this.collabType,
    required this.modelProfile,
    required this.epoch,
    required this.binding,
    required this.originalPipelineAvailable,
  });
  final String workspaceId, objectId, modelProfile, binding;
  final int collabType;

  /// Canonical positive decimal signed-64 value, retained as a string on Web.
  /// Null identifies the implicit original model, not a fenced epoch zero.
  final String? epoch;
  final bool originalPipelineAvailable;
  static const protocol = 'openmuse.collab-model-contract/1';
}
