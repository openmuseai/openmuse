import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_file_viewer/openmuse_file_viewer.dart';
import 'package:openmuse_mobile/host/mobile_file_viewer.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  for (final name in ['README.md', 'main.dart', 'config.json', 'Makefile']) {
    testWidgets('$name opens through the compiled viewer and cleans its copy', (
      tester,
    ) async {
      final cache = Directory.systemTemp.createTempSync('mobile-viewer-test-');
      addTearDown(() => cache.deleteSync(recursive: true));
      final navigator = GlobalKey<NavigatorState>();
      await tester.pumpWidget(
        MaterialApp(
          navigatorKey: navigator,
          home: const Scaffold(body: Text('Workspace')),
        ),
      );
      final plugin = OpenMuseFileViewerPlugin();
      final resource = OpenMuseResource(
        uri: Uri.parse('workspace:/$name'),
        displayName: name,
        mediaType: 'application/octet-stream',
      );
      expect(plugin.canPreviewText(resource), isTrue);
      late Future<bool> opened;
      await tester.runAsync(() async {
        opened = openMobileFileViewer(
          navigator: navigator.currentState!,
          plugin: plugin,
          resource: resource,
          bytes: Uint8List.fromList(utf8.encode('viewer contents')),
          cacheRoot: cache,
        );
        // Wait for the actual file write; the route remains open.
        for (var i = 0; i < 100 && !navigator.currentState!.canPop(); i++) {
          await Future<void>.delayed(const Duration(milliseconds: 10));
        }
      });
      await tester.runAsync(() async {
        await tester.pump(const Duration(milliseconds: 350));
        await Future<void>.delayed(const Duration(milliseconds: 100));
        await tester.pump();
      });
      await tester.pumpAndSettle();
      expect(find.text(name), findsOneWidget);
      expect(find.text('viewer contents'), findsOneWidget);
      expect(cache.listSync(), hasLength(1));
      navigator.currentState!.pop();
      await tester.pumpAndSettle();
      await tester.runAsync(() async {
        expect(await opened, isTrue);
      });
      expect(cache.listSync(), isEmpty);
    });
  }
}
