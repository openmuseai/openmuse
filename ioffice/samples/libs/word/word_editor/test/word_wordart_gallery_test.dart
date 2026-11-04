import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/src/word_menu_grid.dart';
import 'package:word_editor/word_editor.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    final font = Platform.environment['WORD_WORDART_UI_FONT'];
    if (font == null) return;
    final bytes = await File(font).readAsBytes();
    await (FontLoader('WordArtVisual')
          ..addFont(Future<ByteData>.value(ByteData.sublistView(bytes))))
        .load();
  });

  testWidgets('signed WordArt gallery retains all twelve source presets',
      (tester) async {
    final path = Platform.environment['WORD_WORDART_MENU_PATH'];
    final Map gallery;
    if (path == null) {
      gallery = {
        'id': 'insert.wordArt.gallery',
        'label': '艺术字样式库',
        'kind': 'grid',
        'presentation': 'wordArtGallery',
        'columns': 4,
        'children': [
          for (var i = 1; i <= 12; i++)
            {
              'id': 'insert.wordArt.$i',
              'label': '艺术字 $i',
              'swatch': i.isEven ? '#FFFFFF' : '#4472C4',
            },
        ],
      };
    } else {
      final raw = await tester.runAsync(() => File(path).readAsString());
      final product = jsonDecode(raw!) as Map;
      gallery = (product['menus']['insert.wordArt'] as List).first as Map;
    }
    final entry = WordMenuEntry.fromJson(Map<String, dynamic>.from(gallery));
    expect(entry.children, hasLength(12));
    if (path != null) {
      expect(
          entry.children.every((cell) => cell.previewWordArt != null), isTrue);
      expect(entry.children[4].previewWordArt!.italic, isTrue);
      expect(entry.children[5].previewWordArt!.outlineWidthEmu, 19050);
      expect(entry.children[11].previewWordArt!.outlineColor, '#595959');
    }
    final chrome = WordChromeDefinition.fromJson({
      'profile': 'openmuse-word-chrome/1',
      'metrics': {
        'tabRowHeight': 36,
        'ribbonHeight': 80,
        'compactRibbonHeight': 45,
        'statusHeight': 24,
        'pageTopInset': 26,
        'pageBottomInset': 140,
        'pageGap': 28,
      },
      'colors': {
        'canvas': '#e7e7e7',
        'chrome': '#ffffff',
        'status': '#f3f4f6',
        'border': '#e1e4e8',
        'accent': '#185abd',
        'text': '#232425',
        'muted': '#666666',
      },
      'quickAccess': [],
      'status': {
        'page': '第 {current} 页，共 {total} 页',
        'wordCount': '{count} 个字',
        'revision': '修订:关',
        'ready': '就绪',
        'viewModes': [],
      },
    });
    tester.view.physicalSize = const Size(440, 380);
    tester.view.devicePixelRatio = 2;
    final capture = GlobalKey();
    await tester.pumpWidget(MaterialApp(
        theme: ThemeData(fontFamily: 'WordArtVisual'),
        home: RepaintBoundary(
            key: capture,
            child: Scaffold(
                body: Align(
                    alignment: Alignment.topLeft,
                    child: WordMenuGrid(
                      entry: entry,
                      chrome: chrome,
                      touch: false,
                      isCommandAvailable: (_) => false,
                      onCommand: (_) {},
                    ))))));
    await tester.pump();
    expect(
        find.byKey(Key('word-menu-${entry.children.last.id}')), findsOneWidget);
    if (path != null) {
      expect(find.byKey(Key('word-wordart-preview-${entry.children.last.id}')),
          findsOneWidget);
    }
    expect(tester.takeException(), isNull);
    final screenshot = Platform.environment['WORD_WORDART_SCREENSHOT_PATH'];
    if (screenshot != null) {
      final boundary =
          capture.currentContext!.findRenderObject()! as RenderRepaintBoundary;
      await tester.runAsync(() async {
        final image = await boundary.toImage(pixelRatio: 1);
        final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(screenshot).writeAsBytes(bytes!.buffer.asUint8List());
        image.dispose();
      });
    }
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
