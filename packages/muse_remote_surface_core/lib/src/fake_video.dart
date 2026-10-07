import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'provider.dart';

/// In-memory stand-in for a second Desktop domain: a one-clip trim.
final class FakeVideoEditProvider implements RemoteSurfaceProvider {
  FakeVideoEditProvider({this.workspaceRef = 'ws.opaque.1'});

  final String workspaceRef;
  var trimExecutions = 0;
  var _revision = 1;
  var _startMs = 0;
  var _endMs = 3000;

  @override
  String get pluginId => 'com.openmuse.fake-video';

  @override
  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context) =>
      [_descriptor];

  @override
  String initialRevision(String surfaceId) => 'state-$_revision';

  @override
  List<RemoteSurfaceNode> initialNodes(String surfaceId) => _nodes();

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) {
    if (request.actionId != 'clip.trim') {
      return const RemoteProviderReject('UNKNOWN_ACTION');
    }
    final start = request.input['startMs'];
    final end = request.input['endMs'];
    if (start is! int || end is! int || end < start) {
      return const RemoteProviderReject('SCHEMA_REJECTED');
    }
    trimExecutions += 1;
    _startMs = start;
    _endMs = end;
    _revision += 1;
    return RemoteProviderUpdate(
      stateRevision: initialRevision(request.surfaceId),
      nodes: _nodes(),
    );
  }

  List<RemoteSurfaceNode> _nodes() => parseNodeList([
    {
      'nodeId': 'player',
      'type': 'video-player',
      'required': true,
      'props': {
        'posterHandle': 'media.video.1',
        'durationMs': _endMs - _startMs,
      },
      'children': <Object?>[],
    },
    {
      'nodeId': 'timeline',
      'type': 'timeline-basic',
      'required': true,
      'props': {
        'clips': [
          {
            'id': 'clip-1',
            'label': '片段一',
            'startMs': _startMs,
            'endMs': _endMs,
          },
        ],
      },
      'children': <Object?>[],
    },
    {
      'nodeId': 'trim',
      'type': 'button',
      'required': true,
      'props': {
        'label': '裁剪',
        'actionId': 'clip.trim',
        'input': {'clipId': 'clip-1', 'startMs': 200, 'endMs': 2400},
      },
      'children': <Object?>[],
    },
  ]);

  RemoteSurfaceDescriptor get _descriptor => RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-video',
    surfaceId: 'edit-timeline',
    title: '视频编辑',
    workspaceRef: workspaceRef,
    modes: const ['declarative', 'media'],
    requiredCapabilities: const ['video-player', 'timeline-basic', 'button'],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'clip.trim',
        effect: 'workspace-commit',
        requiredPermissions: const ['workspace.resource.write'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': ['clipId', 'startMs', 'endMs'],
          'properties': {
            'clipId': {'type': 'string', 'minLength': 1, 'maxLength': 64},
            'startMs': {'type': 'integer', 'minimum': 0},
            'endMs': {'type': 'integer', 'minimum': 0},
          },
        },
      ),
    ],
  );
}
