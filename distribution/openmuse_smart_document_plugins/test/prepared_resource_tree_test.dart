import 'dart:async';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import '../../../plugins/smart-document/test/resource_scope_test.dart'
    as document;
import '../../../plugins/dataset/test/resource_test.dart' as database;

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test(
    'real document and database owners prepare the entire tree, recover a child failure, and retain a database surface outside that tree',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'prepared-resource-tree-',
      );
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => directory.path,
          );
      final docs = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => document.Memory(),
      );
      final databases = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => database.Memory(),
      );
      final rootSeed = await docs.createResource('smart-document.document', {}),
          childSeed = await docs.createResource('smart-document.document', {}),
          dbSeed = await databases.createResource('dataset.database', {});
      var checking = false, reject = true;
      late SmartDocumentResourceLease root, child;
      final commits = <String>[];
      final allFrozen = Completer<void>();
      Future<void> before(String id) async {
        await allFrozen.future;
        if (checking) {
          expect(root.controller.canWrite, isFalse);
          expect(child.controller.canWrite, isFalse);
          commits.add(id);
          if (id == 'child' && reject)
            throw StateError('Child resource checkpoint unavailable');
        }
      }

      final parentSource = document.session(
        rootSeed.bytes,
        resourceId: 'parent',
        beforeCommit: () => before('parent'),
      );
      final childSource = document.session(
        childSeed.bytes,
        resourceId: 'child',
        beforeCommit: () => before('child'),
      );
      final firstDb = database.session(dbSeed.bytes, resourceId: 'database');
      final secondDb = database.session(dbSeed.bytes, resourceId: 'database');
      final outsideDb = database.session(dbSeed.bytes, resourceId: 'database');
      root = await docs.acquireResourceSession(parentSource);
      child = await docs.acquireResourceSession(childSource);
      final first = await databases.acquireResourceSession(firstDb),
          second = await databases.acquireResourceSession(secondDb),
          outside = await databases.acquireResourceSession(outsideDb);
      expect(first.controller, same(second.controller));
      expect(first.controller, same(outside.controller));
      parentSource.addDependentSession(childSource);
      parentSource.addDependentSession(firstDb);
      childSource.addDependentSession(secondDb);
      await root.controller.insertParagraph('父文档应留在原会话😀');
      await child.controller.insertParagraph('子文档检查点失败后恢复');
      final beforeRows = first.controller.views.first.rows.length;
      await first.controller.addRow(cells: {'title': '两个嵌入与独立页的同一记录😀'});
      final preparing = parentSource.prepareToClose();
      for (
        var i = 0;
        i < 1000 &&
            (root.controller.canWrite ||
                child.controller.canWrite ||
                outside.controller.canWrite);
        i++
      ) {
        await Future<void>.delayed(const Duration(milliseconds: 2));
      }
      expect(
        root.controller.canWrite ||
            child.controller.canWrite ||
            outside.controller.canWrite,
        isFalse,
      );
      checking = true;
      allFrozen.complete();
      expect(await preparing, isFalse);
      expect(root.controller.isDisposed, isFalse);
      expect(child.controller.isDisposed, isFalse);
      expect(root.controller.canWrite, isTrue);
      expect(child.controller.canWrite, isTrue);
      expect(outside.controller.canWrite, isTrue);
      expect(child.controller.blocks.last.delta!.toPlainText(), '子文档检查点失败后恢复');
      reject = false;
      expect(await parentSource.prepareToClose(), isTrue);
      expect(
        root.controller.canWrite ||
            child.controller.canWrite ||
            outside.controller.canWrite,
        isFalse,
      );
      final queuedSource = database.session(
        dbSeed.bytes,
        resourceId: 'database',
      );
      var acquired = false;
      final disposingSelectedSurface = first.close();
      final queued = databases.acquireResourceSession(queuedSource).then((
        lease,
      ) {
        acquired = true;
        return lease;
      });
      await Future<void>.delayed(Duration.zero);
      expect(acquired, isFalse);
      expect(await parentSource.finalizeToClose(), isTrue);
      expect(root.controller.isDisposed, isTrue);
      expect(child.controller.isDisposed, isTrue);
      await disposingSelectedSurface;
      final later = await queued;
      expect(later.controller, same(outside.controller));
      expect(outside.controller.canWrite, isTrue);
      checking = false;
      parentSource.dispose();
      childSource.dispose();
      firstDb.dispose();
      secondDb.dispose();
      await outside.controller.addRow(cells: {'title': '树关闭后独立页继续编辑😀'});
      await outside.checkpoint.flush();
      expect(outside.controller.views.first.rows, hasLength(beforeRows + 2));
      expect(commits, containsAll(['parent', 'child']));
      await root.close();
      await child.close();
      await first.close();
      await second.close();
      await outside.close();
      await later.close();
      outsideDb.dispose();
      queuedSource.dispose();
      await docs.deactivate();
      await databases.deactivate();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            null,
          );
      await directory.delete(recursive: true);
    },
  );
}
