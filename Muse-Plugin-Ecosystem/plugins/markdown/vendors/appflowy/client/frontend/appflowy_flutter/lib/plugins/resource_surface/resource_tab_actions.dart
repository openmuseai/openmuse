import 'dart:async';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/resource_file_plugin.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_with_menu.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_session.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/image_overlay_diff_service.dart';
import 'package:appflowy/plugins/version_diff/presentation/resource_version_pane.dart';
import 'package:appflowy/plugins/version_diff/presentation/version_history_dialog.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;

void registerDefaultMuseResourceTabActions(
  MuseResourceTabActionRegistry registry,
) {
  bool versionable(MuseResourceTabTarget target) {
    final file = target.resource.file;
    return getIt<MuseTextVersionDiffService>().supports(file) ||
        getIt<MuseImageOverlayDiffService>().supports(file);
  }

  registry
    ..register(
      MuseResourceTabAction(
        id: 'muse.open-with',
        label: '打开方式',
        icon: Icons.open_with,
        group: 10,
        order: 10,
        submenuBuilder: (context, target, closeMenu) =>
            MuseResourceOpenWithMenu(
          target: target,
          closeMenu: closeMenu,
        ),
        handler: (context, target) async {
          if (target is MuseResourceFilePlugin) return;
          if (!context.mounted) return;
          await showDialog<void>(
            context: context,
            builder: (ctx) => Dialog(
              child: SizedBox(
                width: 280,
                child: MuseResourceOpenWithMenu(
                  target: target,
                  closeMenu: () => Navigator.of(ctx).pop(),
                ),
              ),
            ),
          );
        },
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'muse.version.save',
        label: '保存当前版本',
        icon: Icons.save_outlined,
        group: 20,
        order: 10,
        enabled: (target) {
          if (!versionable(target)) return false;
          final file = target.resource.file;
          return MuseResourceSurfaceSession.instance.isBufferDirty(file) ||
              getIt<MuseResourceVersionPaneRegistry>().of(file).canSave;
        },
        handler: (context, target) async {
          final file = target.resource.file;
          await MuseResourceSurfaceSession.instance.flush(file);
          if (!await getIt<MuseTextVersionDiffService>()
              .hasUncommittedChanges(file)) {
            unawaited(
              getIt<MuseResourceVersionPaneRegistry>()
                  .of(file)
                  .refreshCanSave(),
            );
            return;
          }
          final version = getIt<MuseImageOverlayDiffService>().supports(file)
              ? await getIt<MuseImageOverlayDiffService>().saveVersion(file)
              : await getIt<MuseTextVersionDiffService>().saveVersion(file);
          MuseResourceSurfaceSession.instance.clearDirty(file);
          unawaited(
            getIt<MuseResourceVersionPaneRegistry>().of(file).reload(),
          );
          if (!context.mounted) return;
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(
                '已保存版本 ${version.contentDigest.substring(0, 8)}',
              ),
            ),
          );
        },
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'muse.version.panel',
        label: '版本',
        icon: Icons.history,
        group: 20,
        order: 20,
        enabled: versionable,
        submenuBuilder: (context, target, closeMenu) {
          final pane =
              getIt<MuseResourceVersionPaneRegistry>().of(target.resource.file);
          unawaited(pane.prepareMenu());
          return MuseResourceVersionPane(
            controller: pane,
            onDone: closeMenu,
          );
        },
        handler: (context, target) async {
          final file = target.resource.file;
          final pane = getIt<MuseResourceVersionPaneRegistry>().of(file);
          await pane.prepareMenu();
          if (target is MuseResourceFilePlugin) return;
          if (!context.mounted) return;
          await showDialog<void>(
            context: context,
            builder: (ctx) => Dialog(
              child: SizedBox(
                width: 300,
                height: 420,
                child: MuseResourceVersionPane(
                  controller: pane,
                  onDone: () => Navigator.of(ctx).pop(),
                ),
              ),
            ),
          );
        },
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'muse.version.history',
        label: '版本历史与审计…',
        icon: Icons.history,
        group: 20,
        order: 30,
        enabled: (target) =>
            getIt<MuseTextVersionDiffService>().supports(target.resource.file),
        handler: (context, target) => showMuseVersionHistoryDialog(
          context: context,
          file: target.resource.file,
          service: getIt<MuseTextVersionDiffService>(),
        ),
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'host.resource.copy-path',
        label: 'Copy Path',
        icon: Icons.content_copy_outlined,
        group: 30,
        order: 10,
        handler: (_, target) => Clipboard.setData(
          ClipboardData(text: target.resource.file.path),
        ),
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'host.resource.copy-relative-path',
        label: 'Copy Relative Path',
        icon: Icons.content_copy_outlined,
        group: 30,
        order: 15,
        handler: (_, target) => Clipboard.setData(
          ClipboardData(text: _relativePath(target.resource.file)),
        ),
      ),
    )
    ..register(
      MuseResourceTabAction(
        id: 'host.resource.reveal',
        label: '在 Finder 中显示',
        icon: Icons.folder_open_outlined,
        group: 30,
        order: 20,
        enabled: (_) => Platform.isMacOS,
        handler: (_, target) =>
            Process.run('open', ['-R', target.resource.file.path]),
      ),
    );
}

String _relativePath(File file) {
  final path = file.absolute.path;
  try {
    for (final mount in getIt<MuseWorkspaceController>().mounts) {
      final root = mount.rootLocator;
      if (p.equals(root, path)) return mount.displayName;
      if (p.isWithin(root, path)) return p.relative(path, from: root);
    }
  } on Object {
    // Workspace controller is optional in unit tests.
  }
  return p.basename(path);
}
