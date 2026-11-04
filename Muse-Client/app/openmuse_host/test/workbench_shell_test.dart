import 'dart:io';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/desktop_update.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/layout/plugin_panel.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:path/path.dart' as p;
import 'package:pub_semver/pub_semver.dart';

void main() {
  late Directory root;
  late OpenMusePluginRegistry registry;
  late LocalWorkspaceController workspace;

  setUp(() async {
    root = await Directory.systemTemp.createTemp('openmuse-shell-test-');
    File('${root.path}/README.md').writeAsStringSync('# Notes\n');
    File(
      '${root.path}/preview.png',
    ).writeAsBytesSync([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    workspace = LocalWorkspaceController(
      rootPath: root.path,
      resourceInspector: (resource) async => resource,
      initialResources: [
        OpenMuseResource(
          uri: Uri.file('${root.path}/README.md'),
          displayName: 'README.md',
        ),
        OpenMuseResource(
          uri: Uri.file('${root.path}/preview.png'),
          displayName: 'preview.png',
        ),
      ],
    );
    DshNativeOverlay.popupRoutes.value = 0;
  });

  tearDown(() async {
    if (await root.exists()) await root.delete(recursive: true);
  });

  test(
    'restore a closed plugin panel without replacing files or duplicating panes',
    () {
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
      layout.unbind('dsh');
      layout.closeEmptyPane('dsh');
      final editor = layout.snapshot.bindingFor('editor');
      final plugin = _StatefulPanelPlugin();
      final candidate = OpenMusePanelCandidate(
        plugin: plugin,
        panel: plugin.descriptor.panels.single,
      );
      revealPluginPanel(layout, candidate);
      expect(layout.snapshot.bindingFor('editor'), editor);
      final count = layout.snapshot.paneIds.length;
      revealPluginPanel(layout, candidate);
      expect(layout.snapshot.paneIds.length, count);
      expect(
        layout.snapshot.bindings.values.where(
          (b) => b.surfaceRef == 'plugin.panel:test.panel',
        ),
        hasLength(1),
      );
      layout.dispose();
    },
  );

  testWidgets(
    'Home hides the native DSH overlay until the resource route closes',
    (tester) async {
      late NativeResourceCenter center;
      await tester.runAsync(() async {
        center = await NativeResourceCenter.open(
          scope: 'desktop-home-test',
          deviceRef: 'desktop-home-test',
          deviceLabel: 'Desktop',
          registry: registry,
          storageRoot: Directory('${root.path}/resources'),
        );
      });
      final resources = ValueNotifier<NativeResourceCenter?>(center);
      addTearDown(() async {
        resources.dispose();
        await center.close();
      });
      await tester.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          resources: resources,
        ),
      );
      await tester.pumpAndSettle();
      expect(DshNativeOverlay.obscured, isFalse);
      await tester.tap(find.byKey(const ValueKey('desktop-resource-center')));
      await tester.pumpAndSettle();
      expect(find.byType(ResourceCenterView), findsOneWidget);
      expect(DshNativeOverlay.obscured, isTrue);
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();
      expect(DshNativeOverlay.obscured, isFalse);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );

  testWidgets('search dialog filters and opens a local resource', (
    tester,
  ) async {
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );

    await tester.tap(find.text('搜索'));
    await tester.pumpAndSettle();
    expect(find.text('本地资源'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'preview');
    await tester.pumpAndSettle();
    final dialog = find.byType(Dialog);
    expect(
      find.descendant(of: dialog, matching: find.text('README.md')),
      findsNothing,
    );
    expect(
      find.descendant(of: dialog, matching: find.text('preview.png')),
      findsOneWidget,
    );

    await tester.tap(
      find.descendant(of: dialog, matching: find.text('preview.png')),
    );
    await tester.pumpAndSettle();
    expect(workspace.selected?.displayName, 'preview.png');
  });

  testWidgets('running Desktop shows update at the sidebar foot and notes', (
    tester,
  ) async {
    final desktopUpdate = OpenMuseDesktopUpdate();
    addTearDown(desktopUpdate.dispose);
    final updates = OpenMuseUpdateController(
      feed: OpenMuseUpdateFeed(
        origin: Uri.parse('http://127.0.0.1:3080'),
        channel: 'canary',
        target: 'macos-universal',
        allowInsecureLoopback: true,
        fetchOverride: (_) async => jsonEncode({
          'schema': 'openmuse.app-update/v1',
          'channel': 'canary',
          'target': 'macos-universal',
          'version': '1.0.1',
          'build': 2,
          'publishedAt': '2026-10-13T00:00:00Z',
          'downloadUrl':
              'http://127.0.0.1:3080/downloads/updates/v1/canary/OpenMuse-1.0.1-macos-universal.zip',
          'sha256': 'a' * 64,
          'sizeBytes': 123,
          'installKind': 'macos-zip',
          'notes': {'zh': '改进应用更新。', 'en': 'App update improvements.'},
        }),
      ),
      installedVersion: Version.parse('1.0.0'),
      installedBuild: 1,
    );
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        updates: updates,
        desktopUpdate: desktopUpdate,
      ),
    );
    expect(find.byKey(const ValueKey('app-update-footer')), findsNothing);
    await updates.check();
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('app-update-footer')), findsOneWidget);
    expect(find.textContaining('发现新版本'), findsOneWidget);
    expect(find.byKey(const ValueKey('app-update-button')), findsOneWidget);
    expect(
      find.byKey(const ValueKey('notification-unread-dot')),
      findsOneWidget,
    );
    await tester.tap(find.text("What's new ›"));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('app-update-dialog')), findsOneWidget);
    expect(find.text('改进应用更新。'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('app-update-later')));
    desktopUpdate.status.value = OpenMuseDesktopUpdateStatus.downloading;
    desktopUpdate.progress.value = 0.42;
    await tester.pump();
    expect(find.byKey(const ValueKey('app-update-progress')), findsOneWidget);
    expect(find.text('42%'), findsOneWidget);
    expect(
      find.byKey(const ValueKey('app-update-download-dialog')),
      findsNothing,
    );
    desktopUpdate.status.value = OpenMuseDesktopUpdateStatus.ready;
    await tester.pump();
    expect(
      find.byKey(const ValueKey('app-update-restart-button')),
      findsOneWidget,
    );
    desktopUpdate.releaseSha256 = 'b' * 64;
    desktopUpdate.progress.value = 0.43;
    await tester.pump();
    expect(find.byKey(const ValueKey('app-update-button')), findsOneWidget);
    desktopUpdate.releaseSha256 = null;
    desktopUpdate.progress.value = 0.44;
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('notifications-footer')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('notifications-dialog')), findsOneWidget);
    expect(
      find.byKey(const ValueKey('notification-update-action')),
      findsOneWidget,
    );
    expect(find.byKey(const ValueKey('notification-unread-dot')), findsNothing);
    await tester.tap(find.text('关闭'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('app-update-dismiss')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('app-update-footer')), findsNothing);
    expect(find.byKey(const ValueKey('notifications-footer')), findsOneWidget);
  });

  testWidgets('plugin entry opens the full market page and sidebar', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1600, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, _) async {
          if (command == 'plugin.market.list') {
            return [
              {
                'id': 'archify',
                'name': 'Archify',
                'ecosystem': 'dsh',
                'package': '@tt-a1i/archify-dsh',
                'version': '1.0.0',
                'categories': ['编程'],
              },
            ];
          }
          if (command == 'plugin.market.plan') {
            return {
              'id': 'archify',
              'version': '1.0.0',
              'profile': 'web',
              'installedVersion': '1.0.0',
              'enabled': true,
              'action': 'already-installed',
            };
          }
          return null;
        },
      ),
    );
    final market = createOpenMuseBuiltInPlugins().firstWhere(
      (plugin) => plugin.descriptor.id == 'com.openmuse.plugin-market',
    );
    registry.install(market);
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    final editorPane = layout.snapshot.bindings.entries
        .firstWhere(
          (entry) => entry.value.surfaceRef.startsWith('host.editorGroup:'),
        )
        .key;
    final legacyPane = layout.splitPane(editorPane, axis: Axis.horizontal);
    layout.bind(
      legacyPane,
      SurfaceBinding(
        bindingId: 'legacy.market',
        surfaceRef: 'plugin.panel:plugin.market',
        instanceRef: 'legacy.market',
        mobility: SurfaceMobility.snapshotRestore,
      ),
    );
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );
    await tester.tap(find.text('插件').first);
    await tester.pumpAndSettle();
    expect(find.byType(Dialog), findsNothing);
    expect(
      tester.getSize(find.byKey(const ValueKey('desktop-market-page'))).width,
      1600,
    );
    expect(find.byKey(const ValueKey('plugin-market-catalog')), findsOneWidget);
    expect(
      find.byKey(const ValueKey('plugin-market-sidebar-search')),
      findsOneWidget,
    );
    await tester.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
    await tester.sendKeyEvent(LogicalKeyboardKey.keyK);
    await tester.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
    await tester.pump();
    expect(
      tester
          .widget<TextField>(find.byKey(const ValueKey('plugin-market-search')))
          .focusNode
          ?.hasFocus,
      isTrue,
    );
    expect(find.textContaining('Archify 1.0.0'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('plugin-market-nav-办公')));
    await tester.pumpAndSettle();
    expect(find.text('没有匹配的插件'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('plugin-market-nav-编程')));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('plugin-market-card-archify')),
      findsOneWidget,
    );
    expect(
      layout.snapshot.bindings.values.where(
        (binding) => binding.surfaceRef == 'plugin.panel:plugin.market',
      ),
      isEmpty,
    );
    await tester.tap(find.byKey(const ValueKey('plugin-market-tab-1')));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('plugin-market-installed')),
      findsOneWidget,
    );
    expect(find.text('OpenMuse 插件'), findsOneWidget);
    expect(
      tester
          .getBottomLeft(
            find.byKey(const ValueKey('plugin-market-card-archify')),
          )
          .dy,
      lessThan(tester.getTopLeft(find.text('OpenMuse 插件')).dy),
    );
    await tester.tap(
      find.byKey(const ValueKey('plugin-market-sidebar-search')),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('plugin-market-catalog')), findsOneWidget);
    expect(
      tester
          .widget<TextField>(find.byKey(const ValueKey('plugin-market-search')))
          .focusNode
          ?.hasFocus,
      isTrue,
    );
    await tester.tap(find.byTooltip('关闭插件市场'));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('plugin-market-catalog')), findsNothing);
    await tester.tap(find.text('插件').first);
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('plugin-market-catalog')), findsOneWidget);
  });

  testWidgets(
    'installed plugin opens and reuses its panel without replacing files',
    (tester) async {
      tester.view.physicalSize = const Size(1600, 900);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (command, _) async =>
              command == 'plugin.market.list' ? <Map<String, Object?>>[] : null,
        ),
      );
      registry.install(
        createOpenMuseBuiltInPlugins().firstWhere(
          (plugin) => plugin.descriptor.id == 'com.openmuse.plugin-market',
        ),
      );
      final plugin = _StatefulPanelPlugin();
      registry.install(plugin);
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
      final editor = layout.snapshot.bindingFor('editor');
      await tester.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          layoutController: layout,
        ),
      );
      for (var attempt = 0; attempt < 2; attempt++) {
        await tester.tap(find.text('插件').first);
        await tester.pumpAndSettle();
        await tester.tap(find.byKey(const ValueKey('plugin-market-tab-1')));
        await tester.pumpAndSettle();
        final open = find.byKey(
          const ValueKey('plugin-open-test.stateful-panel'),
        );
        await tester.ensureVisible(open);
        await tester.tap(open);
        await tester.pumpAndSettle();
        expect(find.byKey(const ValueKey('desktop-market-page')), findsNothing);
        expect(find.byKey(const Key('surface-probe')), findsOneWidget);
        expect(
          registry.stateOf(plugin.descriptor.id),
          OpenMusePluginState.active,
        );
        expect(layout.snapshot.bindingFor('editor'), editor);
        expect(
          layout.snapshot.bindings.values.where(
            (binding) => binding.surfaceRef == 'plugin.panel:test.panel',
          ),
          hasLength(1),
        );
      }
      expect(plugin.created, 1);
      expect(plugin.disposed, 0);
    },
  );

  testWidgets('market page collapses its sidebar in a narrow window', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(700, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, _) async {
          if (command == 'plugin.market.list') return <Map<String, Object?>>[];
          return null;
        },
      ),
    );
    registry.install(
      createOpenMuseBuiltInPlugins().firstWhere(
        (plugin) => plugin.descriptor.id == 'com.openmuse.plugin-market',
      ),
    );
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    await tester.tap(find.text('插件').first);
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('desktop-market-page')), findsOneWidget);
    expect(find.byKey(const ValueKey('plugin-market-search')), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('authentication gate hides the desktop workbench', (
    tester,
  ) async {
    final authentication = _SignedOutAuthenticationController();
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        authentication: OpenMuseGoTruePlugin(
          authentication: authentication,
          cloudLabel: 'https://cloud.openmuse.test',
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('欢迎使用 OpenMuse'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.email')), findsOneWidget);
    expect(find.text('搜索'), findsNothing);
    expect(find.text('新建文档'), findsNothing);
  });

  testWidgets('new document dialog is local-only', (tester) async {
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    await _showIdeView(tester);

    await tester.tap(find.text('新建文档').first);
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'Local Notes');
    expect(find.text('文件会保存在当前本地工作区。'), findsOneWidget);
    expect(find.textContaining('登录'), findsNothing);
    await tester.tap(find.text('取消'));
    await tester.pumpAndSettle();
  });

  testWidgets('settings contain only local product sections', (tester) async {
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );

    final settingsButton = find.byKey(const ValueKey('title-settings'));
    expect(settingsButton, findsOneWidget);
    await tester.tap(settingsButton);
    await tester.pumpAndSettle();

    expect(find.text('Appearance'), findsOneWidget);
    expect(find.text('工作区'), findsWidgets);
    expect(find.text('插件'), findsWidgets);
    expect(find.text('Agent'), findsOneWidget);
    expect(find.text('账号'), findsNothing);
    expect(find.text('云服务'), findsNothing);
    expect(find.text('协作'), findsNothing);
  });

  testWidgets(
    'CLI plugin docks below the editor and can be disabled',
    (tester) async {
      registry.install(
        createOpenMuseBuiltInPlugins().singleWhere(
          (plugin) => plugin.descriptor.id == 'com.openmuse.cli',
        ),
      );
      final settings = OpenMuseLocalSettings();
      await tester.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          settings: settings,
        ),
      );
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('cli-bottom-panel')), findsNothing);
      if (Platform.isWindows) {
        expect(
          find.byKey(const ValueKey('title-sidebar-toggle')),
          findsOneWidget,
        );
        expect(find.byKey(const ValueKey('title-settings')), findsOneWidget);
        expect(find.byKey(const ValueKey('title-terminal')), findsOneWidget);
        await tester.tap(find.byKey(const ValueKey('title-terminal')));
        await tester.pump();
      }
      expect(find.byKey(const ValueKey('cli-bottom-panel')), findsOneWidget);
      expect(find.byKey(const ValueKey('cli-bottom-resizer')), findsOneWidget);
      final initialHeight = tester
          .getSize(find.byKey(const ValueKey('cli-bottom-panel')))
          .height;
      await tester.drag(
        find.byKey(const ValueKey('cli-bottom-resizer')),
        const Offset(0, -70),
      );
      await tester.pumpAndSettle();
      expect(
        tester.getSize(find.byKey(const ValueKey('cli-bottom-panel'))).height,
        greaterThan(initialHeight),
      );
      await tester.tap(find.byKey(const ValueKey('cli-bottom-collapse')).first);
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('cli-bottom-panel')), findsNothing);
      await tester.tap(find.byKey(const ValueKey('title-terminal')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('cli-bottom-panel')), findsOneWidget);
      await settings.updatePluginValues('com.openmuse.cli', {'enabled': false});
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('cli-bottom-panel')), findsNothing);
    },
    skip: !Platform.isWindows,
  );

  testWidgets('theme selection changes the app immediately', (tester) async {
    final settings = OpenMuseLocalSettings();
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        settings: settings,
      ),
    );
    await tester.tap(find.byKey(const ValueKey('title-settings')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Dark'));
    await tester.pumpAndSettle();
    expect(settings.themeMode, ThemeMode.dark);
    expect(
      tester.widget<MaterialApp>(find.byType(MaterialApp)).themeMode,
      ThemeMode.dark,
    );
  });

  testWidgets('workspace divider changes width through pointer drag', (
    tester,
  ) async {
    final settings = OpenMuseLocalSettings();
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        settings: settings,
        layoutController: layout,
      ),
    );
    final divider = find.byKey(const Key('pane-resizer')).first;
    final start = tester.getCenter(divider);
    final gesture = await tester.startGesture(start);
    await gesture.moveBy(const Offset(42, 0));
    await tester.pump();
    await gesture.up();
    expect((layout.snapshot.root as SplitNode).ratio, greaterThan(0.22));
  });

  testWidgets('Project Workspace section can collapse independently', (
    tester,
  ) async {
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    // The sidebar labels a workspace with its basename; split('/') is a POSIX
    // assumption that yields the whole path on Windows.
    final label = p.basename(root.path);
    expect(find.text(label), findsOneWidget);
    await tester.tap(find.byKey(const Key('project-workspace-toggle')));
    await tester.pump();
    expect(workspace.projectSectionExpanded, isFalse);
    expect(find.text(label), findsNothing);
  });

  testWidgets('swapping adjacent panes keeps the same surface State mounted', (
    tester,
  ) async {
    final plugin = _StatefulPanelPlugin();
    registry.install(plugin);
    final layout = WorkbenchLayoutController(
      WorkbenchLayoutSnapshot(
        root: SplitNode(
          axis: Axis.horizontal,
          ratio: 0.5,
          first: PaneNode(paneId: 'left'),
          second: PaneNode(paneId: 'right'),
        ),
        bindings: {
          'left': SurfaceBinding(
            bindingId: 'panel',
            surfaceRef: 'plugin.panel:test.panel',
            instanceRef: 'surface.panel',
          ),
          'right': SurfaceBinding(
            bindingId: 'workspace',
            surfaceRef: 'host.workspaceExplorer',
            instanceRef: 'surface.workspace',
          ),
        },
      ),
    );
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );
    await tester.pumpAndSettle();
    expect(plugin.created, 1);
    expect(plugin.disposed, 0);
    final before = tester.getTopLeft(find.byKey(const Key('surface-probe')));

    layout.swap('left', 'right');
    await tester.pump();

    expect(plugin.created, 1);
    expect(plugin.disposed, 0);
    final after = tester.getTopLeft(find.byKey(const Key('surface-probe')));
    expect(after.dx, greaterThan(before.dx));
  });

  testWidgets('layout mutations are persisted after the debounce', (
    tester,
  ) async {
    final store = _RecordingLayoutWriter();
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
        layoutStore: store,
      ),
    );

    layout.splitPane('editor', axis: Axis.vertical, newPaneId: 'editor-bottom');
    await tester.pump(const Duration(milliseconds: 400));

    expect(store.snapshots.single.paneIds, contains('editor-bottom'));
  });

  testWidgets('pending layout save is flushed when workbench disposes', (
    tester,
  ) async {
    final store = _RecordingLayoutWriter();
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
        layoutStore: store,
      ),
    );

    layout.splitPane('editor', axis: Axis.vertical, newPaneId: 'dispose-flush');
    await tester.pump();
    await tester.pumpWidget(const SizedBox.shrink());

    expect(store.snapshots.single.paneIds, contains('dispose-flush'));
  });

  testWidgets('pane menu can split vertically and bind a new editor group', (
    tester,
  ) async {
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );
    await _showIdeView(tester);

    await tester.tap(find.byKey(const Key('pane-menu-button:editor')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('向下切分'));
    await tester.pumpAndSettle();
    expect(layout.snapshot.paneIds, contains('pane-1'));
    expect(layout.snapshot.bindingFor('pane-1'), isNull);

    await tester.tap(find.byKey(const Key('pane-menu-button:pane-1')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('绑定新编辑组'));
    await tester.pumpAndSettle();

    expect(
      layout.snapshot.bindingFor('pane-1')?.surfaceRef,
      'host.editorGroup:editor.2',
    );
    expect(
      workspace.editorGroups.map((group) => group.id),
      contains('editor.2'),
    );

    await tester.tap(find.byKey(const Key('pane-menu-button:pane-1')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('重置默认布局'));
    await tester.pumpAndSettle();
    expect(layout.snapshot.paneIds, isNot(contains('pane-1')));
    expect(
      workspace.editorGroups.map((group) => group.id),
      isNot(contains('editor.2')),
    );
  });

  testWidgets('workspace pane is primary and has no pane menu', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 800));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );
    await _showIdeView(tester);

    expect(find.byKey(const Key('pane-menu-button:workspace')), findsNothing);
    expect(find.byKey(const Key('pane-menu-button:editor')), findsOneWidget);
    expect(find.byKey(const Key('pane-menu-button:dsh')), findsOneWidget);

    final menu = tester.getRect(
      find.byKey(const Key('pane-menu-button:editor')),
    );
    final tabLabel = tester.getRect(find.text('Blank page'));
    expect(menu.center.dy, closeTo(tabLabel.center.dy, 12));

    final workspaceBinding = layout.snapshot.bindingFor('workspace');
    final editorBinding = layout.snapshot.bindingFor('editor');
    await tester.tap(find.byKey(const Key('pane-menu-button:editor')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('与左侧交换'));
    await tester.pumpAndSettle();
    expect(layout.snapshot.bindingFor('workspace'), workspaceBinding);
    expect(layout.snapshot.bindingFor('editor'), editorBinding);
  });

  testWidgets('directional swap ignores panes hidden by visibility settings', (
    tester,
  ) async {
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );
    workspace.toggleSidebar();
    await tester.pump();
    await _showIdeView(tester);

    final editorBinding = layout.snapshot.bindingFor('editor');
    final workspaceBinding = layout.snapshot.bindingFor('workspace');
    await tester.tap(find.byKey(const Key('pane-menu-button:editor')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('与左侧交换'));
    await tester.pumpAndSettle();

    expect(layout.snapshot.bindingFor('editor'), editorBinding);
    expect(layout.snapshot.bindingFor('workspace'), workspaceBinding);
  });

  testWidgets('an empty pane can bind a contributed plugin panel', (
    tester,
  ) async {
    await tester.binding.setSurfaceSize(const Size(1400, 800));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    registry.install(_StatefulPanelPlugin());
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    layout.splitPane(
      'editor',
      axis: Axis.horizontal,
      newPaneId: 'plugin-target',
    );
    workspace.openResource(
      OpenMuseResource(
        uri: Uri.file('${root.path}/README.md'),
        displayName: 'README.md',
      ),
    );
    await tester.pumpWidget(
      OpenMuseHostApp(
        registry: registry,
        workspace: workspace,
        layoutController: layout,
      ),
    );

    await tester.tap(find.byKey(const Key('pane-menu-button:plugin-target')));
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('绑定/交换 Stateful Panel'));
    await tester.pumpAndSettle();

    expect(
      layout.snapshot.bindingFor('plugin-target')?.surfaceRef,
      'plugin.panel:test.panel',
    );

    await tester.tap(find.byKey(const Key('pane-menu-button:editor')));
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('绑定/交换 Stateful Panel'));
    await tester.pumpAndSettle();
    expect(
      layout.snapshot.bindingFor('editor')?.surfaceRef,
      'plugin.panel:test.panel',
    );
    expect(
      layout.snapshot.bindingFor('plugin-target')?.surfaceRef,
      'host.editorGroup:editor.primary',
    );
  });

  testWidgets('resource menu groups opening and versions into cascades', (
    tester,
  ) async {
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    await tester.tap(find.text('README.md').first, buttons: kSecondaryButton);
    await tester.pumpAndSettle();
    expect(find.text('打开方式'), findsOneWidget);
    expect(find.text('版本'), findsOneWidget);
    expect(find.textContaining('使用 Helix'), findsNothing);
    await tester.tap(find.text('打开方式'));
    await tester.pumpAndSettle();
    expect(find.text('iOffice'), findsOneWidget);
    expect(find.text('默认打开方式'), findsOneWidget);
  });

  testWidgets('workspace menu offers opening its directory in a terminal', (
    tester,
  ) async {
    registry.install(
      createOpenMuseBuiltInPlugins().singleWhere(
        (plugin) => plugin.descriptor.id == 'com.openmuse.cli',
      ),
    );
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    await tester.tap(
      find.text(workspace.mounts.first.name).first,
      buttons: kSecondaryButton,
    );
    await tester.pumpAndSettle();
    final action = find.byWidgetPredicate(
      (widget) => widget is PopupMenuItem<String> && widget.value == 'terminal',
    );
    expect(action, findsOneWidget);
    expect(tester.widget<PopupMenuItem<String>>(action).enabled, isTrue);
  });
  testWidgets(
    'opening a terminal activates its plugin and reveals an empty Agent editor area',
    (tester) async {
      var activated = false;
      registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (name, _) async {
            if (name == 'workspace.snapshot') {
              activated = true;
              return {'activeMountPath': root.path};
            }
            if (name == 'terminal.session.create') {
              throw StateError('PTY intentionally unavailable in widget test');
            }
            return null;
          },
        ),
      );
      registry.install(
        createOpenMuseBuiltInPlugins().singleWhere(
          (plugin) => plugin.descriptor.id == 'com.openmuse.cli',
        ),
      );
      await tester.pumpWidget(
        OpenMuseHostApp(registry: registry, workspace: workspace),
      );
      expect(find.byKey(const ValueKey('cli-terminal-tab')), findsNothing);
      await tester.tap(
        find.text(workspace.mounts.first.name).first,
        buttons: kSecondaryButton,
      );
      await tester.pumpAndSettle();
      await tester.tap(
        find.byWidgetPredicate(
          (widget) =>
              widget is PopupMenuItem<String> && widget.value == 'terminal',
        ),
      );
      await tester.pumpAndSettle();
      expect(activated, isTrue);
      expect(find.byKey(const ValueKey('cli-terminal-tab')), findsOneWidget);
      expect(
        find.textContaining('PTY intentionally unavailable'),
        findsOneWidget,
      );
      final console = find.byKey(
        const ValueKey('cli-bottom-panel'),
        skipOffstage: false,
      );
      final state = tester.state(console);
      await tester.tap(find.byKey(const ValueKey('cli-bottom-collapse')).first);
      await tester.pumpAndSettle();
      expect(console, findsOneWidget);
      expect(tester.state(console), same(state));
    },
  );
}

