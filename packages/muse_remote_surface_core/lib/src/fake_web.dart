import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'provider.dart';

final class FakeWebSnapshotProvider implements RemoteSurfaceProvider {
  FakeWebSnapshotProvider({this.workspaceRef = 'ws.opaque.1'});

  final String workspaceRef;

  @override
  String get pluginId => 'com.openmuse.fake-web-snapshot';

  @override
  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context) =>
      [_descriptor];

  @override
  String initialRevision(String surfaceId) => 'state-1';

  @override
  List<RemoteSurfaceNode> initialNodes(String surfaceId) => parseNodeList([
    {
      'nodeId': 'caption',
      'type': 'text',
      'required': true,
      'props': {'text': '只读网页快照'},
      'children': <Object?>[],
    },
    {
      'nodeId': 'origin',
      'type': 'text',
      'required': true,
      'props': {'text': '虚拟来源 surface.openmuse.invalid'},
      'children': <Object?>[],
    },
    {
      'nodeId': 'shot',
      'type': 'image',
      'required': true,
      'props': {'mediaHandle': 'media.snapshot.1', 'alt': '发布页快照'},
      'children': <Object?>[],
    },
    {
      'nodeId': 'refresh',
      'type': 'button',
      'required': true,
      'props': {
        'label': '刷新快照',
        'actionId': 'snapshot.refresh',
        'input': {'page': 'publish'},
      },
      'children': <Object?>[],
    },
  ]);

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) {
    if (request.actionId != 'snapshot.refresh') {
      return const RemoteProviderReject('UNKNOWN_ACTION');
    }
    return RemoteProviderUpdate(
      stateRevision: 'state-2',
      nodes: initialNodes(request.surfaceId),
    );
  }

  RemoteSurfaceDescriptor get _descriptor => RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-web-snapshot',
    surfaceId: 'page-snapshot',
    title: '网页快照',
    workspaceRef: workspaceRef,
    modes: const ['web-snapshot'],
    requiredCapabilities: const ['text', 'image', 'button'],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'snapshot.refresh',
        effect: 'read',
        requiredPermissions: const ['workspace.resource.read'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': ['page'],
          'properties': {
            'page': {'type': 'string', 'minLength': 1, 'maxLength': 40},
          },
        },
      ),
    ],
  );
}

final class FakeWebInteractiveProvider implements RemoteSurfaceProvider {
  FakeWebInteractiveProvider({this.workspaceRef = 'ws.opaque.1'});

  final String workspaceRef;

  @override
  String get pluginId => 'com.openmuse.fake-web-interactive';

  @override
  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context) =>
      [_descriptor];

  @override
  String initialRevision(String surfaceId) => 'state-1';

  @override
  List<RemoteSurfaceNode> initialNodes(String surfaceId) => parseNodeList([
    {
      'nodeId': 'title',
      'type': 'text',
      'required': true,
      'props': {'text': '交互网页'},
      'children': <Object?>[],
    },
  ]);

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) =>
      const RemoteProviderReject('UNKNOWN_ACTION');

  RemoteSurfaceDescriptor get _descriptor => RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-web-interactive',
    surfaceId: 'interactive-page',
    title: '交互网页',
    workspaceRef: workspaceRef,
    modes: const ['web-interactive'],
    requiredCapabilities: const ['text'],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'web.refresh',
        effect: 'read',
        requiredPermissions: const ['workspace.resource.read'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': <String>[],
          'properties': <String, Object?>{},
        },
      ),
    ],
  );
}
