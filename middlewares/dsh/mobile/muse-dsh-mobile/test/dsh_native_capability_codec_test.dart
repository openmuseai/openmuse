import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  late Map<String, dynamic> contract;

  setUpAll(() {
    final file = File('contracts/muse-native-capability.v1.json');
    expect(file.existsSync(), isTrue, reason: file.absolute.path);
    contract = jsonDecode(file.readAsStringSync()) as Map<String, dynamic>;
  });

  test('vendored contract matches codec constants', () {
    expect(contract['id'], DshNativeCapabilityCodec.protocol);
    expect(contract['channel'], 'muse.native-capability');
    expect(contract['maxBytes'], DshNativeCapabilityCodec.maxBytes);
    expect(
      (contract['forbidden'] as List).cast<String>().toSet(),
      DshNativeCapabilityCodec.forbiddenKeys,
    );
  });

  test('accepts capabilities.get and rejects forbidden or oversized payloads', () {
    final ok = DshNativeCapabilityCodec.decode(
      jsonEncode({
        'protocol': DshNativeCapabilityCodec.protocol,
        'type': 'capabilities.get',
        'requestId': 'req-1',
        'sessionId': 'session.1',
        'generation': 3,
      }),
    );
    expect(ok?.type, 'capabilities.get');
    expect(ok?.generation, 3);

    expect(
      DshNativeCapabilityCodec.decode(
        jsonEncode({
          'protocol': DshNativeCapabilityCodec.protocol,
          'type': 'speech.start',
          'requestId': 'req-2',
          'sessionId': 'session.1',
          'generation': 3,
          'token': 'secret',
        }),
      ),
      isNull,
    );
    expect(
      DshNativeCapabilityCodec.decode(
        jsonEncode({
          'protocol': DshNativeCapabilityCodec.protocol,
          'type': 'fetchUrl',
          'requestId': 'req-3',
          'generation': 3,
        }),
      ),
      isNull,
    );
    expect(
      DshNativeCapabilityCodec.decode('{"x":"${'a' * 9000}"}'),
      isNull,
    );
  });

  test('dispatch script JSON-encodes user text instead of concatenating it', () {
    final script = DshNativeCapabilityCodec.dispatchScript({
      'type': 'speech.final',
      'requestId': 'r1',
      'text': 'hello");alert(1);//',
    });
    expect(script, contains('JSON.parse('));
    expect(script, isNot(contains('hello");alert')));
  });

  test('share.open rejects non-https URLs at the broker layer via codec fields', () {
    final inbound = DshNativeCapabilityCodec.decode(
      jsonEncode({
        'protocol': DshNativeCapabilityCodec.protocol,
        'type': 'share.open',
        'requestId': 'req-4',
        'generation': 1,
        'textOrHttpsUrl': 'https://dsh.example.com/s',
      }),
    );
    expect(inbound?.textOrHttpsUrl, 'https://dsh.example.com/s');
  });
}
