import 'package:openmuse_dataset/src/record_ui.dart';
import 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart';
import 'dart:async';
import 'package:http/http.dart' as http;
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
    'original full-sync admission keeps local edits after an actual server quota rejection',
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
        'smartdoc-full-sync-admission-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_FULL_SYNC_ADMISSION_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      final quotaDoc = await File(fixture['oversizeZstdPath']).readAsBytes();
      final quotaVector = await File(
        fixture['stateVectorZstdPath'],
      ).readAsBytes();
      final admissions = <_AdmissionControl>[];
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
        final admission = _AdmissionControl(quotaDoc, quotaVector);
        admissions.add(admission);
        online = AppFlowyDocumentWorkspace(
          authentication: auth,
          service: cloud.service,
          checkpointRoot: root,
          collaborationHttpClientFactory: () => _AdmissionClient(admission),
        );
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => null,
          ),
        );
        registry.install(cloud);
        final plugins =
            createNativeSmartDocumentPlugins(
                  cloudDocuments: online,
                  datasetImagePicker: () async => PickedImage(
                    await File(fixture['mediaImagePath']).readAsBytes(),
                    'acceptance.png',
                  ),
                )
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
        debugPrint('FULL_SYNC_ADMISSION_NATIVE begin:$reason');
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (ready()) {
            debugPrint('FULL_SYNC_ADMISSION_NATIVE done:$reason');
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
      final scopedGridId =
          t.widget<DatasetSurface>(find.byType(DatasetSurface)).viewId ??
          owner.views.first.id;
      final rowId = owner.views
              .singleWhere((v) => v.id == scopedGridId)
              .rows
              .first
              .id,
          fieldId = owner.views
              .singleWhere((v) => v.id == scopedGridId)
              .columns
              .first
              .id;
      await edit(rowId, fieldId, '同一云端记录😀');
      await tap('dataset.add-field');
      await t.tap(find.text('图片与附件').last);
      await t.pump(const Duration(milliseconds: 350));
      await t.enterText(
        find.byKey(const ValueKey('dataset.field-name')),
        '协作图片',
      );
      await tap('dataset.field-apply');
      await until(
        () => owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .columns
            .any((f) => f.isMedia),
        'original Media14 field',
      );
      final mediaField = owner.views
          .singleWhere((v) => v.id == scopedGridId)
          .columns
          .singleWhere((f) => f.isMedia)
          .id;
      await tap('dataset.cell.$rowId.$mediaField');
      await tap('dataset.media-add-image');
      await until(
        () => owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[mediaField]!
            .mediaFiles
            .isNotEmpty,
        'private image reference enters original media cell',
      );
      final mediaFile = owner
          .readRow(
            owner.views.singleWhere((v) => v.id == scopedGridId).id,
            rowId,
          )
          .cells[mediaField]!
          .mediaFiles
          .single;
      final assetRef = AssetReference.fromUri(Uri.parse(mediaFile.url));
      await until(
        () => find
            .byKey(
              ValueKey(
                'dataset.media-grid-preview.$rowId.$mediaField.${mediaFile.id}',
              ),
            )
            .evaluate()
            .isNotEmpty,
        'private inline grid preview',
      );
      expect(assetRef.owner, rowId);
      expect(assetRef.scope, wid);
      expect(mediaFile.uploadType, 2);
      expect(mediaFile.fileType, 1);
      await until(
        () => find
            .byKey(ValueKey('dataset.media-preview.${mediaFile.id}'))
            .evaluate()
            .isNotEmpty,
        'actual private image bytes rendered',
      );
      Future<List<int>> readPrivate(_Actor actor) async {
        final repo = doc.CloudAssetRepository(
          origin: Uri.parse(fixture['origin']),
          workspaceId: wid,
          accessToken: () async {
            final token = await actor.auth.accessToken();
            if (token == null) throw StateError('synthetic session ended');
            return token;
          },
        );
        try {
          return await repo.read(assetRef);
        } finally {
          repo.close();
        }
      }

      final selectedBytes = await File(fixture['mediaImagePath']).readAsBytes();
      expect(await domain(() => readPrivate(actors[0])), selectedBytes);
      await shot('09-owner-original-private-media');
      await tap('dataset.media-close');
      checks.add(
        'formal-Media14-field-picker-and-original-private-Row4-blob-upload-match-independent-server-read-and-render',
      );
      final mediaHeader = find.text('协作图片').last;
      await t.ensureVisible(mediaHeader);
      await t.tap(mediaHeader);
      await t.pump(const Duration(milliseconds: 350));
      await tap('dataset.media-hide-names');
      await tap('dataset.field-apply');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == scopedGridId)
                .columns
                .singleWhere((f) => f.id == mediaField)
                .mediaHideFileNames ==
            false,
        'original hide_file_names setting',
      );
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[mediaField]!
            .mediaFiles
            .single
            .id,
        mediaFile.id,
      );
      checks.add(
        'original-MediaTypeOption-file-name-setting-preserves-original-Row-file-id-and-private-reference',
      );
      Future<void> mediaRule(
        DatasetController controller,
        int condition,
      ) async {
        final viewPicker = t.widget<DropdownButton<String>>(
          find.byKey(const ValueKey('dataset.view-picker')),
        );
        if (viewPicker.value != scopedGridId) {
          await tap('dataset.view-picker');
          await tap('dataset.view-option.$scopedGridId');
        }
        await tap('dataset.filters');
        if (controller.views
            .singleWhere((v) => v.id == scopedGridId)
            .filters
            .isEmpty) {
          await tap('dataset.rules-add-filter');
        }
        if (controller.views
            .singleWhere((v) => v.id == scopedGridId)
            .filters
            .isEmpty) {
          final picker = find.byType(DropdownButtonFormField<String>).first;
          await t.ensureVisible(picker);
          await t.tap(picker);
          await t.pump(const Duration(milliseconds: 350));
          await t.tap(find.text('协作图片').last);
          await t.pump(const Duration(milliseconds: 350));
        }
        expect(find.byType(TextFormField), findsNothing);
        final picker = find.byType(DropdownButtonFormField<int>).first;
        await t.ensureVisible(picker);
        await t.tap(picker);
        await t.pump(const Duration(milliseconds: 350));
        await t.tap(find.text(condition == 0 ? '为空' : '不为空').last);
        await t.pump(const Duration(milliseconds: 350));
        await tap('dataset.rules-apply');
        await until(
          () => find
              .byKey(const ValueKey('dataset.rules-apply'))
              .evaluate()
              .isEmpty,
          'original media rule accepted',
        );
        await t.pump(const Duration(milliseconds: 350));
        expect(
          controller.views
              .singleWhere((v) => v.id == scopedGridId)
              .filters
              .single['ty'],
          14,
        );
        expect(
          controller.views
              .singleWhere((v) => v.id == scopedGridId)
              .filters
              .single['condition'],
          condition,
        );
        expect(
          controller.views
              .singleWhere((v) => v.id == scopedGridId)
              .filters
              .single['content'],
          '',
        );
      }

      await mediaRule(owner, 0);
      expect(
        owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .rows
            .any((r) => r.id == rowId),
        false,
      );
      expect(owner.rowObjectIds.contains(rowId), true);
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[mediaField]!
            .mediaFiles
            .single
            .id,
        mediaFile.id,
      );
      await shot('12-owner-original-media-empty-filter');
      checks.add(
        'media-empty-original-condition0-hides-only-projection-and-keeps-original-Row4-private-file',
      );
      await mediaRule(owner, 1);
      expect(
        owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .rows
            .any((r) => r.id == rowId),
        true,
      );
      await tap('dataset.sorts');
      await tap('dataset.rules-add-sort');
      final sortPicker = find.byType(DropdownButtonFormField<String>).first;
      await t.ensureVisible(sortPicker);
      await t.tap(sortPicker);
      await t.pump(const Duration(milliseconds: 350));
      expect(
        find.byWidgetPredicate(
          (w) => w is DropdownMenuItem<String> && w.value == mediaField,
        ),
        findsNothing,
      );
      await t.tapAt(const Offset(1, 1));
      await t.pump(const Duration(milliseconds: 350));
      await t.tap(find.text('取消').last);
      await t.pump(const Duration(milliseconds: 350));
      checks.add(
        'media-nonempty-original-condition1-has-no-free-text-and-no-unsupported-media-sort',
      );
      await tap('dataset.undo');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == scopedGridId)
                .filters
                .single['condition'] ==
            0,
        'undo original media nonempty rule',
      );
      await tap('dataset.undo');
      await until(
        () => owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .filters
            .isEmpty,
        'undo original media empty rule',
      );
      expect(
        owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .rows
            .any((r) => r.id == rowId),
        true,
      );
      checks.add(
        'media-filter-own-Root-history-restores-full-view-with-original-file-id-unchanged',
      );
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
        () => owner.views
            .singleWhere((v) => v.id == scopedGridId)
            .columns
            .any((f) => f.name == '资源链接' && f.semanticType == 'url'),
        'original URLTypeOption field',
      );
      final urlField = owner.views
          .singleWhere((v) => v.id == scopedGridId)
          .columns
          .singleWhere((f) => f.name == '资源链接')
          .id;
      final parentUri = parentRef.toUri().toString();
      await editUrl(rowId, urlField, parentUri);
      await until(
        () =>
            owner
                .readRow(
                  owner.views.singleWhere((v) => v.id == scopedGridId).id,
                  rowId,
                )
                .cells[urlField]!
                .value ==
            parentUri,
        'original Row URL cell',
      );
      final originalGrid = owner.views
          .singleWhere((v) => v.id == scopedGridId)
          .id;
      await domain(
        () => owner.createField(
          '展示状态',
          'singleSelect',
          options: const [DatasetOptionDraft(name: '待处理')],
        ),
      );
      final displayStatus = owner.views
          .singleWhere((v) => v.id == scopedGridId)
          .columns
          .singleWhere((f) => f.name == '展示状态');
      final beforeBoard = owner.readRow(originalGrid, rowId);
      await domain(
        () => owner.setCell(
          beforeBoard,
          displayStatus,
          [displayStatus.options.first.id],
          expectedHash: beforeBoard.cells[displayStatus.id]!.hash,
        ),
      );
      final summaryBoard = await domain(
        () => owner.createBoardView('资源卡片', displayStatus),
      );
      await tap('dataset.view-picker');
      await tap('dataset.view-option.$summaryBoard');
      await until(
        () => find.text(mediaFile.name).evaluate().isNotEmpty,
        'original media filename in Board summary',
      );
      expect(find.textContaining('文档链接'), findsOneWidget);
      expect(find.textContaining('muse-asset://'), findsNothing);
      expect(find.textContaining('openmuse-resource://'), findsNothing);
      await shot('15-owner-original-media-and-resource-card-summary');
      await tap('dataset.view-picker');
      await tap('dataset.view-option.$originalGrid');
      expect(
        owner.readRow(originalGrid, rowId).cells[urlField]!.value,
        parentUri,
      );
      expect(
        owner
            .readRow(originalGrid, rowId)
            .cells[mediaField]!
            .mediaFiles
            .single
            .url,
        mediaFile.url,
      );
      checks.add(
        'actual-original-Board-reuses-media-summary-and-resource-caption-without-changing-private-reference-or-URL-cell',
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
        guest.views
            .singleWhere((v) => v.id == scopedGridId)
            .rows
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
      await until(
        () => guest.views
            .singleWhere((v) => v.id == scopedGridId)
            .columns
            .any((f) => f.id == mediaField),
        'recipient Media14 schema',
      );
      await tap('dataset.cell.$rowId.$mediaField');
      await until(
        () => find
            .byKey(ValueKey('dataset.media-preview.${mediaFile.id}'))
            .evaluate()
            .isNotEmpty,
        'actual Guest Row4 private asset inherited read',
      );
      expect(
        find.byKey(const ValueKey('dataset.media-add-image')),
        findsNothing,
      );
      expect(
        find.byKey(ValueKey('dataset.media-rename.${mediaFile.id}')),
        findsNothing,
      );
      expect(await domain(() => readPrivate(actors[1])), selectedBytes);
      final mediaSize = t.view.physicalSize;
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
      expect(
        t.getSize(find.byKey(const ValueKey('dataset.media-close'))).height,
        greaterThanOrEqualTo(44),
      );
      await shot('10-readonly-private-image-at430');
      await tap('dataset.media-close');
      t.view.physicalSize = mediaSize;
      await t.pump(const Duration(milliseconds: 350));
      expect(guest.localMutationEpoch, readEpoch);
      checks.add(
        'original-inherited-readonly-Guest-can-read-same-Row4-private-image-without-cell-writes-at430',
      );
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
      expect(
        t
            .widget<SelectableText>(
              find.byKey(const ValueKey('dataset.url-value')),
            )
            .data,
        '文档链接',
      );
      expect(find.byKey(const ValueKey('dataset.url-address')), findsNothing);
      expect(find.text(parentUri), findsNothing);
      expect(
        t
            .getSize(find.byKey(const ValueKey('dataset.url-show-address')))
            .height,
        greaterThanOrEqualTo(44),
      );
      await shot('16-readonly-resource-link-summary-at430');
      await tap('dataset.url-show-address');
      expect(
        t
            .widget<SelectableText>(
              find.byKey(const ValueKey('dataset.url-address')),
            )
            .data,
        parentUri,
      );
      await copyUrl(parentUri);
      await tap('dataset.url-show-address');
      expect(find.byKey(const ValueKey('dataset.url-address')), findsNothing);
      checks.add(
        'actual430-readonly-resource-caption-explicit-full-address-and-exact-original-link-copy-preserve-reader-epoch',
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
      await showActor(1);
      await tap('dataset.sync');
      await until(() => guest.canWrite, 'original share upgrade');
      await edit(rowId, fieldId, '链接接收者协作编辑😀');
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == scopedGridId)
                .rows
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
            guest
                .readRow(
                  guest.views.singleWhere((v) => v.id == scopedGridId).id,
                  rowId,
                )
                .cells[fieldId]!
                .value ==
            'URL撤销保留同伴标题😀',
        'original peer title before own URL edit',
      );
      await editUrl(rowId, urlField, 'https://example.test/peer-native');
      await until(
        () =>
            owner
                .readRow(
                  owner.views.singleWhere((v) => v.id == scopedGridId).id,
                  rowId,
                )
                .cells[urlField]!
                .value ==
            'https://example.test/peer-native',
        'original Row4 URL update reaches the peer',
      );
      await tap('dataset.undo');
      await until(
        () =>
            guest
                    .readRow(
                      guest.views.singleWhere((v) => v.id == scopedGridId).id,
                      rowId,
                    )
                    .cells[urlField]!
                    .value ==
                parentUri &&
            owner
                    .readRow(
                      owner.views.singleWhere((v) => v.id == scopedGridId).id,
                      rowId,
                    )
                    .cells[urlField]!
                    .value ==
                parentUri,
        'own URL undo crosses original Cloud',
      );
      expect(
        guest
            .readRow(
              guest.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[fieldId]!
            .value,
        'URL撤销保留同伴标题😀',
      );
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[fieldId]!
            .value,
        'URL撤销保留同伴标题😀',
      );
      checks.add(
        'original-Row4-URL-collaboration-and-own-UI-undo-preserve-the-peer-other-cell-edit',
      );
      await tap('dataset.cell.$rowId.$mediaField');
      await tap('dataset.media-rename.${mediaFile.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.media-name')),
        '同伴图片.png',
      );
      await tap('dataset.media-name-apply');
      await until(
        () =>
            owner
                .readRow(
                  owner.views.singleWhere((v) => v.id == scopedGridId).id,
                  rowId,
                )
                .cells[mediaField]!
                .mediaFiles
                .single
                .name ==
            '同伴图片.png',
        'peer media name uses original Row4 collaboration',
      );
      await tap('dataset.media-close');
      await showActor(0);
      await edit(rowId, fieldId, '媒体撤销保留同伴标题😀');
      await showActor(1);
      await until(
        () =>
            guest
                .readRow(
                  guest.views.singleWhere((v) => v.id == scopedGridId).id,
                  rowId,
                )
                .cells[fieldId]!
                .value ==
            '媒体撤销保留同伴标题😀',
        'peer other-cell title before media removal',
      );
      await tap('dataset.cell.$rowId.$mediaField');
      await tap('dataset.media-remove.${mediaFile.id}');
      await tap('dataset.media-remove-confirm');
      await until(
        () => guest
            .readRow(
              guest.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[mediaField]!
            .mediaFiles
            .isEmpty,
        'original media reference removal',
      );
      await tap('dataset.media-close');
      await tap('dataset.undo');
      await until(
        () =>
            owner
                    .readRow(
                      owner.views.singleWhere((v) => v.id == scopedGridId).id,
                      rowId,
                    )
                    .cells[mediaField]!
                    .mediaFiles
                    .length ==
                1 &&
            guest
                    .readRow(
                      guest.views.singleWhere((v) => v.id == scopedGridId).id,
                      rowId,
                    )
                    .cells[mediaField]!
                    .mediaFiles
                    .length ==
                1,
        'own media undo converges',
      );
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[mediaField]!
            .mediaFiles
            .single
            .id,
        mediaFile.id,
      );
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[fieldId]!
            .value,
        '媒体撤销保留同伴标题😀',
      );
      expect(await domain(() => readPrivate(actors[1])), selectedBytes);
      checks.add(
        'original-Row4-media-peer-rename-remove-and-own-UI-undo-preserve-file-id-private-blob-and-peer-other-cell',
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
            owner.views
                .singleWhere((v) => v.id == scopedGridId)
                .rows
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

      await mediaRule(guest, 0);
      await domain(owner.synchronize);
      await until(
        () =>
            owner.views
                .singleWhere((v) => v.id == scopedGridId)
                .filters
                .isNotEmpty &&
            owner.views
                    .singleWhere((v) => v.id == scopedGridId)
                    .filters
                    .single['condition'] ==
                0,
        'shared media filter reaches independent Owner',
      );
      final peerRow = owner.readRow(
            owner.views.singleWhere((v) => v.id == scopedGridId).id,
            rowId,
          ),
          peerField = owner.views
              .singleWhere((v) => v.id == scopedGridId)
              .columns
              .singleWhere((f) => f.id == fieldId);
      await domain(
        () => owner.setCell(
          peerRow,
          peerField,
          '筛选隐藏行同伴更新😀',
          expectedHash: peerRow.cells[fieldId]!.hash,
        ),
      );
      await domain(owner.synchronize);
      await domain(guest.synchronize);
      expect(
        guest.views
            .singleWhere((v) => v.id == scopedGridId)
            .rows
            .any((r) => r.id == rowId),
        false,
      );
      final recordContext = t.element(find.byType(DatasetSurface));
      unawaited(
        showDatasetRecordEditor(
          recordContext,
          guest,
          guest.views.singleWhere((v) => v.id == scopedGridId).id,
          rowId,
        ),
      );
      await t.pump(const Duration(milliseconds: 350));
      expect(find.text('该记录未匹配当前筛选，仍可在这里编辑。'), findsOneWidget);
      expect(find.text('筛选隐藏行同伴更新😀'), findsOneWidget);
      final recordDialog = find.byType(AlertDialog).last;
      expect(
        find.descendant(of: recordDialog, matching: find.text('文档链接')),
        findsOneWidget,
      );
      expect(
        find.descendant(of: recordDialog, matching: find.text(parentUri)),
        findsNothing,
      );
      expect(
        guest
            .readRow(
              guest.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[urlField]!
            .value,
        parentUri,
      );
      checks.add(
        'actual-authorized-hidden-record-renders-compact-resource-link-and-keeps-original-portable-URI',
      );
      await shot('13-guest-shared-filter-hidden-row-detail');
      await tap('dataset.record-close');
      checks.add(
        'original-sharing-writer-media-rule-sync-keeps-hidden-row-authorized-detail-and-peer-edit',
      );
      await tap('dataset.undo');
      await until(
        () => guest.views
            .singleWhere((v) => v.id == scopedGridId)
            .filters
            .isEmpty,
        'guest own media rule undo',
      );
      expect(
        guest
            .readRow(
              guest.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[fieldId]!
            .value,
        '筛选隐藏行同伴更新😀',
      );
      await domain(guest.synchronize);
      await domain(owner.synchronize);
      expect(
        owner.views.singleWhere((v) => v.id == scopedGridId).filters,
        isEmpty,
      );
      expect(
        owner
            .readRow(
              owner.views.singleWhere((v) => v.id == scopedGridId).id,
              rowId,
            )
            .cells[fieldId]!
            .value,
        '筛选隐藏行同伴更新😀',
      );
      checks.add(
        'guest-own-Root-filter-undo-preserves-independent-Owner-Row4-edit',
      );

      const pendingQuotaValue = '本机保留待同步😀';
      admissions[1].blockObject = rowId;
      admissions[1].wireType = 4;
      await edit(rowId, fieldId, pendingQuotaValue);
      await until(
        () =>
            admissions[1].rejectedRequests > 0 &&
            guest.syncState == DatasetSyncState.unavailable,
        'real server payload quota rejection',
      );
      expect(
        guest.syncFailure,
        isA<AppFlowyTransportFailure>().having(
          (e) => e.code,
          'code',
          'QUOTA_EXCEEDED',
        ),
      );
      expect(guest.canRead, true);
      expect(guest.canWrite, true);
      checks.add(
        'actual-original-server-payload-limit-is-a-typed-quota-failure-without-forbidden-or-removed-downgrade',
      );
      await until(
        () => guest.saveState == DatasetSaveState.saved,
        'pending edit saved to original local snapshot store',
      );
      expect(
        guest.readRow(scopedGridId, rowId).cells[fieldId]!.value,
        pendingQuotaValue,
      );
      // HTTP admission is independent of the existing live WS2 channel.
      // Verify actual original local storage; a rejected HTTP request does not
      // imply that another authorized transport could not deliver the edit.
      final persistedQuota = await domain(guest.store.read);
      expect(persistedQuota, isNotNull);
      final reopenedQuota = await domain(
        () => DatasetController.open(
          core: NativeDatasetCore(),
          store: _LegacyStore(),
          objectId: rootId,
          editable: false,
          syncInterval: null,
          initialBundle: Map<String, dynamic>.from(
            persistedQuota!['bundle'] as Map,
          ),
        ),
      );
      expect(
        reopenedQuota.readRow(scopedGridId, rowId).cells[fieldId]!.value,
        pendingQuotaValue,
      );
      await domain(reopenedQuota.close);
      checks.add(
        'original-Row4-local-snapshot-independently-reopens-retained-edit-after-actual-HTTP-quota-rejection-with-live-WS2-preserved',
      );
      await until(
        () => find.textContaining('同步内容超过服务处理上限').evaluate().isNotEmpty,
        'human-readable pending-local-edit notice',
      );
      await shot('17-actual-server-quota-keeps-local-row-edit');
      await tap('dataset.cell.$rowId.$fieldId');
      expect(
        t.widget<EditableText>(find.byType(EditableText).last).controller.text,
        pendingQuotaValue,
      );
      expect(
        t
            .widget<ButtonStyleButton>(
              find.byKey(const ValueKey('dataset.cell-apply')),
            )
            .onPressed,
        isNotNull,
      );
      await t.tap(find.text('取消').last);
      await t.pump(const Duration(milliseconds: 350));
      checks.add(
        'actual-Host-keeps-pending-value-visible-and-editable-after-original-server-rejection',
      );
      admissions[1].blockObject = null;
      await tap('dataset.sync');
      await until(
        () => guest.syncState == DatasetSyncState.accepted,
        'ordinary original sync recovers after controlled quota payload released',
      );
      await domain(owner.synchronize);
      await until(
        () =>
            owner.readRow(scopedGridId, rowId).cells[fieldId]!.value ==
            pendingQuotaValue,
        'preserved local edit reaches original peer',
      );
      await shot('18-original-sync-recovers-with-retained-row-value');
      checks.add(
        'explicit-sync-after-controlled-fault-release-preserves-local-value-and-reaches-original-peer-without-new-ACK-claim',
      );
      await tap('dataset.cell.$rowId.$mediaField');
      final staleRename = t
          .widget<TextButton>(
            find.byKey(ValueKey('dataset.media-rename.${mediaFile.id}')),
          )
          .onPressed!;
      final staleAdd = t
          .widget<TextButton>(
            find.byKey(const ValueKey('dataset.media-add-image')),
          )
          .onPressed!;
      final staleRemove = t
          .widget<TextButton>(
            find.byKey(ValueKey('dataset.media-remove.${mediaFile.id}')),
          )
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
              find.text('附件不可用，或访问权限已变更').evaluate().isNotEmpty,
          'real original parent revocation retires private media reading',
        );
        staleRename();
        staleAdd();
        staleRemove();
        await t.pump(const Duration(milliseconds: 350));
        expect(find.byKey(const ValueKey('dataset.media-name')), findsNothing);
        expect(
          find.byKey(const ValueKey('dataset.media-remove-confirm')),
          findsNothing,
        );
        expect(
          find.byKey(ValueKey('dataset.media-preview.${mediaFile.id}')),
          findsNothing,
        );
        expect(
          find.byKey(
            ValueKey(
              'dataset.media-grid-preview.$rowId.$mediaField.${mediaFile.id}',
            ),
          ),
          findsNothing,
        );
        await expectLater(
          domain(() => readPrivate(actors[1])),
          throwsA(isA<AssetFailure>()),
        );
        expect(t.takeException(), isNull);
        var dispatched = 0;
        await expectLater(
          openDatasetUrl(
            guest,
            DatasetUrlReference(
              objectId: rootId,
              viewId: guest.views.singleWhere((v) => v.id == scopedGridId).id,
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
        await shot('11-revoked-original-private-media');
        checks.add(
          'real-parent-sharing-revocation-hides-private-pixels-rejects-actual-Row4-blob-read-and-fences-old-media-actions',
        );
        await tap('dataset.media-close');
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
      final legacyFixture =
          jsonDecode(await File(fixture['legacyCellPath']).readAsString())
              as Map;
      final legacy = await domain(
        () => DatasetController.open(
          core: NativeDatasetCore(),
          store: _LegacyStore(),
          objectId: legacyFixture['objectId'],
          initialBundle: Map<String, dynamic>.from(
            legacyFixture['bundle'] as Map,
          ),
          syncInterval: null,
        ),
      );
      final legacySize = t.view.physicalSize;
      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      await t.pumpWidget(
        RepaintBoundary(
          key: const ValueKey('legacy-guard-boundary'),
          child: MaterialApp(
            home: Scaffold(
              body: DatasetSurface(controller: legacy, title: '原单元格类型保护'),
            ),
          ),
        ),
      );
      await t.pump(const Duration(milliseconds: 350));
      final legacyView = legacy.views.single,
          legacyRow = legacyView.rows.single,
          legacyField = legacyView.columns.singleWhere((f) => f.id == 'media');
      expect(legacyRow.cells['media']!.unresolvedCellType, true);
      expect(
        find.byKey(ValueKey('dataset.cell.${legacyRow.id}.media')),
        findsNothing,
      );
      final legacyContext = t.element(find.byType(DatasetSurface));
      unawaited(
        showDatasetRecordEditor(
          legacyContext,
          legacy,
          legacyView.id,
          legacyRow.id,
        ),
      );
      await t.pump(const Duration(milliseconds: 350));
      expect(
        t
            .widget<ListTile>(
              find.byKey(const ValueKey('dataset.record-field.media')),
            )
            .enabled,
        false,
      );
      await t.ensureVisible(
        find.byKey(const ValueKey('dataset.record-field.media')),
      );
      await t.pump(const Duration(milliseconds: 350));
      final legacyBoundary = t.renderObject<RenderRepaintBoundary>(
        find.byKey(const ValueKey('legacy-guard-boundary')),
      );
      expect(legacyBoundary.size.width, 430);
      final legacyImage = await legacyBoundary.toImage(pixelRatio: 1),
          legacyBytes = await legacyImage.toByteData(
            format: ui.ImageByteFormat.png,
          );
      await File(
        '${evidence.path}/14-original-legacy-cell-guard-at430.png',
      ).writeAsBytes(legacyBytes!.buffer.asUint8List());
      legacyImage.dispose();
      final legacyBefore = jsonEncode(legacy.bundle);
      await expectLater(
        domain(
          () => legacy.setCell(
            legacyRow,
            legacyField,
            '',
            expectedHash: legacyRow.cells['media']!.hash,
          ),
        ),
        throwsA(isA<DatasetFailure>()),
      );
      expect(jsonEncode(legacy.bundle), legacyBefore);
      await t.pumpWidget(const SizedBox());
      await domain(legacy.close);
      t.view.physicalSize = legacySize;
      checks.add(
        'actual-original-legacy-Media14-under-Text0-renders-readonly-preservation-at430-and-clear-rejects-without-state-change',
      );
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
          'mediaFieldType': 14,
          'originalUrlAndAssetIdentityPreserved': true,
          'resourcePresentationChecks': 3,
          'quotaUiChecks': 4,
          'quotaInjectionScope':
              'original scoped provider forwarded a controlled over-budget protobuf to the actual server; user edited a small local value, not50MiB UI typing',
          'legacyFixtureScope':
              'original-synthetic-Root1-Row4-local-bundle; not Cloud-import UI',
          'assetOwnerType': 4,
          'imageSelection':
              'controlled-image-picker-capability; system picker UI not claimed',
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_FULL_SYNC_ADMISSION_CHECKS=${checks.length}');
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

class _LegacyStore implements SnapshotStore {
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> data) async {
    value = data;
  }
}

class _AdmissionControl {
  _AdmissionControl(this.doc, this.vector);
  final Uint8List doc, vector;
  String? blockObject;
  int wireType = 4, rejectedRequests = 0;
  List<int> integer(int value) {
    final out = <int>[];
    while (value > 127) {
      out.add((value & 127) | 128);
      value >>= 7;
    }
    out.add(value);
    return out;
  }

  List<int> field(int tag, List<int> bytes) => [
    ...integer(tag * 8 + 2),
    ...integer(bytes.length),
    ...bytes,
  ];
  Uint8List request(String id) => Uint8List.fromList([
    ...field(1, utf8.encode(id)),
    16,
    ...integer(wireType),
    24,
    1,
    ...field(4, vector),
    ...field(5, doc),
  ]);
}

class _AdmissionClient extends http.BaseClient {
  _AdmissionClient(this.control);
  final _AdmissionControl control;
  final http.Client inner = http.Client();
  @override
  Future<http.StreamedResponse> send(http.BaseRequest request) {
    final segments = request.url.pathSegments;
    if (request is http.Request &&
        request.method == 'POST' &&
        segments.length == 7 &&
        segments.last == 'full-sync' &&
        control.blockObject != null &&
        segments[5] == control.blockObject) {
      control.rejectedRequests++;
      request.bodyBytes = control.request(control.blockObject!);
    }
    return inner.send(request);
  }

  @override
  void close() => inner.close();
}
