import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_plugin/src/linked_content.dart';

void main() {
  test('Word, Excel and PPTX attachments retain their OOXML media types', () {
    final bytes = [0x50, 0x4b, 3, 4, 0, 0];
    for (final type in {
      'docx': 'wordprocessingml.document',
      'xlsx': 'spreadsheetml.sheet',
      'pptx': 'presentationml.presentation',
    }.entries) {
      final content = linkedDocumentContent('1234.${type.key}', bytes);
      expect(
        content.mediaType,
        'application/vnd.openxmlformats-officedocument.${type.value}',
      );
      expect(content.bytes, same(bytes));
    }
  });
}
