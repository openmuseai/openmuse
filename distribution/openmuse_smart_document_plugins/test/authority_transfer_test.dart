import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';
import '../../../plugins/smart-document/test/resource_scope_test.dart'
    as document;
import '../../../plugins/dataset/test/resource_test.dart' as database;

class DocumentMemory extends document.Memory {
  bool reject = false;
  Completer<void>? entered, release;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    if (reject) throw StateError('Private state unavailable');
    entered?.complete();
    entered = null;
    await release?.future;
    await super.write(value);
  }
}

class DatabaseMemory extends database.Memory {
  bool reject = false;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    if (reject) throw StateError('Private database state unavailable');
    await super.write(value);
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Directory directory;
  setUp(() async {
    directory = await Directory.systemTemp.createTemp('resource-authority-');
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          (_) async => directory.path,
        );
  });
  tearDown(() async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          null,
        );
    await directory.delete(recursive: true);
  });
  test(
    'readonly-first document retains the same owner, projection and checkpoint; a fresh writer commits against its own current baseline',
    () async {
      final store = DocumentMemory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final decoded = OpenMuseSmartDocumentPlugin.decode(seed.bytes);
      final peer = await SmartDocumentController.open(
        core: NativeDocumentCore(),
        store: document.Memory(),
        objectId: decoded['objectId'] as String,
        initialDocState: decoded['docState'] as String,
      );
      await peer.editor.apply(
        peer.editor.transaction..insertText(peer.blocks.first, 0, '原始正文😀'),
      );
      List<int> export() => utf8.encode(
        jsonEncode({...decoded, 'docState': peer.snapshot['docState']}),
      );
      var readerCommits = 0;
      final readerSource = document.session(
        export(),
        allowEditing: false,
        beforeCommit: () async {
          readerCommits++;
        },
      );
      final reader = await plugin.acquireResourceSession(readerSource);
      final c = reader.controller, checkpoint = reader.checkpoint;
      final readView = c.openPresentation(allowEditing: reader.allowEditing);
      readView.editor.selection = Selection.collapsed(
        Position(path: [0], offset: 2),
      );
      expect(c.editable, isFalse);
      await peer.editor.apply(
        peer.editor.transaction..insertText(peer.blocks.first, 0, '前缀'),
      );
      final committedFrom = <String>[];
      late final ResourceEditorSession writerSource;
      writerSource = document.session(
        export(),
        revision: 'v2',
        beforeCommit: () async {
          committedFrom.add(writerSource.record.version.revision);
        },
      );
      final writer = await plugin.acquireResourceSession(writerSource);
      expect(writer.controller, same(c));
      expect(writer.checkpoint, same(checkpoint));
      expect(checkpoint.session, same(writerSource));
      expect(c.editable, isTrue);
      expect(c.isClosed, isFalse);
      expect(reader.allowEditing, isFalse);
      expect(readView.canWrite, isFalse);
      expect(
        readView.editor.document.root.children.first.delta!.toPlainText(),
        '前缀原始正文😀',
      );
      expect(readView.editor.selection!.start.offset, 4);
      await expectLater(
        c.insertParagraph('旧面不能写', presentation: readView),
        throwsA(isA<SmartDocumentFailure>()),
      );
      final writeView = c.openPresentation(allowEditing: writer.allowEditing);
      await c.insertParagraph('新写面😀', presentation: writeView);
      await writer.checkpoint.flush();
      expect(committedFrom.first, 'v2');
      expect(readerCommits, 0);
      expect(writerSource.record.version.revision, 'v2-next');
      await readView.close();
      await reader.close();
      readerSource.dispose();
      expect(
        plugin.hasActiveResourceOwner(writerSource.ownershipScope),
        isTrue,
      );
      expect(c.isDisposed, isFalse);
      await c.insertParagraph('旧面关闭后继续😀', presentation: writeView);
      await writeView.close();
      await writer.close();
      writerSource.dispose();
      expect(c.isDisposed, isTrue);
      expect(
        plugin.hasActiveResourceOwner(writerSource.ownershipScope),
        isFalse,
      );
      await peer.close();
      await plugin.deactivate();
    },
  );
  test(
    'readonly-first original dataset merges fresh Root/Row state and rebinds the stable checkpoint to the writer grant',
    () async {
      final store = DatabaseMemory();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('dataset.database', {});
      final bundle = OpenMuseDatasetPlugin.decode(seed.bytes);
      final peer = await DatasetController.open(
        core: NativeDatasetCore(),
        store: database.Memory(),
        objectId: bundle['objectId'] as String,
        initialBundle: bundle,
      );
      await peer.addRow(cells: {'title': '原始行😀'});
      List<int> export() => OpenMuseDatasetPlugin.exportBundle(peer.bundle);
      var readerCommits = 0;
      final readSource = database.session(
        export(),
        allowEditing: false,
        beforeCommit: () async {
          readerCommits++;
        },
      );
      final read = await plugin.acquireResourceSession(readSource);
      final c = read.controller, checkpoint = read.checkpoint;
      await peer.addRow(cells: {'title': '新版本同伴行😀'});
      final bases = <String>[];
      late final ResourceEditorSession writeSource;
      writeSource = database.session(
        export(),
        revision: 'v2',
        beforeCommit: () async {
          bases.add(writeSource.record.version.revision);
        },
      );
      final write = await plugin.acquireResourceSession(writeSource);
      expect(write.controller, same(c));
      expect(write.checkpoint, same(checkpoint));
      expect(checkpoint.session, same(writeSource));
      expect(c.canWrite, isTrue);
      expect(read.allowEditing, isFalse);
      expect(write.allowEditing, isTrue);
      expect(c.views.first.rows, hasLength(2));
      await c.addRow(cells: {'title': '正式写面新增😀'});
      await write.flush();
      expect(bases.first, 'v2');
      expect(readerCommits, 0);
      expect(writeSource.record.version.revision, 'v2-next');
      await read.close();
      readSource.dispose();
      expect(c.isDisposed, isFalse);
      await c.addRow(cells: {'title': '仍是同一原Root😀'});
      await write.close();
      writeSource.dispose();
      await peer.close();
      await plugin.deactivate();
    },
  );
  test(
    'document private-IO failure restores its reader and old checkpoint binding; retry can adopt the same fresh writer',
    () async {
      final store = DocumentMemory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = document.session(seed.bytes, allowEditing: false);
      var commits = 0;
      final next = document.session(
        seed.bytes,
        revision: 'v2',
        beforeCommit: () async {
          commits++;
        },
      );
      final reader = await plugin.acquireResourceSession(first);
      store.reject = true;
      await expectLater(plugin.acquireResourceSession(next), throwsStateError);
      expect(reader.controller.editable, isFalse);
      expect(reader.controller.isClosed, isFalse);
      expect(reader.checkpoint.session, same(first));
      expect(commits, 0);
      store.reject = false;
      final writer = await plugin.acquireResourceSession(next);
      expect(writer.controller, same(reader.controller));
      expect(writer.checkpoint, same(reader.checkpoint));
      expect(reader.controller.editable, isTrue);
      await reader.close();
      await writer.close();
      first.dispose();
      next.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'dataset private-IO failure preserves the original reader and permits a complete current-baseline retry',
    () async {
      final store = DatabaseMemory();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('dataset.database', {});
      final first = database.session(seed.bytes, allowEditing: false),
          next = database.session(seed.bytes, revision: 'v2');
      final reader = await plugin.acquireResourceSession(first);
      store.reject = true;
      await expectLater(plugin.acquireResourceSession(next), throwsStateError);
      expect(reader.controller.editable, isFalse);
      expect(reader.controller.isClosed, isFalse);
      expect(reader.checkpoint.session, same(first));
      store.reject = false;
      final writer = await plugin.acquireResourceSession(next);
      expect(writer.controller, same(reader.controller));
      expect(reader.controller.canWrite, isTrue);
      await reader.close();
      await writer.close();
      first.dispose();
      next.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'writer-first close retains its authorized owner for the reader until the last lease closes',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => DocumentMemory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = document.session(seed.bytes, allowEditing: false);
      final next = document.session(seed.bytes, revision: 'v2');
      final reader = await plugin.acquireResourceSession(first);
      final writer = await plugin.acquireResourceSession(next);
      final c = reader.controller;
      await writer.close();
      next.dispose();
      expect(c.isDisposed, isFalse);
      expect(next.isActive, isTrue);
      expect(next.isPresentationActive, isFalse);
      expect(reader.allowEditing, isFalse);
      expect(plugin.hasActiveResourceOwner(first.ownershipScope), isTrue);
      await reader.close();
      first.dispose();
      expect(c.isDisposed, isTrue);
      expect(next.isActive, isFalse);
      await plugin.deactivate();
    },
  );
  test(
    'queued concurrent writer opens and reader close share one completed authority handoff',
    () async {
      final store = DocumentMemory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = document.session(seed.bytes, allowEditing: false);
      final reader = await plugin.acquireResourceSession(first);
      final entered = Completer<void>(), release = Completer<void>();
      store.entered = entered;
      store.release = release;
      final next = document.session(seed.bytes, revision: 'v2');
      final pending = plugin.acquireResourceSession(next);
      await entered.future;
      final otherSource = document.session(seed.bytes, revision: 'v2');
      final otherPending = plugin.acquireResourceSession(otherSource);
      var closed = false;
      final closing = reader.close().then((_) => closed = true);
      await Future<void>.delayed(Duration.zero);
      expect(closed, isFalse);
      release.complete();
      final writer = await pending, other = await otherPending;
      await closing;
      first.dispose();
      expect(writer.controller, same(reader.controller));
      expect(other.controller, same(reader.controller));
      expect(writer.checkpoint.session, same(next));
      expect(writer.controller.isDisposed, isFalse);
      await writer.close();
      await other.close();
      next.dispose();
      otherSource.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'a disposed incoming grant cancels an in-progress handoff without changing reader authority',
    () async {
      final store = DocumentMemory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = document.session(seed.bytes, allowEditing: false);
      final reader = await plugin.acquireResourceSession(first);
      final entered = Completer<void>(), release = Completer<void>();
      store.entered = entered;
      store.release = release;
      final next = document.session(seed.bytes, revision: 'v2');
      final pending = plugin.acquireResourceSession(next);
      final rejected = expectLater(pending, throwsA(isA<ResourceFailure>()));
      await entered.future;
      next.dispose();
      release.complete();
      await rejected;
      expect(reader.controller.editable, isFalse);
      expect(reader.checkpoint.session, same(first));
      expect(reader.controller.isClosed, isFalse);
      await reader.close();
      first.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'plugin deactivation drains and cancels a pending authority preparation before closing its owner',
    () async {
      final store = DocumentMemory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = document.session(seed.bytes, allowEditing: false);
      final reader = await plugin.acquireResourceSession(first);
      final entered = Completer<void>(), release = Completer<void>();
      store.entered = entered;
      store.release = release;
      final next = document.session(seed.bytes, revision: 'v2');
      final pending = plugin.acquireResourceSession(next);
      final rejected = expectLater(pending, throwsA(isA<ResourceFailure>()));
      await entered.future;
      final closing = plugin.deactivate();
      release.complete();
      await rejected;
      await closing;
      expect(reader.controller.isDisposed, isTrue);
      expect(reader.isClosed, isTrue);
      expect(next.editing, isFalse);
      first.dispose();
      next.dispose();
    },
  );
}
