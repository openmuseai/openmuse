import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';

const _context = RemoteConnectionContext(
  actorRef: 'actor.1',
  mobileDeviceRef: 'mobile.1',
  desktopDeviceRef: 'desktop.1',
  workspaceRef: 'ws.opaque.1',
  permissions: {
    'workspace.resource.read',
    'workspace.resource.write',
    'social.content.publish',
  },
);

void main() {
  test('RS-HOST-01 disabling publish leaves the video surface usable', () {
    final publish = FakePublishSurfaceProvider();
    final video = FakeVideoEditProvider();
    final host = RemoteSurfaceHost()
      ..register(publish)
      ..register(video);
    final publishSession = _open(host, publish.pluginId, 'publish-workflow');
    final videoSession = _open(host, video.pluginId, 'edit-timeline');

    host.unregister(publish.pluginId);
    final denied = host.submit(
      _request(publishSession, 'social.preview', {'title': '周末散步'}),
      context: _context,
    );
    final trimmed = host.submit(
      _request(videoSession, 'clip.trim', {
        'clipId': 'clip-1',
        'startMs': 200,
        'endMs': 2400,
      }),
      context: _context,
    );

    expect(denied.status, 'unsupported');
    expect(denied.errorCode, 'STALE_GENERATION');
    expect(publish.externalExecutions, 0);
    expect(trimmed.status, 'accepted');
    expect(trimmed.jobRef, isNull);
    expect(video.trimExecutions, 1);
  });

  test('RS-HOST-02 idempotency covers one submission and allows another', () {
    final publish = FakePublishSurfaceProvider();
    final host = RemoteSurfaceHost()..register(publish);
    final opened = _open(host, publish.pluginId, 'publish-workflow');
    final tooEarly = host.submit(
      _request(opened, 'social.publish.commit', {'accountRef': 'account-1'}),
      context: _context,
    );
    expect(tooEarly.errorCode, 'SCHEMA_REJECTED');
    expect(publish.externalExecutions, 0);

    final preview = host.submit(
      _request(opened, 'social.preview', {'title': '周末散步'}, key: 'preview-1'),
      context: _context,
    );
    expect(preview.status, 'accepted');
    final current = host.readSnapshot(
      surfaceSessionRef: opened.surfaceSessionRef,
      generation: opened.generation,
    );
    final commit = _request(current, 'social.publish.commit', {
      'previewRef': 'preview-1',
      'accountRef': 'account-1',
    }, key: 'commit-1');
    final first = host.submit(commit, context: _context);
    final replay = host.submit(commit, context: _context);
    final conflict = host.submit(
      _request(current, 'social.publish.commit', {
        'previewRef': 'preview-1',
        'accountRef': 'account-9',
      }, key: 'commit-1'),
      context: _context,
    );
    final again = host.submit(
      _request(
        host.readSnapshot(
          surfaceSessionRef: opened.surfaceSessionRef,
          generation: opened.generation,
        ),
        'social.publish.commit',
        {'previewRef': 'preview-1', 'accountRef': 'account-1'},
        key: 'commit-2',
      ),
      context: _context,
    );

    expect(first.status, 'accepted');
    expect(replay.decisionRef, first.decisionRef);
    expect(conflict.status, 'conflict');
    expect(conflict.errorCode, 'IDEMPOTENCY_CONFLICT');
    expect(again.status, 'accepted');
    expect(publish.externalExecutions, 2);
    expect(publish.readExecutions, 1);
  });

  test('RS-HOST-03 a stale generation does not execute', () {
    final publish = FakePublishSurfaceProvider();
    final host = RemoteSurfaceHost()..register(publish);
    final opened = _open(host, publish.pluginId, 'publish-workflow');
    host.invalidateAll();

    final receipt = host.submit(
      _request(opened, 'social.preview', {'title': '周末散步'}),
      context: _context,
    );

    expect(receipt.status, 'unsupported');
    expect(receipt.errorCode, 'STALE_GENERATION');
    expect(publish.readExecutions, 0);
    expect(
      () => host.readSnapshot(
        surfaceSessionRef: opened.surfaceSessionRef,
        generation: opened.generation,
      ),
      throwsA(
        isA<RemoteSurfaceClosed>().having(
          (error) => error.code,
          'code',
          'STALE_GENERATION',
        ),
      ),
    );
  });

  test('RS-HOST-04 publish permission is checked before execution', () {
    final publish = FakePublishSurfaceProvider();
    final host = RemoteSurfaceHost()..register(publish);
    final opened = _open(host, publish.pluginId, 'publish-workflow');
    host.submit(
      _request(opened, 'social.preview', {'title': '周末散步'}, key: 'preview'),
      context: _context,
    );
    final current = host.readSnapshot(
      surfaceSessionRef: opened.surfaceSessionRef,
      generation: opened.generation,
    );

    final receipt = host.submit(
      _request(current, 'social.publish.commit', {
        'previewRef': 'preview-1',
        'accountRef': 'account-1',
      }),
      context: const RemoteConnectionContext(
        actorRef: 'actor.1',
        mobileDeviceRef: 'mobile.1',
        desktopDeviceRef: 'desktop.1',
        workspaceRef: 'ws.opaque.1',
        permissions: {'workspace.resource.read'},
      ),
    );

    expect(receipt.status, 'denied');
    expect(receipt.errorCode, 'DENIED');
    expect(publish.externalExecutions, 0);
  });

  test('RS-HOST-05 a required unknown component opens nothing', () {
    final host = RemoteSurfaceHost()..register(_CameraProvider());
    final result = host.open(
      pluginId: 'com.openmuse.fake-camera',
      surfaceId: 'capture',
      hello: RemoteClientHello.mobileV1,
      context: _context,
    );
    final offers = host.discover(
      hello: RemoteClientHello.mobileV1,
      context: _context,
    );

    expect(result, isA<RemoteOpenRejected>());
    expect((result as RemoteOpenRejected).errorCode, 'UNSUPPORTED_COMPONENT');
    expect(offers.single.compatible, isFalse);
    expect(
      host.lookup(surfaceSessionRef: 'sess.1', generation: 1, requestId: 'req'),
      isNull,
    );
  });

  test('RS-HOST-06 web-interactive is not replaced by a proxy mode', () {
    final host = RemoteSurfaceHost()..register(_WebOnlyProvider());
    final offers = host.discover(
      hello: RemoteClientHello.mobileV1,
      context: _context,
    );
    final opened = host.open(
      pluginId: 'com.openmuse.fake-web',
      surfaceId: 'page',
      hello: RemoteClientHello.mobileV1,
      context: _context,
    );

    expect(offers.single.compatible, isFalse);
    expect(offers.single.mode, isNull);
    expect(offers.single.unsupportedReason, 'UNSUPPORTED_MODE');
    expect((opened as RemoteOpenRejected).errorCode, 'UNSUPPORTED_MODE');
  });

  test('RS-HOST-07 a surface from another workspace is not discovered', () {
    final host = RemoteSurfaceHost()..register(FakePublishSurfaceProvider());

    final offers = host.discover(
      hello: RemoteClientHello.mobileV1,
      context: const RemoteConnectionContext(
        actorRef: 'actor.1',
        mobileDeviceRef: 'mobile.1',
        desktopDeviceRef: 'desktop.1',
        workspaceRef: 'ws.other',
        permissions: {'workspace.resource.read', 'social.content.publish'},
      ),
    );

    expect(offers, isEmpty);
  });

  test('RS-HOST-08 a crashing provider does not block another plugin', () {
    final video = FakeVideoEditProvider();
    final host = RemoteSurfaceHost()
      ..register(_BoomProvider())
      ..register(video);
    final boom = _open(host, 'com.openmuse.fake-boom', 'boom');
    final edited = _open(host, video.pluginId, 'edit-timeline');

    final failed = host.submit(
      _request(boom, 'boom.run', {'title': 'x'}),
      context: _context,
    );
    final trimmed = host.submit(
      _request(edited, 'clip.trim', {
        'clipId': 'clip-1',
        'startMs': 200,
        'endMs': 2400,
      }, key: 'trim'),
      context: _context,
    );

    expect(failed.errorCode, 'PROVIDER_FAILED');
    expect(trimmed.status, 'accepted');
    expect(video.trimExecutions, 1);
  });
}

