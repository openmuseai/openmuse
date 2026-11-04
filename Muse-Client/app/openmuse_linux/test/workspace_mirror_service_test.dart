import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'package:openmuse_linux/src/host/workspace_controller.dart';
import 'package:openmuse_linux/src/host/workspace_mirror_service.dart';

void main() {
  test(
    'remote mutations preserve bounds and move removed files to recoverable trash',
    () async {
      final account = await Directory.systemTemp.createTemp('mirror-action-');
      final root = await Directory('${account.path}/Workspace').create();
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      addTearDown(() async {
        workspace.dispose();
        await account.delete(recursive: true);
      });
      final service = DesktopWorkspaceMirrorService(workspace);
      Future<Map<String, Object?>> read(
        String op, {
        String? mount,
        String? parent,
      }) => service.handle(
        WorkspaceMirrorQuery(
          operation: op,
          accountRef: 'a',
          deviceRef: 'd',
          workspaceRef: 'w',
          mountRef: mount,
          parentRef: parent,
        ),
      );
      Future<void> act(String op, {String? node, String? name}) async {
        await service.act(
          WorkspaceMirrorAction(
            operation: op,
            accountRef: 'a',
            workspaceRef: 'w',
            nodeRef: node,
            name: name,
          ),
        );
      }

      final mounts = await read('mounts');
      final mount = (mounts['mounts'] as List).single as Map;
      final ref = mount['rootRef'] as String;
      await expectLater(
        () => act('new-file', node: ref, name: '../escape'),
        throwsFormatException,
      );
      await act('new-file', node: ref, name: 'remote.md');
      expect(await File('${root.path}/remote.md').exists(), isTrue);
      var page = await read('children', mount: ref, parent: ref);
      var file = (page['entries'] as List).single as Map;
      await act('rename', node: file['nodeRef'] as String, name: 'renamed.md');
      page = await read('children', mount: ref, parent: ref);
      file = (page['entries'] as List).single as Map;
      await act('trash', node: file['nodeRef'] as String);
      expect(await File('${root.path}/renamed.md').exists(), isFalse);
      expect(
        (await Directory(
          '${root.path}/.openmuse-trash',
        ).list().toList()).single.path,
        endsWith('-renamed.md'),
      );
      page = await read('children', mount: ref, parent: ref);
      final trash = (page['entries'] as List).cast<Map>().firstWhere((e) => e['name'] == '.openmuse-trash');
      final trashPage = await read('children', mount: ref, parent: trash['nodeRef'] as String);
      final trashed = (trashPage['entries'] as List).single as Map;
      expect(trashed['canRestore'], isTrue);
      await act('restore', node: trashed['nodeRef'] as String);
      expect(await File('${root.path}/renamed.md').exists(), isTrue);
      await expectLater(() => act('trash', node: ref), throwsFormatException);
      await act('new-folder', node: ref, name: 'folder');
      page = await read('children', mount: ref, parent: ref);
      final folder = (page['entries'] as List).cast<Map>().firstWhere(
        (e) => e['name'] == 'folder',
      );
      await Directory('${root.path}/folder').delete();
      await Link('${root.path}/folder').create(account.path);
      await expectLater(
        () =>
            act('new-file', node: folder['nodeRef'] as String, name: 'outside'),
        throwsFormatException,
      );
      await act('add-workspace', name: 'Phone Workspace');
      expect(workspace.mounts.length, 2);
      expect(
        await Directory('${account.path}/Workspaces/Phone Workspace').exists(),
        isTrue,
      );
    },
  );

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
    final search = await service.handle(
      WorkspaceMirrorQuery(
        operation: 'search',
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        searchText: 'FILE',
      ),
    );
    expect(jsonEncode(search), isNot(contains(root.path)));
    expect((search['entries'] as List).single['name'], 'file.md');
    final deck = await File(
      '../openmuse_mobile/test_assets/genoffice_standard_business.pptx',
    ).copy('${root.path}/Deck.pptx');
    final deckSearch = await service.handle(
      WorkspaceMirrorQuery(
        operation: 'search',
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        searchText: 'Deck.pptx',
      ),
    );
    final deckEntry = (deckSearch['entries'] as List).single as Map;
    final deckResource = await service.readResource(
      WorkspaceMirrorResourceQuery(
        accountRef: base.accountRef,
        deviceRef: base.deviceRef,
        workspaceRef: base.workspaceRef,
        resourceRef: deckEntry['resourceRef'] as String,
      ),
    );
    expect(
      deckResource.mediaType,
      'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    );
    expect(deckResource.bytes.length, await deck.length());

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
