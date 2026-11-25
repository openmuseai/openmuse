import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
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

  testWidgets('variable workbook geometry renders from the engine', (
    tester,
  ) async {
    if (!enabled) return;
    final fixture = File('test/fixtures/spreadsheet-geometry.xlsx').absolute;
    final plugin = (await tester.runAsync(() async {
      final engine = GridcraftEngine();
      try {
        engine.open('spreadsheet-geometry.xlsx', await fixture.readAsBytes());
        final geometry =
            engine.snapshot(rows: 6, cols: 5)['sheetGeometry'] as Map;
        expect(geometry['defaultRowHeight'], 20);
        expect(geometry['defaultColWidth'], closeTo(69.333, .01));
        expect(geometry['rowsComplete'], isTrue);
        final rowSizes = {
          for (final pair in geometry['rowSizes'] as List)
            (pair[0] as num).toInt(): (pair[1] as num).toDouble(),
        };
        final colSizes = {
          for (final pair in geometry['colSizes'] as List)
            (pair[0] as num).toInt(): (pair[1] as num).toDouble(),
        };
        expect(rowSizes, containsPair(0, 40));
        expect(rowSizes, containsPair(1, 12));
        expect(rowSizes, containsPair(3, 0));
        expect(colSizes, containsPair(1, 176));
        expect(colSizes, containsPair(2, 32));
      } finally {
        engine.dispose();
      }
      if (screenshotDir != null) {
        Future<void> font(String family, String path) async {
          final bytes = await File(path).readAsBytes();
          await (FontLoader(family)
                ..addFont(Future<ByteData>.value(ByteData.sublistView(bytes))))
              .load();
        }

        await font('SpreadsheetE2E', env['SPREADSHEET_PRODUCT_E2E_UI_FONT']!);
        await font('MaterialIcons', env['SPREADSHEET_PRODUCT_E2E_ICON_FONT']!);
        await Directory(screenshotDir).create(recursive: true);
      }
      return OpenMuseSpreadsheetProductPlugin.loadBundled(
        bundleFile: File(bundlePath),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
        expectedVersion: env['SPREADSHEET_PRODUCT_E2E_VERSION'] ?? '0.1.18',
      );
    }))!;
    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    final capture = GlobalKey();
    try {
      await tester.pumpWidget(
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            theme: ThemeData(
              fontFamily: screenshotDir == null ? null : 'SpreadsheetE2E',
            ),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(
                  context,
                  OpenMuseResource(
                    uri: fixture.uri,
                    displayName: 'spreadsheet-geometry.xlsx',
                    mediaType:
                        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                  ),
                ),
              ),
            ),
          ),
        ),
      );
      for (var i = 0; i < 50 && find.text('Region').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 50)),
        );
        await tester.pump();
      }
      expect(find.byType(SpreadsheetProductShell), findsOneWidget);
      expect(find.text('Region'), findsOneWidget);
      await tester.pumpAndSettle();
      Rect cell(String address) =>
          tester.getRect(find.byKey(ValueKey('sheet-cell-$address')));
      expect(cell('A1').width, closeTo(69.333, .1));
      expect(cell('A1').height, closeTo(40, .1));
      expect(cell('B1').width, closeTo(176, .1));
      expect(cell('C1').width, closeTo(32, .1));
      expect(cell('D1').width, closeTo(128, .1));
      expect(cell('B1').left, closeTo(cell('A1').right, .1));
      expect(cell('A2').top, closeTo(cell('A1').bottom, .1));
      expect(cell('A2').height, closeTo(12, .1));
      expect(cell('A5').top, closeTo(cell('A3').bottom, .1));
      if (screenshotDir != null) {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/desktop-variable-geometry.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        });
      }
      await tester.tap(find.byKey(const ValueKey('sheet-go-to-open')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-go-to-dialog')), findsOneWidget);
      expect(
        find.byKey(const ValueKey('sheet-go-to-backdrop')),
        findsOneWidget,
      );
      await tester.enterText(
        find.byKey(const ValueKey('sheet-go-to-input')),
        'XFE1',
      );
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('sheet-go-to-confirm')));
      await tester.pumpAndSettle();
      expect(find.text('请输入有效的单元格或区域引用。'), findsOneWidget);
      await tester.enterText(
        find.byKey(const ValueKey('sheet-go-to-input')),
        'Z100',
      );
      await tester.pump();
      if (screenshotDir != null) {
        await tester.pumpAndSettle();
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/desktop-go-to-dialog.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        });
      }
      await tester.tap(find.byKey(const ValueKey('sheet-go-to-confirm')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('sheet-go-to-dialog')), findsNothing);
      expect(find.byKey(const ValueKey('sheet-cell-Z100')), findsOneWidget);
      expect(find.text('Z100'), findsWidgets);
      expect(cell('Z100').left, greaterThan(1000));
      expect(cell('Z100').top, lessThan(220));
      if (screenshotDir != null) {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$screenshotDir/desktop-go-to-result.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        });
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    }
  });
}
