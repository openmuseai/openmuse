import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';

void main() {
  late Directory root;
  late OpenMusePluginRegistry registry;
  late NativeResourceCenter center;
  setUp(() async {
    root = await Directory.systemTemp.createTemp('resource-ui-test-');
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (a, b) async => null),
    );
    registry.install(OpenMuseResourceNotesPlugin());
    await registry.activate('com.openmuse.resource-notes');
    center = await NativeResourceCenter.open(
      scope: 'test-account',
      deviceRef: 'device-1',
      deviceLabel: 'This phone',
      registry: registry,
      storageRoot: root,
      historyStore: MemoryResourceStateStore(),
    );
  });
  tearDown(() async {
    await center.close();
    registry.dispose();
    await root.delete(recursive: true);
  });
  test(
    'linked document handoff retains a stable private read-only snapshot',
    () async {
      final parent = await center.controller.create(
        center.controller.creators.firstWhere(
          (c) => c.spec.id == 'notes.markdown',
        ),
        center.documents,
        'root',
        'Parent.md',
        const {},
        'parent',
      );
      const content = ResourceSeed(bytes: [65, 66], mediaType: 'text/plain');
      final linked = await center.controller.materializeLinkedContent!(
        parent,
        '1234.txt',
        content,
      );
      final again = await center.controller.materializeLinkedContent!(
        parent,
        '1234.txt',
        content,
      );
      expect(again.identity, linked.identity);
      expect(linked.provenance!.toJson(), parent.reference(pin: true).toJson());
      expect(linked.readOnly, isTrue);
      expect(center.hub.visibleProviders, isNot(contains(center.snapshots)));
      expect(
        (await center.controller.resolve(
          linked.reference(pin: true),
        )).content.bytes,
        [65, 66],
      );
      expect(await center.controller.handlers(linked), isNotEmpty);
    },
  );
  test(
    'missing saved mounts do not disable resources and can recover',
    () async {
      final missing = Directory('${root.path}/missing-project');
      final existing = await Directory(
        '${root.path}/existing-project',
      ).create();
      final mounts = {
        missing.path: 'Disconnected project',
        existing.path: 'Available project',
      };
      await center.mountDirectories(mounts);
      expect(center.hub.visibleProviders, contains(center.documents));
      expect(
        center.hub.visibleProviders.where(
          (provider) => provider.source.mountLabel == 'Available project',
        ),
        hasLength(1),
      );
      expect(
        center.hub.visibleProviders.where(
          (provider) => provider.source.mountLabel == 'Disconnected project',
        ),
        isEmpty,
      );
      await missing.create();
      await center.mountDirectories(mounts);
      expect(
        center.hub.visibleProviders.where(
          (provider) => provider.source.mountLabel == 'Disconnected project',
        ),
        hasLength(1),
      );
    },
  );
  test('welcome resources seed once per account without duplicates', () async {
    await center.seedWelcome();
    final first = await center.documents.list(
      'root',
      center.hub.access('test'),
    );
    expect(first.entries, hasLength(2));
    expect(center.hub.recent, hasLength(2));
    expect(
      first.entries.map((entry) => entry.name),
      contains('OpenMuse Introduction.pptx'),
    );
    await center.seedWelcome();
    expect(
      (await center.documents.list('root', center.hub.access('test'))).entries,
      hasLength(2),
    );
    expect(center.hub.recent, hasLength(2));
  });

  Future<ResourceRecord> oldGuide() async {
    final record = await center.documents.create(
      parentRef: 'root',
      name: 'OpenMuse User Guide.sydoc',
      mediaType: 'application/vnd.openmuse.siyuan.document+zip',
      bytes: await File('test/fixtures/legacy-guide.sydoc').readAsBytes(),
      idempotencyKey: 'welcome-v1-OpenMuse-User-Guide.sydoc',
      access: center.hub.access('test'),
    );
    await File(
      '${center.root.path}/welcome-v1.json',
    ).writeAsString('{"version":1}');
    return record;
  }

  test(
    'illustrated guide upgrades only a pristine starter and keeps history',
    () async {
      final original = await oldGuide();
      await center.seedWelcome();
      final current = await center.documents.describe(
        original.identity.resource,
        center.hub.access('test'),
      );
      expect(current.identity, original.identity);
      expect(current.version, isNot(original.version));
      expect(
        (await center.documents.read(
          original.identity.resource,
          original.version,
          center.hub.access('test'),
        )).bytes,
        await File('test/fixtures/legacy-guide.sydoc').readAsBytes(),
      );
      await center.seedWelcome();
      expect(
        (await center.documents.list(
          'root',
          center.hub.access('test'),
        )).entries,
        hasLength(1),
      );
    },
  );

  test(
    'personal guide edits survive while the new edition is offered once',
    () async {
      final original = await oldGuide();
      final edited = await center.documents.commit(
        original.identity.resource,
        original.version,
        [1, 2, 3],
        'personal-edit',
        center.hub.access('test'),
      );
      await center.seedWelcome();
      await center.seedWelcome();
      expect(
        (await center.documents.describe(
          original.identity.resource,
          center.hub.access('test'),
        )).version,
        edited.version,
      );
      expect(
        (await center.documents.read(
          original.identity.resource,
          null,
          center.hub.access('test'),
        )).bytes,
        [1, 2, 3],
      );
      final entries = (await center.documents.list(
        'root',
        center.hub.access('test'),
      )).entries;
      expect(entries, hasLength(2));
      expect(
        entries.map((e) => e.name),
        contains('OpenMuse User Guide — 2026-11.sydoc'),
      );
    },
  );

  test(
    'missing starter is not recreated in an already seeded account',
    () async {
      await File(
        '${center.root.path}/welcome-v1.json',
      ).writeAsString('{"version":1}');
      await center.seedWelcome();
      await center.seedWelcome();
      expect(
        (await center.documents.list(
          'root',
          center.hub.access('test'),
        )).entries,
        isEmpty,
      );
    },
  );

  test(
    'interrupted old seed is resumed without overwriting its resource',
    () async {
      final original = await oldGuide();
      await File('${center.root.path}/welcome-v1.json').delete();
      await center.seedWelcome();
      final entries = (await center.documents.list(
        'root',
        center.hub.access('test'),
      )).entries;
      expect(entries, hasLength(2));
      expect(
        entries.where((e) => e.resource?.identity == original.identity),
        hasLength(1),
      );
    },
  );

  Widget host({
    double scale = 1,
    ResourceCenterController? controller,
  }) => MaterialApp(
    builder: (context, child) => MediaQuery(
      data: MediaQuery.of(
        context,
      ).copyWith(textScaler: TextScaler.linear(scale)),
      child: child!,
    ),
    locale: const Locale('en'),
    supportedLocales: const [Locale('en')],
    localizationsDelegates: [OpenMuseStringsDelegate(OpenMuseCatalog.builtIn)],
    home: Scaffold(
      body: ColoredBox(
        color: Colors.white,
        child: ResourceCenterView(controller: controller ?? center.controller),
      ),
    ),
  );
  Future<void> tap(WidgetTester tester, String key) async {
    await tester.pumpAndSettle();
    await tester.runAsync(() async {
      await tester.tap(find.byKey(ValueKey(key)));
      await Future<void>.delayed(const Duration(milliseconds: 100));
    });
    await tester.pumpAndSettle();
    debugPrint('UI step: $key');
  }

  testWidgets(
    'offline peer refresh does not block local documents or recent history',
    (tester) async {
      var remoteCalls = 0;
      final controller = ResourceCenterController(
        hub: center.hub,
        registry: registry,
        refreshSources: () async {
          remoteCalls++;
          throw const ResourceFailure('SOURCE_UNAVAILABLE');
        },
      );
      addTearDown(controller.dispose);
      await tester.pumpWidget(host(controller: controller));
      await tap(tester, 'resource-section-remote');
      await tap(tester, 'resource-refresh');
      expect(remoteCalls, 1);
      expect(controller.failure, isA<ResourceFailure>());
      await tap(tester, 'resource-section-local');
      await tap(tester, 'resource-refresh');
      expect(controller.failure, isNull);
      expect(
        find.byKey(ValueKey('resource-source-${center.documents.source.key}')),
        findsOneWidget,
      );
      await tap(tester, 'resource-section-recent');
      await tap(tester, 'resource-refresh');
      expect(controller.failure, isNull);
      expect(remoteCalls, 1);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );

  testWidgets(
    'create via installed plugin, edit actual bytes, record opened, rename stable identity',
    (tester) async {
      await tester.pumpWidget(host());
      await tester.pumpAndSettle();
      await tap(tester, 'resource-new');
      await tap(
        tester,
        'resource-creator-com.openmuse.resource-notes/notes.markdown',
      );
      await tester.enterText(
        find.byKey(const ValueKey('resource-create-name')),
        'Project brief.md',
      );
      await tap(tester, 'resource-create-confirm');
      // Wait for the real filesystem create/open event under parallel tests.
      for (
        var i = 0;
        i < 100 &&
            find
                .byKey(const ValueKey('resource-notes-content'))
                .evaluate()
                .isEmpty;
        i++
      ) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 20)),
        );
        await tester.pump();
      }
      expect(
        find.byKey(const ValueKey('resource-notes-content')),
        findsOneWidget,
      );
      await tap(tester, 'resource-edit');
      await tester.enterText(
        find.byKey(const ValueKey('resource-notes-content')),
        'A real edited document',
      );
      await tap(tester, 'resource-save');
      final record = center.hub.recent.single.record;
      expect(
        String.fromCharCodes(
          (await tester.runAsync(
            () => center.documents.read(
              record.identity.resource,
              null,
              center.hub.access('verify'),
            ),
          ))!.bytes,
        ),
        'A real edited document',
      );
      await tester.pageBack();
      await tester.pumpAndSettle();
      await tap(tester, 'resource-more-${record.identity.resource}');
      await tap(tester, 'resource-menu-rename');
      await tester.enterText(
        find.byKey(const ValueKey('resource-rename-input')),
        'Reviewed brief.md',
      );
      await tap(tester, 'resource-rename-save');
      final renamed = await tester.runAsync(
        () => center.documents.describe(
          record.identity.resource,
          center.hub.access('verify'),
        ),
      );
      expect(renamed!.identity, record.identity);
      expect(renamed.name, 'Reviewed brief.md');
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
      await tester.runAsync(center.close);
    },
  );
  testWidgets(
    'This Device expansion state never reads the list scroll offset',
    (tester) async {
      await tester.pumpWidget(host());
      await tap(tester, 'resource-section-local');
      final list = find.byType(ListView).first;
      // ScrollPosition stores a double under this ancestor identifier. An
      // ExpansionTile without its own PageStorageKey would read it as bool.
      PageStorage.of(
        tester.element(list),
      ).writeState(tester.element(list), 42.0);
      await tap(tester, 'resource-section-recent');
      await tap(tester, 'resource-section-local');
      expect(tester.takeException(), isNull);
      expect(find.text('Documents'), findsWidgets);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
    },
  );
  testWidgets(
    'workspace folders and deferred files paint above an opaque host',
    (tester) async {
      await tester.runAsync(() async {
        final dir = Directory('${root.path}/mounted/Research');
        await dir.create(recursive: true);
        await File('${dir.path}/Brief.md').writeAsString('# Brief');
        await center.mountDirectories({
          '${root.path}/mounted': 'Project files',
        });
      });
      await tester.pumpWidget(host());
      await tap(tester, 'resource-section-local');
      final source = center.hub.visibleProviders.firstWhere(
        (p) => p.source.mountLabel == 'Project files',
      );
      await tap(tester, 'resource-source-${source.source.key}');
      expect(find.text('Research'), findsOneWidget);
      await tester.runAsync(() async {
        await tester.tap(find.text('Research'));
        await Future<void>.delayed(const Duration(milliseconds: 100));
      });
      await tester.pumpAndSettle();
      expect(find.text('Brief.md'), findsOneWidget);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
    },
  );
  testWidgets(
    'plugin disable removes creator immediately; 320px and large text remain usable',
    (tester) async {
      tester.view.physicalSize = const Size(320, 740);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(
        MediaQuery(
          data: const MediaQueryData(textScaler: TextScaler.linear(1.5)),
          child: host(scale: 1.5),
        ),
      );
      await tester.pumpAndSettle();
      await tap(tester, 'resource-new');
      expect(
        find.byKey(
          const ValueKey(
            'resource-creator-com.openmuse.resource-notes/notes.markdown',
          ),
        ),
        findsOneWidget,
      );
      await registry.deactivate('com.openmuse.resource-notes');
      await tester.pumpAndSettle();
      expect(
        find.byKey(
          const ValueKey(
            'resource-creator-com.openmuse.resource-notes/notes.markdown',
          ),
        ),
        findsNothing,
      );
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
      await tester.runAsync(center.close);
    },
  );
}
