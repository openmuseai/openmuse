import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

class SourcePlugin
    implements OpenMusePlugin, OpenMuseResourceSourceContributor {
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'test.source',
    name: 'Source',
    version: '1',
    runtime: OpenMusePluginRuntime.webView,
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox();
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
  @override
  List<ResourceProvider> createResourceSources(ResourceSourceContext context) {
    expect(context.state, isNotNull);
    return [
      TestSource(
        ResourceSource(
          scope: context.scope,
          id: 'plugin-source',
          workspace: context.workspace,
          mount: 'source',
          label: 'Cloud',
          workspaceLabel: 'Workspace',
          mountLabel: 'Cloud',
          deviceLabel: context.deviceLabel,
          local: true,
        ),
      ),
    ];
  }
}

class TestSource implements ResourceProvider {
  TestSource(this.source);
  @override
  final ResourceSource source;
  @override
  bool get available => true;
  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnimplementedError();
}

void main() {
  test(
    'resource sources follow plugin lifecycle within each account',
    () async {
      final root = await Directory.systemTemp.createTemp('plugin-source-');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      registry.install(SourcePlugin());
      final center = await NativeResourceCenter.open(
        scope: 'account',
        deviceRef: 'device',
        deviceLabel: 'Device',
        registry: registry,
        storageRoot: root,
      );
      expect(center.hub.providers.whereType<TestSource>(), isEmpty);
      await registry.activate('test.source');
      final source = center.hub.providers.whereType<TestSource>().single;
      expect(source.source.scope, 'account');
      await registry.deactivate('test.source');
      expect(center.hub.providers.whereType<TestSource>(), isEmpty);
      await center.close();
      await registry.activate(
        'test.source',
      ); // A closed account must not re-register.
      await registry.deactivate('test.source');
      await root.delete(recursive: true);
    },
  );
}
