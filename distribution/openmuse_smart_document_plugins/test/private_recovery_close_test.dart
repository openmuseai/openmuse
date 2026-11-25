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

class PrivateDatabaseStore extends database.Memory {
  bool reject = false;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    if (reject) throw StateError('Private database checkpoint unavailable');
    await super.write(value);
  }
}

class PrivateStore extends document.Memory {
  bool reject = false;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    if (reject) throw StateError('Private checkpoint unavailable');
    await super.write(value);
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test(
    'private close preserves accepted document and database state without another resource save; an outside database writer retains its dirty baseline',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'private-resource-tree-',
      );
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => directory.path,
          );
      final docStore = PrivateStore(), dbStore = database.Memory();
      final docs = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => docStore,
      );
      final databases = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => dbStore,
      );
      final docSeed = await docs.createResource('smart-document.document', {}),
          dbSeed = await databases.createResource('dataset.database', {});
      var attempts = 0, reject = true;
      Future<void> commit() async {
        attempts++;
        if (reject) throw StateError('Resource checkpoint unavailable');
      }

      final source = document.session(
        docSeed.bytes,
        resourceId: 'parent',
        beforeCommit: commit,
      );
      final embedded = database.session(
        dbSeed.bytes,
        resourceId: 'database',
        beforeCommit: commit,
      );
      final outsideSource = database.session(
        dbSeed.bytes,
        resourceId: 'database',
        beforeCommit: commit,
      );
      final parent = await docs.acquireResourceSession(source),
          first = await databases.acquireResourceSession(embedded),
          outside = await databases.acquireResourceSession(outsideSource);
      final remove = source.addDependentSession(embedded);
      await parent.controller.insertParagraph('本机恢复正文😀');
      await first.controller.addRow(cells: {'title': '恢复行😀'});
      final versions = (source.record.version, embedded.record.version);
      expect(await source.prepareToClose(), isFalse);
      final failedAttempts = attempts;
      expect(failedAttempts, greaterThan(0));
      expect(await source.prepareToClose(privateRecoveryOnly: true), isTrue);
      // Wait beyond the autosave interval while both owners are held.
      await Future<void>.delayed(const Duration(milliseconds: 320));
      expect(attempts, failedAttempts);
      final waitingPublicFlush = parent.checkpoint.flush();
      expect(await source.finalizeToClose(), isTrue);
      await waitingPublicFlush;
      await Future<void>.delayed(const Duration(milliseconds: 350));
      expect(attempts, failedAttempts);
      expect(parent.controller.isDisposed, isTrue);
      expect(outside.controller.isDisposed, isFalse);
      expect(outside.controller.canWrite, isTrue);
      expect(outsideSource.dirty, isTrue);
      expect((source.record.version, embedded.record.version), versions);
      expect(source.closeFinalized && embedded.closeFinalized, isTrue);
      await parent.close();
      await first.close();
      expect(attempts, failedAttempts);
      remove();
      source.dispose();
      embedded.dispose();
      // Reopen with original resource bytes: only the private store contains this input.
      final reopenedSource = document.session(
        docSeed.bytes,
        resourceId: 'parent',
        beforeCommit: commit,
      );
      final reopened = await docs.acquireResourceSession(reopenedSource);
      expect(reopened.controller.blocks.last.delta!.toPlainText(), '本机恢复正文😀');
      expect(reopenedSource.record.version, versions.$1);
      expect(
        await reopenedSource.prepareToClose(privateRecoveryOnly: true),
        isTrue,
      );
      expect(await reopenedSource.finalizeToClose(), isTrue);
      await reopened.close();
      reopenedSource.dispose();
      reject = false;
      await outside.controller.addRow(cells: {'title': '树外继续输入😀'});
      await outside.checkpoint.flush();
      expect(outsideSource.dirty, isFalse);
      expect(outsideSource.record.version, isNot(versions.$2));
      expect(outside.controller.views.first.rows, hasLength(2));
      await outside.close();
      outsideSource.dispose();
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
  test(
    'private document close with an outside writer does not retry identical rejected content on resume or status notifications',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'private-document-survivor-',
      );
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => directory.path,
          );
      final docs = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => document.Memory(),
      );
      final seed = await docs.createResource('smart-document.document', {});
      var attempts = 0, reject = true;
      Future<void> commit() async {
        attempts++;
        if (reject) throw StateError('Rejected checkpoint');
      }

      final first = document.session(seed.bytes, beforeCommit: commit),
          other = document.session(seed.bytes, beforeCommit: commit);
      final a = await docs.acquireResourceSession(first),
          b = await docs.acquireResourceSession(other);
      await a.controller.insertParagraph('accepted original input');
      await expectLater(a.flush(), throwsStateError);
      expect(attempts, 1);
      expect(await first.prepareToClose(privateRecoveryOnly: true), isTrue);
      expect(await first.finalizeToClose(), isTrue);
      first.dispose();
      a.controller.setRealtimeListening(true);
      a.controller.setRealtimeListening(false);
      await Future<void>.delayed(const Duration(milliseconds: 350));
      expect(attempts, 1);
      expect(other.dirty, isTrue);
      expect(b.controller.isDisposed, isFalse);
      await b.controller.insertParagraph('outside writer accepts new input');
      await Future<void>.delayed(const Duration(milliseconds: 350));
      expect(attempts, 2);
      await Future<void>.delayed(const Duration(milliseconds: 350));
      expect(attempts, 2);
      reject = false;
      await b.flush();
      expect(attempts, 3);
      await b.close();
      other.dispose();
      await docs.deactivate();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            null,
          );
      await directory.delete(recursive: true);
    },
  );
  test(
    'private IO failure prevents recovery closing, retains the same owner, and permits retry without a provider commit',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'private-close-failure-',
      );
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => directory.path,
          );
      final store = PrivateStore();
      final docs = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await docs.createResource('smart-document.document', {});
      var attempts = 0;
      final source = document.session(
        seed.bytes,
        beforeCommit: () async {
          attempts++;
          throw StateError('Provider unavailable');
        },
      );
      final lease = await docs.acquireResourceSession(source);
      store.reject = true;
      try {
        await lease.controller.insertParagraph('私有失败后仍保留😀');
      } on Object {
        /* Input remains recoverable in the same owner. */
      }
      expect(await source.prepareToClose(privateRecoveryOnly: true), isFalse);
      expect(lease.controller.isDisposed, isFalse);
      expect(lease.controller.isClosed, isFalse);
      expect(source.closeFinalized, isFalse);
      store.reject = false;
      expect(await source.prepareToClose(privateRecoveryOnly: true), isTrue);
      expect(await source.finalizeToClose(), isTrue);
      expect(store.data, isNotNull);
      expect(attempts, 0);
      await lease.close();
      source.dispose();
      await docs.deactivate();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            null,
          );
      await directory.delete(recursive: true);
    },
  );
  test(
    'an already dispatched resource commit drains before private close; recovery causes no new commit',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'private-inflight-close-',
      );
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => directory.path,
          );
      final docs = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => document.Memory(),
      );
      final seed = await docs.createResource('smart-document.document', {});
      final dispatched = Completer<void>(), accepted = Completer<void>();
      var attempts = 0;
      final source = document.session(
        seed.bytes,
        beforeCommit: () async {
          attempts++;
          dispatched.complete();
          await accepted.future;
        },
      );
      final lease = await docs.acquireResourceSession(source);
      await lease.controller.insertParagraph('原在途保存需要等待😀');
      final saving = lease.checkpoint.flush();
      await dispatched.future;
      var prepared = false;
      final preparing = source.prepareToClose(privateRecoveryOnly: true).then((
        v,
      ) {
        prepared = true;
        return v;
      });
      for (var i = 0; i < 100 && !lease.controller.isClosed; i++) {
        await Future<void>.delayed(const Duration(milliseconds: 2));
      }
      expect(lease.controller.isClosed, isTrue);
      expect(prepared, isFalse);
      accepted.complete();
      await saving;
      expect(await preparing, isTrue);
      expect(await source.finalizeToClose(), isTrue);
      expect(attempts, 1);
      expect(source.record.version.revision, 'v1-next');
      await lease.close();
      source.dispose();
      await docs.deactivate();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            null,
          );
      await directory.delete(recursive: true);
    },
  );
  test(
    'database private IO failure retains accepted rows and blocks the entire private close',
    () async {
      final store = PrivateDatabaseStore();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('dataset.database', {});
      var attempts = 0;
      final source = database.session(
        seed.bytes,
        beforeCommit: () async {
          attempts++;
          throw StateError('Provider rejected');
        },
      );
      final lease = await plugin.acquireResourceSession(source);
      store.reject = true;
      try {
        await lease.controller.addRow(cells: {'title': '本机失败恢复行😀'});
      } on Object {
        /* Accepted rows remain on the same original owner. */
      }
      expect(await source.prepareToClose(privateRecoveryOnly: true), isFalse);
      expect(lease.controller.isDisposed, isFalse);
      expect(lease.controller.isClosed, isFalse);
      expect(lease.controller.views.first.rows, hasLength(1));
      store.reject = false;
      expect(await source.prepareToClose(privateRecoveryOnly: true), isTrue);
      expect(await source.finalizeToClose(), isTrue);
      expect(attempts, 0);
      await lease.close();
      source.dispose();
      await plugin.deactivate();
    },
  );
}
