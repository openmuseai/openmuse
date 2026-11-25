import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

void main() {
  test(
    'mounted owner resolves relative and escaped links without crossing its root',
    () async {
      final root = await Directory.systemTemp.createTemp('viewer-relative-');
      addTearDown(() => root.delete(recursive: true));
      await Directory('${root.path}/docs').create();
      await File('${root.path}/docs/README.md').writeAsString('base');
      await File(
        '${root.path}/docs/中文.dart',
      ).writeAsString('const answer = 42;');
      await File('${root.path}/parent.txt').writeAsString('parent');
      await Link('${root.path}/escape').create(Directory.systemTemp.path);
      final provider = await FileTreeResourceProvider.open(
        root: root,
        index: MemoryResourceStateStore(),
        scope: 'account',
        workspace: 'workspace',
        deviceRef: 'desktop',
        deviceLabel: 'Desktop',
        workspaceLabel: 'Workspace',
        mountLabel: 'Files',
        generation: () => 1,
      );
      final hub = ResourceCenterHub(scope: 'account')..register(provider);
      addTearDown(hub.dispose);
      final access = hub.access('test');
      final docs = (await provider.list(
        'root',
        access,
      )).entries.firstWhere((e) => e.name == 'docs');
      final readme = (await provider.list(
        docs.nodeRef,
        access,
      )).entries.firstWhere((e) => e.name == 'README.md');
      final base = ResourceReference(identity: readme.deferredIdentity!);
      final result = await hub.locateRelative(
        base,
        '%E4%B8%AD%E6%96%87.dart#L1C4',
        access,
      );
      expect(result.matches.single.label, 'docs/中文.dart');
      expect(result.line, 1);
      expect(result.column, 4);
      expect(
        (await hub.resolve(
          result.matches.single.reference,
          access,
        )).content.bytes,
        isNotEmpty,
      );
      expect(
        (await hub.locateRelative(
          base,
          '../parent.txt',
          access,
        )).matches.single.label,
        'parent.txt',
      );
      await expectLater(
        hub.locateRelative(base, '../../outside.txt', access),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        hub.locateRelative(base, '../escape/file.txt', access),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
}
