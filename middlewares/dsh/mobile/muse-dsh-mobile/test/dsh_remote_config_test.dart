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

  test('accepts same-origin /dsh/ and session webUrl with launch token', () {
    final sameOrigin = DshRemoteConfig.tryParse('https://app.example.com/dsh/')!;
    expect(sameOrigin.publicUri.path, '/dsh/');
    final session = DshRemoteConfig.fromWebUrl(
      'https://app.example.com/u/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/?token=launch',
    )!;
    expect(session.allows(Uri.parse(
      'https://app.example.com/u/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/?token=launch',
    )), isTrue);
    expect(DshRemoteConfig.tryParse(
      'https://app.example.com/u/x/?token=launch',
    ), isNull);
  });

  test('rejects insecure or ambiguous configured URLs', () {
    expect(DshRemoteConfig.tryParse('http://127.0.0.1:3080'), isNull);
    expect(DshRemoteConfig.tryParse('https://user@dsh.example.com'), isNull);
    expect(DshRemoteConfig.tryParse('https://dsh.example.com?token=x'), isNull);
    expect(DshRemoteConfig.tryParse('https://dsh.example.com/#x'), isNull);
    expect(DshRemoteConfig.tryParse('not a url'), isNull);
  });
}
