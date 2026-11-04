import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/main.dart';
import 'package:pdf_rebuild_flutter/pdf_product_chrome.dart';
import 'package:pdf_rebuild_flutter/pdf_view_navigation.dart';
import 'package:pdf_rebuild_flutter/pdf_shell_layout.dart';
import 'package:pdf_rebuild_flutter/pdf_page_arrangement.dart';
import 'package:pdf_rebuild_flutter/pdf_document_status.dart';
import 'package:shared_preferences/shared_preferences.dart';

Future<void> loadShell(WidgetTester tester) async {
  final definition = PdfChromeDefinition.fromJson(
    jsonDecode(File('assets/product/chrome.json').readAsStringSync())
        as Map<String, dynamic>,
    iconData:
        jsonDecode(File('assets/product/icons.json').readAsStringSync())
            as Map<String, dynamic>,
  );
  await tester.pumpWidget(PdfRebuildApp(chrome: definition));
  for (
    var attempt = 0;
    attempt < 20 && find.byType(PdfProductShell).evaluate().isEmpty;
    attempt++
  ) {
    await tester.pump(const Duration(milliseconds: 50));
  }
  expect(find.byType(PdfProductShell), findsOneWidget);
}

void main() {
  test('GenOffice spread pairs pages and steps by row', () {
    final pages = [
      const Size(595, 842),
      const Size(612, 792),
      const Size(420, 595),
    ];
    final single = PdfArrangementGeometry.layout(
      pages,
      arrangement: PdfPageArrangement.single,
      horizontalPadding: 24,
      verticalPadding: 16,
      pageGap: 16,
    );
    expect(single.pageRects[1].top, greaterThan(single.pageRects[0].bottom));
    final spread = PdfArrangementGeometry.layout(
      pages,
      arrangement: PdfPageArrangement.spread,
      horizontalPadding: 24,
      verticalPadding: 16,
      pageGap: 16,
    );
    expect(spread.pageRects[0].top, spread.pageRects[1].top);
    expect(spread.pageRects[1].left - spread.pageRects[0].right, 16);
    expect(spread.pageRects[2].top, spread.pageRects[0].bottom + 16);
    expect(spread.rowBounds(2, PdfPageArrangement.spread).width, 1223);
    expect(spread.documentSize.width, 1271);
    expect(PdfPageArrangement.spread.stepPage(1, 1, 3), 3);
    expect(PdfPageArrangement.spread.stepPage(3, -1, 3), 1);
    expect(PdfPageArrangement.spread.stepPage(3, 1, 3), 3);
  });

  testWidgets('view ribbon toggles spread and keyboard steps whole rows', (
    tester,
  ) async {
    SharedPreferences.setMockInitialValues({});
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
      iconData:
          jsonDecode(File('assets/product/icons.json').readAsStringSync())
              as Map<String, dynamic>,
    );
    final commands = <String>[];
    final pages = <int>[];
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 2,
            pageCount: 5,
            pageArrangement: PdfPageArrangement.spread,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (_) => true,
            onCommand: (command, _) => commands.add(command),
            onGoToPage: pages.add,
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    await tester.tap(find.byKey(const Key('pdf-tab-view')));
    await tester.pump();
    expect(find.text('单页'), findsOneWidget);
    await tester.tap(find.byKey(const Key('pdf-control-view-spread')));
    expect(commands, ['pdf.view.toggleSpread']);
    tester
        .widget<Focus>(find.byKey(const Key('pdf-shell-keyboard')))
        .focusNode!
        .requestFocus();
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowRight);
    expect(pages, [3]);
  });
  test('signed responsive columns preserve the PDF viewport', () {
    final policy = PdfShellLayoutPolicy.fromJson(const {
      'mobileBelow': 600,
      'compactBelow': 900,
      'minDocumentWidth': 400,
    });
    PdfShellColumns resolve(double width, {bool explicit = false}) =>
        policy.resolve(
          width: width,
          aiPreferredWidth: 440,
          aiMinimumWidth: 280,
          aiRailWidth: 34,
          sidebarWidth: 153,
          aiRequested: true,
          aiExplicit: explicit,
          sidebarRequested: true,
        );
    final desktop = resolve(1365);
    expect(desktop.aiPanel, isTrue);
    expect(desktop.sidebar, isTrue);
    final tablet = resolve(800);
    expect(tablet.compact, isTrue);
    expect(tablet.aiPanel, isFalse);
    expect(tablet.sidebar, isTrue);
    final tabletOpen = resolve(800, explicit: true);
    expect(tabletOpen.aiPanel, isTrue);
    expect(tabletOpen.sidebar, isFalse);
    expect(tabletOpen.aiWidth, 400);
    final noSidebar = policy.resolve(
      width: 800,
      aiPreferredWidth: 440,
      aiMinimumWidth: 280,
      aiRailWidth: 34,
      sidebarWidth: 153,
      aiRequested: true,
      aiExplicit: false,
      sidebarRequested: false,
    );
    expect(noSidebar.aiPanel, isTrue);
    expect(resolve(650, explicit: true).aiPanel, isFalse);
    expect(resolve(390).mobile, isTrue);
  });
  test('legacy PDF chrome retains horizontal page keys', () {
    final navigation = PdfKeyboardNavigation.fromJson(null);
    expect(navigation.actionFor('ArrowRight'), PdfNavigationAction.pageNext);
    expect(navigation.actionFor('ArrowLeft'), PdfNavigationAction.pagePrevious);
  });
  test('signed chrome selects matching GenOffice light and dark tokens', () {
    final chrome =
        jsonDecode(File('assets/product/chrome.json').readAsStringSync())
            as Map<String, dynamic>;
    final light = PdfChromeDefinition.fromJson(chrome);
    final dark = PdfChromeDefinition.fromJson(chrome, themeMode: 'dark');
    expect(light.color('chrome'), const Color(0xffffffff));
    expect(light.color('canvas'), const Color(0xffe6e6e6));
    expect(light.color('status'), const Color(0xfff3f4f6));
    expect(light.color('saveOk'), const Color(0xff217346));
    expect(light.pageViewport.horizontalPadding, 24);
    expect(light.pageViewport.verticalPadding, 16);
    expect(light.pageViewport.pageGap, 16);
    expect(
      light.pageViewport.pageShadow.color,
      const Color(0xff000000).withValues(alpha: 0.18),
    );
    expect(dark.color('chrome'), const Color(0xff252525));
    expect(dark.color('accent'), const Color(0xffe8663f));
    expect(dark.color('saveOk'), const Color(0xff58b380));
    expect(
      () => PdfChromeDefinition.fromJson(chrome, themeMode: 'sepia'),
      throwsFormatException,
    );
  });

  testWidgets('signed PDF shortcuts search and navigate without editing text', (
    tester,
  ) async {
    SharedPreferences.setMockInitialValues({});
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    final commands = <String>[];
    final pages = <int>[];
    final navigation = <PdfNavigationAction>[];
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 2,
            pageCount: 3,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (_) => true,
            onCommand: (command, _) => commands.add(command),
            onGoToPage: pages.add,
            onNavigateDocument: navigation.add,
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    tester
        .widget<Focus>(find.byKey(const Key('pdf-shell-keyboard')))
        .focusNode!
        .requestFocus();
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowRight);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowLeft);
    expect(pages, [3, 1]);
    await tester.sendKeyEvent(LogicalKeyboardKey.pageDown);
    await tester.sendKeyEvent(LogicalKeyboardKey.space);
    await tester.sendKeyEvent(LogicalKeyboardKey.pageUp);
    await tester.sendKeyEvent(LogicalKeyboardKey.home);
    await tester.sendKeyEvent(LogicalKeyboardKey.end);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowDown);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowUp);
    expect(navigation, [
      PdfNavigationAction.viewportNext,
      PdfNavigationAction.viewportNext,
      PdfNavigationAction.viewportPrevious,
      PdfNavigationAction.documentStart,
      PdfNavigationAction.documentEnd,
      PdfNavigationAction.lineNext,
      PdfNavigationAction.linePrevious,
    ]);
    final thumbnailFocus = tester
        .widget<Focus>(find.byKey(const Key('pdf-thumb-focus-2')))
        .focusNode!;
    await tester.tap(find.byKey(const Key('pdf-thumb-focus-2')));
    await tester.pumpAndSettle();
    expect(thumbnailFocus.hasFocus, isTrue);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowDown);
    expect(pages.last, 3);
    expect(navigation, hasLength(7));
    tester
        .widget<Focus>(find.byKey(const Key('pdf-shell-keyboard')))
        .focusNode!
        .requestFocus();
    await tester.pump();
    await tester.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
    await tester.sendKeyEvent(LogicalKeyboardKey.keyF);
    await tester.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
    await tester.pump();
    expect(find.byKey(const Key('pdf-search-bar')), findsOneWidget);
    await tester.enterText(find.byKey(const Key('pdf-search-input')), 'word');
    await tester.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
    await tester.sendKeyEvent(LogicalKeyboardKey.keyZ);
    await tester.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
    expect(commands, isNot(contains('pdf.edit.undo')));
    expect(tester.takeException(), isNull);
  });

  testWidgets('ribbon state follows PDF session and copy-save lifecycle', (
    tester,
  ) async {
    SharedPreferences.setMockInitialValues({});
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    Future<void> show(PdfDocumentStatus status) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: PdfProductShell(
              definition: definition,
              document: const SizedBox.expand(),
              documentStatus: status,
              pageNumber: 1,
              pageCount: 2,
              fileName: 'fixture.pdf',
              zoom: 1,
              isCommandAvailable: (_) => true,
              onCommand: (_, _) {},
              onGoToPage: (_) {},
              onZoom: (_) {},
              renderThumbnail: (_, _) async => null,
            ),
          ),
        ),
      );
      await tester.pump();
    }

    await show(
      const PdfDocumentStatus(hasDocument: true, hasPendingChanges: true),
    );
    expect(find.text('未保存'), findsOneWidget);
    await show(
      const PdfDocumentStatus(
        hasDocument: true,
        hasPendingChanges: true,
        copySavePhase: PdfCopySavePhase.saving,
      ),
    );
    expect(find.text('正在保存副本…'), findsOneWidget);
    expect(
      tester
          .widget<IconButton>(
            find.ancestor(
              of: find.byTooltip('保存'),
              matching: find.byType(IconButton),
            ),
          )
          .onPressed,
      isNull,
    );
    await show(
      const PdfDocumentStatus(
        hasDocument: true,
        hasPendingChanges: true,
        copySavePhase: PdfCopySavePhase.saved,
      ),
    );
    expect(find.text('副本已保存'), findsOneWidget);
    await show(
      const PdfDocumentStatus(
        hasDocument: true,
        hasPendingChanges: true,
        copySavePhase: PdfCopySavePhase.failed,
        copySaveError: '磁盘不可写',
      ),
    );
    expect(find.text('副本保存失败'), findsOneWidget);
    expect(find.byTooltip('磁盘不可写'), findsOneWidget);
    await show(
      const PdfDocumentStatus(
        hasDocument: true,
        hasPendingChanges: false,
        isReadOnly: true,
      ),
    );
    expect(find.text('加密文档仅支持查看'), findsOneWidget);
    await show(
      const PdfDocumentStatus(hasDocument: true, hasPendingChanges: false),
    );
    expect(find.byKey(const Key('pdf-ribbon-document-state')), findsNothing);
  });

  testWidgets(
    'AI dock follows signed width and stays gated without a provider',
    (tester) async {
      SharedPreferences.setMockInitialValues({});
      tester.view.physicalSize = const Size(1200, 800);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final definition = PdfChromeDefinition.fromJson(
        jsonDecode(File('assets/product/chrome.json').readAsStringSync())
            as Map<String, dynamic>,
      );
      Widget shell() => MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 2,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (_) => false,
            onCommand: (_, _) {},
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      );
      await tester.pumpWidget(shell());
      await tester.pumpAndSettle();
      expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 360);
      expect(find.byKey(const Key('pdf-ai-rail')), findsNothing);
      expect(find.byKey(const Key('pdf-ai-unavailable')), findsOneWidget);
      expect(
        tester
            .widget<OutlinedButton>(find.widgetWithText(OutlinedButton, '总结全文'))
            .onPressed,
        isNull,
      );
      await tester.drag(
        find.byKey(const Key('pdf-ai-resizer')),
        const Offset(160, 0),
      );
      await tester.pumpAndSettle();
      expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 520);
      await tester.runAsync(
        () async => Future<void>.delayed(const Duration(milliseconds: 100)),
      );
      final preferences = await SharedPreferences.getInstance();
      expect(preferences.getDouble('openmuse.pdf.ai.width'), 520);
      expect(preferences.getBool('openmuse.pdf.ai.open'), isNull);
      await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
      await tester.pumpAndSettle();
      expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 34);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpWidget(shell());
      await tester.pumpAndSettle();
      expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 34);
      await tester.tap(find.byKey(const Key('pdf-ai-rail')));
      await tester.pumpAndSettle();
      expect(tester.getSize(find.byKey(const Key('pdf-ai-dock'))).width, 520);
      expect(preferences.getBool('openmuse.pdf.ai.open'), isTrue);
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('AI plugin content survives dock collapse and reopening', (
    tester,
  ) async {
    SharedPreferences.setMockInitialValues({});
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 1,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (_) => false,
            onCommand: (_, _) {},
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
            assistantContent: const TextField(key: Key('ai-plugin-draft')),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const Key('ai-plugin-draft')), 'draft');
    await tester.tap(find.byKey(const Key('pdf-ai-collapse')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('ai-plugin-draft')), findsNothing);
    await tester.tap(find.byKey(const Key('pdf-ai-rail')));
    await tester.pumpAndSettle();
    expect(find.text('draft'), findsOneWidget);
  });

  testWidgets(
    'sidebar follows signed bounds, rerasterizes, and restores width',
    (tester) async {
      SharedPreferences.setMockInitialValues({});
      tester.view.physicalSize = const Size(700, 700);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final definition = PdfChromeDefinition.fromJson(
        jsonDecode(File('assets/product/chrome.json').readAsStringSync())
            as Map<String, dynamic>,
      );
      final rasterWidths = <int>[];
      Widget shell() => MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 2,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (_) => false,
            onCommand: (_, _) {},
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, width) async {
              rasterWidths.add(width);
              return null;
            },
          ),
        ),
      );
      await tester.pumpWidget(shell());
      await tester.pumpAndSettle();
      expect(
        tester.getSize(find.byKey(const Key('pdf-sidebar-pane'))).width,
        150,
      );
      expect(rasterWidths, contains(126));
      await tester.drag(
        find.byKey(const Key('pdf-sidebar-resizer')),
        const Offset(300, 0),
      );
      await tester.pumpAndSettle();
      expect(find.byKey(const Key('pdf-sidebar-pane')), findsNothing);
      await tester.runAsync(
        () async => Future<void>.delayed(const Duration(milliseconds: 100)),
      );
      expect(
        (await SharedPreferences.getInstance()).getDouble(
          'openmuse.pdf.sidebar.width',
        ),
        280,
      );
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpWidget(shell());
      await tester.pumpAndSettle();
      expect(find.byKey(const Key('pdf-sidebar-pane')), findsNothing);
      tester.view.physicalSize = const Size(800, 700);
      await tester.pumpAndSettle();
      expect(
        tester.getSize(find.byKey(const Key('pdf-sidebar-pane'))).width,
        280,
      );
      expect(rasterWidths, contains(256));
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('status bar uses signed GenOffice zoom range and steps', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    double? requestedZoom;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 2,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (command) =>
                command == 'pdf.view.zoomIn' || command == 'pdf.view.zoomOut',
            onCommand: (_, _) {},
            onGoToPage: (_) {},
            onZoom: (zoom) => requestedZoom = zoom,
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    final slider = tester.widget<Slider>(
      find.byKey(const Key('pdf-status-zoom-slider')),
    );
    expect(slider.min, 0.1);
    expect(slider.max, 8);
    expect(
      tester.getSize(find.byKey(const Key('pdf-status-zoom-in'))).width,
      20,
    );
    slider.onChanged!(1.03);
    expect(requestedZoom, 1.05);
    await tester.tap(find.byKey(const Key('pdf-status-zoom-in')));
    expect(requestedZoom, 1.1);
    await tester.tap(find.byKey(const Key('pdf-status-zoom-out')));
    expect(requestedZoom, 0.9);
    expect(tester.getSize(find.byKey(const Key('pdf-status-bar'))).height, 24);
    expect(tester.takeException(), isNull);
  });

  testWidgets('product resource exposes all seven GenOffice PDF tabs', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1800, 1000);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await loadShell(tester);
    for (final tab in [
      'home',
      'annotate',
      'edit',
      'page',
      'view',
      'fillForm',
      'formDesign',
    ]) {
      expect(find.byKey(Key('pdf-tab-$tab')), findsOneWidget);
    }
    expect(find.byKey(const Key('pdf-status-bar')), findsOneWidget);
    expect(find.byType(SvgPicture), findsWidgets);
    expect(tester.getTopLeft(find.byKey(const Key('pdf-status-bar'))).dx, 360);
    expect(find.text('第 1 页，共 0 页'), findsOneWidget);
    expect(find.text('打开一个 PDF 开始阅读'), findsOneWidget);
    await tester.tap(find.byKey(const Key('pdf-tab-page')));
    await tester.pumpAndSettle();
    expect(find.text('抽取页面'), findsOneWidget);
    expect(find.text('合并 PDF'), findsOneWidget);
  });

  testWidgets(
    'conversion menu is reachable and unsupported leaves are disabled',
    (tester) async {
      tester.view.physicalSize = const Size(1365, 860);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await loadShell(tester);
      await tester.tap(find.byKey(const Key('pdf-control-home-convert')));
      await tester.pumpAndSettle();
      expect(find.text('转为 Word'), findsOneWidget);
      expect(find.text('转为 Excel'), findsOneWidget);
      expect(find.text('转为 PPT'), findsOneWidget);
      expect(
        tester.getSize(find.byType(MenuItemButton).first).height,
        closeTo(28, 0.1),
      );
    },
  );

  testWidgets('tablet ribbon keeps all controls accessible without overflow', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(800, 600);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await loadShell(tester);
    expect(find.byKey(const Key('pdf-control-home-convert')), findsOneWidget);
    expect(find.byKey(const Key('pdf-page-input')), findsOneWidget);
    final ribbon = find.byKey(const Key('pdf-ribbon-home'));
    final scrollable = find.descendant(
      of: ribbon,
      matching: find.byType(Scrollable),
    );
    final before = tester
        .state<ScrollableState>(scrollable.first)
        .position
        .pixels;
    await tester.sendEventToBinding(
      PointerScrollEvent(
        position: tester.getCenter(ribbon),
        scrollDelta: const Offset(0, 240),
      ),
    );
    await tester.pump();
    expect(
      tester.state<ScrollableState>(scrollable.first).position.pixels,
      greaterThan(before),
    );
    await tester.tap(find.byKey(const Key('pdf-tab-view')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-page-input')), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('thumbnail context menu dispatches the selected page', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
      iconData:
          jsonDecode(File('assets/product/icons.json').readAsStringSync())
              as Map<String, dynamic>,
    );
    String? issuedCommand;
    Map<String, dynamic>? issuedArgs;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 2,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (command) =>
                command == 'pdf.page.extract' || command == 'pdf.page.delete',
            onCommand: (command, args) {
              issuedCommand = command;
              issuedArgs = args;
            },
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    await tester.tap(
      find.byKey(const Key('pdf-thumb-menu-2')),
      buttons: kSecondaryButton,
    );
    await tester.pumpAndSettle();
    expect(tester.getSize(find.byType(MenuItemButton).first).height, 32);
    await tester.tap(find.text('抽取页面').last);
    await tester.pumpAndSettle();
    expect(issuedCommand, 'pdf.page.extract');
    expect(issuedArgs, {'pageNumber': 2});
    expect(tester.takeException(), isNull);
  });

  testWidgets(
    'GenOffice page and zoom group and full color palette remain reachable',
    (tester) async {
      tester.view.physicalSize = const Size(1800, 1000);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await loadShell(tester);
      expect(find.byKey(const Key('pdf-page-input')), findsOneWidget);
      expect(find.text('共 0 页'), findsOneWidget);
      await tester.tap(find.byKey(const Key('pdf-highlight-colors-home')));
      await tester.pumpAndSettle();
      expect(find.text('主题颜色'), findsOneWidget);
      expect(find.text('标准颜色'), findsOneWidget);
      expect(find.byKey(const Key('pdf-palette-highlight-69')), findsOneWidget);
      await tester.tap(find.byKey(const Key('pdf-palette-highlight-69')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const Key('pdf-tab-view')));
      await tester.pumpAndSettle();
      expect(find.byKey(const Key('pdf-page-input')), findsOneWidget);
      expect(find.text('搜索'), findsOneWidget);
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('ribbon collapses on selected tab and expands on another tab', (
    tester,
  ) async {
    await loadShell(tester);
    expect(find.byKey(const Key('pdf-ribbon-home')), findsOneWidget);
    await tester.tap(find.byKey(const Key('pdf-tab-home')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-ribbon-home')), findsNothing);
    expect(find.byKey(const Key('pdf-status-bar')), findsOneWidget);
    await tester.tap(find.byKey(const Key('pdf-tab-annotate')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-ribbon-annotate')), findsOneWidget);
  });

  testWidgets('drawing color uses the signed palette and custom hex dialog', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1800, 1000);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await loadShell(tester);
    await tester.tap(find.byKey(const Key('pdf-tab-annotate')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('pdf-control-annotate-color')));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-palette-draw-69')), findsOneWidget);
    await tester.tap(find.byKey(const Key('pdf-palette-draw-more')));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const Key('pdf-palette-draw-custom-input')),
      findsOneWidget,
    );
    await tester.enterText(
      find.byKey(const Key('pdf-palette-draw-custom-input')),
      '#123456',
    );
    await tester.tap(find.text('确定'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const Key('pdf-palette-draw-custom-input')),
      findsNothing,
    );
    expect(tester.takeException(), isNull);
  });

  testWidgets('phone chrome keeps tabs and status in the viewport', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await loadShell(tester);
    expect(find.byKey(const Key('pdf-status-bar')), findsOneWidget);
    expect(tester.takeException(), isNull);
    await tester.scrollUntilVisible(
      find.byKey(const Key('pdf-tab-formDesign')),
      250,
      scrollable: find.byType(Scrollable).first,
    );
    await tester.tap(find.byKey(const Key('pdf-tab-formDesign')));
    await tester.pumpAndSettle();
    expect(find.text('文本字段'), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('page Ribbon extract dialog validates and dispatches range', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    Map<String, dynamic>? issuedArgs;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 2,
            pageCount: 5,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (command) => command == 'pdf.page.extract',
            onCommand: (_, args) => issuedArgs = args,
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    await tester.tap(find.byKey(const Key('pdf-tab-page')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
    await tester.pumpAndSettle();
    expect(find.text('抽取页面为 PDF'), findsOneWidget);
    expect(find.text('2'), findsWidgets);
    await tester.enterText(
      find.byKey(const Key('pdf-extract-range-input')),
      '2-6',
    );
    await tester.tap(find.text('确定'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-extract-dialog')), findsOneWidget);
    expect(issuedArgs, isNull);
    await tester.enterText(
      find.byKey(const Key('pdf-extract-range-input')),
      '1-3,5',
    );
    await tester.tap(find.text('确定'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pdf-extract-dialog')), findsNothing);
    expect(issuedArgs, {
      'pageRange': [1, 2, 3, 5],
    });
    expect(tester.takeException(), isNull);
  });

  testWidgets('Android width keeps search controls inside document viewport', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(360, 720);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 1,
            pageCount: 2,
            fileName: 'fixture.pdf',
            zoom: 0.6,
            isCommandAvailable: (command) => command == 'pdf.search.open',
            onCommand: (_, _) {},
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    await tester.scrollUntilVisible(
      find.byKey(const Key('pdf-control-home-search')),
      200,
      scrollable: find
          .descendant(
            of: find.byKey(const Key('pdf-ribbon-home')),
            matching: find.byType(Scrollable),
          )
          .first,
    );
    await tester.tap(find.byKey(const Key('pdf-control-home-search')));
    await tester.pump();
    final rect = tester.getRect(find.byKey(const Key('pdf-search-bar')));
    expect(rect.left, greaterThanOrEqualTo(0));
    expect(rect.right, lessThanOrEqualTo(360));
    expect(find.byKey(const Key('pdf-search-close')), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('split PDF dialog validates pages per output file', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final definition = PdfChromeDefinition.fromJson(
      jsonDecode(File('assets/product/chrome.json').readAsStringSync())
          as Map<String, dynamic>,
    );
    Map<String, dynamic>? issuedArgs;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: PdfProductShell(
            definition: definition,
            document: const SizedBox.expand(),
            pageNumber: 2,
            pageCount: 5,
            fileName: 'fixture.pdf',
            zoom: 1,
            isCommandAvailable: (command) => command == 'pdf.page.split',
            onCommand: (_, args) => issuedArgs = args,
            onGoToPage: (_) {},
            onZoom: (_) {},
            renderThumbnail: (_, _) async => null,
          ),
        ),
      ),
    );
    await tester.tap(find.byKey(const Key('pdf-tab-page')));
    await tester.pump();
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
    expect(find.text('拆分 PDF'), findsWidgets);
    expect(find.byKey(const Key('pdf-split-dialog')), findsOneWidget);
    await tester.enterText(find.byKey(const Key('pdf-split-count-input')), '5');
    await tester.tap(find.text('确定'));
    await tester.pumpAndSettle();
    expect(issuedArgs, isNull);
    expect(find.byKey(const Key('pdf-split-dialog')), findsOneWidget);
    await tester.enterText(find.byKey(const Key('pdf-split-count-input')), '2');
    await tester.tap(find.text('确定'));
    await tester.pumpAndSettle();
    expect(issuedArgs, {'pagesPerFile': 2});
    expect(find.byKey(const Key('pdf-split-dialog')), findsNothing);
    expect(tester.takeException(), isNull);
  });
}
