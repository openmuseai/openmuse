import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_host/src/host/workspace_mirror_service.dart';

void main() {
  test('mount metadata and paged children contain no local paths', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-mirror-');
    final workspace = LocalWorkspaceController(
      rootPath: root.path,
      initialResources: const [],
    );
    addTearDown(() async {
      workspace.dispose();
      await root.delete(recursive: true);
    });
    await Directory('${root.path}/folder').create();
    await File('${root.path}/file.md').writeAsString('private');
    final service = DesktopWorkspaceMirrorService(workspace);
    const base = (
      accountRef: 'account',
      deviceRef: 'browser',
      workspaceRef: 'workspace',
    );
    final mounts = await service.handle(
      WorkspaceMirrorQuery(
        operation: 'mounts',
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
      ),
    );
    expect(jsonEncode(mounts), isNot(contains(root.path)));
    final mount = (mounts['mounts']! as List).single as Map;
    final mountRef = mount['mountRef'] as String;
    final rootRef = mount['rootRef'] as String;
    final page1 = await service.handle(
      WorkspaceMirrorQuery(
        operation: 'children',
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        mountRef: mountRef,
        parentRef: rootRef,
        limit: 1,
      ),
    );
    expect(jsonEncode(page1), isNot(contains(root.path)));
    expect((page1['entries']! as List).length, 1);
    expect(page1['nextCursor'], isNotNull);
    final page2 = await service.handle(
      WorkspaceMirrorQuery(
        operation: 'children',
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        mountRef: mountRef,
        parentRef: rootRef,
        limit: 1,
        cursor: page1['nextCursor'] as String,
      ),
    );
    expect((page2['entries']! as List).length, 1);
    expect(page2['nextCursor'], isNull);
    final fileEntry = (page2['entries']! as List).single as Map;
    final resource = await service.readResource(
      WorkspaceMirrorResourceQuery(
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        resourceRef: fileEntry['resourceRef'] as String,
      ),
    );
    expect(utf8.decode(resource.bytes), 'private');
    expect(resource.mediaType, startsWith('text/markdown'));
    expect(
      () => service.handle(
        WorkspaceMirrorQuery(
          operation: 'children',
          accountRef: base.accountRef,
          deviceRef: base.deviceRef,
          workspaceRef: base.workspaceRef,
          mountRef: mountRef,
          parentRef: root.path,
        ),
      ),
      throwsFormatException,
    );
  });
}
