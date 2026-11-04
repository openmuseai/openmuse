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
    final font = Platform.environment['WORD_COVER_UI_FONT'];
    if (font == null) return;
    final bytes = await File(font).readAsBytes();
    await (FontLoader('WordCoverVisual')
          ..addFont(Future<ByteData>.value(ByteData.sublistView(bytes))))
        .load();
  });

  testWidgets('signed cover gallery retains all twelve previews',
      (tester) async {
    final path = Platform.environment['WORD_COVER_MENU_PATH'];
    final Map cover;
    if (path == null) {
      cover = {
        'id': 'insert.cover.gallery',
        'label': '封面样式库',
        'kind': 'grid',
        'presentation': 'cardGallery',
        'columns': 4,
        'children': [
          for (var i = 1; i <= 12; i++)
            {
              'id': 'insert.cover.$i',
              'label': '封面 $i',
              'previewParas': [
                {'spaceBefore': 2400},
                {
                  'text': '文档标题',
                  'bold': true,
                  'color': i.isEven ? 'FFFFFF' : '2B579A',
                  'shadingFill': i.isEven ? '1F3864' : null,
                  'sizeHalfPoints': 64,
                  'align': 'center',
                },
                {'text': '文档副标题', 'spaceAfter': 4000},
              ],
            },
        ],
      };
    } else {
      final menuJson = await tester.runAsync(() => File(path).readAsString());
      final product = jsonDecode(menuJson!) as Map;
      final menus = product['menus'] as Map;
      cover = (menus['insert.cover'] as List).first as Map;
    }
    final entry = WordMenuEntry.fromJson(Map<String, dynamic>.from(cover));
    expect(entry.children, hasLength(12));
    if (path != null) {
      expect(
          entry.children.every((cell) => cell.previewParas.isNotEmpty), isTrue);
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
    tester.view.physicalSize = const Size(520, 550);
    tester.view.devicePixelRatio = 1;
    final capture = GlobalKey();
    await tester.pumpWidget(MaterialApp(
        theme: ThemeData(fontFamily: 'WordCoverVisual'),
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
    expect(tester.takeException(), isNull);
    final screenshot = Platform.environment['WORD_COVER_SCREENSHOT_PATH'];
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
    tester.view.physicalSize = const Size(393, 800);
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: WordMenuGrid(
      entry: entry,
      chrome: chrome,
      touch: true,
      isCommandAvailable: (_) => false,
      onCommand: (_) {},
    ))));
    await tester.pump();
    final mobileCards = [
      for (final cell in entry.children)
        tester.getRect(find.byKey(Key('word-menu-${cell.id}'))),
    ];
    expect(mobileCards.every((rect) => rect.right <= 393), isTrue);
    expect(mobileCards.map((rect) => rect.top).toSet(), hasLength(4));
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
