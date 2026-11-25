import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_test.dart' as fixture;

void main() {
  testWidgets(
    'parent read-only policy narrows one shared presentation without revoking its sibling',
    (t) async {
      late OpenMuseDatasetPlugin plugin;
      late ResourceEditorSession standalone, editor, reader;
      late DatasetResourceLease owner;
      late ResourceEmbedTarget target;
      await t.runAsync(() async {
        plugin = OpenMuseDatasetPlugin(
          core: NativeDatasetCore(),
          storeFactory: (_) async => fixture.Memory(),
        );
        final seed = await plugin.createResource('dataset.database', {});
        standalone = fixture.session(seed.bytes);
        editor = fixture.session(seed.bytes);
        reader = fixture.session(seed.bytes);
        owner = await plugin.acquireResourceSession(standalone);
        target = (await plugin.resourceEmbedTargets(
          standalone.opened,
          'dataset.grid',
        )).single.target;
      });
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Row(
              children: [
                Expanded(
                  child: Builder(
                    key: const ValueKey('editable-parent'),
                    builder: (context) => plugin.buildResourceEmbed(
                      context,
                      editor,
                      target,
                      const ResourceEmbedPolicy(allowEditing: true),
                    ),
                  ),
                ),
                Expanded(
                  child: Builder(
                    key: const ValueKey('readonly-parent'),
                    builder: (context) => plugin.buildResourceEmbed(
                      context,
                      reader,
                      target,
                      const ResourceEmbedPolicy(allowEditing: false),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 100 && find.byType(DatasetSurface).evaluate().length != 2;
        i++
      ) {
        await t.pump(const Duration(milliseconds: 20));
        await t.runAsync(() => Future<void>.delayed(Duration.zero));
      }
      await t.pump();
      final views = t
          .widgetList<DatasetSurface>(find.byType(DatasetSurface))
          .toList();
      expect(views, hasLength(2));
      expect(
        views.every((v) => identical(v.controller, owner.controller)),
        isTrue,
      );
      final readonlyAdd = find.descendant(
        of: find.byKey(const ValueKey('readonly-parent')),
        matching: find.byKey(const ValueKey('dataset.add-row')),
      );
      final editableAdd = find.descendant(
        of: find.byKey(const ValueKey('editable-parent')),
        matching: find.byKey(const ValueKey('dataset.add-row')),
      );
      expect(t.widget<TextButton>(readonlyAdd).onPressed, isNull);
      expect(t.widget<TextButton>(editableAdd).onPressed, isNotNull);
      await t.tap(editableAdd);
      for (var i = 0; i < 100; i++) {
        await t.pump(const Duration(milliseconds: 20));
        await t.runAsync(() => Future<void>.delayed(Duration.zero));
        if (owner.controller.views.first.rows.length == 1 &&
            t.widget<TextButton>(editableAdd).onPressed != null)
          break;
      }
      expect(owner.controller.views.first.rows, hasLength(1));
      expect(owner.controller.canWrite, isTrue);
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      Future<void> domain(Future<void> Function() action) async {
        var done = false;
        Object? error;
        unawaited(
          action().then(
            (_) => done = true,
            onError: (Object e) {
              error = e;
              done = true;
            },
          ),
        );
        for (var i = 0; i < 100 && !done; i++) {
          await t.pump(const Duration(milliseconds: 20));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 5)),
          );
        }
        if (error != null) throw error!;
        expect(done, isTrue, reason: 'both test scheduling zones must advance');
      }

      await domain(owner.close);
      standalone.dispose();
      editor.dispose();
      reader.dispose();
      await domain(plugin.deactivate);
    },
  );
  testWidgets(
    'ended account keeps the same database surface without invoking expired metadata or action ports',
    (t) async {
      late OpenMuseDatasetPlugin plugin;
      late ResourceEditorSession session;
      final lifetime = ResourceSessionLifetime();
      String? expectedView;
      void check() {
        if (!lifetime.active) throw const ResourceFailure('DENIED');
      }

      await t.runAsync(() async {
        plugin = OpenMuseDatasetPlugin(
          core: NativeDatasetCore(),
          storeFactory: (_) async => fixture.Memory(),
          viewIdFor: (_) {
            check();
            return expectedView;
          },
          allowRowCreationFor: (_) {
            check();
            return true;
          },
          headerActions: (_, _) {
            check();
            return const [];
          },
        );
        final seed = await plugin.createResource('dataset.database', {});
        final base = fixture.session(seed.bytes);
        session = ResourceEditorSession(
          opened: base.opened,
          lifetime: lifetime,
          allowEditing: true,
          commitContent: base.commitContent,
          onOpened: (_) {},
          onChanged: (_) {},
        );
        base.dispose();
        final lease = await plugin.acquireResourceSession(session);
        expectedView = lease.controller.views.first.id;
      });
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => plugin.buildResourceEditor(
                context,
                'dataset.database',
                session,
              ),
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 100 && find.byType(DatasetSurface).evaluate().isEmpty;
        i++
      ) {
        await t.pump(const Duration(milliseconds: 20));
        await t.runAsync(() => Future<void>.delayed(Duration.zero));
      }
      final initial = t.state(find.byType(DatasetSurface));
      lifetime.invalidate();
      await t.pump();
      expect(t.takeException(), isNull);
      expect(t.state(find.byType(DatasetSurface)), same(initial));
      final surface = t.widget<DatasetSurface>(find.byType(DatasetSurface));
      expect(surface.viewId, expectedView);
      expect(surface.allowEditing, isFalse);
      expect(surface.allowRowCreation, isFalse);
      expect(surface.controller.canRead, isFalse);
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      await t.runAsync(plugin.deactivate);
      session.dispose();
    },
  );
}
