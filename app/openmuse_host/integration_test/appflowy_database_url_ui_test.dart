import 'dart:async';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart' as ae;
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart'
    show
        AppFlowyWorkspaceAccessApi,
        AppFlowyDatabasePage,
        AppFlowyDocumentPage,
        AppFlowyDocumentCreationApi,
        AppFlowyPageRenameApi;
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original database URLs reuse URL cells, readonly navigation and original Row collaboration',
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
        'smartdoc-database-url-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_DATABASE_URL_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      final metadataReads = _MetadataReadControl(
        Uri.parse(fixture['origin'] as String),
      );
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
          collaborationHttpClientFactory:
              const bool.fromEnvironment('SMARTDOC_METADATA_ORDERING') &&
                  name == 'alice'
              ? metadataReads.client
              : null,
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
        debugPrint('DATABASE_URL_NATIVE begin:$reason');
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (ready()) {
            debugPrint('DATABASE_URL_NATIVE done:$reason');
            return;
          }
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

      Future<void> editUrl(String row, String field, String value) async {
        await tap('dataset.cell.$row.$field');
        await tap('dataset.url-edit');
        await until(
          () => find
              .byKey(const ValueKey('dataset.cell-input'))
              .evaluate()
              .isNotEmpty,
          'original URL scalar editor',
        );
        await t.enterText(
          find.byKey(const ValueKey('dataset.cell-input')),
          value,
        );
        await tap('dataset.cell-apply');
        await until(
          () => find
              .byKey(const ValueKey('dataset.cell-dialog'))
              .evaluate()
              .isEmpty,
          'URL original cell command completed',
        );
      }

      Future<void> copyUrl(String value) async {
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await tap('dataset.url-copy');
          expect((await Clipboard.getData(Clipboard.kTextPlain))?.text, value);
        } finally {
          await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
        }
      }

      const verifyCapabilities = bool.fromEnvironment(
        'SMARTDOC_WRITE_CAPABILITIES',
      );
      Future<void> capabilities(int actor, bool? canWrite, String label) async {
        if (!verifyCapabilities) return;
        final user = users[actor == 0 ? 'alice' : 'outsider'] as Map;
        for (final scope in <(String, int)>[
          (db['parentViewId'] as String, 0),
          (rootId, 1),
          ((db['rows'] as List).first as String, 4),
        ]) {
          final transport = AppFlowyObjectTransport(
            origin: Uri.parse(fixture['origin']),
            workspaceId: wid,
            collabType: scope.$2,
            boundObjectId: scope.$1,
            deviceId: 'native-capability-${fixture['run']}-$actor',
            accessToken: () async => user['accessToken'] as String,
            binding: const CollaborationBinding(
              protocol: 'appflowy.full-sync/1',
              engine: 'original-capability-probe',
              encoding: 'yrs.update-v1',
              schema: 'original',
              epoch: 0,
            ),
          );
          try {
            if (canWrite == null) {
              AppFlowyTransportFailure? denial;
              try {
                await domain(() => transport.readWriteCapabilities(scope.$1));
              } on AppFlowyTransportFailure catch (error) {
                denial = error;
              }
              expect(denial?.code, 'FORBIDDEN');
            } else {
              final result = await domain(
                () => transport.readWriteCapabilities(scope.$1),
              );
              expect(result, isNotNull);
              expect(result!.canWrite, canWrite);
              expect(result.qualifiedEditing, isTrue);
              expect(result.collabType, scope.$2);
              expect(result.maxFullSyncBytes, 50 * 1024 * 1024);
            }
          } finally {
            transport.close();
          }
        }
        checks.add(label);
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
      await capabilities(
        0,
        true,
        'original-owner-object-capabilities-match-host-writer',
      );
      await capabilities(
        1,
        false,
        'original-shared-Guest-object-capabilities-match-host-readonly',
      );
      if (const bool.fromEnvironment('SMARTDOC_CATALOG_NAVIGATION')) {
        final pages = await domain(
          () => actors[0].cloud.service.workspacePages(wid),
        );
        final child = pages.singleWhere((p) => p.viewId == db['viewId']);
        expect((child as AppFlowyDatabasePage).databaseId, rootId);
        expect(child.ancestors, contains(db['parentViewId']));
        final sibling = pages.singleWhere(
          (p) => p.viewId == db['siblingViewId'],
        );
        await catalog();
        await tap('cloud-page.toggle.${db['parentViewId']}');
        expect(
          find.byKey(ValueKey('cloud-page.${db['viewId']}')),
          findsNothing,
        );
        checks.add(
          'original-Folder-hierarchy-collapses-canonical-database-child-without-changing-source',
        );
        final search = find.byKey(const ValueKey('cloud-pages.search'));
        await t.enterText(search, child.name);
        await t.pump(const Duration(milliseconds: 250));
        expect(
          find.byKey(ValueKey('cloud-page.${db['viewId']}')),
          findsOneWidget,
        );
        expect(
          find.byKey(ValueKey('cloud-page.${db['parentViewId']}')),
          findsOneWidget,
        );
        expect(
          find.byKey(ValueKey('cloud-page.${db['siblingViewId']}')),
          sibling.name.toLowerCase().contains(child.name.toLowerCase())
              ? findsOneWidget
              : findsNothing,
        );
        await shot('09-original-catalog-search-with-readable-parent');
        checks.add(
          'original-catalog-search-reveals-matching-child-and-readable-parent-by-stable-identity',
        );
        await tap('cloud-pages.clear-search');
        expect(
          find.byKey(ValueKey('cloud-page.${db['viewId']}')),
          findsNothing,
        );
        await tap('cloud-page.toggle.${db['parentViewId']}');
        checks.add(
          'clearing-catalog-search-restores-private-expansion-state-with-original-Root1-binding',
        );
        final ratio = t.view.devicePixelRatio, oldSize = t.view.physicalSize;
        final catalogState = t.state(find.byType(CloudWorkspacePages));
        final searchController = t.widget<TextField>(search).controller;
        FocusManager.instance.primaryFocus?.unfocus();
        await t.pump(const Duration(milliseconds: 250));
        t.view.physicalSize = Size(430 * ratio, 900 * ratio);
        await until(
          () => t.getSize(find.byType(CloudWorkspacePages)).width == 430,
          'catalog logical width430',
        );
        expect(t.state(find.byType(CloudWorkspacePages)), same(catalogState));
        expect(t.widget<TextField>(search).controller, same(searchController));
        await tap('cloud-pages.search');
        await t.enterText(search, child.name);
        await until(
          () =>
              searchController!.text == child.name &&
              find
                      .byKey(const ValueKey('cloud-pages.clear-search'))
                      .evaluate()
                      .length ==
                  1,
          '430px search text and filtered catalog',
        );
        final share = find.byKey(ValueKey('cloud-page.share.${db['viewId']}'));
        expect(t.getSize(share).height, greaterThanOrEqualTo(44));
        expect(t.takeException(), isNull);
        await shot('10-original-catalog-at430');
        expect(searchController!.text, child.name);
        checks.add(
          '430px-original-catalog-search-path-and-share-target-remain-visible-with44px-actions',
        );
        t.view.physicalSize = oldSize;
        FocusManager.instance.primaryFocus?.unfocus();
        for (var i = 0; i < 2; i++) {
          await until(
            () => find.byType(BackButton).evaluate().length == 1,
            'one catalog route back button',
          );
          await t.tap(find.byType(BackButton));
          await t.pump(const Duration(milliseconds: 250));
        }
        await showActor(1);
        await catalog(refresh: true);
        await until(
          () => find
              .byKey(ValueKey('cloud-page.${db['viewId']}'))
              .evaluate()
              .isNotEmpty,
          'Guest original shared catalog',
        );
        expect(
          find.byKey(ValueKey('cloud-page.${db['siblingViewId']}')),
          findsNothing,
        );
        await t.enterText(
          find.byKey(const ValueKey('cloud-pages.search')),
          sibling.name,
        );
        await t.pump(const Duration(milliseconds: 250));
        expect(find.byType(ListTile), findsNothing);
        await shot('11-original-Guest-catalog-excludes-unshared-sibling');
        checks.add(
          'real-Guest-original-sharing-catalog-and-search-never-reveal-unshared-sibling',
        );
        FocusManager.instance.primaryFocus?.unfocus();
        for (var i = 0; i < 2; i++) {
          await until(
            () => find.byType(BackButton).evaluate().length == 1,
            'one Guest catalog route back button',
          );
          await t.tap(find.byType(BackButton));
          await t.pump(const Duration(milliseconds: 250));
        }
        await showActor(0);
      }
      final owner = await openDatabase();
      await until(() => owner.canWrite, 'owner write');
      final ownerRecord =
          actors[0].workspace.activeTab!.resourceSession!.session.record;
      final databaseRef = ownerRecord.reference();
      final rowId = owner.views.first.rows.first.id,
          fieldId = owner.views.first.columns.first.id;
      await edit(rowId, fieldId, '同一云端记录😀');
      await domain(owner.synchronize);
      await catalog();
      await tap('cloud-page.${db['parentViewId']}');
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'owner parent document',
      );
      final parent = t
          .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .controller;
      await until(() => parent.canWrite, 'parent writer');
      await until(
        () => find
            .byType(ae.AppFlowyRichText)
            .first
            .hitTestable()
            .evaluate()
            .isNotEmpty,
        'native paragraph receives pointer events',
      );
      await t.tap(find.byType(ae.AppFlowyRichText).first);
      await t.pump(const Duration(milliseconds: 200));
      final previous = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '打开共享数据库😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => parent.blocks.first.delta!.toPlainText().contains('打开共享数据库'),
          'native parent text',
        );
      } finally {
        await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
      }
      parent.editor.selection = ae.Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 7,
      );
      await t.pump();
      await tap('smart-document.link');
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        databaseRef.toUri().toString(),
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(
          parent.blocks.first.delta!.toJson(),
        ).contains(databaseRef.toUri().toString()),
        'original href stores reference',
      );
      await tap('smart-document.insert');
      await t.pump(const Duration(milliseconds: 350));
      await t.tap(find.text('数据库').last);
      await until(
        () => find
            .byKey(const ValueKey('resource-embed-picker'))
            .evaluate()
            .isNotEmpty,
        'actual resource picker',
      );
      await tap('resource-row-$rootId');
      await until(
        () => find.byType(DatasetSurface).evaluate().isNotEmpty,
        'owner Cloud embed',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(owner),
      );
      await domain(parent.synchronize);
      final parentRef = actors[0]
          .workspace
          .activeTab!
          .resourceSession!
          .session
          .record
          .reference();
      await tap('dataset.add-field');
      await t.tap(find.text('链接').hitTestable().last);
      await until(
        () => find
            .byKey(const ValueKey('dataset.field-name'))
            .evaluate()
            .isNotEmpty,
        'original URL field creator',
      );
      await t.enterText(
        find.byKey(const ValueKey('dataset.field-name')),
        '资源链接',
      );
      await tap('dataset.field-apply');
      await until(
        () => owner.views.first.columns.any(
          (f) => f.name == '资源链接' && f.semanticType == 'url',
        ),
        'original URLTypeOption field',
      );
      final urlField = owner.views.first.columns
          .singleWhere((f) => f.name == '资源链接')
          .id;
      final parentUri = parentRef.toUri().toString();
      await editUrl(rowId, urlField, parentUri);
      await until(
        () =>
            owner.readRow(owner.views.first.id, rowId).cells[urlField]!.value ==
            parentUri,
        'original Row URL cell',
      );
      await domain(owner.synchronize);
      checks.add(
        'formal-grid-URL-field-and-original-scalar-cell-store-portable-parent-reference-without-a-new-model',
      );
      expect(
        parent.blocks
            .singleWhere((b) => b.type == 'grid')
            .attributes['muse_embed']
            .toString(),
        contains(actors[0].center.hub.scope),
      );
      checks.add(
        'original-cloud-document-href-and-picker-embed-keep-owner-root-identity',
      );
      await shot('01-owner-cloud-link-and-embedded-database');

      await showActor(1);
      expect(
        actors[1].center.hub.providers.any(
          (p) => p.source.id == actors[1].online.sourceId,
        ),
        isFalse,
      );
      final guestContext = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(guestContext, actors[1].center.controller, parentRef),
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'recipient resolves owner document URI',
      );
      final guestDoc = t
          .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .controller;
      await until(
        () => find.byType(DatasetSurface).evaluate().isNotEmpty,
        'recipient Cloud embed resolves sender URI',
      );
      final guest = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      await until(
        () => guest.canRead && !guest.canWrite && !guestDoc.canWrite,
        'inherited readonly document and database',
      );
      expect(guest.objectId, rootId);
      expect(
        guest.views.first.rows
            .singleWhere((r) => r.id == rowId)
            .cells[fieldId]!
            .value,
        '同一云端记录😀',
      );
      expect(
        actors[1]
            .workspace
            .activeTab!
            .resourceSession!
            .session
            .record
            .identity
            .scope,
        actors[1].center.hub.scope,
      );
      expect(actors[1].center.hub.scope, isNot(parentRef.identity.scope));
      checks.add(
        'recipient-current-account-authorizes-portable-document-and-embedded-root-without-prior-catalog',
      );
      await shot('02-recipient-inherited-readonly-cloud-embed');

      final linkedText = find
          .byWidgetPredicate(
            (w) => w is RichText && w.text.toPlainText() == '打开共享数据库😀',
          )
          .first;
      await t.ensureVisible(linkedText);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.tapAt(t.getTopLeft(linkedText) + const Offset(8, 10));
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(
        () =>
            actors[1]
                    .workspace
                    .activeTab!
                    .resourceSession!
                    .session
                    .record
                    .identity
                    .mount ==
                'databases' &&
            find.byType(DatasetSurface).evaluate().isNotEmpty,
        'original href opens independent Cloud database',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(guest),
      );
      final guestRecord =
          actors[1].workspace.activeTab!.resourceSession!.session.record;
      expect(guestRecord.identity.resource, rootId);
      expect(guestRecord.identity.scope, actors[1].center.hub.scope);
      expect(
        guestRecord.descriptor.capabilities,
        isNot(contains(MuseResourceCapability.commit)),
      );
      expect(
        actors[1].center.hub
            .provider(guestRecord.identity)
            .source
            .exactVersions,
        isFalse,
      );
      await tap('dataset.cell.$rowId.$fieldId');
      expect(find.byKey(const ValueKey('dataset.cell-input')), findsNothing);
      checks.add(
        'actual-resource-href-opens-recipient-independent-view-and-reuses-embedded-owner',
      );
      await shot('03-recipient-link-target-same-database-owner');

      final readEpoch = guest.localMutationEpoch;
      await tap('dataset.cell.$rowId.$urlField');
      expect(
        t
            .widget<TextButton>(find.byKey(const ValueKey('dataset.url-edit')))
            .onPressed,
        isNull,
      );
      final originalSize = t.view.physicalSize;
      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      await t.pump(const Duration(milliseconds: 350));
      expect(
        t
            .renderObject<RenderRepaintBoundary>(
              find.byKey(const ValueKey('database-host-1')),
            )
            .size
            .width,
        430,
      );
      await copyUrl(parentUri);
      for (final key in [
        'dataset.url-copy',
        'dataset.url-open',
        'dataset.url-close',
      ]) {
        expect(
          t.getSize(find.byKey(ValueKey(key))).height,
          greaterThanOrEqualTo(44),
        );
      }
      await shot('07-readonly-original-database-url-actions');
      expect(guest.localMutationEpoch, readEpoch);
      checks.add(
        'readonly-recipient-can-copy-original-URL-cell-with44px-actions-without-mutating-Root-or-Row',
      );
      t.view.physicalSize = originalSize;
      await t.pump(const Duration(milliseconds: 350));
      await tap('dataset.url-open');
      await until(
        () =>
            actors[1]
                    .workspace
                    .activeTab!
                    .resourceSession!
                    .session
                    .record
                    .identity
                    .resource ==
                parentRef.identity.resource &&
            find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'URL uses original SDK route to the authenticated parent document',
      );
      expect(
        t
            .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
            .controller,
        same(guestDoc),
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(guest),
      );
      checks.add(
        'URL-cell-internal-resource-navigation-reauthorizes-recipient-and-retains-original-document-and-embedded-Root-owners',
      );
      await t.ensureVisible(linkedText);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.tapAt(t.getTopLeft(linkedText) + const Offset(8, 10));
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(
        () =>
            actors[1]
                .workspace
                .activeTab!
                .resourceSession!
                .session
                .record
                .identity
                .mount ==
            'databases',
        'return through the original document href',
      );
      Future<void> deniedReference(
        ResourceReference reference,
        String code,
      ) async {
        final context = t.element(
          find.byKey(const ValueKey('desktop-resource-center')),
        );
        final message = resourceErrorText(context, ResourceFailure(code));
        final count = actors[1].workspace.tabs.length;
        unawaited(
          openResourcePage(context, actors[1].center.controller, reference),
        );
        await until(
          () =>
              find.byType(AlertDialog).evaluate().isNotEmpty &&
              find.text(message).evaluate().isNotEmpty,
          'reference rejected: $code',
        );
        expect(actors[1].workspace.tabs.length, count);
        await t.tap(
          find
              .descendant(
                of: find.byType(AlertDialog),
                matching: find.byType(TextButton),
              )
              .last,
        );
        await t.pump(const Duration(milliseconds: 350));
      }

      ResourceReference changedRef({String? source, String? resource}) =>
          ResourceReference(
            identity: ResourceIdentity(
              scope: databaseRef.identity.scope,
              source: source ?? databaseRef.identity.source,
              workspace: wid,
              mount: 'databases',
              resource: resource ?? rootId,
            ),
          );
      await deniedReference(
        changedRef(resource: db['siblingRootId']),
        'DENIED',
      );
      await deniedReference(
        changedRef(source: 'unregistered-cloud-origin'),
        'DENIED',
      );
      await deniedReference(
        ResourceReference(
          identity: databaseRef.identity,
          version: ownerRecord.version,
        ),
        'DENIED',
      );
      checks.add(
        'unshared-sibling-forged-source-and-sender-device-pin-denied-by-actual-host-route',
      );
      await shot('04-recipient-no-authority-from-portable-reference');

      await showActor(0);
      await shareParent(level: 30);
      await capabilities(
        1,
        true,
        'original-share-upgrade-refreshes-object-capabilities-without-new-ACL',
      );
      await showActor(1);
      await tap('dataset.sync');
      await until(() => guest.canWrite, 'original share upgrade');
      await edit(rowId, fieldId, '链接接收者协作编辑😀');
      await until(
        () =>
            owner.views.first.rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            '链接接收者协作编辑😀',
        'owner original row receives recipient write',
      );
      await showActor(0);
      await edit(rowId, fieldId, 'URL撤销保留同伴标题😀');
      await showActor(1);
      await until(
        () =>
            guest.readRow(guest.views.first.id, rowId).cells[fieldId]!.value ==
            'URL撤销保留同伴标题😀',
        'original peer title before own URL edit',
      );
      await editUrl(rowId, urlField, 'https://example.test/peer-native');
      await until(
        () =>
            owner.readRow(owner.views.first.id, rowId).cells[urlField]!.value ==
            'https://example.test/peer-native',
        'original Row4 URL update reaches the peer',
      );
      await tap('dataset.undo');
      await until(
        () =>
            guest.readRow(guest.views.first.id, rowId).cells[urlField]!.value ==
                parentUri &&
            owner.readRow(owner.views.first.id, rowId).cells[urlField]!.value ==
                parentUri,
        'own URL undo crosses original Cloud',
      );
      expect(
        guest.readRow(guest.views.first.id, rowId).cells[fieldId]!.value,
        'URL撤销保留同伴标题😀',
      );
      expect(
        owner.readRow(owner.views.first.id, rowId).cells[fieldId]!.value,
        'URL撤销保留同伴标题😀',
      );
      checks.add(
        'original-Row4-URL-collaboration-and-own-UI-undo-preserve-the-peer-other-cell-edit',
      );
      final standalone = actors[1].workspace.activeTab!;
      actors[1].workspace.closeTab(standalone);
      await until(
        () =>
            !actors[1].workspace.tabs.contains(standalone) &&
            find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'independent view closes',
      );
      await tap('smart-document.sync');
      await until(
        () =>
            guestDoc.canWrite &&
            find.byType(DatasetSurface).evaluate().isNotEmpty,
        'parent and child fresh writer capability',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(guest),
      );
      await edit(rowId, fieldId, '嵌入继续协作同一记录😀');
      await until(
        () =>
            owner.views.first.rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            '嵌入继续协作同一记录😀',
        'retained embed owner writes original row',
      );
      checks.add(
        'original-share-upgrade-enables-live-linked-edit-and-embedded-owner-survives-standalone-close',
      );
      await shot('05-recipient-live-cloud-embed-after-independent-close');

      if (const bool.fromEnvironment('SMARTDOC_GRID_COLUMNS')) {
        DatasetViewProjection grid(DatasetController controller) =>
            controller.views.singleWhere((v) => v.id == db['viewId']);
        DatasetColumn column(DatasetController controller, String id) =>
            grid(controller).columns.singleWhere((f) => f.id == id);
        Future<void> columnDialog(String id) async {
          await tap('dataset.column-settings');
          await tap('dataset.column-select');
          final controller = t
              .widget<DatasetSurface>(find.byType(DatasetSurface))
              .controller;
          final field = column(controller, id);
          final label =
              '${field.name}${field.gridSettings?.hidden == true ? '（已隐藏）' : ''}';
          await t.tap(find.text(label).last);
          await t.pumpAndSettle();
        }

        await showActor(0);
        final columnOwner = await openDatabase();
        expect(columnOwner, same(owner));
        final initialWidth = column(owner, fieldId).gridSettings!.width;
        final ratio = t.view.devicePixelRatio;
        t.view.physicalSize = Size(320 * ratio, 900 * ratio);
        await t.pumpAndSettle();
        await columnDialog(fieldId);
        await t.enterText(
          find.byKey(const ValueKey('dataset.column-width')),
          '320',
        );
        expect(
          t
              .getSize(find.byKey(const ValueKey('dataset.column-settings')))
              .height,
          greaterThanOrEqualTo(44),
        );
        expect(
          t.getSize(find.byKey(const ValueKey('dataset.column-apply'))).height,
          greaterThanOrEqualTo(44),
        );
        await shot('22-grid-column-settings-at320');
        await t.tap(find.text('取消'));
        await t.pumpAndSettle();
        expect(column(owner, fieldId).gridSettings!.width, initialWidth);
        checks.add(
          'column-settings-cancel-at320-does-not-change-original-view',
        );
        t.view.physicalSize = Size(430 * ratio, 900 * ratio);
        await t.pumpAndSettle();
        await columnDialog(fieldId);
        await t.enterText(
          find.byKey(const ValueKey('dataset.column-width')),
          '280',
        );
        await shot('23-grid-column-settings-at430');
        await tap('dataset.column-apply');
        await columnDialog(urlField);
        await tap('dataset.column-hidden');
        await tap('dataset.column-apply');
        await domain(owner.synchronize);
        await domain(guest.synchronize);
        await until(
          () =>
              column(guest, fieldId).gridSettings?.width == 280 &&
              column(guest, urlField).gridSettings?.hidden == true,
          'real peer receives original shared per-view column settings',
        );
        expect(
          owner.readRow(grid(owner).id, rowId).cells[urlField]!.value,
          parentUri,
        );
        expect(
          grid(owner).visibleColumns.any((f) => f.id == urlField),
          isFalse,
        );
        await shot('24-original-shared-grid-column-hidden');
        checks.add(
          'column-width-and-hidden-state-sync-original-Root1-without-changing-Row4-value',
        );
        t.view.physicalSize = Size(1280 * ratio, 800 * ratio);
        await t.pumpAndSettle();
        await shareParent(level: 10);
        await showActor(1);
        await tap('smart-document.sync');
        await domain(guest.synchronize);
        await until(
          () => !guest.canWrite,
          'original Guest readonly column configuration',
        );
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('dataset.column-settings')),
              )
              .onPressed,
          isNull,
        );
        checks.add('original-Guest10-cannot-change-shared-column-settings');
        await showActor(0);
        await shareParent(level: 30);
        await showActor(1);
        await tap('smart-document.sync');
        await domain(guest.synchronize);
        await until(
          () => guest.canWrite,
          'original Guest column writer restored',
        );
        await columnDialog(urlField);
        await t.enterText(
          find.byKey(const ValueKey('dataset.column-width')),
          '220',
        );
        await tap('dataset.column-hidden');
        await tap('dataset.column-apply');
        await domain(guest.synchronize);
        await domain(owner.synchronize);
        expect(column(owner, urlField).gridSettings!.hidden, isFalse);
        await domain(
          () => owner.configureGridColumn(
            grid(owner),
            column(owner, fieldId),
            width: 320,
          ),
        );
        await domain(owner.synchronize);
        await domain(guest.synchronize);
        await tap('dataset.undo');
        await domain(guest.synchronize);
        await domain(owner.synchronize);
        expect(column(owner, fieldId).gridSettings!.width, 320);
        expect(column(owner, urlField).gridSettings!.hidden, isTrue);
        await tap('dataset.redo');
        await domain(guest.synchronize);
        await domain(owner.synchronize);
        expect(column(owner, urlField).gridSettings!.width, 220);
        expect(column(owner, urlField).gridSettings!.hidden, isFalse);
        checks.add(
          'column-own-undo-retains-peer-other-column-and-redo-restores-original-shared-setting',
        );
        await showActor(0);
        final current = actors[0].workspace.activeTab!;
        actors[0].workspace.closeTab(current);
        await t.pumpAndSettle();
        final reopenedColumns = await openDatabase();
        expect(reopenedColumns, same(owner));
        expect(column(reopenedColumns, fieldId).gridSettings!.width, 320);
        expect(column(reopenedColumns, urlField).gridSettings!.width, 220);
        expect(
          reopenedColumns
              .readRow(grid(reopenedColumns).id, rowId)
              .cells[urlField]!
              .value,
          parentUri,
        );
        checks.add(
          'original-independent-view-reopen-retains-shared-column-settings-and-cell-values',
        );
        if (const bool.fromEnvironment('SMARTDOC_GRID_COLUMN_ORDER')) {
          List<String> order(DatasetController controller) =>
              grid(controller).columns.map((f) => f.id).toList();
          Future<void> position(int n) async {
            await tap('dataset.column-position.$urlField');
            await t.tap(find.text('第 $n 列').last);
            await t.pumpAndSettle();
          }

          final originalOrder = order(owner);
          t.view.physicalSize = Size(320 * ratio, 900 * ratio);
          await t.pumpAndSettle();
          await columnDialog(urlField);
          await position(2);
          await shot('25-grid-column-position-cancel-at320');
          await t.tap(find.text('取消'));
          await t.pumpAndSettle();
          expect(order(owner), originalOrder);
          checks.add('column-order-cancel-at320-keeps-original-sequence');
          t.view.physicalSize = Size(430 * ratio, 900 * ratio);
          await t.pumpAndSettle();
          await columnDialog(urlField);
          await position(2);
          await t.enterText(
            find.byKey(const ValueKey('dataset.column-width')),
            '230',
          );
          await tap('dataset.column-apply');
          await domain(owner.synchronize);
          await domain(guest.synchronize);
          expect(order(owner)[1], urlField);
          expect(order(guest), order(owner));
          expect(column(guest, urlField).gridSettings!.width, 230);
          expect(
            owner.readRow(grid(owner).id, rowId).cells[urlField]!.value,
            parentUri,
          );
          await shot('26-original-shared-grid-column-moved-at430');
          checks.add(
            'column-order-and-width-sync-original-Root1-with-Row4-value-preserved',
          );

          t.view.physicalSize = Size(1280 * ratio, 800 * ratio);
          await showActor(1);
          await columnDialog(urlField);
          await position(originalOrder.length);
          await t.enterText(
            find.byKey(const ValueKey('dataset.column-width')),
            '480',
          );
          final peerDraft = grid(owner);
          await domain(
            () => owner.configureGridColumn(
              peerDraft,
              peerDraft.columns.firstWhere((f) => f.id == fieldId),
              moveToFieldId: peerDraft.columns.last.id,
            ),
          );
          await domain(owner.synchronize);
          await domain(guest.synchronize);
          final peerOrder = order(owner);
          await tap('dataset.column-apply');
          expect(find.text('列设置已变化或无法保存，请关闭后重新打开'), findsOneWidget);
          expect(order(guest), peerOrder);
          expect(column(guest, urlField).gridSettings!.width, 230);
          await shot('27-original-stale-column-order-draft-rejected');
          await t.tap(find.text('取消'));
          await t.pumpAndSettle();
          checks.add(
            'stale-column-order-draft-rejects-width-and-move-without-overwriting-peer',
          );

          await columnDialog(urlField);
          await position(originalOrder.length);
          await t.enterText(
            find.byKey(const ValueKey('dataset.column-width')),
            '240',
          );
          await tap('dataset.column-apply');
          await domain(guest.synchronize);
          await domain(owner.synchronize);
          final moved = order(owner);
          expect(moved.last, urlField);
          await domain(
            () => owner.configureGridColumn(
              grid(owner),
              column(owner, fieldId),
              width: 360,
            ),
          );
          await domain(owner.synchronize);
          await domain(guest.synchronize);
          await tap('dataset.undo');
          await domain(guest.synchronize);
          await domain(owner.synchronize);
          expect(order(owner), peerOrder);
          expect(column(owner, urlField).gridSettings!.width, 230);
          expect(column(owner, fieldId).gridSettings!.width, 360);
          await tap('dataset.redo');
          await domain(guest.synchronize);
          await domain(owner.synchronize);
          expect(order(owner), moved);
          expect(column(owner, urlField).gridSettings!.width, 240);
          expect(column(owner, fieldId).gridSettings!.width, 360);
          await showActor(0);
          actors[0].workspace.closeTab(actors[0].workspace.activeTab!);
          await t.pumpAndSettle();
          final reopenedOrder = await openDatabase();
          expect(order(reopenedOrder), moved);
          expect(
            reopenedOrder
                .readRow(grid(reopenedOrder).id, rowId)
                .cells[urlField]!
                .value,
            parentUri,
          );
          checks.add(
            'column-order-own-undo-redo-and-reopen-preserve-peer-width-and-original-values',
          );
          if (const bool.fromEnvironment('SMARTDOC_GRID_CELL_WRAP')) {
            const longValue =
                '真实协作的长文本😀，保留原始中文与组合字符 e\u0301。\n第二行继续验证自动换行，列宽改变不会重建或截断记录值。\n第三行确认同一记录所有单元格保持对齐。';
            await domain(
              () => owner.configureGridColumn(
                grid(owner),
                column(owner, fieldId),
                width: 180,
                wrap: false,
                moveToFieldId: grid(owner).columns.first.id,
              ),
            );
            await tap('dataset.cell.$rowId.$fieldId');
            await t.enterText(
              find.byKey(const ValueKey('dataset.cell-input')),
              longValue,
            );
            await tap('dataset.cell-apply');
            await domain(owner.synchronize);
            await domain(guest.synchronize);
            final collapsedHeight = t
                .getSize(find.byKey(ValueKey('dataset.row.$rowId')))
                .height;
            final wrapRatio = t.view.devicePixelRatio;
            t.view.physicalSize = Size(320 * wrapRatio, 900 * wrapRatio);
            await t.pumpAndSettle();
            await columnDialog(fieldId);
            await tap('dataset.column-wrap');
            await shot('28-original-grid-wrap-settings-at320');
            await tap('dataset.column-apply');
            await domain(owner.synchronize);
            await domain(guest.synchronize);
            expect(column(guest, fieldId).gridSettings!.wrap, isTrue);
            expect(
              guest.readRow(grid(guest).id, rowId).cells[fieldId]!.value,
              longValue,
            );
            final wrappedHeight = t
                .getSize(find.byKey(ValueKey('dataset.row.$rowId')))
                .height;
            expect(wrappedHeight, greaterThan(collapsedHeight));
            expect(
              t
                  .getSize(
                    find.byKey(ValueKey('dataset.grid-cell.$rowId.$fieldId')),
                  )
                  .height,
              closeTo(wrappedHeight, .01),
            );
            expect(
              t
                  .getSize(
                    find.byKey(ValueKey('dataset.grid-cell.$rowId.$urlField')),
                  )
                  .height,
              closeTo(wrappedHeight, .01),
            );
            t.view.physicalSize = Size(430 * wrapRatio, 900 * wrapRatio);
            await t.pumpAndSettle();
            expect(t.getSize(find.byType(DatasetSurface).first).width, 430);
            await tap('title-sidebar-toggle');
            expect(
              find.byKey(const ValueKey('workspace-navigation-drawer')),
              findsOneWidget,
            );
            await shot('32-original-host-navigation-drawer-at430');
            await t.tapAt(const Offset(420, 400));
            await t.pumpAndSettle();
            expect(
              find.byKey(const ValueKey('workspace-navigation-drawer')),
              findsNothing,
            );
            expect(column(owner, fieldId).gridSettings!.wrap, isTrue);
            expect(
              owner.readRow(grid(owner).id, rowId).cells[fieldId]!.value,
              longValue,
            );
            await shot('29-original-shared-grid-wrapped-at430');
            t.view.physicalSize = Size(1280 * wrapRatio, 800 * wrapRatio);
            await t.pumpAndSettle();
            await shot('30-original-shared-grid-wrapped-aligned');
            checks.add(
              'original-wrap-sync-and-actual-aligned-row-height-keep-complete-Row4-text',
            );

            t.view.physicalSize = Size(320 * wrapRatio, 900 * wrapRatio);
            await t.pumpAndSettle();
            await columnDialog(fieldId);
            await tap('dataset.column-wrap');
            await t.tap(find.text('取消'));
            await t.pumpAndSettle();
            expect(column(owner, fieldId).gridSettings!.wrap, isTrue);
            expect(
              owner.readRow(grid(owner).id, rowId).cells[fieldId]!.value,
              longValue,
            );
            checks.add(
              'wrap-cancel-at320-keeps-original-config-and-complete-value',
            );

            t.view.physicalSize = Size(1280 * wrapRatio, 800 * wrapRatio);
            await showActor(1);
            await columnDialog(fieldId);
            await tap('dataset.column-wrap');
            await tap('dataset.column-apply');
            await domain(guest.synchronize);
            await domain(owner.synchronize);
            await domain(
              () => owner.configureGridColumn(
                grid(owner),
                column(owner, urlField),
                width: 280,
              ),
            );
            await domain(owner.synchronize);
            await domain(guest.synchronize);
            expect(column(guest, fieldId).gridSettings!.wrap, isFalse);
            await tap('dataset.undo');
            await domain(guest.synchronize);
            await domain(owner.synchronize);
            expect(column(owner, fieldId).gridSettings!.wrap, isTrue);
            expect(column(owner, urlField).gridSettings!.width, 280);
            await tap('dataset.redo');
            await domain(guest.synchronize);
            await domain(owner.synchronize);
            expect(column(owner, fieldId).gridSettings!.wrap, isFalse);
            expect(column(owner, urlField).gridSettings!.width, 280);
            expect(
              owner.readRow(grid(owner).id, rowId).cells[fieldId]!.value,
              longValue,
            );
            t.view.physicalSize = Size(430 * wrapRatio, 900 * wrapRatio);
            await t.pumpAndSettle();
            await shot('31-original-grid-nowrap-keeps-full-value');
            checks.add(
              'wrap-own-undo-redo-preserves-peer-width-and-complete-text',
            );

            t.view.physicalSize = Size(1280 * wrapRatio, 800 * wrapRatio);
            await showActor(0);
            actors[0].workspace.closeTab(actors[0].workspace.activeTab!);
            await t.pumpAndSettle();
            final reopenedWrap = await openDatabase();
            expect(column(reopenedWrap, fieldId).gridSettings!.wrap, isFalse);
            expect(
              reopenedWrap
                  .readRow(grid(reopenedWrap).id, rowId)
                  .cells[fieldId]!
                  .value,
              longValue,
            );
            checks.add(
              'original-independent-view-reopen-retains-wrap-and-complete-text',
            );
          }
        }
        await showActor(1);
      }

      await tap('dataset.cell.$rowId.$urlField');
      final staleCopy = t
          .widget<TextButton>(find.byKey(const ValueKey('dataset.url-copy')))
          .onPressed!;
      final staleOpen = t
          .widget<TextButton>(find.byKey(const ValueKey('dataset.url-open')))
          .onPressed!;
      final previousClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(
          const ClipboardData(text: 'synthetic revoked URL sentinel'),
        );
        await showActor(0);
        await shareParent(revoke: true);
        await capabilities(
          1,
          null,
          'original-revocation-denies-object-capabilities-and-retains-local-content',
        );
        await showActor(1);
        await domain(guest.synchronize);
        await until(
          () =>
              !guest.canRead &&
              find.text('链接不可用，或访问权限已变更').evaluate().isNotEmpty,
          'real original parent revocation retires URL reading',
        );
        staleCopy();
        staleOpen();
        var dispatched = 0;
        await expectLater(
          openDatasetUrl(
            guest,
            DatasetUrlReference(
              objectId: rootId,
              viewId: guest.views.first.id,
              rowId: rowId,
              fieldId: urlField,
            ),
            capabilities: DatasetLinkCapabilities(
              open: (_) async {
                dispatched++;
                return true;
              },
            ),
          ),
          throwsA(isA<DatasetFailure>()),
        );
        await t.pump(const Duration(milliseconds: 200));
        expect(dispatched, 0);
        expect(
          (await Clipboard.getData(Clipboard.kTextPlain))?.text,
          'synthetic revoked URL sentinel',
        );
        await shot('08-revoked-original-URL-cell');
        checks.add(
          'real-parent-sharing-revocation-hides-URL-and-fences-stale-read-callbacks-and-public-opener',
        );
        await tap('dataset.url-close');
        await deniedReference(databaseRef, 'DENIED');
        if (guestDoc.canRead) await tap('smart-document.sync');
        await until(
          () => !guestDoc.canRead,
          'parent document revocation confirmed',
        );
        expect(
          find.byWidgetPredicate(
            (w) => w is RichText && w.text.toPlainText() == '打开共享数据库😀',
          ),
          findsNothing,
        );
        checks.add(
          'original-parent-revocation-hides-embedded-content-and-denies-old-live-uri-with-private-cache-present',
        );
        await shot('06-recipient-revoked-cloud-reference');
      } finally {
        await Clipboard.setData(
          ClipboardData(text: previousClipboard?.text ?? ''),
        );
      }
      if (const bool.fromEnvironment('SMARTDOC_DOCUMENT_CREATION')) {
        await showActor(0);
        await shareParent(level: 10);
        await catalog();
        await tap('cloud-pages.create');
        await until(
          () => find
              .byKey(const ValueKey('cloud-document.parent'))
              .evaluate()
              .isNotEmpty,
          'original creation locations',
        );
        final ratio = t.view.devicePixelRatio,
            previousSize = t.view.physicalSize;
        t.view.physicalSize = Size(320 * ratio, 900 * ratio);
        await until(
          () => t.getSize(find.byType(CloudWorkspacePages)).width == 320,
          'creation dialog logical width320',
        );
        await t.enterText(
          find.byKey(const ValueKey('cloud-document.name')),
          '取消的新建草稿',
        );
        await until(
          () =>
              t
                  .widget<FilledButton>(
                    find.byKey(const ValueKey('cloud-document.create')),
                  )
                  .onPressed !=
              null,
          '320px creation command enabled',
        );
        await t.pump(const Duration(milliseconds: 600));
        await shot('12-original-document-create-at320');
        expect(
          t.getSize(find.byKey(const ValueKey('cloud-document.close'))).height,
          greaterThanOrEqualTo(44),
        );
        await tap('cloud-document.close');
        expect(actors[0].cloud.service.pendingDocumentCreation(wid), isNull);
        checks.add(
          'original-create-dialog-cancel-at320-does-not-deliver-a-document-command',
        );
        t.view.physicalSize = Size(430 * ratio, 900 * ratio);
        await tap('cloud-pages.create');
        await until(
          () => find
              .byKey(const ValueKey('cloud-document.parent'))
              .evaluate()
              .isNotEmpty,
          'fresh creation target choices',
        );
        final location = t.widget<DropdownButtonFormField<String>>(
          find.byKey(const ValueKey('cloud-document.parent')),
        );
        if (location.initialValue != db['parentViewId']) {
          await tap('cloud-document.parent');
          await tap('cloud-document.parent.${db['parentViewId']}');
        }
        const createdName = '原群组新建智能文档😀';
        await t.enterText(
          find.byKey(const ValueKey('cloud-document.name')),
          createdName,
        );
        await t.pump(const Duration(milliseconds: 250));
        expect(
          t.getSize(find.byKey(const ValueKey('cloud-document.create'))).height,
          greaterThanOrEqualTo(44),
        );
        await shot('13-original-document-create-at430');
        await tap('cloud-document.create');
        await until(
          () =>
              find.byType(doc.SmartDocumentView).evaluate().isNotEmpty &&
              actors[0]
                      .workspace
                      .activeTab!
                      .resourceSession!
                      .session
                      .record
                      .descriptor
                      .displayName ==
                  createdName,
          'original-created-document-handler',
        );
        t.view.physicalSize = previousSize;
        final created = t
            .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
            .controller;
        final createdRef = actors[0]
            .workspace
            .activeTab!
            .resourceSession!
            .session
            .record
            .reference();
        await until(
          () => created.canWrite,
          'created original Document0 writer',
        );
        final originalPages = await domain(
          () => actors[0].cloud.service.workspacePages(wid),
        );
        final originalCreated = originalPages.singleWhere(
          (p) => p.viewId == createdRef.identity.resource,
        );
        expect(originalCreated, isA<AppFlowyDocumentPage>());
        expect(originalCreated.ancestors.last, db['parentViewId']);
        expect(actors[0].cloud.service.pendingDocumentCreation(wid), isNull);
        checks.add(
          'actual-creation-dialog-publishes-one-canonical-original-Document0-under-selected-parent-and-opens-existing-handler',
        );

        Future<void> pasteCreated(String text) async {
          final rich = find.byType(ae.AppFlowyRichText).first;
          await until(
            () => rich.hitTestable().evaluate().isNotEmpty,
            'created paragraph hit target',
          );
          await t.tap(rich);
          await t.pump(const Duration(milliseconds: 200));
          final before = await Clipboard.getData(Clipboard.kTextPlain);
          try {
            await Clipboard.setData(ClipboardData(text: text));
            await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
            await t.sendKeyEvent(LogicalKeyboardKey.keyV);
            await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
            await until(
              () => created.blocks.any(
                (b) => b.delta?.toPlainText().contains(text) == true,
              ),
              'created document native text',
            );
          } finally {
            await Clipboard.setData(ClipboardData(text: before?.text ?? ''));
          }
          await domain(created.synchronize);
        }

        const initialText = '新建后原协作正文😀';
        await pasteCreated(initialText);
        await shot('14-created-original-document-edited');
        checks.add(
          'new-original-document-accepts-native-editor-input-and-original-Cloud-sync',
        );
        await showActor(1);
        await catalog(refresh: true);
        await until(
          () => find
              .byKey(ValueKey('cloud-page.${createdRef.identity.resource}'))
              .evaluate()
              .isNotEmpty,
          'Guest inherited newly created document',
        );
        expect(find.byKey(const ValueKey('cloud-pages.create')), findsNothing);
        await tap('cloud-page.${createdRef.identity.resource}');
        await until(
          () =>
              find.byType(doc.SmartDocumentView).evaluate().isNotEmpty &&
              actors[1]
                      .workspace
                      .activeTab!
                      .resourceSession!
                      .session
                      .record
                      .identity
                      .resource ==
                  createdRef.identity.resource,
          'Guest created document opens',
        );
        var recipientCreated = t
            .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
            .controller;
        await until(
          () =>
              recipientCreated.canRead &&
              !recipientCreated.canWrite &&
              recipientCreated.blocks.any(
                (b) => b.delta?.toPlainText().contains(initialText) == true,
              ),
          'new document original readonly inheritance',
        );
        await shot('15-created-document-original-Guest-readonly');
        checks.add(
          'original-parent-share-inherits-new-document-to-Guest-without-granting-creation-or-edit-permission',
        );
        await showActor(0);
        const peerText = '创建后的实时更新';
        await pasteCreated(peerText);
        await showActor(1);
        await until(
          () => recipientCreated.blocks.any(
            (b) => b.delta?.toPlainText().contains(peerText) == true,
          ),
          'new document real peer delivery',
        );
        checks.add(
          'new-Document0-live-peer-update-uses-existing-original-collaboration',
        );
        final tab = actors[1].workspace.activeTab!;
        actors[1].workspace.closeTab(tab);
        await until(
          () => !actors[1].workspace.tabs.contains(tab),
          'new document recipient close',
        );
        final context = t.element(
          find.byKey(const ValueKey('desktop-resource-center')),
        );
        unawaited(
          openResourcePage(context, actors[1].center.controller, createdRef),
        );
        await until(
          () =>
              actors[1]
                      .workspace
                      .activeTab!
                      .resourceSession!
                      .session
                      .record
                      .identity
                      .resource ==
                  createdRef.identity.resource &&
              find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
          'new document original reference reopen',
        );
        recipientCreated = t
            .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
            .controller;
        await until(
          () =>
              recipientCreated.canRead &&
              recipientCreated.blocks.any(
                (b) => b.delta?.toPlainText().contains(peerText) == true,
              ),
          'new document reopen retains original content',
        );
        checks.add(
          'created-document-close-and-reference-reopen-preserve-canonical-identity-and-peer-text',
        );
        if (const bool.fromEnvironment('SMARTDOC_PAGE_RENAME')) {
          Future<void> leaveCatalog() async {
            for (var i = 0; i < 2; i++) {
              await until(
                () => find.byType(BackButton).evaluate().length == 1,
                'rename catalog back',
              );
              await t.tap(find.byType(BackButton));
              await t.pump(const Duration(milliseconds: 350));
            }
          }

          await showActor(0);
          final titleSize = t.view.physicalSize,
              titleRatio = t.view.devicePixelRatio;
          final beforeText = jsonEncode(
            created.blocks.map((b) => b.delta?.toJson()).toList(),
          );
          await catalog();
          await tap('cloud-page.rename.${createdRef.identity.resource}');
          await until(
            () =>
                find
                    .byKey(const ValueKey('cloud-rename.name'))
                    .evaluate()
                    .isNotEmpty &&
                t
                        .widget<TextField>(
                          find.byKey(const ValueKey('cloud-rename.name')),
                        )
                        .enabled ==
                    true,
            'original rename ready',
          );
          t.view.physicalSize = Size(320 * titleRatio, 900 * titleRatio);
          await until(
            () => t.getSize(find.byType(CloudWorkspacePages)).width == 320,
            'rename logical width320',
          );
          await t.enterText(
            find.byKey(const ValueKey('cloud-rename.name')),
            '取消的改名草稿',
          );
          await until(
            () =>
                t
                    .widget<FilledButton>(
                      find.byKey(const ValueKey('cloud-rename.submit')),
                    )
                    .onPressed !=
                null,
            'rename320 command enabled',
          );
          await t.pump(const Duration(milliseconds: 600));
          expect(
            t.getSize(find.byKey(const ValueKey('cloud-rename.close'))).height,
            greaterThanOrEqualTo(44),
          );
          await shot('17-original-document-rename-at320');
          await tap('cloud-rename.close');
          final cancelledPages = await domain(
            () => actors[0].cloud.service.workspacePages(wid),
          );
          expect(
            cancelledPages
                .singleWhere((p) => p.viewId == createdRef.identity.resource)
                .name,
            createdName,
          );
          checks.add(
            'original-document-rename-cancel-at320-leaves-Folder-name-and-identity-unchanged',
          );
          t.view.physicalSize = Size(430 * titleRatio, 900 * titleRatio);
          await tap('cloud-page.rename.${createdRef.identity.resource}');
          await until(
            () =>
                find
                    .byKey(const ValueKey('cloud-rename.name'))
                    .evaluate()
                    .isNotEmpty &&
                t
                        .widget<TextField>(
                          find.byKey(const ValueKey('cloud-rename.name')),
                        )
                        .enabled ==
                    true,
            'fresh rename location',
          );
          const renamedTitle = '重命名后的智能文档😀';
          await t.enterText(
            find.byKey(const ValueKey('cloud-rename.name')),
            renamedTitle,
          );
          await until(
            () =>
                t
                    .widget<FilledButton>(
                      find.byKey(const ValueKey('cloud-rename.submit')),
                    )
                    .onPressed !=
                null,
            'rename430 command enabled',
          );
          await t.pump(const Duration(milliseconds: 600));
          expect(
            t.getSize(find.byKey(const ValueKey('cloud-rename.submit'))).height,
            greaterThanOrEqualTo(44),
          );
          await shot('18-original-document-rename-at430');
          await tap('cloud-rename.submit');
          await until(
            () =>
                find
                    .byKey(const ValueKey('cloud-rename.dialog'))
                    .evaluate()
                    .isEmpty &&
                find.text(renamedTitle).evaluate().isNotEmpty &&
                actors[0].workspace.activeTab!.title == renamedTitle,
            'original Folder name and live Host tab confirmed',
          );
          t.view.physicalSize = titleSize;
          await leaveCatalog();
          await until(
            () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
            'renamed owner document visible',
          );
          expect(
            t
                .widget<doc.SmartDocumentView>(
                  find.byType(doc.SmartDocumentView),
                )
                .controller,
            same(created),
          );
          expect(
            t
                .widget<doc.SmartDocumentView>(
                  find.byType(doc.SmartDocumentView),
                )
                .title,
            renamedTitle,
          );
          expect(
            jsonEncode(created.blocks.map((b) => b.delta?.toJson()).toList()),
            beforeText,
          );
          expect(
            actors[0]
                .workspace
                .activeTab!
                .resourceSession!
                .session
                .record
                .identity
                .resource,
            createdRef.identity.resource,
          );
          await shot('19-renamed-original-document-same-owner');
          checks.add(
            'original-Folder-rename-updates-live-Host-tab-and-document-header-without-content-or-owner-replacement',
          );
          await showActor(1);
          await catalog(refresh: true);
          await until(
            () =>
                find.text(renamedTitle).evaluate().isNotEmpty &&
                actors[1].workspace.activeTab!.title == renamedTitle,
            'Guest original catalog name observation',
          );
          expect(
            find.byKey(
              ValueKey('cloud-page.rename.${createdRef.identity.resource}'),
            ),
            findsNothing,
          );
          t.view.physicalSize = Size(430 * titleRatio, 900 * titleRatio);
          await until(
            () => t.getSize(find.byType(CloudWorkspacePages)).width == 430,
            'Guest renamed catalog width430',
          );
          await shot('20-original-Guest-renamed-document-catalog');
          t.view.physicalSize = titleSize;
          await leaveCatalog();
          await until(
            () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
            'Guest renamed document visible',
          );
          expect(
            t
                .widget<doc.SmartDocumentView>(
                  find.byType(doc.SmartDocumentView),
                )
                .controller,
            same(recipientCreated),
          );
          expect(
            t
                .widget<doc.SmartDocumentView>(
                  find.byType(doc.SmartDocumentView),
                )
                .title,
            renamedTitle,
          );
          expect(
            recipientCreated.canRead && !recipientCreated.canWrite,
            isTrue,
          );
          checks.add(
            'Guest-original-catalog-read-projects-the-same-name-to-live-header-without-Folder-or-document-write-permission',
          );
          await showActor(0);
          const renamedPeerText = '改名后仍然协作';
          await pasteCreated(renamedPeerText);
          await showActor(1);
          await until(
            () => recipientCreated.blocks.any(
              (b) => b.delta?.toPlainText().contains(renamedPeerText) == true,
            ),
            'existing document collaboration after metadata rename',
          );
          expect(actors[1].workspace.activeTab!.title, renamedTitle);
          checks.add(
            'original-live-Document0-collaboration-and-checkpointing-retain-verified-name-after-rename',
          );
          final renamedTab = actors[1].workspace.activeTab!;
          actors[1].workspace.closeTab(renamedTab);
          await until(
            () => !actors[1].workspace.tabs.contains(renamedTab),
            'renamed document recipient close',
          );
          final renamedContext = t.element(
            find.byKey(const ValueKey('desktop-resource-center')),
          );
          unawaited(
            openResourcePage(
              renamedContext,
              actors[1].center.controller,
              createdRef,
            ),
          );
          await until(
            () =>
                find.byType(doc.SmartDocumentView).evaluate().isNotEmpty &&
                actors[1]
                        .workspace
                        .activeTab!
                        .resourceSession!
                        .session
                        .record
                        .identity
                        .resource ==
                    createdRef.identity.resource &&
                actors[1].workspace.activeTab!.title == renamedTitle,
            'same original reference resolves renamed document',
          );
          recipientCreated = t
              .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
              .controller;
          await until(
            () =>
                recipientCreated.canRead &&
                !recipientCreated.canWrite &&
                recipientCreated.blocks.any(
                  (b) =>
                      b.delta?.toPlainText().contains(renamedPeerText) == true,
                ),
            'renamed document reopened content',
          );
          checks.add(
            'unchanged-portable-reference-reopens-original-renamed-document-with-peer-content-and-permissions',
          );
          if (const bool.fromEnvironment('SMARTDOC_METADATA_ORDERING')) {
            await showActor(0);
            metadataReads.arm(wid, createdRef.identity.resource);
            var completed = false;
            OpenedResource? resolved;
            Object? resolveFailure;
            unawaited(
              actors[0].center.controller
                  .resolve(createdRef)
                  .then(
                    (value) {
                      resolved = value;
                      completed = true;
                    },
                    onError: (Object error) {
                      resolveFailure = error;
                      completed = true;
                    },
                  ),
            );
            await until(
              () => metadataReads.arrived.isCompleted,
              'real original content GET held after complete authorized read',
            );
            final target = await domain(
              () => actors[0].cloud.service.preparePageRename(
                wid,
                createdRef.identity.resource,
              ),
            );
            const duringReadName = '读取期间完成的原目录改名😀';
            final rename = await domain(
              () => actors[0].cloud.service.renamePage(target, duringReadName),
            );
            expect(rename.observed!.name, duringReadName);
            actors[0].cloud.service.finishPageRename(rename);
            await until(
              () => actors[0].workspace.activeTab!.title == duringReadName,
              'live Host name observes newer Folder while content read pending',
            );
            metadataReads.release.complete();
            await until(() => completed, 'held original content read resolves');
            expect(resolveFailure, isNull);
            expect(resolved!.content.record.name, duringReadName);
            expect(
              resolved!.content.record.identity.resource,
              createdRef.identity.resource,
            );
            expect(metadataReads.heldResponses, 1);
            expect(metadataReads.heldClientClosed, isTrue);
            expect(
              t
                  .widget<doc.SmartDocumentView>(
                    find.byType(doc.SmartDocumentView),
                  )
                  .controller,
              same(created),
            );
            expect(
              t
                  .widget<doc.SmartDocumentView>(
                    find.byType(doc.SmartDocumentView),
                  )
                  .title,
              duringReadName,
            );
            await shot('21-late-original-content-read-keeps-newer-Folder-name');
            checks.add(
              'real-authorized-original-content-read-released-after-rename-uses-newer-verified-name-without-owner-replacement-and-closes-its-client',
            );
          }
        }
        await showActor(0);
        await shareParent(revoke: true);
        await showActor(1);
        await domain(recipientCreated.synchronize);
        await until(
          () => !recipientCreated.canRead,
          'created document original parent revocation',
        );
        await deniedReference(createdRef, 'DENIED');
        await shot('16-created-document-original-parent-revoked');
        checks.add(
          'original-parent-revocation-denies-new-document-live-session-and-old-reference',
        );
      }
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
          'parentViewId': db['parentViewId'],
          'rowId': rowId,
          'twoAuthenticatedHostContexts': true,
          'uriContainsCredentialOrEndpoint': false,
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_DATABASE_URL_CHECKS=${checks.length}');
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

