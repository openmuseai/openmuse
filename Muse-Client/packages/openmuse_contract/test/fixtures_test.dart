import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_contract/openmuse_contract.dart';

Future<Object?> fixture(String name) async => jsonDecode(
    await File('../../schemas/fixtures/contract/v1/$name').readAsString());

void main() {
  for (final name in [
    'request.success.json',
    'response.success.json',
    'response.denied.json',
    'response.expired.json',
    'response.conflict.json',
  ]) {
    test('round trips $name', () async {
      final input = fixture(name);
      final value = await input;
      final parsed = parseContractEnvelope(value, expectedGeneration: 3);
      expect(parsed.toJson(), equals(value));
    });
  }

  test('rejects stale generation before exposing the outcome', () async {
    final input = await fixture('response.stale-generation.json');
    expect(
      () => parseContractEnvelope(input, expectedGeneration: 3),
      throwsA(
        isA<OpenMuseContractException>().having(
          (error) => error.message,
          'message',
          contains('stale generation'),
        ),
      ),
    );
  });

  test('rejects an unknown protocol major', () async {
    final input = await fixture('request.unknown-major.json');
    expect(
      () => parseContractEnvelope(input, expectedGeneration: 3),
      throwsA(
        isA<OpenMuseContractException>().having(
          (error) => error.message,
          'message',
          contains('incompatible protocol'),
        ),
      ),
    );
  });

  test('round trips descriptor, handle, lease and receipt', () async {
    final input = await fixture('lifecycle.snapshot.json');
    expect(LifecycleSnapshot.fromJson(input).toJson(), equals(input));
  });

  test('deadline and error vocabulary are frozen', () async {
    final request = parseContractEnvelope(
      await fixture('request.success.json'),
    ) as RequestEnvelope;
    expect(
      () => request.ensureLiveAt(request.deadlineAtMs),
      throwsA(isA<OpenMuseContractException>()),
    );
    expect(ContractErrorCode.values, hasLength(8));
  });
}
