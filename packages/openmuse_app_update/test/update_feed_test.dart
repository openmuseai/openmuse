import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:pub_semver/pub_semver.dart';

void main() {
  final origin = Uri.parse('http://127.0.0.1:3080');
  final release = {
    'schema': 'openmuse.app-update/v1',
    'channel': 'canary',
    'target': 'macos-universal',
    'version': '1.0.1',
    'build': 2,
    'publishedAt': '2026-10-13T00:00:00Z',
    'downloadUrl':
        'http://127.0.0.1:3080/downloads/updates/v1/canary/OpenMuse-1.0.1-macos-universal.zip',
    'sha256': 'a' * 64,
    'sizeBytes': 123,
    'installKind': 'macos-zip',
    'mandatory': false,
    'notes': {'zh': '修复插件安装。', 'en': 'Plugin installation fixes.'},
  };

  test('checks exact channel, target, version and download identity', () async {
    final feed = OpenMuseUpdateFeed(
      origin: origin,
      channel: 'canary',
      target: 'macos-universal',
      allowInsecureLoopback: true,
      fetchOverride: (_) async => jsonEncode(release),
    );
    final controller = OpenMuseUpdateController(
      feed: feed,
      installedVersion: Version.parse('1.0.0'),
      installedBuild: 1,
    );
    await controller.check();
    expect(controller.available?.version, Version.parse('1.0.1'));
    expect(controller.available?.notesFor('zh'), '修复插件安装。');
    controller.dispose();

    final wrong = OpenMuseUpdateFeed(
      origin: origin,
      channel: 'stable',
      target: 'macos-universal',
      allowInsecureLoopback: true,
      fetchOverride: (_) async => jsonEncode(release),
    );
    expect(wrong.fetch(), throwsFormatException);
    final older = OpenMuseUpdateController(
      feed: feed,
      installedVersion: Version.parse('1.0.1'),
      installedBuild: 2,
    );
    await older.check();
    expect(older.available, isNull);
    older.dispose();
  });

  test('update feed and package keep a configured gateway base path', () async {
    final prefixed = {
      ...release,
      'downloadUrl': 'https://updates.example/gateway/downloads/package.zip',
    };
    final feed = OpenMuseUpdateFeed(
      origin: Uri.parse('https://updates.example/gateway/'),
      channel: 'canary',
      target: 'macos-universal',
      fetchOverride: (uri) async {
        expect(
          uri.path,
          '/gateway/downloads/updates/v1/canary/macos-universal.json',
        );
        return jsonEncode(prefixed);
      },
    );
    expect(
      (await feed.fetch())?.downloadUri.path,
      '/gateway/downloads/package.zip',
    );
    prefixed['downloadUrl'] = 'https://updates.example/downloads/package.zip';
    await expectLater(feed.fetch(), throwsFormatException);
  });

  testWidgets('launch update dialog shows notes and dismisses', (tester) async {
    final feed = OpenMuseUpdateFeed(
      origin: origin,
      channel: 'canary',
      target: 'macos-universal',
      allowInsecureLoopback: true,
      fetchOverride: (_) async => jsonEncode(release),
    );
    final controller = OpenMuseUpdateController(
      feed: feed,
      installedVersion: Version.parse('1.0.0'),
      installedBuild: 1,
    );
    await controller.check();
    await tester.pumpWidget(
      MaterialApp(
        home: OpenMuseStartupUpdatePrompt(
          controller: controller,
          onUpdate: (_) async {},
          child: const Scaffold(body: Text('Main application')),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('app-update-dialog')), findsOneWidget);
    expect(find.text('Plugin installation fixes.'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('app-update-later')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('app-update-dialog')), findsNothing);
    controller.dispose();
  });
}
