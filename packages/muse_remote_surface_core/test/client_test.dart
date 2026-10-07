import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';

const _context = RemoteConnectionContext(
  actorRef: 'actor.1',
  mobileDeviceRef: 'mobile.1',
  desktopDeviceRef: 'desktop.1',
  workspaceRef: 'ws.opaque.1',
  permissions: {'workspace.resource.read', 'social.content.publish'},
);

void main() {
  test(
    'RS-CLIENT-01 a dropped external response is recovered without resubmit',
    () async {
      final harness = await _Harness.open();
      await harness.preview();
      harness.transport.dropSubmitResponse = true;

      await harness.controller.invoke(harness.node('commit'));

      expect(harness.transport.submits, 2);
      expect(harness.publish.externalExecutions, 1);
      expect(harness.controller.jobState, 'running');
      expect(harness.controller.error, isNull);
      expect(_label(harness.controller.snapshot!, 'state'), '发布已提交');
    },
  );

  test('RS-CLIENT-02 a request that never arrives is not retried', () async {
    final harness = await _Harness.open();
    await harness.preview();
    harness.transport.failBeforeSubmit = true;

    await harness.controller.invoke(harness.node('commit'));

    expect(harness.transport.submits, 1);
    expect(harness.publish.externalExecutions, 0);
    expect(harness.controller.jobState, 'outcome_unknown');
    expect(harness.controller.error, '结果未知，未重新提交');
  });

  test(
    'RS-CLIENT-03 reconnect replays events after the stored cursor',
    () async {
      final harness = await _Harness.open();
      final opened = harness.controller.snapshot!;
      harness.host.submit(
        RemoteControlRequest(
          requestId: 'req-server-preview',
          surfaceSessionRef: opened.surfaceSessionRef,
          generation: opened.generation,
          actionId: 'social.preview',
          input: const {'title': '周末散步'},
          expectedStateRevision: opened.stateRevision,
          idempotencyKey: 'server-preview',
          deadlineMs: 10000,
        ),
        context: _context,
      );
      final previewed = harness.host.readSnapshot(
        surfaceSessionRef: opened.surfaceSessionRef,
        generation: opened.generation,
      );
      harness.host.submit(
        RemoteControlRequest(
          requestId: 'req-server-commit',
          surfaceSessionRef: previewed.surfaceSessionRef,
          generation: previewed.generation,
          actionId: 'social.publish.commit',
          input: const {'previewRef': 'preview-1', 'accountRef': 'account-1'},
          expectedStateRevision: previewed.stateRevision,
          idempotencyKey: 'server-commit',
          deadlineMs: 10000,
        ),
        context: _context,
      );

      await harness.controller.reconnect();
      await harness.controller.reconnect();

      expect(harness.controller.lastSeq, 1);
      expect(harness.controller.jobState, 'running');
      expect(_label(harness.controller.snapshot!, 'state'), '发布已提交');
      expect(harness.publish.externalExecutions, 1);
    },
  );
}

String _label(RemoteSurfaceSnapshot snapshot, String nodeId) =>
    _node(snapshot, nodeId).props['label']! as String;

RemoteSurfaceNode _node(RemoteSurfaceSnapshot snapshot, String nodeId) {
  for (final node in snapshot.nodes) {
    if (node.nodeId == nodeId) return node;
  }
  throw StateError('missing $nodeId');
}

final class _Harness {
  _Harness(this.host, this.publish, this.transport, this.controller);

  final RemoteSurfaceHost host;
  final FakePublishSurfaceProvider publish;
  final RemoteSurfaceFaultTransport transport;
  final RemoteWorkbenchController controller;

  static Future<_Harness> open() async {
    final publish = FakePublishSurfaceProvider();
    final host = RemoteSurfaceHost()..register(publish);
    final transport = RemoteSurfaceFaultTransport(
      InMemoryRemoteSurfaceTransport(host),
    );
    final controller = RemoteWorkbenchController(
      transport: transport,
      context: _context,
    );
    await controller.start();
    return _Harness(host, publish, transport, controller);
  }

  Future<void> preview() => controller.invoke(node('preview'));

  RemoteSurfaceNode node(String nodeId) => _node(controller.snapshot!, nodeId);
}
