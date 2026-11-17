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
    'unbound original Cloud group recovers its owner, close barrier and live peer collaboration',
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
      debugPrint('SMARTDOC_UNBOUND_EVIDENCE=${evidence.path}');
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
        FocusManager.instance.primaryFocus?.unfocus();
        active.value = index;
        await t.pump(const Duration(milliseconds: 400));
      }

      Future<doc.SmartDocumentView> openParent() async {
        await tap('cloud-workspace.open');
        await tap('cloud-workspace.$wid');
        await tap('cloud-page.${db['parentViewId']}');
        await until(
          () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
          'original parent document',
        );
        final surface = t.widget<doc.SmartDocumentView>(
          find.byType(doc.SmartDocumentView),
        );
        await until(
          () =>
              surface.controller.canWrite &&
              surface.realtime?.connected == true,
          'original document authority and connection',
        );
        return surface;
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
        await focus(editor);
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await Clipboard.setData(ClipboardData(text: value));
          await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
          await t.sendKeyEvent(LogicalKeyboardKey.keyV);
          await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
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
      final first = await openParent(), c = first.controller;
      await paste(first.editor, '唯一内容会话😀');
      await until(() => text(c) == '唯一内容会话😀', 'native first surface input');
      final alice = actors[0];
      final firstTab = alice.workspace.activeTab!;
      final record = firstTab.resourceSession!.session.record;
      final rt = first.realtime!, clientId = rt.connection.clientId;
      final firstStateBeforeRecovery = t.state(surfaceFor(first.editor));
      final group = alice.workspace.createEditorGroup();
      expect(
        alice.layout.snapshot.bindings.values.any(
          (b) => b.surfaceRef == 'host.editorGroup:${group.id}',
        ),
        isFalse,
      );
      alice.workspace.focusEditorGroup(group.id);
      final context = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(context, alice.center.controller, record.reference()),
      );
      await until(
        () => alice.workspace.tabsFor(group.id).length == 1,
        'API opens the existing resource into an unbound group',
      );
      final recoveredTab = alice.workspace.activeTabFor(group.id)!;
      final recoveredSession = recoveredTab.resourceSession!.session;
      expect(find.byType(doc.SmartDocumentView), findsOneWidget);
      expect(c.localPresentationEditors, hasLength(1));
      await shot('00-pending-unbound-group-before-recovery');
      await tap('title-view');
      await t.tap(find.text('IDE 视图'));
      await t.pump(const Duration(milliseconds: 350));
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 2,
        'two actual Host document presentations',
      );
      final second = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.presentation != null);
      final p = second.presentation!;
      await until(() => p.canWrite, 'secondary projection ready');
      expect(second.controller, same(c));
      expect(second.realtime, same(rt));
      expect(second.editor, isNot(same(first.editor)));
      expect(c.localPresentationEditors, hasLength(2));
      expect(rt.connection.clientId, clientId);
      checks.add(
        'two-host-editor-groups-share-one-original-document-handle-checkpoint-and-realtime-actor',
      );
      expect(t.state(surfaceFor(first.editor)), same(firstStateBeforeRecovery));
      expect(alice.workspace.activeTabFor(group.id), same(recoveredTab));
      expect(recoveredTab.resourceSession!.session, same(recoveredSession));
      expect(
        alice.layout.snapshot.bindings.values.where(
          (b) => b.surfaceRef == 'host.editorGroup:${group.id}',
        ),
        hasLength(1),
      );
      checks.add(
        'api-opened-unbound-resource-recovers-the-original-session-with-one-owner-actor-and-existing-widget-state',
      );
      final recoveredState = t.state(surfaceFor(second.editor));
      final recoveredPane = alice.layout.snapshot.bindings.entries
          .singleWhere(
            (e) => e.value.surfaceRef == 'host.editorGroup:${group.id}',
          )
          .key;
      for (var i = 0; i < 2; i++) {
        await tap('pane-menu-button:$recoveredPane');
        await t.tap(find.text('重置默认布局'));
        await t.pump(const Duration(milliseconds: 350));
        expect(
          alice.layout.snapshot.bindings.values.where(
            (b) => b.surfaceRef == 'host.editorGroup:${group.id}',
          ),
          hasLength(1),
        );
        expect(
          t.state(surfaceFor(first.editor)),
          same(firstStateBeforeRecovery),
        );
        expect(t.state(surfaceFor(second.editor)), same(recoveredState));
      }
      expect(rt.connection.clientId, clientId);
      checks.add(
        'repeated-formal-reset-keeps-one-recovered-binding-and-both-original-editor-states',
      );
      await shot('00-recovered-original-cloud-group');
      final originalClose = recoveredSession.closeParticipant!;
      recoveredSession.closeParticipant = ResourceCloseParticipant(
        prepare: (sessions) async =>
            _RejectedCheckpoint(await originalClose.prepare(sessions)),
      );
      alice.workspace.closeTab(recoveredTab, groupId: group.id);
      await until(
        () =>
            find
                .byKey(const ValueKey('resource-keep-editing'))
                .hitTestable()
                .evaluate()
                .length ==
            1,
        'recovered tab retains its original prepared-close failure UI',
      );
      await tap('resource-keep-editing');
      await until(
        () => !recoveredTab.resourceSession!.confirming,
        'failed recovered close is cancelled',
      );
      expect(alice.workspace.tabsFor(group.id), [recoveredTab]);
      expect(t.state(surfaceFor(second.editor)), same(recoveredState));
      expect(c.canWrite, isTrue);
      expect(rt.connection.clientId, clientId);
      recoveredSession.closeParticipant = originalClose;
      checks.add(
        'recovered-tab-prepared-checkpoint-failure-keeps-the-same-session-editor-and-retryable-lease',
      );
      await paste(second.editor, '第二编辑面😀');
      await until(
        () =>
            text(c).contains('第二编辑面') &&
            c.primaryProjectionCurrent &&
            p.projectionCurrent,
        'secondary input projects into primary',
      );
      await shot('01-two-owner-presentations');
      await showActor(1);
      final peer = await openParent(), other = peer.controller;
      await until(
        () => text(other) == text(c),
        'original Cloud peer receives secondary input',
      );
      expect(peer.realtime!.connection.clientId, isNot(clientId));
      checks.add(
        'authenticated-second-account-receives-secondary-editor-update-through-original-cloud',
      );
      await showActor(0);
      await focus(second.editor, offset: 3);
      await until(
        () =>
            other.editor.remoteSelections.value.length == 1 &&
            other.editor.remoteSelections.value.single.selection.start.offset ==
                3,
        'peer sees secondary focus caret',
      );
      // Peer cursors use server-authenticated slots, not raw Yrs sources.
      final ownerUid = rt.connection.actorUid;
      expect(ownerUid, isNotNull);
      final ownerPresence = peer.realtime!.collaborators
          .where((person) => person.uid == ownerUid)
          .toList();
      expect(ownerPresence, hasLength(1));
      final authenticatedSlot = ownerPresence.single.clientId;
      expect(other.editor.remoteSelections.value.single.id, authenticatedSlot);
      await focus(first.editor, offset: 1);
      await until(
        () =>
            other.editor.remoteSelections.value.length == 1 &&
            other.editor.remoteSelections.value.single.selection.start.offset ==
                1,
        'one actor follows the primary caret',
      );
      expect(rt.connection.clientId, clientId);
      expect(other.editor.remoteSelections.value.single.id, authenticatedSlot);
      final afterFocus = peer.realtime!.collaborators
          .where((person) => person.uid == ownerUid)
          .toList();
      expect(afterFocus, hasLength(1));
      expect(afterFocus.single.clientId, authenticatedSlot);
      checks.add(
        'focus-switch-publishes-one-actor-current-selection-without-a-second-local-collaborator',
      );
      final firstStateBeforeLayout = t.state(surfaceFor(first.editor));
      final secondStateBeforeLayout = t.state(surfaceFor(second.editor));
      for (final viewName in ['IDE 视图', 'Agent 视图']) {
        await tap('title-view');
        final item = find.ancestor(
          of: find.text(viewName),
          matching: find.byType(PopupMenuItem<String>),
        );
        await until(
          () => item.hitTestable().evaluate().length == 1,
          'view menu receives pointer',
        );
        await t.tap(item);
        await t.pump(const Duration(milliseconds: 350));
      }
      expect(t.state(surfaceFor(first.editor)), same(firstStateBeforeLayout));
      expect(t.state(surfaceFor(second.editor)), same(secondStateBeforeLayout));
      expect(rt.connection.clientId, clientId);
      expect(c.localPresentationEditors, hasLength(2));
      await shot('layout-reframe-keeps-two-authenticated-presentations');
      await showActor(1);
      await paste(peer.editor, '真实同伴输入😀');
      await until(
        () =>
            text(c).contains('真实同伴输入') &&
            c.primaryProjectionCurrent &&
            p.projectionCurrent,
        'peer updates both local projections',
      );
      expect(
        first.editor.document.root.children.first.delta!.toPlainText(),
        text(c),
      );
      expect(
        second.editor.document.root.children.first.delta!.toPlainText(),
        text(c),
      );
      await focus(peer.editor, offset: 2);
      await until(
        () =>
            first.editor.remoteSelections.value.length == 1 &&
            second.editor.remoteSelections.value.length == 1,
        'peer selection appears in both owner projections',
      );
      expect(
        first.editor.remoteSelections.value.single.selection,
        second.editor.remoteSelections.value.single.selection,
      );
      checks.add(
        'peer-content-and-selection-project-into-both-original-editor-states',
      );
      checks.add(
        'view-reframe-preserves-both-live-editor-states-one-cloud-actor-and-subsequent-peer-projection',
      );
      await shot('02-authenticated-peer');
      await showActor(0);
      await focus(second.editor);
      await domain(() => p.history());
      await until(
        () =>
            !text(c).contains('第二编辑面') &&
            text(c).contains('真实同伴输入') &&
            text(other) == text(c),
        'own undo preserves peer input',
      );
      checks.add(
        'shared-original-domain-undo-removes-own-secondary-input-and-preserves-peer-input',
      );
      alice.workspace.closeTab(
        firstTab,
        groupId: LocalWorkspaceController.primaryEditorGroupId,
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
        'first surface released',
      );
      expect(c.isDisposed, isFalse);
      expect(p.canWrite, isTrue);
      expect(second.realtime, same(rt));
      await paste(second.editor, '首面关闭后😀');
      await until(
        () => text(other).contains('首面关闭后'),
        'surviving surface still synchronizes',
      );
      checks.add(
        'first-surface-close-retains-original-cloud-connection-private-owner-and-surviving-input',
      );
      await shot('03-surviving-owner-presentation');
      await showActor(1);
      await focus(peer.editor, offset: 1);
      await until(
        () => second.editor.remoteSelections.value.length == 1,
        'survivor retains peer presence',
      );
      await showActor(0);
      alice.workspace.focusEditorGroup(
        LocalWorkspaceController.primaryEditorGroupId,
      );
      unawaited(
        openResourcePage(
          t.element(find.byKey(const ValueKey('desktop-resource-center'))),
          alice.center.controller,
          record.reference(),
        ),
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 2,
        'reopened primary shares surviving Cloud owner',
      );
      final reopened = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.presentation == null);
      expect(reopened.controller, same(c));
      expect(reopened.realtime, same(rt));
      expect(rt.connection.clientId, clientId);
      checks.add(
        'reopening-primary-retains-same-one-cloud-owner-and-client-identity',
      );
      alice.workspace.closeTab(recoveredTab, groupId: group.id);
      await until(
        () =>
            !alice.workspace.tabsFor(group.id).contains(recoveredTab) &&
            find.byType(doc.SmartDocumentView).evaluate().length == 1,
        'explicit recovered close retry succeeds',
      );
      expect(c.isDisposed, isFalse);
      expect(reopened.controller, same(c));
      expect(reopened.realtime, same(rt));
      expect(rt.connection.clientId, clientId);
      checks.add(
        'explicit-recovered-close-retry-releases-only-its-lease-and-keeps-the-reopened-original-owner',
      );
      await shot('03-recovered-close-retry-with-outside-owner');
      final endedGroup = alice.workspace.createEditorGroup();
      alice.workspace.focusEditorGroup(endedGroup.id);
      unawaited(
        openResourcePage(
          t.element(find.byKey(const ValueKey('desktop-resource-center'))),
          alice.center.controller,
          record.reference(),
        ),
      );
      await until(
        () => alice.workspace.tabsFor(endedGroup.id).length == 1,
        'unbound presentation exists before original account ends',
      );
      final endedTab = alice.workspace.activeTabFor(endedGroup.id)!;
      expect(
        alice.layout.snapshot.bindings.values.any(
          (b) => b.surfaceRef == 'host.editorGroup:${endedGroup.id}',
        ),
        isFalse,
      );
      await domain(alice.auth.signOut);
      await until(
        () => !c.canRead && !p.canRead,
        'ended account fences all its presentations',
      );
      expect(find.byType(ae.AppFlowyRichText), findsNothing);
      expect(other.canWrite, isTrue);
      checks.add(
        'account-lifetime-end-hides-all-owner-presentations-without-narrowing-the-other-account',
      );
      expect(endedTab.resourceSession!.session.isPresentationActive, isFalse);
      await tap('title-view');
      await t.tap(find.text('IDE 视图'));
      await t.pump(const Duration(milliseconds: 350));
      await until(
        () => find.text('智能文档无法打开，请检查内容与编辑能力。').evaluate().length == 1,
        'ended unmounted session reports unavailable instead of mounting an owner',
      );
      expect(alice.workspace.tabsFor(endedGroup.id), [endedTab]);
      expect(find.byType(ae.AppFlowyRichText), findsNothing);
      expect(other.canWrite, isTrue);
      checks.add(
        'recovering-an-account-ended-unmounted-session-shows-unavailable-without-restoring-editing-or-starting-a-new-owner',
      );
      await shot('04-ended-owner-account');
      await showActor(1);
      await paste(peer.editor, '同伴继续编辑😀');
      await until(
        () => text(other).contains('同伴继续编辑'),
        'other account continues',
      );
      await shot('05-peer-remains-editable');
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
          'twoAuthenticatedHostContexts': true,
          'singleOwnerConnection': true,
          'fullProductAcceptance': 'not-complete',
          'limits': [
            'Document embeds, close-tree atomicity, durable delivery and complete SiYuan parity remain open',
          ],
        }),
      );
      debugPrint('SMARTDOC_UNBOUND_CHECKS=${checks.length}');
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

class _RejectedCheckpoint implements PreparedResourceClose {
  _RejectedCheckpoint(this.inner);
  final PreparedResourceClose inner;
  @override
  bool get isActive => inner.isActive;
  @override
  Future<void> checkpoint() async =>
      throw const ResourceFailure('INJECTED_PREPARED_CHECKPOINT_FAILURE');
  @override
  Future<void> commit() => inner.commit();
  @override
  void cancel() => inner.cancel();
}
