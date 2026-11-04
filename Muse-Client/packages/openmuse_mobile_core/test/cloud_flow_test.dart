import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  test('cloud only vertical slice reaches receipt-backed applied state', () {
    final flow = CloudWorkspaceFlow()
      ..login()
      ..select('w1', 'r1');
    flow.sessionOpened(1);
    flow.bound(1, 'w1');
    flow.propose('r1');
    flow.approve('r2');
    expect(flow.state, CloudFlowState.applied);
    expect(flow.revision, 'r2');
  });
  test('cross-workspace stale revision and late callbacks fail', () {
    final flow = CloudWorkspaceFlow()
      ..login()
      ..select('w1', 'r1');
    flow.sessionOpened(0);
    expect(flow.state, CloudFlowState.queued);
    flow.sessionOpened(1);
    flow.bound(1, 'w2');
    expect(flow.state, CloudFlowState.binding);
    flow.bound(1, 'w1');
    expect(() => flow.propose('old'), throwsStateError);
    flow.storageFailed();
    expect(flow.state, CloudFlowState.storageUnavailable);
  });
}
