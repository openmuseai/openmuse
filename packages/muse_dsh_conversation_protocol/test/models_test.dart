import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

void main() {
  test('workspace and created session parse the native catalog contract', () {
    final workspace = DshNativeWorkspaceSummary.fromJson({
      'workspaceId': 'w-1',
      'title': 'Project',
      'sessionIds': ['s-2', 's-1'],
      'createdAt': '2026-09-01T00:00:00.000Z',
      'updatedAt': '2026-10-01T00:00:00.000Z',
    });
    expect(workspace.workspaceId, 'w-1');
    expect(workspace.sessionIds, ['s-2', 's-1']);
    expect(workspace.updatedAt.toUtc().year, 2026);

    final created = DshNativeCreatedSession.fromJson({
      'sessionId': 's-new',
      'agentPreset': 'standard',
    });
    expect(created.sessionId, 's-new');
    expect(created.agentPreset, 'standard');
  });

  test('accepts only the pinned DSH native protocol', () {
    final hello = DshGatewayHello.fromJson({
      'protocolVersion': 1,
      'dshVersion': '0.1.7-rc.1',
      'capabilities': ['session.follow'],
      'compatibility': {
        'nativeConversationCompatible': true,
        'fallbackRequired': false,
        'unsupportedPlugins': <Object?>[],
      },
    });
    expect(hello.supported, isTrue);
  });

  test('decodes snapshot and preserves unknown wire events', () {
    final frame =
        DshFollowFrame.fromJson({
              'type': 'snapshot',
              'header': {'id': 's-1'},
              'cursor': 7,
              'records': [
                {
                  'type': 'event',
                  'event': {
                    'type': 'plugin/custom',
                    'seq': 7,
                    'time': 9,
                    'data': {'value': true},
                  },
                },
              ],
              'hasMore': false,
              'projections': {'asOfSeq': 7, 'values': <String, Object?>{}},
            })
            as DshSnapshotFrame;
    expect(frame.records.single.type, 'plugin/custom');
    expect(frame.records.single.raw['seq'], 7);
  });

  test('decodes declarative native UI negotiation', () {
    final value = DshNativeNegotiation.fromJson({
      'schemaVersion': 1,
      'mode': 'native',
      'plugins': <Object?>[],
      'contributions': [
        {
          'slot': 'tool.call.toolview',
          'key': 'weather',
          'template': 'toolCard',
          'body': [
            {'component': 'keyValue', 'label': '温度', 'value': 21},
          ],
          'actions': <Object?>[],
        },
      ],
    });
    expect(value.mode, DshNativeUiMode.native);
    expect(value.contributions.single.key, 'weather');
    expect(DshNativeCapabilities.standard().components, contains('toolCard@1'));
  });

  test('decodes Desktop model and permission catalogs', () {
    final value = DshNativeSessionOptions.fromJson({
      'models': {
        'default': {'provider': 'deepseek', 'model': 'chat'},
        'groups': [
          {
            'id': 'deepseek',
            'name': 'DeepSeek',
            'models': [
              {
                'id': 'chat',
                'name': 'Chat',
                'reasoning': {
                  'defaultEffort': 'medium',
                  'efforts': [
                    {'id': 'high', 'name': 'High'},
                  ],
                },
              },
            ],
          },
        ],
      },
      'permissions': {
        'defaultPreset': 'workspace-write',
        'options': [
          {'value': 'workspace-write', 'name': 'Workspace write'},
        ],
      },
    });
    expect(value.defaultModel.provider, 'deepseek');
    expect(value.models.single.efforts.single.id, 'high');
    expect(value.permissions.single.value, 'workspace-write');
  });

  test('decodes a path-redacted newly-created artifact preview', () {
    final value = DshNativeArtifactPreview.fromJson({
      'kind': 'text',
      'display': 'blog/example.mdx',
      'before': false,
      'after': true,
      'coarse': false,
      'hunks': [
        {
          'oldStart': 1,
          'oldLines': 0,
          'newStart': 1,
          'newLines': 2,
          'lines': ['+---', '+# Hello'],
        },
      ],
    });
    expect(value.isCreated, isTrue);
    expect(value.display, 'blog/example.mdx');
    expect(value.hunks.single.lines, ['+---', '+# Hello']);
  });
}
