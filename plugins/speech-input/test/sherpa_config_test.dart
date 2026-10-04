import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_speech_input/openmuse_speech_input.dart';

void main() {
  test('zh14m reports every missing model artifact', () {
    final directory = Directory.systemTemp.createTempSync('openmuse-asr-');
    addTearDown(() => directory.deleteSync(recursive: true));

    final config = SherpaZipformerConfig.zh14m(directory.path);

    expect(config.isInstalled, isFalse);
    expect(config.missingFiles, hasLength(4));
    expect(config.modelId, contains('zipformer-zh-14M'));
  });

  test('zh14m becomes installed only when its complete bundle exists', () {
    final directory = Directory.systemTemp.createTempSync('openmuse-asr-');
    addTearDown(() => directory.deleteSync(recursive: true));
    final config = SherpaZipformerConfig.zh14m(directory.path);

    for (final path in config.files) {
      File(path).writeAsStringSync('fixture');
    }

    expect(config.isInstalled, isTrue);
    expect(config.missingFiles, isEmpty);
  });
}