RemoteSurfaceSnapshot _open(
  RemoteSurfaceHost host,
  String pluginId,
  String surfaceId,
) {
  final result = host.open(
    pluginId: pluginId,
    surfaceId: surfaceId,
    hello: RemoteClientHello.mobileV1,
    context: _context,
  );
  expect(result, isA<RemoteOpenAccepted>());
  return (result as RemoteOpenAccepted).snapshot;
}

RemoteControlRequest _request(
  RemoteSurfaceSnapshot snapshot,
  String actionId,
  Map<String, Object?> input, {
  String? key,
}) {
  final id = key ?? 'submission-1';
  return RemoteControlRequest(
    requestId: 'req-$id',
    surfaceSessionRef: snapshot.surfaceSessionRef,
    generation: snapshot.generation,
    actionId: actionId,
    input: input,
    expectedStateRevision: snapshot.stateRevision,
    idempotencyKey: id,
    deadlineMs: 10000,
  );
}

final class _CameraProvider implements RemoteSurfaceProvider {
  @override
  String get pluginId => 'com.openmuse.fake-camera';

  @override
  List<RemoteSurfaceDescriptor> descriptors(RemoteConnectionContext context) =>
      [_descriptor];

  @override
  String initialRevision(String surfaceId) => 'state-1';

