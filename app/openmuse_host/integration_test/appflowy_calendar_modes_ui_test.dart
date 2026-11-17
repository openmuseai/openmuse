import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original Calendar and private Root/Row objects work across authenticated Host contexts',
    (t) async {
      HttpOverrides.global = null;
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      expect(fixture['origin'], 'http://127.0.0.1:18001');
      final db = fixture['database'] as Map, users = fixture['users'] as Map;
      final wid = fixture['workspaceId'] as String,
          rootId = db['rootId'] as String;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-calendar-modes-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_CALENDAR_MODES_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      for (final name in ['alice', 'outsider']) {
        final auth = GoTrueAuthenticationController(
          provider: GoTrueHttpClient(
            config: GoTrueClientConfig(
              origin: Uri.parse(fixture['authOrigin']),
              allowInsecureLoopback: true,
            ),
          ),
          store: MemoryAuthSessionStore(),
          bootstrapper: AppFlowyAccountBootstrapper(
            cloudOrigin: Uri.parse(fixture['origin']),
            allowInsecureLoopback: true,
          ),
        );
        await t.runAsync(
          () => auth.signInWithPassword(
            users[name]['email'],
            'SmartdocLocal123!',
          ),
        );
        expect(auth.snapshot.isAuthenticated, isTrue);
        late final AppFlowyDocumentWorkspace online;
        final cloud = OpenMuseCloudWorkspacePlugin(
          authentication: auth,
          cloudOrigin: Uri.parse(fixture['origin']),
          deviceId: 'database-ui-$name-${fixture['run']}',
          allowInsecureLoopback: true,
          onOpenDocument: (page) => online.openDocument(page),
          onOpenDatabase: (page) => online.databases.openDatabase(page),
        );
        online = AppFlowyDocumentWorkspace(
          authentication: auth,
          service: cloud.service,
          checkpointRoot: root,
        );
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => null,
          ),
        );
        registry.install(cloud);
        final plugins = createOpenMuseBuiltInPlugins(cloudDocuments: online)
            .where(
              (p) => {
                'io.openmuse.smart-document',
                'io.openmuse.dataset',
              }.contains(p.descriptor.id),
            )
            .toList();
        for (final plugin in plugins) {
          registry.install(plugin);
        }
        await t.runAsync(() async {
          await registry.activate(cloud.descriptor.id);
          for (final plugin in plugins) {
            await registry.activate(plugin.descriptor.id);
          }
        });
        final workspace = LocalWorkspaceController(
          rootPath: root.path,
          initialResources: const [],
        );
        final center = await t.runAsync(
          () => NativeResourceCenter.open(
            scope: auth.snapshot.identity!.subject,
            deviceRef: 'database-ui-$name',
            deviceLabel: 'Mac',
            registry: registry,
            storageRoot: Directory('${root.path}/$name'),
          ),
        );
        online.attach(center!.controller);
        actors.add(
          _Actor(auth, cloud, online, registry, plugins, workspace, center),
        );
      }
      final active = ValueNotifier<int>(0);
      final checks = <String>[];
      Future<void> until(bool Function() ready, String reason) async {
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (ready()) return;
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
        }
        fail(
          '$reason: ${t.widgetList<Text>(find.byType(Text)).map((v) => v.data).join(' | ')}',
        );
      }

      Future<T> domain<T>(Future<T> Function() action) async {
        var done = false;
        T? value;
        Object? error;
        StackTrace? stack;
        await t.runAsync(() async {
          action().then(
            (v) {
              value = v;
              done = true;
            },
            onError: (Object e, StackTrace s) {
              error = e;
              stack = s;
              done = true;
            },
          );
        });
        await until(() => done, 'domain settled');
        if (error != null) Error.throwWithStackTrace(error!, stack!);
        return value as T;
      }

      Future<void> tap(String key) async {
        final finder = find.byKey(ValueKey(key));
        await until(() => finder.evaluate().isNotEmpty, key);
        if (key.startsWith('dataset.view-option.')) {
          final menuText = find
              .descendant(of: finder, matching: find.byType(Text))
              .hitTestable();
          await until(
            () => menuText.evaluate().length == 1,
            '$key menu text receives pointer',
          );
          await t.tap(menuText);
          await t.pump(const Duration(milliseconds: 350));
          return;
        }
        if (key.startsWith('cloud-access.person-level.')) {
          // The short popup already lays out all four choices. Forcing
          // ensureVisible interferes with DropdownRoute's selected-item offset.
          await t.pumpAndSettle();
          final boundary = t.renderObject<RenderRepaintBoundary>(
            find.byKey(ValueKey('database-host-${active.value}')),
          );
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '${evidence.path}/permission-menu-${key.split('.').last}.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        } else {
          await t.ensureVisible(finder);
        }
        final cell = t.widget(finder);
        final target =
            key.startsWith('cloud-access.person-level.') ||
                cell is DropdownMenuItem ||
                (cell is InkWell && cell.onTap == null)
            ? find.descendant(of: finder, matching: find.byType(Text)).first
            : finder;
        await until(
          () => target.hitTestable().evaluate().length == 1,
          '$key receives pointer events',
        );
        await t.tap(target);
        await t.pump(const Duration(milliseconds: 350));
      }

      Future<void> showActor(int index) async {
        FocusManager.instance.primaryFocus?.unfocus();
        active.value = index;
        await t.pump(const Duration(milliseconds: 400));
      }

      Future<void> catalog({bool refresh = false}) async {
        await tap('cloud-workspace.open');
        if (refresh) await tap('cloud-workspace.refresh');
        await tap('cloud-workspace.$wid');
      }

      Future<void> idle() => until(
        () =>
            find.byKey(const ValueKey('cloud-access.busy')).evaluate().isEmpty,
        'sharing settled',
      );
      Future<void> shareParent({int? level, bool revoke = false}) async {
        await catalog();
        await tap('cloud-page.share.${db['parentViewId']}');
        await idle();
        if (revoke) {
          await tap('cloud-access.revoke.${users['outsider']['email']}');
        } else if (level == 10 &&
            find
                .byKey(
                  ValueKey(
                    'cloud-access.person-permission.${users['outsider']['email']}',
                  ),
                )
                .evaluate()
                .isEmpty) {
          await t.enterText(
            find.byKey(const ValueKey('cloud-access.email')),
            users['outsider']['email'],
          );
          await tap('cloud-access.submit');
        } else {
          await tap(
            'cloud-access.person-permission.${users['outsider']['email']}',
          );
          await tap(
            'cloud-access.person-level.${users['outsider']['email']}.$level',
          );
        }
        await idle();
        await tap('cloud-access.close');
        await until(
          () => find.byType(BackButton).evaluate().length == 1,
          'one current route back button',
        );
        await t.tap(find.byType(BackButton));
        await t.pump(const Duration(milliseconds: 350));
        await until(
          () => find.byType(BackButton).evaluate().length == 1,
          'one current route back button',
        );
        await t.tap(find.byType(BackButton));
        await t.pump(const Duration(milliseconds: 350));
      }

      Future<DatasetController> openDatabase({bool refresh = false}) async {
        await catalog(refresh: refresh);
        await tap('cloud-page.${db['viewId']}');
        await until(
          () => find.byType(DatasetSurface).evaluate().isNotEmpty,
          'actual Host database',
        );
        final surface = t.widget<DatasetSurface>(find.byType(DatasetSurface));
        expect(surface.viewId, db['viewId']);
        expect(surface.controller.objectId, rootId);
        return surface.controller;
      }

      Future<void> edit(String row, String field, String text) async {
        await tap('dataset.cell.$row.$field');
        await until(
          () => find
              .byKey(const ValueKey('dataset.cell-input'))
              .evaluate()
              .isNotEmpty,
          'cell dialog',
        );
        await t.enterText(
          find.byKey(const ValueKey('dataset.cell-input')),
          text,
        );
        await tap('dataset.cell-apply');
      }

      Future<void> shot(String name) async {
        await t.pump(const Duration(milliseconds: 250));
        final surface = t.renderObject<RenderRepaintBoundary>(
          find.byKey(ValueKey('database-host-${active.value}')),
        );
        final image = await surface.toImage(pixelRatio: 1);
        final data = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(data!.buffer.asUint8List());
        image.dispose();
      }

      await t.pumpWidget(
        ValueListenableBuilder<int>(
          valueListenable: active,
          builder: (_, index, _) => Stack(
            alignment: Alignment.topLeft,
            children: [
              for (var i = 0; i < actors.length; i++)
                Offstage(
                  offstage: i != index,
                  child: RepaintBoundary(
                    key: ValueKey('database-host-$i'),
                    child: OpenMuseHostApp(
                      registry: actors[i].registry,
                      workspace: actors[i].workspace,
                      resources: actors[i].centers,
                    ),
                  ),
                ),
            ],
          ),
        ),
      );
      await shareParent(level: 10);
      checks.add(
        'original-parent-page-share-ui-grants-inherited-database-read',
      );
      final owner = await openDatabase();
      await until(() => owner.canWrite, 'owner write');
      final initialRows = owner.views
          .singleWhere((v) => v.id == db['viewId'])
          .rows
          .map((r) => r.id)
          .toSet();
      await tap('dataset.add-row');
      await until(
        () =>
            owner.views.singleWhere((v) => v.id == db['viewId']).rows.length ==
            initialRows.length + 1,
        'owner native row creation',
      );
      var view = owner.views.singleWhere((v) => v.id == db['viewId']);
      final rowId = view.rows
              .singleWhere((r) => !initialRows.contains(r.id))
              .id,
          fieldId = view.columns.first.id;
      await edit(rowId, fieldId, 'Owner native database');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            'Owner native database',
        'owner cell edit',
      );
      await tap('dataset.undo');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value !=
            'Owner native database',
        'row domain undo',
      );
      await tap('dataset.redo');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            'Owner native database',
        'row domain redo',
      );
      await domain(owner.synchronize);
      checks.add(
        'production-cloud-root-view-row-create-cell-dialog-and-own-undo-redo',
      );
      await shot('01-owner-original-cloud-database');
      await showActor(1);
      final guest = await openDatabase(refresh: true);
      await until(
        () =>
            guest.canRead &&
            !guest.canWrite &&
            guest.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .any((r) => r.id == rowId),
        'guest inherits original row',
      );
      expect(
        t
            .widget<TextButton>(find.byKey(const ValueKey('dataset.add-row')))
            .onPressed,
        isNull,
      );
      await tap('dataset.cell.$rowId.$fieldId');
      expect(find.byKey(const ValueKey('dataset.cell-input')), findsNothing);
      expect(actors[0].center.hub.scope, isNot(actors[1].center.hub.scope));
      checks.add(
        'independent-authenticated-host-context-reads-same-root-and-cannot-edit-viewer-cell',
      );
      await shot('02-guest-inherited-readonly-grid');
      await showActor(0);
      await shareParent(level: 30);
      await showActor(1);
      await tap('dataset.sync');
      await until(() => guest.canWrite, 'fresh inherited writer capabilities');
      await edit(rowId, fieldId, 'Guest native inherited edit');
      await until(
        () =>
            guest.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            'Guest native inherited edit',
        'guest cell UI edit',
      );
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            'Guest native inherited edit',
        'other Host receives live row',
      );
      checks.add(
        'parent-share-upgrade-enables-guest-cell-ui-and-other-host-live-projection',
      );
      await shot('03-guest-native-cell-edit');
      await showActor(0);
      await shot('04-owner-sees-guest-edit');
      await showActor(1);
      final beforeCreation = guest.views
          .singleWhere((v) => v.id == db['viewId'])
          .rows
          .map((r) => r.id)
          .toSet();
      await tap('dataset.add-row');
      await until(
        () =>
            !guest.rowCreationPending &&
            guest.views.singleWhere((v) => v.id == db['viewId']).rows.length ==
                beforeCreation.length + 1,
        'Guest original server row creation',
      );
      final guestCreated = guest.views
          .singleWhere((v) => v.id == db['viewId'])
          .rows
          .singleWhere((r) => !beforeCreation.contains(r.id))
          .id;
      expect(guest.creationNeedsReview, false);
      await showActor(0);
      await until(
        () => owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any((r) => r.id == guestCreated && r.loaded),
        'owner receives Guest-created registered row',
      );
      await edit(guestCreated, fieldId, 'Owner edit in Guest-created row');
      await until(
        () =>
            guest.views
                .singleWhere((v) => v.id == db['viewId'])
                .rows
                .singleWhere((r) => r.id == guestCreated)
                .cells[fieldId]!
                .value ==
            'Owner edit in Guest-created row',
        'Guest receives peer cell edit before own creation undo',
      );
      await showActor(1);
      await tap('dataset.undo');
      await until(
        () => !guest.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any((r) => r.id == guestCreated),
        'Guest undoes only own creation',
      );
      await until(
        () => !owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any((r) => r.id == guestCreated),
        'other Host receives original creation undo',
      );
      await tap('dataset.redo');
      await until(
        () => guest.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any(
              (r) =>
                  r.id == guestCreated &&
                  r.cells[fieldId]!.value == 'Owner edit in Guest-created row',
            ),
        'Guest redoes same original row and preserves peer content',
      );
      await until(
        () => owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any(
              (r) =>
                  r.id == guestCreated &&
                  r.cells[fieldId]!.value == 'Owner edit in Guest-created row',
            ),
        'other Host receives same-row redo',
      );
      checks.add(
        'guest-native-original-server-row-create-own-undo-redo-preserves-owner-peer-edit',
      );
      await until(
        () =>
            guest.canWrite &&
            t
                    .widget<TextButton>(
                      find.byKey(const ValueKey('dataset.add-row')),
                    )
                    .onPressed !=
                null,
        'Guest editor usable again after restored Root membership',
      );
      await shot('07-guest-created-original-row-after-redo');
      await showActor(0);
      await domain(
        () => owner.createField(
          '协作状态',
          'singleSelect',
          options: const [
            DatasetOptionDraft(name: '待处理', color: 1),
            DatasetOptionDraft(name: '进行中', color: 2),
          ],
        ),
      );
      final boardStatus = owner.views.first.columns.singleWhere(
        (f) => f.name == '协作状态',
      );
      final boardId = await domain(
        () => owner.createBoardView('协作看板', boardStatus),
      );
      await domain(
        () => owner.configureViewRules(
          owner.views.singleWhere((v) => v.id == boardId),
          filters: [
            {
              'id': 'host-card-prefill',
              'filter_type': 2,
              'field_id': fieldId,
              'ty': 0,
              'condition': 0,
              'content': '看板预填记录',
            },
          ],
          sorts: [],
        ),
      );
      await domain(owner.synchronize);
      await showActor(1);
      await until(
        () => guest.views.any((v) => v.id == boardId) && guest.canWrite,
        'Guest receives original shared Board view',
      );
      await tap('dataset.view-picker');
      await tap('dataset.view-option.$boardId');
      final groupId = boardStatus.options.last.id;
      final beforeCards = guest.views
          .singleWhere((v) => v.id == boardId)
          .allRows
          .map((r) => r.id)
          .toSet();
      await tap('dataset.group-add.$groupId');
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-dialog'))
            .evaluate()
            .isNotEmpty,
        'original new card record panel',
      );
      expect(
        find.descendant(
          of: find.byKey(const ValueKey('dataset.record-dialog')),
          matching: find.text('看板预填记录'),
        ),
        findsOneWidget,
      );
      final boardCard = guest.views
          .singleWhere((v) => v.id == boardId)
          .allRows
          .singleWhere((r) => !beforeCards.contains(r.id))
          .id;
      expect(
        guest.views
            .singleWhere((v) => v.id == boardId)
            .board!
            .groups
            .singleWhere((g) => g.id == groupId)
            .rowIds,
        contains(boardCard),
      );
      await tap('dataset.record-close');
      checks.add(
        'guest-original-server-board-card-group-and-filter-prefill-in-actual-host',
      );
      await showActor(0);
      await until(
        () => owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any((r) => r.id == boardCard && r.loaded),
        'Owner Grid receives Guest-created Board record',
      );
      final checkboxId = owner.views.first.columns
          .firstWhere((f) => f.semanticType == 'checkbox')
          .id;
      await tap('dataset.cell.$boardCard.$checkboxId');
      await until(
        () =>
            guest.views
                .singleWhere((v) => v.id == boardId)
                .rows
                .singleWhere((r) => r.id == boardCard)
                .cells[checkboxId]!
                .value ==
            true,
        'Guest card receives peer checkbox',
      );
      await showActor(1);
      await tap('dataset.undo');
      await until(
        () => !guest.views
            .singleWhere((v) => v.id == boardId)
            .allRows
            .any((r) => r.id == boardCard),
        'Guest undoes only Board insertion',
      );
      await until(
        () => !owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .allRows
            .any((r) => r.id == boardCard),
        'Owner receives Board insertion undo',
      );
      await tap('dataset.redo');
      await until(
        () =>
            guest.canWrite &&
            guest.views
                .singleWhere((v) => v.id == boardId)
                .rows
                .any(
                  (r) =>
                      r.id == boardCard && r.cells[checkboxId]!.value == true,
                ),
        'Board redo retains peer content and restored authority',
      );
      await until(
        () => owner.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any(
              (r) => r.id == boardCard && r.cells[checkboxId]!.value == true,
            ),
        'same Board record and peer cell restored in Owner Grid',
      );
      checks.add(
        'guest-board-create-own-undo-redo-preserves-peer-checkbox-and-shared-grid-row',
      );
      await shot('08-guest-board-card-after-own-redo');
      await tap('dataset.view-picker');
      await tap('dataset.view-option.${db['viewId']}');
      await showActor(0);
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(owner),
      );
      // Exercise the new view through the formal original database UI. Root
      // and Row IDs were created by the same original service above.
      await tap('dataset.add-field');
      await t.tap(find.text('日期').last);
      await until(
        () => find
            .byKey(const ValueKey('dataset.field-name'))
            .evaluate()
            .isNotEmpty,
        'original date field dialog',
      );
      await t.enterText(
        find.byKey(const ValueKey('dataset.field-name')),
        '日历日期',
      );
      await t.enterText(
        find.byKey(const ValueKey('dataset.field-timezone')),
        'Asia/Shanghai',
      );
      await tap('dataset.field-apply');
      await until(
        () => owner.views.first.columns.any((f) => f.name == '日历日期'),
        'original date schema accepted',
      );
      final dateFieldId = owner.views.first.columns
          .singleWhere((f) => f.name == '日历日期')
          .id;
      final rowIdsBeforeCalendar = owner.views.first.allRows
          .map((r) => r.id)
          .toSet();
      await tap('dataset.add-view');
      await t.enterText(
        find.byKey(const ValueKey('dataset.view-name')),
        '共享月历',
      );
      await t.tap(find.byKey(const ValueKey('dataset.view-layout')));
      await t.pump(const Duration(milliseconds: 250));
      await t.tap(find.text('日历').last);
      await tap('dataset.view-apply');
      await until(
        () => find
            .byKey(const ValueKey('dataset.calendar'))
            .evaluate()
            .isNotEmpty,
        'formal original calendar view',
      );
      final calendarId = owner.views.singleWhere((v) => v.name == '共享月历').id;
      DatasetCalendar calendarOf(DatasetController c) =>
          c.views.singleWhere((v) => v.id == calendarId).calendar!;
      expect(calendarOf(owner).ready, isTrue);
      expect(calendarOf(owner).timezone, 'Asia/Shanghai');
      expect(
        owner.views
            .singleWhere((v) => v.id == calendarId)
            .allRows
            .map((r) => r.id)
            .toSet(),
        rowIdsBeforeCalendar,
      );
      checks.add(
        'formal-original-calendar-creation-shares-root-schema-and-original-row-membership-without-event-copies',
      );

      Future<void> openEvent(String row) async {
        final event = find
            .byWidgetPredicate(
              (w) =>
                  w is InkWell &&
                  w.key.toString().contains('dataset.calendar-event.$row.'),
            )
            .first;
        await until(
          () => event.evaluate().isNotEmpty,
          'calendar event for original row',
        );
        await t.ensureVisible(event);
        await t.pump(const Duration(milliseconds: 200));
        await until(
          () => event.hitTestable().evaluate().length == 1,
          'calendar event receives pointer',
        );
        await t.tap(event);
        await until(
          () => find
              .byKey(const ValueKey('dataset.record-dialog'))
              .evaluate()
              .isNotEmpty,
          'original record details',
        );
      }

      Future<void> dateInDetails(String text) async {
        await tap('dataset.record-field.$dateFieldId');
        await until(
          () => find
              .byKey(const ValueKey('dataset.date-start'))
              .evaluate()
              .isNotEmpty,
          'original date editor',
        );
        await t.enterText(
          find.byKey(const ValueKey('dataset.date-start')),
          text,
        );
        await tap('dataset.date-apply');
        await until(
          () => find
              .byKey(const ValueKey('dataset.date-dialog'))
              .evaluate()
              .isEmpty,
          'date dialog completed',
        );
        final visibleDate = t.widget<DatasetSurface>(
          find.byType(DatasetSurface),
        );
        expect(active.value, 0);
        expect(visibleDate.controller, same(owner));
        debugPrint('CALENDAR_DATE_DIAGNOSTIC');
        debugPrint(
          jsonEncode({
            'actor': active.value,
            'sameOwner': identical(visibleDate.controller, owner),
            'historyObjectId':
                visibleDate.controller.snapshot['historyObjectId'],
            'canUndo': visibleDate.controller.canUndo,
            'expectedDate': text,
            'date': calendarOf(visibleDate.controller).events
                .where((e) => e.rowId == rowId)
                .map((e) => e.startDay)
                .toList(),
          }),
        );
      }

      final now = DateTime.now(),
          nextDay = DateTime(
            DateTime.now().year,
            DateTime.now().month,
            DateTime.now().day + 1,
          );
      String civil(DateTime d) =>
          '${d.year.toString().padLeft(4, '0')}-${d.month.toString().padLeft(2, '0')}-${d.day.toString().padLeft(2, '0')}';
      final todayText = civil(now), nextDayText = civil(nextDay);
      await tap('dataset.calendar-unplaced');
      await tap('dataset.calendar-unplaced.$rowId');
      await dateInDetails(todayText);
      await tap('dataset.record-close');
      await until(
        () => calendarOf(
          owner,
        ).events.any((e) => e.rowId == rowId && e.startDay == todayText),
        'original Row date projects into calendar',
      );
      expect(
        owner.views.first.allRows.map((r) => r.id).toSet(),
        rowIdsBeforeCalendar,
      );
      checks.add(
        'calendar-unplaced-entry-edits-the-original-date-cell-with-field-zone-and-does-not-create-an-event-object',
      );
      final epoch = owner.localMutationEpoch;
      final month = t
          .widget<Text>(find.byKey(const ValueKey('dataset.calendar-month')))
          .data;
      await tap('dataset.calendar-next');
      await until(
        () =>
            t
                .widget<Text>(
                  find.byKey(const ValueKey('dataset.calendar-month')),
                )
                .data !=
            month,
        'local month navigation',
      );
      expect(owner.localMutationEpoch, epoch);
      await tap('dataset.calendar-today');
      checks.add(
        'calendar-month-navigation-is-presentation-local-and-does-not-mutate-root-or-row',
      );
      await shot('09-original-calendar-month-and-field-zone');

      await openEvent(rowId);
      await dateInDetails(nextDayText);
      await tap('dataset.record-close');
      await domain(owner.synchronize);
      await until(
        () =>
            guest.views.any((v) => v.id == calendarId) &&
            calendarOf(
              guest,
            ).events.any((e) => e.rowId == rowId && e.startDay == nextDayText),
        'peer receives original calendar and date',
      );
      await showActor(1);
      await tap('dataset.view-picker');
      await tap('dataset.view-option.$calendarId');
      await openEvent(rowId);
      await tap('dataset.record-field.$fieldId');
      await until(
        () => find
            .byKey(const ValueKey('dataset.cell-input'))
            .evaluate()
            .isNotEmpty,
        'original title editor',
      );
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '日历同伴标题',
      );
      await tap('dataset.cell-apply');
      await tap('dataset.record-close');
      await domain(guest.synchronize);
      await until(
        () => calendarOf(
          owner,
        ).events.any((e) => e.rowId == rowId && e.title == '日历同伴标题'),
        'peer title projects to owner calendar',
      );
      await showActor(0);
      final visibleBeforeCalendarUndo = t.widget<DatasetSurface>(
        find.byType(DatasetSurface),
      );
      debugPrint('CALENDAR_UNDO_DIAGNOSTIC');
      debugPrint(
        jsonEncode({
          'sameOwner': identical(visibleBeforeCalendarUndo.controller, owner),
          'sameGuest': identical(visibleBeforeCalendarUndo.controller, guest),
          'ownerDisposed': owner.isDisposed,
          'historyObjectId': owner.snapshot['historyObjectId'],
          'canUndo': owner.canUndo,
          'selectedView': t
              .widget<DropdownButton<String>>(
                find.byKey(const ValueKey('dataset.view-picker')),
              )
              .value,
          'expectedRow': rowId,
          'date': calendarOf(owner).events
              .where((e) => e.rowId == rowId)
              .map((e) => e.startDay)
              .toList(),
        }),
      );
      expect(visibleBeforeCalendarUndo.controller, same(owner));
      await tap('dataset.undo');
      debugPrint('CALENDAR_UNDO_RESULT');
      debugPrint(
        jsonEncode({
          'historyObjectId': owner.snapshot['historyObjectId'],
          'historyApplied': owner.lastHistoryApplied,
          'canUndo': owner.canUndo,
          'canRedo': owner.canRedo,
          'date': calendarOf(owner).events
              .where((e) => e.rowId == rowId)
              .map((e) => e.startDay)
              .toList(),
        }),
      );
      await until(
        () => calendarOf(owner).events.any(
          (e) =>
              e.rowId == rowId &&
              e.startDay == todayText &&
              e.title == '日历同伴标题',
        ),
        'own date undo preserves peer title',
      );
      await tap('dataset.redo');
      await until(
        () => calendarOf(owner).events.any(
          (e) =>
              e.rowId == rowId &&
              e.startDay == nextDayText &&
              e.title == '日历同伴标题',
        ),
        'own date redo preserves peer title',
      );
      checks.add(
        'calendar-peer-title-and-own-original-date-undo-redo-preserve-each-other-through-original-cloud',
      );

      await tap('dataset.calendar-settings');
      await showActor(1);
      await tap('dataset.calendar-settings');
      await t.tap(find.byKey(const ValueKey('dataset.calendar-week-start')));
      await t.pump(const Duration(milliseconds: 250));
      await t.tap(find.text('周日').last);
      await tap('dataset.view-apply');
      await until(
        () => calendarOf(guest).firstDayOfWeek == 0,
        'peer calendar setting accepted',
      );
      await domain(guest.synchronize);
      await until(
        () => calendarOf(owner).firstDayOfWeek == 0,
        'original Root projects peer calendar settings',
      );
      await showActor(0);
      await tap('dataset.view-apply');
      await until(
        () => find
            .byKey(const ValueKey('dataset.calendar-error'))
            .evaluate()
            .isNotEmpty,
        'stale calendar settings retained',
      );
      expect(calendarOf(owner).firstDayOfWeek, 0);
      await shot('10-stale-calendar-settings-retain-peer-root-configuration');
      await t.tap(find.text('取消').last);
      await t.pump(const Duration(milliseconds: 350));
      checks.add(
        'stale-calendar-settings-reject-known-peer-root-change-and-retain-the-opening-draft',
      );
      // Restore the original regression's title so its later independent
      // revoke/close assertions still exercise their original data contract.
      await openEvent(rowId);
      await tap('dataset.record-field.$fieldId');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        'Guest native inherited edit',
      );
      await tap('dataset.cell-apply');
      await tap('dataset.record-close');
      await domain(owner.synchronize);

      Future<void> chooseMode(String label, int mode) async {
        await tap('dataset.calendar-settings');
        await tap('dataset.calendar-mode');
        final option = find.text(label).hitTestable();
        await until(() => option.evaluate().length == 1, 'mode option $label');
        await t.tap(option);
        await tap('dataset.view-apply');
        await until(
          () => calendarOf(owner).mode == mode,
          'original mode $mode',
        );
        await domain(owner.synchronize);
        await until(
          () => calendarOf(guest).mode == mode,
          'peer original mode $mode',
        );
      }

      await chooseMode('周视图', 1);
      final modeEpoch = owner.localMutationEpoch;
      final period = t
          .widget<Text>(find.byKey(const ValueKey('dataset.calendar-period')))
          .data;
      await tap('dataset.calendar-next');
      expect(
        t
            .widget<Text>(find.byKey(const ValueKey('dataset.calendar-period')))
            .data,
        isNot(period),
      );
      expect(owner.localMutationEpoch, modeEpoch);
      await tap('dataset.calendar-today');
      expect(
        t
            .widget<Text>(find.byKey(const ValueKey('dataset.calendar-period')))
            .data,
        period,
      );
      await shot('13-original-week-mode-through-original-shared-root');
      checks.add(
        'formal-week-mode-synchronizes-through-original-root-while-week-browsing-stays-presentation-local',
      );
      await chooseMode('日视图', 2);
      await tap('dataset.calendar-today');
      await tap('dataset.calendar-next');
      final point = find.byKey(
        ValueKey('dataset.calendar-overview.$rowId.$nextDayText'),
      );
      await until(
        () => point.evaluate().isNotEmpty,
        'next-day original point record',
      );
      await t.ensureVisible(point);
      await until(
        () => point.hitTestable().evaluate().length == 1,
        'original point record hit target',
      );
      expect(t.getSize(point).height, greaterThanOrEqualTo(44));
      await t.tap(point);
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-dialog'))
            .evaluate()
            .isNotEmpty,
        'original day record dialog',
      );
      expect(find.text('Guest native inherited edit'), findsWidgets);
      await tap('dataset.record-close');
      await t.ensureVisible(
        find.byKey(const ValueKey('dataset.calendar-period')),
      );
      await t.pump(const Duration(milliseconds: 200));
      await shot('14-original-day-mode-and-source-record-point');
      checks.add(
        'formal-day-mode-opens-the-original-undurationed-record-without-inventing-an-endpoint',
      );

      await catalog();
      await tap('cloud-page.${db['parentViewId']}');
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'original parent document',
      );
      await tap('smart-document.insert');
      await t.pump(const Duration(milliseconds: 250));
      await t.tap(find.text('数据库').last);
      await until(
        () => find
            .byKey(const ValueKey('resource-embed-picker'))
            .evaluate()
            .isNotEmpty,
        'formal calendar resource picker',
      );
      await tap('resource-row-$rootId');
      await until(
        () => find.byType(SimpleDialog).evaluate().isNotEmpty,
        'installed supported original view choices',
      );
      final calendarChoice = find.descendant(
        of: find.byType(SimpleDialog),
        matching: find.text('共享月历'),
      );
      await until(
        () => calendarChoice.hitTestable().evaluate().length == 1,
        'calendar view option receives pointer',
      );
      await t.tap(calendarChoice);
      await until(
        () => find
            .byType(DatasetSurface)
            .evaluate()
            .any((e) => (e.widget as DatasetSurface).viewId == calendarId),
        'original calendar inside parent document',
      );
      final embedded = t.widget<DatasetSurface>(find.byType(DatasetSurface));
      expect(embedded.controller, same(owner));
      expect(find.byKey(const ValueKey('dataset.calendar')), findsOneWidget);
      expect(calendarOf(embedded.controller).mode, 2);
      final parentView = t.widget<doc.SmartDocumentView>(
        find.byType(doc.SmartDocumentView),
      );
      final parentEpoch = parentView.controller.localMutationEpoch;
      final expand = find.byWidgetPredicate(
        (w) =>
            w is TextButton &&
            w.key.toString().contains('smart-document.embed-expand.'),
      );
      await t.ensureVisible(expand);
      await t.tap(expand);
      await until(
        () => find.text('收起视图').evaluate().isNotEmpty,
        'expanded private resource viewport',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(owner),
      );
      expect(parentView.controller.localMutationEpoch, parentEpoch);
      await tap('dataset.calendar-next');
      await until(
        () => point.evaluate().isNotEmpty,
        'independent embedded day anchor',
      );
      await t.ensureVisible(
        find.byKey(const ValueKey('dataset.calendar-period')),
      );
      await t.pump(const Duration(milliseconds: 200));
      await shot('15-expanded-original-day-embed-same-owner');
      checks.add(
        'generic-embed-expansion-keeps-the-original-child-owner-and-does-not-mutate-the-parent-document',
      );
      await shot('11-original-calendar-embedded-with-existing-content-owner');
      final parentTab = actors[0].workspace.activeTab!;
      actors[0].workspace.closeTab(parentTab);
      await until(
        () => !actors[0].workspace.tabs.contains(parentTab),
        'calendar parent releases only its child lease',
      );
      expect(await openDatabase(), same(owner));
      await chooseMode('月视图', 0);
      await shot('16-original-month-restored-after-mode-roundtrip');
      checks.add(
        'original-calendar-mode-roundtrip-keeps-shared-root-row-membership-and-date-values',
      );
      checks.add(
        'calendar-document-embed-reuses-the-original-root-owner-and-parent-close-keeps-the-independent-owner',
      );
      await showActor(1);
      await tap('dataset.view-picker');
      await tap('dataset.view-option.${db['viewId']}');
      await showActor(0);
      await shareParent(level: 10);
      await showActor(1);
      await tap('dataset.sync');
      await until(() => guest.canRead && !guest.canWrite, 'downgraded viewer');
      await tap('dataset.view-picker');
      await tap('dataset.view-option.$calendarId');
      expect(
        t
            .widget<IconButton>(
              find.byKey(const ValueKey('dataset.calendar-settings')),
            )
            .onPressed,
        isNull,
      );
      await openEvent(rowId);
      for (final tile in t.widgetList<ListTile>(
        find.descendant(
          of: find.byKey(const ValueKey('dataset.record-dialog')),
          matching: find.byType(ListTile),
        ),
      )) {
        expect(tile.enabled, isFalse);
      }
      await tap('dataset.record-close');
      await shot(
        '12-viewer-original-calendar-retains-date-and-disables-record-editing',
      );
      checks.add(
        'original-share-downgrade-keeps-calendar-readable-with-disabled-settings-and-original-record-mutations',
      );
      await tap('dataset.view-picker');
      await tap('dataset.view-option.${db['viewId']}');

      await tap('dataset.cell.$rowId.$fieldId');
      expect(find.byKey(const ValueKey('dataset.cell-input')), findsNothing);
      checks.add(
        'parent-downgrade-disables-cached-guest-editor-in-actual-host',
      );
      await showActor(0);
      await shareParent(revoke: true);
      await showActor(1);
      await tap('dataset.sync');
      await until(
        () => !guest.canRead && !guest.canWrite,
        'revoked inherited root',
      );
      expect(find.text('无法访问此数据集，请联系所有者'), findsOneWidget);
      expect(
        guest.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .singleWhere((r) => r.id == rowId)
            .cells[fieldId]!
            .value,
        'Guest native inherited edit',
      );
      checks.add(
        'parent-revoke-hides-guest-grid-and-retains-private-native-cache',
      );
      await shot('05-revoked-inherited-database');
      await showActor(0);
      final tab = actors[0].workspace.activeTab!;
      actors[0].workspace.closeTab(tab);
      await until(
        () => !actors[0].workspace.tabs.contains(tab),
        'owner closes bounded resource session',
      );
      final reopened = await openDatabase();
      await until(
        () => reopened.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .any(
              (r) =>
                  r.id == rowId &&
                  r.cells[fieldId]!.value == 'Guest native inherited edit',
            ),
        'owner reopens accepted row',
      );
      checks.add(
        'owner-checkpoint-close-and-reopen-preserve-same-root-row-and-guest-content',
      );
      expect(
        reopened.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .singleWhere((r) => r.id == guestCreated)
            .cells[fieldId]!
            .value,
        'Owner edit in Guest-created row',
      );
      expect(
        reopened.views
            .singleWhere((v) => v.id == boardId)
            .rows
            .singleWhere((r) => r.id == boardCard)
            .cells[checkboxId]!
            .value,
        true,
      );
      await shot('06-owner-reopened-cloud-database');
      expect(calendarOf(reopened).firstDayOfWeek, 0);
      expect(
        calendarOf(reopened).events.any(
          (e) =>
              e.rowId == rowId &&
              e.startDay == nextDayText &&
              e.title == 'Guest native inherited edit',
        ),
        isTrue,
      );
      expect(
        reopened.views
            .singleWhere((v) => v.id == calendarId)
            .allRows
            .map((r) => r.id)
            .toSet(),
        rowIdsBeforeCalendar,
      );
      checks.add(
        'original-calendar-settings-and-original-row-date-reopen-without-an-event-store-or-duplicated-row',
      );

      await t.pumpWidget(const SizedBox());
      await t.pump();
      for (final actor in actors) {
        await domain(actor.center.close);
        for (final plugin in actor.plugins) {
          await domain(plugin.deactivate);
        }
        await domain(actor.cloud.deactivate);
        actor.online.dispose();
        actor.auth.dispose();
      }
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'workspaceId': wid,
          'rootId': rootId,
          'viewId': db['viewId'],
          'rowId': rowId,
          'twoAuthenticatedHostContexts': true,
          'guestNativeRowCreation':
              'passed-original-server-creation-and-own-origin-undo',
          'guestCreatedRowId': guestCreated,
          'guestNativeBoardCreation':
              'passed-original-server-creation-own-undo-and-prefill',
          'boardViewId': boardId,
          'guestCreatedBoardRowId': boardCard,
          'calendarViewId': calendarId,
          'calendarSpecificChecks': 12,
          'calendarScope':
              'Original month/week/day layouts with field-zone clock, original date rows and private viewport expansion; unsupported original flags stay opaque',
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_CALENDAR_MODES_CHECKS=${checks.length}');
    },
    timeout: const Timeout(Duration(minutes: 8)),
  );
}

class _Actor {
  _Actor(
    this.auth,
    this.cloud,
    this.online,
    this.registry,
    this.plugins,
    this.workspace,
    this.center,
  ) : centers = ValueNotifier(center);
  final GoTrueAuthenticationController auth;
  final OpenMuseCloudWorkspacePlugin cloud;
  final AppFlowyDocumentWorkspace online;
  final OpenMusePluginRegistry registry;
  final List<OpenMusePlugin> plugins;
  final LocalWorkspaceController workspace;
  final NativeResourceCenter center;
  final ValueNotifier<NativeResourceCenter?> centers;
}
