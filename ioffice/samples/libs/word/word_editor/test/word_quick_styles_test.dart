import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/word_editor.dart';

void main() {
  test('versioned Rust snapshot keeps document style IDs and flags', () {
    final snapshot = WordStyleSnapshot.fromEngineJson('''{
      "schemaVersion":1,"styles":[
        {"styleId":"1","name":"Normal","type":"paragraph","isDefault":true,"qFormat":true},
        {"styleId":"LocalTitle","name":"Local Title","type":"paragraph","qFormat":true,
         "semiHidden":true,"unhideWhenUsed":true,"uiPriority":3,"sizeHalfPoints":36,"color":"FF0000","bold":true}
      ],"usedStyleIds":["LocalTitle"],"defaultSizeHalfPoints":24
    }''');
    expect(snapshot.quickStyles.map((style) => style.styleId),
        ['1', 'LocalTitle']);
    expect(snapshot.quickStyles.last.sizeHalfPoints, 36);
    expect(snapshot.defaultSizeHalfPoints, 24);
  });

  test('quick styles follow GenOffice filtering and priority rules', () {
    const normal = WordStyleInfo(
        styleId: 'Normal', name: 'Normal', type: 'paragraph', isDefault: true);
    const heading = WordStyleInfo(
        styleId: 'Heading1',
        name: 'heading 1',
        type: 'paragraph',
        qFormat: true,
        uiPriority: 9);
    const hidden = WordStyleInfo(
        styleId: 'Heading3',
        name: 'heading 3',
        type: 'paragraph',
        qFormat: true,
        semiHidden: true,
        unhideWhenUsed: true,
        uiPriority: 9);
    const linked = WordStyleInfo(
        styleId: 'Heading1Char',
        name: 'heading 1 char',
        type: 'character',
        qFormat: true,
        linkedCharShell: true);
    const ignored = WordStyleInfo(
        styleId: 'TableGrid', name: 'Table Grid', type: 'table', qFormat: true);
    const character = WordStyleInfo(
        styleId: 'Emphasis',
        name: 'Emphasis',
        type: 'character',
        qFormat: true,
        uiPriority: 5);
    const snapshot = WordStyleSnapshot(
      styles: [hidden, heading, normal, linked, ignored, character],
      usedStyleIds: {'Heading3'},
    );
    expect(snapshot.quickStyles.map((s) => s.styleId),
        ['Normal', 'Emphasis', 'Heading1', 'Heading3']);
    expect(
      const WordStyleSnapshot(styles: [normal, heading, hidden])
          .quickStyles
          .map((s) => s.styleId),
      ['Normal', 'Heading1'],
    );
    expect(
      const WordStyleSnapshot(styles: [normal, heading, hidden])
          .quickStyles
          .first
          .isDefault,
      isTrue,
    );
  });

  testWidgets(
      'snapshot replaces signed fallback cards without changing document',
      (tester) async {
    tester.view.physicalSize = const Size(1365, 600);
    tester.view.devicePixelRatio = 1;
    final controller = WordEditorController();
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
                'id': 'styles',
                'label': '样式',
                'layout': 'gallery',
                'gallerySource': 'document.quickStyles',
                'items': [
                  {
                    'id': 'styles-pane',
                    'label': '样式窗格',
                    'kind': 'large',
                    'icon': 'article'
                  }
                ]
              }
            ]
          }
        ]
      },
      menus: {
        'profile': 'openmuse-word-menus/1',
        'menus': {
          'home.styles': [
            {
              'id': 'home.styles.gallery',
              'label': '样式库',
              'kind': 'grid',
              'columns': 5,
              'presentation': 'styleGallery',
              'children': [
                {
                  'id': 'home.styles.cell.1',
                  'label': '正文',
                  'styleInfo': {
                    'styleId': 'Normal',
                    'name': 'Normal',
                    'type': 'paragraph',
                    'isDefault': true,
                    'qFormat': true
                  }
                },
                {
                  'id': 'home.styles.cell.2',
                  'label': '标题 1',
                  'styleInfo': {
                    'styleId': 'Heading1',
                    'name': 'heading 1',
                    'type': 'paragraph',
                    'qFormat': true
                  }
                },
                {
                  'id': 'home.styles.cell.3',
                  'label': '标题 2',
                  'styleInfo': {
                    'styleId': 'Heading2',
                    'name': 'heading 2',
                    'type': 'paragraph',
                    'qFormat': true
                  }
                },
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
          'viewModes': []
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
        isCommandAvailable: (_) => false,
        onCommand: (_) {},
      )),
    ));
    expect(find.byKey(const Key('word-inline-style-Normal')), findsOneWidget);
    controller.setStyleSnapshot(const WordStyleSnapshot(styles: [
      WordStyleInfo(
          styleId: 'Custom',
          name: 'My Custom',
          type: 'paragraph',
          qFormat: true),
    ]));
    await tester.pump();
    expect(find.byKey(const Key('word-inline-style-Custom')), findsOneWidget);
    expect(find.byKey(const Key('word-inline-style-Normal')), findsNothing);
    expect(find.byKey(const Key('word-document')), findsOneWidget);
    controller.dispose();
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
