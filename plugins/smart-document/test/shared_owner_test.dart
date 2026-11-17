import 'dart:async';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'resource_scope_test.dart' as fixture;

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Directory directory;
  setUp(() async {
    directory = await Directory.systemTemp.createTemp('document-owner-test-');
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
    'a widget release paused before the owner preparation cannot save or consume the outside sibling after private tree close',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var attempts = 0;
      final first = fixture.session(
        seed.bytes,
        beforeCommit: () async {
          attempts++;
        },
      );
      final second = fixture.session(seed.bytes);
      final a = await plugin.acquireResourceSession(first),
          b = await plugin.acquireResourceSession(second);
      await a.controller.insertParagraph('准备前释放竞态😀');
      final hold = a.checkpoint.pauseForClose();
      final disposing = a.close();
      await Future<void>.delayed(Duration.zero);
      expect(await first.prepareToClose(privateRecoveryOnly: true), isTrue);
      expect(await first.finalizeToClose(), isTrue);
      hold.resume();
      await disposing;
      expect(attempts, 0);
      expect(b.controller.isDisposed, isFalse);
      expect(b.controller.canWrite, isTrue);
      await b.controller.insertParagraph('保留兄弟继续输入😀');
      await b.close();
      expect(attempts, 1);
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'content observer uses the shared original owner and retires after its final lease',
    () async {
      var opens = 0, stops = 0;
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
        contentObserverFactory: (_, controller) {
          opens++;
          expect(controller.isClosed, isFalse);
          return () => stops++;
        },
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes),
          second = fixture.session(seed.bytes);
      final a = await plugin.acquireResourceSession(first);
      final b = await plugin.acquireResourceSession(second);
      expect(identical(a.controller, b.controller), isTrue);
      expect(opens, 1);
      await a.close();
      expect(stops, 0);
      await b.controller.insertParagraph('剩余原会话');
      await b.close();
      expect(stops, 1);
      await plugin.deactivate();
      expect(stops, 1);
      first.dispose();
      second.dispose();
    },
  );
  test(
    'observer survives failed owner close and retires only after private checkpoint recovery',
    () async {
      final store = _ObserverFailureStore();
      var stops = 0;
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
        contentObserverFactory: (_, _) =>
            () => stops++,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final session = fixture.session(seed.bytes);
      final lease = await plugin.acquireResourceSession(session);
      store.reject = true;
      await expectLater(
        lease.controller.insertParagraph('原正文仍由同一观察者持有'),
        throwsA(isA<StateError>()),
      );
      await expectLater(lease.close(), throwsA(isA<StateError>()));
      expect(stops, 0);
      expect(lease.controller.isClosed, isFalse);
      store.reject = false;
      await lease.close();
      expect(stops, 1);
      await plugin.deactivate();
      session.dispose();
    },
  );
  test(
    'an editing listener can immediately release a newly activated lease without observing a partial lease',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes),
          second = fixture.session(seed.bytes);
      final a = await plugin.acquireResourceSession(first);
      Future<void>? immediateClose;
      void changed() {
        if (second.editing &&
            second.releaseForClose != null &&
            immediateClose == null) {
          immediateClose = second.releaseForClose!();
        }
      }

      second.addListener(changed);
      final b = await plugin.acquireResourceSession(second);
      expect(immediateClose, isNotNull);
      await immediateClose;
      expect(a.controller.isDisposed, isFalse);
      expect(a.controller.canWrite, isTrue);
      await b.close();
      await a.close();
      second.removeListener(changed);
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'a Widget release waiting on its prepared tree cannot consume a surviving sibling lease',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes),
          second = fixture.session(seed.bytes);
      final a = await plugin.acquireResourceSession(first),
          b = await plugin.acquireResourceSession(second);
      await a.controller.insertParagraph('等待树提交的编辑面');
      expect(await first.prepareToClose(), isTrue);
      final disposing = a.close();
      expect(await first.finalizeToClose(), isTrue);
      await disposing;
      expect(b.controller.canWrite, isTrue);
      expect(b.controller.isDisposed, isFalse);
      await b.controller.insertParagraph('兄弟面依然拥有原会话😀');
      await b.close();
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'fresh authority closes an ended lifetime privately before opening the same account resource again',
    () async {
      final store = fixture.Memory(),
          oldLifetime = ResourceSessionLifetime(),
          newLifetime = ResourceSessionLifetime();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var oldCommits = 0;
      final first = fixture.session(
        seed.bytes,
        lifetime: oldLifetime,
        beforeCommit: () async {
          oldCommits++;
        },
      );
      final old = await plugin.acquireResourceSession(first);
      await old.controller.insertParagraph('原账号寿命的私有恢复😀');
      oldLifetime.invalidate();
      final second = fixture.session(seed.bytes, lifetime: newLifetime);
      final next = await plugin.acquireResourceSession(second);
      expect(old.controller.isDisposed, isTrue);
      expect(oldCommits, 0);
      expect(next.controller, isNot(same(old.controller)));
      expect(next.controller.canWrite, isTrue);
      expect(next.controller.blocks.last.delta!.toPlainText(), '原账号寿命的私有恢复😀');
      await old.close();
      expect(next.controller.canWrite, isTrue);
      await next.close();
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'a different active lifetime cannot evict a live owner or bypass its scope boundary',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(
            seed.bytes,
            lifetime: ResourceSessionLifetime(),
          ),
          second = fixture.session(
            seed.bytes,
            lifetime: ResourceSessionLifetime(),
          );
      final old = await plugin.acquireResourceSession(first);
      await expectLater(
        plugin.acquireResourceSession(second),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'RESOURCE_SESSION_LIFETIME_MISMATCH',
          ),
        ),
      );
      expect(old.controller.canWrite, isTrue);
      expect(old.controller.isDisposed, isFalse);
      await old.close();
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'an already reserved acquisition waits for the close preparation and retains the same live owner',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes),
          second = fixture.session(seed.bytes);
      final initial = await plugin.acquireResourceSession(first),
          c = initial.controller;
      await c.insertParagraph('准备时已有打开请求😀');
      var joined = false;
      final pending = plugin.acquireResourceSession(second).then((lease) {
        joined = true;
        return lease;
      });
      expect(await first.prepareToClose(), isTrue);
      expect(joined, isFalse);
      expect(c.canWrite, isFalse);
      expect(await first.finalizeToClose(), isTrue);
      final next = await pending;
      expect(next.controller, same(c));
      expect(c.isDisposed, isFalse);
      expect(c.canWrite, isTrue);
      expect(second.record.version, first.record.version);
      first.dispose();
      await c.insertParagraph('解冻后才允许挂载和输入');
      await next.close();
      await initial.close();
      await plugin.deactivate();
      second.dispose();
    },
  );
  test(
    'an open queued after preparation carries the accepted version into a newly reopened owner',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var commits = 0;
      final first = fixture.session(
            seed.bytes,
            beforeCommit: () async {
              commits++;
            },
          ),
          second = fixture.session(
            seed.bytes,
            beforeCommit: () async {
              commits++;
            },
          );
      final initial = await plugin.acquireResourceSession(first),
          c = initial.controller;
      await c.insertParagraph('最终检查点的新内容😀');
      expect(await first.prepareToClose(), isTrue);
      final pending = plugin.acquireResourceSession(second);
      expect(await first.finalizeToClose(), isTrue);
      final next = await pending;
      expect(c.isDisposed, isTrue);
      expect(next.controller, isNot(same(c)));
      expect(next.controller.blocks.last.delta!.toPlainText(), '最终检查点的新内容😀');
      expect(second.record.version, first.record.version);
      expect(commits, 1);
      await next.close();
      expect(commits, 1);
      await initial.close();
      await plugin.deactivate();
      first.dispose();
      second.dispose();
    },
  );
  test(
    'a presentation released during asynchronous opening never receives a lease or leaves a phantom owner',
    () async {
      final gate = Completer<void>();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async {
          await gate.future;
          return fixture.Memory();
        },
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes);
      final opening = plugin.acquireResourceSession(first);
      final failed = expectLater(
        opening,
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'RESOURCE_SESSION_CLOSED',
          ),
        ),
      );
      first.dispose();
      gate.complete();
      await failed;
      expect(plugin.activeController(first.record.identity), isNull);
      final second = fixture.session(seed.bytes),
          next = await plugin.acquireResourceSession(second);
      expect(next.controller.canWrite, isTrue);
      await next.close();
      await plugin.deactivate();
      second.dispose();
    },
  );
  test(
    'ended account closes a prepared tree from private recovery without advancing the resource version',
    () async {
      final store = fixture.Memory(), lifetime = ResourceSessionLifetime();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var commits = 0;
      final source = fixture.session(
        seed.bytes,
        lifetime: lifetime,
        beforeCommit: () async {
          commits++;
        },
      );
      final lease = await plugin.acquireResourceSession(source);
      await lease.controller.insertParagraph('原账号的待恢复内容😀');
      lifetime.invalidate();
      expect(await source.prepareToClose(), isTrue);
      expect(await source.finalizeToClose(), isTrue);
      expect(lease.controller.isDisposed, isTrue);
      expect(commits, 0);
      expect(source.record.version.revision, 'v1');
      expect(store.data!['pendingLocalChanges'], isTrue);
      expect(source.dirty, isFalse);
      await lease.close();
      await plugin.deactivate();
      source.dispose();
    },
  );
  test(
    'deactivation cancels an abandoned prepared tree before releasing its owner',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final source = fixture.session(seed.bytes),
          lease = await plugin.acquireResourceSession(source);
      await lease.controller.insertParagraph('准备后退出插件');
      expect(await source.prepareToClose(), isTrue);
      expect(lease.controller.canWrite, isFalse);
      await plugin.deactivate();
      expect(lease.controller.isDisposed, isTrue);
      expect(await source.finalizeToClose(), isFalse);
      await lease.close();
      source.dispose();
    },
  );
  test(
    'failed prepared resource checkpoint restores the same editor and the successful retry stays frozen until release',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var reject = true, commits = 0;
      late SmartDocumentController c;
      final source = fixture.session(
        seed.bytes,
        beforeCommit: () async {
          commits++;
          expect(c.canWrite, isFalse);
          if (reject) throw StateError('Resource checkpoint unavailable');
        },
      );
      final lease = await plugin.acquireResourceSession(source);
      c = lease.controller;
      await c.insertParagraph('父树准备需要保留的输入😀');
      expect(await source.prepareToClose(), isFalse);
      expect(c.canWrite, isTrue);
      expect(plugin.activeController(source.record.identity), same(c));
      reject = false;
      expect(await source.prepareToClose(), isTrue);
      expect(c.canWrite, isFalse);
      await expectLater(
        c.insertParagraph('已冻结，不能再接收此命令'),
        throwsA(isA<SmartDocumentFailure>()),
      );
      source.cancelClosePreparation();
      expect(c.canWrite, isTrue);
      await c.insertParagraph('取消树准备后继续编辑');
      expect(await source.prepareToClose(), isTrue);
      expect(await source.finalizeToClose(), isTrue);
      expect(c.isDisposed, isTrue);
      await lease.close();
      expect(commits, 3);
      await plugin.deactivate();
      source.dispose();
    },
  );
  test(
    'parallel document leases share one domain owner and checkpoint after the first surface closes',
    () async {
      var stores = 0, providers = 0, realtime = 0;
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async {
          stores++;
          return fixture.Memory();
        },
        providerFactory: (_) {
          providers++;
          return null;
        },
        realtimeFactory: (_, _) async {
          realtime++;
          return null;
        },
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final first = fixture.session(seed.bytes),
          second = fixture.session(seed.bytes);
      final readBase = fixture.session(seed.bytes);
      final reader = ResourceEditorSession(
        opened: readBase.opened,
        allowEditing: false,
        commitContent: readBase.commitContent,
        onOpened: (_) {},
        onChanged: (_) {},
      );
      final leases = await Future.wait([
        plugin.acquireResourceSession(first),
        plugin.acquireResourceSession(second),
        plugin.acquireResourceSession(reader),
      ]);
      final c = leases.first.controller;
      expect(
        leases.every(
          (v) =>
              identical(v.controller, c) &&
              identical(v.checkpoint, leases.first.checkpoint),
        ),
        isTrue,
      );
      expect((stores, providers, realtime), (1, 1, 1));
      final a = c.openPresentation(allowEditing: leases[1].allowEditing),
          b = c.openPresentation(allowEditing: leases[2].allowEditing);
      expect(a.canWrite, isTrue);
      expect(b.canWrite, isFalse);
      await c.insertParagraph('同一内容 owner');
      await leases.first.close();
      expect(c.isClosed, isFalse);
      expect(c.canWrite, isTrue);
      first.dispose();
      await c.insertParagraph('首个入口关闭后仍可编辑');
      await leases[1].checkpoint.flush();
      expect(first.record.version, second.record.version);
      expect(second.bytes, first.bytes);
      await a.close();
      await leases[1].close();
      expect(c.isClosed, isFalse);
      await b.close();
      await leases[2].close();
      expect(c.isClosed, isTrue);
      expect(plugin.activeController(first.record.identity), isNull);
      await plugin.deactivate();
      second.dispose();
      reader.dispose();
      readBase.dispose();
    },
  );
  test(
    'late final checkpoint failure keeps its document lease, current content and retryable owner',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      late SmartDocumentController c;
      var commits = 0;
      final source = fixture.session(
        seed.bytes,
        beforeCommit: () async {
          commits++;
          if (commits == 1) await c.insertParagraph('初次保存期间接受的输入😀');
          if (commits == 2) {
            expect(c.canWrite, isFalse);
            expect(c.editor.editable, isFalse);
            throw StateError('Final resource checkpoint rejected');
          }
        },
      );
      final lease = await plugin.acquireResourceSession(source);
      c = lease.controller;
      await c.insertParagraph('先保存的原正文');
      await expectLater(lease.close(), throwsA(isA<StateError>()));
      expect(commits, 2);
      expect(c.canWrite, isTrue);
      expect(c.editor.editable, isTrue);
      expect(c.blocks.last.delta!.toPlainText(), '初次保存期间接受的输入😀');
      expect(plugin.activeController(source.record.identity), same(c));
      expect(lease.checkpoint.failure, isNotNull);
      await c.insertParagraph('保存失败后继续输入');
      await lease.close();
      expect(commits, 3);
      expect(c.isClosed, isTrue);
      expect(
        OpenMuseSmartDocumentPlugin.decode(source.bytes)['docState'],
        c.snapshot['docState'],
      );
      await plugin.deactivate();
      source.dispose();
    },
  );
  test(
    'ended account closes a dirty owner using private recovery without a resource commit',
    () async {
      final store = fixture.Memory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      var commits = 0;
      final lifetime = ResourceSessionLifetime();
      final source = fixture.session(
        seed.bytes,
        lifetime: lifetime,
        beforeCommit: () async {
          commits++;
        },
      );
      final lease = await plugin.acquireResourceSession(source),
          c = lease.controller;
      await c.insertParagraph('账号结束前的本机修改😀');
      final state = c.snapshot['docState'];
      lifetime.invalidate();
      expect(c.canRead, isFalse);
      expect(c.editor.editable, isFalse);
      await lease.close();
      expect(commits, 0);
      expect(store.data!['docState'], state);
      expect(store.data!['pendingLocalChanges'], isTrue);
      expect(c.isClosed, isTrue);
      expect(
        OpenMuseSmartDocumentPlugin.decode(source.bytes)['docState'],
        isNot(state),
      );
      await plugin.deactivate();
      source.dispose();
    },
  );
}

class _ObserverFailureStore extends fixture.Memory {
  bool reject = false;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    if (reject) throw StateError('synthetic private checkpoint failure');
    await super.write(value);
  }
}
