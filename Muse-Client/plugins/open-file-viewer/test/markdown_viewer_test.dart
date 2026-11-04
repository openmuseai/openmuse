import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:openmuse_file_viewer/openmuse_file_viewer.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test('video files are claimed by the local viewer', () {
    final plugin = OpenMuseFileViewerPlugin();
    final resource = OpenMuseResource(
      uri: Uri.file('/tmp/openmuse-vs-dsh-20s.mp4'),
      displayName: 'openmuse-vs-dsh-20s.mp4',
      mediaType: 'video/mp4',
    );
    expect(
      plugin.descriptor.editors.any(
        (editor) => editor.id == 'viewer.video' && editor.accepts(resource),
      ),
      isTrue,
    );
  });

  test('reads the authorized local Markdown file', () async {
    final directory = Directory.systemTemp.createTempSync('openmuse-md-read-');
    addTearDown(() => directory.deleteSync(recursive: true));
    final file = File('${directory.path}/note.md');
    file.writeAsStringSync('# Read from disk');
    expect(await loadOpenMuseMarkdownFile(file.path), '# Read from disk');
  });

  testWidgets(
    'Markdown chosen as default Viewer reaches the Markdown renderer',
    (tester) async {
      final directory = Directory.systemTemp.createTempSync('openmuse-md-');
      addTearDown(() => directory.deleteSync(recursive: true));
      final file = File('${directory.path}/README.md');
      file.writeAsStringSync('# Viewer title\n\nA **local** document.');
      final resource = OpenMuseResource(
        uri: file.uri,
        displayName: 'README.md',
        mediaType: 'text/markdown',
      );
      final plugin = OpenMuseFileViewerPlugin();
      expect(plugin.descriptor.editors.first.accepts(resource), isTrue);
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => plugin.buildEditor(context, resource),
            ),
          ),
        ),
      );
      expect(find.byType(OpenMuseMarkdownPreview), findsOneWidget);
    },
  );

  testWidgets('Markdown renderer displays selectable content', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OpenMuseMarkdownPreview(
            path: '/unused-for-injected-content.md',
            contents: Future<String>.value(
              '# Viewer title\n\nA **local** document.',
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    final rendered = tester.widget<Markdown>(find.byType(Markdown));
    expect(rendered.data, contains('# Viewer title'));
    expect(rendered.data, contains('**local**'));
    expect(rendered.selectable, isTrue);
  });

  testWidgets('Markdown preview banner offers Edit through the host callback', (
    tester,
  ) async {
    String? requestedEditor;
    final plugin = OpenMuseFileViewerPlugin();
    final resource = OpenMuseResource(
      uri: Uri(path: '/notes.md', scheme: 'file'),
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
              canOpenWith: (id) => id == 'helix.editor',
              openWith: (id) => requestedEditor = id,
            )!,
          ),
        ),
      ),
    );
    expect(find.text('Edit'), findsOneWidget);
    await tester.tap(find.text('Edit'));
    expect(requestedEditor, 'helix.editor');
  });
}
