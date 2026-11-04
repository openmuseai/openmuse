import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/settings_dialog.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets('desktop default generated directory is visible and resettable', (
    tester,
  ) async {
    if (!Platform.isMacOS) return;
    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final capture = GlobalKey();
    final uiFont = Platform.environment['PDF_PRODUCT_E2E_UI_FONT'];
    final iconFont = Platform.environment['PDF_PRODUCT_E2E_ICON_FONT'];
    if (uiFont != null && iconFont != null) {
      Future<void> registerFont(String family, String path) async {
        final bytes = await File(path).readAsBytes();
        await (FontLoader(family)
              ..addFont(Future<ByteData>.value(ByteData.sublistView(bytes))))
            .load();
      }

      await tester.runAsync(() async {
        await registerFont('PdfE2E', uiFont);
        await registerFont('MaterialIcons', iconFont);
      });
    }
    final settings = OpenMuseLocalSettings();
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, arguments) async => null,
      ),
    );
    await tester.pumpWidget(
      RepaintBoundary(
        key: capture,
        child: MaterialApp(
          debugShowCheckedModeBanner: false,
          theme: ThemeData(fontFamily: uiFont == null ? null : 'PdfE2E'),
          home: Scaffold(
            body: Builder(
              builder: (context) => FilledButton(
                onPressed: () =>
                    showOpenMuseSettings(context, settings, registry),
                child: const Text('设置'),
              ),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.text('设置'));
    await tester.pumpAndSettle();
    expect(find.text('PDF 生成文件默认目录'), findsOneWidget);
    expect(find.text('默认：应用文档目录/OpenMuse'), findsOneWidget);
    expect(
      find.byKey(const Key('generated-output-directory-choose')),
      findsOneWidget,
    );
    final screenshot = Platform.environment['PDF_PRODUCT_SETTINGS_SCREENSHOT'];
    if (screenshot != null) {
      final boundary =
          capture.currentContext!.findRenderObject()! as RenderRepaintBoundary;
      await tester.runAsync(() async {
        final image = await boundary.toImage(pixelRatio: 1);
        final png = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(screenshot).writeAsBytes(png!.buffer.asUint8List());
        image.dispose();
      });
    }
    await settings.setGeneratedOutputDirectory('/tmp/openmuse-pdf-export-test');
    await tester.pump();
    expect(find.text('/tmp/openmuse-pdf-export-test'), findsOneWidget);
    await tester.tap(find.byKey(const Key('generated-output-directory-reset')));
    await tester.pump();
    expect(settings.generatedOutputDirectory, isNull);
    expect(find.text('默认：应用文档目录/OpenMuse'), findsOneWidget);
  });
}
