import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'resource_embed_ancestry_test.dart' as fixture;

class ValidatingPlugin extends fixture.ChainPlugin
    implements OpenMuseResourceEmbedValidator {
  final gate = Completer<void>();
  int validations = 0;
  bool reject = false;
  @override
  Future<void> validateResourceEmbedTarget(
    OpenedResource opened,
    ResourceEmbedTarget target,
  ) async {
    validations++;
    await gate.future;
    if (reject) throw const ResourceFailure('EMBED_RESOURCE_MISMATCH');
  }
}

Future<void> until(WidgetTester t, bool Function() ready) async {
  for (var i = 0; i < 200; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 2)),
    );
    if (ready()) return;
  }
  fail('Embed admission did not settle');
}

void main() {
  for (final mode in [
    'reject',
    'parent-ended',
    'account-ended',
    'plugin-replaced',
    'unmounted',
  ]) {
    testWidgets(
      'generic admission $mode cannot publish the stale child presentation',
      (t) async {
        late Directory root;
        late NativeResourceCenter center;
        late OpenMusePluginRegistry registry;
        late ResourceEditorSession parent;
        final plugin = ValidatingPlugin();
        await t.runAsync(() async {
          root = await Directory.systemTemp.createTemp('embed-admission-');
          registry = OpenMusePluginRegistry(
            context: OpenMusePluginContext(
              executeHostCommand: (_, _) async => null,
            ),
          );
          registry.install(plugin);
          await registry.activate(plugin.descriptor.id);
          center = await NativeResourceCenter.open(
            scope: 'admission',
            deviceRef: 'test',
            deviceLabel: 'test',
            registry: registry,
            storageRoot: root,
          );
          for (var n = 0; n < 2; n++) {
            plugin.records.add(
              await center.documents.create(
                parentRef: 'root',
                name: '$n.txt',
                mediaType: 'text/plain',
                bytes: [n],
                idempotencyKey: 'seed-$n',
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
                body: SizedBox(
                  height: 110,
                  child: ResourceEditorSessionScope(
                    session: parent,
                    child: Builder(
                      builder: (context) =>
                          ResourceEmbedScope.of(context)!.build(
                            context,
                            plugin.target(1),
                            const ResourceEmbedPolicy(allowEditing: false),
                          ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
        await until(t, () => plugin.validations == 1);
        expect(plugin.built, isEmpty);
        if (mode == 'reject') plugin.reject = true;
        if (mode == 'parent-ended') parent.dispose();
        if (mode == 'account-ended') {
          center.controller.editorLifetime.invalidate();
        }
        fixture.ChainPlugin? replacement;
        if (mode == 'plugin-replaced') {
          await t.runAsync(() async {
            await registry.uninstall(plugin.descriptor.id);
            replacement = fixture.ChainPlugin();
            registry.install(replacement!);
            await registry.activate(replacement!.descriptor.id);
          });
        }
        if (mode == 'unmounted') await t.pumpWidget(const SizedBox.shrink());
        plugin.gate.complete();
        if (mode != 'unmounted') {
          await until(
            t,
            () => find
                .byKey(const ValueKey('resource-embed-retry'))
                .evaluate()
                .isNotEmpty,
          );
        } else {
          await t.pump();
        }
        expect(plugin.built, isEmpty);
        expect(replacement?.built ?? [], isEmpty);
        if (mode == 'reject') {
          expect(
            find.text(
              'The reference does not match this resource. Choose it again.',
            ),
            findsOneWidget,
          );
          await t.tap(find.byKey(const ValueKey('resource-embed-retry')));
          await until(
            t,
            () =>
                plugin.validations == 2 &&
                find
                    .byKey(const ValueKey('resource-embed-retry'))
                    .evaluate()
                    .isNotEmpty,
          );
          expect(plugin.built, isEmpty);
          plugin.reject = false;
          await t.tap(find.byKey(const ValueKey('resource-embed-retry')));
          await until(t, () => plugin.built.isNotEmpty);
          expect(plugin.built, [1]);
        }
        await t.pumpWidget(const SizedBox.shrink());
        await t.pump();
        parent.dispose();
        await t.runAsync(() async {
          await center.close();
          registry.dispose();
          await root.delete(recursive: true);
        });
        expect(t.takeException(), isNull);
      },
    );
  }
}
