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
    'dedicated mobile images and link dialogs follow original private assets, owners and account lifetimes',
    (t) async {
      HttpOverrides.global = null;
      addTearDown(t.view.resetPhysicalSize);
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final nativePhysical = t.view.physicalSize;
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      expect(fixture['origin'], 'http://127.0.0.1:18001');
      final db = fixture['database'] as Map, users = fixture['users'] as Map;
      final source = fixture['networkImages'] as Map;
      final sourceOrigin = source['origin'] as String;
      var filePickerCalls = 0;
      final wid = fixture['workspaceId'] as String;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-image-link-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_IMAGE_LINK_EVIDENCE=${evidence.path}');
      final choices = <String, Future<doc.PickedDocumentImage?> Function()>{};
      final faults = <String, _CleanupFailureStore>{};
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
        choices[name] = () async => null;
        final plugins = createNativeSmartDocumentPlugins(
          cloudDocuments: online,
          imagePicker: () {
            filePickerCalls++;
            return choices[name]!();
          },
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
        debugPrint('NETWORK_NATIVE begin:$reason');
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 80));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
          if (ready()) {
            debugPrint('NETWORK_NATIVE done:$reason');
            return;
          }
        }
        fail(
          '$reason: ${t.widgetList<Text>(find.byType(Text)).map((v) => v.data).join(' | ')}',
        );
      }

      var domainSequence = 0;
      Future<T> domain<T>(Future<T> Function() action) async {
        final sequence = ++domainSequence;
        debugPrint('NETWORK_NATIVE domain-begin:$sequence');
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
        debugPrint('NETWORK_NATIVE domain-done:$sequence');
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
        debugPrint('NETWORK_NATIVE actor:$index');
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
        final paragraphs = find.descendant(
          of: surfaceFor(editor),
          matching: find.byType(ae.AppFlowyRichText),
        );
        if (paragraphs.evaluate().isEmpty) {
          final position = editor.scrollableState?.position;
          if (position != null) position.jumpTo(position.minScrollExtent);
          await until(
            () => paragraphs.evaluate().isNotEmpty,
            'original virtualized paragraph after viewport change',
          );
        }
        final f = paragraphs.first;
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
        debugPrint('NETWORK_NATIVE shot-begin:$name');
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
        debugPrint('NETWORK_NATIVE shot-done:$name');
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
      await tap('pane-menu-button:editor');
      await t.tap(find.text('向右切分'));
      await t.pump(const Duration(milliseconds: 300));
      await tap('pane-menu-button:pane-1');
      await t.tap(find.text('绑定新编辑组'));
      await t.pump(const Duration(milliseconds: 300));
      final group = alice.workspace.editorGroups.singleWhere(
        (g) => g.id != LocalWorkspaceController.primaryEditorGroupId,
      );
      alice.workspace.focusEditorGroup(group.id);
      final context = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(context, alice.center.controller, record.reference()),
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 2,
        'two actual Host document presentations',
      );
      var second = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.presentation != null);
      var p = second.presentation!;
      await until(() => p.canWrite, 'secondary projection ready');
      expect(second.controller, same(c));
      expect(second.realtime, same(rt));
      expect(second.editor, isNot(same(first.editor)));
      expect(c.localPresentationEditors, hasLength(2));
      expect(rt.connection.clientId, clientId);
      checks.add(
        'two-host-editor-groups-share-one-original-document-handle-checkpoint-and-realtime-actor',
      );
      Future<doc.PickedDocumentImage> picture(
        String name,
        int width,
        int height,
        Color color,
      ) async {
        final recorder = ui.PictureRecorder();
        final canvas = Canvas(recorder);
        canvas.drawRect(
          Rect.fromLTWH(0, 0, width.toDouble(), height.toDouble()),
          Paint()..color = color,
        );
        final drawing = recorder.endRecording();
        final rendered = await drawing.toImage(width, height);
        drawing.dispose();
        final bytes = (await rendered.toByteData(
          format: ui.ImageByteFormat.png,
        ))!.buffer.asUint8List();
        rendered.dispose();
        return doc.PickedDocumentImage(bytes, name);
      }

      final blue = doc.PickedDocumentImage(
            await File(source['images']['blue']['path']).readAsBytes(),
            'network-image',
          ),
          green = doc.PickedDocumentImage(
            await File(source['images']['green']['path']).readAsBytes(),
            'network-image',
          ),
          red = await picture('normal.png', 100, 50, Colors.red),
          yellow = await picture('peer.png', 90, 60, Colors.amber);
      doc.DocumentBlockReference? imageRef;
      Map<String, dynamic> imageData(doc.SmartDocumentController owner) =>
          owner.resolveBlock(imageRef!)!.attributes;
      String imageHash(doc.SmartDocumentController owner) =>
          (imageData(owner)['asset'] as Map)['sha256'] as String;
      Finder inView(doc.SmartDocumentView view, String key) => find.descendant(
        of: surfaceFor(view.editor),
        matching: find.byKey(ValueKey(key)),
      );
      Future<void> tapView(doc.SmartDocumentView view, String key) async {
        final button = inView(view, key).first;
        await t.ensureVisible(button);
        await until(
          () => button.hitTestable().evaluate().length == 1,
          '$key is visible in its own presentation',
        );
        await t.tap(button);
        await t.pump(const Duration(milliseconds: 300));
      }

      Future<void> properties(doc.SmartDocumentView view) =>
          tapView(view, 'smart-document.image-actions.${imageRef!.blockId}');
      Future<void> dismissProperties() async {
        final cancel = find.descendant(
          of: find.byKey(const ValueKey('smart-document.image-dialog')),
          matching: find.text('取消'),
        );
        await t.tap(cancel);
        await until(
          () => find
              .byKey(const ValueKey('smart-document.image-dialog'))
              .evaluate()
              .isEmpty,
          'image settings close',
        );
      }

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
        'authenticated private image bytes reach the original image renderer',
      );
      Future<void> keyboardHistory(
        doc.SmartDocumentView view, {
        bool redo = false,
      }) async {
        await focus(view.editor);
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        if (redo) await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
        if (redo) await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      }

      Future<void> networkInsert() async {
        t.view.physicalSize = Size(
          430 * t.view.devicePixelRatio,
          900 * t.view.devicePixelRatio,
        );
        await t.pump(const Duration(milliseconds: 400));
        expect(
          t
              .renderObject<RenderRepaintBoundary>(
                find.byKey(const ValueKey('presentation-host-0')),
              )
              .size
              .width,
          430,
        );
        if (!p.canWrite) {
          final edit = find.descendant(
            of: surfaceFor(second.editor),
            matching: find.text('编辑'),
          );
          await t.ensureVisible(edit);
          await t.tap(edit);
          await until(
            () => p.canWrite,
            'explicit mobile editing before dedicated image source',
          );
        }
        await tapView(second, 'smart-document.mobile.image');
        final entry = find.text('网络图片');
        await t.ensureVisible(entry);
        await until(
          () => entry.hitTestable().evaluate().length == 1,
          'dedicated mobile Image source entry',
        );
        expect(find.text('选择图片'), findsOneWidget);
        await t.tap(entry);
        await until(
          () =>
              find
                  .byKey(const ValueKey('smart-document.network-image-url'))
                  .evaluate()
                  .length ==
              1,
          'network URL dialog',
        );
      }

      Future<void> fillSource(String path) async {
        for (final key in [
          'smart-document.network-image-import',
          'smart-document.network-image-cancel',
        ]) {
          expect(
            t.getSize(find.byKey(ValueKey(key))).height,
            greaterThanOrEqualTo(44),
          );
        }
        final field = find.byKey(
          const ValueKey('smart-document.network-image-url'),
        );
        await t.ensureVisible(field);
        await t.tap(field);
        await t.pump(const Duration(milliseconds: 200));
        final value = '$sourceOrigin$path?synthetic=network-image-proof';
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await Clipboard.setData(ClipboardData(text: value));
          await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
          await t.sendKeyEvent(LogicalKeyboardKey.keyA);
          await t.sendKeyEvent(LogicalKeyboardKey.keyV);
          await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
          await t.pump(const Duration(milliseconds: 200));
          expect(
            t.widget<TextField>(field).controller!.text == value,
            isTrue,
            reason:
                'Native URL field contains the explicitly pasted synthetic source',
          );
        } finally {
          await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
        }
      }

      Future<void> enterSource(String path) async {
        await fillSource(path);
        await tap('smart-document.network-image-import');
      }

      await networkInsert();
      await enterSource('/slow.png');
      await until(
        () =>
            (jsonDecode(
                      File(source['requestsPath']).readAsStringSync(),
                    )['requests']
                    as List)
                .any((r) => r['path'] == '/slow.png'),
        'owned HTTP source received the pending read',
      );
      await tap('smart-document.network-image-cancel');
      await until(
        () => !second.media!.busy,
        'cancelled source returns through the original picker fence',
      );
      expect(c.blocks.where((b) => b.type == 'image'), isEmpty);
      expect(second.media!.hasPending, isFalse);
      checks.add(
        'formal-url-cancel-aborts-pending-http-read-without-private-upload-or-document-image',
      );
      await networkInsert();
      await enterSource('/invalid.png');
      await until(
        () => find.text('无法读取此图片，请检查链接、格式和大小').evaluate().isNotEmpty,
        'real decoder rejects forged PNG content',
      );
      expect(
        t
            .widget<TextField>(
              find.byKey(const ValueKey('smart-document.network-image-url')),
            )
            .controller!
            .text,
        '$sourceOrigin/invalid.png?synthetic=network-image-proof',
      );
      expect(c.blocks.where((b) => b.type == 'image'), isEmpty);
      expect(second.media!.hasPending, isFalse);
      await shot('00-invalid-image-retains-ui-url-before-explicit-retry');
      checks.add(
        'forged-png-source-fails-before-private-upload-and-retains-url-for-explicit-retry',
      );
      await enterSource('/redirect');
      await until(
        () =>
            c.blocks.where((b) => b.type == 'image').length == 1 &&
            p.projectionCurrent &&
            !second.media!.busy,
        'formal image insertion',
      );
      t.view.physicalSize = nativePhysical;
      await t.pump(const Duration(milliseconds: 400));
      expect(second.controller, same(c));
      expect(second.realtime, same(rt));
      checks.add(
        'native-430-dedicated-mobile-image-panel-reuses-original-file-and-network-source-workflow-and-owner',
      );
      imageRef = doc.DocumentBlockReference(
        objectId: c.objectId,
        blockId: c.blocks.singleWhere((b) => b.type == 'image').id,
      );
      final hashA = imageHash(c);
      expect(filePickerCalls, 0);
      expect(hashA, source['images']['blue']['sha256']);
      checks.add(
        'formal-network-insert-follows-bounded-redirect-and-enters-original-private-asset-workflow-without-file-picker',
      );
      expect(hashA, resourceDigest(blue.bytes));
      expect(first.media, same(second.media));
      await properties(second);
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-caption')),
        '原说明',
      );
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-alt')),
        '原替代文本',
      );
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-width')),
        '128',
      );
      await tap('smart-document.image-apply');
      await until(
        () => find
            .byKey(const ValueKey('smart-document.image-dialog'))
            .evaluate()
            .isEmpty,
        'initial image properties applied',
      );
      await properties(second);
      await showActor(1);
      final imagePeer = await openParent();
      await until(
        () => imagePeer.controller.blocks.any((b) => b.id == imageRef!.blockId),
        'original peer receives the inserted image',
      );
      await bitmap(imagePeer, hashA);
      expect(
        await domain(
          () => imagePeer.media!.repository.read(
            doc.AssetReference.fromJson(
              (imageData(imagePeer.controller)['asset'] as Map)
                  .cast<String, dynamic>(),
            ),
          ),
        ),
        orderedEquals(blue.bytes),
      );
      checks.add(
        'formal-secondary-image-insertion-and-original-private-assets-are-readable-by-the-authorized-peer',
      );
      await properties(imagePeer);
      await t.enterText(
        find.byKey(const ValueKey('smart-document.image-caption')),
        '同伴图片说明😀',
      );
      await tap('smart-document.image-apply');
      await until(
        () => imageData(c)['title'] == '同伴图片说明😀',
        'peer caption reaches the open stale settings dialog',
      );
      await showActor(0);
      expect(
        t
            .widget<TextField>(
              find.byKey(const ValueKey('smart-document.image-caption')),
            )
            .controller!
            .text,
        '原说明',
      );
      faults['alice']!.failCleanup = true;
      await tap('smart-document.image-replace-network');
      await until(
        () =>
            find
                .byKey(const ValueKey('smart-document.network-image-url'))
                .evaluate()
                .length ==
            1,
        'original properties opens the URL dialog',
      );
      await fillSource('/green.png');
      await shot('01-network-replacement-dialog');
      await tap('smart-document.network-image-import');
      await until(
        () =>
            imageHash(c) == resourceDigest(green.bytes) &&
            second.media!.hasPending &&
            !second.media!.busy,
        'replacement accepted before known private cleanup failure',
      );
      expect(filePickerCalls, 0);
      expect(imageHash(c), source['images']['green']['sha256']);
      checks.add(
        'formal-network-replacement-reuses-original-block-and-media-owner-without-file-picker',
      );
      final pending = (await domain(faults['alice']!.read))!;
      for (final state in [c.snapshot, pending]) {
        final encoded = jsonEncode(state);
        expect(encoded.contains(sourceOrigin), isFalse);
        expect(encoded.contains('network-image-proof'), isFalse);
        expect(encoded.contains('/redirect'), isFalse);
      }
      checks.add(
        'network-source-origin-path-and-query-are-absent-from-original-document-and-private-replacement-draft',
      );
      final intent = ((pending['draft'] as Map)['replacementIntent'] as Map)
          .cast<String, dynamic>();
      expect(imageData(c)['title'], '同伴图片说明😀');
      expect(imageData(c)['alt'], '原替代文本');
      expect(c.blocks.where((b) => b.type == 'image'), hasLength(1));
      expect(imageData(c)['width'], 128);
      expect(imageData(c)['height'], 192);
      await shot('01-replacement-cleanup-failure-retains-private-selection');
      await dismissProperties();
      checks.add(
        'replacement-through-stale-image-settings-keeps-the-original-block-and-peer-caption-with-private-cleanup-failure-retained',
      );
      await showActor(1);
      await until(
        () => imageHash(imagePeer.controller) == resourceDigest(green.bytes),
        'peer receives the replacement',
      );
      await bitmap(imagePeer, resourceDigest(green.bytes));
      expect(
        await domain(
          () => imagePeer.media!.repository.read(
            doc.AssetReference.fromJson(
              (imageData(imagePeer.controller)['asset'] as Map)
                  .cast<String, dynamic>(),
            ),
          ),
        ),
        orderedEquals(green.bytes),
      );
      await showActor(0);
      await keyboardHistory(second);
      await until(
        () =>
            imageHash(c) == hashA &&
            imageHash(imagePeer.controller) == hashA &&
            p.projectionCurrent,
        'native own image undo preserves the peer caption',
      );
      expect(imageData(c)['title'], '同伴图片说明😀');
      faults['alice']!.failCleanup = false;
      await tapView(second, 'smart-document.image-retry');
      await until(
        () => !second.media!.hasPending && !second.media!.busy,
        'explicit replacement recovery retry',
      );
      expect(imageHash(c), hashA);
      expect(imageData(c)['title'], '同伴图片说明😀');
      checks.add(
        'native-image-undo-and-explicit-cleanup-retry-never-reapply-the-accepted-replacement',
      );
      final probe = doc.NativeDocumentCore();
      final isolated = probe.call({
        'action': 'open',
        'objectId': imagePeer.controller.objectId,
        'docState': imagePeer.controller.snapshot['docState'],
      });
      try {
        final replay = probe.call({
          'action': 'operations',
          'handle': isolated['handle'],
          'operations': [
            {'op': 'replace_image', 'blockId': imageRef.blockId, ...intent},
          ],
        });
        expect(replay['docState'], isolated['docState']);
      } finally {
        probe.call({'action': 'close', 'handle': isolated['handle']});
      }
      checks.add(
        'replacement-acceptance-receipt-crosses-the-original-cloud-update-and-peer-replay-after-undo-is-a-noop',
      );
      choices['alice'] = () async => red;
      await properties(second);
      await tap('smart-document.image-replace');
      await until(
        () =>
            find
                .byKey(const ValueKey('smart-document.image-dialog'))
                .evaluate()
                .isEmpty &&
            imageHash(c) == resourceDigest(red.bytes),
        'successful normal replacement closes settings',
      );
      await keyboardHistory(second);
      await until(() => imageHash(c) == hashA, 'native replacement undo');
      await keyboardHistory(second, redo: true);
      await until(
        () =>
            imageHash(c) == resourceDigest(red.bytes) &&
            imageHash(imagePeer.controller) == resourceDigest(red.bytes),
        'native replacement redo',
      );
      expect(imageData(c)['title'], '同伴图片说明😀');
      await bitmap(second, resourceDigest(red.bytes));
      checks.add(
        'successful-replacement-native-undo-redo-retains-block-and-peer-description',
      );
      final choosing = Completer<doc.PickedDocumentImage?>();
      choices['alice'] = () => choosing.future;
      await properties(second);
      await tap('smart-document.image-replace');
      await until(() => second.media!.busy, 'picker selection remains pending');
      await showActor(1);
      choices['bob'] = () async => yellow;
      await properties(imagePeer);
      await tap('smart-document.image-replace');
      await until(
        () =>
            imageHash(c) == resourceDigest(yellow.bytes) &&
            imageHash(imagePeer.controller) == resourceDigest(yellow.bytes),
        'peer changes the source during the local selection',
      );
      choosing.complete(green);
      await showActor(0);
      await until(
        () => second.media!.hasPending && !second.media!.busy,
        'known source change preserves private choice',
      );
      expect(imageHash(c), resourceDigest(yellow.bytes));
      expect(find.text('图片源已变化，待处理图片保留；可取消后重新选择。'), findsWidgets);
      await shot('02-peer-source-conflict-is-explicit-and-retained');
      await dismissProperties();
      await tapView(second, 'smart-document.image-cancel');
      await until(
        () => !second.media!.hasPending,
        'explicit cancellation clears only its private selection',
      );
      expect(imageHash(c), resourceDigest(yellow.bytes));
      expect(imageData(c)['title'], '同伴图片说明😀');
      checks.add(
        'peer-source-change-during-selection-rejects-stale-replacement-and-explicit-private-cancel-preserves-peer-image',
      );
      await shot('03-original-image-source-and-peer-metadata-preserved');
      final originalPhysical = t.view.physicalSize;
      t.view.physicalSize = Size(
        320 * t.view.devicePixelRatio,
        420 * t.view.devicePixelRatio,
      );
      await t.pump(const Duration(milliseconds: 400));
      expect(
        t
            .renderObject<RenderRepaintBoundary>(
              find.byKey(const ValueKey('presentation-host-0')),
            )
            .size
            .width,
        320,
      );
      expect(p.canWrite, isFalse);
      final edit = find.descendant(
        of: surfaceFor(second.editor),
        matching: find.text('编辑'),
      );
      await t.ensureVisible(edit);
      await t.tap(edit);
      await until(
        () => p.canWrite,
        'explicit mobile edit mode admits replacement',
      );
      await properties(second);
      await tap('smart-document.image-replace-network');
      await until(
        () =>
            find
                .byKey(const ValueKey('smart-document.network-image-url'))
                .evaluate()
                .length ==
            1,
        'narrow network replacement URL dialog',
      );
      await fillSource('/blue.png');
      await shot('06-narrow-network-replacement-dialog');
      await tap('smart-document.network-image-import');
      await until(
        () =>
            imageHash(c) == hashA &&
            imageHash(imagePeer.controller) == hashA &&
            !second.media!.busy,
        'narrow original private replacement and peer readback',
      );
      t.view.physicalSize = originalPhysical;
      await t.pump(const Duration(milliseconds: 400));
      expect(second.controller, same(c));
      expect(second.realtime, same(rt));
      await keyboardHistory(second);
      await until(
        () =>
            imageHash(c) == resourceDigest(yellow.bytes) &&
            imageHash(imagePeer.controller) == resourceDigest(yellow.bytes),
        'narrow replacement own undo preserves previous peer image',
      );
      expect(imageData(c)['title'], '同伴图片说明😀');
      checks.add(
        'native-320x420-url-replacement-requires-explicit-mobile-edit-and-keeps-original-owner-peer-image-and-own-undo',
      );
      final pendingTab = alice.workspace.activeTabFor(group.id)!;
      await until(
        () =>
            !firstTab.resourceSession!.session.dirty &&
            !pendingTab.resourceSession!.session.dirty,
        'original document checkpoints are clean before private input failure',
      );
      faults['alice']!.failSave = true;
      choices['alice'] = () async => green;
      await properties(second);
      await tap('smart-document.image-replace');
      await until(
        () => second.media!.hasPending && !second.media!.busy,
        'private selection remains in memory after failed initial persistence',
      );
      expect(imageHash(c), resourceDigest(yellow.bytes));
      expect(pendingTab.resourceSession!.session.dirty, isFalse);
      await dismissProperties();
      final beforeFailedClose = t.state(surfaceFor(second.editor));
      alice.workspace.closeTab(pendingTab, groupId: group.id);
      await until(
        () =>
            find
                .byKey(const ValueKey('resource-keep-editing'))
                .hitTestable()
                .evaluate()
                .length ==
            1,
        'clean resource close refuses an unsaved private image choice',
      );
      await shot('04-private-image-save-blocks-clean-resource-close');
      await tap('resource-keep-editing');
      await until(
        () => !pendingTab.resourceSession!.confirming,
        'failed private close remains editable',
      );
      expect(t.state(surfaceFor(second.editor)), same(beforeFailedClose));
      expect(c.isDisposed, isFalse);
      faults['alice']!.failSave = false;
      alice.workspace.closeTab(pendingTab, groupId: group.id);
      await until(
        () =>
            !alice.workspace.tabsFor(group.id).contains(pendingTab) &&
            c.localPresentationEditors.length == 1,
        'fixed private store permits close while keeping the outside owner',
      );
      expect(c.isDisposed, isFalse);
      alice.workspace.focusEditorGroup(group.id);
      unawaited(
        openResourcePage(
          t.element(find.byKey(const ValueKey('desktop-resource-center'))),
          alice.center.controller,
          record.reference(),
        ),
      );
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().length == 2,
        'reopen retains the same pending choice and original owner',
      );
      second = t
          .widgetList<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .singleWhere((v) => v.presentation != null);
      p = second.presentation!;
      expect(second.controller, same(c));
      expect(second.realtime, same(rt));
      expect(second.media!.hasPending, isTrue);
      expect((await domain(faults['alice']!.read))!['draft'], isNotNull);
      await tapView(second, 'smart-document.image-cancel');
      await until(
        () => !second.media!.hasPending,
        'explicitly cancel the recovered local image choice',
      );
      expect(imageHash(c), resourceDigest(yellow.bytes));
      checks.add(
        'clean-resource-close-blocks-unsaved-private-image-and-reopen-retains-the-choice-with-the-original-outside-owner',
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
      await shot('04-ended-owner-account');
      await showActor(1);
      await paste(peer.editor, '同伴继续编辑😀');
      await until(
        () => text(other).contains('同伴继续编辑'),
        'other account continues',
      );
      await shot('05-peer-remains-editable');
      await focus(peer.editor, offset: 0);
      peer.editor.selection = ae.Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tapView(peer, 'smart-document.link');
      const link = 'https://example.test/native-link';
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        link,
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(other.blocks.first.delta!.toJson()).contains(link),
        'original href link command commits through the original owner',
      );
      await focus(peer.editor, offset: 0);
      peer.editor.selection = ae.Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tapView(peer, 'smart-document.link');
      t.view.physicalSize = Size(
        320 * t.view.devicePixelRatio,
        420 * t.view.devicePixelRatio,
      );
      await t.pump(const Duration(milliseconds: 400));
      expect(
        t
            .renderObject<RenderRepaintBoundary>(
              find.byKey(const ValueKey('presentation-host-1')),
            )
            .size
            .width,
        320,
      );
      await shot('07-narrow-original-link-dialog');
      final previousClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await tap('smart-document.link-copy');
        expect((await Clipboard.getData(Clipboard.kTextPlain))?.text, link);
        for (final key in [
          'smart-document.link-copy',
          'smart-document.link-open',
          'smart-document.link-cancel',
          'smart-document.link-apply',
        ]) {
          expect(
            t.getSize(find.byKey(ValueKey(key))).height,
            greaterThanOrEqualTo(44),
          );
        }
        checks.add(
          'native-320-original-href-dialog-keeps44px-actions-and-system-clipboard-copy-without-opening-an-external-site',
        );
        final address = t
            .widget<TextField>(
              find.byKey(const ValueKey('smart-document.link-address')),
            )
            .controller!;
        final label = t
            .widget<TextField>(
              find.byKey(const ValueKey('smart-document.link-label')),
            )
            .controller!;
        final oldCopy = t
            .widget<TextButton>(
              find.byKey(const ValueKey('smart-document.link-copy')),
            )
            .onPressed!;
        final oldOpen = t
            .widget<TextButton>(
              find.byKey(const ValueKey('smart-document.link-open')),
            )
            .onPressed!;
        await Clipboard.setData(
          const ClipboardData(text: 'synthetic stale-callback sentinel'),
        );
        await domain(actors[1].auth.signOut);
        await until(
          () => !other.canRead && find.text('无法访问此文档').evaluate().isNotEmpty,
          'real account end retires the open link dialog',
        );
        expect(address.text, isEmpty);
        expect(label.text, isEmpty);
        oldCopy();
        oldOpen();
        var dispatched = 0;
        await expectLater(
          doc.openDocumentLink(
            other,
            link,
            opener: (_) async {
              dispatched++;
              return true;
            },
          ),
          throwsA(isA<doc.SmartDocumentFailure>()),
        );
        await t.pump(const Duration(milliseconds: 200));
        expect(dispatched, 0);
        expect(
          (await Clipboard.getData(Clipboard.kTextPlain))?.text,
          'synthetic stale-callback sentinel',
        );
        await shot('08-account-ended-link-draft-cleared');
        checks.add(
          'real-auth-signout-clears-link-fields-and-blocks-captured-copy-open-and-public-opener-before-dispatch',
        );
        await tap('smart-document.link-cancel');
      } finally {
        await Clipboard.setData(
          ClipboardData(text: previousClipboard?.text ?? ''),
        );
        t.view.physicalSize = nativePhysical;
        await t.pump(const Duration(milliseconds: 400));
      }
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
            'Dedicated mobile Image sources, actual controlled HTTP/original private Cloud assets, original href link dialog and real account signout. Native rendered320/430 frames are not OS-window resizing. Widget clipboard uses a controlled platform double; this Native test uses/restores the real clipboard. Web/iOS, OS picker, durable delivery, complete SiYuan parity and independent design acceptance remain open',
          ],
        }),
      );
      debugPrint('SMARTDOC_IMAGE_LINK_CHECKS=${checks.length}');
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
