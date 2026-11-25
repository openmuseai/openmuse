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
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original link preview embeds an independently authorized document with shared owner and reversible close trees',
    (t) async {
      HttpOverrides.global = null;
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      expect(fixture['origin'], 'http://127.0.0.1:18001');
      final db = fixture['database'] as Map, users = fixture['users'] as Map;
      final wid = fixture['workspaceId'] as String;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-presentation-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_EMBED_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      for (final name in ['alice', 'bob']) {
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
          deviceId: 'presentation-ui-$name-${fixture['run']}',
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
            deviceRef: 'presentation-ui-$name',
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
          await t.pump(const Duration(milliseconds: 80));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
          if (ready()) return;
        }
        fail(
          '$reason: ${t.widgetList<Text>(find.byType(Text)).map((v) => v.data).join(' | ')}',
        );
      }

      Future<T> domain<T>(Future<T> Function() action) async {
        T? result;
        Object? error;
        bool done = false;
        await t.runAsync(() async {
          unawaited(
            action().then(
              (value) {
                result = value;
                done = true;
              },
              onError: (Object e) {
                error = e;
                done = true;
              },
            ),
          );
        });
        await until(() => done, 'domain operation completed');
        if (error != null) throw error!;
        return result as T;
      }

      Future<void> tap(String key) async {
        debugPrint('SMARTDOC_EMBED_TAP_START=$key');
        final f = find.byKey(ValueKey(key));
        await until(() => f.evaluate().isNotEmpty, '$key exists');
        await t.ensureVisible(f);
        await until(
          () => f.hitTestable().evaluate().length == 1,
          '$key receives pointer events',
        );
        await t.tap(f);
        await t.pump(const Duration(milliseconds: 300));
      }

      Future<void> showActor(int index) async {
        debugPrint('SMARTDOC_EMBED_ACTOR_START=$index');
        FocusManager.instance.primaryFocus?.unfocus();
        active.value = index;
        await t.pump(const Duration(milliseconds: 400));
        debugPrint('SMARTDOC_EMBED_ACTOR_DONE=$index');
      }

      Finder surfaceFor(ae.EditorState editor) => find.byWidgetPredicate(
        (w) => w is doc.SmartDocumentView && identical(w.editor, editor),
      );
      Future<void> focus(ae.EditorState editor, {int? offset}) async {
        final f = find
            .descendant(
              of: surfaceFor(editor),
              matching: find.byType(ae.AppFlowyRichText),
            )
            .first;
        await until(
          () => f.hitTestable().evaluate().length == 1,
          'editor receives pointer',
        );
        await t.tap(f);
        await t.pump(const Duration(milliseconds: 100));
        editor.selection = ae.Selection.collapsed(
          ae.Position(
            path: [0],
            offset: offset ?? editor.document.root.children.first.delta!.length,
          ),
        );
        await t.pump(const Duration(milliseconds: 250));
      }

      Future<void> paste(ae.EditorState editor, String value) async {
        debugPrint('SMARTDOC_EMBED_PASTE_START');
        await focus(editor);
        debugPrint('SMARTDOC_EMBED_PASTE_FOCUSED');
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await Clipboard.setData(ClipboardData(text: value));
          await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
          await t.sendKeyEvent(LogicalKeyboardKey.keyV);
          await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
          debugPrint('SMARTDOC_EMBED_PASTE_DONE');
        } finally {
          await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
        }
      }

      String text(doc.SmartDocumentController c) =>
          c.blocks.first.delta!.toPlainText();
      Future<void> shot(String name) async {
        await t.pump(const Duration(milliseconds: 300));
        final boundary = t.renderObject<RenderRepaintBoundary>(
          find.byKey(ValueKey('presentation-host-${active.value}')),
        );
        final image = await boundary.toImage(pixelRatio: 1);
        final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(bytes!.buffer.asUint8List());
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
                    key: ValueKey('presentation-host-$i'),
                    child: OpenMuseHostApp(
                      registry: actors[i].registry,
                      workspace: actors[i].workspace,
                      resources: actors[i].centers,
                      layoutController: actors[i].layout,
                    ),
                  ),
                ),
            ],
          ),
        ),
      );

      final alice = actors[0];
      final childId = fixture['documentChildViewId'] as String;
      Future<doc.SmartDocumentView> openCloud(String id) async {
        debugPrint('SMARTDOC_EMBED_OPEN_CLOUD_START');
        await tap('cloud-workspace.open');
        await tap('cloud-workspace.$wid');
        await tap('cloud-page.$id');
        await until(
          () => t
              .widgetList<doc.SmartDocumentView>(
                find.byType(doc.SmartDocumentView),
              )
              .any((v) => v.controller.objectId == id),
          'original cloud document opens',
        );
        final v = t
            .widgetList<doc.SmartDocumentView>(
              find.byType(doc.SmartDocumentView),
            )
            .firstWhere((v) => v.controller.objectId == id);
        await until(
          () => v.controller.canWrite && v.realtime?.connected == true,
          'current original document permission',
        );
        debugPrint('SMARTDOC_EMBED_OPEN_CLOUD_DONE');
        return v;
      }

      Future<void> openRecord(ResourceRecord record) async {
        final context = t.element(
          find.byKey(const ValueKey('desktop-resource-center')),
        );
        unawaited(
          openResourcePage(
            context,
            alice.center.controller,
            record.reference(),
          ),
        );
        await t.pump(const Duration(milliseconds: 300));
      }

      Future<void> closeTab(WorkspaceTab tab, String group) async {
        alice.workspace.closeTab(tab, groupId: group);
        await until(
          () => !alice.workspace.tabsFor(group).contains(tab),
          'formal tab close',
        );
      }

      Future<void> insert(
        doc.SmartDocumentView v,
        ResourceRecord record,
      ) async {
        final action = find
            .descendant(
              of: surfaceFor(v.editor),
              matching: find.byKey(const ValueKey('smart-document.insert')),
            )
            .first;
        await t.ensureVisible(action);
        await until(
          () => action.hitTestable().evaluate().length == 1,
          'insert button receives pointer',
        );
        await t.tap(action);
        await t.pump(const Duration(milliseconds: 350));
        await t.tap(find.text('嵌入文档').last);
        await until(
          () =>
              find
                  .byKey(const ValueKey('resource-embed-picker'))
                  .evaluate()
                  .length ==
              1,
          'formal resource picker',
        );
        await tap('resource-section-recent');
        await tap('resource-row-${record.identity.resource}');
        await until(
          () => find
              .byKey(const ValueKey('resource-embed-picker'))
              .evaluate()
              .isEmpty,
          'picker resolves current child',
        );
      }

      final registered = await openCloud(childId);
      await paste(registered.editor, '独立子文档😀');
      await until(
        () => text(registered.controller).contains('独立子文档'),
        'native child seed',
      );
      final firstTab = alice.workspace.activeTab!,
          childRecord = firstTab.resourceSession!.session.record;
      await closeTab(firstTab, LocalWorkspaceController.primaryEditorGroupId);
      await until(
        () => registered.controller.isDisposed,
        'initial child registration released',
      );
      final originalProvider = alice.center.hub.provider(childRecord.identity);
      final fault = _CheckpointFault(originalProvider);
      expect(originalProvider, isA<ResourceDeviceCheckpointProvider>());
      alice.center.hub.unregister(originalProvider.source.key);
      alice.center.hub.register(fault);
      await openRecord(childRecord);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
        'standalone child reopens',
      );
      final standalone = t.widget<doc.SmartDocumentView>(
        find.byType(doc.SmartDocumentView),
      );
      final child = standalone.controller, rt = standalone.realtime!;
      await until(
        () => child.canWrite && rt.connected,
        'standalone owner current',
      );
      final clientId = rt.connection.clientId;
      var childTab = alice.workspace.activeTab!;
      await tap('pane-menu-button:editor');
      await t.tap(find.text('向右切分'));
      await t.pump(const Duration(milliseconds: 350));
      await tap('pane-menu-button:pane-1');
      await t.tap(find.text('绑定新编辑组'));
      await t.pump(const Duration(milliseconds: 350));
      final group = alice.workspace.editorGroups.singleWhere(
        (g) => g.id != LocalWorkspaceController.primaryEditorGroupId,
      );
      alice.workspace.focusEditorGroup(group.id);
      final parentView = await openCloud(db['parentViewId'] as String),
          parent = parentView.controller;
      final parentTab = alice.workspace.activeTabFor(group.id)!;
      final parentRecord = parentTab.resourceSession!.session.record;
      await paste(parentView.editor, '父文档😀');
      await insert(parentView, childRecord);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 3,
        'standalone, parent and embedded child',
      );
      var embedded = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere(
            (v) => v.controller.objectId == childId && v.presentation != null,
          );
      await until(
        () => embedded.presentation!.canWrite,
        'embedded child independently authorized',
      );
      expect(embedded.controller, same(child));
      expect(embedded.realtime, same(rt));
      expect(rt.connection.clientId, clientId);
      expect(embedded.editor, isNot(same(standalone.editor)));
      expect(parent.blocks.last.type, 'link_preview');
      expect(parent.blocks.last.attributes['preview_type'], 'embed');
      expect(
        parent.blocks.last.attributes['url'],
        childRecord.reference().toUri().toString(),
      );
      expect(parent.blocks.where((b) => b.type == 'grid'), isEmpty);
      checks.add(
        'formal-picker-inserts-original-link-preview-embed-without-reinterpreting-database-grid',
      );
      checks.add(
        'standalone-and-embedded-document-share-one-owner-checkpoint-realtime-actor-and-independent-editor-states',
      );
      final before = parent.snapshot['docState'];
      final rich = find
          .descendant(
            of: surfaceFor(embedded.editor),
            matching: find.byType(ae.AppFlowyRichText),
          )
          .first;
      await t.ensureVisible(rich);
      await paste(embedded.editor, '嵌入输入😀');
      await until(
        () =>
            text(child).contains('嵌入输入') &&
            embedded.presentation!.projectionCurrent,
        'native embedded input',
      );
      expect(parent.snapshot['docState'], before);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(() => !text(child).contains('嵌入输入'), 'embedded own undo');
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
      await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
      await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(() => text(child).contains('嵌入输入'), 'embedded own redo');
      expect(parent.snapshot['docState'], before);
      checks.add(
        'native-embedded-input-and-original-own-undo-redo-stay-in-child-content-domain',
      );
      await shot('01-original-document-embed-and-standalone');
      await showActor(1);
      final peer = await openCloud(childId);
      await until(
        () => text(peer.controller) == text(child),
        'original member receives child content',
      );
      await paste(peer.editor, '同伴子输入😀');
      await until(
        () =>
            text(child).contains('同伴子输入') &&
            embedded.presentation!.projectionCurrent,
        'peer reaches both local child projections',
      );
      checks.add(
        'authenticated-original-cloud-peer-updates-standalone-and-embedded-child-with-one-local-actor',
      );
      final peerParent = await openCloud(db['parentViewId'] as String);
      await until(
        () =>
            t
                .widgetList<doc.SmartDocumentView>(
                  find.byType(doc.SmartDocumentView),
                )
                .where(
                  (v) =>
                      v.controller.objectId == childId &&
                      v.presentation != null,
                )
                .length ==
            1,
        'peer resolves portable parent reference under its own account',
      );
      final peerEmbedded = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere(
            (v) => v.controller.objectId == childId && v.presentation != null,
          );
      expect(peerEmbedded.controller, same(peer.controller));
      expect(peerEmbedded.realtime, same(peer.realtime));
      expect(peerParent.controller.blocks.last.type, 'link_preview');
      await t.ensureVisible(
        find
            .descendant(
              of: surfaceFor(peerEmbedded.editor),
              matching: find.byType(ae.AppFlowyRichText),
            )
            .first,
      );
      await paste(peerEmbedded.editor, '同伴嵌入输入😀');
      await until(
        () =>
            text(child).contains('同伴嵌入输入') &&
            embedded.presentation!.projectionCurrent,
        'current-account authorized embedded peer projects to both local child views',
      );
      checks.add(
        'portable-document-embed-rebinds-to-the-receiving-original-account-and-reuses-its-authorized-child-owner',
      );

      await showActor(0);
      final childState = t.state(surfaceFor(embedded.editor));
      await closeTab(childTab, LocalWorkspaceController.primaryEditorGroupId);
      expect(child.isDisposed, isFalse);
      expect(rt.connection.clientId, clientId);
      expect(t.state(surfaceFor(embedded.editor)), same(childState));
      await t.ensureVisible(
        find
            .descendant(
              of: surfaceFor(embedded.editor),
              matching: find.byType(ae.AppFlowyRichText),
            )
            .first,
      );
      await paste(embedded.editor, '独立页关闭后😀');
      await until(
        () => text(child).contains('独立页关闭后'),
        'embedded continues after standalone close',
      );
      checks.add(
        'closing-standalone-child-retains-the-same-embedded-editor-content-owner-and-live-connection',
      );
      alice.workspace.focusEditorGroup(
        LocalWorkspaceController.primaryEditorGroupId,
      );
      await openRecord(childRecord);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 3,
        'standalone rejoins same child owner',
      );
      childTab = alice.workspace.activeTabFor(
        LocalWorkspaceController.primaryEditorGroupId,
      )!;
      expect(rt.connection.clientId, clientId);
      fault.failResource = childRecord.identity.resource;
      await t.ensureVisible(
        find
            .descendant(
              of: surfaceFor(embedded.editor),
              matching: find.byType(ae.AppFlowyRichText),
            )
            .first,
      );
      await paste(embedded.editor, '检查点失败后😀');
      await until(
        () => text(child).contains('检查点失败后'),
        'accepted child input before failing save',
      );
      debugPrint('SMARTDOC_EMBED_CLOSE_PARENT_BEFORE');
      alice.workspace.closeTab(parentTab, groupId: group.id);
      debugPrint('SMARTDOC_EMBED_CLOSE_PARENT_AFTER');
      await until(
        () => find
            .byKey(const ValueKey('resource-keep-editing'))
            .evaluate()
            .isNotEmpty,
        'failed child checkpoint keeps parent mounted',
      );
      await tap('resource-keep-editing');
      await until(
        () => !parentTab.resourceSession!.confirming,
        'continue original editors',
      );
      expect(t.state(surfaceFor(embedded.editor)), same(childState));
      expect(parent.canWrite, isTrue);
      expect(child.canWrite, isTrue);
      checks.add(
        'child-device-checkpoint-failure-restores-the-same-parent-and-embedded-original-editor',
      );
      await shot('02-child-checkpoint-failure-retains-documents');
      alice.workspace.closeTab(parentTab, groupId: group.id);
      await until(
        () => find
            .byKey(const ValueKey('resource-preserve-private'))
            .evaluate()
            .isNotEmpty,
        'private recovery close available for document tree',
      );
      final attempts = fault.attempts,
          version = childTab.resourceSession!.session.record.version;
      await tap('resource-preserve-private');
      await until(
        () => !alice.workspace.tabsFor(group.id).contains(parentTab),
        'private parent tree closes',
      );
      expect(fault.attempts, attempts);
      expect(child.isDisposed, isFalse);
      expect(childTab.resourceSession!.session.dirty, isTrue);
      expect(childTab.resourceSession!.session.record.version, version);
      checks.add(
        'private-parent-close-releases-only-its-child-lease-without-new-checkpoint-or-outside-version-change',
      );
      alice.workspace.focusEditorGroup(group.id);
      await openRecord(parentRecord);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 3,
        'parent reopens original reference and same outside child',
      );
      var reopened = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.controller.objectId == parent.objectId);
      embedded = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere(
            (v) => v.controller.objectId == childId && v.presentation != null,
          );
      expect(embedded.controller, same(child));
      expect(text(child), contains('检查点失败后'));
      expect(reopened.controller.blocks.last.type, 'link_preview');
      checks.add(
        'parent-private-reopen-retains-original-link-locator-and-accepted-child-input',
      );
      fault.failResource = null;
      await domain(
        () => childTab.resourceSession!.session.flushPendingChanges!(),
      );
      await until(
        () => !childTab.resourceSession!.session.dirty,
        'explicit current child checkpoint succeeds',
      );

      final writableParentTab = alice.workspace.activeTabFor(group.id)!;
      await closeTab(writableParentTab, group.id);
      alice.workspace.focusEditorGroup(group.id);
      await tap('desktop-resource-center');
      await tap('resource-section-recent');
      await tap('resource-more-${parentRecord.identity.resource}');
      await tap('resource-menu-open-readonly');
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 3,
        'formal readonly parent resolves the same live child',
      );
      reopened = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.controller.objectId == parent.objectId);
      embedded = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere(
            (v) => v.controller.objectId == childId && v.presentation != null,
          );
      await until(
        () => embedded.presentation!.projectionCurrent,
        'readonly child projection ready',
      );
      expect(reopened.presentation!.canWrite, isFalse);
      expect(embedded.presentation!.canWrite, isFalse);
      expect(child.canWrite, isTrue);
      final readonlyParentState = t.state(surfaceFor(reopened.editor));
      final readonlyActor = reopened.realtime!.connection.clientId;
      final beforeDeniedInput = child.snapshot['docState'];
      await t.ensureVisible(
        find
            .descendant(
              of: surfaceFor(embedded.editor),
              matching: find.byType(ae.AppFlowyRichText),
            )
            .first,
      );
      await paste(embedded.editor, '只读父面不能写😀');
      await t.pump(const Duration(milliseconds: 350));
      expect(child.snapshot['docState'], beforeDeniedInput);
      checks.add(
        'formal-readonly-parent-keeps-its-embedded-child-immutable-and-native-paste-cannot-escape-to-the-outside-writer',
      );
      await shot('03-readonly-parent-with-outside-child-writer');
      final outsideWriter = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere(
            (v) => v.controller.objectId == childId && v.presentation == null,
          );
      await insert(outsideWriter, parentRecord);

      await until(
        () => find
            .byKey(const ValueKey('resource-embed-retry'))
            .evaluate()
            .isNotEmpty,
        'recursive original document references are explicitly unavailable',
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 4,
        'bounded two-root reference graph',
      );
      expect(child.blocks.last.type, 'link_preview');
      expect(reopened.controller.blocks.last.type, 'link_preview');
      expect(rt.connection.clientId, clientId);
      expect(t.state(surfaceFor(reopened.editor)), same(readonlyParentState));
      expect(reopened.presentation!.canWrite, isFalse);
      expect(reopened.controller.canWrite, isTrue);
      expect(reopened.realtime!.connection.clientId, readonlyActor);
      checks.add(
        'fresh-writable-nested-reference-reuses-readonly-parent-owner-without-raising-the-old-parent-policy-or-duplicating-its-actor',
      );
      checks.add(
        'A-to-B-to-A-retains-original-reference-blocks-and-stops-recursion-before-another-content-owner',
      );
      await shot('04-cycle-reference-preserved-and-refused');
      await domain(alice.auth.signOut);
      await until(
        () => !child.canRead && !reopened.controller.canRead,
        'original account lifetime hides every shared document view',
      );
      expect(find.byType(ae.AppFlowyRichText), findsNothing);
      checks.add(
        'original-account-lifetime-end-fences-parent-embedded-and-standalone-documents',
      );
      await shot('05-ended-original-account');
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      for (final actor in actors) {
        await domain(actor.center.close);
        for (final plugin in actor.plugins) {
          await domain(plugin.deactivate);
        }
        await domain(actor.cloud.deactivate);
        actor.online.dispose();
        actor.auth.dispose();
        actor.centers.dispose();
        actor.layout.dispose();
        actor.workspace.dispose();
      }
      active.dispose();
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'fullProductAcceptance': 'not-complete',
          'originalLinkPreviewStructure': true,
          'limits': [
            'OS file picker and full external URL iframe previews are not qualified',
            'Durable ACK, exact history/Agent, complete SiYuan parity and independent design/platform gates remain open',
          ],
        }),
      );
      debugPrint('SMARTDOC_EMBED_CHECKS=${checks.length}');
    },
    timeout: const Timeout(Duration(minutes: 10)),
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
  final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
  final GoTrueAuthenticationController auth;
  final OpenMuseCloudWorkspacePlugin cloud;
  final AppFlowyDocumentWorkspace online;
  final OpenMusePluginRegistry registry;
  final List<OpenMusePlugin> plugins;
  final LocalWorkspaceController workspace;
  final NativeResourceCenter center;
  final ValueNotifier<NativeResourceCenter?> centers;
}

