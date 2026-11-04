import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

void main() {
  test('RS-WIRE round-trips valid fixtures and rejects invalid ones', () async {
    final fixtures =
        (jsonDecode(
                  await File(
                    '../../contracts/openmuse-remote-surface/v1/messages.json',
                  ).readAsString(),
                )
                as List)
            .cast<Map>();
    expect(fixtures, hasLength(18));
    for (final raw in fixtures) {
      final fixture = raw.cast<String, Object?>();
      final name = fixture['name']! as String;
      final kind = fixture['kind']! as String;
      final value = (fixture['value']! as Map).cast<String, Object?>();
      if (fixture['valid'] == true) {
        final parsed = parseRemoteSurfaceFixture(kind, value) as dynamic;
        expect(parsed.toJson(), value, reason: name);
      } else {
        expect(
          () => parseRemoteSurfaceFixture(kind, value),
          throwsA(isA<FormatException>()),
          reason: name,
        );
      }
    }
  });
}
