import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'resource_scope_test.dart' as fixture;

class FailedDraft implements DocumentStore {
  bool fail = true;
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> record) async {
    if (fail) throw StateError('private replacement draft unavailable');
    value = record;
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test(
    'a clean resource checkpoint cannot close over an unsaved private media selection',
    () async {
      final root = await Directory.systemTemp.createTemp('media-close-');
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(
            const MethodChannel('plugins.flutter.io/path_provider'),
            (_) async => root.path,
          );
      final pending = FailedDraft();
      final png = await File(
        '../../../Muse-Plugin-Ecosystem/plugins/markdown/smart_document/flutter/test/fixtures/test-image.png',
      ).readAsBytes();
      final plugin = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async => fixture.Memory(),
        draftStoreFactory: (_) async => fixture.Memory(),
        replacementDraftStoreFactory: (_) async => pending,
        imagePicker: () async => PickedDocumentImage(png, 'replacement.png'),
      );
      final seed = await plugin.createResource('smart-document.document', {});
      final s = fixture.session(seed.bytes);
      final lease = await plugin.acquireResourceSession(s);
      ResourceEditorSession? reopened;
      try {
        final c = lease.controller;
        final asset = AssetReference(
          provider: 'muse.local-assets/1',
          scope: 'fixture',
          owner: c.objectId,
          key: 'a' * 64,
          sha256: 'a' * 64,
          mediaType: 'image/png',
          byteLength: 100,
        );
        await c.insertImage(
          blockId: 'image-before-close',
          asset: asset,
          width: 100,
          height: 50,
        );
        await lease.flush();
        final media = plugin.mediaFor(s.record.identity)!;
        final target = DocumentBlockReference(
          objectId: c.objectId,
          blockId: 'image-before-close',
        );
        await expectLater(media.replace(target), throwsStateError);
        expect(media.hasPending, isTrue);
        expect(s.dirty, isFalse);
        expect(await s.prepareToClose(), isFalse);
        expect(c.isDisposed, isFalse);
        expect(c.canWrite, isTrue);
        pending.fail = false;
        expect(await s.prepareToClose(), isTrue);
        expect(await s.finalizeToClose(), isTrue);
        expect(c.isDisposed, isTrue);
        expect(pending.value!['draft'], isNotNull);
        reopened = fixture.session(s.bytes);
        final next = await plugin.acquireResourceSession(reopened);
        final restored = plugin.mediaFor(reopened.record.identity)!;
        expect(restored.hasPending, isTrue);
        await restored.retry();
        expect(restored.hasPending, isFalse);
        expect(
          next.controller.blocks.where((b) => b.type == 'image'),
          hasLength(1),
        );
        await next.close();
      } finally {
        s.cancelClosePreparation();
        pending.fail = false;
        await lease.close();
        await plugin.deactivate();
        s.dispose();
        reopened?.dispose();
        TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
            .setMockMethodCallHandler(
              const MethodChannel('plugins.flutter.io/path_provider'),
              null,
            );
        await root.delete(recursive: true);
      }
    },
  );
}
