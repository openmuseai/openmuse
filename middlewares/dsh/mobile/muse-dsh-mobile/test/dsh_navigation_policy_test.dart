import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  final policy = DshNavigationPolicy(
    DshRemoteConfig.tryParse('https://dsh.example.com')!,
  );

  test('allows same-origin https paths', () {
    expect(policy.allows(Uri.parse('https://dsh.example.com/chat/1')), isTrue);
  });

  test('rejects insecure, foreign, and dangerous schemes', () {
    for (final url in [
      'http://dsh.example.com/',
      'https://evil.example.com/',
      'https://dsh.example.com.evil.invalid/',
      'file:///tmp/x',
      'content://media/1',
      'intent://scan/#Intent;end',
      'javascript:alert(1)',
      'data:text/html,hi',
      'blob:https://dsh.example.com/1',
    ]) {
      expect(policy.allows(Uri.parse(url)), isFalse, reason: url);
    }
  });
}
