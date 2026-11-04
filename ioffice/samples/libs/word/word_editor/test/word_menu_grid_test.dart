import 'dart:ui' show PointerDeviceKind;

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/src/word_menu_grid.dart';
import 'package:word_editor/word_editor.dart';

void main() {
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

  testWidgets('source drop cap and equation galleries fit phone menus',
      (tester) async {
    tester.view.physicalSize = const Size(320, 800);
    tester.view.devicePixelRatio = 1;
    final drop = WordMenuEntry.fromJson({
      'id': 'dropcap.gallery',
      'label': '首字下沉',
      'kind': 'grid',
      'presentation': 'dropCapGallery',
      'columns': 3,
      'children': [
        {'id': 'dropcap.none', 'label': '无', 'previewText': '—'},
        {'id': 'dropcap.dropped', 'label': '下沉', 'previewText': 'A'},
        {'id': 'dropcap.margin', 'label': '悬挂', 'previewText': 'A'},
      ],
    });
    final equations = WordMenuEntry.fromJson({
      'id': 'equation.gallery',
      'label': '内置公式',
      'kind': 'grid',
      'presentation': 'equationGallery',
      'columns': 1,
      'children': [
        for (var i = 1; i <= 8; i++)
          {
            'id': 'equation.style.$i',
            'label': '公式 $i',
            'previewText': 'a² + b² = c²'
          },
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: ListView(children: [
      WordMenuGrid(
          entry: drop,
          chrome: chrome,
          touch: true,
          isCommandAvailable: (_) => false,
          onCommand: (_) {}),
      WordMenuGrid(
          entry: equations,
          chrome: chrome,
          touch: true,
          isCommandAvailable: (_) => false,
          onCommand: (_) {}),
    ]))));
    expect(find.byKey(const Key('word-menu-dropcap.margin')), findsOneWidget);
    expect(find.byKey(const Key('word-menu-equation.style.8')), findsOneWidget);
    expect(
        tester.getSize(find.byKey(const Key('word-menu-dropcap.none'))).width,
        72);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets(
      'table picker previews a rectangle and only dispatches available cells',
      (tester) async {
    tester.view.physicalSize = const Size(600, 700);
    tester.view.devicePixelRatio = 1;
    final calls = <String>[];
    final entry = WordMenuEntry.fromJson({
      'id': 'table.grid',
      'label': '插入表格',
      'kind': 'grid',
      'presentation': 'tablePicker',
      'columns': 10,
      'children': [
        for (var row = 1; row <= 8; row++)
          for (var col = 1; col <= 10; col++)
            {
              'id': 'table.$row.$col',
              'label': '$col × $row',
              'command': 'word.insert.table.$row.$col'
            }
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: WordMenuGrid(
                entry: entry,
                chrome: chrome,
                touch: false,
                isCommandAvailable: (id) => id == 'word.insert.table.2.3',
                onCommand: calls.add))));
    expect(find.byKey(const Key('word-menu-table.8.10')), findsOneWidget);
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer();
    await gesture
        .moveTo(tester.getCenter(find.byKey(const Key('word-menu-table.2.3'))));
    await tester.pump();
    expect(find.text('3 × 2 表格'), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-menu-table.1.1')));
    expect(calls, isEmpty);
    await tester.tap(find.byKey(const Key('word-menu-table.2.3')));
    expect(calls, ['word.insert.table.2.3']);
    await gesture.removePointer();
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('touch palette remains scrollable at phone width',
      (tester) async {
    tester.view.physicalSize = const Size(320, 700);
    tester.view.devicePixelRatio = 1;
    final entry = WordMenuEntry.fromJson({
      'id': 'font.theme',
      'label': '主题颜色',
      'kind': 'grid',
      'presentation': 'colorPalette',
      'columns': 10,
      'children': [
        for (var i = 1; i <= 60; i++)
          {'id': 'font.color.$i', 'label': '色 $i', 'swatch': '#156082'}
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: WordMenuGrid(
                entry: entry,
                chrome: chrome,
                touch: true,
                isCommandAvailable: (_) => false,
                onCommand: (_) {}))));
    expect(find.byKey(const Key('word-menu-font.color.60')), findsOneWidget);
    expect(
        tester.getSize(find.byKey(const Key('word-menu-font.color.1'))).width,
        44);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('shape gallery keeps all source presets reachable on a foldable',
      (tester) async {
    tester.view.physicalSize = const Size(480, 700);
    tester.view.devicePixelRatio = 1;
    final entry = WordMenuEntry.fromJson({
      'id': 'shapes.basic',
      'label': '基本形状',
      'kind': 'grid',
      'presentation': 'shapeGallery',
      'columns': 8,
      'children': [
        for (var i = 1; i <= 37; i++)
          {
            'id': 'shapes.basic.$i',
            'label': '形状 $i',
            'previewPath':
                i.isEven ? 'M 0 18 C 9 18 9 0 18 0 Z' : 'M 9 0 L 18 18 L 0 18 Z'
          }
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: WordMenuGrid(
                entry: entry,
                chrome: chrome,
                touch: true,
                isCommandAvailable: (_) => false,
                onCommand: (_) {}))));
    expect(find.byKey(const Key('word-menu-shapes.basic.37')), findsOneWidget);
    expect(
        tester.getSize(find.byKey(const Key('word-menu-shapes.basic.1'))).width,
        44);
    expect(
        find.descendant(
            of: find.byKey(const Key('word-menu-shapes.basic.1')),
            matching: find.byType(CustomPaint)),
        findsOneWidget);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('cover cards reflow to two columns on a narrow phone',
      (tester) async {
    tester.view.physicalSize = const Size(320, 700);
    tester.view.devicePixelRatio = 1;
    final entry = WordMenuEntry.fromJson({
      'id': 'covers',
      'label': '封面',
      'kind': 'grid',
      'presentation': 'cardGallery',
      'columns': 4,
      'children': [
        for (var i = 1; i <= 12; i++) {'id': 'cover.$i', 'label': '封面 $i'}
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: ListView(children: [
      WordMenuGrid(
          entry: entry,
          chrome: chrome,
          touch: true,
          isCommandAvailable: (_) => false,
          onCommand: (_) {})
    ]))));
    expect(find.byKey(const Key('word-menu-cover.12')), findsOneWidget);
    expect(
        tester.getSize(find.byKey(const Key('word-menu-cover.1'))).width, 120);
    final cards = [
      for (var i = 1; i <= 12; i++)
        tester.getRect(find.byKey(Key('word-menu-cover.$i'))),
    ];
    expect(cards.every((rect) => rect.right <= 320), isTrue);
    expect(cards.map((rect) => rect.top).toSet(), hasLength(6));
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('position picker draws nine source positions on phone',
      (tester) async {
    tester.view.physicalSize = const Size(320, 700);
    tester.view.devicePixelRatio = 1;
    final entry = WordMenuEntry.fromJson({
      'id': 'position.grid',
      'label': '文字环绕位置',
      'kind': 'grid',
      'presentation': 'positionPicker',
      'columns': 3,
      'children': [
        for (final row in ['top', 'center', 'bottom'])
          for (var column = 1; column <= 3; column++)
            {'id': 'position.$row.$column', 'label': '$row $column'}
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: WordMenuGrid(
                entry: entry,
                chrome: chrome,
                touch: true,
                isCommandAvailable: (_) => false,
                onCommand: (_) {}))));
    for (final row in ['top', 'center', 'bottom']) {
      for (var column = 1; column <= 3; column++) {
        expect(find.byKey(Key('word-position-preview-position.$row.$column')),
            findsOneWidget);
      }
    }
    expect(
        tester.getSize(find.byKey(const Key('word-menu-position.top.1'))).width,
        52);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('list gallery and level row fit a desktop menu', (tester) async {
    tester.view.physicalSize = const Size(600, 700);
    tester.view.devicePixelRatio = 1;
    final gallery = WordMenuEntry.fromJson({
      'id': 'bullets.gallery',
      'label': '项目符号库',
      'kind': 'grid',
      'presentation': 'bulletGallery',
      'columns': 3,
      'children': [
        {'id': 'bullets.none', 'label': '无'},
        for (var i = 1; i <= 6; i++) {'id': 'bullets.glyph.$i', 'label': '•'}
      ],
    });
    final levels = WordMenuEntry.fromJson({
      'id': 'bullets.levels',
      'label': '更改列表级别',
      'kind': 'grid',
      'presentation': 'listLevels',
      'columns': 9,
      'children': [
        for (var i = 1; i <= 9; i++) {'id': 'bullets.level.$i', 'label': '$i'}
      ],
    });
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: Column(children: [
      WordMenuGrid(
          entry: gallery,
          chrome: chrome,
          touch: false,
          isCommandAvailable: (_) => false,
          onCommand: (_) {}),
      WordMenuGrid(
          entry: levels,
          chrome: chrome,
          touch: false,
          isCommandAvailable: (_) => false,
          onCommand: (_) {}),
    ]))));
    expect(find.byKey(const Key('word-menu-bullets.glyph.6')), findsOneWidget);
    expect(find.byKey(const Key('word-menu-bullets.level.9')), findsOneWidget);
    expect(
        tester
            .getSize(find.byKey(const Key('word-menu-bullets.level.1')))
            .width,
        22);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
