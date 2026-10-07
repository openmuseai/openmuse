import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

import 'provider.dart';

/// In-memory stand-in for a Desktop publish workflow. It does not talk to a
/// social network or keep credentials.
final class FakePublishSurfaceProvider implements RemoteSurfaceProvider {
  FakePublishSurfaceProvider({this.workspaceRef = 'ws.opaque.1'});

  static const accountRef = 'account-1';
  final String workspaceRef;

  var readExecutions = 0;
  var externalExecutions = 0;
  var _revision = 1;
  String? previewRef;
  String _status = '尚未预览';
  String _statusState = 'idle';

  @override
  String get pluginId => 'com.openmuse.fake-publish';

  @override
  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context) =>
      [_descriptor];

  @override
  String initialRevision(String surfaceId) => 'state-$_revision';

  @override
  List<RemoteSurfaceNode> initialNodes(String surfaceId) => _nodes();

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) {
    switch (request.actionId) {
      case 'social.preview':
        final title = request.input['title'];
        if (title is! String) {
          return const RemoteProviderReject('SCHEMA_REJECTED');
        }
        readExecutions += 1;
        previewRef = 'preview-$readExecutions';
        _status = '预览已生成：$title';
        _statusState = 'ready';
        _revision += 1;
        return RemoteProviderUpdate(
          stateRevision: initialRevision(request.surfaceId),
          nodes: _nodes(),
        );
      case 'social.publish.commit':
        if (request.input['previewRef'] != previewRef ||
            request.input['accountRef'] != accountRef) {
          return const RemoteProviderReject('PREVIEW_MISMATCH');
        }
        externalExecutions += 1;
        _status = '发布已提交';
        _statusState = 'running';
        _revision += 1;
        return RemoteProviderUpdate(
          stateRevision: initialRevision(request.surfaceId),
          nodes: _nodes(),
          jobRef: 'job-$externalExecutions',
          eventState: 'running',
        );
      default:
        return const RemoteProviderReject('UNKNOWN_ACTION');
    }
  }

  List<RemoteSurfaceNode> _nodes() {
    final commitInput = <String, Object?>{
      'accountRef': accountRef,
      if (previewRef != null) 'previewRef': previewRef,
    };
    return parseNodeList([
      _text('heading', '社媒发布'),
      {
        'nodeId': 'cover',
        'type': 'image',
        'required': true,
        'props': {'mediaHandle': 'media.cover.1', 'alt': '封面'},
        'children': <Object?>[],
      },
      {
        'nodeId': 'clip',
        'type': 'video-player',
        'required': true,
        'props': {'posterHandle': 'media.poster.1', 'durationMs': 12000},
        'children': <Object?>[],
      },
      {
        'nodeId': 'extra',
        'type': 'sparkline',
        'required': false,
        'props': {'label': '可选图表'},
        'children': <Object?>[],
      },
      {
        'nodeId': 'draft',
        'type': 'form',
        'required': true,
        'props': {
          'fields': [
            {'id': 'title', 'kind': 'text', 'label': '标题', 'value': '周末散步'},
          ],
        },
        'children': <Object?>[],
      },
      {
        'nodeId': 'state',
        'type': 'status',
        'required': true,
        'props': {'label': _status, 'state': _statusState},
        'children': <Object?>[],
      },
      {
        'nodeId': 'preview',
        'type': 'button',
        'required': true,
        'props': {
          'label': '生成预览',
          'actionId': 'social.preview',
          'inputFromFields': ['title'],
        },
        'children': <Object?>[],
      },
      {
        'nodeId': 'commit',
        'type': 'confirmation',
        'required': true,
        'props': {
          'label': '确认发布',
          'actionId': 'social.publish.commit',
          'prompt': '确认发布这份预览？',
          'input': commitInput,
        },
        'children': <Object?>[],
      },
    ]);
  }

  static Map<String, Object?> _text(String id, String text) => {
    'nodeId': id,
    'type': 'text',
    'required': true,
    'props': {'text': text},
    'children': <Object?>[],
  };

  RemoteSurfaceDescriptor get _descriptor => RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-publish',
    surfaceId: 'publish-workflow',
    title: '社媒发布',
    workspaceRef: workspaceRef,
    modes: const ['declarative', 'media'],
    requiredCapabilities: const [
      'text',
      'image',
      'video-player',
      'form',
      'button',
      'confirmation',
      'status',
    ],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'social.preview',
        effect: 'read',
        requiredPermissions: const ['workspace.resource.read'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': ['title'],
          'properties': {
            'title': {'type': 'string', 'minLength': 1, 'maxLength': 200},
          },
        },
      ),
      RemoteSurfaceAction(
        id: 'social.publish.commit',
        effect: 'external-side-effect',
        requiredPermissions: const ['social.content.publish'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': ['previewRef', 'accountRef'],
          'properties': {
            'previewRef': {'type': 'string', 'minLength': 1, 'maxLength': 80},
            'accountRef': {'type': 'string', 'minLength': 1, 'maxLength': 80},
          },
        },
      ),
    ],
  );
}
