import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

final class RemoteConnectionContext {
  const RemoteConnectionContext({
    required this.actorRef,
    required this.mobileDeviceRef,
    required this.desktopDeviceRef,
    required this.workspaceRef,
    required this.permissions,
  });

  const RemoteConnectionContext.unbound()
    : actorRef = 'unbound',
      mobileDeviceRef = 'unbound',
      desktopDeviceRef = 'unbound',
      workspaceRef = 'unbound',
      permissions = const {};

  final String actorRef;
  final String mobileDeviceRef;
  final String desktopDeviceRef;
  final String workspaceRef;
  final Set<String> permissions;
}

final class RemoteProviderRequest {
  const RemoteProviderRequest({
    required this.surfaceId,
    required this.actionId,
    required this.effect,
    required this.input,
    required this.stateRevision,
  });

  final String surfaceId;
  final String actionId;
  final String effect;
  final Map<String, Object?> input;
  final String stateRevision;
}

sealed class RemoteProviderResult {
  const RemoteProviderResult();
}

final class RemoteProviderUpdate extends RemoteProviderResult {
  const RemoteProviderUpdate({
    required this.stateRevision,
    required this.nodes,
    this.jobRef,
    this.eventState,
  });

  final String stateRevision;
  final List<RemoteSurfaceNode> nodes;
  final String? jobRef;
  final String? eventState;
}

final class RemoteProviderReject extends RemoteProviderResult {
  const RemoteProviderReject(this.errorCode);

  final String errorCode;
}

abstract interface class RemoteSurfaceProvider {
  String get pluginId;

  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context);

  String initialRevision(String surfaceId);

  List<RemoteSurfaceNode> initialNodes(String surfaceId);

  RemoteProviderResult execute(RemoteProviderRequest request);
}
