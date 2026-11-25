import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import '../../../scripts/ci/local_account_file.dart';

void main() {
  test(
    'operator comment is skipped and later tokens are outside the account section',
    () {
      final a = parseLocalAccount(
        'CN service:\naccounts: [\n{\n"test@example.test",\n// operator note\n"fake-pass"\n}\n]\nother-token:"unrelated"',
      );
      expect(a.email, 'test@example.test');
      expect(a.password, 'fake-pass');
    },
  );
  test('JSON password slashes, quotes and brackets are preserved', () {
    const p = 'fake//[value]"\\end';
    expect(
      parseLocalAccount('["test@example.test",${jsonEncode(p)}]').password,
      p,
    );
  });
  test('valid object works and incomplete or ambiguous inputs fail closed', () {
    expect(
      parseLocalAccount(
        '[{"email":"test@example.test","password":"fake"}]',
      ).password,
      'fake',
    );
    for (final text in [
      '["test@example.test",// missing\n]',
      '["test@example.test","fake","extra"]',
      '["test@example.test","unterminated]',
      'other-token:"fake"',
    ]) {
      expect(() => parseLocalAccount(text), throwsFormatException);
    }
  });
}
