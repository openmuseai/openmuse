import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';

void main() {
  testWidgets(
    'actual glyph tap resolves symbol, selects target and preserves source scroll',
    (tester) async {
      final calls = <(ResourceCodeOperation, int, int)>[];
      ResourceCodeTarget? opened;
      final target = ResourceCodeTarget(
        reference: ResourceReference(
          identity: ResourceIdentity(
            scope: 'account',
            source: 'source',
            workspace: 'workspace',
            mount: 'mount',
            resource: 'target',
          ),
          version: ResourceVersion(revision: 'v1', digest: 'a' * 64),
        ),
        label: '中文定义.dart',
        line: 40,
        character: 7,
        endLine: 40,
        endCharacter: 11,
      );
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: OpenMuseCodeBody(
              contents: List.generate(
                160,
                (i) => i == 119 ? 'final value = load();' : '// line ${i + 1}',
              ).join('\n'),
              language: 'dart',
              initialLine: 120,
              navigateCode: (operation, line, character) async {
                calls.add((operation, line, character));
                return ResourceCodeResult(
                  enabled: true,
                  matches: operation == ResourceCodeOperation.status
                      ? []
                      : [target],
                );
              },
              openCodeTarget: (value) async {
                opened = value;
                return true;
              },
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      final row = find.byKey(const ValueKey('file-viewer-code-line-120'));
      final paragraph = tester.renderObject<RenderParagraph>(
        find.descendant(of: row, matching: find.byType(RichText)),
      );
      final box = paragraph
          .getBoxesForSelection(
            const TextSelection(baseOffset: 14, extentOffset: 18),
          )
          .first
          .toRect();
      await tester.tapAt(paragraph.localToGlobal(box.center));
      await tester.pumpAndSettle();
      expect(
        calls,
        isNotEmpty,
        reason:
            'glyph=${paragraph.localToGlobal(box.center)} row=${tester.getRect(row)}',
      );
      // The selected token and the queried character must agree after hit testing.
      final titles = tester
          .widgetList<Text>(find.byType(Text))
          .map((t) => t.data)
          .whereType<String>()
          .where((t) => t.startsWith('代码导航 ·'))
          .toList();
      expect(titles, ['代码导航 · load']);
      expect(find.text('代码导航 · load'), findsOneWidget);
      expect(calls.last.$1, ResourceCodeOperation.definition);
      expect(calls.last.$2, 119);
      expect(calls.last.$3, inInclusiveRange(14, 18));
      await tester.tap(find.text('中文定义.dart:41'));
      await tester.pumpAndSettle();
      expect(opened, same(target));
      expect(row, findsOneWidget);
      expect(find.text('// line 1'), findsNothing);
    },
  );

  testWidgets(
    'default-off service needs explicit opt-in; closing releases it',
    (tester) async {
      var enabled = false;
      final calls = <ResourceCodeOperation>[];
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: OpenMuseCodeBody(
              contents: 'void main() {}',
              language: 'dart',
              navigateCode: (operation, line, character) async {
                calls.add(operation);
                if (operation == ResourceCodeOperation.enable) enabled = true;
                if (operation == ResourceCodeOperation.disable) enabled = false;
                return ResourceCodeResult(enabled: enabled);
              },
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(calls, isEmpty);
      await tester.tap(find.text('代码导航'));
      await tester.pumpAndSettle();
      expect(calls, [ResourceCodeOperation.status]);
      await tester.tap(find.text('为此工作区启用代码导航'));
      await tester.pumpAndSettle();
      expect(enabled, true);
      await tester.tap(find.text('关闭并释放'));
      await tester.pumpAndSettle();
      expect(enabled, false);
    },
  );
}