Future<void> _showIdeView(WidgetTester tester) async {
  await tester.tap(find.byKey(const ValueKey('title-view')));
  await tester.pumpAndSettle();
  await tester.tap(
    find.ancestor(
      of: find.text('IDE 视图'),
      matching: find.byType(PopupMenuItem<String>),
    ),
  );
  await tester.pumpAndSettle();
}

final class _SignedOutAuthenticationController extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;
  @override
  Future<void> restore() async {}
  @override
  Future<void> signInWithPassword(String email, String password) async {}
  @override
  Future<void> signOut() async {}
}

final class _RecordingLayoutWriter implements LayoutSnapshotWriter {
  final List<WorkbenchLayoutSnapshot> snapshots = [];

  @override
  Future<void> save(WorkbenchLayoutSnapshot snapshot) async {
    snapshots.add(snapshot);
  }
}

final class _StatefulPanelPlugin implements OpenMusePlugin {
  int created = 0;
  int disposed = 0;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'test.stateful-panel',
    name: 'Stateful Panel',
    version: '1.0.0',
    runtime: OpenMusePluginRuntime.builtIn,
    panels: [
      OpenMusePanelContribution(
        id: 'test.panel',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      _SurfaceProbe(onInit: () => created++, onDispose: () => disposed++);
}

final class _SurfaceProbe extends StatefulWidget {
  const _SurfaceProbe({required this.onInit, required this.onDispose});

  final VoidCallback onInit;
  final VoidCallback onDispose;

  @override
  State<_SurfaceProbe> createState() => _SurfaceProbeState();
}

final class _SurfaceProbeState extends State<_SurfaceProbe> {
  @override
  void initState() {
    super.initState();
    widget.onInit();
  }

  @override
  void dispose() {
    widget.onDispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      const ColoredBox(key: Key('surface-probe'), color: Colors.blue);
}
