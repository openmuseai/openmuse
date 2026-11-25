import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart'
    show TargetPlatform, debugDefaultTargetPlatformOverride;
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:pdfrx/pdfrx.dart';
import 'package:pdf_rebuild_flutter/pdf_product_chrome.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';
import 'package:pdf_rebuild_flutter/pdf_page_operations.dart';

import 'package:openmuse_host/src/host/pdf_product_plugin.dart';
import 'package:openmuse_host/src/host/local_settings.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'OpenMuse opens and navigates the supplied 160-page PDF',
    (tester) async {
      final samplePath = Platform.environment['PDF_PRODUCT_SAMPLE_PATH'];
      final bundlePath = Platform.environment['PDF_PRODUCT_E2E_BUNDLE'];
      final publicKeyPath = Platform.environment['PDF_PRODUCT_E2E_PUBLIC_KEY'];
      final keyId = Platform.environment['PDF_PRODUCT_E2E_KEY_ID'];
      final version = Platform.environment['PDF_PRODUCT_E2E_VERSION'];
      final evidenceDir =
          Platform.environment['PDF_PRODUCT_E2E_SCREENSHOT_DIR'];
      if ([
        samplePath,
        bundlePath,
        publicKeyPath,
        keyId,
        version,
        evidenceDir,
      ].contains(null)) {
        fail('The signed product, sample, and evidence directory are required');
      }

      late OpenMusePdfProductPlugin plugin;
      late OpenMuseResource resource;
      late ResourceEditorSession session;
      late Directory generatedDirectory;
      String? generatedPath;
      await tester.runAsync(() async {
        final sample = File(samplePath!);
        expect(await sample.exists(), isTrue);
        final sampleBytes = await sample.readAsBytes();
        generatedDirectory = await Directory.systemTemp.createTemp(
          'openmuse-pdf-sample-extract-',
        );
        final settings = OpenMuseLocalSettings()
          ..generatedOutputDirectory = generatedDirectory.path;
        resource = OpenMuseResource(
          uri: sample.uri,
          displayName: sample.uri.pathSegments.last,
          mediaType: 'application/pdf',
        );
        plugin = await OpenMusePdfProductPlugin.loadBundled(
          bundleFile: File(bundlePath!),
          trustedPublicKeyPem: await File(publicKeyPath!).readAsString(),
          expectedKeyId: keyId!,
          expectedVersion: version!,
          settings: settings,
          openGeneratedDocument: (path) async => generatedPath = path,
        );
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => null,
          ),
        )..install(plugin);
        expect(registry.editorFor(resource), same(plugin));
        final record = ResourceRecord(
          identity: ResourceIdentity(
            scope: 'sample-qa',
            source: 'local',
            workspace: 'w',
            mount: 'downloads',
            resource: 'sample.pdf',
          ),
          version: ResourceVersion(
            revision: 'v1',
            digest: resourceDigest(sampleBytes),
          ),
          descriptor: MuseResourceDescriptorV1(
            resourceRef: 'sample.pdf',
            revision: 'v1',
            displayName: resource.displayName,
            mediaType: 'application/pdf',
            format: const MuseResourceFormat(
              formatId: 'pdf',
              confidence: MuseFormatConfidence.claimed,
            ),
            capabilities: const [
              MuseResourceCapability.describe,
              MuseResourceCapability.materialize,
            ],
            security: const MuseResourceSecurity(
              classification: MuseSecurityClassification.internal,
              activeContent: MuseActiveContent.unknown,
            ),
          ),
          sourceLabel: 'Downloads',
          location: const ['Downloads'],
          parentRef: 'root',
          metadataRevision: 1,
        );
        expect(
          await plugin.probeResource(record, ResourceOpenIntent.view),
          isTrue,
        );
        session = ResourceEditorSession(
          opened: OpenedResource(
            reference: ResourceReference(identity: record.identity),
            content: ResourceContent(record, sampleBytes),
          ),
          allowEditing: false,
          commitContent: (_, _, _) => throw UnsupportedError('read only'),
          onOpened: (_) {},
          onChanged: (_) {},
        );
        final backend = PdfProductBackend();
        final document = await backend.open(
          sampleBytes,
          sample.uri.pathSegments.last,
        );
        expect(document.pageCount, 160);
        for (final page in [0, 159]) {
          final raster = await backend.renderPageAt(page, 612, 792);
          final dark = raster.rgba.where((value) => value < 100).length;
          debugPrint('Backend page ${page + 1} dark channels: $dark');
          expect(dark, greaterThan(100));
        }
        final region = await backend.renderPageRegion(
          0,
          0,
          0,
          612,
          792,
          612,
          792,
        );
        debugPrint(
          'Backend first-page region dark channels: ${region.rgba.where((value) => value < 100).length}',
        );
        backend.close();
        await Directory(evidenceDir!).create(recursive: true);
      });

      debugDefaultTargetPlatformOverride = TargetPlatform.macOS;
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      final uiFont = Platform.environment['PDF_PRODUCT_E2E_UI_FONT'];
      final iconFont = Platform.environment['PDF_PRODUCT_E2E_ICON_FONT'];
      if (uiFont != null && iconFont != null) {
        await tester.runAsync(() async {
          for (final (family, path) in [
            ('PdfE2E', uiFont),
            ('MaterialIcons', iconFont),
          ]) {
            final bytes = await File(path).readAsBytes();
            await (FontLoader(
                  family,
                )..addFont(Future<ByteData>.value(ByteData.sublistView(bytes))))
                .load();
          }
        });
      }

      final capture = GlobalKey();
      Future<void> screenshot(String name) async {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage();
          final png = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '$evidenceDir/$name',
          ).writeAsBytes(png!.buffer.asUint8List());
          image.dispose();
        });
      }

      Future<int> pageInkCount() async {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        return await tester.runAsync<int>(() async {
              final image = await boundary.toImage();
              final rgba = (await image.toByteData(
                format: ui.ImageByteFormat.rawRgba,
              ))!.buffer.asUint8List();
              var ink = 0;
              for (var y = 160; y < 800; y += 2) {
                for (var x = 550; x < 1100; x += 2) {
                  final i = (y * image.width + x) * 4;
                  if (rgba[i] < 100 && rgba[i + 1] < 100 && rgba[i + 2] < 100) {
                    ink++;
                  }
                }
              }
              image.dispose();
              return ink;
            }) ??
            0;
      }

      Future<void> waitForPageInk() async {
        for (var attempt = 0; attempt < 220; attempt++) {
          await tester.pump(const Duration(milliseconds: 20));
          if (attempt % 10 == 0 && await pageInkCount() > 100) return;
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 10)),
          );
        }
        fail('The product viewer did not paint PDF page content');
      }

      try {
        await tester.pumpWidget(
          RepaintBoundary(
            key: capture,
            child: MaterialApp(
              debugShowCheckedModeBanner: false,
              theme: ThemeData(fontFamily: uiFont == null ? null : 'PdfE2E'),
              home: Scaffold(
                body: Builder(
                  builder: (context) => plugin.buildResourceEditor(
                    context,
                    plugin.resourceHandlers.single.id,
                    session,
                  ),
                ),
              ),
            ),
          ),
        );
        for (
          var i = 0;
          i < 200 && find.text('第 1 页，共 160 页').evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump();
        }
        expect(find.byType(PdfWorkspace), findsOneWidget);
        expect(find.byType(PdfProductShell), findsOneWidget);
        expect(find.text('第 1 页，共 160 页'), findsOneWidget);
        final viewer = tester.widget<PdfViewer>(find.byType(PdfViewer));
        expect(viewer.controller!.pageCount, 160);
        await waitForPageInk();
        await screenshot('sample-home-page-1.png');

        await tester.tap(find.byKey(const Key('pdf-control-home-properties')));
        for (
          var i = 0;
          i < 100 && find.text('pdfTeX-1.40.21').evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 50)),
          );
          await tester.pump();
        }
        expect(find.byKey(const Key('pdf-properties-dialog')), findsOneWidget);
        expect(find.text('pdfTeX-1.40.21'), findsOneWidget);
        expect(find.text('2104.13478v21113333 - English.pdf'), findsOneWidget);
        expect(find.text('1.5'), findsOneWidget);
        expect(find.text('7.0 MB'), findsOneWidget);
        await tester.pumpAndSettle();
        await screenshot('sample-properties.png');
        await tester.tap(find.text('确定'));
        await tester.pumpAndSettle();

        await tester.tap(find.byKey(const Key('pdf-tab-view')));
        await tester.pump();
        await tester.tap(find.byKey(const Key('pdf-control-view-outline')));
        for (
          var i = 0;
          i < 400 &&
              find.byKey(const Key('pdf-outline-list')).evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 50)),
          );
          await tester.pump();
        }
        await screenshot('sample-view-outline.png');
        expect(find.byKey(const Key('pdf-outline-list')), findsOneWidget);
        await tester.tap(find.text('1 Introduction'));
        for (
          var i = 0;
          i < 100 && find.text('第 1 页，共 160 页').evaluate().isNotEmpty;
          i++
        ) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 50)),
          );
          await tester.pump();
        }
        expect(find.text('第 8 页，共 160 页'), findsOneWidget);
        await waitForPageInk();
        await screenshot('sample-outline-navigation.png');
        tester
            .widget<PdfProductShell>(find.byType(PdfProductShell))
            .onGoToPage(1);
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('pdf-control-view-thumbs')));
        await tester.pumpAndSettle();

        for (final tab in [
          'annotate',
          'edit',
          'page',
          'view',
          'fillForm',
          'formDesign',
        ]) {
          await tester.tap(find.byKey(Key('pdf-tab-$tab')));
          await tester.pump(const Duration(milliseconds: 120));
          await screenshot('sample-ribbon-$tab.png');
        }
        await tester.tap(find.byKey(const Key('pdf-tab-page')));
        await tester.pumpAndSettle();
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-page-splitPdf')),
          240,
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
        await screenshot('sample-split-dialog.png');
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        await tester.scrollUntilVisible(
          find.byKey(const Key('pdf-control-page-extract')),
          -240,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('pdf-ribbon-page')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
        await tester.pumpAndSettle();
        await screenshot('sample-extract-range-dialog.png');
        await tester.enterText(
          find.byKey(const Key('pdf-extract-range-input')),
          '1',
        );
        await tester.tap(find.text('确定'));
        for (var i = 0; i < 100 && generatedPath == null; i++) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump();
        }
        expect(generatedPath, isNotNull);
        final extracted = File(generatedPath!);
        expect(await tester.runAsync(extracted.exists), isTrue);
        final extractedBytes = (await tester.runAsync(extracted.readAsBytes))!;
        expect(
          await tester.runAsync(
            () => createPdfPageOperations().pageCount(extractedBytes),
          ),
          1,
        );
        await tester.runAsync(
          () => extracted.copy('$evidenceDir/sample-ui-extracted-p1.pdf'),
        );
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('pdf-extract-dialog')), findsNothing);
        await screenshot('sample-extract-success.png');
        expect(find.text('第 1 页，共 160 页'), findsOneWidget);
        await tester.tap(find.byKey(const Key('pdf-tab-view')));
        await tester.pump();
        await tester.tap(find.byKey(const Key('pdf-control-view-spread')));
        await tester.pumpAndSettle();
        expect(
          viewer.controller!.layout.pageLayouts[0].top,
          viewer.controller!.layout.pageLayouts[1].top,
        );
        expect(find.text('单页'), findsOneWidget);
        await screenshot('sample-view-spread.png');
        await tester.tap(find.byKey(const Key('pdf-control-view-spread')));
        await tester.pumpAndSettle();

        await tester.tap(find.byKey(const Key('pdf-control-view-night')));
        await tester.pumpAndSettle();
        expect(
          tester
              .widget<PdfViewer>(find.byType(PdfViewer))
              .params
              .backgroundColor,
          const Color(0xff1e1e1e),
        );
        await screenshot('sample-view-night.png');
        await tester.tap(find.byKey(const Key('pdf-control-view-night')));
        await tester.pumpAndSettle();

        await tester.tap(find.byKey(const Key('pdf-tab-home')));
        await tester.pump();
        await tester.tap(find.byKey(const Key('pdf-control-home-search')));
        await tester.pump();
        await tester.enterText(
          find.byKey(const Key('pdf-search-input')),
          'Geometric Deep Learning',
        );
        await tester.pump(const Duration(milliseconds: 600));
        final searchCount = find.byWidgetPredicate(
          (widget) =>
              widget is Text &&
              RegExp(
                r'^[1-9][0-9]* / [1-9][0-9]*$',
              ).hasMatch(widget.data ?? ''),
        );
        for (var i = 0; i < 100 && searchCount.evaluate().isEmpty; i++) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump();
        }
        expect(searchCount, findsOneWidget);
        await screenshot('sample-search-results.png');
        await tester.tap(find.byKey(const Key('pdf-search-close')));
        await tester.pumpAndSettle();

        tester
            .widget<PdfProductShell>(find.byType(PdfProductShell))
            .onGoToPage(160);
        for (
          var i = 0;
          i < 80 && find.text('第 160 页，共 160 页').evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () async => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump();
        }
        await waitForPageInk();
        await screenshot('sample-page-160.png');
        expect(find.text('第 160 页，共 160 页'), findsOneWidget);
        expect(tester.takeException(), isNull);
      } finally {
        session.dispose();
        await tester.runAsync(() => generatedDirectory.delete(recursive: true));
        debugDefaultTargetPlatformOverride = null;
        tester.view.resetPhysicalSize();
        tester.view.resetDevicePixelRatio();
      }
    },
    timeout: const Timeout(Duration(minutes: 3)),
  );
}
