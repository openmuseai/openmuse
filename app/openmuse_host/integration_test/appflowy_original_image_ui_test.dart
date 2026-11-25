import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart'
    show AppFlowyDocumentPage, AppFlowyWorkspaceAccessApi;
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
    'original imported row document images use the authenticated Host, original renderer and sharing',
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
      debugPrint('SMARTDOC_ORIGINAL_IMAGE_EVIDENCE=${evidence.path}');
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
          replacementDraftStoreFactory: (session) async => faults.putIfAbsent(
            name,
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

      Future<doc.SmartDocumentView> openParent() async {
        await domain(
          () => actors[active.value].online.openDocument(
            AppFlowyDocumentPage(
              workspaceId: wid,
              viewId: fixture['originalImageDocument']['objectId'],
              name: '原导入行正文',
            ),
          ),
        );
        await until(
          () => find.byType(doc.SmartDocumentView).evaluate().length == 1,
          'original parent document',
        );
        final surface = t.widget<doc.SmartDocumentView>(
          find.byType(doc.SmartDocumentView),
        );
        await until(
          () =>
              surface.controller.canRead && surface.realtime?.connected == true,
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
      final original = fixture['originalImageDocument'] as Map;
      final first = await openParent(), c = first.controller;
      expect(c.objectId, original['objectId']);
      expect(c.canWrite, isTrue);
      final image = c.blocks.singleWhere((b) => b.type == 'image');
      final ref = doc.DocumentBlockReference(
        objectId: c.objectId,
        blockId: image.id,
      );
      Map<String, dynamic> attrs(doc.SmartDocumentController owner) =>
          owner.resolveBlock(ref)!.attributes;
      Future<void> bitmap(doc.SmartDocumentView view, String hash) => until(
        () => find
            .descendant(
              of: surfaceFor(view.editor),
              matching: find.byWidgetPredicate(
                (w) =>
                    w is Image &&
                    w.image is MemoryImage &&
                    resourceDigest((w.image as MemoryImage).bytes) == hash,
              ),
            )
            .evaluate()
            .isNotEmpty,
        'original image bytes rendered',
      );
      Future<void> properties() =>
          tap('smart-document.image-actions.${ref.blockId}');
      Future<void> history({bool redo = false}) async {
        await focus(first.editor, offset: 0);
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        if (redo) await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
        if (redo) await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      }

      expect(attrs(c)['url'], original['url']);
      expect(attrs(c).containsKey('asset'), isFalse);
      expect(attrs(c).containsKey('width'), isFalse);
      expect(
        c.snapshot['data'].toString(),
        contains('https://example.invalid/cohort'),
      );
      expect(c.snapshot['data'].toString(), contains('const original = 1;'));
      await bitmap(first, original['sha256']);
      checks.add(
        'controlled-original-Row-document-opens-in-Host-with-original-image-link-code-and-no-fabricated-asset',
      );
      await shot('01-original-row-document-network-image');
      await properties();
      await showActor(1);
      final peer = await openParent();
      expect(peer.controller.objectId, c.objectId);
      expect(peer.controller.canWrite, isTrue);
      await bitmap(peer, original['sha256']);
      checks.add(
        'second-authenticated-member-reads-same-original-row-document-and-image',
      );
      await properties();
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-caption')),
        '同伴原图说明😀',
      );
      await tap('smart-document.image-apply');
      await until(() => attrs(c)['title'] == '同伴原图说明😀', 'live caption');
      await showActor(0);
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-width')),
        '360',
      );
      await tap('smart-document.image-apply');
      await until(
        () =>
            attrs(c)['width'] == 360 && attrs(peer.controller)['width'] == 360,
        'live original image width',
      );
      expect(attrs(c)['title'], '同伴原图说明😀');
      checks.add(
        'original-URL-image-settings-resize-missing-dimensions-and-preserve-concurrent-peer-caption',
      );
      await shot('02-original-image-metadata-and-size');
      choices['alice'] = () async => doc.PickedDocumentImage(
        await File(original['replacementPath']).readAsBytes(),
        'replacement.png',
      );
      await properties();
      await tap('smart-document.image-replace');
      await until(
        () => attrs(c)['asset'] != null && !first.media!.busy,
        'real private asset replacement',
      );
      final managed = (attrs(c)['asset'] as Map)['sha256'] as String;
      expect(managed, original['replacementSha256']);
      expect(attrs(c)['width'], 360);
      expect(attrs(c)['height'], 540);
      expect(attrs(c)['title'], '同伴原图说明😀');
      expect(c.blocks.where((b) => b.type == 'image'), hasLength(1));
      await until(
        () => attrs(peer.controller)['asset'] != null,
        'peer receives replacement',
      );
      checks.add(
        'replace-original-URL-with-real-private-asset-retains-block-caption-size-and-reaches-peer',
      );
      await bitmap(first, managed);
      await shot('03-original-image-replaced');
      await history();
      await until(
        () =>
            attrs(c)['asset'] == null &&
            attrs(peer.controller)['asset'] == null,
        'own keyboard undo original source',
      );
      expect(attrs(c)['url'], original['url']);
      expect(attrs(c)['title'], '同伴原图说明😀');
      checks.add(
        'native-keyboard-undo-restores-original-URL-and-preserves-peer-caption',
      );
      await history(redo: true);
      await until(
        () =>
            attrs(c)['asset'] != null &&
            attrs(peer.controller)['asset'] != null,
        'native redo',
      );
      checks.add('native-keyboard-redo-restores-accepted-managed-replacement');
      await showActor(2);
      final guest = await openParent();
      expect(guest.controller.canWrite, isFalse);
      expect(guest.controller.canRead, isTrue);
      await bitmap(guest, managed);
      expect(
        find.byKey(ValueKey('smart-document.image-actions.${ref.blockId}')),
        findsNothing,
      );
      checks.add(
        'actual-inherited-readonly-Guest-sees-private-image-and-has-no-edit-actions',
      );
      await shot('04-readonly-Guest-original-row-document');
      await domain(
        () => actors[0].cloud.service.revokeViewAccess(
          wid,
          db['parentViewId'],
          [users['outsider']['email']],
        ),
      );
      await until(
        () => !guest.controller.canRead,
        'actual inherited Guest revocation',
      );
      expect(
        find.descendant(
          of: surfaceFor(guest.editor),
          matching: find.byType(Image),
        ),
        findsNothing,
      );
      checks.add(
        'actual-source-revocation-hides-Guest-document-and-clears-image',
      );
      await shot('05-revoked-Guest-original-row-document');
      expect(c.canWrite, isTrue);
      expect(peer.controller.canWrite, isTrue);
      checks.add(
        'Guest-source-revocation-retains-authorized-members-editability',
      );
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
          'databaseRowBodyEntryUi': false,
          'singleOwnerConnection': true,
          'fullProductAcceptance': 'not-complete',
          'limits': [
            'Original row document was created through the original API and opened by its original resource route; database record body entry/creation UI, full protocol admission, durable delivery, SiYuan parity and independent design acceptance remain open',
          ],
        }),
      );
      debugPrint('SMARTDOC_ORIGINAL_IMAGE_CHECKS=${checks.length}');
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
    if (failSave && record['draft'] != null) {
      throw StateError('synthetic initial replacement draft unavailable');
    }
    if (failCleanup && record['draft'] == null) {
      throw StateError('synthetic replacement cleanup unavailable');
    }
    await inner.write(record);
  }
}
