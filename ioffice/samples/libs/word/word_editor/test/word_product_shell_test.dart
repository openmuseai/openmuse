import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:word_editor/word_editor.dart';
import 'package:word_render/word_render.dart' as wr;

void main() {
  test('automatic page fit yields to explicit zoom', () {
    final controller = WordEditorController();
    controller.setAutoFitZoom(0.45);
    expect(controller.zoom, 0.45);
    controller.setZoom(0.8);
    controller.setAutoFitZoom(0.5);
    expect(controller.zoom, 0.8);
    controller.dispose();
  });

  testWidgets(
      'signed-style ribbon switches tabs without dispatching unsupported controls',
      (tester) async {
    tester.view.physicalSize = const Size(900, 600);
    tester.view.devicePixelRatio = 1;
    final controller = WordEditorController();
    final calls = <String>[];
    final definition = WordProductDefinition.fromJson(
      ribbon: {
        'profile': 'openmuse-word-ribbon/2',
        'initialTab': 'home',
        'tabs': [
          {
            'id': 'home',
            'label': '开始',
            'groups': [
              {
                'id': 'font',
                'label': '字体',
                'layout': 'large',
                'items': [
                  {
                    'id': 'bold',
                    'label': '加粗',
                    'kind': 'large',
                    'icon': 'format_bold',
                    'command': 'word.format.bold'
                  },
                  {
                    'id': 'paste',
                    'label': '粘贴',
                    'kind': 'large',
                    'icon': 'content_paste',
                    'menu': 'home.paste'
                  },
                ]
              }
            ]
          },
          {
            'id': 'view',
            'label': '视图',
            'groups': [
              {
                'id': 'zoom',
                'layout': 'large',
                'items': [
                  {
                    'id': 'zoom-in',
                    'label': '放大',
                    'kind': 'large',
                    'icon': 'zoom_in',
                    'command': 'word.view.zoomIn'
                  },
                  {
                    'id': 'read-mode',
                    'label': '阅读视图',
                    'kind': 'large',
                    'icon': 'menu_book',
                    'command': 'word.view.readMode'
                  },
                ]
              }
            ]
          },
        ],
      },
      menus: {
        'profile': 'openmuse-word-menus/1',
        'menus': {
          'home.paste': [
            {
              'id': 'paste.format',
              'label': '粘贴格式',
              'kind': 'submenu',
              'children': [
                {
                  'id': 'paste.source',
                  'label': '保留源格式',
                  'kind': 'submenu',
                  'children': [
                    {
                      'id': 'paste.source.deep',
                      'label': '应用源格式',
                      'command': 'word.view.zoomIn'
                    }
                  ]
                }
              ]
            },
            {'id': 'paste.text', 'label': '只保留文本'},
            {
              'id': 'paste.grid',
              'label': '插入表格',
              'kind': 'grid',
              'columns': 10,
              'children': [
                for (var i = 1; i <= 80; i++)
                  {'id': 'paste.cell.$i', 'label': '$i'}
              ]
            }
          ]
        }
      },
      chrome: {
        'profile': 'openmuse-word-chrome/1',
        'metrics': {
          'tabRowHeight': 36,
          'ribbonHeight': 80,
          'compactRibbonHeight': 45,
          'statusHeight': 24,
          'pageTopInset': 26,
          'pageBottomInset': 140,
          'pageGap': 28
        },
        'responsive': {'mobileStatusHeight': 44},
        'colors': {
          'canvas': '#e7e7e7',
          'chrome': '#ffffff',
          'status': '#f3f4f6',
          'border': '#e1e4e8',
          'accent': '#185abd',
          'text': '#232425',
          'muted': '#666666'
        },
        'quickAccess': [],
        'status': {
          'page': '第 {current} 页，共 {total} 页',
          'wordCount': '{count} 个字',
          'revision': '修订:关',
          'ready': '就绪',
          'readExit': '退出阅读视图 (Esc)',
          'viewModes': [
            {
              'id': 'print',
              'label': '页面视图',
              'icon': 'description',
              'command': 'word.view.printLayout'
            },
            {
              'id': 'read',
              'label': '阅读视图',
              'icon': 'menu_book',
              'command': 'word.view.readMode'
            }
          ]
        },
      },
      icons: {
        'profile': 'openmuse-word-icons/1',
        'icons': {
          'content_paste':
              '<svg viewBox="0 0 16 16" stroke="#000000" stroke-width="__STROKE_WIDTH__" fill="none"><rect x="3" y="3" width="10" height="10"/></svg>',
        },
      },
    );
    await tester.pumpWidget(MaterialApp(
        theme: ThemeData(platform: TargetPlatform.macOS),
        home: Scaffold(
            body: WordProductShell(
          definition: definition,
          controller: controller,
          document:
              const ColoredBox(key: Key('word-document'), color: Colors.white),
          isCommandAvailable: (id) =>
              id == 'word.view.zoomIn' ||
              id == 'word.view.readMode' ||
              id == 'word.view.printLayout',
          onCommand: calls.add,
        ))));
    expect(find.text('开始'), findsOneWidget);
    expect(find.byType(SvgPicture), findsOneWidget);
    expect(find.byKey(const Key('word-document')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-control-bold')));
    await tester.tap(find.byKey(const Key('word-control-paste')));
    await tester.pumpAndSettle();
    expect(find.text('粘贴格式'), findsOneWidget);
    await tester.tap(find.text('粘贴格式'));
    await tester.pumpAndSettle();
    expect(find.text('保留源格式'), findsOneWidget);
    await tester.tap(find.text('保留源格式'));
    await tester.pumpAndSettle();
    expect(find.text('应用源格式'), findsOneWidget);
    expect(calls, isEmpty);
    await tester.sendKeyEvent(LogicalKeyboardKey.escape);
    await tester.sendKeyEvent(LogicalKeyboardKey.escape);
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-tab-view')));
    await tester.pumpAndSettle();
    expect(find.text('放大'), findsOneWidget);
    expect(find.byKey(const Key('word-document')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-control-zoom-in')));
    expect(calls, ['word.view.zoomIn']);
    controller.envelope = wr.RenderEnvelope(
      ok: true,
      pipeline: 'test',
      page: wr.PageMetrics.defaultA4(),
      commands: const [],
    );
    controller.report('ready');
    await tester.pump();
    await tester.tap(find.byKey(const Key('word-control-read-mode')));
    await tester.pumpAndSettle();
    expect(controller.readingMode, isTrue);
    expect(find.byKey(const Key('word-ribbon')), findsNothing);
    expect(find.byKey(const Key('word-document')), findsOneWidget);
    await tester.sendKeyEvent(LogicalKeyboardKey.escape);
    await tester.pumpAndSettle();
    expect(controller.readingMode, isFalse);
    expect(find.byKey(const Key('word-ribbon')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-control-read-mode')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-read-exit')));
    await tester.pumpAndSettle();
    expect(controller.readingMode, isFalse);
    controller.envelope = null;
    controller.report('idle');
    await tester.pump();
    for (final (width, ribbonHeight) in [
      (1365.0, 80.0),
      (768.0, 64.0),
      (600.0, 52.0),
      (390.0, 45.0),
      (320.0, 45.0),
    ]) {
      tester.view.physicalSize = Size(width, 600);
      await tester.pump();
      expect(tester.getSize(find.byKey(const Key('word-ribbon'))).height,
          ribbonHeight);
      expect(tester.takeException(), isNull);
    }
    expect(tester.getSize(find.byKey(const Key('word-status'))).height, 44);
    await tester.tap(find.byKey(const Key('word-tab-home')));
    await tester.pump();
    expect(tester.getSize(find.byKey(const Key('word-control-paste'))).height,
        greaterThanOrEqualTo(44));
    await tester.tap(find.byKey(const Key('word-control-paste')));
    await tester.pumpAndSettle();
    expect(find.text('插入表格'), findsOneWidget);
    expect(find.byKey(const Key('word-menu-level-0')), findsOneWidget);
    expect(
        tester.getSize(find.byKey(const Key('word-menu-paste.cell.1'))).width,
        greaterThanOrEqualTo(44));
    await tester.tap(find.byKey(const Key('word-menu-paste.format')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('word-menu-level-1')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-menu-paste.source')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('word-menu-level-2')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-menu-back')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('word-menu-level-1')), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-menu-paste.source')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-menu-paste.source.deep')));
    await tester.pumpAndSettle();
    expect(calls, ['word.view.zoomIn', 'word.view.zoomIn']);
    expect(find.byKey(const Key('word-menu-level-2')), findsNothing);
    expect(tester.takeException(), isNull);
    tester.view.physicalSize = const Size(1365, 600);
    await tester.pump();
    expect(find.text('字体'), findsNothing);
    await tester.pumpWidget(MaterialApp(
        theme: ThemeData(platform: TargetPlatform.windows),
        home: Scaffold(
            body: WordProductShell(
          definition: definition,
          controller: controller,
          document:
              const ColoredBox(key: Key('word-document'), color: Colors.white),
          isCommandAvailable: (id) =>
              id == 'word.view.zoomIn' ||
              id == 'word.view.readMode' ||
              id == 'word.view.printLayout',
          onCommand: calls.add,
        ))));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('word-control-paste')), findsOneWidget);
    expect(Theme.of(tester.element(find.byType(WordProductShell))).platform,
        TargetPlatform.windows);
    expect(tester.getSize(find.byKey(const Key('word-ribbon'))).height, 80);
    expect(find.text('字体'), findsOneWidget);
    controller.dispose();
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
