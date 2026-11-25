import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/dsh_resource_source.dart';

void main() {
  test(
    'Unicode TypeScript source can be opened as text despite the transport-stream suffix',
    () {
      expect(
        dshResourceFileMediaType(
          '导航示例.ts',
          utf8.encode('export const value: string = "中文🙂";\n'),
        ),
        'text/typescript',
      );
    },
  );
  test(
    'binary TS and magic-detected content are not reclassified as source',
    () {
      expect(
        dshResourceFileMediaType('movie.ts', [
          0x47,
          0x40,
          0x11,
          0x10,
          0xff,
          0x00,
        ]),
        startsWith('video/'),
      );
      expect(
        dshResourceFileMediaType('document.ts', utf8.encode('%PDF-1.7\n')),
        'application/pdf',
      );
    },
  );
}
