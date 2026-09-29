import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets(
    'Markdown Helix banner requests preview in the same editor host',
    (tester) async {
      String? requestedEditor;
      final plugin = OpenMuseHelixPlugin();
      final resource = OpenMuseResource(
        uri: Uri.file('/notes.md'),
        displayName: 'notes.md',
        mediaType: 'text/markdown',
      );
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => plugin.buildEditorBanner(
                context,
                resource,
                canOpenWith: (id) => id == 'viewer.markdown',
                openWith: (id) => requestedEditor = id,
              )!,
            ),
          ),
        ),
      );
      expect(find.text('Preview'), findsOneWidget);
      await tester.tap(find.text('Preview'));
      expect(requestedEditor, 'viewer.markdown');
    },
  );
}
