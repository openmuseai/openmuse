import 'dart:io';

import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:appflowy_editor_native_plugin/appflowy_editor_native_plugin.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('loads a Markdown file in the native Flutter editor', (
    tester,
  ) async {
    final directory = Directory.systemTemp.createTempSync('appflowy-native-');
    addTearDown(() => directory.deleteSync(recursive: true));
    final file = File('${directory.path}/README.md');
    file.writeAsStringSync('# Native editor\n\nA real document.\n');

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AppFlowyMarkdownEditor(
            filePath: file.path,
            initialMarkdown: file.readAsStringSync(),
          ),
        ),
      ),
    );
    await tester.pump();
    await tester.runAsync(() async {
      await Future<void>.delayed(const Duration(milliseconds: 300));
    });
    await tester.pump(const Duration(milliseconds: 300));

    expect(find.byType(AppFlowyEditor), findsOneWidget);
    final editor = tester.widget<AppFlowyEditor>(find.byType(AppFlowyEditor));
    final document = documentToMarkdown(editor.editorState.document);
    expect(document, contains('Native editor'));
    expect(document, contains('A real document.'));
    expect(find.text('已保存'), findsOneWidget);
  });

  testWidgets('an empty draft starts with an editable paragraph', (
    tester,
  ) async {
    final directory = Directory.systemTemp.createTempSync('appflowy-blank-');
    addTearDown(() => directory.deleteSync(recursive: true));
    final file = File('${directory.path}/draft.md')..writeAsStringSync('');

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AppFlowyMarkdownEditor(
            filePath: file.path,
            initialMarkdown: '',
          ),
        ),
      ),
    );
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 50));

    final editor = tester.widget<AppFlowyEditor>(find.byType(AppFlowyEditor));
    expect(editor.editorState.document.root.children, isNotEmpty);
  });
}
