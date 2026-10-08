import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';

void main() {
  testWidgets('authorized Markdown bytes use the shared preview renderer', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OpenMuseFileViewerBody(
            resourceName: 'readme.md',
            readBytes: () async => Uint8List.fromList(utf8.encode('# Shared')),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byType(Markdown), findsOneWidget);
    expect(find.text('Shared'), findsOneWidget);
  });
}
