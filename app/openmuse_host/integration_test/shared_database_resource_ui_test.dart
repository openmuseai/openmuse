import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart' as ae;
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'standalone and embedded databases share a bounded owner in the actual Host',
    (t) async {
      final run = DateTime.now().microsecondsSinceEpoch.toString();
      final root = await Directory.systemTemp.createTemp('smartdoc-host-');
      final evidence = Directory('${root.path}/evidence');
      await evidence.create();
      debugPrint('SMARTDOC_HOST_WORKING=${evidence.path}');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final plugins = createOpenMuseBuiltInPlugins()
          .where(
            (p) => {
              'io.openmuse.smart-document',
              'io.openmuse.dataset',
            }.contains(p.descriptor.id),
          )
          .toList();
      expect(plugins, hasLength(2));
      for (final plugin in plugins) {
        registry.install(plugin);
        await registry.activate(plugin.descriptor.id);
      }
      final existing = File('${root.path}/existing.txt');
      await existing.writeAsString('Existing workspace resource');
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: [
          OpenMuseResource(uri: existing.uri, displayName: 'existing.txt'),
        ],
      );
      workspace.openResource(
        OpenMuseResource(uri: existing.uri, displayName: 'existing.txt'),
      );
      final center = await NativeResourceCenter.open(
        scope: 'smartdoc-host-$run',
        deviceRef: 'host-test',
        deviceLabel: 'Mac',
        registry: registry,
        storageRoot: Directory('${root.path}/resources'),
      );
      final fault = _CheckpointFault(center.documents);
      center.hub.unregister(center.documents.source.key);
      center.hub.register(fault);
      final centers = ValueNotifier<NativeResourceCenter?>(center);
      final checks = <String>[];
      Future<void> until(bool Function() condition) async {
        for (var i = 0; i < 200; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (condition()) return;
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
        }
        fail(
          'Host UI deadline: ${t.widgetList<Text>(find.byType(Text)).map((w) => w.data).join(' | ')}',
        );
      }

      Future<void> tap(String key) async {
        final finder = find.byKey(ValueKey(key));
        await until(() => finder.evaluate().isNotEmpty);
        await t.ensureVisible(finder);
        await until(() => finder.hitTestable().evaluate().length == 1);
        await t.tap(finder);
        await t.pump(const Duration(milliseconds: 200));
      }

      Future<void> create(String plugin, String creator, String name) async {
        await tap('desktop-resource-center');
        await tap('resource-new');
        await tap('resource-creator-$plugin/$creator');
        await t.enterText(
          find.byKey(const ValueKey('resource-create-name')),
          name,
        );
        await tap('resource-create-confirm');
        await t.pump(const Duration(milliseconds: 500));
      }

      Future<void> shot(String name) async {
        await t.pump(const Duration(milliseconds: 250));
        // Integration screenshots avoid Native AX queries during the driver run.
        final boundary = t.renderObject<RenderRepaintBoundary>(
          find.byKey(const ValueKey('host.acceptance.surface')),
        );
        final image = await boundary.toImage(pixelRatio: 1);
        final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(bytes!.buffer.asUint8List());
        image.dispose();
      }

      await t.pumpWidget(
        RepaintBoundary(
          key: const ValueKey('host.acceptance.surface'),
          child: OpenMuseHostApp(
            registry: registry,
            workspace: workspace,
            resources: centers,
          ),
        ),
      );
      await create(
        'io.openmuse.dataset',
        'dataset.database',
        '共享数据库.musedataset',
      );
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      final data = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      final databaseTab = workspace.activeTab!;
      final databaseRecord = databaseTab.resourceSession!.session.record;
      await tap('dataset.add-row');
      await until(() => data.views.first.rows.isNotEmpty);
      final row = data.views.first.rows.first,
          field = data.views.first.columns.first;
      await tap('dataset.cell.${row.id}.${field.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '独立入口创建😀',
      );
      await tap('dataset.cell-apply');
      await create(
        'io.openmuse.smart-document',
        'smart-document.document',
        '嵌入共享文档.musedoc',
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      final document = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      final docTab = workspace.activeTab!;
      Future<void> insertDatabase() async {
        await tap('smart-document.insert');
        await t.pump(const Duration(milliseconds: 350));
        await t.tap(find.text('数据库').last);
        await until(
          () => find
              .byKey(const ValueKey('resource-embed-picker'))
              .evaluate()
              .isNotEmpty,
        );
        await t.pump(const Duration(milliseconds: 400));
        await tap('resource-row-${databaseRecord.identity.resource}');
        await until(
          () => find
              .byKey(const ValueKey('resource-embed-picker'))
              .evaluate()
              .isEmpty,
        );
      }

      await insertDatabase();
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      final embedded = find.byType(DatasetSurface);
      expect(t.widget<DatasetSurface>(embedded.first).controller, same(data));
      expect(find.text('独立入口创建😀'), findsOneWidget);
      checks.add(
        'actual-resource-picker-embeds-original-database-and-reuses-standalone-owner',
      );
      await shot('shared-database-first');
      await tap('dataset.cell.${row.id}.${field.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '嵌入修改同一记录😀',
      );
      await tap('dataset.cell-apply');
      await until(
        () =>
            data.views.first.rows.first.cells[field.id]?.value == '嵌入修改同一记录😀',
      );
      checks.add('embedded-cell-edit-updates-original-root-and-row');
      final parentBeforeHistory = jsonEncode(document.snapshot['data']);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(
        () => data.views.first.rows.first.cells[field.id]?.value == '独立入口创建😀',
      );
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
      await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
      await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(
        () =>
            data.views.first.rows.first.cells[field.id]?.value == '嵌入修改同一记录😀',
      );
      expect(jsonEncode(document.snapshot['data']), parentBeforeHistory);
      checks.add('embedded-keyboard-undo-redo-remains-in-child-domain');

      workspace.closeTab(databaseTab);
      await until(() => !workspace.tabs.contains(databaseTab));
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface).first).controller,
        same(data),
      );
      await tap('dataset.add-row');
      await until(() => data.views.first.rows.length == 2);
      checks.add(
        'closing-original-standalone-tab-retains-embedded-owner-and-editing',
      );
      await insertDatabase();
      await until(
        () => document.blocks.where((b) => b.type == 'grid').length == 2,
      );
      // The original editor lazily mounts long documents; inspect mounted child
      // presentations while the domain keeps a single shared Root/Row owner.
      await until(() => find.byType(DatasetSurface).evaluate().length == 2);
      for (final surface in t.widgetList<DatasetSurface>(
        find.byType(DatasetSurface),
      )) {
        expect(surface.controller, same(data));
      }
      checks.add('second-database-reference-keeps-original-object-identity');
      await shot('shared-database-two-references');
      final firstReference = document.blocks.firstWhere(
        (b) => b.type == 'grid',
      );
      await tap('smart-document.embed-menu.${firstReference.id}');
      await t.pump(const Duration(milliseconds: 350));
      await t.tap(find.text('移除引用').last);
      await until(
        () =>
            document.blocks.where((b) => b.type == 'grid').length == 1 &&
            find.byType(DatasetSurface).evaluate().length == 1,
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(data),
      );
      final childScope = ResourceEditorSessionScope.maybeOf(
        t.element(find.byType(DatasetSurface)),
      );
      expect(childScope!.record.identity, databaseRecord.identity);
      expect(
        childScope.record.identity,
        isNot(docTab.resourceSession!.session.record.identity),
      );
      final parentBeforeChildMutation = jsonEncode(document.snapshot['data']);
      fault.failResource = databaseRecord.identity.resource;
      fault.holdFailure = Completer<void>();
      await tap('dataset.add-row');
      await until(() => data.views.first.rows.length == 3);
      expect(jsonEncode(document.snapshot['data']), parentBeforeChildMutation);
      checks.add(
        'removing-one-reference-preserves-other-owner-and-parent-content-boundary',
      );

      workspace.closeTab(docTab);
      await until(() => document.isClosed && data.isClosed);
      expect(document.isDisposed || data.isDisposed, isFalse);
      await shot('shared-tree-prepared-before-child-failure');
      checks.add(
        'all-original-owners-prepare-before-child-checkpoint-failure-completes',
      );
      fault.holdFailure!.complete();
      await until(
        () => find
            .byKey(const ValueKey('resource-keep-editing'))
            .evaluate()
            .isNotEmpty,
      );
      expect(workspace.tabs.contains(docTab), isTrue);
      expect(data.views.first.rows, hasLength(3));
      await shot('shared-child-save-failure');
      await tap('resource-keep-editing');
      await until(() => !docTab.resourceSession!.confirming);
      expect(document.canWrite, isTrue);
      expect(data.canWrite, isTrue);
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(data),
      );
      final parentStateBeforeLayout = t.state(find.byType(SmartDocumentView));
      final childStateBeforeLayout = t.state(find.byType(DatasetSurface));
      for (final viewName in ['IDE 视图', 'Agent 视图']) {
        await tap('title-view');
        final item = find.ancestor(
          of: find.text(viewName),
          matching: find.byType(PopupMenuItem<String>),
        );
        await until(() => item.hitTestable().evaluate().length == 1);
        await t.tap(item);
        await t.pump(const Duration(milliseconds: 350));
      }
      expect(
        t.state(find.byType(SmartDocumentView)),
        same(parentStateBeforeLayout),
      );
      expect(
        t.state(find.byType(DatasetSurface)),
        same(childStateBeforeLayout),
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(data),
      );
      checks.add(
        'view-changes-after-child-checkpoint-failure-keep-parent-and-embedded-database-states',
      );
      await shot('shared-failed-close-layout-keeps-parent-child');
      final parentText = find.byType(ae.AppFlowyRichText).first;
      await t.ensureVisible(parentText);
      await until(() => parentText.hitTestable().evaluate().length == 1);
      await t.tap(parentText);
      await t.pump(const Duration(milliseconds: 150));
      final clipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '关闭失败后的父正文😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () =>
              document.blocks.first.delta!.toPlainText().contains('关闭失败后的父正文'),
        );
      } finally {
        await Clipboard.setData(ClipboardData(text: clipboard?.text ?? ''));
      }
      expect(data.views.first.rows, hasLength(3));
      // Agent view is narrower, so its wrapped toolbar leaves fewer grid rows
      // visible. Scroll the actual child grid before targeting its last row.
      final childRows = find.descendant(
        of: find.byType(DatasetSurface),
        matching: find.byType(ListView),
      );
      await t.drag(childRows.first, const Offset(0, -160));
      await t.pump(const Duration(milliseconds: 250));
      final retainedRow = data.views.first.rows.last.id;
      await tap('dataset.cell.$retainedRow.${field.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '子关闭失败后仍可编辑',
      );
      await tap('dataset.cell-apply');
      await until(
        () => data.views.first.rows.last.cells[field.id]?.value == '子关闭失败后仍可编辑',
      );
      checks.add(
        'failed-child-checkpoint-restores-parent-and-child-input-on-original-owners',
      );
      await shot('shared-parent-child-input-after-failed-close');
      fault.failResource = null;
      final retry = find.widgetWithText(TextButton, '重试').last;
      await t.ensureVisible(retry);
      await t.tap(retry);
      await until(() => find.text('资源保存失败，本机修改保留').evaluate().isEmpty);
      checks.add('parent-close-retains-child-failure-and-explicit-retry');
      final docRecord = docTab.resourceSession!.session.record;
      workspace.closeTab(docTab);
      await until(() => !workspace.tabs.contains(docTab));
      await center.controller.refresh();
      final context = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(
          context,
          center.controller,
          databaseRecord.reference(),
        ),
      );
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      final reopened = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      expect(reopened.objectId, data.objectId);
      expect(reopened.views.first.rows, hasLength(3));
      expect(reopened.views.first.rows.first.id, row.id);
      expect(
        reopened.views.first.rows.first.cells[field.id]?.value,
        '嵌入修改同一记录😀',
      );
      checks.add(
        'last-surface-close-checkpoints-independent-resource-for-reopen',
      );
      await shot('shared-database-reopen');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(DatasetSurface).evaluate().isEmpty);
      final docContext = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(docContext, center.controller, docRecord.reference()),
      );
      await until(
        () =>
            find.byType(SmartDocumentView).evaluate().isNotEmpty &&
            find.byType(DatasetSurface).evaluate().isNotEmpty,
      );
      final restored = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      expect(restored.blocks.where((b) => b.type == 'grid'), hasLength(1));
      expect(restored.blocks.first.delta!.toPlainText(), contains('关闭失败后的父正文'));
      expect(
        t
            .widget<DatasetSurface>(find.byType(DatasetSurface).first)
            .controller
            .views
            .first
            .rows
            .last
            .cells[field.id]
            ?.value,
        '子关闭失败后仍可编辑',
      );
      expect(
        t
            .widget<DatasetSurface>(find.byType(DatasetSurface).first)
            .controller
            .views
            .first
            .rows,
        hasLength(3),
      );
      checks.add(
        'parent-reopen-resolves-persisted-locators-without-copying-child-content',
      );
      await shot('shared-parent-reopen');
      final recoveredDb = t
          .widget<DatasetSurface>(find.byType(DatasetSurface).first)
          .controller;
      final recoveryTab = workspace.activeTab!;
      final recoverySession = recoveryTab.resourceSession!.session;
      fault.rejectAll = true;
      final privateClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        final editor = t
            .widget<SmartDocumentView>(find.byType(SmartDocumentView))
            .editor;
        editor.selection = ae.Selection.collapsed(
          ae.Position(path: [0], offset: 0),
        );
        await t.pump();
        final privateText = find.byType(ae.AppFlowyRichText).first;
        await t.ensureVisible(privateText);
        await until(() => privateText.hitTestable().evaluate().length == 1);
        await t.tap(privateText);
        await Clipboard.setData(const ClipboardData(text: '仅本机恢复父正文😀 '));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => restored.blocks.first.delta!.toPlainText().contains('仅本机恢复父正文'),
        );
      } finally {
        await Clipboard.setData(
          ClipboardData(text: privateClipboard?.text ?? ''),
        );
      }
      final privateRow = recoveredDb.views.first.rows.last.id;
      await tap('dataset.cell.$privateRow.${field.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '仅本机恢复数据库😀',
      );
      await tap('dataset.cell-apply');
      await until(
        () =>
            recoveredDb.views.first.rows.last.cells[field.id]?.value ==
            '仅本机恢复数据库😀',
      );
      final versionBeforePrivate = recoverySession.record.version;
      workspace.closeTab(recoveryTab);
      await until(
        () => find
            .byKey(const ValueKey('resource-preserve-private'))
            .evaluate()
            .isNotEmpty,
      );
      expect(find.text('可以保留本机副本后关闭，稍后重新打开继续编辑。已共享的内容不会撤回。'), findsOneWidget);
      await shot('private-recovery-close-choice');
      final attemptsBeforePrivate = fault.attempts;
      await tap('resource-preserve-private');
      await until(() => !workspace.tabs.contains(recoveryTab));
      await t.pump(const Duration(milliseconds: 600));
      expect(fault.attempts, attemptsBeforePrivate);
      expect(recoverySession.record.version, versionBeforePrivate);
      expect(recoverySession.closeFinalized, isTrue);
      checks.add(
        'private-recovery-tree-close-does-not-retry-provider-or-advance-resource-version',
      );
      await shot('private-recovery-closed');
      await center.controller.refresh();
      final recoveryContext = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(
          recoveryContext,
          center.controller,
          docRecord.reference(),
        ),
      );
      await until(
        () =>
            find.byType(SmartDocumentView).evaluate().isNotEmpty &&
            find.byType(DatasetSurface).evaluate().isNotEmpty,
      );
      final privateDocument = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      final privateDatabase = t
          .widget<DatasetSurface>(find.byType(DatasetSurface).first)
          .controller;
      expect(
        privateDocument.blocks.first.delta!.toPlainText(),
        contains('仅本机恢复父正文'),
      );
      expect(
        privateDatabase.views.first.rows.last.cells[field.id]?.value,
        '仅本机恢复数据库😀',
      );
      checks.add(
        'private-document-and-database-reopen-from-original-isolated-recovery-stores',
      );
      await shot('private-recovery-parent-child-reopen');
      final finalRecoveryTab = workspace.activeTab!;
      workspace.closeTab(finalRecoveryTab);
      await until(
        () => find
            .byKey(const ValueKey('resource-preserve-private'))
            .evaluate()
            .isNotEmpty,
      );
      await tap('resource-preserve-private');
      await until(() => !workspace.tabs.contains(finalRecoveryTab));
      fault.rejectAll = false;
      checks.add(
        'repeated-private-close-releases-owner-and-embedded-leases-without-late-save',
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isEmpty);
      expect(t.takeException(), isNull);
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'status': 'passed-shared-database-host-slice',
          'checks': checks,
          'fullProductAcceptance': 'not-complete',
          'limits': [
            'Document-in-document embedding, hidden/rebound surfaces, readonly-owner upgrade and full-platform/durable gates remain open',
          ],
        }),
      );
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      workspace.dispose();
      centers.dispose();
      await center.close();
      for (final plugin in plugins) {
        await registry.deactivate(plugin.descriptor.id);
      }
      debugPrint('SMARTDOC_HOST_EVIDENCE=${evidence.path}');
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}

// Test-only fault at the bounded commit port. Every successful operation still
// reaches the actual local resource provider; no content or storage is mocked.
final class _CheckpointFault implements ResourceProvider {
  _CheckpointFault(this.inner);
  final ResourceProvider inner;
  String? failResource;
  bool rejectAll = false;
  int attempts = 0;
  Completer<void>? holdFailure;
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
  ) async {
    attempts++;
    if (rejectAll || ref == failResource) {
      await holdFailure?.future;
      throw const ResourceFailure('TEST_CHECKPOINT_FAILURE');
    }
    return inner.commit(ref, base, bytes, key, access);
  }
}
