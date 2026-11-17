import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import '../../../plugins/smart-document/test/resource_scope_test.dart'
    as fixture;

class Harness {
  Harness(this.root, this.registry, this.plugin, this.center);
  final Directory root;
  final OpenMusePluginRegistry registry;
  final OpenMuseSmartDocumentPlugin plugin;
  final NativeResourceCenter center;
  static Future<Harness> open() async {
    final root = await Directory.systemTemp.createTemp(
      'document-embed-component-',
    );
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          (_) async => root.path,
        );
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final stores = <ResourceSessionScope, fixture.Memory>{};
    final plugin = OpenMuseSmartDocumentPlugin(
      storeFactory: (s) async =>
          stores.putIfAbsent(s.ownershipScope, fixture.Memory.new),
    );
    registry.install(plugin);
    await registry.activate(plugin.descriptor.id);
    final center = await NativeResourceCenter.open(
      scope: 'embed-component',
      deviceRef: 'test',
      deviceLabel: 'test',
      registry: registry,
      storageRoot: root,
    );
    return Harness(root, registry, plugin, center);
  }

  Future<ResourceRecord> create(String name) => center.controller.create(
    center.controller.creators.single,
    center.documents,
    'root',
    name,
    const {},
    name,
  );
  Future<ResourceEditorSession> session(
    ResourceRecord r, {
    bool readOnly = false,
  }) async {
    final opened = await center.controller.resolve(r.reference());
    return center.controller.createEditorSession(
      opened,
      (await center.controller.handlers(opened.content.record)).single,
      readOnly: readOnly,
    );
  }

  ResourceEmbedTarget target(ResourceEditorSession session) =>
      ResourceEmbedTarget(
        reference: session.record.reference(),
        surface: 'smart-document.document',
        selector: {
          'objectId':
              OpenMuseSmartDocumentPlugin.decode(session.bytes)['objectId']
                  as String,
        },
      );
  Widget app(
    ResourceEditorSession parent, {
    ResourceEditorSession? standalone,
  }) => MaterialApp(
    locale: const Locale('en'),
    supportedLocales: const [Locale('en')],
    localizationsDelegates: [OpenMuseStringsDelegate(OpenMuseCatalog.builtIn)],
    home: ResourceEmbeddingScope(
      controller: center.controller,
      child: Scaffold(
        body: Row(
          children: [
            Expanded(
              child: ResourceEditorSessionScope(
                session: parent,
                child: Builder(
                  builder: (context) => plugin.buildResourceEditor(
                    context,
                    'smart-document.document',
                    parent,
                  ),
                ),
              ),
            ),
            if (standalone != null)
              Expanded(
                child: ResourceEditorSessionScope(
                  session: standalone,
                  child: Builder(
                    builder: (context) => plugin.buildResourceEditor(
                      context,
                      'smart-document.document',
                      standalone,
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    ),
  );
  Future<void> close() async {
    await plugin.deactivate();
    await center.close();
    registry.dispose();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          const MethodChannel('plugins.flutter.io/path_provider'),
          null,
        );
    await root.delete(recursive: true);
  }
}

Future<void> until(WidgetTester t, bool Function() condition) async {
  for (var i = 0; i < 200; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 2)),
    );
    if (condition()) return;
  }
  fail('Bounded document embedding did not settle');
}

Future<T> domain<T>(WidgetTester t, Future<T> Function() action) async {
  var done = false;
  T? result;
  Object? error;
  await t.runAsync(() async {
    unawaited(
      action().then(
        (v) {
          result = v;
          done = true;
        },
        onError: (Object e) {
          error = e;
          done = true;
        },
      ),
    );
  });
  await until(t, () => done);
  if (error != null) throw error!;
  return result as T;
}

void main() {
  testWidgets(
    'A to B to A preserves both original reference blocks and refuses recursive mounting before another content lease',
    (t) async {
      t.view.physicalSize = const Size(1280, 1100);
      t.view.devicePixelRatio = 1;
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetDevicePixelRatio);
      final h = (await t.runAsync(Harness.open))!;
      late ResourceRecord a, b;
      await t.runAsync(() async {
        a = await h.create('A.musedoc');
        b = await h.create('B.musedoc');
        final sa = await h.session(a), sb = await h.session(b);
        final la = await h.plugin.acquireResourceSession(sa),
            lb = await h.plugin.acquireResourceSession(sb);
        await la.controller.insertDocumentView(h.target(sb));
        await lb.controller.insertDocumentView(h.target(sa));
        await la.close();
        await lb.close();
        sa.dispose();
        sb.dispose();
      });
      final source = (await t.runAsync(() => h.session(a)))!;
      await t.pumpWidget(h.app(source));
      await until(
        t,
        () => find
            .byKey(const ValueKey('resource-embed-retry'))
            .evaluate()
            .isNotEmpty,
      );
      expect(find.byType(SmartDocumentView), findsNWidgets(2));
      final views = t
          .widgetList<SmartDocumentView>(find.byType(SmartDocumentView))
          .toList();
      expect(
        views.every((v) => v.controller.blocks.last.type == 'link_preview'),
        isTrue,
      );
      expect(await domain(t, source.prepareToClose), isTrue);
      expect(await domain(t, source.finalizeToClose), isTrue);
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      source.dispose();
      await domain(t, h.close);
      expect(t.takeException(), isNull);
    },
  );
  testWidgets(
    'readonly parent embeds the same live child owner as its standalone writer and cannot regain editing on resize',
    (t) async {
      t.view.physicalSize = const Size(1280, 1100);
      t.view.devicePixelRatio = 1;
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetDevicePixelRatio);
      final h = (await t.runAsync(Harness.open))!;
      late ResourceRecord a, b;
      await t.runAsync(() async {
        a = await h.create('Parent.musedoc');
        b = await h.create('Child.musedoc');
        final sa = await h.session(a), sb = await h.session(b);
        final la = await h.plugin.acquireResourceSession(sa);
        await la.controller.insertDocumentView(h.target(sb));
        await la.close();
        sa.dispose();
        sb.dispose();
      });
      final parent = (await t.runAsync(() => h.session(a, readOnly: true)))!;
      final standalone = (await t.runAsync(() => h.session(b)))!;
      final id = OpenMuseSmartDocumentPlugin.decode(
        standalone.bytes,
      )['objectId'];
      await t.pumpWidget(h.app(parent, standalone: standalone));
      await until(
        t,
        () => find.byType(SmartDocumentView).evaluate().length == 3,
      );
      List<SmartDocumentView> childViews() => t
          .widgetList<SmartDocumentView>(find.byType(SmartDocumentView))
          .where((v) => v.controller.objectId == id)
          .toList();
      final views = childViews();
      expect(views, hasLength(2));
      final embedded = views.singleWhere((v) => v.presentation != null);
      final writer = views.singleWhere((v) => v.presentation == null);
      expect(embedded.controller, same(writer.controller));
      expect(embedded.editor, isNot(same(writer.editor)));
      expect(embedded.presentation!.canWrite, isFalse);
      expect(writer.controller.canWrite, isTrue);
      for (final width in [600.0, 1280.0]) {
        t.view.physicalSize = Size(width, 1100);
        await t.pump();
        await t.pump(const Duration(milliseconds: 100));
        expect(embedded.editor.editable, isFalse);
      }
      await domain(
        t,
        () => writer.controller.insertParagraph(
          'standalone writer',
          presentation: writer.presentation,
        ),
      );
      await until(t, () => embedded.presentation!.projectionCurrent);
      expect(
        embedded.editor.document.root.children.last.delta!.toPlainText(),
        'standalone writer',
      );
      await expectLater(
        embedded.controller.insertParagraph(
          'denied',
          presentation: embedded.presentation,
        ),
        throwsA(isA<SmartDocumentFailure>()),
      );
      expect(await domain(t, parent.prepareToClose), isTrue);
      expect(await domain(t, parent.finalizeToClose), isTrue);
      expect(writer.controller.isDisposed, isFalse);
      await t.pumpWidget(const SizedBox.shrink());
      await t.pump();
      parent.dispose();
      standalone.dispose();
      await domain(t, h.close);
      expect(t.takeException(), isNull);
    },
  );
}
