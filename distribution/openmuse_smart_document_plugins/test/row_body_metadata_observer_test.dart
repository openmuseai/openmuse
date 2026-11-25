import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart';
import 'authority_transfer_test.dart' as fixture;

Future<void> until(bool Function() ready) async {
  for (var i = 0; i < 500; i++) {
    await Future<void>.delayed(const Duration(milliseconds: 2));
    if (ready()) return;
  }
  fail('Original row-body metadata observation did not settle');
}

class Pair {
  final store = fixture.DatabaseMemory();
  late db.DatasetController database;
  late doc.SmartDocumentController document;
  late String row, view, documentId;
  bool active = true;
  final authority = ChangeNotifier();
  late RowBodyMetadataObserver observer;
  bool get empty => database.readRow(view, row).isDocumentEmpty!;
  Future<void> open() async {
    database = await db.DatasetController.open(
      core: db.NativeDatasetCore(),
      store: store,
      objectId: '6100d320-e04d-4b7f-8fc1-1013c7115554',
      syncInterval: null,
    );
    row = await database.addRow(cells: {'title': '原字段'});
    view = database.views.single.id;
    documentId = database.readRow(view, row).documentId!;
    document = await doc.SmartDocumentController.open(
      core: doc.NativeDocumentCore(),
      store: fixture.DocumentMemory(),
      objectId: documentId,
    );
    observer = RowBodyMetadataObserver(
      document: document,
      database: database,
      viewId: view,
      rowId: row,
      documentId: documentId,
      canObserve: () => active,
      authorityChanges: authority,
    );
  }

  Future<void> insert(String text) => document.editor.apply(
    document.editor.transaction..insertText(
      document.blocks.first,
      document.blocks.first.delta!.length,
      text,
    ),
  );
  Future<void> close() async {
    observer.dispose();
    store.reject = false;
    await document.close();
    await database.close();
    authority.dispose();
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test(
    'original text, own undo/redo and peer content update only the original row hint',
    () async {
      final p = Pair();
      await p.open();
      final root = (p.database.bundle['objects'] as List).singleWhere(
        (o) => o['collabType'] == 1,
      );
      final rootBytes = jsonEncode(root);
      expect(p.empty, isTrue);
      await p.insert('原正文😀');
      await until(() => !p.empty);
      expect(
        jsonEncode(
          (p.database.bundle['objects'] as List).singleWhere(
            (o) => o['collabType'] == 1,
          ),
        ),
        rootBytes,
      );
      expect(p.database.readRow(p.view, p.row).cells['title']!.value, '原字段');
      await p.document.history();
      await until(() => p.empty);
      await p.document.history(redo: true);
      await until(() => !p.empty);
      final peer = await doc.SmartDocumentController.open(
        core: doc.NativeDocumentCore(),
        store: fixture.DocumentMemory(),
        objectId: p.documentId,
        initialDocState: p.document.snapshot['docState'] as String,
      );
      await peer.editor.apply(
        peer.editor.transaction..insertText(peer.blocks.first, 0, '同伴'),
      );
      await p.document.applyRealtimeUpdate(
        base64Decode(peer.snapshot['docState'] as String),
      );
      await p.document.history();
      await until(() => !p.empty);
      expect(p.document.blocks.first.delta!.toPlainText(), '同伴');
      await peer.close();
      await p.close();
    },
  );
  test(
    'ended source and readonly document do not write an observed peer hint',
    () async {
      final p = Pair();
      await p.open();
      p.active = false;
      p.authority.notifyListeners();
      await p.insert('离开原来源后输入');
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(p.empty, isTrue);
      p.document.acceptRealtimeAccess(
        canRead: true,
        canWrite: false,
        reason: 'denied',
      );
      p.active = true;
      p.authority.notifyListeners();
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(p.empty, isTrue);
      p.document.acceptRealtimeAccess(
        canRead: true,
        canWrite: true,
        reason: 'granted',
      );
      await until(() => !p.empty);
      await p.close();
    },
  );
  test(
    'private hint checkpoint failure remains visible until an explicit successful retry',
    () async {
      final p = Pair();
      await p.open();
      p.store.reject = true;
      await p.insert('正文仍保留');
      await until(() => p.observer.failure != null);
      expect(p.document.blocks.first.delta!.toPlainText(), '正文仍保留');
      expect(p.database.persistenceFailure, isNotNull);
      p.observer.retry();
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(p.observer.failure, isNotNull);
      p.store.reject = false;
      await p.observer.retryCheckpoint();
      await until(() => p.observer.failure == null);
      expect(p.database.persistenceFailure, isNull);
      expect(p.empty, isFalse);
      await p.close();
    },
  );
  test(
    'retired observer and unrelated document identity cannot update the original row',
    () async {
      final p = Pair();
      await p.open();
      p.observer.dispose();
      await p.insert('监听结束');
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(p.empty, isTrue);
      final wrong = RowBodyMetadataObserver(
        document: p.document,
        database: p.database,
        viewId: p.view,
        rowId: p.row,
        documentId: p.database.objectId,
        canObserve: () => true,
      );
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(p.empty, isTrue);
      wrong.dispose();
      await p.close();
    },
  );
}
