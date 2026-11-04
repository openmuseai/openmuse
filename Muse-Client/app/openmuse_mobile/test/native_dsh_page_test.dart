import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile/native_dsh_page.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  test('running Desktop session wins native selection', () {
    const descriptor = DshSessionDescriptor(
      sessionRef: 'paired:1',
      origin: 'http://127.0.0.1:1234',
      path: '/u/grant',
      generation: 1,
      allowInsecureLoopback: true,
    );
    final selected = chooseNativeDshSession(descriptor, const [
      DshNativeSessionSummary(
        sessionId: 'older',
        updatedAt: 1,
        running: false,
        blank: false,
      ),
      DshNativeSessionSummary(
        sessionId: 'desktop-live',
        updatedAt: 2,
        running: true,
        blank: false,
      ),
    ]);
    expect(selected?.sessionId, 'desktop-live');
  });
}
