import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

final class FakeConnector implements DshRuntimeConnector {
  FakeConnector(this.placement, {this.origin = 'https://dsh.example'});
  @override
  final DshPlacement placement;
  final String origin;
  @override
  Future<DshSessionDescriptor> open(
    String workspaceRef,
    int generation,
  ) async => DshSessionDescriptor(
    sessionRef: 'session:$workspaceRef',
    origin: origin,
    path: '/session/view',
    generation: generation,
  );
  @override
  Future<void> close(String sessionRef) async {}
}

void main() {
  test('all placements use one presentation state machine', () async {
    for (final placement in DshPlacement.values) {
      final controller = DshPresentationController();
      await controller.open(FakeConnector(placement), 'workspace:1');
      controller.pageLoaded(1);
      controller.bridgeBound(1);
      controller.workspaceAttached(1);
      expect(controller.state, DshPresentationState.ready);
    }
  });
  test('late generation callback and unsafe origin fail closed', () async {
    final controller = DshPresentationController();
    await controller.open(FakeConnector(DshPlacement.cloudRemote), 'w');
    controller.close();
    controller.pageLoaded(1);
    expect(controller.state, DshPresentationState.closed);
    await controller.open(
      FakeConnector(DshPlacement.cloudRemote, origin: 'http://evil'),
      'w',
    );
    expect(controller.session, isNull);
  });
  test('loading bridge attachment and ready remain distinct', () async {
    final controller = DshPresentationController();
    await controller.open(FakeConnector(DshPlacement.cloudRemote), 'w');
    expect(controller.state, DshPresentationState.loading);
    controller.pageLoaded(1);
    expect(controller.state, DshPresentationState.bridgeBound);
    controller.bridgeBound(1);
    expect(controller.state, DshPresentationState.workspaceAttached);
    controller.workspaceAttached(1);
    expect(controller.state, DshPresentationState.ready);
    controller.disconnected();
    expect(controller.state, DshPresentationState.reconnecting);
  });
}
