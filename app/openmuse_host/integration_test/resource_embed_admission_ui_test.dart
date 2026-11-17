import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart' as db;
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';

class DocumentMemory implements doc.DocumentStore {
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> data) async {
    value = data;
  }
}

class DatabaseMemory implements db.SnapshotStore {
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> data) async {
    value = data;
  }
}

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'native reference admission rejects invalid targets before owners and recovers via the original editors',
    (t) async {
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-embed-admission-native-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_ADMISSION_EVIDENCE=${evidence.path}');
      final checks = <String>[];
      final calls = <String, int>{
        'documentStore': 0,
        'databaseStore': 0,
        'provider': 0,
        'realtime': 0,
      };
      final document = OpenMuseSmartDocumentPlugin(
        storeFactory: (_) async {
          calls['documentStore'] = calls['documentStore']! + 1;
          return DocumentMemory();
        },
        providerFactory: (_) {
          calls['provider'] = calls['provider']! + 1;
          return null;
        },
        realtimeFactory: (_, _) async {
          calls['realtime'] = calls['realtime']! + 1;
          return null;
        },
      );
      final database = OpenMuseDatasetPlugin(
        core: db.NativeDatasetCore(),
        storeFactory: (_) async {
          calls['databaseStore'] = calls['databaseStore']! + 1;
          return DatabaseMemory();
        },
        collaborationFactory: (_) {
          calls['provider'] = calls['provider']! + 1;
          return null;
        },
        realtimeFactory: (_, _) async {
          calls['realtime'] = calls['realtime']! + 1;
          return null;
        },
      );
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      registry.install(document);
      registry.install(database);
      await t.runAsync(() async {
        await registry.activate(document.descriptor.id);
        await registry.activate(database.descriptor.id);
      });
      final center = (await t.runAsync(
        () => NativeResourceCenter.open(
          scope: 'admission-native',
          deviceRef: 'test',
          deviceLabel: 'Mac',
          registry: registry,
          storageRoot: root,
        ),
      ))!;
      Future<ResourceRecord> create(String id, String name) =>
          center.controller.create(
            center.controller.creators.singleWhere((c) => c.spec.id == id),
            center.documents,
            'root',
            name,
            {},
            name,
          );
      late ResourceRecord parentRecord, documentRecord, databaseRecord;
      late ResourceEditorSession parent;
      late ResourceEmbedTarget documentTarget, databaseTarget;
      await t.runAsync(() async {
        parentRecord = await create(
          'smart-document.document',
          'Parent.musedoc',
        );
        documentRecord = await create(
          'smart-document.document',
          'Child.musedoc',
        );
        databaseRecord = await create(
          'dataset.database',
          'Database.musedataset',
        );
        final opened = await center.controller.resolve(
          parentRecord.reference(),
        );
        parent = center.controller.createEditorSession(
          opened,
          (await center.controller.handlers(opened.content.record)).single,
        );
        documentTarget = (await document.resourceEmbedTargets(
          await center.controller.resolve(documentRecord.reference()),
          'smart-document.document',
        )).single.target;
        databaseTarget = (await database.resourceEmbedTargets(
          await center.controller.resolve(databaseRecord.reference()),
          'dataset.grid',
        )).single.target;
      });
      final recentBefore = center.hub.recent
          .map((r) => r.record.identity.key)
          .toList();
      final boundary = GlobalKey();
      final current = ValueNotifier<ResourceEmbedTarget>(
        ResourceEmbedTarget(
          reference: documentTarget.reference,
          surface: documentTarget.surface,
          selector: {'objectId': '00000000-0000-4000-8000-000000000099'},
        ),
      );
      final language = ValueNotifier<String>('en');
      await t.pumpWidget(
        ValueListenableBuilder<String>(
          valueListenable: language,
          builder: (context, locale, _) => MaterialApp(
            locale: Locale(locale),
            supportedLocales: const [Locale('en'), Locale('zh')],
            localizationsDelegates: [
              OpenMuseStringsDelegate(OpenMuseCatalog.builtIn),
              ...OpenMuseAuthLocalizations.localizationsDelegates,
            ],
            home: RepaintBoundary(
              key: boundary,
              child: Scaffold(
                appBar: AppBar(
                  title: const Text('OpenMuse · Reference admission'),
                ),
                body: ResourceEmbeddingScope(
                  controller: center.controller,
                  child: ResourceEditorSessionScope(
                    session: parent,
                    child: ValueListenableBuilder<ResourceEmbedTarget>(
                      valueListenable: current,
                      builder: (context, target, _) => Align(
                        alignment: Alignment.topLeft,
                        child: SizedBox(
                          width: 460,
                          // Failure feedback is tested in a small frame; the original
                          // editors use the product's 360px embed frame.
                          height:
                              target.key == documentTarget.key ||
                                  target.key == databaseTarget.key
                              ? 360
                              : 180,
                          child: ResourceEmbedScope.of(context)!.build(
                            context,
                            target,
                            const ResourceEmbedPolicy(allowEditing: true),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      );
      Future<void> until(bool Function() ready, String reason) async {
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 40));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 5)),
          );
          if (ready()) return;
        }
        fail(reason);
      }

      Future<void> shot(String name) async {
        await t.pump();
        final render =
            boundary.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        final image = await render.toImage(pixelRatio: 1);
        final data = (await image.toByteData(format: ui.ImageByteFormat.png))!;
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(data.buffer.asUint8List());
        image.dispose();
      }

      ResourceEmbedTarget invalid(ResourceEmbedTarget target, String key) =>
          ResourceEmbedTarget(
            reference: target.reference,
            surface: target.surface,
            selector: {
              ...target.selector,
              key: '00000000-0000-4000-8000-000000000099',
            },
          );
      Future<void> rejected(ResourceEmbedTarget target, String name) async {
        current.value = target;
        await until(
          () => find
              .byKey(const ValueKey('resource-embed-retry'))
              .evaluate()
              .isNotEmpty,
          '$name must report refusal',
        );
        expect(
          find.text(
            'The reference does not match this resource. Choose it again.',
          ),
          findsOneWidget,
        );
        expect(
          find.byKey(const ValueKey('resource-embed-retry')).hitTestable(),
          findsOneWidget,
        );
        expect(calls.values, everyElement(0));
        expect(
          center.hub.recent.map((r) => r.record.identity.key),
          recentBefore,
        );
        expect(document.activeController(documentRecord.identity), isNull);
        expect(database.activeController(databaseRecord.identity), isNull);
        await shot(name);
        await t.tap(find.byKey(const ValueKey('resource-embed-retry')));
        await until(
          () => find
              .byKey(const ValueKey('resource-embed-retry'))
              .evaluate()
              .isNotEmpty,
          'explicit retry',
        );
        expect(calls.values, everyElement(0));
        checks.add(
          '$name-visible-specific-failure-before-owner-store-provider-realtime-opened-or-checkpoint',
        );
        checks.add(
          '$name-explicit-native-retry-preserves-locator-and-has-no-owner-side-effects',
        );
      }

      // Replace the initial target before the first asynchronous resolve can publish.
      await rejected(
        invalid(documentTarget, 'objectId'),
        '01-document-object-mismatch',
      );
      await rejected(
        invalid(databaseTarget, 'objectId'),
        '02-database-object-mismatch',
      );
      await rejected(
        invalid(databaseTarget, 'viewId'),
        '03-database-view-mismatch',
      );
      language.value = 'zh';
      await t.pump();
      await t.pump(const Duration(milliseconds: 100));
      expect(find.text('引用目标与资源内容不一致，请重新选择。'), findsOneWidget);
      checks.add(
        'same-failure-surface-switches-language-without-creating-an-owner',
      );
      await shot('04-chinese-feedback');
      language.value = 'en';
      await t.pump();
      current.value = ResourceEmbedTarget(
        reference: parentRecord.reference(),
        surface: 'smart-document.document',
        selector: {
          'objectId':
              OpenMuseSmartDocumentPlugin.decode(parent.bytes)['objectId']
                  as String,
        },
      );
      await until(
        () => find
            .text(
              'This resource is already in the containing reference chain. Choose another resource.',
            )
            .evaluate()
            .isNotEmpty,
        'cycle-specific feedback',
      );
      expect(calls.values, everyElement(0));
      await t.tap(find.byKey(const ValueKey('resource-embed-retry')));
      await until(
        () => find
            .byKey(const ValueKey('resource-embed-retry'))
            .evaluate()
            .isNotEmpty,
        'cycle explicit retry',
      );
      expect(calls.values, everyElement(0));
      checks.add(
        'self-reference-refused-before-child-session-with-visible-specific-native-retry',
      );
      await shot('05-cycle-feedback');
      current.value = documentTarget;
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'valid original document editor',
      );
      expect(calls['documentStore'], 1);
      checks.add('corrected-document-target-opens-one-original-editor-owner');
      await shot('06-corrected-original-document');
      current.value = databaseTarget;
      await until(
        () => find.byType(db.DatasetSurface).evaluate().isNotEmpty,
        'valid original database editor',
      );
      expect(calls['databaseStore'], 1);
      checks.add(
        'corrected-database-target-opens-one-original-root-row-editor-owner',
      );
      await shot('07-corrected-original-database');
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      parent.dispose();
      await t.runAsync(() async {
        await document.deactivate();
        await database.deactivate();
        await center.close();
      });
      registry.dispose();
      current.dispose();
      language.dispose();
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'fullProductAcceptance': 'not-complete',
          'scope':
              'Native macOS generic resolver, original document/database local providers; Cloud sharing covered by the accompanying prior regression campaign',
          'calls': calls,
        }),
      );
      expect(t.takeException(), isNull);
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
