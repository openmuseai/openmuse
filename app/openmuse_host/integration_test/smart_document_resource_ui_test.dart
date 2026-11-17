import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart';
import 'package:openmuse_appflowy_editor/src/editor/editor_component/service/keyboard_service_widget.dart'
    as keyboard;
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'built-in smart document and dataset open inside the actual Desktop Host',
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
      final centers = ValueNotifier<NativeResourceCenter?>(center);
      final checks = <String>[];
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
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
        await until(() {
          if (finder.hitTestable().evaluate().length != 1) return false;
          final w = t.widget(finder);
          if (w is IconButton) return w.onPressed != null;
          if (w is ButtonStyleButton) return w.onPressed != null;
          return true;
        });
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
            layoutController: layout,
            resources: centers,
          ),
        ),
      );
      await create(
        'io.openmuse.smart-document',
        'smart-document.document',
        'Host 文档.musedoc',
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      expect(
        t.getSize(find.byType(SmartDocumentView)).width,
        greaterThan(t.view.physicalSize.width / t.view.devicePixelRatio * .6),
      );
      final document = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      await until(() => find.byType(AppFlowyRichText).evaluate().isNotEmpty);
      await t.tap(find.byType(AppFlowyRichText).first);
      await t.pump(const Duration(milliseconds: 200));
      final clipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '正式宿主文档😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => document.blocks.first.delta!.toPlainText().contains('正式宿主'),
        );
      } finally {
        await Clipboard.setData(ClipboardData(text: clipboard?.text ?? ''));
      }
      checks.add(
        'production-built-in-creator-handler-and-native-keyboard-content',
      );
      document.editor.selection = Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tap('smart-document.link');
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        'https://example.com/host',
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(
          document.blocks.first.delta!.toJson(),
        ).contains('https://example.com/host'),
      );
      checks.add('original-hyperlink-dialog-and-domain-mark-inside-host-tab');
      const stableIndent = bool.fromEnvironment('SMARTDOC_STABLE_INDENT');
      String? indentedBlockId;
      String? exitedListBlockId;
      if (stableIndent) {
        await t.runAsync(() async {
          await document.insertParagraph('列表父块');
          await document.insertParagraph('稳定块缩进😀');
          await document.insertParagraph('同批第二块');
        });
        await until(() => document.blocks.length == 4);
        final ids = document.blocks.map((n) => n.id).toList();
        indentedBlockId = ids[2];
        final before = document.snapshot['data'] as Map;
        final textId = before['blocks'][ids[2]]['external_id'];
        final childrenId = before['blocks'][ids[2]]['children'];
        await t.tap(find.byType(AppFlowyRichText).last);
        document.editor.selection = Selection(
          start: Position(path: [3], offset: 1),
          end: Position(path: [2], offset: 1),
        );
        await t.pump(const Duration(milliseconds: 100));
        await t.sendKeyEvent(LogicalKeyboardKey.tab);
        await until(() => document.blocks.length == 2);
        expect(document.blocks[1].children.map((n) => n.id), [ids[2], ids[3]]);
        expect(document.editor.selection!.start.path, [1, 1]);
        final after = document.snapshot['data'] as Map;
        expect(after['blocks'][ids[2]]['external_id'], textId);
        expect(after['blocks'][ids[2]]['children'], childrenId);
        checks.add(
          'native-tab-moves-original-blocks-text-and-children-with-backward-selection',
        );
        await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.tab);
        await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
        await until(() => document.blocks.length == 4);
        expect(document.blocks.map((n) => n.id), ids);
        checks.add('native-shift-tab-restores-original-sibling-order');
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(() => document.blocks.length == 2);
        expect(document.blocks[1].children.first.id, ids[2]);
        checks.add('native-own-undo-restores-original-indented-identities');
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
        await t.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(() => document.blocks.length == 4);
        expect(document.blocks.map((n) => n.id), ids);
        // Persist an indented tree to exercise the actual Host close/reopen.
        await t.sendKeyEvent(LogicalKeyboardKey.tab);
        await until(() => document.blocks.length == 2);
        checks.add(
          'native-redo-and-second-indent-reuse-original-domain-history',
        );
        final size = t.view.physicalSize;
        try {
          for (final width in [320.0, 430.0]) {
            t.view.physicalSize = Size(
              width * t.view.devicePixelRatio,
              900 * t.view.devicePixelRatio,
            );
            await t.pump(const Duration(milliseconds: 300));
            expect(t.takeException(), isNull);
            if (width == 430) {
              await t.tap(find.text('编辑'));
              await t.pump(const Duration(milliseconds: 300));
              document.editor.selection = Selection(
                start: Position(path: [1, 1], offset: 1),
                end: Position(path: [1, 0], offset: 1),
              );
              await tap('smart-document.mobile.align');
              await tap('smart-document.mobile.减少缩进');
              await until(() => document.blocks.length == 4);
              expect(document.blocks.map((n) => n.id), ids);
              checks.add(
                'compact-toolbar-outdent-keeps-original-sibling-identities',
              );
              await tap('smart-document.mobile.增加缩进');
              await until(() => document.blocks.length == 2);
              expect(document.blocks[1].children.map((n) => n.id), [
                ids[2],
                ids[3],
              ]);
              checks.add(
                'compact-toolbar-indent-shares-domain-command-and-original-text',
              );
            }
            await shot('stable-indent-at${width.toInt()}');
          }
        } finally {
          t.view.physicalSize = size;
          await t.pump(const Duration(milliseconds: 200));
        }
        checks.add('native-indent-320-and430-renders-existing-editor');
      }
      const stableListExit = bool.fromEnvironment('SMARTDOC_STABLE_LIST_EXIT');
      if (stableListExit) {
        Future<void> domain(Future<void> Function() apply) async {
          bool done = false;
          Object? error;
          await t.runAsync(() async {
            apply().then(
              (_) => done = true,
              onError: (Object e) {
                error = e;
                done = true;
              },
            );
          });
          await until(() => done);
          if (error != null) throw error!;
        }

        await domain(() => document.insertParagraph(''));
        final index = document.blocks.length - 1;
        exitedListBlockId = document.blocks[index].id;
        document.editor.selection = Selection.collapsed(
          Position(path: [index]),
        );
        await domain(() => document.convertSelection('todo_list'));
        final original =
            (document.snapshot['data'] as Map)['blocks'][exitedListBlockId]
                as Map;
        Future<void> focusEmpty() async {
          await t.tap(find.byType(AppFlowyRichText).last);
          final node = document.resolveBlock(
            DocumentBlockReference(
              objectId: document.objectId,
              blockId: exitedListBlockId!,
            ),
          )!;
          document.editor.selection = Selection.collapsed(
            Position(path: node.path),
          );
          await t.pump(const Duration(milliseconds: 200));
        }

        Future<void> inputEnter() async {
          final input =
              (document.editor.service.keyboardService
                      as keyboard.KeyboardServiceWidgetState)
                  .textInputService;
          expect(input.attached, isTrue);
          // WidgetTester key events do not create an OS IME event. Exercise
          // the actual mounted input pipeline, as the existing code suite does.
          await domain(
            () => input.apply([
              const TextEditingDeltaInsertion(
                oldText: '',
                textInserted: '\n',
                insertionOffset: 0,
                selection: TextSelection.collapsed(offset: 1),
                composing: TextRange.empty,
              ),
            ]),
          );
        }

        await focusEmpty();
        await inputEnter();
        await until(() => document.blocks[index].type == 'paragraph');
        expect(document.blocks[index].id, exitedListBlockId);
        expect(
          (document.snapshot['data']
              as Map)['blocks'][exitedListBlockId]['external_id'],
          original['external_id'],
        );
        checks.add(
          'mounted-original-input-empty-root-list-Enter-retains-block-and-YText',
        );
        await tap('smart-document.undo');
        await until(() => document.blocks[index].type == 'todo_list');
        await tap('smart-document.redo');
        await until(() => document.blocks[index].type == 'paragraph');
        expect(document.blocks[index].id, exitedListBlockId);
        checks.add(
          'empty-root-list-Enter-original-undo-redo-keeps-same-identity',
        );
        document.editor.selection = Selection.collapsed(
          Position(path: [index]),
        );
        await domain(() => document.convertSelection('numbered_list'));
        await focusEmpty();
        await t.sendKeyEvent(LogicalKeyboardKey.tab);
        await until(
          () =>
              document
                  .resolveBlock(
                    DocumentBlockReference(
                      objectId: document.objectId,
                      blockId: exitedListBlockId!,
                    ),
                  )!
                  .path
                  .length ==
              2,
        );
        await focusEmpty();
        await inputEnter();
        await until(() => document.blocks.length == index + 1);
        expect(document.blocks[index].id, exitedListBlockId);
        expect(document.blocks[index].type, 'numbered_list');
        expect(
          (document.snapshot['data']
              as Map)['blocks'][exitedListBlockId]['external_id'],
          original['external_id'],
        );
        checks.add(
          'mounted-original-input-empty-nested-list-Enter-moves-same-block-one-level',
        );
        final clipboard = await Clipboard.getData(Clipboard.kTextPlain);
        try {
          await focusEmpty();
          await Clipboard.setData(const ClipboardData(text: '原列表退出后文字😀'));
          await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
          await t.sendKeyEvent(LogicalKeyboardKey.keyV);
          await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
          await until(
            () => document.blocks[index].delta!.toPlainText() == '原列表退出后文字😀',
          );
        } finally {
          await Clipboard.setData(ClipboardData(text: clipboard?.text ?? ''));
        }
        document.editor.selection = Selection.collapsed(
          Position(path: [index]),
        );
        await t.pump(const Duration(milliseconds: 100));
        await t.sendKeyEvent(LogicalKeyboardKey.backspace);
        await until(() => document.blocks[index].type == 'paragraph');
        expect(document.blocks[index].id, exitedListBlockId);
        expect(document.blocks[index].delta!.toPlainText(), '原列表退出后文字😀');
        checks.add(
          'native-block-start-Backspace-converts-without-replacing-nonempty-text',
        );
        await tap('smart-document.undo');
        await until(() => document.blocks[index].type == 'numbered_list');
        await tap('smart-document.redo');
        await until(() => document.blocks[index].type == 'paragraph');
        expect(document.blocks[index].id, exitedListBlockId);
        checks.add(
          'block-start-conversion-original-undo-redo-preserves-text-identity',
        );
        final size = t.view.physicalSize;
        try {
          for (final width in [320.0, 430.0]) {
            t.view.physicalSize = Size(
              width * t.view.devicePixelRatio,
              900 * t.view.devicePixelRatio,
            );
            await t.pump(const Duration(milliseconds: 250));
            expect(t.takeException(), isNull);
            await shot('stable-list-exit-at${width.toInt()}');
          }
        } finally {
          t.view.physicalSize = size;
          await t.pump(const Duration(milliseconds: 200));
        }
        checks.add(
          'exited-list-content-renders-at320-and430-with-existing-document-layout',
        );
      }
      final docRecord = workspace.activeTab!.resourceSession!.session.record;
      final docTab = workspace.activeTab!;
      await shot('host-document');
      workspace.closeTab(docTab);
      await until(() => !workspace.tabs.contains(docTab));
      checks.add('host-close-barrier-saves-resource-before-releasing-surface');
      await create(
        'io.openmuse.dataset',
        'dataset.database',
        'Host 数据库.musedataset',
      );
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      final dataset = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      final before = dataset.views.first.rows.length;
      await tap('dataset.add-row');
      await until(() => dataset.views.first.rows.length == before + 1);
      final savedRow = dataset.views.first.rows.last;
      final titleColumn = dataset.views.first.columns.first;
      await tap('dataset.cell.${savedRow.id}.${titleColumn.id}');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        '正式宿主数据库😀',
      );
      await tap('dataset.cell-apply');
      await until(
        () =>
            dataset.views.first.rows.last.cells[titleColumn.id]?.value ==
            '正式宿主数据库😀',
      );
      final savedDatasetId = dataset.objectId;
      final savedRowId = savedRow.id;
      final savedFieldId = titleColumn.id;
      final datasetRecord =
          workspace.activeTab!.resourceSession!.session.record;
      checks.add('independent-original-database-resource-and-add-row-in-host');
      final datasetTab = workspace.activeTab!;
      await shot('host-dataset');
      workspace.closeTab(datasetTab);
      await until(() => !workspace.tabs.contains(datasetTab));
      await center.controller.refresh();
      final context = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(context, center.controller, docRecord.reference()),
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      final reopened = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      expect(reopened.blocks.first.delta!.toPlainText(), '正式宿主文档😀');
      expect(
        jsonEncode(reopened.blocks.first.delta!.toJson()),
        contains('https://example.com/host'),
      );
      checks.add('host-resource-reopen-retains-text-and-hyperlink');
      if (stableIndent) {
        expect(reopened.blocks[1].children.first.id, indentedBlockId);
        expect(
          reopened.blocks[1].children.first.delta!.toPlainText(),
          '稳定块缩进😀',
        );
        checks.add('host-reopen-retains-original-indented-block-and-text');
      }
      if (stableListExit) {
        final node = reopened.resolveBlock(
          DocumentBlockReference(
            objectId: reopened.objectId,
            blockId: exitedListBlockId!,
          ),
        );
        expect(node!.type, 'paragraph');
        expect(node.delta!.toPlainText(), '原列表退出后文字😀');
        checks.add(
          'host-cold-reopen-retains-original-exited-list-block-text-and-type',
        );
      }
      final existingOwnerTab = workspace.activeTab!;
      await tap('pane-menu-button:editor');
      await t.tap(find.text('向下切分'));
      await t.pump(const Duration(milliseconds: 300));
      await tap('pane-menu-button:pane-1');
      await t.tap(find.text('绑定新编辑组'));
      await t.pump(const Duration(milliseconds: 300));
      final secondary = workspace.editorGroups.singleWhere(
        (g) => g.id != LocalWorkspaceController.primaryEditorGroupId,
      );
      expect(
        layout.snapshot.bindingFor('pane-1')?.surfaceRef,
        'host.editorGroup:${secondary.id}',
      );
      workspace.focusEditorGroup(secondary.id);
      unawaited(
        openResourcePage(context, center.controller, docRecord.reference()),
      );
      await until(() => find.byType(SmartDocumentView).evaluate().length == 2);
      final secondaryTab = workspace.activeTab!;
      final surfaces = t
          .widgetList<SmartDocumentView>(find.byType(SmartDocumentView))
          .toList();
      expect(surfaces.every((v) => identical(v.controller, reopened)), isTrue);
      final additional = surfaces.singleWhere((v) => v.presentation != null);
      expect(additional.editor, isNot(same(reopened.editor)));
      expect(
        additional.editor.service.selectionService,
        isNot(same(reopened.editor.service.selectionService)),
      );
      final selected = additional.editor;
      await until(
        () =>
            additional.presentation!.projectionCurrent &&
            reopened.primaryProjectionCurrent,
      );
      final secondFinder = find.byWidgetPredicate(
        (w) => w is SmartDocumentView && identical(w.editor, selected),
      );
      final secondRichText = find
          .descendant(of: secondFinder, matching: find.byType(AppFlowyRichText))
          .first;
      final paragraph = t.renderObject<RenderParagraph>(
        find
            .descendant(
              of: secondRichText,
              matching: find.byWidgetPredicate(
                (w) => w is RichText && w.text.toPlainText() == '正式宿主文档😀',
              ),
            )
            .first,
      );
      final unlinked = paragraph
          .getBoxesForSelection(
            const TextSelection(baseOffset: 4, extentOffset: 6),
          )
          .first;
      await t.tapAt(paragraph.localToGlobal(unlinked.toRect().center));
      await t.pump(const Duration(milliseconds: 150));
      expect(
        find.byKey(const ValueKey('smart-document.link-dialog')),
        findsNothing,
      );
      selected.selection = Selection.collapsed(
        Position(
          path: [0],
          offset: selected.document.root.children.first.delta!.length,
        ),
      );
      await t.pump();
      final previous = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '第二编辑面😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => reopened.blocks.first.delta!.toPlainText().contains('第二编辑面'),
        );
      } finally {
        await Clipboard.setData(ClipboardData(text: previous?.text ?? ''));
      }
      await until(
        () =>
            reopened.primaryProjectionCurrent &&
            additional.presentation!.projectionCurrent,
      );
      expect(
        reopened.editor.document.root.children.first.delta!.toPlainText(),
        contains('第二编辑面'),
      );
      expect(workspace.tabsFor(secondary.id), [secondaryTab]);
      checks.add(
        'cross-group-presentations-share-one-domain-owner-with-independent-editor-services',
      );
      checks.add(
        'native-secondary-editor-keyboard-projects-into-both-surfaces',
      );
      await shot('host-two-document-presentations');
      final primaryFinder = find.byWidgetPredicate(
        (w) => w is SmartDocumentView && identical(w.editor, reopened.editor),
      );
      final primaryState = t.state(primaryFinder),
          secondaryState = t.state(secondFinder);
      reopened.editor.selection = Selection.collapsed(
        Position(path: [0], offset: 1),
      );
      selected.selection = Selection(
        start: Position(path: [0], offset: 2),
        end: Position(path: [0], offset: 4),
      );
      final firstSelection = reopened.editor.selection,
          secondSelection = selected.selection;
      Future<void> switchView(String name) async {
        await tap('title-view');
        final item = find.ancestor(
          of: find.text(name),
          matching: find.byType(PopupMenuItem<String>),
        );
        await until(() => item.hitTestable().evaluate().length == 1);
        await t.tap(item);
        await t.pump(const Duration(milliseconds: 350));
      }

      await until(
        () =>
            !existingOwnerTab.resourceSession!.session.dirty &&
            !secondaryTab.resourceSession!.session.dirty,
      );
      final versionsBeforeLayout = (
        existingOwnerTab.resourceSession!.session.record.version,
        secondaryTab.resourceSession!.session.record.version,
      );
      await switchView('Agent 视图');
      await switchView('IDE 视图');
      expect(t.state(primaryFinder), same(primaryState));
      expect(t.state(secondFinder), same(secondaryState));
      expect(reopened.editor.selection, firstSelection);
      expect(selected.selection, secondSelection);
      expect(reopened.localPresentationEditors, hasLength(2));
      checks.add(
        'actual-view-menu-keeps-both-document-states-independent-selections-and-one-owner',
      );
      await tap('pane-menu-button:pane-1');
      await t.ensureVisible(find.text('重置默认布局'));
      await until(
        () => find.text('重置默认布局').hitTestable().evaluate().length == 1,
      );
      await t.tap(find.text('重置默认布局'));
      await t.pump(const Duration(milliseconds: 350));
      expect(t.state(primaryFinder), same(primaryState));
      expect(t.state(secondFinder), same(secondaryState));
      expect(workspace.tabsFor(secondary.id), [secondaryTab]);
      expect((
        existingOwnerTab.resourceSession!.session.record.version,
        secondaryTab.resourceSession!.session.record.version,
      ), versionsBeforeLayout);
      checks.add(
        'reset-layout-retains-nonempty-secondary-group-without-another-resource-checkpoint',
      );
      await shot('host-layout-reset-retains-two-document-presentations');
      final resumedRich = t.renderObject<RenderParagraph>(
        find
            .descendant(
              of: secondRichText,
              matching: find.byWidgetPredicate(
                (w) => w is RichText && w.text.toPlainText().contains('第二编辑面'),
              ),
            )
            .first,
      );
      final resumedText = selected.document.root.children.first.delta!
          .toPlainText();
      final resumedOffset = resumedText.indexOf('第二编辑面') + 1;
      final resumedBox = resumedRich
          .getBoxesForSelection(
            TextSelection(
              baseOffset: resumedOffset,
              extentOffset: resumedOffset + 2,
            ),
          )
          .first;
      await t.tapAt(resumedRich.localToGlobal(resumedBox.toRect().center));
      await t.pump(const Duration(milliseconds: 150));
      selected.selection = Selection.collapsed(
        Position(path: [0], offset: resumedText.length),
      );
      await t.pump();
      final layoutClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '布局切换后继续输入😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () =>
              reopened.blocks.first.delta!.toPlainText().contains('布局切换后继续输入'),
        );
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyZ);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () =>
              !reopened.blocks.first.delta!.toPlainText().contains('布局切换后继续输入'),
        );
        expect(reopened.blocks.first.delta!.toPlainText(), contains('第二编辑面'));
      } finally {
        await Clipboard.setData(
          ClipboardData(text: layoutClipboard?.text ?? ''),
        );
      }
      await until(
        () =>
            reopened.primaryProjectionCurrent &&
            additional.presentation!.projectionCurrent,
      );
      await t.tapAt(resumedRich.localToGlobal(resumedBox.toRect().center));
      await t.pump(const Duration(milliseconds: 150));
      selected.selection = Selection.collapsed(Position(path: [0], offset: 2));
      final placementWhileClearing = secondaryTab.resourceSession!.session
          .restorePresentationAfterPlacement();
      selected.selection = null;
      await placementWhileClearing;
      await t.pump(const Duration(milliseconds: 200));
      expect(selected.selection, isNull);
      selected.selection = Selection.collapsed(Position(path: [0], offset: 2));
      await t.sendKeyEvent(LogicalKeyboardKey.escape);
      await t.pump(const Duration(milliseconds: 200));
      expect(selected.selection, isNull);
      await switchView('Agent 视图');
      await switchView('IDE 视图');
      expect(selected.selection, isNull);
      checks.add(
        'secondary-native-input-and-own-keyboard-undo-continue-after-view-and-layout-changes',
      );

      workspace.closeTab(
        existingOwnerTab,
        groupId: LocalWorkspaceController.primaryEditorGroupId,
      );
      await until(() => find.byType(SmartDocumentView).evaluate().length == 1);
      expect(reopened.isClosed, isFalse);
      expect(additional.presentation!.canWrite, isTrue);
      expect(
        t.widget<SmartDocumentView>(find.byType(SmartDocumentView)).editor,
        same(selected),
      );
      checks.add(
        'closing-primary-surface-retains-secondary-domain-and-checkpoint-owner',
      );
      // Restore the seeded text through the surviving editor's own undo port.
      unawaited(additional.presentation!.history());
      await until(
        () => reopened.blocks.first.delta!.toPlainText() == '正式宿主文档😀',
      );
      await shot('host-surviving-document-presentation');
      final lastBinding = secondaryTab.resourceSession!;
      final finalParticipant = lastBinding.session.closeParticipant!;
      lastBinding.session.closeParticipant = ResourceCloseParticipant(
        prepare: (sessions) async =>
            _RejectedCheckpoint(await finalParticipant.prepare(sessions)),
      );
      workspace.closeTab(secondaryTab, groupId: secondary.id);
      await until(
        () =>
            find
                .byKey(const ValueKey('resource-keep-editing'))
                .hitTestable()
                .evaluate()
                .length ==
            1,
      );
      await shot('host-final-release-failure-retains-surface');
      await tap('resource-keep-editing');
      await until(() => !lastBinding.confirming);
      expect(reopened.isDisposed, isFalse);
      expect(additional.presentation!.canWrite, isTrue);
      expect(
        t.widget<SmartDocumentView>(find.byType(SmartDocumentView)).editor,
        same(selected),
      );
      checks.add(
        'injected-prepared-owner-checkpoint-failure-keeps-actual-host-surface-and-retryable-lease',
      );
      final failedState = t.state(secondFinder);
      await switchView('Agent 视图');
      await switchView('IDE 视图');
      expect(t.state(secondFinder), same(failedState));
      expect(additional.presentation!.canWrite, isTrue);
      checks.add(
        'failed-checkpoint-page-can-switch-views-without-late-release-or-editor-recreation',
      );
      lastBinding.session.closeParticipant = finalParticipant;
      workspace.closeTab(secondaryTab, groupId: secondary.id);
      await until(
        () =>
            find.byType(SmartDocumentView).evaluate().isEmpty &&
            reopened.isDisposed,
      );
      checks.add(
        'last-presentation-close-checkpoints-and-releases-domain-owner',
      );
      await tap('pane-menu-button:pane-1');
      await t.ensureVisible(find.text('重置默认布局'));
      await until(
        () => find.text('重置默认布局').hitTestable().evaluate().length == 1,
      );
      await t.tap(find.text('重置默认布局'));
      await t.pump(const Duration(milliseconds: 300));
      workspace.focusEditorGroup(LocalWorkspaceController.primaryEditorGroupId);
      unawaited(
        openResourcePage(context, center.controller, docRecord.reference()),
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      expect(
        workspace.tabs.any((tab) => tab.resource.uri == existing.uri),
        isTrue,
      );
      expect(t.takeException(), isNull);
      checks.add(
        'existing-workspace-resource-preserved-and-no-widget-exception',
      );
      await shot('host-reopen');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(SmartDocumentView).evaluate().isEmpty);
      await center.controller.refresh();
      final datasetContext = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(
          datasetContext,
          center.controller,
          datasetRecord.reference(),
        ),
      );
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      final reopenedDataset = t
          .widget<DatasetSurface>(find.byType(DatasetSurface))
          .controller;
      expect(reopenedDataset.objectId, savedDatasetId);
      expect(
        reopenedDataset.views.first.rows
            .singleWhere((r) => r.id == savedRowId)
            .cells[savedFieldId]
            ?.value,
        '正式宿主数据库😀',
      );
      checks.add(
        'host-database-reopen-retains-original-root-row-and-edited-cell',
      );
      await shot('host-dataset-reopen');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(DatasetSurface).evaluate().isEmpty);
      final linkContext = t.element(
        find.byKey(const ValueKey('desktop-resource-center')),
      );
      unawaited(
        openResourcePage(linkContext, center.controller, docRecord.reference()),
      );
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      final linkedDocument = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      linkedDocument.editor.selection = Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tap('smart-document.link');
      final resourceUri = ResourceReference(
        identity: datasetRecord.identity,
      ).toUri().toString();
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        resourceUri,
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(
          linkedDocument.blocks.first.delta!.toJson(),
        ).contains(resourceUri),
      );
      await shot('host-resource-link');
      final text = find
          .byWidgetPredicate(
            (w) => w is RichText && w.text.toPlainText() == '正式宿主文档😀',
          )
          .first;
      final hit = t.getTopLeft(text) + const Offset(8, 10);
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.tapAt(hit);
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      expect(
        workspace.activeTab!.resourceSession!.session.record.identity,
        datasetRecord.identity,
      );
      expect(
        workspace.tabs.where(
          (tab) =>
              tab.resourceSession?.session.record.identity ==
              docRecord.identity,
        ),
        hasLength(1),
      );
      checks.add(
        'host-original-href-opens-independent-database-through-resource-router',
      );
      await shot('host-resource-link-target');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      expect(
        t.widget<SmartDocumentView>(find.byType(SmartDocumentView)).controller,
        same(linkedDocument),
      );
      expect(linkedDocument.canWrite, isTrue);
      checks.add('source-document-owner-preserved-after-linked-resource-close');
      linkedDocument.editor.selection = Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tap('smart-document.link');
      await tap('smart-document.link-open');
      await until(() => find.byType(DatasetSurface).evaluate().isNotEmpty);
      expect(
        find.byKey(const ValueKey('smart-document.link-dialog')),
        findsNothing,
      );
      expect(
        workspace.activeTab!.resourceSession!.session.record.identity,
        datasetRecord.identity,
      );
      checks.add('link-dialog-finishes-before-host-resource-navigation');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(SmartDocumentView).evaluate().isNotEmpty);
      final foreign = ResourceReference(
        identity: ResourceIdentity(
          scope: 'different-account-$run',
          source: datasetRecord.identity.source,
          workspace: datasetRecord.identity.workspace,
          mount: datasetRecord.identity.mount,
          resource: datasetRecord.identity.resource,
        ),
      ).toUri().toString();
      linkedDocument.editor.selection = Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tap('smart-document.link');
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        foreign,
      );
      await tap('smart-document.link-apply');
      final tabCount = workspace.tabs.length;
      final denied = resourceErrorText(
        t.element(find.byType(SmartDocumentView)),
        const ResourceFailure('DENIED'),
      );
      await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
      await t.tapAt(t.getTopLeft(text) + const Offset(8, 10));
      await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
      await until(() => find.text(denied).evaluate().isNotEmpty);
      expect(workspace.tabs, hasLength(tabCount));
      expect(find.byType(DatasetSurface), findsNothing);
      expect(linkedDocument.canWrite, isTrue);
      checks.add('resource-href-does-not-grant-cross-account-access');
      await shot('host-resource-link-denied');
      workspace.closeTab(workspace.activeTab!);
      await until(() => find.byType(SmartDocumentView).evaluate().isEmpty);
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'status': 'passed-host-registration-slice',
          'checks': checks,
          'fullProductAcceptance': 'not-complete',
          'limits': [
            'Cloud peer multi-presentation, embedded documents and full product acceptance remain open',
          ],
        }),
      );
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      workspace.dispose();
      centers.dispose();
      layout.dispose();
      await center.close();
      for (final plugin in plugins) {
        await registry.deactivate(plugin.descriptor.id);
      }
      debugPrint('SMARTDOC_HOST_EVIDENCE=${evidence.path}');
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
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
