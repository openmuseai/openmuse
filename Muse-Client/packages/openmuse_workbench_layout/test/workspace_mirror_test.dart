import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

final class _Port implements WorkspaceMirrorPort {
  final requests = <(String, String, String?)>[];
  Future<WorkspaceMirrorPage> Function(String, String?)? reply;

  @override
  Future<List<WorkspaceMirrorMount>> listMounts(String workspaceRef) async => [
    const WorkspaceMirrorMount(
      mountRef: 'mount',
      rootRef: 'root',
      title: 'Desktop Workspace',
    ),
  ];

  @override
  Future<WorkspaceMirrorPage> listChildren({
    required String workspaceRef,
    required String mountRef,
    required String parentRef,
    required int limit,
    String? cursor,
  }) {
    requests.add((workspaceRef, parentRef, cursor));
    return reply!(parentRef, cursor);
  }
}

void main() {
  test('directory pages load only on expand and explicit load more', () async {
    final port = _Port()
      ..reply = (parent, cursor) async {
        if (parent == 'root' && cursor == null) {
          return const WorkspaceMirrorPage(
            entries: [
              WorkspaceMirrorEntry(
                nodeRef: 'src',
                name: 'src',
                isDirectory: true,
              ),
            ],
            nextCursor: 'page-2',
          );
        }
        if (parent == 'root') {
          return const WorkspaceMirrorPage(
            entries: [
              WorkspaceMirrorEntry(
                nodeRef: 'README',
                name: 'README.md',
                isDirectory: false,
                resourceRef: 'resource-1',
              ),
            ],
          );
        }
        return const WorkspaceMirrorPage(entries: []);
      };
    final controller = WorkspaceMirrorController();
    await controller.connect('workspace-1', port);
    expect(port.requests, isEmpty);

    final root = controller.mounts.single;
    await controller.expand(root);
    expect(port.requests, [('workspace-1', 'root', null)]);
    expect(root.children.single.name, 'src');
    controller.collapse(root);
    await controller.expand(root);
    expect(port.requests, hasLength(1));

    await controller.loadMore(root);
    expect(port.requests.last, ('workspace-1', 'root', 'page-2'));
    expect(root.children, hasLength(2));
    await controller.expand(root.children.first);
    expect(port.requests.last, ('workspace-1', 'src', null));
  });

  test(
    'late Desktop response cannot populate a different connection',
    () async {
      final pending = Completer<WorkspaceMirrorPage>();
      final first = _Port()..reply = (_, _) => pending.future;
      final second = _Port()
        ..reply = (_, _) async => const WorkspaceMirrorPage(entries: []);
      final controller = WorkspaceMirrorController();
      await controller.connect('old-workspace', first);
      final oldRoot = controller.mounts.single;
      final loading = controller.expand(oldRoot);
      await controller.connect('new-workspace', second);
      pending.complete(
        const WorkspaceMirrorPage(
          entries: [
            WorkspaceMirrorEntry(
              nodeRef: 'secret',
              name: 'old file',
              isDirectory: false,
            ),
          ],
        ),
      );
      await loading;
      expect(controller.mounts.single.children, isEmpty);
      await controller.expand(oldRoot);
      expect(second.requests, isEmpty);
    },
  );
}
