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
import 'package:openmuse_dataset/openmuse_dataset.dart';
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
    'readonly-first original Cloud document and database retain their owner when a formal writer joins',
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
      debugPrint('SMARTDOC_AUTHORITY_EVIDENCE=${evidence.path}');
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

      final alice = actors[0];
      Future<void> closeTab(WorkspaceTab tab, {String? group}) async {
        alice.workspace.closeTab(
          tab,
          groupId: group ?? LocalWorkspaceController.primaryEditorGroupId,
        );
        await until(
          () => !alice.workspace
              .tabsFor(group ?? LocalWorkspaceController.primaryEditorGroupId)
              .contains(tab),
          'resource tab closes',
        );
      }

      Future<void> readonlyMenu(ResourceRecord record) async {
        await tap('desktop-resource-center');
        await tap('resource-section-recent');
        await tap('resource-more-${record.identity.resource}');
        await tap('resource-menu-open-readonly');
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

      Future<void> split() async {
        await tap('pane-menu-button:editor');
        await t.tap(find.text('向下切分'));
        await t.pump(const Duration(milliseconds: 350));
        await tap('pane-menu-button:pane-1');
        await t.tap(find.text('绑定新编辑组'));
        await t.pump(const Duration(milliseconds: 350));
      }

      final registered = await openParent();
      await paste(registered.editor, '原始正文😀');
      await until(
        () => text(registered.controller) == '原始正文😀',
        'native seed text',
      );
      final registeredTab = alice.workspace.activeTab!;
      final record = registeredTab.resourceSession!.session.record;
      await closeTab(registeredTab);
      await until(
        () => registered.controller.isDisposed,
        'initial registration owner released',
      );
      await readonlyMenu(record);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
        'readonly menu opens a live document',
      );
      final reader = t.widget<doc.SmartDocumentView>(
        find.byType(doc.SmartDocumentView),
      );
      final c = reader.controller, readProjection = reader.presentation!;
      await until(
        () =>
            c.canRead &&
            reader.realtime?.connected == true &&
            readProjection.projectionCurrent,
        'readonly owner initialized',
      );
      final readerTab = alice.workspace.activeTab!;
      expect(readerTab.resourceSession!.session.allowEditing, isFalse);
      expect(c.editable, isFalse);
      expect(readProjection.canWrite, isFalse);
      final readerState = t.state(surfaceFor(reader.editor));
      final realtime = reader.realtime!,
          clientId = realtime.connection.clientId;
      reader.editor.selection = ae.Selection.collapsed(
        ae.Position(path: [0], offset: 2),
      );
      checks.add(
        'actual-resource-menu-opens-live-readonly-document-with-immutable-presentation-policy',
      );
      await split();
      final group = alice.workspace.editorGroups.singleWhere(
        (g) => g.id != LocalWorkspaceController.primaryEditorGroupId,
      );
      alice.workspace.focusEditorGroup(group.id);
      await openRecord(record);
      await until(
        () =>
            find.byType(doc.SmartDocumentView).evaluate().length == 2 &&
            c.canWrite,
        'fresh authorized writer joins reader owner',
      );
      final writer = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.editor != reader.editor);
      final writerTab = alice.workspace.activeTabFor(group.id)!;
      expect(writerTab.id, isNot(readerTab.id));
      expect(writer.controller, same(c));
      expect(writer.realtime, same(realtime));
      expect(realtime.connection.clientId, clientId);
      expect(t.state(surfaceFor(reader.editor)), same(readerState));
      expect(readProjection.canWrite, isFalse);
      expect(reader.editor.editable, isFalse);
      expect(writer.editor.editable, isTrue);
      expect(c.localPresentationEditors, hasLength(2));
      checks.add(
        'fresh-formal-writer-reuses-readonly-owner-checkpoint-connection-and-preserves-old-view-policy',
      );
      await domain(
        () => expectLater(
          reader.media!.insert(presentation: readProjection),
          throwsA(isA<doc.SmartDocumentFailure>()),
        ),
      );
      await paste(writer.editor, '写面新增😀');
      await until(
        () => text(c).contains('写面新增') && readProjection.projectionCurrent,
        'native input reaches both views',
      );
      checks.add(
        'native-writer-input-projects-into-the-retained-readonly-editor-and-readonly-media-remains-denied',
      );
      writer.editor.selection = ae.Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      final link = find.descendant(
        of: surfaceFor(writer.editor),
        matching: find.byKey(const ValueKey('smart-document.link')),
      );
      await t.tap(link);
      await t.pump(const Duration(milliseconds: 350));
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        'https://example.com/authority',
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(
          c.blocks.first.delta!.toJson(),
        ).contains('https://example.com/authority'),
        'native link dialog commits original text mark',
      );
      checks.add(
        'writer-link-dialog-retains-original-mark-model-after-authority-transfer',
      );
      final png = await domain(() async {
        final recorder = ui.PictureRecorder();
        Canvas(recorder).drawColor(Colors.indigo, BlendMode.src);
        final picture = recorder.endRecording();
        final image = await picture.toImage(2, 2);
        final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
        image.dispose();
        picture.dispose();
        return bytes!.buffer.asUint8List();
      });
      final media = writer.media!;
      final asset = await domain(
        () => media.repository.put(
          owner: c.objectId,
          bytes: png,
          mediaType: 'image/png',
        ),
      );
      await domain(
        () => c.insertImage(
          blockId: 'authority-image',
          asset: asset,
          width: 32,
          height: 32,
          presentation: writer.presentation,
        ),
      );
      await until(
        () =>
            c.blocks.any((b) => b.type == 'image') &&
            readProjection.projectionCurrent,
        'original image block in both views',
      );
      expect(await domain(() => media.repository.read(asset)), png);
      checks.add(
        'original-private-cloud-asset-route-and-original-image-block-work-after-readonly-first-handoff',
      );
      await shot('01-readonly-and-writer-original-cloud');
      await showActor(1);
      final peer = await openParent();
      await until(
        () => text(peer.controller) == text(c),
        'authenticated member receives original document updates',
      );
      await paste(peer.editor, '同伴更新😀');
      await until(
        () =>
            text(c).contains('同伴更新') &&
            c.primaryProjectionCurrent &&
            readProjection.projectionCurrent,
        'peer projects into both views',
      );
      expect(realtime.connection.clientId, clientId);
      checks.add(
        'original-authenticated-member-updates-both-presentations-with-one-local-realtime-actor',
      );
      await showActor(0);
      await closeTab(readerTab);
      expect(c.isDisposed, isFalse);
      expect(
        (media.repository as doc.CachedAssetRepository).allowOffline(),
        isTrue,
      );
      expect(await domain(() => media.repository.read(asset)), png);
      await paste(writer.editor, '只读入口关闭后😀');
      await until(
        () => text(c).contains('只读入口关闭后'),
        'writer continues after original reader session disposal',
      );
      checks.add(
        'closing-initial-readonly-session-keeps-live-writer-private-media-and-owner',
      );
      await closeTab(writerTab, group: group.id);
      await until(
        () => c.isDisposed,
        'last document presentation releases owner',
      );
      expect(
        (media.repository as doc.CachedAssetRepository).allowOffline(),
        isFalse,
      );
      alice.workspace.focusEditorGroup(
        LocalWorkspaceController.primaryEditorGroupId,
      );
      await openRecord(record);
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
        'document reopens from current original provider',
      );
      final reopened = t.widget<doc.SmartDocumentView>(
        find.byType(doc.SmartDocumentView),
      );
      await until(
        () => reopened.controller.canWrite,
        'reopened document authority',
      );
      expect(text(reopened.controller), contains('只读入口关闭后'));
      expect(
        jsonEncode(reopened.controller.snapshot['data']),
        contains('https://example.com/authority'),
      );
      expect(reopened.controller.blocks.any((b) => b.type == 'image'), isTrue);
      checks.add('last-close-and-formal-reopen-retain-text-link-and-image');
      await closeTab(alice.workspace.activeTab!);
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.$wid');
      await tap('cloud-page.${db['viewId']}');
      await until(
        () => find.byType(DatasetSurface).evaluate().length == 1,
        'original database registration',
      );
      final initialData = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      await until(() => initialData.canWrite, 'original root write manifest');
      final dataRecord =
          alice.workspace.activeTab!.resourceSession!.session.record;
      await closeTab(alice.workspace.activeTab!);
      await until(
        () => initialData.isDisposed,
        'initial database registration released',
      );
      await readonlyMenu(dataRecord);
      await until(
        () => find.byType(DatasetSurface).evaluate().length == 1,
        'readonly-first database UI',
      );
      final readData = t.widget<DatasetSurface>(find.byType(DatasetSurface));
      final data = readData.controller,
          readDataState = t.state(find.byType(DatasetSurface));
      await until(() => data.canRead, 'readonly original root initialized');
      expect(data.editable, isFalse);
      expect(readData.allowEditing, isFalse);
      final readDataTab = alice.workspace.activeTab!;
      checks.add(
        'actual-resource-menu-opens-readonly-original-Root1-Row4-database',
      );
      alice.workspace.focusEditorGroup(group.id);
      await openRecord(dataRecord);
      await until(
        () =>
            find.byType(DatasetSurface).evaluate().length == 2 && data.canWrite,
        'database writer joins existing reader',
      );
      final writeData = t
          .widgetList<DatasetSurface>(find.byType(DatasetSurface))
          .singleWhere((v) => v.allowEditing);
      final writeDataTab = alice.workspace.activeTabFor(group.id)!;
      expect(writeData.controller, same(data));
      expect(readData.allowEditing, isFalse);
      expect(
        t.state(
          find.byWidgetPredicate(
            (w) =>
                w is DatasetSurface &&
                identical(w.controller, data) &&
                !w.allowEditing,
          ),
        ),
        same(readDataState),
      );
      final row = data.views.first.rows.first,
          field = data.views.first.columns.first;
      final writeSurface = find.byWidgetPredicate(
        (w) =>
            w is DatasetSurface &&
            identical(w.controller, data) &&
            w.allowEditing,
      );
      final cell = find.descendant(
        of: writeSurface,
        matching: find.byKey(ValueKey('dataset.cell.${row.id}.${field.id}')),
      );
      await t.ensureVisible(cell);
      await t.tap(cell);
      await t.pump(const Duration(milliseconds: 350));
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '正式数据库写面😀',
      );
      await tap('dataset.cell-apply');
      await until(
        () =>
            data.readRow(data.views.first.id, row.id).cells[field.id]?.value ==
            '正式数据库写面😀',
        'original database cell update',
      );
      checks.add(
        'same-original-database-owner-retains-readonly-surface-and-accepts-native-cell-editor-input',
      );
      final previousRows = data.views.first.rows.length;
      final addRow = find.descendant(
        of: writeSurface,
        matching: find.byKey(const ValueKey('dataset.add-row')),
      );
      await t.ensureVisible(addRow);
      await until(
        () => t.widget<TextButton>(addRow).onPressed != null,
        'fresh row creation capability is enabled',
      );
      await t.tap(addRow);
      await until(
        () =>
            data.views.first.rows.length == previousRows + 1 &&
            !data.rowCreationPending,
        'fresh writer uses the original authorized row creation port',
      );
      checks.add(
        'readonly-first-database-adopts-the-fresh-original-row-creation-port-without-raw-collab-creation',
      );

      await shot('02-readonly-and-writer-database');
      await closeTab(writeDataTab, group: group.id);
      expect(data.isDisposed, isFalse);
      expect(readData.allowEditing, isFalse);
      expect(
        t.state(
          find.byWidgetPredicate(
            (w) =>
                w is DatasetSurface &&
                identical(w.controller, data) &&
                !w.allowEditing,
          ),
        ),
        same(readDataState),
      );
      checks.add(
        'writer-first-close-retains-live-original-database-for-readonly-surface',
      );
      await closeTab(readDataTab);
      await until(() => data.isDisposed, 'last database lease releases');
      await openRecord(dataRecord);
      await until(
        () => find.byType(DatasetSurface).evaluate().length == 1,
        'current database reopens',
      );
      final reopenedData = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      await until(
        () => reopenedData.canWrite,
        'current database writer permission',
      );
      expect(
        reopenedData
            .readRow(reopenedData.views.first.id, row.id)
            .cells[field.id]
            ?.value,
        '正式数据库写面😀',
      );
      checks.add(
        'original-provider-database-reopen-retains-cell-change-after-both-close-orders',
      );
      await domain(alice.auth.signOut);
      await until(
        () => !reopenedData.canRead,
        'account lifetime fences transferred original database',
      );
      checks.add(
        'original-account-lifetime-revocation-fences-the-reopened-database',
      );
      await shot('03-original-account-ended');
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
          'limits': [
            'Native image upload uses the production repository port; OS file picker is not exercised.',
            'This narrows local presentation policy; original Cloud authority remains authoritative.',
            'Durable delivery, all SiYuan blocks/fields/views and platform/design gates remain open.',
          ],
        }),
      );
      debugPrint('SMARTDOC_AUTHORITY_CHECKS=${checks.length}');
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
