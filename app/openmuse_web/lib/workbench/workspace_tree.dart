import 'package:flutter/material.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

final class WorkspaceMirrorTree extends StatelessWidget {
  const WorkspaceMirrorTree({
    super.key,
    required this.controller,
    required this.onFileSelected,
    this.disconnectedView,
    this.onRetry,
  });

  final WorkspaceMirrorController controller;
  final ValueChanged<WorkspaceMirrorNode> onFileSelected;
  final Widget? disconnectedView;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) {
      if (controller.workspaceRef == null) {
        return disconnectedView ??
            const Center(child: Text('连接 Desktop 后显示 Workspace'));
      }
      if (controller.mountsLoading) {
        return const Center(child: CircularProgressIndicator());
      }
      if (controller.mountError != null) {
        return Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text('Workspace 目录加载失败'),
              if (onRetry != null)
                TextButton(onPressed: onRetry, child: const Text('重试连接')),
            ],
          ),
        );
      }
      return ListView(
        children: [for (final mount in controller.mounts) _node(mount, 0)],
      );
    },
  );

  Widget _node(WorkspaceMirrorNode node, int depth) => Column(
    mainAxisSize: MainAxisSize.min,
    children: [
      OpenMuseWorkspaceRow(
        name: node.name,
        isDirectory: node.isDirectory,
        expanded: node.expanded,
        depth: depth,
        isMount: depth == 0,
        loading: node.loading,
        extension: node.name.split('.').last,
        onTap: () {
          if (node.isDirectory) {
            if (node.expanded) {
              controller.collapse(node);
            } else {
              controller.expand(node);
            }
          } else {
            onFileSelected(node);
          }
        },
      ),
      if (node.error != null && node.expanded)
        TextButton(
          onPressed: () => controller.refresh(node),
          child: const Text('加载失败，重试'),
        ),
      if (node.expanded) ...[
        for (final child in node.children) _node(child, depth + 1),
        if (node.nextCursor != null)
          TextButton(
            onPressed: node.loading ? null : () => controller.loadMore(node),
            child: const Text('加载更多'),
          ),
      ],
    ],
  );
}
