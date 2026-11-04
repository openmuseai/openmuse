import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/account_storage.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';

void main() {
  test('switching subjects clears Project and DSH workspace state', () async {
    final support = await Directory.systemTemp.createTemp('openmuse-account-');
    addTearDown(() => support.delete(recursive: true));
    final first = HostAccountStorage(
      supportPath: support.path,
      subject: 'user-one',
    );
    final second = HostAccountStorage(
      supportPath: support.path,
      subject: 'user-two',
    );
    expect(first.root, isNot(second.root));
    expect(first.dshHome, isNot(second.dshHome));

    await Directory(first.workspacePath).create(recursive: true);
    await File('${first.workspacePath}/private.md').writeAsString('private');
    final controller = LocalWorkspaceController(
      rootPath: first.workspacePath,
      initialResources: const [],
      mountStore: WorkspaceMountStore(File(first.mountsPath)),
      versionStore: LocalVersionStore(Directory(first.versionsPath)),
    );
    addTearDown(controller.dispose);
    await controller.initialize();
    expect(controller.search('private.md'), hasLength(1));

    await controller.switchStorage(
      rootPath: second.workspacePath,
      mountStore: WorkspaceMountStore(File(second.mountsPath)),
      versionStore: LocalVersionStore(Directory(second.versionsPath)),
    );
    expect(controller.rootPath, second.workspacePath);
    expect(controller.search('private.md'), isEmpty);
    expect(controller.tabs, isEmpty);
  });
}
