import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_file_viewer_flutter/src/syntax.dart';

void main() {
  test(
    'bundled native Tree-sitter parses Dart and preserves UTF-16 source',
    () async {
      const source = 'final greeting = "你好🙂"; // 注释\nint answer() => 42;\n';
      expect(viewerCodeLanguage('folder/example.dart'), 'dart');
      final ranges = await highlightViewerCode(source, 'dart');
      expect(ranges, isNotNull);
      expect(
        ranges!.any(
          (r) =>
              r.capture.startsWith('keyword') &&
              source.substring(r.from, r.to) == 'final',
        ),
        true,
      );
      expect(
        ranges.any(
          (r) =>
              r.capture.startsWith('string') &&
              source.substring(r.from, r.to).contains('你好🙂'),
        ),
        true,
      );
      expect(ranges.any((r) => r.capture.startsWith('comment')), true);
      final lines = viewerSyntaxLines(source, ranges, Brightness.light);
      expect(
        lines.map((line) => TextSpan(children: line).toPlainText()).join('\n'),
        source,
      );
      expect(lines.expand((e) => e).any((s) => s.style?.color != null), true);
    },
  );

  test(
    'unknown languages and oversized sources retain plain-text fallback',
    () async {
      expect(viewerCodeLanguage('notes.unknownformat'), isNull);
      expect(
        await highlightViewerCode('x' * (maxHighlightCodeUnits + 1), 'dart'),
        isNull,
      );
      expect(
        viewerSyntaxLines(
          '你好🙂\n',
          [],
          Brightness.dark,
        ).map((line) => TextSpan(children: line).toPlainText()).join('\n'),
        '你好🙂\n',
      );
    },
  );
}
