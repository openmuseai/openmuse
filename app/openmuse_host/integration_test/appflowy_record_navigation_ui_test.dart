import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart'
    show AppFlowyRowDocumentApi, AppFlowyWorkspaceAccessApi;
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
import 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart';
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
    'record navigation retains original row identities, saved child owners and readonly boundaries',
    (t) async {
      HttpOverrides.global = null;
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      expect(fixture['origin'], 'http://127.0.0.1:18001');
      final database = fixture['database'] as Map,
          users = fixture['users'] as Map;
      final wid = fixture['workspaceId'] as String;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-presentation-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_RECORD_NAVIGATION_EVIDENCE=${evidence.path}');
      final choices = <String, Future<doc.PickedDocumentImage?> Function()>{};
      final faults = <String, _CleanupFailureStore>{};
      final actors = <_Actor>[];
      for (final name in ['alice', 'bob', 'outsider']) {
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
        choices[name] = () async => null;
        final plugins = createNativeSmartDocumentPlugins(
          cloudDocuments: online,
          imagePicker: () => choices[name]!(),
          documentStoreFactory: (session) async {
            final store = faults.putIfAbsent(
              'body-$name-${session.record.identity.key}',
              () => _CleanupFailureStore(
                doc.FileDocumentStore(
                  File(
                    '${root.path}/$name/${resourceDigest(utf8.encode(session.record.identity.key))}.body.json',
                  ),
                ),
              ),
            );
            faults['body-$name'] = store;
            return store;
          },
          replacementDraftStoreFactory: (session) async => faults.putIfAbsent(
            'replacement-$name-${session.record.identity.key}',
            () => _CleanupFailureStore(
              doc.FileDocumentStore(
                File(
                  '${root.path}/$name/${resourceDigest(utf8.encode(session.record.identity.key))}.replacement.json',
                ),
              ),
            ),
          ),
        );
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

      Future<db.DatasetSurface> openDatabase() async {
        await tap('cloud-workspace.open');
        await tap('cloud-workspace.$wid');
        await tap('cloud-page.${database['viewId']}');
        await until(
          () => find.byType(db.DatasetSurface).evaluate().length == 1,
          'original Root editor',
        );
        final surface = t.widget<db.DatasetSurface>(
          find.byType(db.DatasetSurface),
        );
        await until(
          () =>
              surface.controller.canRead &&
              surface.controller.views.any(
                (v) => v.rows.any(
                  (r) => r.id == (database['rows'] as List).first && r.loaded,
                ),
              ),
          'original Row dependency',
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
        await t.ensureVisible(f);
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
                      settings: actors[i].settings,
                    ),
                  ),
                ),
            ],
          ),
        ),
      );
      final row = (database['rows'] as List).first as String;
      Future<doc.SmartDocumentView> body({String? objectId}) async {
        await until(() {
          final found = find.byType(doc.SmartDocumentView);
          return found.evaluate().length == 1 &&
              (objectId == null ||
                  t.widget<doc.SmartDocumentView>(found).controller.objectId ==
                      objectId);
        }, 'original embedded Document0');
        final view = t.widget<doc.SmartDocumentView>(
          find.byType(doc.SmartDocumentView),
        );
        await until(
          () => view.controller.canRead && view.realtime?.connected == true,
          'child original realtime',
        );
        return view;
      }

      String text(doc.SmartDocumentView view) => view.controller.blocks
          .map((b) => b.delta?.toPlainText() ?? '')
          .join();
      Future<void> openRecord() async {
        await tap('dataset.record-open.$row');
        await until(
          () => find
              .byKey(const ValueKey('dataset.record-dialog'))
              .evaluate()
              .isNotEmpty,
          'record detail',
        );
      }

      Future<void> paste(doc.SmartDocumentView view, String value) async {
        await focus(view.editor);
        final old = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await Clipboard.setData(ClipboardData(text: value));
          await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
          await t.sendKeyEvent(LogicalKeyboardKey.keyV);
          await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        } finally {
          await Clipboard.setData(ClipboardData(text: old?.text ?? ''));
        }
      }

      final owner = await openDatabase();
      await openRecord();
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-document-create'))
            .evaluate()
            .isNotEmpty,
        'explicit first body action',
      );
      final before = await domain(
        () => actors[0].cloud.service.inspectRowDocument(
          wid,
          owner.controller.objectId,
          row,
        ),
      );
      expect(before.exists, isFalse);
      checks.add(
        'actual-grid-record-detail-keeps-absent-body-uncreated-until-explicit-action',
      );
      await shot('01-existing-record-missing-body');
      await tap('dataset.record-document-create');
      final first = await body();
      void trace(String stage, doc.SmartDocumentView v) {
        debugPrint(
          'RECORD_HISTORY_TRACE=${jsonEncode({'stage': stage, 'text': text(v), 'epoch': v.controller.localMutationEpoch, 'canUndo': v.controller.canUndo, 'handle': v.controller.handle, 'selection': v.editor.selection?.toString()})}',
        );
      }

      first.editor.transactionStream.listen((e) {
        debugPrint('RECORD_OWNER_TX=${e.$1}:${e.$2.operations}');
      });
      await until(
        () => first.controller.canEditPresentation(first.presentation),
        'child writable projection',
      );
      expect(first.controller.objectId, before.documentId);
      expect(first.presentation, isNotNull);
      expect(owner.controller.isClosed, isFalse);
      checks.add(
        'explicit-create-opens-original-Document0-inside-existing-record-attributes',
      );
      await paste(first, '记录内编辑原正文😀');
      await until(
        () => text(first).contains('记录内编辑原正文'),
        'native record body input',
      );
      await shot('02-original-record-body-edited');
      trace('owner-after-own-input', first);
      await showActor(1);
      final member = await openDatabase();
      await openRecord();
      final peer = await body();
      peer.editor.transactionStream.listen((e) {
        debugPrint('RECORD_PEER_TX=${e.$1}:${e.$2.operations}');
      });
      await until(
        () => text(peer).contains('记录内编辑原正文'),
        'member original child content',
      );
      expect(peer.controller.objectId, first.controller.objectId);
      expect(member.controller.objectId, owner.controller.objectId);
      checks.add(
        'second-authenticated-record-editor-reuses-the-same-original-Root-Row-Document-identities',
      );
      await paste(peer, '同伴正文😀');
      await until(() => text(first).contains('同伴正文'), 'live peer child edit');
      trace('owner-after-peer-input', first);
      trace('peer-after-peer-input', peer);
      checks.add('native-input-in-record-body-reaches-the-original-live-peer');
      await showActor(0);
      await focus(first.editor, offset: 0);
      trace('owner-after-focus-before-undo', first);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await t.pump(const Duration(milliseconds: 400));
      trace('owner-after-keyboard-undo', first);
      trace('peer-after-keyboard-undo', peer);
      await until(
        () => text(first) == '同伴正文😀' && text(peer) == '同伴正文😀',
        'own child undo retains peer',
      );
      checks.add(
        'native-child-undo-preserves-peer-text-and-does-not-undo-Root-fields',
      );
      await tap('dataset.record-close');
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-dialog'))
            .evaluate()
            .isEmpty,
        'prepared child dialog close',
      );
      await openRecord();
      var reopened = await body();
      expect(reopened.controller.objectId, before.documentId);
      await until(() => text(reopened) == '同伴正文😀', 'original child reopen');
      expect(owner.controller.isClosed, isFalse);
      checks.add(
        'record-dialog-close-and-reopen-retain-original-child-content-and-parent-Root-owner',
      );
      await shot('03-original-record-body-reopened');
      final currentView = owner.controller.views.singleWhere(
        (v) => v.id == database['viewId'],
      );
      final position = currentView.rows.indexWhere((r) => r.id == row);
      expect(position, greaterThanOrEqualTo(0));
      expect(position + 1, lessThan(currentView.rows.length));
      final nextRow = currentView.rows[position + 1].id;
      final rootState = (owner.controller.bundle['objects'] as List)
          .singleWhere((o) => o['collabType'] == 1)['docState'];
      final nextInfo = await domain(
        () => actors[0].cloud.service.inspectRowDocument(
          wid,
          owner.controller.objectId,
          nextRow,
        ),
      );
      expect(nextInfo.exists, isFalse);
      await tap('dataset.record-next');
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-document-create'))
            .evaluate()
            .isNotEmpty,
        'next original row has no implicit body',
      );
      expect(reopened.controller.isClosed, isTrue);
      expect(
        (await domain(
          () => actors[0].cloud.service.inspectRowDocument(
            wid,
            owner.controller.objectId,
            nextRow,
          ),
        )).exists,
        isFalse,
      );
      checks.add(
        'current-view-next-record-prepares-old-child-without-implicitly-creating-neighbor-body',
      );
      await shot('07-current-view-next-record-missing-body');
      await tap('dataset.record-document-create');
      final secondBody = await body(objectId: nextInfo.documentId);
      expect(secondBody.controller.objectId, nextInfo.documentId);
      expect(secondBody.controller.objectId, isNot(before.documentId));
      checks.add(
        'explicit-neighbor-body-reuses-its-own-original-RowDetail-Document0-identity',
      );
      await paste(secondBody, '第二条原记录正文😀');
      await until(
        () => text(secondBody).contains('第二条原记录正文'),
        'second original body accepts native input',
      );
      expect(text(peer), '同伴正文😀');
      checks.add(
        'editing-second-record-body-does-not-replace-live-peer-first-record-body',
      );
      await tap('dataset.record-previous');
      reopened = await body(objectId: before.documentId);
      expect(reopened.controller.objectId, before.documentId);
      await until(
        () => text(reopened) == '同伴正文😀',
        'previous record restores first original child',
      );
      checks.add(
        'previous-record-reopens-original-first-body-with-peer-text-intact',
      );
      await tap('dataset.record-next');
      final secondReopened = await body(objectId: nextInfo.documentId);
      expect(secondReopened.controller.objectId, nextInfo.documentId);
      await until(
        () => text(secondReopened) == '第二条原记录正文😀',
        'next record restores its saved original child',
      );
      checks.add('returning-to-neighbor-preserves-its-saved-original-body');
      await shot('08-current-view-second-original-body');
      await tap('dataset.record-previous');
      reopened = await body(objectId: before.documentId);
      await until(
        () => text(reopened) == '同伴正文😀',
        'first record before failure injection',
      );
      expect(
        (owner.controller.bundle['objects'] as List).singleWhere(
          (o) => o['collabType'] == 1,
        )['docState'],
        rootState,
      );
      checks.add(
        'record-navigation-and-child-input-do-not-rewrite-parent-Root-fields',
      );
      faults['body-alice']!.failSave = true;
      await paste(reopened, '本机保存失败后仍保留😀');
      await until(
        () => text(reopened).contains('本机保存失败后仍保留'),
        'failed private write retains original child',
      );
      await tap('dataset.record-next');
      await t.pump(const Duration(milliseconds: 500));
      expect(
        find.byKey(const ValueKey('dataset.record-dialog')),
        findsOneWidget,
      );
      expect(reopened.controller.isClosed, isFalse);
      expect(reopened.controller.objectId, before.documentId);
      expect(
        find.byKey(const ValueKey('dataset.record-document-create')),
        findsNothing,
      );
      checks.add(
        'actual-private-body-store-failure-blocks-record-navigation-and-retains-current-original-editor',
      );
      await tap('dataset.record-close');
      await t.pump(const Duration(milliseconds: 500));
      expect(
        find.byKey(const ValueKey('dataset.record-dialog')),
        findsOneWidget,
      );
      expect(reopened.controller.isClosed, isFalse);
      checks.add(
        'failed-private-child-checkpoint-blocks-record-close-with-editor-still-mounted',
      );
      await shot('04-child-save-failure-blocks-record-close');
      faults['body-alice']!.failSave = false;
      await tap('dataset.record-close');
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-dialog'))
            .evaluate()
            .isEmpty,
        'fixed child checkpoint closes',
      );
      checks.add('fixed-private-child-store-allows-prepared-close');
      await showActor(2);
      await openDatabase();
      await openRecord();
      var guest = await body();
      await until(
        () => !guest.controller.canEditPresentation(guest.presentation),
        'inherited readonly child',
      );
      expect(
        find.byKey(const ValueKey('dataset.record-document-create')),
        findsNothing,
      );
      await until(
        () => text(guest).contains('同伴正文'),
        'readonly original child body',
      );
      checks.add(
        'actual-Guest10-opens-existing-record-body-readonly-without-an-implicit-create',
      );
      await shot('05-Guest-readonly-record-body');
      await tap('dataset.record-next');
      final guestSecond = await body(objectId: nextInfo.documentId);
      expect(guestSecond.controller.objectId, nextInfo.documentId);
      await until(
        () => text(guestSecond) == '第二条原记录正文😀',
        'Guest original neighbor body',
      );
      expect(
        guestSecond.controller.canEditPresentation(guestSecond.presentation),
        isFalse,
      );
      expect(
        find.byKey(const ValueKey('dataset.record-document-create')),
        findsNothing,
      );
      checks.add(
        'Guest10-navigates-existing-original-record-bodies-without-enabling-editing-or-creation',
      );
      await shot('09-Guest-readonly-neighbor-body');
      await tap('dataset.record-previous');
      guest = await body(objectId: before.documentId);
      expect(guest.controller.objectId, before.documentId);

      await domain(
        () => actors[0].cloud.service.revokeViewAccess(
          wid,
          database['parentViewId'],
          [users['outsider']['email']],
        ),
      );
      await until(
        () => !guest.controller.canRead,
        'actual source revoke reaches child',
      );
      expect(find.textContaining('同伴正文'), findsNothing);
      checks.add(
        'actual-source-revocation-clears-record-body-without-widening-other-accounts',
      );
      await shot('06-Guest-revoked-record-body');
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
        actor.settings.dispose();
        actor.workspace.dispose();
      }
      active.dispose();
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'threeAuthenticatedHostContexts': true,
          'originalRowDocumentSource': true,
          'databaseRowBodyEntryUi': true,
          'currentViewRecordNavigationUi': true,
          'singleOwnerConnection': true,
          'fullProductAcceptance': 'not-complete',
          'limits': [
            'Actual existing-record creation/body UI slice only; Full board/calendar navigation, empty-content metadata synchronization, full protocol admission, durable delivery, SiYuan parity and independent design acceptance remain open',
          ],
        }),
      );
      debugPrint('SMARTDOC_RECORD_NAVIGATION_CHECKS=${checks.length}');
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
  final settings = OpenMuseLocalSettings()..themeMode = ThemeMode.light;
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

class _CleanupFailureStore implements doc.DocumentStore {
  _CleanupFailureStore(this.inner);
  final doc.DocumentStore inner;
  bool failCleanup = false;
  bool failSave = false;
  @override
  Future<Map<String, dynamic>?> read() => inner.read();
  @override
  Future<void> write(Map<String, dynamic> record) async {
    if (failSave) {
      throw StateError('synthetic private checkpoint unavailable');
    }
    if (failCleanup && record['draft'] == null) {
      throw StateError('synthetic replacement cleanup unavailable');
    }
    await inner.write(record);
  }
}
