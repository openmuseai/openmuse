import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

class ChainPlugin
    implements
        OpenMusePlugin,
        OpenMuseResourceHandlerContributor,
        OpenMuseResourceEmbedContributor {
  final records = <ResourceRecord>[];
  final built = <int>[];
  bool cycle = false;
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'test.embed-chain',
    name: 'Chain',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'resource.read'},
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();
  @override
  Widget? buildPanel(BuildContext context, String id) => null;
  @override
  List<ResourceHandlerSpec> get resourceHandlers => const [
    ResourceHandlerSpec(
      id: 'chain',
      title: 'chain',
      mediaTypes: {'text/plain'},
    ),
  ];
  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async => true;
  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handler,
    ResourceEditorSession session,
  ) => const SizedBox.shrink();
  @override
  Set<String> get resourceEmbedSurfaces => const {'test.chain'};
  ResourceEmbedTarget target(int n) => ResourceEmbedTarget(
    reference: records[n].reference(),
    surface: 'test.chain',
    selector: {'index': '$n'},
  );
  @override
  Future<List<({String title, ResourceEmbedTarget target})>>
  resourceEmbedTargets(OpenedResource opened, String surface) async => const [];
  @override
  Widget buildResourceEmbed(
    BuildContext context,
    ResourceEditorSession session,
    ResourceEmbedTarget target,
    ResourceEmbedPolicy policy,
  ) {
    final n = int.parse(target.selector['index']!);
    expect(session.bytes, [n]);
    built.add(n);
    final next = cycle && n == 1 ? 0 : n + 1;
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('chain $n'),
        if (next < records.length)
          ResourceEmbedScope.of(
            context,
          )!.build(context, this.target(next), policy),
      ],
    );
  }
}

Future<void> waitForBoundary(WidgetTester t) async {
  for (var i = 0; i < 300; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 2)),
    );
    if (find
        .byKey(const ValueKey('resource-embed-retry'))
        .evaluate()
        .isNotEmpty) {
      return;
    }
  }
  fail('Expected a bounded ancestry refusal');
}

void main() {
  for (final cycle in [false, true]) {
    testWidgets(
      cycle
          ? 'generic resolver refuses a different-presentation A to B to A before building the repeated scope'
          : 'generic resolver stops a deep acyclic resource chain before another presentation is mounted',
      (t) async {
        late NativeResourceCenter center;
        late OpenMusePluginRegistry registry;
        late ResourceEditorSession parent;
        late Directory root;
        final plugin = ChainPlugin()..cycle = cycle;
        await t.runAsync(() async {
          root = await Directory.systemTemp.createTemp('embed-ancestry-');
          registry = OpenMusePluginRegistry(
            context: OpenMusePluginContext(
              executeHostCommand: (_, _) async => null,
            ),
          );
          registry.install(plugin);
          await registry.activate(plugin.descriptor.id);
          center = await NativeResourceCenter.open(
            scope: 'chain-test',
            deviceRef: 'test',
            deviceLabel: 'test',
            registry: registry,
            storageRoot: root,
          );
          for (var n = 0; n < (cycle ? 2 : 20); n++) {
            plugin.records.add(
              await center.documents.create(
                parentRef: 'root',
                name: '$n.txt',
                mediaType: 'text/plain',
                bytes: [n],
                idempotencyKey: 'chain-$n',
                access: center.hub.access(
                  'fixture',
                  operations: {ResourceOperation.create},
                ),
              ),
            );
          }
          final opened = await center.controller.resolve(
            plugin.records.first.reference(),
          );
          parent = center.controller.createEditorSession(
            opened,
            (await center.controller.handlers(opened.content.record)).single,
          );
        });
        await t.pumpWidget(
          MaterialApp(
            locale: const Locale('en'),
            supportedLocales: const [Locale('en')],
            localizationsDelegates: [
              OpenMuseStringsDelegate(OpenMuseCatalog.builtIn),
            ],
            home: ResourceEmbeddingScope(
              controller: center.controller,
              child: Scaffold(
                body: ResourceEditorSessionScope(
                  session: parent,
                  child: Builder(
                    builder: (context) => ResourceEmbedScope.of(context)!.build(
                      context,
                      plugin.target(1),
                      const ResourceEmbedPolicy(allowEditing: false),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
        await waitForBoundary(t);
        expect(plugin.built, cycle ? [1] : List.generate(15, (n) => n + 1));
        expect(
          find.byKey(const ValueKey('resource-embed-retry')),
          findsOneWidget,
        );
        expect(t.takeException(), isNull);
        await t.pumpWidget(const SizedBox.shrink());
        await t.pump();
        parent.dispose();
        await t.runAsync(() async {
          await center.close();
          await plugin.deactivate();
          registry.dispose();
          await root.delete(recursive: true);
        });
      },
    );
  }
}
