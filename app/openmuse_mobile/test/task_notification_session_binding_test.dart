import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile/native_dsh_page.dart';

void main() {
  test(
    'an old session cannot supply task outcomes for a renewed connection',
    () async {
      DshNativeGatewayClient client() => DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:18803'),
        bootstrapPath: '/bootstrap',
        allowInsecureLoopback: true,
      );
      final first = client(), renewed = client();
      final conversation = DshConversationController(client: first);
      final handle = NativeDshSessionHandle();
      handle.attach(conversation, null);
      expect(handle.isBoundToClient(first), isTrue);
      expect(handle.isBoundToClient(renewed), isFalse);
      handle.detach(conversation);
      expect(handle.isBoundToClient(first), isFalse);
      handle.dispose();
      await conversation.dispose();
      renewed.close();
    },
  );
}
