import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_test.dart' as fixture;

void main() {
  for (final layout in ['calendar', 'board']) {
    testWidgets(
      'supported original $layout is admitted before owners and narrows its existing owner presentation',
      (t) async {
        t.view.physicalSize = const Size(1100, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        late OpenMuseDatasetPlugin plugin;
        late ResourceEditorSession standalone, embedded;
        late DatasetResourceLease owner;
        late ResourceEmbedTarget target;
        late String viewId;
        var stores = 0;
        await t.runAsync(() async {
          final core = NativeDatasetCore();
          final source = await DatasetController.open(
            core: core,
            store: fixture.Memory(),
            objectId: layout == 'calendar'
                ? '9163356e-5f30-474e-9cbd-e1db9d380f5a'
                : '3529a9fb-5094-473f-a31f-c68bce96b619',
            syncInterval: null,
          );
          await source.createField(
            'View field',
            layout == 'calendar' ? 'date' : 'singleSelect',
            timezone: layout == 'calendar' ? 'Asia/Shanghai' : null,
            options: layout == 'calendar'
                ? const []
                : const [DatasetOptionDraft(name: 'Todo')],
          );
          final field = source.views.first.columns.last;
          viewId = layout == 'calendar'
              ? await source.createCalendarView('Original calendar', field)
              : await source.createBoardView('Original board', field);
          await source.addRow(cells: {'title': 'One original record'});
          final bytes = OpenMuseDatasetPlugin.exportBundle(source.bundle);
          await source.close();
          plugin = OpenMuseDatasetPlugin(
            core: core,
            storeFactory: (_) async {
              stores++;
              return fixture.Memory();
            },
          );
          standalone = fixture.session(bytes);
          embedded = fixture.session(bytes);
          final targets = await plugin.resourceEmbedTargets(
            embedded.opened,
            'dataset.grid',
          );
          expect(targets.length, 2);
          target = targets
              .singleWhere((e) => e.target.selector['viewId'] == viewId)
              .target;
          await plugin.validateResourceEmbedTarget(embedded.opened, target);
          expect(stores, 0);
          expect(plugin.activeController(standalone.record.identity), isNull);
          owner = await plugin.acquireResourceSession(standalone);
          expect(stores, 1);
        });
        await t.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildResourceEmbed(
                  context,
                  embedded,
                  target,
                  const ResourceEmbedPolicy(allowEditing: false),
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
        final surface = t.widget<DatasetSurface>(find.byType(DatasetSurface));
        expect(surface.controller, same(owner.controller));
        expect(surface.viewId, viewId);
        expect(surface.allowEditing, isFalse);
        expect(owner.controller.canWrite, isTrue);
        expect(stores, 1);
        expect(surface.controller.views.first.allRows.length, 1);
        if (layout == 'calendar')
          expect(
            find.byKey(const ValueKey('dataset.calendar')),
            findsOneWidget,
          );
        await t.pumpWidget(const SizedBox.shrink());
        for (var i = 0; i < 8; i++) {
          await t.pump();
          await t.runAsync(() => Future<void>.delayed(Duration.zero));
        }
        expect(owner.isClosed, isFalse);
        await t.runAsync(() async {
          await owner.close();
          await plugin.deactivate();
        });
        standalone.dispose();
        embedded.dispose();
        expect(t.takeException(), isNull);
      },
    );
  }
}
