import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/src/market_discover_view.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  testWidgets('media tiles identify GIF and explain a blocked video URL', (
    tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 220,
            child: MarketMediaView(
              media: MobileMarketMedia(
                type: 'gif',
                uri:
                    'asset:packages/openmuse_market_ui/assets/task-board-editorial.png',
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('GIF'), findsOneWidget);

    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 220,
            child: MarketMediaView(
              media: MobileMarketMedia(
                type: 'video',
                uri: 'http://example.invalid/video.mp4',
              ),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.byTooltip('Play video'));
    await tester.pump();
    expect(find.text('Media unavailable'), findsOneWidget);
  });

  testWidgets(
    'dark Discover keeps its search surface and editorial text legible',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final store = MobilePluginStore(
        Directory('${Directory.systemTemp.path}/market-ui-only'),
      );
      const entry = MobileMarketEntry(
        id: 'com.openmuse.mobile.task-board',
        name: '任务看板',
        version: '0.2.0',
        description: '本地任务演示',
        author: 'OpenMuse',
        platforms: ['android', 'ios'],
        bundle: 'task-board.json',
        sha256:
            'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
        runtime: 'webview',
        categories: ['办公'],
        media: [
          MobileMarketMedia(
            type: 'image',
            uri:
                'asset:packages/openmuse_market_ui/assets/task-board-editorial.png',
          ),
        ],
      );
      await tester.pumpWidget(
        MaterialApp(
          theme: ThemeData.dark(),
          home: MarketDiscoverView(
            entries: const [entry],
            store: store,
            category: '精选',
            mine: false,
            query: '',
            onSection: (_) {},
            onCategory: (_) {},
            onSearch: (_) {},
            onDetail: (_) {},
            onInstall: (_) {},
            onOpen: (_) {},
            onPreview: (_) {},
            desktopSection: const SizedBox.shrink(),
          ),
        ),
      );
      await tester.pumpAndSettle();
      final search = tester.widget<TextField>(
        find.byKey(const ValueKey('mobile-market-search')),
      );
      expect(search.decoration!.filled, isTrue);
      expect(
        search.decoration!.fillColor,
        ThemeData.dark().colorScheme.surface,
      );
      final heading = tester.widget<Text>(
        find.text('Bring useful tools\nto your workbench'),
      );
      expect(heading.style!.color, const Color(0xFF25303E));
      final semantics = tester.ensureSemantics();
      expect(find.bySemanticsLabel(RegExp('Discover plugins')), findsOneWidget);
      expect(find.bySemanticsLabel(RegExp('My plugins')), findsOneWidget);
      semantics.dispose();
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('an Android-only market package cannot install on iOS', (
    tester,
  ) async {
    debugDefaultTargetPlatformOverride = TargetPlatform.iOS;
    addTearDown(() => debugDefaultTargetPlatformOverride = null);
    final store = MobilePluginStore(
      Directory('${Directory.systemTemp.path}/market-platform-only'),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: MarketDiscoverView(
          entries: const [
            MobileMarketEntry(
              id: 'com.openmuse.pptx',
              name: 'PPTX',
              version: '0.1.0',
              description: 'Android only',
              author: 'OpenMuse',
              platforms: ['android'],
              bundle: 'pptx-mobile.json',
              sha256:
                  'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
              categories: ['办公'],
            ),
          ],
          store: store,
          category: '精选',
          mine: false,
          query: '',
          onSection: (_) {},
          onCategory: (_) {},
          onSearch: (_) {},
          onDetail: (_) {},
          onInstall: (_) {},
          onOpen: (_) {},
          onPreview: (_) {},
          desktopSection: const SizedBox.shrink(),
        ),
      ),
    );
    await tester.pumpAndSettle();
    final button = tester.widget<FilledButton>(
      find.byKey(const ValueKey('install-com.openmuse.pptx')),
    );
    expect(button.onPressed, isNull);
    expect(find.text('Unsupported'), findsOneWidget);
    debugDefaultTargetPlatformOverride = null;
  });

  testWidgets('My plugins exposes updates and workbench management', (
    tester,
  ) async {
    var openedWorkbench = false;
    await tester.pumpWidget(
      MaterialApp(
        home: MarketDiscoverView(
          entries: const [],
          store: MobilePluginStore(
            Directory('${Directory.systemTemp.path}/market-mine-only'),
          ),
          category: '全部',
          mine: true,
          query: '',
          onSection: (_) {},
          onCategory: (_) {},
          onSearch: (_) {},
          onDetail: (_) {},
          onInstall: (_) {},
          onOpen: (_) {},
          onPreview: (_) {},
          onManageWorkbench: () => openedWorkbench = true,
          desktopSection: const SizedBox.shrink(),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('market-category-更新')), findsOneWidget);
    await tester.ensureVisible(
      find.byKey(const ValueKey('market-manage-workbench')),
    );
    await tester.tap(find.byKey(const ValueKey('market-manage-workbench')));
    expect(openedWorkbench, isTrue);
  });
}
