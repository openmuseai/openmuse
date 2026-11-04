import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import '../../../plugins/siyuan-client/integration_test/resource_flow.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    runApp(const SizedBox.shrink());
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  testWidgets(
    'SiYuan opens inside actual Desktop workspace resource tabs',
    (tester) async {
      final root = await Directory.systemTemp.createTemp(
        'siyuan-workspace-ui-',
      );
      final existing = File('${root.path}/existing.txt')
        ..writeAsStringSync('Existing workspace resource');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      for (final plugin in createOpenMuseBuiltInPlugins()) {
        if (plugin is! OpenMuseResourceCreatorContributor) continue;
        registry.install(plugin);
        await registry.activate(plugin.descriptor.id);
      }
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: [
          OpenMuseResource(uri: existing.uri, displayName: 'existing.txt'),
        ],
      );
      workspace.openResource(OpenMuseResource(uri: existing.uri, displayName: 'existing.txt'));
      final center = await NativeResourceCenter.open(
        scope: 'siyuan-ui-acceptance',
        deviceRef: 'desktop-ui',
        deviceLabel: 'Mac',
        registry: registry,
        storageRoot: Directory('${root.path}/resources'),
      );
      final centers = ValueNotifier<NativeResourceCenter?>(center);
      await tester.pumpWidget(
        OpenMuseHostApp(
          registry: registry,
          workspace: workspace,
          resources: centers,
        ),
      );
      final result = await runSiYuanResourceFlow(tester, desktop: true);
      expect(result['passed'], isTrue);
      expect(workspace.tabs.any((t) => t.resource.uri == existing.uri), isTrue);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
      workspace.dispose();
      centers.dispose();
      await center.close();
      await root.delete(recursive: true);
    },
    timeout: const Timeout(Duration(minutes: 7)),
  );
}
