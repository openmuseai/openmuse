import 'dart:async';
import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';

class Memory implements DocumentStore {
  Map<String, dynamic>? data;
  @override
  Future<Map<String, dynamic>?> read() async => data;
  @override
  Future<void> write(Map<String, dynamic> value) async => data = value;
}

ResourceEditorSession session(
  List<int> bytes, {
  bool historical = false,
  bool allowEditing = true,
  String account = 'test',
  String resourceId = 'r',
  String revision = 'v1',
  Future<void> Function()? beforeCommit,
  ResourceSessionLifetime? lifetime,
}) {
  final record = ResourceRecord(
    identity: ResourceIdentity(
      scope: account,
      source: 'local',
      workspace: 'w',
      mount: 'm',
      resource: resourceId,
    ),
    version: ResourceVersion(revision: revision, digest: resourceDigest(bytes)),
    descriptor: MuseResourceDescriptorV1(
      resourceRef: resourceId,
      revision: revision,
      displayName: 'Resource',
      mediaType: smartDocumentMediaType,
      format: const MuseResourceFormat(
        formatId: 'text',
        confidence: MuseFormatConfidence.claimed,
      ),
      capabilities: const [
        MuseResourceCapability.describe,
        MuseResourceCapability.materialize,
      ],
      security: const MuseResourceSecurity(
        classification: MuseSecurityClassification.internal,
        activeContent: MuseActiveContent.unknown,
      ),
    ),
    sourceLabel: 'Local',
    location: const ['Documents'],
    parentRef: 'root',
    metadataRevision: 1,
  );
  return ResourceEditorSession(
    lifetime: lifetime,
    opened: OpenedResource(
      reference: ResourceReference(identity: record.identity),
      content: ResourceContent(record, bytes),
      fromSnapshot: historical,
    ),
    allowEditing: allowEditing,
    commitContent: (base, next, _) async {
      await beforeCommit?.call();
      return ResourceRecord.fromJson({
        ...record.toJson(),
        'descriptor': {
          ...record.descriptor.toJson(),
          'revision': '${base.revision}-next',
        },
        'version': ResourceVersion(
          revision: '${base.revision}-next',
          digest: resourceDigest(next),
        ).toJson(),
      });
    },
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Directory root;
  setUp(() async {
    root = await Directory.systemTemp.createTemp('smartdoc-resource-scope-');
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          (_) async => root.path,
        );
  });
  tearDown(() async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          null,
        );
    await root.delete(recursive: true);
  });
  test(
    'account ending during a checkpoint keeps the private document and closes its lease',
    () async {
      final entered = Completer<void>(), release = Completer<void>();
      final lifetime = ResourceSessionLifetime(), store = Memory();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final editor = session(
        seed.bytes,
        lifetime: lifetime,
        beforeCommit: () async {
          if (!entered.isCompleted) entered.complete();
          await release.future;
          if (!lifetime.active)
            throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
        },
      );
      final lease = await plugin.acquireResourceSession(editor);
      await lease.controller.insertParagraph(
        'Private document draft after account ended',
      );
      final closing = lease.close();
      await entered.future;
      lifetime.invalidate();
      release.complete();
      await closing;
      expect(lease.isClosed, isTrue);
      expect(store.data?['pendingLocalChanges'], isTrue);
      expect(editor.dirty, isTrue);
      final recovered = session(seed.bytes);
      final next = await plugin.acquireResourceSession(recovered);
      expect(
        next.controller.blocks.map((b) => b.delta?.toPlainText()).join(),
        contains('Private document draft after account ended'),
      );
      await next.close();
      recovered.dispose();
      await plugin.deactivate();
      editor.dispose();
    },
  );
  test(
    'live resolved observed version stays a live embedded reference; explicit snapshot remains pinned',
    () async {
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final s = session(seed.bytes);
      final live = OpenedResource(
        reference: s.record.reference(pin: true),
        content: s.opened.content,
      );
      final target = (await plugin.resourceEmbedTargets(
        live,
        'smart-document.document',
      )).single.target;
      expect(target.reference.pinned, isFalse);
      final snapshot = OpenedResource(
        reference: s.record.reference(pin: true),
        content: s.opened.content,
        fromSnapshot: true,
      );
      expect(
        (await plugin.resourceEmbedTargets(
          snapshot,
          'smart-document.document',
        )).single.target.reference.pinned,
        isTrue,
      );
      s.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'live recovery cannot replace concurrent historical previews or another account',
    () async {
      final stores = <ResourceSessionScope, Memory>{};
      var storeCalls = 0, draftCalls = 0, providerCalls = 0, realtimeCalls = 0;
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (s) async {
          storeCalls++;
          return stores.putIfAbsent(s.ownershipScope, Memory.new);
        },
        draftStoreFactory: (_) async {
          draftCalls++;
          return Memory();
        },
        providerFactory: (_) {
          providerCalls++;
          return null;
        },
        realtimeFactory: (_, _) async {
          realtimeCalls++;
          return null;
        },
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final live = session(seed.bytes),
          pin = session(seed.bytes, historical: true);
      final pin2 = session(seed.bytes, historical: true, revision: 'v2');
      final otherAccount = session(seed.bytes, account: 'other');
      final c = await plugin.openController(live);
      await c.insertParagraph('当前账号本机草稿😀');
      await c.flush();
      final calls = [storeCalls, draftCalls, providerCalls, realtimeCalls];
      final old = await plugin.openController(pin),
          old2 = await plugin.openController(pin2);
      expect([storeCalls, draftCalls, providerCalls, realtimeCalls], calls);
      expect(old.canWrite, isFalse);
      expect(
        old.blocks.map((b) => b.delta?.toPlainText()).join(),
        isNot(contains('本机草稿')),
      );
      expect(old2, isNot(same(old)));
      await expectLater(
        old.insertParagraph('不能写入'),
        throwsA(isA<SmartDocumentFailure>()),
      );
      final other = await plugin.openController(otherAccount);
      expect(
        other.blocks.map((b) => b.delta?.toPlainText()).join(),
        isNot(contains('本机草稿')),
      );
      await expectLater(
        plugin.openController(live),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        plugin.release(pin, c),
        throwsA(isA<ResourceFailure>()),
      );
      expect(plugin.activeController(live.record.identity), same(c));
      expect(
        plugin.activeController(
          pin.record.identity,
          snapshotVersion: pin.record.version,
        ),
        same(old),
      );
      await plugin.release(pin, old);
      await plugin.release(pin2, old2);
      await c.insertParagraph('历史关闭后仍可编辑');
      await plugin.release(otherAccount, other);
      await plugin.release(live, c);
      final reopened = await plugin.openController(live);
      expect(
        reopened.blocks.map((b) => b.delta?.toPlainText()).join(),
        contains('当前账号本机草稿😀'),
      );
      await expectLater(
        plugin.release(live, c),
        throwsA(isA<ResourceFailure>()),
      );
      expect(plugin.activeController(live.record.identity), same(reopened));
      await plugin.release(live, reopened);
      for (final s in [live, pin, pin2, otherAccount]) {
        s.dispose();
      }
      await plugin.deactivate();
    },
  );
  test('account lifetime terminally revokes a native document owner', () async {
    final plugin = OpenMuseSmartDocumentPlugin(
      storeFactory: (_) async => Memory(),
      draftStoreFactory: (_) async => Memory(),
    );
    final seed = await plugin.createResource('smart-document.document', {});
    final lifetime = ResourceSessionLifetime();
    final owner = session(seed.bytes, lifetime: lifetime);
    final c = await plugin.openController(owner);
    await c.insertParagraph('保留本机草稿');
    final state = c.snapshot['docState'];
    lifetime.invalidate();
    expect(c.canRead, isFalse);
    expect(c.canWrite, isFalse);
    c.acceptRealtimeAccess(canRead: true, canWrite: true, reason: 'late-grant');
    c.pauseCollaboration(true);
    c.pauseCollaboration(false);
    expect(c.canRead, isFalse);
    expect(c.canWrite, isFalse);
    expect(c.snapshot['docState'], state);
    await plugin.release(owner, c);
    owner.dispose();
    await plugin.deactivate();
  });
}