/// Existing per-transport client DI; only one exact synthetic GET is held after
/// the original server has returned all bytes. Credentials are never recorded.
class _MetadataReadControl {
  _MetadataReadControl(this.origin);
  final Uri origin;
  String? workspace, object;
  late Completer<void> arrived, release;
  bool armed = false, heldClientClosed = false;
  int heldResponses = 0;
  void arm(String wid, String oid) {
    expect(armed, isFalse);
    workspace = wid;
    object = oid;
    arrived = Completer<void>();
    release = Completer<void>();
    armed = true;
  }

  http.Client client() => _MetadataReadClient(this, http.Client());
  bool claim(http.BaseRequest request) {
    if (!armed ||
        request.method != 'GET' ||
        request.url.origin != origin.origin ||
        !request.url.path.contains('/$workspace/') ||
        !request.url.path.endsWith('/$object')) {
      return false;
    }
    armed = false;
    return true;
  }
}

class _MetadataReadClient extends http.BaseClient {
  _MetadataReadClient(this.control, this.inner);
  final _MetadataReadControl control;
  final http.Client inner;
  bool held = false;
  @override
  Future<http.StreamedResponse> send(http.BaseRequest request) async {
    final response = await inner.send(request);
    if (!control.claim(request)) return response;
    held = true;
    final bytes = await response.stream.toBytes();
    control.arrived.complete();
    await control.release.future;
    control.heldResponses++;
    return http.StreamedResponse(
      Stream.value(bytes),
      response.statusCode,
      contentLength: bytes.length,
      request: response.request,
      headers: response.headers,
      isRedirect: response.isRedirect,
      persistentConnection: response.persistentConnection,
      reasonPhrase: response.reasonPhrase,
    );
  }

  @override
  void close() {
    if (held) control.heldClientClosed = true;
    inner.close();
  }
}
