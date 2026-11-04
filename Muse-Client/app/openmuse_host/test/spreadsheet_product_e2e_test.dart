import 'dart:convert';
import 'dart:io';
import 'dart:math' as math;
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_spreadsheet_product/openmuse_spreadsheet_product.dart';
import 'package:spreadsheet_editor/spreadsheet_editor.dart';

import 'package:openmuse_host/src/host/spreadsheet_product_plugin.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  final env = Platform.environment;
  final bundlePath = env['SPREADSHEET_PRODUCT_E2E_BUNDLE'];
  final keyPath = env['SPREADSHEET_PRODUCT_E2E_PUBLIC_KEY'];
  final keyId = env['SPREADSHEET_PRODUCT_E2E_KEY_ID'];
  final screenshotDir = env['SPREADSHEET_PRODUCT_E2E_SCREENSHOT_DIR'];
  final enabled =
      bundlePath != null &&
      keyPath != null &&
      keyId != null &&
      env['OPENMUSE_GRIDCRAFT_DYLIB'] != null;
  var fontsReady = false;
  Future<void> ensureScreenshotFonts() async {
    if (screenshotDir == null || fontsReady) return;
    Future<void> registerFont(String family, String path) async {
      final bytes = await File(path).readAsBytes();
      await (FontLoader(
        family,
      )..addFont(Future<ByteData>.value(ByteData.sublistView(bytes)))).load();
    }

    await registerFont(
      'SpreadsheetE2E',
      env['SPREADSHEET_PRODUCT_E2E_UI_FONT']!,
    );
    await registerFont(
      'MaterialIcons',
      env['SPREADSHEET_PRODUCT_E2E_ICON_FONT']!,
    );
    await Directory(screenshotDir).create(recursive: true);
    fontsReady = true;
  }

  testWidgets('signed Spreadsheet product opens, renders, edits and exports XLSX', (
    tester,
  ) async {
    if (!enabled) return;
    final fixture = File('test/fixtures/spreadsheet-shell.xlsx').absolute;
    late OpenMuseSpreadsheetProductPlugin plugin;
    await tester.runAsync(() async {
      expect(await fixture.exists(), isTrue);
      stdout.writeln('Spreadsheet E2E: verifying bundle');
      final bytes = Uint8List.fromList(await File(bundlePath).readAsBytes());
      final key = await File(keyPath).readAsString();
      final verified = await VerifiedSpreadsheetProduct.loadBundle(
        bundle: bytes,
        trustedPublicKeyPem: key,
        expectedKeyId: keyId,
        expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
      );
      expect(verified.chrome.tabs, hasLength(11));
      expect(verified.chrome.quickAccess, hasLength(5));
      expect(verified.chrome.menus[verified.chrome.sheetTabMenu], isNotEmpty);
      expect(verified.chrome.statusStatistics, hasLength(6));
      expect(verified.chrome.statusViews.map((view) => view.value), [
        'normal',
        'pageBreakPreview',
      ]);
      expect(verified.chrome.metric('statusZoomSliderWidth'), 110);
      expect(verified.chrome.metric('rowHeaderWidth'), 46);
      expect(verified.chrome.metric('columnHeaderHeight'), 20);
      expect(verified.chrome.zoomDialog.presets, [200, 100, 75, 50, 25]);
      expect(
        verified.chrome.zoomDialog.fitSelectionCommand,
        'view.zoomToSelection',
      );
      expect(verified.chrome.visualActions?.move, 'object.move');
      expect(verified.chrome.visualActions?.resize, 'object.move');
      expect(verified.chrome.visualActions?.delete, 'object.delete');
      expect(verified.chrome.showRibbonGroupCaptions, isFalse);
      expect(verified.chrome.metric('ribbonLargeWidth'), 45);
      final insert = verified.chrome.tabs.firstWhere(
        (tab) => tab.id == 'insert',
      );
      final charts = insert.groups.firstWhere((group) => group.id == 'charts');
      expect(charts.items, hasLength(11));
      expect(
        verified.chrome.menus['shape']!
            .firstWhere((node) => node.label == '椭圆')
            .params['kind'],
        'ellipse',
      );
      expect(charts.layout.any((block) => block.type == 'grid'), isTrue);
      expect(
        charts.items.firstWhere((item) => item.id == 'pie').params['type'],
        'pie',
      );
      expect(
        verified.chrome.tabs.every(
          (tab) => tab.groups.every((group) => group.layout.isNotEmpty),
        ),
        isTrue,
      );
      stdout.writeln(
        'Spreadsheet E2E: verified ${verified.chrome.tabs.length} tabs',
      );
      expect(verified.chrome.menus.length, greaterThan(40));
      final tampered = Uint8List.fromList(bytes)..[bytes.length ~/ 2] ^= 1;
      stdout.writeln('Spreadsheet E2E: checking tamper');
      await expectLater(
        VerifiedSpreadsheetProduct.loadBundle(
          bundle: tampered,
          trustedPublicKeyPem: key,
          expectedKeyId: keyId,
        ),
        throwsFormatException,
      );
      plugin = await OpenMuseSpreadsheetProductPlugin.loadBundled(
        bundleFile: File(bundlePath),
        trustedPublicKeyPem: key,
        expectedKeyId: keyId,
        expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
      );
      final engine = GridcraftEngine();
      stdout.writeln('Spreadsheet E2E: opening native workbook');
      try {
        engine.open('spreadsheet-shell.xlsx', await fixture.readAsBytes());
        final snapshot = engine.snapshot(rows: 8, cols: 5);
        expect(
          (snapshot['cells'] as List).any((cell) => cell['text'] == 'Region'),
          isTrue,
        );
        expect(
          (snapshot['cells'] as List).any((cell) => cell['text'] == '267'),
          isTrue,
        );
        engine.execute('cell.set', {'cell': 'B2', 'input': '126'});
        expect(
          (engine.snapshot(rows: 3, cols: 3)['cells'] as List).any(
            (cell) => cell['text'] == '126',
          ),
          isTrue,
        );
        final saved = engine.export('xlsx');
        expect(saved.length, greaterThan(1000));
        final reopened = GridcraftEngine();
        try {
          reopened.open('roundtrip.xlsx', saved);
          expect(
            (reopened.snapshot(rows: 3, cols: 3)['cells'] as List).any(
              (cell) => cell['text'] == '126',
            ),
            isTrue,
          );
          reopened.execute('insert.table', {
            'range': 'A1:D5',
            'name': 'Sales',
            'style': 'TableStyleMedium2',
          });
          expect(
            reopened.snapshot(rows: 8, cols: 5)['contexts'],
            contains('table'),
          );
        } finally {
          reopened.dispose();
        }
      } finally {
        engine.dispose();
      }
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (command, arguments) async => null,
        ),
      );
      registry.install(plugin);
      expect(
        registry.editorFor(
          OpenMuseResource(
            uri: fixture.uri,
            displayName: 'spreadsheet-shell.xlsx',
            mediaType:
                'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
          ),
        ),
        same(plugin),
      );
      stdout.writeln('Spreadsheet E2E: native roundtrip complete');
    });

    await tester.runAsync(ensureScreenshotFonts);
    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    final capture = GlobalKey();
    Future<void> screenshot(String name) async {
      if (screenshotDir == null) return;
      final boundary =
          capture.currentContext!.findRenderObject()! as RenderRepaintBoundary;
      await tester.runAsync(() async {
        final image = await boundary.toImage(pixelRatio: 1);
        final data = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '$screenshotDir/$name.png',
        ).writeAsBytes(data!.buffer.asUint8List());
        image.dispose();
      });
    }

    try {
      final resource = OpenMuseResource(
        uri: fixture.uri,
        displayName: 'spreadsheet-shell.xlsx',
        mediaType:
            'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      );
      await tester.pumpWidget(
        // The capture boundary covers the entire Host editor surface.
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(
              fontFamily: screenshotDir == null ? null : 'SpreadsheetE2E',
            ),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(context, resource),
              ),
            ),
          ),
        ),
      );
      for (var i = 0; i < 60 && find.text('North').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.byType(SpreadsheetProductShell), findsOneWidget);
      expect(find.text('Region'), findsWidgets);
      expect(find.text('字体'), findsNothing);
      await tester.pumpAndSettle();
      await screenshot('desktop-home');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-zoom-dialog')), findsOneWidget);
      await screenshot('desktop-zoom-dialog');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-preset-75')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(find.text('75%'), findsOneWidget);
      await screenshot('desktop-zoom-75');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('sheet-zoom-custom-input')),
        '999',
      );
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pump();
      expect(
        find.byKey(const ValueKey('sheet-zoom-range-error')),
        findsOneWidget,
      );
      await screenshot('desktop-zoom-range-error');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-cancel')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-preset-100')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(find.text('100%'), findsOneWidget);
      expect(find.text('North'), findsOneWidget);
      expect(find.text('267'), findsOneWidget);
      await tester.enterText(find.byType(TextField).first, 'A1:D2');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-fit-selection')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(find.text('100%'), findsNothing);
      expect(tester.getTopRight(find.text('Total')).dx, lessThan(1365));
      await screenshot('desktop-zoom-fit-selection');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-preset-100')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(find.textContaining('求和: 534'), findsOneWidget);
      await screenshot('desktop-selection-stats');
      expect(
        find.byKey(const ValueKey('sheet-workbook-status-top')),
        findsOneWidget,
      );
      expect(find.text('工作簿已加载，行列可编辑。'), findsNWidgets(2));
      await tester.longPress(
        find.byKey(const ValueKey('sheet-workbook-status-bottom')),
      );
      await tester.pump(const Duration(milliseconds: 150));
      expect(find.text('最大值'), findsOneWidget);
      await screenshot('desktop-status-menu');
      await tester.tap(find.text('最大值'));
      await tester.pump(const Duration(milliseconds: 150));
      expect(find.textContaining('最大值: 267'), findsOneWidget);
      await screenshot('desktop-selection-stats-max');
      await tester.tapAt(const Offset(800, 550));
      await tester.pump();
      expect(find.byKey(const ValueKey('sheet-auto-save')), findsOneWidget);
      await tester.tap(find.text('125'));
      await tester.pump(const Duration(milliseconds: 80));
      await tester.tap(find.text('125'));
      await tester.pump();
      expect(
        find.byKey(const ValueKey('sheet-cell-editor-B2')),
        findsOneWidget,
      );
      await screenshot('desktop-inline-editor');
      await tester.enterText(
        find.byKey(const ValueKey('sheet-cell-editor-B2')),
        '126',
      );
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      expect(find.text('126'), findsOneWidget);
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-0')));
      await tester.pump(const Duration(milliseconds: 150));
      expect(find.text('插入工作表'), findsOneWidget);
      expect(find.text('工作表标签颜色'), findsOneWidget);
      await screenshot('desktop-sheet-tab-menu');
      await tester.tap(find.text('插入工作表'));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-tab-item-1')), findsOneWidget);
      await screenshot('desktop-sheet-inserted');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-0')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('重命名'));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).last, 'Forecast');
      await tester.tap(find.text('确定'));
      await tester.pumpAndSettle();
      expect(find.text('Forecast'), findsOneWidget);
      await screenshot('desktop-sheet-renamed');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-0')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('隐藏'));
      await tester.pumpAndSettle();
      expect(find.text('Forecast'), findsNothing);
      await screenshot('desktop-sheet-hidden');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-1')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('取消隐藏…'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Forecast'));
      await tester.pumpAndSettle();
      expect(find.text('Forecast'), findsOneWidget);
      await screenshot('desktop-sheet-unhidden');
      await tester.tap(find.byKey(const ValueKey('sheet-tab-item-1')));
      await tester.pump();
      await tester.tap(find.byTooltip('分页预览'));
      await tester.pump();
      await screenshot('desktop-page-break-view');
      await tester.tap(find.byTooltip('普通视图'));
      await tester.pump();
      await screenshot('desktop-status-normal');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-1')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('工作表标签颜色'));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-color-palette')), findsOneWidget);
      expect(
        find.byKey(const ValueKey('sheet-color-shade-4-9')),
        findsOneWidget,
      );
      await screenshot('desktop-sheet-color-palette');
      await tester.tap(find.byKey(const ValueKey('sheet-color-standard-1')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-color-palette')), findsNothing);
      final sheetEngine = tester
          .widget<SpreadsheetProductShell>(find.byType(SpreadsheetProductShell))
          .engine;
      expect(
        (sheetEngine.snapshot()['sheetStates'] as List)[1]['tabColor'],
        '#FF0000',
      );
      await screenshot('desktop-sheet-colored');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-1')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('移动或复制…'));
      await tester.pumpAndSettle();
      expect(find.text('移动或复制工作表'), findsOneWidget);
      await screenshot('desktop-sheet-move-dialog');
      await tester.tap(find.byKey(const ValueKey('sheet-move-copy-check')));
      await tester.pump();
      await tester.tap(find.text('确定'));
      await tester.pumpAndSettle();
      expect(find.text('Sheet1 (2)'), findsOneWidget);
      expect(
        (sheetEngine.snapshot()['sheetStates'] as List)[2]['tabColor'],
        '#FF0000',
      );
      await screenshot('desktop-sheet-copied');
      await tester.longPress(find.byKey(const ValueKey('sheet-tab-item-1')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('移动或复制…'));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-move-target')), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('sheet-move-target-0')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('确定'));
      await tester.pumpAndSettle();
      expect((sheetEngine.snapshot()['sheets'] as List).first, 'Sheet1');
      await screenshot('desktop-sheet-moved');
      final verifiedBytes = sheetEngine.export('xlsx');
      final reopenedSheets = GridcraftEngine();
      try {
        reopenedSheets.open('sheet-structure-roundtrip.xlsx', verifiedBytes);
        final savedState = reopenedSheets.snapshot();
        expect(savedState['sheets'], ['Sheet1', 'Forecast', 'Sheet1 (2)']);
        expect((savedState['sheetStates'] as List)[0]['tabColor'], '#FF0000');
      } finally {
        reopenedSheets.dispose();
      }
      await tester.enterText(find.byType(TextField).first, 'A1');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      await tester.tap(
        find.byKey(const ValueKey('sheet-control-menu-borders')),
      );
      await tester.pump(const Duration(milliseconds: 150));
      expect(find.text('更多边框'), findsOneWidget);
      await tester.tap(find.text('更多边框'));
      await tester.pump(const Duration(milliseconds: 150));
      await screenshot('desktop-border-submenu');
      await tester.tapAt(const Offset(800, 550));
      await tester.pump(const Duration(milliseconds: 100));
      for (final tab in [
        'insert',
        'pageLayout',
        'formulas',
        'data',
        'review',
        'view',
      ]) {
        await tester.tap(find.byKey(ValueKey('sheet-tab-$tab')));
        await tester.pump(const Duration(milliseconds: 120));
        await screenshot('desktop-$tab');
      }
      await tester.tap(find.byKey(const ValueKey('sheet-tab-home')));
      await tester.pump(const Duration(milliseconds: 120));
      tester.view.physicalSize = const Size(834, 1112);
      await tester.pump();
      await screenshot('tablet-view');
      tester.view.physicalSize = const Size(720, 900);
      await tester.pump();
      await screenshot('foldable-view');
      tester.view.physicalSize = const Size(390, 844);
      await tester.pump();
      expect(find.byType(SpreadsheetProductShell), findsOneWidget);
      await screenshot('mobile-view');
      await tester.enterText(find.byType(TextField).first, 'A501');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      await tester.enterText(find.byType(TextField).last, 'Far row');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      final gridRows = tester.widget<ListView>(
        find.byKey(const ValueKey('sheet-grid-rows')),
      );
      gridRows.controller!.jumpTo(500 * 20);
      await tester.pump();
      expect(find.text('501'), findsOneWidget);
      expect(
        find.descendant(
          of: find.byKey(const ValueKey('sheet-grid-rows')),
          matching: find.text('Far row'),
        ),
        findsOneWidget,
      );
      await screenshot('mobile-row-501');
      await tester.enterText(find.byType(TextField).first, 'CV501');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      await tester.enterText(find.byType(TextField).last, 'Far column');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump();
      final horizontalGrid = tester.widget<SingleChildScrollView>(
        find.byType(SingleChildScrollView).first,
      );
      final defaultColWidth =
          ((sheetEngine.snapshot()['sheetGeometry'] as Map)['defaultColWidth']
                  as num)
              .toDouble();
      horizontalGrid.controller!.jumpTo(99 * defaultColWidth);
      await tester.pump();
      expect(
        find.descendant(
          of: find.byKey(const ValueKey('sheet-grid-rows')),
          matching: find.text('Far column'),
        ),
        findsOneWidget,
      );
      await screenshot('mobile-row-501-col-CV');
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    }
  });

  testWidgets('chart survives XLSX roundtrip and opens chart ribbon', (
    tester,
  ) async {
    if (!enabled) return;
    await tester.runAsync(ensureScreenshotFonts);
    final temp = File(
      '${Directory.systemTemp.path}/openmuse-spreadsheet-chart-e2e.xlsx',
    );
    await tester.runAsync(() async {
      final engine = GridcraftEngine();
      try {
        engine.open(
          'spreadsheet-shell.xlsx',
          await File('test/fixtures/spreadsheet-shell.xlsx').readAsBytes(),
        );
        engine.execute('insert.chart', {
          'range': 'A1:D5',
          'type': 'column',
          'at': 'F3',
          'title': 'Regional Sales',
          'width': 420,
          'height': 260,
        });
        final visuals = engine.snapshot(rows: 8, cols: 12)['visuals'] as List;
        expect(visuals, hasLength(1));
        expect((visuals.single['primitives'] as List), isNotEmpty);
        final bytes = engine.export('xlsx');
        await temp.writeAsBytes(bytes);
        final reopened = GridcraftEngine();
        try {
          reopened.open('chart-e2e.xlsx', bytes);
          final roundtrip =
              reopened.snapshot(rows: 8, cols: 12)['visuals'] as List;
          expect(roundtrip, hasLength(1));
          expect((roundtrip.single['primitives'] as List), isNotEmpty);
        } finally {
          reopened.dispose();
        }
      } finally {
        engine.dispose();
      }
    });
    try {
      final plugin = (await tester.runAsync(
        () async => OpenMuseSpreadsheetProductPlugin.loadBundled(
          bundleFile: File(bundlePath),
          trustedPublicKeyPem: await File(keyPath).readAsString(),
          expectedKeyId: keyId,
          expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
        ),
      ))!;
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      final capture = GlobalKey();
      await tester.pumpWidget(
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(
              fontFamily: screenshotDir == null ? null : 'SpreadsheetE2E',
            ),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(
                  context,
                  OpenMuseResource(
                    uri: temp.uri,
                    displayName: 'chart-e2e.xlsx',
                    mediaType:
                        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                  ),
                ),
              ),
            ),
          ),
        ),
      );
      final chart = find.byKey(const ValueKey('sheet-visual-chart-1'));
      for (var i = 0; i < 60 && chart.evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(chart, findsOneWidget);
      Future<void> captureImage(String name) async {
        if (screenshotDir == null) return;
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/$name.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        });
      }

      await captureImage('desktop-chart');
      await tester.tap(chart);
      await tester.pump();
      expect(
        find.byKey(const ValueKey('sheet-tab-chartDesign')),
        findsOneWidget,
      );
      await captureImage('desktop-chart-context');
      await tester.tap(find.byKey(const ValueKey('sheet-tab-insert')));
      await tester.pump();
      await tester.tap(
        find.byKey(const ValueKey('sheet-control-menu-recommended')),
      );
      await tester.pump();
      expect(find.text('柱形图'), findsWidgets);
      await captureImage('desktop-chart-menu');
      await tester.tap(find.text('柱形图').last);
      await tester.pump();
      expect(find.text('簇状柱形图'), findsOneWidget);
      await captureImage('desktop-chart-submenu');
      if (screenshotDir != null) {
        await tester.runAsync(
          () async => File(
            '$screenshotDir/chart-roundtrip.xlsx',
          ).writeAsBytes(await temp.readAsBytes()),
        );
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
      await tester.runAsync(() async {
        if (await temp.exists()) await temp.delete();
      });
    }
  });

  testWidgets('floating shape and picture render and survive XLSX roundtrip', (
    tester,
  ) async {
    if (!enabled) return;
    await tester.runAsync(ensureScreenshotFonts);
    final temp = File(
      '${Directory.systemTemp.path}/openmuse-spreadsheet-objects-e2e.xlsx',
    );
    late int shapeId;
    late int imageId;
    late int nextVisualId;
    await tester.runAsync(() async {
      final picture = await File(
        'test/fixtures/spreadsheet-visual-image.png',
      ).readAsBytes();
      final engine = GridcraftEngine();
      try {
        engine.open(
          'spreadsheet-shell.xlsx',
          await File('test/fixtures/spreadsheet-shell.xlsx').readAsBytes(),
        );
        engine.execute('insert.chart', {
          'range': 'A1:D5',
          'type': 'column',
          'at': 'F3',
          'title': 'Regional Sales',
          'width': 420,
          'height': 260,
        });
        shapeId =
            (engine.execute('insert.shape', {
                      'kind': 'roundedRectangle',
                      'at': 'B8',
                      'width': 170,
                      'height': 75,
                      'text': 'Visual model',
                      'fill': '#4472C4',
                      'line': '#203864',
                    })['shape']
                    as num)
                .toInt();
        imageId =
            (engine.execute('insert.picture', {
                      'base64': base64Encode(picture),
                      'mime': 'image/png',
                      'at': 'B14',
                      'width': 144,
                      'height': 108,
                      'alt': 'Color sample',
                    })['image']
                    as num)
                .toInt();
        final visuals = engine.snapshot(rows: 22, cols: 12)['visuals'] as List;
        expect(
          visuals.map((v) => v['kind']),
          containsAll(['chart', 'shape', 'image']),
        );
        expect(engine.imageBytes(imageId), orderedEquals(picture));
        final saved = engine.export('xlsx');
        await temp.writeAsBytes(saved);
        final reopened = GridcraftEngine();
        try {
          reopened.open('objects-e2e.xlsx', saved);
          final roundtrip =
              reopened.snapshot(rows: 22, cols: 12)['visuals'] as List;
          expect(
            roundtrip.map((v) => v['kind']),
            containsAll(['chart', 'shape', 'image']),
          );
          expect(
            roundtrip.firstWhere((v) => v['kind'] == 'shape')['text'],
            'Visual model',
          );
          shapeId =
              (roundtrip.firstWhere((v) => v['kind'] == 'shape')['id'] as num)
                  .toInt();
          imageId =
              (roundtrip.firstWhere((v) => v['kind'] == 'image')['id'] as num)
                  .toInt();
          nextVisualId =
              roundtrip.map((v) => (v['id'] as num).toInt()).reduce(math.max) +
              1;
          expect(reopened.imageBytes(imageId), orderedEquals(picture));
        } finally {
          reopened.dispose();
        }
      } finally {
        engine.dispose();
      }
    });
    try {
      final plugin = (await tester.runAsync(
        () async => OpenMuseSpreadsheetProductPlugin.loadBundled(
          bundleFile: File(bundlePath),
          trustedPublicKeyPem: await File(keyPath).readAsString(),
          expectedKeyId: keyId,
          expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
        ),
      ))!;
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      final capture = GlobalKey();
      await tester.pumpWidget(
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(
              fontFamily: screenshotDir == null ? null : 'SpreadsheetE2E',
            ),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(
                  context,
                  OpenMuseResource(
                    uri: temp.uri,
                    displayName: 'objects-e2e.xlsx',
                    mediaType:
                        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                  ),
                ),
              ),
            ),
          ),
        ),
      );
      final shape = find.byKey(ValueKey('sheet-visual-shape-$shapeId'));
      final image = find.byKey(ValueKey('sheet-visual-image-$imageId'));
      for (var i = 0; i < 60 && image.evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(shape, findsOneWidget);
      expect(image, findsOneWidget);
      await tester.runAsync(
        () async => Future<void>.delayed(const Duration(milliseconds: 150)),
      );
      await tester.pumpAndSettle();
      Future<void> captureImage(String name) async {
        if (screenshotDir == null) return;
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final frame = await boundary.toImage(pixelRatio: 1);
          final bytes = await frame.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/$name.png',
          ).writeAsBytes(bytes!.buffer.asUint8List());
          frame.dispose();
        });
      }

      await captureImage('desktop-objects');
      final originalShapeSize = tester.getSize(shape);
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-preset-75')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(
        tester.getSize(shape).width,
        closeTo(originalShapeSize.width * .75, 2),
      );
      await captureImage('desktop-objects-zoom-75');
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-value')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-preset-100')));
      await tester.tap(find.byKey(const ValueKey('sheet-zoom-confirm')));
      await tester.pumpAndSettle();
      expect(tester.getSize(shape).width, closeTo(originalShapeSize.width, 2));
      await tester.tap(shape);
      await tester.pump();
      await captureImage('desktop-shape-eight-handles');
      expect(
        find.byKey(const ValueKey('sheet-tab-shapeFormat')),
        findsOneWidget,
      );
      await captureImage('desktop-shape-context');
      await tester.tap(image);
      await tester.pump();
      expect(
        find.byKey(const ValueKey('sheet-tab-pictureFormat')),
        findsOneWidget,
      );
      await captureImage('desktop-picture-context');
      await tester.tap(find.byKey(const ValueKey('sheet-tab-insert')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('sheet-control-menu-shape')));
      await tester.pump();
      expect(find.text('椭圆'), findsOneWidget);
      await captureImage('desktop-shape-menu');
      await tester.tap(find.text('椭圆'));
      await tester.pumpAndSettle();
      await captureImage('desktop-shape-inserted');
      expect(
        find.byKey(ValueKey('sheet-visual-shape-$nextVisualId')),
        findsOneWidget,
      );
      await tester.tap(shape);
      await tester.pump();
      final beforeMove = tester.getTopLeft(shape);
      await tester.drag(shape, const Offset(140, 28));
      await tester.pumpAndSettle();
      expect(tester.getTopLeft(shape).dx, closeTo(beforeMove.dx + 140, 3));
      expect(tester.getTopLeft(shape).dy, closeTo(beforeMove.dy + 28, 3));
      await captureImage('desktop-shape-moved');
      final beforeResize = tester.getSize(shape);
      await tester.dragFrom(
        tester.getBottomRight(shape) - const Offset(6, 6),
        const Offset(40, 32),
      );
      await tester.pumpAndSettle();
      expect(tester.getSize(shape).width, greaterThan(beforeResize.width + 30));
      expect(
        tester.getSize(shape).height,
        greaterThan(beforeResize.height + 20),
      );
      await captureImage('desktop-shape-resized');
      final beforeNorthwest = tester.getRect(shape);
      await tester.dragFrom(
        beforeNorthwest.topLeft + const Offset(6, 6),
        const Offset(-32, -28),
      );
      await tester.pumpAndSettle();
      final afterNorthwest = tester.getRect(shape);
      expect(afterNorthwest.left, closeTo(beforeNorthwest.left - 32, 3));
      expect(afterNorthwest.top, closeTo(beforeNorthwest.top - 28, 3));
      expect(afterNorthwest.right, closeTo(beforeNorthwest.right, 3));
      expect(afterNorthwest.bottom, closeTo(beforeNorthwest.bottom, 3));
      await captureImage('desktop-shape-northwest-resized');
      await tester.dragFrom(
        Offset(afterNorthwest.left + 6, afterNorthwest.center.dy),
        const Offset(24, 0),
      );
      await tester.pumpAndSettle();
      final afterWest = tester.getRect(shape);
      expect(afterWest.left, closeTo(afterNorthwest.left + 24, 3));
      expect(afterWest.right, closeTo(afterNorthwest.right, 3));
      expect(afterWest.height, closeTo(afterNorthwest.height, 3));
      await captureImage('desktop-shape-west-resized');
      await tester.tap(find.byTooltip('保存'));
      for (var i = 0; i < 40 && find.text('已保存').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 50)),
        );
        await tester.pump();
      }
      expect(find.text('已保存'), findsNWidgets(2));
      await tester.runAsync(() async {
        final reopened = GridcraftEngine();
        try {
          reopened.open('objects-edited.xlsx', await temp.readAsBytes());
          final edited =
              (reopened.snapshot(rows: 22, cols: 12)['visuals'] as List)
                  .firstWhere(
                    (v) => v['kind'] == 'shape' && v['text'] == 'Visual model',
                  );
          final anchor = edited['anchor'] as Map;
          expect((anchor['col'] as num).toInt(), greaterThanOrEqualTo(2));
          expect((anchor['row'] as num).toInt(), greaterThanOrEqualTo(7));
          expect((anchor['width'] as num).toDouble(), greaterThan(190));
          expect((anchor['height'] as num).toDouble(), greaterThan(90));
        } finally {
          reopened.dispose();
        }
        if (screenshotDir != null) {
          await File(
            '$screenshotDir/objects-edited.xlsx',
          ).writeAsBytes(await temp.readAsBytes());
        }
      });
      await tester.tap(image);
      await tester.pump();
      await tester.sendKeyEvent(LogicalKeyboardKey.delete);
      await tester.pumpAndSettle();
      expect(image, findsNothing);
      await captureImage('desktop-picture-deleted');
      await tester.tap(find.byTooltip('撤销'));
      await tester.pumpAndSettle();
      expect(image, findsOneWidget);
      tester.view.physicalSize = const Size(390, 844);
      await tester.pumpAndSettle();
      await captureImage('mobile-objects');
      if (screenshotDir != null) {
        await tester.runAsync(
          () async => File(
            '$screenshotDir/objects-roundtrip.xlsx',
          ).writeAsBytes(await temp.readAsBytes()),
        );
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
      await tester.runAsync(() async {
        if (await temp.exists()) await temp.delete();
      });
    }
  });

  testWidgets('table selection exposes signed contextual ribbon', (
    tester,
  ) async {
    if (!enabled) return;
    final temp = File(
      '${Directory.systemTemp.path}/openmuse-spreadsheet-table-e2e.xlsx',
    );
    await tester.runAsync(() async {
      final engine = GridcraftEngine();
      try {
        engine.open(
          'spreadsheet-shell.xlsx',
          await File('test/fixtures/spreadsheet-shell.xlsx').readAsBytes(),
        );
        engine.execute('insert.table', {
          'range': 'A1:D5',
          'name': 'Sales',
          'style': 'TableStyleMedium2',
        });
        await temp.writeAsBytes(engine.export('xlsx'));
      } finally {
        engine.dispose();
      }
    });
    try {
      final plugin = (await tester.runAsync(
        () async => OpenMuseSpreadsheetProductPlugin.loadBundled(
          bundleFile: File(bundlePath),
          trustedPublicKeyPem: await File(keyPath).readAsString(),
          expectedKeyId: keyId,
          expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
        ),
      ))!;
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      final contextualCapture = GlobalKey();
      await tester.pumpWidget(
        RepaintBoundary(
          key: contextualCapture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(
              fontFamily: screenshotDir == null ? null : 'SpreadsheetE2E',
            ),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(
                  context,
                  OpenMuseResource(
                    uri: temp.uri,
                    displayName: 'table-e2e.xlsx',
                    mediaType:
                        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                  ),
                ),
              ),
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 30 &&
            find
                .byKey(const ValueKey('sheet-tab-tableDesign'))
                .evaluate()
                .isEmpty;
        i++
      ) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(
        find.byKey(const ValueKey('sheet-tab-tableDesign')),
        findsOneWidget,
      );
      await tester.tap(find.byKey(const ValueKey('sheet-tab-tableDesign')));
      await tester.pump();
      expect(find.text('表格样式'), findsWidgets);
      if (screenshotDir != null) {
        final boundary =
            contextualCapture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/desktop-table-context.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        });
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
      await tester.runAsync(() async {
        if (await temp.exists()) await temp.delete();
      });
    }
  });

  testWidgets('product auto-save persists an inline cell edit', (tester) async {
    if (!enabled) return;
    final directory = (await tester.runAsync(
      () => Directory.systemTemp.createTemp('openmuse-sheet-autosave-'),
    ))!;
    final workbook = File('${directory.path}/auto-save.xlsx');
    await tester.runAsync(() async {
      await workbook.writeAsBytes(
        await File('test/fixtures/spreadsheet-shell.xlsx').readAsBytes(),
      );
    });
    try {
      final plugin = (await tester.runAsync(
        () async => OpenMuseSpreadsheetProductPlugin.loadBundled(
          bundleFile: File(bundlePath),
          trustedPublicKeyPem: await File(keyPath).readAsString(),
          expectedKeyId: keyId,
          expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
        ),
      ))!;
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => plugin.buildEditor(
                context,
                OpenMuseResource(
                  uri: workbook.uri,
                  displayName: 'auto-save.xlsx',
                  mediaType:
                      'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                ),
              ),
            ),
          ),
        ),
      );
      for (var i = 0; i < 30 && find.text('125').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.text('125'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('sheet-auto-save')));
      await tester.tap(find.text('125'));
      await tester.pump(const Duration(milliseconds: 80));
      await tester.tap(find.text('125'));
      await tester.pump();
      await tester.enterText(
        find.byKey(const ValueKey('sheet-cell-editor-B2')),
        '126',
      );
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pump(const Duration(milliseconds: 900));
      for (var i = 0; i < 20 && find.text('已保存').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 50)),
        );
        await tester.pump();
      }
      expect(find.text('已保存'), findsNWidgets(2));
      await tester.runAsync(() async {
        final reopened = GridcraftEngine();
        try {
          reopened.open('auto-save.xlsx', await workbook.readAsBytes());
          expect(
            (reopened.snapshot(rows: 3, cols: 3)['cells'] as List).any(
              (cell) => cell['text'] == '126',
            ),
            isTrue,
          );
        } finally {
          reopened.dispose();
        }
      });
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
      await tester.runAsync(() async => directory.delete(recursive: true));
    }
  });
}
