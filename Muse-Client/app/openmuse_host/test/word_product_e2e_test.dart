import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_word_product/openmuse_word_product.dart';
import 'package:path/path.dart' as p;
import 'package:word_editor/word_editor.dart';

import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_host/src/host/resource_inspector.dart';
import 'package:openmuse_host/src/host/word_product_plugin.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  final environment = Platform.environment;
  final bundlePath = environment['WORD_STACK_BUNDLE'];
  final publicKeyPath = environment['WORD_STACK_PUBLIC_KEY'];
  final keyId = environment['WORD_STACK_KEY_ID'];
  final evidencePath = environment['WORD_STACK_E2E_EVIDENCE_PATH'];
  final appSupport = environment['OPENMUSE_E2E_SUPPORT_DIR'];
  final enabled = bundlePath != null && publicKeyPath != null && keyId != null;
  late Directory installRoot;
  late McpMarketEntry installed;
  late McpRegistryMarket market;
  late OpenMuseWordProductPlugin plugin;
  late OpenMuseResource resource;
  late int pageCount;
  late int commandCount;
  late int wordCount;
  late Uint8List documentBytes;
  late String nativeSource;
  late WordEditorController preview;

  setUpAll(() async {
    if (!enabled) return;
    if (environment['WORD_STACK_SHELL_SCREENSHOT_PATH'] != null) {
      Future<void> registerFont(String family, String file) async {
        final bytes = await File(file).readAsBytes();
        await (FontLoader(
          family,
        )..addFont(Future<ByteData>.value(ByteData.sublistView(bytes)))).load();
      }

      await registerFont('WordE2E', environment['WORD_STACK_UI_FONT']!);
      await registerFont(
        'MaterialIcons',
        environment['WORD_STACK_MATERIAL_ICONS_FONT']!,
      );
    }
    installRoot = appSupport == null
        ? await Directory.systemTemp.createTemp('openmuse-word-e2e-')
        : Directory(p.join(appSupport, 'OpenMuse', 'mcpb'));
    final bundle = await File(bundlePath).readAsBytes();
    final sample = File(
      p.normalize(
        p.join(
          Directory.current.path,
          '../../../ioffice/samples/docx-mvp-render/flutter/assets/sample1/sample2.docx',
        ),
      ),
    );
    expect(await sample.exists(), isTrue);
    final entry = McpMarketEntry(
      name: wordProductId,
      title: 'Word',
      description: 'Word product',
      version: environment['WORD_STACK_PLUGIN_VERSION']!,
      packageUri: Uri.parse('http://127.0.0.1:8765/word.mcpb'),
      sha256: sha256.convert(bundle).toString(),
    );
    market = McpRegistryMarket(
      origin: Uri.parse('http://127.0.0.1:8765'),
      installRoot: installRoot,
    );
    await market.install(entry, bundleBytes: bundle);
    installed = (await market.installedEntries()).single;
    final publicKey = await File(publicKeyPath).readAsString();
    final bundled = await VerifiedWordProduct.loadBundle(
      bundle: Uint8List.fromList(bundle),
      trustedPublicKeyPem: publicKey,
      expectedKeyId: keyId,
      expectedVersion: entry.version,
    );
    expect(bundled.definition.ribbon.tabs, isNotEmpty);
    plugin = await OpenMuseWordProductPlugin.load(
      entry: installed,
      installedDirectory: market.directoryFor(installed),
      trustedPublicKeyPem: publicKey,
      expectedKeyId: keyId,
    );
    final ribbonFile = File(
      p.join(market.directoryFor(installed).path, 'product/ribbon.json'),
    );
    final originalRibbon = await ribbonFile.readAsBytes();
    try {
      await ribbonFile.writeAsBytes([...originalRibbon, 0x20]);
      await expectLater(
        OpenMuseWordProductPlugin.load(
          entry: installed,
          installedDirectory: market.directoryFor(installed),
          trustedPublicKeyPem: publicKey,
          expectedKeyId: keyId,
        ),
        throwsFormatException,
      );
    } finally {
      await ribbonFile.writeAsBytes(originalRibbon);
    }
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, arguments) async => null,
      ),
    );
    registry.install(plugin);
    resource = await inspectLocalResource(
      OpenMuseResource(uri: sample.uri, displayName: p.basename(sample.path)),
    );
    expect(resource.mediaType, wordDocxMediaType);
    expect(registry.editorFor(resource), same(plugin));

    documentBytes = await sample.readAsBytes();
    final editor = WordEditorController();
    await editor.init();
    expect(editor.engine.ffiReady, isTrue, reason: editor.status);
    await editor.openBytes(documentBytes, name: resource.displayName);
    expect(editor.envelope, isNotNull, reason: editor.status);
    expect(editor.pageCount, greaterThan(0));
    expect(editor.envelope!.commands, isNotEmpty);
    expect(editor.wordCount, greaterThan(0));
    expect(editor.statisticsSnapshot?.pages, editor.pageCount);
    expect(editor.statisticsSnapshot?.words, editor.wordCount);
    expect(editor.statisticsSnapshot?.asianChars, greaterThan(0));
    expect(editor.statisticsSnapshot?.lines, greaterThan(0));
    expect(editor.styleSnapshot?.styles, isNotEmpty);
    expect(
      editor.styleSnapshot?.quickStyles.map((style) => style.styleId),
      contains('1'),
    );
    pageCount = editor.pageCount;
    commandCount = editor.envelope!.commands.length;
    wordCount = editor.wordCount!;
    nativeSource = editor.engine.source;
    preview = WordEditorController();
    await preview.showGolden(editor.envelope!, name: resource.displayName);
    preview.setStyleSnapshot(editor.styleSnapshot);
    preview.setWordStatistics(wordCount);
    preview.setStatisticsSnapshot(editor.statisticsSnapshot);
    editor.dispose();

    // A fresh instance can load the installed receipt and local document offline.
    final offline = await OpenMuseWordProductPlugin.load(
      entry: installed,
      installedDirectory: market.directoryFor(installed),
      trustedPublicKeyPem: publicKey,
      expectedKeyId: keyId,
    );
    expect(offline.descriptor.id, wordProductId);
    final reopened = WordEditorController();
    await reopened.init();
    await reopened.openBytes(documentBytes, name: resource.displayName);
    expect(reopened.pageCount, pageCount);
    expect(reopened.wordCount, wordCount);
    reopened.dispose();
    if (evidencePath != null) {
      await File(evidencePath).writeAsString(
        jsonEncode({
          'profile': 'openmuse-word-render-e2e/1',
          'pluginVersion': plugin.descriptor.version,
          'docxSha256': sha256.convert(documentBytes).toString(),
          'pageCount': pageCount,
          'commandCount': commandCount,
          'wordCount': wordCount,
          'statistics': {
            'asianChars': editor.statisticsSnapshot?.asianChars,
            'nonAsianWords': editor.statisticsSnapshot?.nonAsianWords,
            'charsNoSpace': editor.statisticsSnapshot?.charsNoSpace,
            'charsWithSpace': editor.statisticsSnapshot?.charsWithSpace,
            'paragraphs': editor.statisticsSnapshot?.paragraphs,
            'lines': editor.statisticsSnapshot?.lines,
          },
          'mediaType': resource.mediaType,
          'nativeEngine': nativeSource,
          'installedReceipt': installed.sha256,
        }),
      );
    }
  });

  tearDownAll(() async {
    if (enabled) {
      preview.dispose();
      if (appSupport == null) await installRoot.delete(recursive: true);
    }
  });

  testWidgets(
    'every signed GenOffice glyph parses in the native Flutter surface',
    (tester) async {
      if (!enabled) return;
      final icons = plugin.definition.icons;
      expect(icons.length, 75);
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Wrap(
              children: [
                for (final svg in icons.values)
                  SvgPicture.string(
                    svg.replaceFirst('__STROKE_WIDTH__', '1.2'),
                    width: 20,
                    height: 20,
                  ),
              ],
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.byType(SvgPicture), findsNWidgets(75));
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('installed Word product opens and renders DOCX in OpenMuse', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1365, 860);
    tester.view.devicePixelRatio = 1;
    final capture = GlobalKey();
    try {
      await tester.pumpWidget(
        RepaintBoundary(
          key: capture,
          child: MaterialApp(
            debugShowCheckedModeBanner: false,
            theme: ThemeData(
              fontFamily: 'WordE2E',
              platform: TargetPlatform.macOS,
            ),
            home: Scaffold(
              body: WordProductShell(
                definition: plugin.definition,
                controller: preview,
                isCommandAvailable: (id) =>
                    id == 'word.view.zoomIn' ||
                    id == 'word.view.zoomOut' ||
                    id == 'word.view.zoomDialog' ||
                    id == 'word.view.zoom100' ||
                    id == 'word.view.fitWidth' ||
                    id == 'word.view.fitPage' ||
                    id == 'word.view.fitMultiplePages' ||
                    id == 'word.view.gridlines' ||
                    id == 'word.view.darkMode' ||
                    id == 'word.view.ruler' ||
                    id == 'word.tools.wordCount' ||
                    id == 'word.navigation.goTo' ||
                    id == 'word.view.printLayout' ||
                    id == 'word.view.readMode',
                onCommand: (_) {},
                document: WordWorkspace(
                  controller: preview,
                  showChrome: false,
                  readOnly: true,
                  canvasColor: plugin.definition.chrome.canvas,
                  pageTopInset: plugin.definition.chrome.pageTopInset,
                  pageBottomInset: plugin.definition.chrome.pageBottomInset,
                  pageGap: plugin.definition.chrome.pageGap,
                ),
              ),
            ),
          ),
        ),
      );
      await tester.pump();
      expect(
        plugin.buildEditor(tester.element(find.byType(Scaffold)), resource),
        isA<WordProductSurface>(),
      );
      expect(find.byType(WordProductShell), findsOneWidget);
      expect(find.byType(WordWorkspace), findsOneWidget);
      expect(find.byType(CustomPaint), findsWidgets);
      expect(find.text('开始'), findsOneWidget);
      expect(preview.pageCount, pageCount, reason: preview.status);
      expect(find.textContaining('共 $pageCount 页'), findsOneWidget);
      expect(find.text('$wordCount 个字'), findsOneWidget);
      final screenshotPath = environment['WORD_STACK_SHELL_SCREENSHOT_PATH'];
      if (screenshotPath != null) {
        final boundary =
            capture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(screenshotPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-status-word-count')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-statistics-dialog')), findsOneWidget);
        expect(
          find.text('${preview.statisticsSnapshot!.asianChars}'),
          findsWidgets,
        );
        expect(find.byKey(const Key('word-statistics-words')), findsOneWidget);
        final statisticsPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-statistics.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(statisticsPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-statistics-close')));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-status-page')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-go-to-panel')), findsOneWidget);
        await tester.enterText(find.byKey(const Key('word-go-to-input')), '4');
        await tester.tap(find.text('定位').last);
        await tester.pumpAndSettle();
        expect(preview.visiblePage, 4);
        final goToPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-go-to.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(goToPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-go-to-close')));
        preview.navigateToPage(1);
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-status-view-read')));
        await tester.pumpAndSettle();
        expect(preview.readingMode, isTrue);
        expect(find.byKey(const Key('word-ribbon')), findsNothing);
        expect(find.byType(WordWorkspace), findsOneWidget);
        final desktopReadPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-read-mode.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(desktopReadPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-read-exit')));
        await tester.pumpAndSettle();
        expect(preview.readingMode, isFalse);
        expect(find.byKey(const Key('word-ribbon')), findsOneWidget);
        expect(find.byKey(const Key('word-inline-style-1')), findsOneWidget);
        await tester.tap(find.byKey(const Key('word-control-styles-more')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-menu-document.style.paragraph.1')),
          findsOneWidget,
        );
        final stylesPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-styles.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(stylesPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-tab-insert')));
        await tester.pump();
        await tester.tap(find.byKey(const Key('word-control-cover-page')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-cover-preview-insert.cover.annual')),
          findsOneWidget,
        );
        final coverPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-cover.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(coverPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.pumpAndSettle();
        await tester.ensureVisible(
          find.byKey(const Key('word-control-word-art')),
        );
        await tester.tap(find.byKey(const Key('word-control-word-art')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(
            const Key('word-wordart-preview-insert.wordArt.silver-dark'),
          ),
          findsOneWidget,
        );
        final wordArtPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-wordart.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(wordArtPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-control-shapes')));
        await tester.pump();
        await tester.pump(const Duration(milliseconds: 200));
        expect(find.text('线条'), findsOneWidget);
        expect(
          find.byKey(const Key('word-menu-insert.shapes.line')),
          findsOneWidget,
        );
        final menuPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-menu.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(menuPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.ensureVisible(
          find.byKey(const Key('word-control-drop-cap')),
        );
        await tester.tap(find.byKey(const Key('word-control-drop-cap')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-menu-dropcap.margin')),
          findsOneWidget,
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.ensureVisible(
          find.byKey(const Key('word-control-equation')),
        );
        await tester.tap(find.byKey(const Key('word-control-equation')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-menu-equation.style.8')),
          findsOneWidget,
        );
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.ensureVisible(
          find.byKey(const Key('word-control-page-number')),
        );
        await tester.tap(find.byKey(const Key('word-control-page-number')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-menu-pagenum.top.1')),
          findsOneWidget,
        );
        expect(
          find.byKey(const Key('word-menu-pagenum.bottom.3')),
          findsOneWidget,
        );
        expect(find.byKey(const Key('word-menu-pagenum.top')), findsNothing);
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.tap(find.byKey(const Key('word-tab-layout')));
        await tester.pump();
        await tester.ensureVisible(
          find.byKey(const Key('word-control-position')),
        );
        await tester.tap(find.byKey(const Key('word-control-position')));
        await tester.pumpAndSettle();
        for (final row in ['top', 'center', 'bottom']) {
          for (var column = 1; column <= 3; column++) {
            expect(
              find.byKey(Key('word-position-preview-position.$row.$column')),
              findsOneWidget,
            );
          }
        }
        final positionPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-position.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(positionPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.tap(find.byKey(const Key('word-tab-home')));
        await tester.pump();
        await tester.ensureVisible(
          find.byKey(const Key('word-control-bullets')),
        );
        await tester.tap(find.byKey(const Key('word-control-bullets')));
        await tester.pumpAndSettle();
        expect(
          find.byKey(const Key('word-menu-bullets.glyph.6')),
          findsOneWidget,
        );
        expect(
          find.byKey(const Key('word-menu-bullets.level.9')),
          findsOneWidget,
        );
        final listsPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-lists.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(listsPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.sendKeyEvent(LogicalKeyboardKey.escape);
        await tester.tap(find.byKey(const Key('word-status-zoom-value')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-zoom-dialog')), findsOneWidget);
        expect(find.text('页宽'), findsOneWidget);
        final zoomPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-zoom.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(zoomPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.text('取消'));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-tab-view')));
        await tester.pumpAndSettle();
        final viewRibbonPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-view-ribbon.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(viewRibbonPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        final fitPage = find.byKey(const Key('word-control-whole-page'));
        await tester.ensureVisible(fitPage);
        await tester.tap(fitPage);
        await tester.pumpAndSettle();
        expect(preview.zoomFitMode, WordZoomFitMode.wholePage);
        expect(preview.zoom, lessThan(1));
        final fitPagePath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-fit-page.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(fitPagePath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        final multiplePages = find.byKey(const Key('word-control-multi-page'));
        await tester.ensureVisible(multiplePages);
        await tester.tap(multiplePages);
        await tester.pumpAndSettle();
        expect(preview.zoomFitMode, WordZoomFitMode.multiplePages);
        expect(preview.multiplePages, isTrue);
        final firstPaper = find.byKey(const Key('word-paper-page-1'));
        final secondPaper = find.byKey(const Key('word-paper-page-2'));
        expect(firstPaper, findsOneWidget);
        expect(secondPaper, findsOneWidget);
        expect(
          tester.getTopLeft(secondPaper).dx,
          greaterThan(tester.getTopLeft(firstPaper).dx),
        );
        expect(
          tester.getTopLeft(secondPaper).dy,
          closeTo(tester.getTopLeft(firstPaper).dy, 1),
        );
        preview.navigateToPage(4);
        await tester.pumpAndSettle();
        await tester.pump(const Duration(milliseconds: 160));
        expect(preview.visiblePage, 4);
        preview.navigateToPage(2);
        await tester.pumpAndSettle();
        expect(preview.visiblePage, 2);
        final multiplePath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-multiple-pages.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(multiplePath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        final zoom100 = find.byKey(const Key('word-control-zoom-100'));
        await tester.ensureVisible(zoom100);
        await tester.tap(zoom100);
        await tester.pumpAndSettle();
        expect(preview.zoom, closeTo(1, 0.001));
        final fitWidth = find.byKey(const Key('word-control-page-width'));
        await tester.ensureVisible(fitWidth);
        await tester.tap(fitWidth);
        await tester.pumpAndSettle();
        expect(preview.zoomFitMode, WordZoomFitMode.pageWidth);
        expect(preview.zoom, greaterThan(1));
        final gridlines = find.byKey(const Key('word-control-gridlines'));
        await tester.ensureVisible(gridlines);
        await tester.pumpAndSettle();
        final gridOffPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-grid-off.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(gridOffPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(gridlines);
        await tester.pumpAndSettle();
        expect(preview.showGridlines, isTrue);
        expect(preview.pageCount, pageCount);
        expect(preview.envelope!.commands.length, commandCount);
        final gridOnPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-grid-on.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(gridOnPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(gridlines);
        await tester.pumpAndSettle();
        expect(preview.showGridlines, isFalse);
        final darkMode = find.byKey(const Key('word-control-dark-mode'));
        await tester.ensureVisible(darkMode);
        await tester.pumpAndSettle();
        final darkOffPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-dark-off.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(darkOffPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(darkMode);
        await tester.pumpAndSettle();
        expect(preview.darkPage, isTrue);
        expect(preview.pageCount, pageCount);
        expect(preview.envelope!.commands.length, commandCount);
        final darkOnPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-dark-on.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(darkOnPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(darkMode);
        await tester.pumpAndSettle();
        expect(preview.darkPage, isFalse);
        preview.navigateToPage(1);
        await tester.pumpAndSettle();
        final ruler = find.byKey(const Key('word-control-ruler'));
        await tester.ensureVisible(ruler);
        await tester.pumpAndSettle();
        final rulerOffPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-ruler-off.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(rulerOffPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(ruler);
        await tester.pumpAndSettle();
        expect(preview.showRuler, isTrue);
        final pageRuler = find.byKey(const Key('word-page-ruler'));
        expect(pageRuler, findsOneWidget);
        expect(
          tester.getTopLeft(pageRuler).dx,
          closeTo(
            tester.getTopLeft(find.byKey(const Key('word-paper-page-1'))).dx,
            1,
          ),
        );
        expect(preview.pageCount, pageCount);
        expect(preview.envelope!.commands.length, commandCount);
        final rulerOnPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-ruler-on.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(rulerOnPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        preview.setZoomFitMode(WordZoomFitMode.multiplePages);
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-page-ruler')), findsNothing);
        preview.setZoomFitMode(WordZoomFitMode.pageWidth);
        await tester.pumpAndSettle();
        await tester.tap(ruler);
        await tester.pumpAndSettle();
        expect(preview.showRuler, isFalse);
        await tester.tap(find.byKey(const Key('word-tab-home')));
        await tester.pumpAndSettle();
        tester.view.physicalSize = const Size(390, 844);
        await tester.pump();
        await tester.pump();
        expect(tester.getSize(find.byKey(const Key('word-ribbon'))).height, 45);
        expect(preview.zoom, lessThan(1.0));
        expect(tester.takeException(), isNull);
        final mobilePath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(mobilePath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-status-word-count')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-statistics-dialog')), findsOneWidget);
        expect(tester.takeException(), isNull);
        final mobileStatisticsPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-statistics-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            mobileStatisticsPath,
          ).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-statistics-close')));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-status-page')));
        await tester.pumpAndSettle();
        expect(find.byKey(const Key('word-go-to-panel')), findsOneWidget);
        expect(tester.takeException(), isNull);
        final mobileGoToPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-go-to-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(mobileGoToPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-go-to-close')));
        await tester.pumpAndSettle();
        final viewTab = find.byKey(const Key('word-tab-view'));
        await tester.scrollUntilVisible(
          viewTab,
          180,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('word-tab-row')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        await tester.tap(viewTab);
        await tester.pumpAndSettle();
        final mobileFitPage = find.byKey(const Key('word-control-whole-page'));
        await tester.scrollUntilVisible(
          mobileFitPage,
          160,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('word-ribbon')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        for (var attempt = 0; attempt < 12; attempt++) {
          final x = tester.getCenter(mobileFitPage).dx;
          if (x >= 30 && x <= 360) break;
          await tester.drag(
            find.byKey(const Key('word-ribbon')),
            Offset(x < 30 ? 160 : -160, 0),
          );
          await tester.pumpAndSettle();
        }
        expect(tester.getCenter(mobileFitPage).dx, inInclusiveRange(30, 360));
        await tester.tap(mobileFitPage);
        await tester.pumpAndSettle();
        expect(preview.zoomFitMode, WordZoomFitMode.wholePage);
        final mobileFitPagePath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-fit-page-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            mobileFitPagePath,
          ).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        final mobileRuler = find.byKey(const Key('word-control-ruler'));
        await tester.scrollUntilVisible(
          mobileRuler,
          160,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('word-ribbon')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        for (var attempt = 0; attempt < 12; attempt++) {
          final x = tester.getCenter(mobileRuler).dx;
          if (x >= 30 && x <= 360) break;
          await tester.drag(
            find.byKey(const Key('word-ribbon')),
            Offset(x < 30 ? 160 : -160, 0),
          );
          await tester.pumpAndSettle();
        }
        preview.navigateToPage(1);
        await tester.pumpAndSettle();
        await tester.tap(mobileRuler);
        await tester.pumpAndSettle();
        expect(preview.showRuler, isTrue);
        expect(find.byKey(const Key('word-page-ruler')), findsOneWidget);
        expect(
          tester.getSize(find.byKey(const Key('word-page-ruler'))).width,
          lessThan(390),
        );
        expect(tester.takeException(), isNull);
        final mobileRulerPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-ruler-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(mobileRulerPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(mobileRuler);
        await tester.pumpAndSettle();
        expect(preview.showRuler, isFalse);
        await tester.tap(find.byKey(const Key('word-status-zoom-value')));
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const Key('word-zoom-choice-6')));
        await tester.tap(find.byKey(const Key('word-zoom-apply')));
        await tester.pumpAndSettle();
        expect(preview.zoomFitMode, WordZoomFitMode.multiplePages);
        expect(preview.multiplePages, isTrue);
        expect(
          tester.getSize(find.byKey(const Key('word-paper-page-1'))).width,
          lessThan(390),
        );
        final mobileMultiplePath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-multiple-pages-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            mobileMultiplePath,
          ).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        final mobileGrid = find.byKey(const Key('word-control-gridlines'));
        await tester.scrollUntilVisible(
          mobileGrid,
          160,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('word-ribbon')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        for (var attempt = 0; attempt < 12; attempt++) {
          final x = tester.getCenter(mobileGrid).dx;
          if (x >= 30 && x <= 360) break;
          await tester.drag(
            find.byKey(const Key('word-ribbon')),
            Offset(x < 30 ? 160 : -160, 0),
          );
          await tester.pumpAndSettle();
        }
        expect(tester.getCenter(mobileGrid).dx, inInclusiveRange(30, 360));
        await tester.tap(mobileGrid);
        await tester.pumpAndSettle();
        expect(preview.showGridlines, isTrue);
        final mobileGridPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-grid-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(mobileGridPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(mobileGrid);
        await tester.pumpAndSettle();
        final readControl = find.byKey(const Key('word-control-read-mode'));
        await tester.scrollUntilVisible(
          readControl,
          -160,
          scrollable: find
              .descendant(
                of: find.byKey(const Key('word-ribbon')),
                matching: find.byType(Scrollable),
              )
              .first,
        );
        for (var attempt = 0; attempt < 8; attempt++) {
          final x = tester.getCenter(readControl).dx;
          if (x >= 30 && x <= 360) break;
          await tester.drag(
            find.byKey(const Key('word-ribbon')),
            Offset(x < 30 ? 160 : -160, 0),
          );
          await tester.pumpAndSettle();
        }
        expect(tester.getCenter(readControl).dx, inInclusiveRange(30, 360));
        await tester.tap(readControl);
        await tester.pumpAndSettle();
        expect(preview.readingMode, isTrue);
        expect(find.byKey(const Key('word-ribbon')), findsNothing);
        final mobileReadPath = screenshotPath.replaceFirst(
          RegExp(r'\.png$'),
          '-read-mode-mobile.png',
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 1);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(mobileReadPath).writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
        await tester.tap(find.byKey(const Key('word-read-exit')));
        await tester.pumpAndSettle();
        expect(preview.readingMode, isFalse);
      }
    } finally {
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    }
  }, skip: !enabled);
}