  @override
  List<RemoteSurfaceNode> initialNodes(String surfaceId) => [
    RemoteSurfaceNode(
      nodeId: 'pad',
      type: 'camera-pad',
      requiredNode: true,
      props: const {'label': '相机'},
      children: const [],
    ),
  ];

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) =>
      const RemoteProviderReject('UNKNOWN_ACTION');

  static final _descriptor = RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-camera',
    surfaceId: 'capture',
    title: '相机',
    workspaceRef: 'ws.opaque.1',
    modes: const ['declarative'],
    requiredCapabilities: const ['text'],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'camera.open',
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

final class _WebOnlyProvider implements RemoteSurfaceProvider {
  @override
  String get pluginId => 'com.openmuse.fake-web';

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
      'props': {'text': '网页'},
      'children': <Object?>[],
    },
  ]);

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) =>
      const RemoteProviderReject('UNKNOWN_ACTION');

  static final _descriptor = RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-web',
    surfaceId: 'page',
    title: '网页',
    workspaceRef: 'ws.opaque.1',
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

final class _BoomProvider implements RemoteSurfaceProvider {
  @override
  String get pluginId => 'com.openmuse.fake-boom';

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
      'props': {'text': '会失败'},
      'children': <Object?>[],
    },
  ]);

  @override
  RemoteProviderResult execute(RemoteProviderRequest request) {
    throw StateError('boom');
  }

  static final _descriptor = RemoteSurfaceDescriptor(
    pluginId: 'com.openmuse.fake-boom',
    surfaceId: 'boom',
    title: '故障',
    workspaceRef: 'ws.opaque.1',
    modes: const ['declarative'],
    requiredCapabilities: const ['text'],
    readPermissions: const ['workspace.resource.read'],
    actions: [
      RemoteSurfaceAction(
        id: 'boom.run',
        effect: 'read',
        requiredPermissions: const ['workspace.resource.read'],
        inputSchema: {
          'type': 'object',
          'additionalProperties': false,
          'required': ['title'],
          'properties': {
            'title': {'type': 'string', 'minLength': 1, 'maxLength': 20},
          },
        },
      ),
    ],
  );
}
