import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_session/muse_remote_session.dart';

void main() {
  test(
      'attach/resume keeps durable cursor but drops short-lived control on network loss',
      () {
    var now = 1000;
    final client = MuseRemoteSessionClient(deviceId: 'mobile', clock: () => now)
      ..connecting();
    client.attached(const MuseRemoteAttachment(
        attachmentId: 'a1',
        sessionRef: 's1',
        deviceId: 'mobile',
        role: 'presentation',
        generation: 1,
        incarnation: 'i1'));
    client.applyStateCursor('7');
    client.putControl('selection', 'hello', 1100);
    expect(client.controlSnapshot(), {'selection': 'hello'});
    client.networkLost(operationInFlight: false);
    expect(client.state, MuseRemoteConnectionState.offline);
    expect(client.resumeCursor, '7');
    expect(client.controlSnapshot(), isEmpty);
    now = 1200;
  });

  test(
      'offline decisions distinguish idempotent queue, unknown in-flight and rejection',
      () {
    final client = MuseRemoteSessionClient(deviceId: 'web')
      ..networkLost(operationInFlight: false);
    expect(client.decideAgentOperation(idempotent: true, alreadyStarted: false),
        MuseRemoteOperationDecision.queueIdempotent);
    expect(client.decideAgentOperation(idempotent: true, alreadyStarted: true),
        MuseRemoteOperationDecision.suspendUnknown);
    expect(
        client.decideAgentOperation(idempotent: false, alreadyStarted: false),
        MuseRemoteOperationDecision.reject);
  });

  test('revoke clears attachment/context and permanently fails closed', () {
    final client = MuseRemoteSessionClient(deviceId: 'mobile')..connecting();
    client.attached(const MuseRemoteAttachment(
        attachmentId: 'a1',
        sessionRef: 's1',
        deviceId: 'mobile',
        role: 'control',
        generation: 1,
        incarnation: 'i1'));
    client.revoke();
    expect(client.attachment, isNull);
    expect(client.state, MuseRemoteConnectionState.revoked);
    expect(() => client.connecting(), throwsStateError);
  });
}
