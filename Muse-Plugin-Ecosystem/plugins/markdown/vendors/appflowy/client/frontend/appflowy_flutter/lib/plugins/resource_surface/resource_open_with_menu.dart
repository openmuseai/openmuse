import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_defaults.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/presentation/widgets/muse_context_menu.dart';
import 'package:flutter/material.dart';

/// Right-growing "打开方式" cascade: engines, then "默认打开方式".
class MuseResourceOpenWithMenu extends StatelessWidget {
  const MuseResourceOpenWithMenu({
    super.key,
    required this.target,
    required this.closeMenu,
    this.defaultsOnly = false,
  });

  final MuseResourceTabTarget target;
  final VoidCallback closeMenu;
  final bool defaultsOnly;

  @override
  Widget build(BuildContext context) {
    final registry = getIt<MuseResourceEngineRegistry>();
    final defaults = getIt<MuseResourceOpenDefaults>();
    final extension =
        MuseResourceEngineRegistry.extensionOf(target.resource.file.path);
    final preferred = defaults.engineFor(extension);
    final entries = <MuseContextMenuEntry>[
      for (final spec in registry.all)
        MuseContextMenuAction(
          id: spec.id,
          label: spec.label,
          icon: spec.icon,
          enabled: spec.accepts(extension),
          trailing: _check(
            defaultsOnly
                ? preferred == spec.engine
                : target.selectedEngine == spec.engine,
          ),
        ),
      if (!defaultsOnly) ...[
        const MuseContextMenuDivider(),
        MuseContextMenuAction(
          id: 'muse.open-with.default',
          label: '默认打开方式',
          icon: Icons.star_outline,
          submenuBuilder: (ctx, close) => MuseResourceOpenWithMenu(
            target: target,
            closeMenu: () {
              close();
              closeMenu();
            },
            defaultsOnly: true,
          ),
        ),
      ],
    ];
    return Padding(
      padding: museContextMenuPadding,
      child: MuseContextMenuBody(
        entries: entries,
        onSelected: (id) => _onSelected(
          id: id,
          registry: registry,
          defaults: defaults,
          extension: extension,
        ),
        onDismiss: closeMenu,
      ),
    );
  }

  Widget? _check(bool selected) =>
      selected ? const Icon(Icons.check, size: 16) : null;

  Future<void> _onSelected({
    required String id,
    required MuseResourceEngineRegistry registry,
    required MuseResourceOpenDefaults defaults,
    required String extension,
  }) async {
    final spec =
        registry.all.where((candidate) => candidate.id == id).firstOrNull;
    if (spec == null) return;
    if (defaultsOnly) {
      await defaults.setEngine(extension, spec.engine);
    }
    target.selectEngine(spec.engine);
    closeMenu();
  }
}

extension<T> on Iterable<T> {
  T? get firstOrNull {
    final iterator = this.iterator;
    return iterator.moveNext() ? iterator.current : null;
  }
}
