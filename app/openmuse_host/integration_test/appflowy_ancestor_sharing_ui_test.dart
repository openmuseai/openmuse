import 'dart:async';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart' as ae;
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original ancestor sharing management and trusted Cloud references in the Host',
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
        'smartdoc-ancestor-sharing-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_ANCESTOR_SHARING_EVIDENCE=${evidence.path}');
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
        final finder = key.startsWith('cloud-access.')
            ? find.byKey(ValueKey(key)).last
            : find.byKey(ValueKey(key));
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
      final owner = await openDatabase();
      await until(() => owner.canWrite, 'owner write');
      final ownerRecord =
          actors[0].workspace.activeTab!.resourceSession!.session.record;
      final databaseRef = ownerRecord.reference();
      final rowId = owner.views.first.rows.first.id,
          fieldId = owner.views.first.columns.first.id;
      await edit(rowId, fieldId, '同一云端记录😀');
      await domain(owner.synchronize);
      final sourceKey =
          'cloud-access.source.${db['parentViewId']}.${users['outsider']['email']}';
      await tap('dataset.share');
      await idle();
      expect(find.byKey(ValueKey(sourceKey)), findsOneWidget);
      expect(
        find.byKey(
          ValueKey('cloud-access.revoke.${users['outsider']['email']}'),
        ),
        findsNothing,
      );
      expect(find.textContaining('继承自上级页面'), findsOneWidget);
      await shot('00-child-share-original-ancestor-source');
      checks.add(
        'child-sharing-ui-shows-original-ancestor-name-and-no-child-revoke',
      );
      await tap(sourceKey);
      await idle();
      expect(find.byType(WorkspaceAccessDialog), findsNWidgets(2));
      await tap('cloud-access.person-permission.${users['outsider']['email']}');
      await tap('cloud-access.person-level.${users['outsider']['email']}.30');
      await idle();
      await shot('00-manage-original-source-dialog');
      await tap('cloud-access.close');
      await idle();
      expect(find.textContaining('可编辑 · 访客 · 继承自上级页面'), findsOneWidget);
      checks.add(
        'manage-source-updates-original-parent-grant-and-refreshes-child-panel',
      );
      await tap(sourceKey);
      await idle();
      await tap('cloud-access.revoke.${users['outsider']['email']}');
      await idle();
      expect(
        find.byKey(
          ValueKey(
            'cloud-access.person-permission.${users['outsider']['email']}',
          ),
        ),
        findsNothing,
      );
      await t.enterText(
        find.byKey(const ValueKey('cloud-access.email')).last,
        users['outsider']['email'],
      );
      await tap('cloud-access.submit');
      await idle();
      expect(
        find.byKey(
          ValueKey(
            'cloud-access.person-permission.${users['outsider']['email']}',
          ),
        ),
        findsOneWidget,
      );
      await tap('cloud-access.close');
      await idle();
      expect(find.textContaining('只读 · 访客 · 继承自上级页面'), findsOneWidget);
      checks.add(
        'original-source-revoke-and-readonly-regrant-through-nested-management-ui',
      );
      await tap('cloud-access.close');
      expect(
        actors[0].center.hub
            .provider(ownerRecord.identity)
            .source
            .workspaceLabel,
        isNot(wid),
      );
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
      await tap('resource-section-remote');
      await tap('resource-source-${ownerRecord.identity.sourceKey}');
      await shot('00-original-cloud-workspace-name-in-resource-picker');
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

      Future<void> deniedReference(
        ResourceReference reference,
        String code,
      ) async {
        final context = t.element(find.byType(DatasetSurface));
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

      await showActor(0);
      await shareParent(revoke: true);
      await showActor(1);
      await tap('dataset.sync');
      await until(() => !guest.canRead, 'revoke invalidates embedded root');
      await deniedReference(databaseRef, 'DENIED');
      await tap('smart-document.sync');
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
      debugPrint('SMARTDOC_ANCESTOR_SHARING_CHECKS=${checks.length}');
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
