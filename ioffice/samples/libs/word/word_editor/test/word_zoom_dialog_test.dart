import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/src/word_zoom_dialog.dart';
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
      'zoom': {
        'minPercent': 10,
        'maxPercent': 500,
        'stepPercent': 10,
        'presets': [200, 100, 75]
      },
    },
  });

  testWidgets('signed zoom presets apply without a plugin command',
      (tester) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    final applied = <double>[];
    final fitted = <WordZoomFitMode>[];
    await tester.pumpWidget(MaterialApp(
        home: Scaffold(
            body: Builder(
      builder: (context) => TextButton(
        onPressed: () => showDialog<void>(
            context: context,
            builder: (_) => WordZoomDialog(
                chrome: chrome,
                initialZoom: 1,
                initialFitMode: null,
                onApply: applied.add,
                onFit: fitted.add)),
        child: const Text('打开'),
      ),
    ))));
    await tester.tap(find.text('打开'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('word-zoom-dialog')), findsOneWidget);
    expect(find.text('页宽'), findsOneWidget);
    expect(find.text('文字宽度'), findsOneWidget);
    expect(find.text('整页'), findsOneWidget);
    expect(find.text('多页'), findsOneWidget);
    await tester.tap(find.byKey(const Key('word-zoom-choice-0')));
    await tester.tap(find.byKey(const Key('word-zoom-apply')));
    await tester.pumpAndSettle();
    expect(applied, [2.0]);

    await tester.tap(find.text('打开'));
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const Key('word-zoom-percent')), '900');
    await tester.tap(find.byKey(const Key('word-zoom-apply')));
    await tester.pumpAndSettle();
    expect(applied, [2.0, 5.0]);

    await tester.tap(find.text('打开'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-zoom-choice-1')));
    await tester.tap(find.byKey(const Key('word-zoom-apply')));
    await tester.pumpAndSettle();
    expect(fitted, [WordZoomFitMode.pageWidth]);

    await tester.tap(find.text('打开'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-zoom-choice-5')));
    await tester.tap(find.byKey(const Key('word-zoom-apply')));
    await tester.pumpAndSettle();
    expect(fitted, [WordZoomFitMode.pageWidth, WordZoomFitMode.wholePage]);
    await tester.tap(find.text('打开'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('word-zoom-choice-6')));
    await tester.tap(find.byKey(const Key('word-zoom-apply')));
    await tester.pumpAndSettle();
    expect(fitted, [
      WordZoomFitMode.pageWidth,
      WordZoomFitMode.wholePage,
      WordZoomFitMode.multiplePages
    ]);
    expect(tester.takeException(), isNull);
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
