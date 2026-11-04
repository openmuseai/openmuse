import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

void main() {
  late Directory temp, workspace;
  late ResourceCenterHub hub;
  late FileTreeResourceProvider tree;
  setUp(() async {
    temp = await Directory.systemTemp.createTemp('resource-tree-test-');
    workspace = await Directory('${temp.path}/workspace').create();
    await File('${workspace.path}/proposal.md').writeAsString('Version one');
    await Directory('${workspace.path}/assets').create();
    await File('${workspace.path}/assets/notes.txt').writeAsString('Nested');
    hub = ResourceCenterHub(scope: 'account-1');
    tree = await FileTreeResourceProvider.open(
      root: workspace,
      index: FileResourceStateStore(File('${temp.path}/index.json')),
      scope: hub.scope,
      workspace: 'workspace-1',
      deviceRef: 'pc-1',
      deviceLabel: 'Studio',
      workspaceLabel: 'Project',
      mountLabel: 'Files',
      generation: () => hub.generation,
    );
    hub.register(tree);
  });
  tearDown(() async {
    await hub.dispose();
    await temp.delete(recursive: true);
  });
  test(
    'directory lazily advertises opaque file identities and preserves them across Host restart',
    () async {
      final page = await tree.list('root', hub.access('browse'));
      final file = page.entries.firstWhere((e) => e.name == 'proposal.md');
      expect(file.resource, isNull);
      expect(file.isDirectory, isFalse);
      expect(
        jsonEncode(page.entries.map((e) => e.toJson()).toList()),
        isNot(contains(workspace.path)),
      );
      final reopened = await FileTreeResourceProvider.open(
        root: workspace,
        index: FileResourceStateStore(File('${temp.path}/index.json')),
        scope: hub.scope,
        workspace: 'workspace-1',
        deviceRef: 'pc-1',
        deviceLabel: 'Studio',
        workspaceLabel: 'Project',
        mountLabel: 'Files',
        generation: () => hub.generation,
      );
      expect(
        (await reopened.list(
          'root',
          hub.access('browse'),
        )).entries.firstWhere((e) => e.name == 'proposal.md').deferredIdentity,
        file.deferredIdentity,
      );
      final folder = page.entries.firstWhere((e) => e.name == 'assets');
      expect(
        (await tree.list(
          folder.nodeRef,
          hub.access('browse'),
        )).entries.single.name,
        'notes.txt',
      );
    },
  );
  test(
    'mounted files reject symlinks and a pinned version never silently becomes current bytes',
    () async {
      final file = (await tree.list(
        'root',
        hub.access('browse'),
      )).entries.firstWhere((e) => e.name == 'proposal.md');
      final v1 = await tree.describe(file.nodeRef, hub.access('open'));
      await File('${workspace.path}/proposal.md').writeAsString('Version two');
      await expectLater(
        tree.read(file.nodeRef, v1.version, hub.access('open')),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'VERSION_UNAVAILABLE',
          ),
        ),
      );
      await File('${workspace.path}/proposal.md').delete();
      await Link(
        '${workspace.path}/proposal.md',
      ).create('${workspace.path}/assets/notes.txt');
      await expectLater(
        tree.read(file.nodeRef, null, hub.access('open')),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'UNSAFE_STORAGE',
          ),
        ),
      );
    },
  );
  test(
    'execution reads verified projection on PC, transcript has canonical link and no private path',
    () async {
      final file = (await tree.list(
        'root',
        hub.access('browse'),
      )).entries.firstWhere((e) => e.name == 'proposal.md');
      final record = await tree.describe(file.nodeRef, hub.access('open'));
      final manager = ResourceProjectionManager(
        Directory('${temp.path}/execution'),
      );
      final execution = ResourceExecutionCoordinator(
        hub: hub,
        projections: manager,
        extractText: (c) async => utf8.decode(c.bytes),
      );
      final result = await execution.prepare([
        record.reference(pin: true),
      ], 'send-1');
      expect(result['content'], contains('Version one'));
      expect(result['content'], contains('openmuse-resource://v1/'));
      expect(jsonEncode(result), isNot(contains(temp.path)));
      final files = await manager.root
          .list(recursive: true)
          .where((e) => e is File && e.path.contains('/input.'))
          .toList();
      expect(files, isNotEmpty);
      expect(await (files.single as File).readAsString(), 'Version one');
      expect(
        await execution.prepare([record.reference(pin: true)], 'send-1'),
        result,
      );
      await manager.clear();
      expect(await manager.root.list().toList(), isEmpty);
      final restored = await execution.prepare([
        record.reference(pin: true),
      ], 'send-2');
      expect(restored['content'], contains('Version one'));
      await execution.close();
    },
  );
  test(
    'unsupported context is a failure and does not leave an execution projection behind',
    () async {
      final file = (await tree.list(
        'root',
        hub.access('browse'),
      )).entries.firstWhere((e) => e.name == 'proposal.md');
      final record = await tree.describe(file.nodeRef, hub.access('open'));
      final manager = ResourceProjectionManager(
        Directory('${temp.path}/execution'),
      );
      final execution = ResourceExecutionCoordinator(
        hub: hub,
        projections: manager,
        extractText: (c) async => null,
      );
      await expectLater(
        execution.prepare([record.reference(pin: true)], 'send-1'),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'CONTEXT_UNAVAILABLE',
          ),
        ),
      );
      expect(await manager.root.list().toList(), isEmpty);
      await execution.close();
    },
  );
  test(
    'completed prepare history is bounded without exhausting a long-running Host',
    () async {
      final manager = ResourceProjectionManager(
        Directory('${temp.path}/execution'),
      );
      final execution = ResourceExecutionCoordinator(
        hub: hub,
        projections: manager,
        extractText: (c) async => utf8.decode(c.bytes),
      );
      final entry = (await tree.list(
        'root',
        hub.access('browse'),
      )).entries.firstWhere((e) => e.name == 'proposal.md');
      final record = await tree.describe(entry.nodeRef, hub.access('open'));
      for (var i = 0; i < 105; i++) {
        expect(
          (await execution.prepare([
            record.reference(pin: true),
          ], 'send-$i'))['content'],
          contains('Version one'),
        );
        await manager.clear();
      }
      await execution.close();
      expect(
        () => execution.prepare([record.reference(pin: true)], 'after-close'),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
  test(
    'closing during extraction refuses the result and removes its private projection',
    () async {
      final entered = Completer<void>(), text = Completer<String>();
      final manager = ResourceProjectionManager(
        Directory('${temp.path}/execution'),
      );
      final execution = ResourceExecutionCoordinator(
        hub: hub,
        projections: manager,
        extractText: (c) {
          entered.complete();
          return text.future;
        },
      );
      final entry = (await tree.list(
        'root',
        hub.access('browse'),
      )).entries.firstWhere((e) => e.name == 'proposal.md');
      final record = await tree.describe(entry.nodeRef, hub.access('open'));
      final pending = execution.prepare([record.reference(pin: true)], 'send');
      await entered.future;
      await execution.close();
      text.complete('Version one');
      await expectLater(pending, throwsA(isA<ResourceFailure>()));
      expect(await manager.root.list().toList(), isEmpty);
    },
  );
}
