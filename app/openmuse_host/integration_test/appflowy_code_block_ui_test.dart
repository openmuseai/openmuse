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
    'original code blocks reuse shared Document text, URL database and original parent permissions',
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
        'smartdoc-code-block-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_CODE_BLOCK_EVIDENCE=${evidence.path}');
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
      late final String codeId;
      Future<void> until(bool Function() ready, String reason) async {
        debugPrint('CODE_BLOCK_NATIVE begin:$reason');
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (ready()) {
            debugPrint('CODE_BLOCK_NATIVE done:$reason');
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

      doc.DocumentBlockReference codeRef(doc.SmartDocumentController c) =>
          doc.DocumentBlockReference(objectId: c.objectId, blockId: codeId);
      Future<void> codeVisible(doc.SmartDocumentController c) async {
        final position = c.editor.scrollableState?.position;
        if (position != null) position.jumpTo(position.minScrollExtent);
        await until(
          () => find
              .byKey(ValueKey('smart-document.code-copy.$codeId'))
              .evaluate()
              .isNotEmpty,
          'original code header visible',
        );
      }

      Future<void> focusCode(doc.SmartDocumentController c) async {
        await codeVisible(c);
        final rich = find.byWidgetPredicate(
          (w) => w is ae.AppFlowyRichText && w.node.id == codeId,
        );
        await until(
          () => rich.evaluate().isNotEmpty,
          'stable code text projection',
        );
        await t.ensureVisible(rich);
        await until(
          () => rich.hitTestable().evaluate().length == 1,
          'code text receives native pointer',
        );
        await t.tap(rich);
        await t.pump(const Duration(milliseconds: 100));
        final node = c.resolveBlock(codeRef(c))!;
        c.editor.selection = ae.Selection.collapsed(
          ae.Position(path: node.path, offset: node.delta!.length),
        );
        await t.pump(const Duration(milliseconds: 150));
      }

      Future<void> pasteCode(
        doc.SmartDocumentController c,
        String value,
      ) async {
        await focusCode(c);
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

      Future<void> copyUrl(String value) async {
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await tap('dataset.url-copy');
          expect((await Clipboard.getData(Clipboard.kTextPlain))?.text, value);
        } finally {
          await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
        }
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
      // Prepare one original paragraph as fixture content; conversion and all
      // language/copy/history/permission actions below use the formal Host UI.
      final codeParagraph = ae.paragraphNode(
        text: 'fn main() { print("智能😀"); }',
      );
      await domain(
        () => parent.editor.apply(
          parent.editor.transaction
            ..insertNode([1], codeParagraph, deepCopy: false),
        ),
      );
      codeId = parent.blocks[1].id;
      final paragraph = find.byWidgetPredicate(
        (w) => w is ae.AppFlowyRichText && w.node.id == codeId,
      );
      await until(
        () => paragraph.evaluate().isNotEmpty,
        'prepared original paragraph projection',
      );
      await t.ensureVisible(paragraph);
      await until(
        () => paragraph.hitTestable().evaluate().length == 1,
        'prepared paragraph receives pointer',
      );
      await t.tap(paragraph);
      parent.editor.selection = ae.Selection.collapsed(
        ae.Position(
          path: parent.blocks[1].path,
          offset: parent.blocks[1].delta!.length,
        ),
      );
      await t.pump();
      await tap('smart-document.block-type');
      final codeType = find.ancestor(
        of: find.text('代码块'),
        matching: find.byType(PopupMenuItem<String>),
      );
      await until(
        () => codeType.hitTestable().evaluate().length == 1,
        'formal code type action',
      );
      await t.tap(codeType);
      await until(
        () => parent.resolveBlock(codeRef(parent))?.type == 'code',
        'original stable code conversion',
      );
      await codeVisible(parent);
      await domain(parent.synchronize);
      checks.add(
        'formal-original-paragraph-to-code-conversion-keeps-block-text-and-private-Cloud-document-owner',
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

      await codeVisible(guestDoc);
      final codeReadSize = t.view.physicalSize;
      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      await t.pump(const Duration(milliseconds: 350));
      try {
        expect(
          t
              .renderObject<RenderRepaintBoundary>(
                find.byKey(const ValueKey('database-host-1')),
              )
              .size
              .width,
          430,
        );
        expect(
          t
              .widget<TextButton>(
                find.byKey(ValueKey('smart-document.code-language.$codeId')),
              )
              .onPressed,
          isNull,
        );
        final docBefore = guestDoc.snapshot['docState'];
        final epochBefore = guest.localMutationEpoch;
        final previous = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await tap('smart-document.code-copy.$codeId');
          expect(
            (await Clipboard.getData(Clipboard.kTextPlain))?.text,
            guestDoc.resolveBlock(codeRef(guestDoc))!.delta!.toPlainText(),
          );
        } finally {
          await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
        }
        for (final key in [
          'smart-document.code-copy.$codeId',
          'smart-document.code-language.$codeId',
        ]) {
          expect(
            t.getSize(find.byKey(ValueKey(key))).height,
            greaterThanOrEqualTo(44),
          );
        }
        expect(guestDoc.snapshot['docState'], docBefore);
        expect(guest.localMutationEpoch, epochBefore);
        await shot('09-readonly-original-code-controls');
        checks.add(
          'native430-readonly-recipient-copies-original-code-with-visible44px-controls-without-Document-Root-Row-mutation',
        );
      } finally {
        t.view.physicalSize = codeReadSize;
        await t.pump(const Duration(milliseconds: 350));
      }
      final linkedTexts = find.byWidgetPredicate(
        (w) => w is RichText && w.text.toPlainText() == '打开共享数据库😀',
      );
      Future<void> ensureOriginalHref() async {
        if (linkedTexts.evaluate().isEmpty) {
          final position = guestDoc.editor.scrollableState?.position;
          if (position != null) position.jumpTo(position.minScrollExtent);
          await until(
            () => linkedTexts.evaluate().isNotEmpty,
            'original href after narrow code viewport',
          );
        }
        await t.ensureVisible(linkedTexts.first);
        await until(
          () => linkedTexts.hitTestable().evaluate().length == 1,
          'original href receives pointer',
        );
      }

      await ensureOriginalHref();
      final linkedText = linkedTexts.first;
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
      await ensureOriginalHref();
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
      await domain(guestDoc.synchronize);
      await until(
        () => guestDoc.canWrite,
        'original parent share upgrade reaches code document',
      );
      await codeVisible(guestDoc);
      await tap('smart-document.code-language.$codeId');
      await tap('smart-document.code-language-search');
      await t.enterText(
        find.byKey(const ValueKey('smart-document.code-language-search')),
        'python',
      );
      await tap('smart-document.code-language-option.python');
      await until(
        () =>
            find
                .byKey(const ValueKey('smart-document.code-language-dialog'))
                .evaluate()
                .isEmpty &&
            parent.captureCodeLanguage(codeRef(parent)).language == 'python',
        'original code language crosses authenticated Cloud',
      );
      expect(guestDoc.resolveBlock(codeRef(guestDoc))!.id, codeId);
      checks.add(
        'original-parent-share-upgrade-enables-formal-code-language-change-through-original-Document0-collaboration',
      );
      await showActor(0);
      await pasteCode(parent, ' //真实同伴😀');
      await until(
        () => guestDoc
            .resolveBlock(codeRef(guestDoc))!
            .delta!
            .toPlainText()
            .contains('真实同伴😀'),
        'code peer text reaches original Y.Text',
      );
      final codePeerText = parent
          .resolveBlock(codeRef(parent))!
          .delta!
          .toPlainText();
      await showActor(1);
      await focusCode(guestDoc);
      await tap('smart-document.undo');
      await until(
        () =>
            guestDoc.captureCodeLanguage(codeRef(guestDoc)).language == null &&
            parent.captureCodeLanguage(codeRef(parent)).language == null,
        'own code language undo crosses original Cloud',
      );
      expect(
        guestDoc.resolveBlock(codeRef(guestDoc))!.delta!.toPlainText(),
        codePeerText,
      );
      expect(
        parent.resolveBlock(codeRef(parent))!.delta!.toPlainText(),
        codePeerText,
      );
      checks.add(
        'formal-own-code-language-undo-preserves-peer-original-Y.Text-and-stable-code-block',
      );
      await shot('10-code-peer-text-after-own-language-undo');
      final staleCodeCopy = t
          .widget<IconButton>(
            find.byKey(ValueKey('smart-document.code-copy.$codeId')),
          )
          .onPressed!;
      final staleCodeLanguage = t
          .widget<TextButton>(
            find.byKey(ValueKey('smart-document.code-language.$codeId')),
          )
          .onPressed!;

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
        staleCodeCopy();
        staleCodeLanguage();
        await expectLater(
          guestDoc.copyCode(codeRef(guestDoc)),
          throwsA(isA<doc.SmartDocumentFailure>()),
        );
        await t.pump(const Duration(milliseconds: 200));
        expect(
          (await Clipboard.getData(Clipboard.kTextPlain))?.text,
          'synthetic revoked URL sentinel',
        );
        expect(
          find.byKey(const ValueKey('smart-document.code-language-dialog')),
          findsNothing,
        );
        checks.add(
          'real-original-parent-sharing-revocation-fences-stale-code-copy-language-and-public-copy-before-dispatch',
        );
        await shot('11-revoked-code-and-database-parent');

        await shot('06-recipient-revoked-cloud-reference');
      } finally {
        await Clipboard.setData(
          ClipboardData(text: previousClipboard?.text ?? ''),
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
      debugPrint('SMARTDOC_CODE_BLOCK_CHECKS=${checks.length}');
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
