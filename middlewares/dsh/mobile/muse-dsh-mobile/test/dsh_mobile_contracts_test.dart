import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  late Map<String, dynamic> json;

  setUpAll(() {
    final file = File('contracts/dsh-mobile-error-codes.v1.json');
    expect(file.existsSync(), isTrue, reason: file.absolute.path);
    json = jsonDecode(file.readAsStringSync()) as Map<String, dynamic>;
  });

  test('fatal codes match frozen JSON messages', () {
    final fatal = json['fatal'] as Map<String, dynamic>;
    for (final code in DshMobileErrorCode.fatal) {
      expect(fatal[code.code], code.message);
    }
    expect(fatal.length, DshMobileErrorCode.fatal.length);
  });

  test('degraded codes match frozen JSON messages', () {
    final degraded = json['degraded'] as Map<String, dynamic>;
    for (final code in DshMobileErrorCode.degraded) {
      expect(degraded[code.code], code.message);
    }
    expect(degraded.length, DshMobileErrorCode.degraded.length);
  });
}
