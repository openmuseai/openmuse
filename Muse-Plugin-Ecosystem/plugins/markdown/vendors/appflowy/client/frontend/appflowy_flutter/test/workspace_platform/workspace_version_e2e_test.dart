import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_persistence.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late Directory sandbox;
  late Directory project;
  late Directory workspaceStore;
  late Directory versionStore;

  setUp(() async {
    sandbox = await Directory.systemTemp.createTemp('openmuse-workspace-e2e-');
    project = await Directory('${sandbox.path}/project').create();
    workspaceStore =
        await Directory('${sandbox.path}/workspace-store').create();
    versionStore = await Directory('${sandbox.path}/version-store').create();
  });

  tearDown(() async {
    if (await sandbox.exists()) await sandbox.delete(recursive: true);
  });

  test('mount browse create reopen then version audit and diff', () async {
    final publishedRoots = <String>[];
    final publishedActive = <String?>[];
    final registry = MuseWorkspaceProviderRegistry()
      ..register(MuseLocalWorkspaceProvider());
    final persistence = MuseWorkspacePersistence(
      rootResolver: () async => workspaceStore,
    );
    var controller = MuseWorkspaceController(
      providers: registry,
      persistence: persistence,
      dshPublisher: (_, __, mounts, activeMountRef) async {
        if (mounts.isNotEmpty) publishedRoots.add(mounts.first.rootLocator);
        publishedActive.add(activeMountRef);
      },
    );

    await controller.open(accountSpaceRef: 'team-1', title: 'Team One');
    await controller.mountLocalDirectory(project.path);
    final root = controller.roots.values.single;
    final created = await controller.createFile(root, 'plan.md');
    await File(created.locator).writeAsString('# Plan\n\nOriginal\n');
    await controller.refresh(root);

    expect(controller.mounts, hasLength(1));
    expect(
      controller.children[root.entryRef]!.map((entry) => entry.name),
      contains('plan.md'),
    );
    expect(publishedRoots, contains(await project.resolveSymbolicLinks()));
    expect(controller.activeMountRef, controller.mounts.first.mountRef);
    expect(publishedActive.last, controller.mounts.first.mountRef);

    controller.dispose();
    controller = MuseWorkspaceController(
      providers: registry,
      persistence: persistence,
      dshPublisher: (_, __, ___, ____) async {},
    );
    await controller.open(accountSpaceRef: 'team-1', title: 'Team One');

    expect(controller.mounts, hasLength(1));
    expect(
      controller.activeMountRef,
      controller.mounts.first.mountRef,
      reason: 'the active Mount survives a Host restart',
    );
    final restoredRoot = controller.roots.values.single;
    expect(controller.expandedEntryRefs, contains(restoredRoot.entryRef));
    expect(
      controller.children[restoredRoot.entryRef]!.map((entry) => entry.name),
      contains('plan.md'),
    );

    final repository = MuseTextVersionRepository(
      rootResolver: () async => versionStore,
    );
    final versions = MuseTextVersionDiffService(repository: repository);
    final restoredFile = File('${project.path}/plan.md');
    await versions.saveVersion(restoredFile, message: 'Workspace baseline');
    await restoredFile.writeAsString(
      '# Plan\n\nExpanded\n\n- audit\n- diff viewer\n',
    );
    final comparison = await versions.compareWithLatest(restoredFile);

    expect(comparison, isNotNull);
    expect(comparison!.diff.payload.hunks, isNotEmpty);
    expect(comparison.diff.payload.hunks.first.semanticLabel, 'Plan');
    expect(comparison.diff.payload.additions, greaterThan(0));
    final audit = await repository.listAudit(restoredFile);
    expect(
      audit.map((event) => event.type),
      containsAll(['version.capture', 'comparison.create']),
    );
    controller.dispose();
  });
}
