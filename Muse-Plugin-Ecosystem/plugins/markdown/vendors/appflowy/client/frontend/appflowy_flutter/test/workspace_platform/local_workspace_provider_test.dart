import 'dart:io';

import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late Directory root;
  late MuseLocalWorkspaceProvider provider;
  late MuseWorkspaceMount mount;
  late MuseWorkspaceEntry rootEntry;

  setUp(() async {
    root =
        await Directory.systemTemp.createTemp('openmuse-workspace-provider-');
    provider = MuseLocalWorkspaceProvider();
    mount = MuseWorkspaceMount(
      mountRef: 'mount.test',
      providerId: MuseLocalWorkspaceProvider.providerId,
      bindingKey: 'test',
      displayName: 'Project',
      rootLocator: root.path,
      readOnly: false,
      order: 0,
    );
    rootEntry = await provider.bind(mount);
  });

  tearDown(() async {
    if (await root.exists()) await root.delete(recursive: true);
  });

  test('lists directories first and exposes capability-based entries',
      () async {
    await File('${root.path}/zeta.ts').writeAsString('const zeta = 1;');
    await Directory('${root.path}/alpha').create();
    await File('${root.path}/beta.md').writeAsString('# Beta');

    final entries =
        await provider.listChildren(mount: mount, parent: rootEntry);

    expect(entries.map((entry) => entry.name), ['alpha', 'beta.md', 'zeta.ts']);
    expect(entries.first.isDirectory, isTrue);
    expect(entries.last.isFile, isTrue);
    expect(
      entries.last.capabilities,
      containsAll({
        MuseWorkspaceCapability.contentRead,
        MuseWorkspaceCapability.contentWrite,
        MuseWorkspaceCapability.rename,
      }),
    );
  });

  test('creates imports renames and deletes through provider contract',
      () async {
    final created = await provider.createFile(
      mount: mount,
      parent: rootEntry,
      name: 'notes.md',
    );
    final folder = await provider.createDirectory(
      mount: mount,
      parent: rootEntry,
      name: 'docs',
    );
    final sourceRoot =
        await Directory.systemTemp.createTemp('openmuse-import-');
    addTearDown(() async {
      if (await sourceRoot.exists()) await sourceRoot.delete(recursive: true);
    });
    final source =
        await File('${sourceRoot.path}/brief.txt').writeAsString('brief');
    final imported = await provider.importFiles(
      mount: mount,
      parent: folder,
      files: [source],
    );
    final renamed = await provider.rename(
      mount: mount,
      entry: created,
      newName: 'renamed.md',
    );

    expect(await File(renamed.locator).readAsString(), isEmpty);
    expect(await File(imported.single.locator).readAsString(), 'brief');
    await provider.delete(mount: mount, entry: folder);
    expect(await Directory(folder.locator).exists(), isFalse);
  });

  test('rejects traversal names and root deletion', () async {
    expect(
      () => provider.createFile(
        mount: mount,
        parent: rootEntry,
        name: '../escape.txt',
      ),
      throwsFormatException,
    );
    expect(
      () => provider.delete(mount: mount, entry: rootEntry),
      throwsStateError,
    );
  });
}
