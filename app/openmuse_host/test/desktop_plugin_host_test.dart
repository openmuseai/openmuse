import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';
import 'package:openmuse_host/src/host/layout/plugin_panel.dart';

void main() {
  testWidgets(
    'shared pages stay in the workspace and retain state across file and sidebar switches',
    (t) async {
      await t.binding.setSurfaceSize(const Size(1440, 900));
      addTearDown(() => t.binding.setSurfaceSize(null));
      final root = (await t.runAsync(() async {
        final root = await Directory.systemTemp.createTemp('desktop-owner-');
        await File(
          '${root.path}/README.txt',
        ).writeAsString('Local resource fixture');
        return root;
      }))!;
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, __) async => null,
        ),
      );
      final plugin = _ContextPlugin();
      registry.install(plugin);
      await registry.activate(plugin.descriptor.id);
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
        resourceInspector: (r) async => r,
      );
      await t.pumpWidget(
        OpenMuseHostApp(registry: registry, workspace: workspace),
      );
      await t.pumpAndSettle();
      final agent = find.byKey(const ValueKey('agent-owned-draft'));
      await t.enterText(agent, 'persistent agent draft');
      final before = t.getTopLeft(agent).dx;
      await t.tap(find.text('Open shared folder'));
      await t.pumpAndSettle();
      final shared = find.byKey(const ValueKey('shared-owned-draft'));
      expect(t.getTopLeft(shared).dx, greaterThan(t.getRect(agent).right));
      await t.enterText(shared, 'persistent shared state');
      final sharedState = t.state(find.byKey(const ValueKey('shared-page')));
      await t.tap(find.byKey(const ValueKey('activity-test.context')));
      await t.pumpAndSettle();
      expect(find.text('Session catalog'), findsOneWidget);
      expect(find.text('Open shared folder'), findsNothing);
      await t.tap(find.byKey(const ValueKey('activity-test.context')));
      await t.pumpAndSettle();
      expect(t.getTopLeft(agent).dx, before - 256);
      workspace.openResource(
        OpenMuseResource(
          uri: Uri.file('${root.path}/README.txt'),
          displayName: 'README.txt',
        ),
      );
      await t.pumpAndSettle();
      expect(shared, findsNothing);
      expect(
        t.state(find.byKey(const ValueKey('shared-page'), skipOffstage: false)),
        same(sharedState),
      );
      await t.tap(find.text('Shared folder'));
      await t.pumpAndSettle();
      expect(
        t.widget<TextField>(shared).controller!.text,
        'persistent shared state',
      );
      expect(
        t.widget<TextField>(agent).controller!.text,
        'persistent agent draft',
      );
      expect(plugin.agentMounts, 1);
      expect(plugin.sharedMounts, 1);
      await registry.deactivate(plugin.descriptor.id);
      await t.pumpAndSettle();
      expect(find.text('Shared folder'), findsNothing);
      expect(plugin.sharedDisposals, 1);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox.shrink());
      workspace.dispose();
      plugin.agentDraft.dispose();
      await t.runAsync(() => root.delete(recursive: true));
    },
  );
  testWidgets(
    'a restored already-focused plugin is visible before an idempotent reveal',
    (t) async {
      await t.binding.setSurfaceSize(const Size(1440, 900));
      addTearDown(() => t.binding.setSurfaceSize(null));
      final root = (await t.runAsync(
        () => Directory.systemTemp.createTemp('desktop-restored-plugin-'),
      ))!;
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, __) async => null,
        ),
      );
      final agent = _ContextPlugin(), plugin = _WorkspacePlugin();
      registry.install(agent);
      registry.install(plugin);
      await registry.activate(agent.descriptor.id);
      await registry.activate(plugin.descriptor.id);
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
      final candidate = OpenMusePanelCandidate(
        plugin: plugin,
        panel: plugin.descriptor.panels.single,
      );
      revealPluginPanel(layout, candidate, right: true, besidePaneId: 'dsh');
      final focused = layout.snapshot.focusedPaneId;
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
        resourceInspector: (r) async => r,
      );
      await t.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          layoutController: layout,
        ),
      );
      await t.pumpAndSettle();
      expect(find.text('Retained plugin content'), findsOneWidget);
      final pane = t.getRect(find.text('Retained plugin content'));
      expect(
        pane.left,
        greaterThan(
          t.getRect(find.byKey(const ValueKey('agent-owned-draft'))).right,
        ),
      );
      revealPluginPanel(layout, candidate, right: true, besidePaneId: 'dsh');
      await t.pumpAndSettle();
      expect(layout.snapshot.focusedPaneId, focused);
      expect(find.text('Retained plugin content'), findsOneWidget);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox.shrink());
      workspace.dispose();
      layout.dispose();
      agent.agentDraft.dispose();
      await t.runAsync(() => root.delete(recursive: true));
    },
  );
}

class _WorkspacePlugin implements OpenMusePlugin {
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'test.workspace',
    name: 'Workspace plugin',
    version: '1',
    runtime: OpenMusePluginRuntime.builtIn,
    panels: [
      OpenMusePanelContribution(
        id: 'test.workspace.panel',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext c, OpenMuseResource r) =>
      const SizedBox.shrink();
  @override
  Widget buildPanel(BuildContext c, String panelId) =>
      const Center(child: Text('Retained plugin content'));
}

class _ContextPlugin
    implements
        OpenMusePlugin,
        OpenMuseNavigationContributor,
        OpenMuseExplorerContributor {
  int agentMounts = 0, sharedMounts = 0, sharedDisposals = 0;
  final agentDraft = TextEditingController();
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'test.context',
    name: 'Context',
    version: '1',
    runtime: OpenMusePluginRuntime.builtIn,
    panels: [
      OpenMusePanelContribution(
        id: 'dsh.agent',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext c, OpenMuseResource r) =>
      const SizedBox.shrink();
  @override
  Widget buildPanel(BuildContext c, String panel) => _Probe(
    key: const ValueKey('agent-page'),
    onMount: () => agentMounts++,
    onDispose: () {},
    child: TextField(
      key: const ValueKey('agent-owned-draft'),
      controller: agentDraft,
    ),
  );
  @override
  List<OpenMuseNavigationItem> get navigationItems => [
    OpenMuseNavigationItem(
      id: 'test.context',
      title: 'Sessions',
      icon: 'agent',
      sidebar: (_) => const Text('Session catalog'),
    ),
  ];
  @override
  Widget buildExplorerSection(BuildContext c) => TextButton(
    onPressed: () => OpenMuseWorkspaceRouting.maybeOf(c)!.open(
      OpenMuseWorkspacePage(
        id: 'shared',
        title: 'Shared folder',
        builder: (_) =>
            _SharedPage(key: const ValueKey('shared-page'), owner: this),
      ),
    ),
    child: const Text('Open shared folder'),
  );
}

class _SharedPage extends StatefulWidget {
  const _SharedPage({super.key, required this.owner});
  final _ContextPlugin owner;
  @override
  State<_SharedPage> createState() => _SharedState();
}

class _SharedState extends State<_SharedPage> {
  final draft = TextEditingController();
  @override
  void initState() {
    super.initState();
    widget.owner.sharedMounts++;
  }

  @override
  void dispose() {
    widget.owner.sharedDisposals++;
    draft.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      TextField(key: const ValueKey('shared-owned-draft'), controller: draft);
}

class _Probe extends StatefulWidget {
  const _Probe({
    super.key,
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
