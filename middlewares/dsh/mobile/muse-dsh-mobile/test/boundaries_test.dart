import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

void main() {
  test('lib does not import AppFlowy', () {
    final hits = <String>[];
    for (final entity in Directory('lib').listSync(recursive: true)) {
      if (entity is! File || !entity.path.endsWith('.dart')) continue;
      final source = entity.readAsStringSync();
      if (source.contains('package:appflowy') ||
          source.contains('muse_appflowy_facets') ||
          source.contains('package:speech_to_text') ||
          source.contains('package:image_picker') ||
          source.contains('package:file_picker') ||
          source.contains('package:share_plus') ||
          source.contains('package:permission_handler')) {
        hits.add(entity.path);
      }
    }
    expect(hits, isEmpty, reason: hits.join(', '));
  });
}
