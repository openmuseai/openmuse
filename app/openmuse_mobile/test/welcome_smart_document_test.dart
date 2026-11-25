import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  for (final region in ['us', 'cn']) {
    test(
      'bundled $region guide routes to Smart Document and native core saves/reopens',
      () async {
        final root = await Directory.systemTemp.createTemp('welcome-native-');
        final core = NativeDocumentCore();
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (a, b) async => null,
          ),
        );
        NativeResourceCenter? center;
        SmartDocumentController? owner;
        try {
          for (final plugin in createNativeSmartDocumentPlugins(
            storageRoot: root,
            documentCore: core,
          )) {
            registry.install(plugin);
            await registry.activate(plugin.descriptor.id);
          }
          center = await NativeResourceCenter.open(
            scope: 'guide-$region',
            deviceRef: 'qa-device',
            deviceLabel: 'QA',
            registry: registry,
            storageRoot: root,
            seedWelcomeResources: true,
            welcomeRegion: region,
          );
          final entries = (await center.documents.list(
            'root',
            center.hub.access('qa'),
          )).entries;
          final guide = entries
              .singleWhere((e) => e.name.endsWith('.musedoc'))
              .resource!;
          expect(
            guide.mediaType,
            'application/vnd.openmuse.smart-document+json',
          );
          final handlers = await center.controller.handlers(
            guide,
            intent: ResourceOpenIntent.edit,
          );
          expect(handlers.first.spec.id, 'smart-document.document');
          final content = await center.documents.read(
            guide.identity.resource,
            null,
            center.hub.access('qa'),
          );
          final seed = (jsonDecode(utf8.decode(content.bytes)) as Map)
              .cast<String, dynamic>();
          expect(seed.keys.toSet(), {'profile', 'objectId', 'docState'});
          expect(seed['profile'], 'muse.smart-document/phase1');
          final file = File('${root.path}/native-guide-checkpoint.json');
          owner = await SmartDocumentController.open(
            core: core,
            store: FileDocumentStore(file),
            objectId: seed['objectId'] as String,
            initialDocState: seed['docState'] as String,
          );
          final projection = jsonEncode(owner.snapshot['data']);
          expect(
            projection,
            contains(
              region == 'cn'
                  ? 'https://openmuseai.com/zh/privacy'
                  : 'https://us.openmuseai.com/en/privacy/',
            ),
          );
          if (region == 'cn')
            expect(projection, isNot(contains('us.openmuseai.com')));
          const marker = 'Native guide persistence QA · 中文🙂';
          await owner.insertParagraph(marker);
          await owner.flush();
          expect(owner.failure, isNull);
          await owner.close();
          owner = null;
          owner = await SmartDocumentController.open(
            core: core,
            store: FileDocumentStore(file),
            objectId: seed['objectId'] as String,
          );
          expect(
            owner.blocks.any((b) => b.delta?.toPlainText() == marker),
            isTrue,
          );
          // Native document checkpoint only; full editor UI and cloud sync have separate acceptance.
          expect(
            (await center.documents.read(
              guide.identity.resource,
              null,
              center.hub.access('qa'),
            )).bytes,
            content.bytes,
          );
        } finally {
          await owner?.close();
          await center?.close();
          for (final descriptor in registry.descriptors.toList().reversed) {
            await registry.deactivate(descriptor.id);
          }
          registry.dispose();
          await root.delete(recursive: true);
        }
      },
    );
  }
}
