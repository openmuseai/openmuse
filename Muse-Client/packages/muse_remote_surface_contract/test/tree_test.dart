import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

void main() {
  test('RS-TREE optional unknown components stay in the tree', () {
    final snapshot = _snapshot([
      _node('extra', 'sparkline', requiredNode: false, props: {'label': '可选'}),
      _node('title', 'text', props: {'text': '标题'}),
    ]);

    expect(snapshot.nodes.first.type, 'sparkline');
    expect(
      () => validateNodesForClient(snapshot.nodes, const ['text']),
      returnsNormally,
    );
  });

  test('RS-TREE a required component missing from the client is rejected', () {
    final nodes = [
      _node(
        'clip',
        'video-player',
        props: {'posterHandle': 'media.poster.1', 'durationMs': 1},
      ),
    ];

    expect(
      () => validateNodesForClient(nodes, const ['text']),
      throwsA(
        isA<RemoteSurfaceTreeRejection>().having(
          (error) => error.code,
          'code',
          'UNSUPPORTED_COMPONENT',
        ),
      ),
    );
  });

  test('RS-TREE deep and oversized trees are rejected', () {
    expect(
      () => parseNodeList([_chain(RemoteSurfaceLimits.maxDepth + 1)]),
      throwsA(isA<FormatException>()),
    );
    expect(
      () => parseNodeList([
        for (var index = 0; index < RemoteSurfaceLimits.maxNodes + 1; index++)
          _node('n$index', 'text', props: {'text': 'x'}),
      ]),
      throwsA(isA<FormatException>()),
    );
    expect(
      () => parseNodeList([_chain(RemoteSurfaceLimits.maxDepth)]),
      returnsNormally,
    );
  });

  test('mode selection never falls through to an implicit web proxy', () {
    final hello = RemoteClientHello(
      protocolMajor: 1,
      protocolMinor: 0,
      components: const ['text'],
      mediaFormats: const ['image/jpeg'],
      webSnapshot: false,
      webInteractive: false,
      maxControlBytes: 65536,
    );

    expect(selectRemoteSurfaceMode(const ['web-interactive'], hello), isNull);
    expect(
      selectRemoteSurfaceMode(const ['web-interactive', 'declarative'], hello),
      'declarative',
    );
    expect(selectRemoteSurfaceMode(const ['media'], hello), 'media');
  });
}

RemoteSurfaceSnapshot _snapshot(List<RemoteSurfaceNode> nodes) {
  return RemoteSurfaceSnapshot.fromJson({
    'protocol': remoteSurfaceSnapshotProtocol,
    'pluginId': 'com.openmuse.fake-publish',
    'surfaceId': 'publish-workflow',
    'surfaceSessionRef': 'sess.1',
    'generation': 1,
    'stateRevision': 'state-1',
    'mode': 'declarative',
    'nodes': nodes.map((node) => node.toJson()).toList(),
  });
}

RemoteSurfaceNode _node(
  String id,
  String type, {
  bool requiredNode = true,
  required Map<String, Object?> props,
}) {
  return parseNode({
    'nodeId': id,
    'type': type,
    'required': requiredNode,
    'props': props,
    'children': const [],
  }, depth: 1);
}

Map<String, Object?> _chain(int depth) {
  Map<String, Object?> node(int level) => {
    'nodeId': 'n$level',
    'type': 'text',
    'required': true,
    'props': {'text': 'level-$level'},
    'children': level == depth ? const [] : [node(level + 1)],
  };
  return node(1);
}
