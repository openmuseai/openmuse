import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  test('accepts an HTTPS origin and same-origin navigation', () {
    final config = DshRemoteConfig.tryParse('https://dsh.example.com')!;
    expect(config.publicUri.toString(), 'https://dsh.example.com/');
    expect(config.allows(Uri.parse('https://dsh.example.com/chat/1')), isTrue);
    expect(config.allows(Uri.parse('https://evil.example.com/')), isFalse);
    expect(config.allows(Uri.parse('http://dsh.example.com/')), isFalse);
  });

  test('rejects insecure or ambiguous configured URLs', () {
    expect(DshRemoteConfig.tryParse('http://127.0.0.1:3080'), isNull);
    expect(DshRemoteConfig.tryParse('https://user@dsh.example.com'), isNull);
    expect(DshRemoteConfig.tryParse('https://dsh.example.com?token=x'), isNull);
    expect(DshRemoteConfig.tryParse('https://dsh.example.com/#x'), isNull);
    expect(DshRemoteConfig.tryParse('not a url'), isNull);
  });
}
