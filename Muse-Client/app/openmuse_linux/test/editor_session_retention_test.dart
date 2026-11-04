import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_linux/src/host/openmuse_app.dart';
import 'package:openmuse_linux/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets('tab switches retain live editor state and closing disposes it', (
    tester,
  ) async {
    final root = (await tester.runAsync(
      () => Directory.systemTemp.createTemp('editor-retention-'),
    ))!;
    final plugin = _EditorProbe();
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    )..install(plugin);
    final workspace = LocalWorkspaceController(
      rootPath: root.path,
      initialResources: [],
      resourceInspector: (r) async => r,
    );
    final first = OpenMuseResource(
      uri: Uri.file('${root.path}/first.testdoc'),
      displayName: 'first.testdoc',
    );
    final second = OpenMuseResource(
      uri: Uri.file('${root.path}/second.testdoc'),
      displayName: 'second.testdoc',
    );
    addTearDown(() async {
      workspace.dispose();
      registry.dispose();
      await root.delete(recursive: true);
    });
    await tester.pumpWidget(
      OpenMuseHostApp(registry: registry, workspace: workspace),
    );
    workspace.openResource(first);
    await tester.pumpAndSettle();
    workspace.openResource(second);
    await tester.pumpAndSettle();
    workspace.openResource(first);
    await tester.pumpAndSettle();
    expect(plugin.created, 2);
    expect(plugin.disposed, 0);
    workspace.closeTab(workspace.tabs.last);
    await tester.pumpAndSettle();
    expect(plugin.disposed, 1);
    await tester.pumpWidget(const SizedBox.shrink());
    expect(plugin.disposed, 2);
  });
}

final class _EditorProbe implements OpenMusePlugin {
  int created = 0, disposed = 0;
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'test.editor-retention',
    name: 'Probe',
    version: '1.0.0',
    runtime: OpenMusePluginRuntime.builtIn,
    editors: [
      OpenMuseEditorContribution(
        id: 'test.doc',
        extensions: {'testdoc'},
        priority: 100,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      _Probe(onInit: () => created++, onDispose: () => disposed++);
  @override
  Widget? buildPanel(BuildContext context, String id) => null;
}

final class _Probe extends StatefulWidget {
  const _Probe({required this.onInit, required this.onDispose});
  final VoidCallback onInit, onDispose;
  @override
  State<_Probe> createState() => _ProbeState();
}

final class _ProbeState extends State<_Probe> {
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
  Widget build(BuildContext context) => const Text('Editor document');
}
