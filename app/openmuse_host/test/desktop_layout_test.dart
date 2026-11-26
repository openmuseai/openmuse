import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/desktop_frame.dart';
import 'package:openmuse_host/src/host/desktop_navigation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

void main() {
  test('selection is mutually exclusive and falls back after uninstall', () {
    final n = DesktopNavigationController();
    n.select('ci');
    expect(n.selected, 'ci');
    n.select('ci');
    expect(n.selected, isNull);
    n.toggle();
    expect(n.selected, 'ci');
    n.reconcile({'explorer', 'agent'});
    expect(n.selected, 'explorer');
    n.dispose();
  });
  for (final width in [1440.0, 1024.0, 720.0]) {
    testWidgets('sidebar pushes retained surfaces at $width', (tester) async {
      await tester.binding.setSurfaceSize(Size(width, 900));
      addTearDown(() => tester.binding.setSurfaceSize(null));
      final navigation = DesktopNavigationController();
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
      addTearDown(navigation.dispose);
      addTearDown(layout.dispose);
      int mounts = 0, disposed = 0;
      int sidebarMounts = 0, sidebarDisposals = 0;
      bool workspace = false;
      final text = TextEditingController();
      addTearDown(text.dispose);
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: StatefulBuilder(
              builder: (context, update) => DesktopFrame(
                navigation: navigation,
                items: [
                  OpenMuseNavigationItem(
                    id: 'agent',
                    title: 'Agent',
                    icon: 'agent',
                    sidebar: (_) => const Text('Session catalog'),
                  ),
                  OpenMuseNavigationItem(
                    id: 'ci',
                    title: 'Muse-CI',
                    icon: 'ci',
                    sidebar: (_) => _Probe(
                      onMount: () => sidebarMounts++,
                      onDispose: () => sidebarDisposals++,
                      child: const Text('CI project catalog'),
                    ),
                  ),
                ],
                explorer: const Text('Real Explorer slot'),
                title: 'Workspace · Muse',
                onDrag: () {},
                onMarket: () {},
                onSettings: () {},
                onNotifications: () {},
                onAccount: () {},
                narrowWorkspace: workspace,
                onNarrowWorkspace: (v) => update(() => workspace = v),
                content: WorkbenchCanvas(
                  controller: layout,
                  pinnedPaneId: 'dsh',
                  pinnedPaneWidth: width >= 1120 ? 400 : 360,
                  hiddenPaneIds: {
                    'workspace',
                    if (width < 900) (workspace ? 'dsh' : 'editor'),
                  },
                  retainedInstanceRefs: const {
                    'surface.dsh',
                    'surface.editor.primary',
                  },
                  paneBuilder: (context, id, binding) => id == 'dsh'
                      ? _Probe(
                          onMount: () => mounts++,
                          onDispose: () => disposed++,
                          child: TextField(
                            key: const ValueKey('agent-draft'),
                            controller: text,
                          ),
                        )
                      : const SizedBox(key: ValueKey('right-workspace')),
                ),
              ),
            ),
          ),
        ),
      );
      await tester.pump();
      final sidebarWidth = width >= 1120 ? 256.0 : 232.0;
      final agent = find.byKey(const ValueKey('agent-draft'));
      final before = tester.getTopLeft(agent).dx;
      await tester.enterText(agent, '保持草稿 / preserve draft');
      await tester.tap(find.byKey(const ValueKey('activity-ci')));
      await tester.pump();
      expect(find.text('CI project catalog'), findsOneWidget);
      expect(find.text('Session catalog'), findsNothing);
      expect(tester.getTopLeft(agent).dx, before);
      await tester.tap(find.byKey(const ValueKey('activity-explorer')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('activity-ci')));
      await tester.pump();
      for (var i = 0; i < 5; i++) {
        await tester.tap(find.byKey(const ValueKey('activity-ci')));
        await tester.pump();
        await tester.tap(find.byKey(const ValueKey('activity-ci')));
        await tester.pump();
      }
      expect(
        sidebarMounts,
        1,
        reason: 'Toggling must not initialize plugin UI again',
      );
      expect(sidebarDisposals, 0);
      await tester.tap(find.byKey(const ValueKey('activity-ci')));
      await tester.pump();
      expect(find.byKey(const ValueKey('desktop-sidebar')), findsNothing);
      expect(tester.getTopLeft(agent).dx, before - sidebarWidth);
      if (width >= 900)
        expect(tester.getSize(agent).width, width >= 1120 ? 400 : 360);
      await tester.tap(find.byKey(const ValueKey('activity-explorer')));
      await tester.pump();
      expect(tester.getTopLeft(agent).dx, before);
      if (width < 900) {
        await tester.tap(
          find.byKey(const ValueKey('desktop-segment-workspace')),
        );
        await tester.pump();
        expect(agent, findsNothing);
        await tester.tap(find.byKey(const ValueKey('desktop-segment-agent')));
        await tester.pump();
      }
      expect(text.text, '保持草稿 / preserve draft');
      expect(mounts, 1);
      expect(disposed, 0);
      expect(tester.takeException(), isNull);
    });
  }
  testWidgets('keyboard toggle and compact marketplace action', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1440, 900));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    final navigation = DesktopNavigationController();
    addTearDown(navigation.dispose);
    int market = 0;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DesktopFrame(
            navigation: navigation,
            items: const [],
            explorer: const Text('Explorer contents'),
            content: const Text('Main'),
            onMarket: () => market++,
            onSettings: () {},
            onNotifications: () {},
            onAccount: () {},
            title: 'Muse',
            onDrag: () {},
            onNarrowWorkspace: (_) {},
          ),
        ),
      ),
    );
    await tester.tap(find.byKey(const ValueKey('activity-market')));
    expect(market, 1);
    final marketCenter = tester.getCenter(
      find.byKey(const ValueKey('activity-market')),
    );
    final bellCenter = tester.getCenter(
      find.byKey(const ValueKey('activity-notifications')),
    );
    expect(bellCenter.dy - marketCenter.dy, 38);
    await tester.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
    await tester.sendKeyEvent(LogicalKeyboardKey.keyB);
    await tester.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
    await tester.pump();
    expect(navigation.selected, isNull);
  });
}

class _Probe extends StatefulWidget {
  const _Probe({
    required this.onMount,
    required this.onDispose,
    required this.child,
  });
  final VoidCallback onMount, onDispose;
  final Widget child;
  @override
  State<_Probe> createState() => _ProbeState();
}

class _ProbeState extends State<_Probe> {
  @override
  void initState() {
    super.initState();
    widget.onMount();
  }

  @override
  void dispose() {
    widget.onDispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}
