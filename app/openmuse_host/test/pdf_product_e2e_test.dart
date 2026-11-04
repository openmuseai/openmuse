import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart'
    show TargetPlatform, debugDefaultTargetPlatformOverride;
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_pdf_product/openmuse_pdf_product.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:pdfrx/pdfrx.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';
import 'package:pdf_rebuild_flutter/pdf_product_chrome.dart';
import 'package:pdf_rebuild_flutter/pdf_document_status.dart';

import 'package:openmuse_host/src/host/pdf_product_plugin.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/pdf_generated_document_port.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  final bundlePath = Platform.environment['PDF_PRODUCT_E2E_BUNDLE'];
  final keyPath = Platform.environment['PDF_PRODUCT_E2E_PUBLIC_KEY'];
  final keyId = Platform.environment['PDF_PRODUCT_E2E_KEY_ID'];
  final nativePath = Platform.environment['KSP_MAC_EDIT_DYLIB'];
  final expectedVersion =
      Platform.environment['PDF_PRODUCT_E2E_VERSION'] ?? '0.1.18';
  final enabled =
      bundlePath != null &&
      keyPath != null &&
      keyId != null &&
      nativePath != null;

  testWidgets('signed PDF product routes and renders a real PDF', (
    tester,
  ) async {
    if (!enabled) return;
    late OpenMusePdfProductPlugin plugin;
    late OpenMuseResource resource;
    late VerifiedPdfProduct snapshot;
    final settings = OpenMuseLocalSettings();
    await tester.runAsync(() async {
      await settings.setGeneratedOutputDirectory(Directory.systemTemp.path);
      final bundle = Uint8List.fromList(await File(bundlePath).readAsBytes());
      final key = await File(keyPath).readAsString();
      plugin = await OpenMusePdfProductPlugin.loadBundled(
        bundleFile: File(bundlePath),
        trustedPublicKeyPem: key,
        expectedKeyId: keyId,
        expectedVersion: expectedVersion,
        settings: settings,
      );
      snapshot = await VerifiedPdfProduct.loadBundle(
        bundle: bundle,
        trustedPublicKeyPem: key,
        expectedKeyId: keyId,
        expectedVersion: expectedVersion,
      );
      expect(snapshot.chrome.tabs, hasLength(7));
      expect(snapshot.chrome.iconSvg('home.highlight'), contains('<svg'));
      expect(snapshot.enabledFeatures, contains('pdf.read'));
      final tampered = Uint8List.fromList(bundle)..[bundle.length ~/ 2] ^= 0x01;
      await expectLater(
        VerifiedPdfProduct.loadBundle(
          bundle: tampered,
          trustedPublicKeyPem: key,
          expectedKeyId: keyId,
        ),
        throwsFormatException,
      );

      final fixture = File(
        '../../../ioffice/koffice/pdf/rebuild/pdf-flutter-app/assets/test/editable-two-page.pdf',
      ).absolute;
      expect(await fixture.exists(), isTrue);
      resource = OpenMuseResource(
        uri: fixture.uri,
        displayName: fixture.uri.pathSegments.last,
        mediaType: 'application/pdf',
      );
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (command, arguments) async => null,
        ),
      );
      registry.install(plugin);
      expect(registry.editorFor(resource), same(plugin));

      final backend = PdfProductBackend();
      final document = await backend.open(
        await fixture.readAsBytes(),
        'editable-two-page.pdf',
      );
      expect(document.pageCount, 2);
      final raster = await backend.renderPageAt(0, 128, 128);
      expect(raster.rgba, hasLength(128 * 128 * 4));
      expect(raster.rgba.any((byte) => byte != 255), isTrue);
      final resizedRaster = await backend.renderPageAt(0, 210, 297);
      expect(resizedRaster.rgba, hasLength(210 * 297 * 4));
      expect(resizedRaster.rgba.any((byte) => byte != 255), isTrue);
      backend.close();
    });

    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    final screenshotPath = Platform.environment['PDF_PRODUCT_E2E_SCREENSHOT'];
    final screenshotDir =
        Platform.environment['PDF_PRODUCT_E2E_SCREENSHOT_DIR'];
    final captureScreenshots = screenshotPath != null || screenshotDir != null;
    if (captureScreenshots) {
      debugDefaultTargetPlatformOverride = TargetPlatform.macOS;
    }
    final uiFont = Platform.environment['PDF_PRODUCT_E2E_UI_FONT'];
    final iconFont = Platform.environment['PDF_PRODUCT_E2E_ICON_FONT'];
    if (captureScreenshots && uiFont != null && iconFont != null) {
      Future<void> registerFont(String family, String path) async {
        final bytes = await File(path).readAsBytes();
        await (FontLoader(
          family,
        )..addFont(Future<ByteData>.value(ByteData.sublistView(bytes)))).load();
      }

      await tester.runAsync(() async {
        await registerFont('PdfE2E', uiFont);
        await registerFont('MaterialIcons', iconFont);
      });
    }
    final capture = GlobalKey();
    try {
      await tester.pumpWidget(
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(fontFamily: uiFont == null ? null : 'PdfE2E'),
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildEditor(context, resource),
              ),
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 80 && find.text('第 1 页，共 2 页').evaluate().isEmpty;
        i++
      ) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.byType(PdfWorkspace), findsOneWidget);
      expect(
        tester
            .widget<PdfWorkspace>(find.byType(PdfWorkspace))
            .generatedOutputDirectory
            ?.call(),
        Directory.systemTemp.path,
      );
      expect(find.byType(PdfProductShell), findsOneWidget);
      expect(find.text('第 1 页，共 2 页'), findsOneWidget);
      expect(find.byKey(const Key('pdf-tab-formDesign')), findsOneWidget);
      await tester.pump(const Duration(milliseconds: 500));
      await tester.runAsync(
        () async => Future<void>.delayed(const Duration(milliseconds: 300)),
      );
      await tester.pump(const Duration(milliseconds: 300));
      final viewer = tester.widget<PdfViewer>(find.byType(PdfViewer));
      final controller = viewer.controller!;
      expect(controller.isReady, isTrue);
      expect(viewer.params.margin, 24);
      expect(controller.layout.pageLayouts.first.left, 24);
      expect(
        controller.layout.pageLayouts[1].top -
            controller.layout.pageLayouts.first.bottom,
        16,
      );
      expect(
        viewer.params.pageDropShadow?.color,
        const Color(0xff000000).withValues(alpha: 0.18),
      );
      expect(viewer.params.pageDropShadow?.blurRadius, 4);
      expect(viewer.params.pageDropShadow?.spreadRadius, 0);
      expect(viewer.params.pageDropShadow?.offset, const Offset(0, 1));
      Future<void> capturePng(String path, {double pixelRatio = 1}) async {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: pixelRatio);
          final png = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(path).writeAsBytes(png!.buffer.asUint8List());
          image.dispose();
        });
      }

      if (screenshotPath != null) await capturePng(screenshotPath);
      if (screenshotDir != null) {
        await tester.runAsync(
          () async => Directory(screenshotDir).create(recursive: true),
        );
        expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 360);
        tester.view.physicalSize = const Size(2400, 1530);
        tester.view.devicePixelRatio = 1.5;
        await tester.pumpAndSettle();
        expect(
          tester.getTopLeft(find.byKey(const Key('pdf-sidebar-pane'))).dx,
          360,
        );
        await capturePng(
          '$screenshotDir/genoffice-viewport-desktop.png',
          pixelRatio: 1.5,
        );
        final viewerController = tester
            .widget<PdfViewer>(find.byType(PdfViewer))
            .controller!;
        final initialDocumentTop = viewerController.visibleRect.top;
        tester
            .widget<Focus>(find.byKey(const Key('pdf-shell-keyboard')))
            .focusNode!
            .requestFocus();
        await tester.pump();
        await tester.sendKeyEvent(LogicalKeyboardKey.pageDown);
        await tester.pumpAndSettle();
        expect(
          viewerController.visibleRect.top,
          greaterThan(initialDocumentTop + 100),
        );
        await capturePng(
          '$screenshotDir/keyboard-page-down-desktop.png',
          pixelRatio: 1.5,
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.pageUp);
        await tester.pumpAndSettle();
        expect(
          viewerController.visibleRect.top,
          closeTo(initialDocumentTop, 2),
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.end);
        await tester.pumpAndSettle();
        expect(
          viewerController.visibleRect.top,
          greaterThan(initialDocumentTop + 100),
        );
        await capturePng(
          '$screenshotDir/keyboard-document-end-desktop.png',
          pixelRatio: 1.5,
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.home);
        await tester.pumpAndSettle();
        expect(
          viewerController.visibleRect.top,
          closeTo(initialDocumentTop, 2),
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.arrowRight);
        await tester.pumpAndSettle();
        expect(find.text('第 2 页，共 2 页'), findsOneWidget);
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 300)),
        );
        await tester.pump(const Duration(milliseconds: 300));
        await capturePng(
          '$screenshotDir/keyboard-next-page-desktop.png',
          pixelRatio: 1.5,
        );
        await tester.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await tester.sendKeyEvent(LogicalKeyboardKey.keyF);
        await tester.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-search-bar')), findsOneWidget);
        await capturePng(
          '$screenshotDir/keyboard-search-desktop.png',
          pixelRatio: 1.5,
        );
        await tester.tap(find.byKey(const Key('pdf-search-close')));
        await tester.pumpAndSettle();
        tester
            .widget<Focus>(find.byKey(const Key('pdf-shell-keyboard')))
            .focusNode!
            .requestFocus();
        await tester.pump();
        await tester.sendKeyEvent(LogicalKeyboardKey.arrowLeft);
        await tester.pumpAndSettle();
        expect(find.text('第 1 页，共 2 页'), findsOneWidget);
        tester.view.devicePixelRatio = 1;
        tester.view.physicalSize = const Size(1365, 860);
        await tester.pumpAndSettle();
        for (final tab in [
          'home',
          'annotate',
          'edit',
          'page',
          'view',
          'fillForm',
          'formDesign',
        ]) {
          if (tab != 'home') await tester.tap(find.byKey(Key('pdf-tab-$tab')));
          await tester.pump(const Duration(milliseconds: 150));
          await capturePng('$screenshotDir/ribbon-$tab.png');
          if (tab == 'view') {
            await tester.tap(find.byKey(const Key('pdf-control-view-spread')));
            await tester.pumpAndSettle();
            expect(
              viewerController.layout.pageLayouts[0].top,
              viewerController.layout.pageLayouts[1].top,
            );
            expect(
              viewerController.visibleRect.right,
              greaterThanOrEqualTo(
                viewerController.layout.pageLayouts[1].right - 2,
              ),
            );
            expect(find.text('单页'), findsOneWidget);
            await capturePng('$screenshotDir/view-spread-desktop.png');
            await tester.tap(find.byKey(const Key('pdf-control-view-spread')));
            await tester.pumpAndSettle();
            expect(
              viewerController.layout.pageLayouts[1].top,
              greaterThan(viewerController.layout.pageLayouts[0].bottom),
            );
            await capturePng('$screenshotDir/view-single-restored-desktop.png');
          }
        }
        await tester.tap(find.byKey(const Key('pdf-tab-home')));
        await tester.pump(const Duration(milliseconds: 150));
        final initialSidebarWidth = tester
            .getSize(find.byKey(const Key('pdf-sidebar-pane')))
            .width;
        expect(initialSidebarWidth, inInclusiveRange(120, 320));
        expect(
          tester.getTopLeft(find.byKey(const Key('pdf-sidebar-resizer'))).dx,
          tester.getTopRight(find.byKey(const Key('pdf-sidebar-pane'))).dx - 3,
        );
        final resizeDelta = initialSidebarWidth > 230 ? -80.0 : 80.0;
        await tester.drag(
          find.byKey(const Key('pdf-sidebar-resizer')),
          Offset(resizeDelta, 0),
        );
        await tester.pumpAndSettle();
        expect(
          tester.getSize(find.byKey(const Key('pdf-sidebar-pane'))).width,
          (initialSidebarWidth + resizeDelta).clamp(120.0, 320.0),
        );
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 1200)),
        );
        await tester.pumpAndSettle();
        final resizedSidebarWidth = tester
            .getSize(find.byKey(const Key('pdf-sidebar-pane')))
            .width;
        final thumbnailKey = Key(
          'thumb-editable-two-page.pdf-0-${(resizedSidebarWidth - 24).round()}',
        );
        expect(
          find.descendant(
            of: find.byKey(thumbnailKey),
            matching: find.byType(RawImage),
          ),
          findsOneWidget,
        );
        await capturePng('$screenshotDir/sidebar-resized-desktop.png');
        await tester.drag(
          find.byKey(const Key('pdf-sidebar-resizer')),
          Offset(-resizeDelta, 0),
        );
        await tester.pumpAndSettle();
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 400)),
        );
        await tester.pump();
        if (find.byKey(const Key('pdf-ai-rail')).evaluate().isEmpty) {
          await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
          await tester.pumpAndSettle();
        }
        await tester.tap(find.byKey(const Key('pdf-ai-rail')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-ai-unavailable')), findsOneWidget);
        final initialAiWidth = tester
            .getSize(find.byKey(const Key('pdf-ai-dock')))
            .width;
        expect(initialAiWidth, inInclusiveRange(280, 720));
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 300)),
        );
        await tester.pumpAndSettle();
        await capturePng('$screenshotDir/ai-dock-expanded-desktop.png');
        final aiResizeDelta = initialAiWidth > 560 ? -80.0 : 80.0;
        await tester.drag(
          find.byKey(const Key('pdf-ai-resizer')),
          Offset(aiResizeDelta, 0),
        );
        await tester.pumpAndSettle();
        expect(
          tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width,
          initialAiWidth + aiResizeDelta,
        );
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 300)),
        );
        await tester.pumpAndSettle();
        await capturePng('$screenshotDir/ai-dock-resized-desktop.png');
        await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('pdf-control-home-search')));
        await tester.pump();
        expect(find.byKey(const Key('pdf-search-bar')), findsOneWidget);
        await capturePng('$screenshotDir/search-bar-desktop.png');
        await tester.enterText(
          find.byKey(const Key('pdf-search-input')),
          'Editable',
        );
        await tester.pump(const Duration(milliseconds: 600));
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 500)),
        );
        await tester.pump();
        expect(find.text('1 / 2'), findsOneWidget);
        await capturePng('$screenshotDir/search-results-desktop.png');
        await tester.tap(find.byKey(const Key('pdf-search-next')));
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 300)),
        );
        await tester.pump(const Duration(milliseconds: 200));
        expect(find.text('2 / 2'), findsOneWidget);
        await tester.tap(find.byKey(const Key('pdf-search-close')));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(390, 844);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-home-search')),
          220,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('pdf-ribbon-home')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.tap(find.byKey(const Key('pdf-control-home-search')));
        await tester.pump();
        await capturePng('$screenshotDir/search-bar-phone.png');
        await tester.tap(find.byKey(const Key('pdf-search-close')));
        await tester.pump();
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-home-ai')),
          -220,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('pdf-ribbon-home')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.drag(
          find.byKey(const Key('pdf-ribbon-home')),
          const Offset(500, 0),
        );
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('pdf-control-home-ai')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-ai-panel')), findsOneWidget);
        await capturePng('$screenshotDir/ai-dock-phone-sheet.png');
        await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(1365, 860);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        await tester.tap(find.byKey(const Key('pdf-tab-page')));
        await tester.pump(const Duration(milliseconds: 150));
        await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-extract-dialog')), findsOneWidget);
        await capturePng('$screenshotDir/extract-range-dialog-desktop.png');
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(390, 844);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        expect(
          tester.widget<PdfProductShell>(find.byType(PdfProductShell)).zoom,
          lessThan(1),
        );
        await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
        await tester.pumpAndSettle();
        await capturePng('$screenshotDir/extract-range-dialog-phone.png');
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-page-splitPdf')),
          220,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('pdf-ribbon-page')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.tap(find.byKey(const Key('pdf-control-page-splitPdf')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-split-dialog')), findsOneWidget);
        await capturePng('$screenshotDir/split-pdf-dialog-phone.png');
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(1365, 860);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-page-splitPdf')),
          220,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('pdf-ribbon-page')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.tap(find.byKey(const Key('pdf-control-page-splitPdf')));
        await tester.pumpAndSettle();
        await capturePng('$screenshotDir/split-pdf-dialog-desktop.png');
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('pdf-tab-home')));
        await tester.pump(const Duration(milliseconds: 150));
        await tester.tap(find.byKey(const Key('pdf-control-home-convert')));
        await tester.pump(const Duration(milliseconds: 150));
        final conversionItem = find.byType(MenuItemButton).first;
        expect(tester.getSize(conversionItem).height, closeTo(28, 0.1));
        expect(tester.getSize(conversionItem).width, greaterThanOrEqualTo(140));
        await capturePng('$screenshotDir/ribbon-home-convert-menu.png');
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.tap(find.byKey(const Key('pdf-highlight-colors-home')));
        await tester.pump(const Duration(milliseconds: 150));
        await capturePng('$screenshotDir/ribbon-home-color-palette.png');
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.tap(find.byKey(const Key('pdf-tab-home')));
        await tester.pump(const Duration(milliseconds: 150));
        await capturePng('$screenshotDir/ribbon-collapsed.png');
        await tester.tap(find.byKey(const Key('pdf-tab-home')));
        await tester.pump(const Duration(milliseconds: 150));
        final desktopZoom = tester
            .widget<PdfProductShell>(find.byType(PdfProductShell))
            .zoom;
        tester.view.physicalSize = const Size(800, 600);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        await capturePng('$screenshotDir/tablet-home.png');
        await tester.tap(find.byKey(const Key('pdf-ai-rail')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-sidebar-pane')), findsNothing);
        expect(
          tester.getSize(find.byType(PdfViewer)).width,
          greaterThanOrEqualTo(399),
        );
        await capturePng('$screenshotDir/tablet-ai-open.png');
        await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(390, 844);
        await tester.pump(const Duration(milliseconds: 150));
        await tester.pump(const Duration(milliseconds: 150));
        expect(
          tester.widget<PdfProductShell>(find.byType(PdfProductShell)).zoom,
          lessThan(desktopZoom),
        );
        await capturePng('$screenshotDir/phone-home.png');
        tester.view.physicalSize = const Size(1365, 860);
        Future<void> captureRibbonState(
          PdfDocumentStatus status,
          String fileName,
        ) async {
          await tester.pumpWidget(
            RepaintBoundary(
              key: capture,
              child: MaterialApp(
                debugShowCheckedModeBanner: false,
                theme: ThemeData(fontFamily: uiFont == null ? null : 'PdfE2E'),
                home: Scaffold(
                  body: PdfProductShell(
                    definition: snapshot.chrome,
                    document: const SizedBox.expand(),
                    documentStatus: status,
                    pageNumber: 1,
                    pageCount: 2,
                    fileName: 'state-fixture.pdf',
                    zoom: 1,
                    isCommandAvailable: (_) => false,
                    onCommand: (_, _) {},
                    onGoToPage: (_) {},
                    onZoom: (_) {},
                    renderThumbnail: (_, _) async => null,
                  ),
                ),
              ),
            ),
          );
          await tester.pumpAndSettle();
          await capturePng('$screenshotDir/$fileName');
        }

        await captureRibbonState(
          const PdfDocumentStatus(hasDocument: true, hasPendingChanges: true),
          'ribbon-state-unsaved-desktop.png',
        );
        await captureRibbonState(
          const PdfDocumentStatus(
            hasDocument: true,
            hasPendingChanges: true,
            copySavePhase: PdfCopySavePhase.saving,
          ),
          'ribbon-state-saving-desktop.png',
        );
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(seconds: 1));
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
      debugDefaultTargetPlatformOverride = null;
    }
  });

  testWidgets('configured desktop directory receives a real extracted PDF', (
    tester,
  ) async {
    if (!enabled) return;
    final output = (await tester.runAsync(
      () => Directory.systemTemp.createTemp('pdf-generated-e2e-'),
    ))!;
    addTearDown(() => output.delete(recursive: true));
    final settings = OpenMuseLocalSettings();
    await settings.setGeneratedOutputDirectory(output.path);
    final fixture = File(
      '../../../ioffice/koffice/pdf/rebuild/pdf-flutter-app/assets/test/editable-two-page.pdf',
    ).absolute;
    final resource = OpenMuseResource(
      uri: fixture.uri,
      displayName: fixture.uri.pathSegments.last,
      mediaType: 'application/pdf',
    );
    final workspace = LocalWorkspaceController(
      rootPath: output.path,
      initialResources: [resource],
    );
    addTearDown(workspace.dispose);
    workspace.openResource(resource);
    final plugin = (await tester.runAsync(
      () => OpenMusePdfProductPlugin.loadBundled(
        bundleFile: File(bundlePath),
        trustedPublicKeyPem: File(keyPath).readAsStringSync(),
        expectedKeyId: keyId,
        expectedVersion: expectedVersion,
        settings: settings,
        openGeneratedDocument: (path) =>
            openGeneratedPdfInWorkspace(workspace, path),
      ),
    ))!;
    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    try {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => plugin.buildEditor(context, resource),
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 80 && find.text('第 1 页，共 2 页').evaluate().isEmpty;
        i++
      ) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.text('第 1 页，共 2 页'), findsOneWidget);
      await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const Key('pdf-tab-page')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const Key('pdf-extract-range-input')),
        '1',
      );
      await tester.tap(find.text('确定'));
      final generated = File('${output.path}/editable-two-page-p1.pdf');
      for (
        var i = 0;
        i < 80 && (!generated.existsSync() || generated.lengthSync() <= 100);
        i++
      ) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(generated.existsSync(), isTrue);
      final bytes = (await tester.runAsync(generated.readAsBytes))!;
      expect(bytes.length, greaterThan(100));
      expect(String.fromCharCodes(bytes.take(5)), '%PDF-');
      for (var i = 0; i < 20 && workspace.tabs.length < 2; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(workspace.tabs, hasLength(2));
      expect(workspace.tabs.first.resource.uri, resource.uri);
      expect(workspace.activeTab?.resource.uri, generated.uri);
      expect(find.text('第 1 页，共 2 页'), findsOneWidget);
      final evidenceDir =
          Platform.environment['PDF_PRODUCT_E2E_SCREENSHOT_DIR'];
      if (evidenceDir != null) {
        await tester.runAsync(
          () => generated.copy('$evidenceDir/extracted-one-page.pdf'),
        );
      }
    } finally {
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(seconds: 1));
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    }
  });
}