// Fault only at the existing device checkpoint port; every successful operation
// delegates to the original Cloud provider and actual local checkpoint storage.
final class _CheckpointFault
    implements ResourceProvider, ResourceDeviceCheckpointProvider {
  _CheckpointFault(this.inner);
  final ResourceProvider inner;
  String? failResource;
  int attempts = 0;
  @override
  ResourceSource get source => inner.source;
  @override
  bool get available => inner.available;
  @override
  Future<ResourcePage> list(
    String parentRef,
    ResourceAccess access, {
    String? cursor,
    int limit = 100,
  }) => inner.list(parentRef, access, cursor: cursor, limit: limit);
  @override
  Future<ResourceRecord> describe(String ref, ResourceAccess access) =>
      inner.describe(ref, access);
  @override
  Future<ResourceContent> read(
    String ref,
    ResourceVersion? version,
    ResourceAccess access,
  ) => inner.read(ref, version, access);
  @override
  Future<ResourceRecord> create({
    required String parentRef,
    required String name,
    required String mediaType,
    required List<int> bytes,
    required String idempotencyKey,
    required ResourceAccess access,
    ResourceReference? provenance,
  }) => inner.create(
    parentRef: parentRef,
    name: name,
    mediaType: mediaType,
    bytes: bytes,
    idempotencyKey: idempotencyKey,
    access: access,
    provenance: provenance,
  );
  @override
  Future<String> createDirectory(
    String parentRef,
    String name,
    ResourceAccess access,
  ) => inner.createDirectory(parentRef, name, access);
  @override
  Future<ResourceRecord> rename(
    String ref,
    String name,
    int revision,
    ResourceAccess access,
  ) => inner.rename(ref, name, revision, access);
  @override
  Future<ResourceRecord> commit(
    String ref,
    ResourceVersion base,
    List<int> bytes,
    String key,
    ResourceAccess access,
  ) => inner.commit(ref, base, bytes, key, access);
  @override
  Future<ResourceRecord> checkpoint(
    ResourceRecord base,
    List<int> bytes,
    String key,
    ResourceAccess access,
  ) async {
    attempts++;
    if (base.identity.resource == failResource) {
      throw const ResourceFailure('TEST_CHECKPOINT_FAILURE');
    }
    return (inner as ResourceDeviceCheckpointProvider).checkpoint(
      base,
      bytes,
      key,
      access,
    );
  }
}
