import 'package:appflowy/plugins/word/word_muse_toolbar.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/word_editor.dart';

void main() {
  testWidgets('P2-T2/T3 Muse toolbar has no vendor Word blue', (tester) async {
    final controller = WordEditorController();
    addTearDown(controller.dispose);
    await tester.pumpWidget(
      MaterialApp(
        theme: ThemeData(useMaterial3: true),
        home: Scaffold(
          body: WordMuseToolbar(controller: controller, canSave: false),
        ),
      ),
    );
    expect(find.text('Save'), findsOneWidget);
    expect(find.text('Open .docx'), findsOneWidget);
    expect(find.text('File'), findsNothing);
    expect(find.text('Home'), findsNothing);
    final save = tester.widget<TextButton>(
      find.widgetWithText(TextButton, 'Save'),
    );
    expect(save.onPressed, isNull);
    expect(find.byWidgetPredicate((widget) {
      if (widget is ColoredBox && widget.color == const Color(0xFF185ABD)) {
        return true;
      }
      if (widget is Container && widget.color == const Color(0xFF185ABD)) {
        return true;
      }
      return false;
    }), findsNothing);
  });
}
