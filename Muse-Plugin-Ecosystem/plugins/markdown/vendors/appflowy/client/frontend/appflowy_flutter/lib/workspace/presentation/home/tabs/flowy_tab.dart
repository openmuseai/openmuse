import 'dart:async';

import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_sizes.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/presentation/widgets/muse_context_menu.dart';

import 'package:easy_localization/easy_localization.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flowy_infra_ui/style_widget/hover.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:provider/provider.dart';

class FlowyTab extends StatefulWidget {
  const FlowyTab({
    super.key,
    required this.pageManager,
    required this.isCurrent,
    required this.onTap,
    required this.isAllPinned,
  });

  final PageManager pageManager;
  final bool isCurrent;
  final VoidCallback onTap;

  /// Signifies whether all tabs are pinned
  ///
  final bool isAllPinned;

  @override
  State<FlowyTab> createState() => _FlowyTabState();
}

class _FlowyTabState extends State<FlowyTab> {
  final controller = PopoverController();

  @override
  Widget build(BuildContext context) {
    final pinned = widget.pageManager.isPinned;
    final current = widget.isCurrent;
    return ConstrainedBox(
      constraints: pinned
          ? const BoxConstraints.tightFor(width: 54)
          : current
              ? const BoxConstraints(minWidth: 128)
              : const BoxConstraints(minWidth: 88, maxWidth: 168),
      child: _wrapInTooltip(
        widget.pageManager.plugin.widgetBuilder.viewName,
        child: FlowyHover(
          resetHoverOnRebuild: false,
          style: HoverStyle(
            borderRadius: BorderRadius.zero,
            backgroundColor: current
                ? Theme.of(context).colorScheme.surface
                : Theme.of(context).colorScheme.surfaceContainerHighest,
            hoverColor: current ? Theme.of(context).colorScheme.surface : null,
          ),
          builder: (context, isHovering) => AppFlowyPopover(
            controller: controller,
            offset: const Offset(0, 8),
            direction: PopoverDirection.bottomWithLeftAligned,
            constraints: museContextMenuConstraints,
            triggerActions: PopoverTriggerFlags.secondaryClick,
            showAtCursor: true,
            popupBuilder: (_) => BlocProvider.value(
              value: context.read<TabsBloc>(),
              child: TabMenu(
                controller: controller,
                pageId: widget.pageManager.plugin.id,
                plugin: widget.pageManager.plugin,
                isPinned: pinned,
                isAllPinned: widget.isAllPinned,
              ),
            ),
            child: ChangeNotifierProvider.value(
              value: widget.pageManager.notifier,
              child: Consumer<PageNotifier>(
                builder: (context, value, _) => Padding(
                  padding: EdgeInsets.symmetric(
                    horizontal: current ? 14.0 : 10.0,
                  ),
                  child: GestureDetector(
                    behavior: HitTestBehavior.opaque,
                    onTap: widget.onTap,
                    onPanStart: (_) {},
                    child: SizedBox(
                      height: HomeSizes.tabBarHeight,
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (current)
                            widget.pageManager.notifier.tabBarWidget(
                              widget.pageManager.plugin.id,
                              pinned,
                            )
                          else
                            Flexible(
                              child: widget.pageManager.notifier.tabBarWidget(
                                widget.pageManager.plugin.id,
                                pinned,
                              ),
                            ),
                          if (widget.pageManager.plugin
                              is PluginTabMenuContributor)
                            Visibility(
                              visible: isHovering || current,
                              child: SizedBox(
                                width: 24,
                                height: 26,
                                child: FlowyIconButton(
                                  tooltipText: 'Tab actions',
                                  onPressed: controller.show,
                                  icon: const Icon(Icons.more_horiz, size: 17),
                                ),
                              ),
                            ),
                          if (!pinned) ...[
                            Visibility(
                              visible: isHovering || current,
                              child: SizedBox(
                                width: 26,
                                height: 26,
                                child: FlowyIconButton(
                                  onPressed: () => _closeTab(context),
                                  icon: const FlowySvg(
                                    FlowySvgs.close_s,
                                    size: Size.square(22),
                                  ),
                                ),
                              ),
                            ),
                          ],
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _closeTab(BuildContext context) => context
      .read<TabsBloc>()
      .add(TabsEvent.closeTab(widget.pageManager.plugin.id));

  Widget _wrapInTooltip(String? viewName, {required Widget child}) {
    if (viewName != null) {
      return FlowyTooltip(
        message: viewName,
        child: child,
      );
    }

    return child;
  }
}

@visibleForTesting
class TabMenu extends StatelessWidget {
  const TabMenu({
    super.key,
    required this.controller,
    required this.pageId,
    required this.plugin,
    required this.isPinned,
    required this.isAllPinned,
  });

  final PopoverController controller;
  final String pageId;
  final Plugin plugin;
  final bool isPinned;
  final bool isAllPinned;

  @override
  Widget build(BuildContext context) {
    final entries = <MuseContextMenuEntry>[
      MuseContextMenuAction(
        id: 'tab.close',
        label: LocaleKeys.tabMenu_close.tr(),
        icon: Icons.close,
        enabled: !isPinned,
      ),
      MuseContextMenuAction(
        id: 'tab.close-others',
        label: LocaleKeys.tabMenu_closeOthers.tr(),
        icon: Icons.copy_all_outlined,
        enabled: !isAllPinned,
      ),
      if (plugin case final PluginTabMenuContributor contributor) ...[
        const MuseContextMenuDivider(),
        ..._pluginEntries(context, contributor),
      ],
      const MuseContextMenuDivider(),
      MuseContextMenuAction(
        id: 'tab.pin',
        label: isPinned
            ? LocaleKeys.tabMenu_unpinTab.tr()
            : LocaleKeys.tabMenu_pinTab.tr(),
        icon: isPinned ? Icons.push_pin : Icons.push_pin_outlined,
      ),
    ];
    return ConstrainedBox(
      constraints: museContextMenuConstraints,
      child: MuseContextMenuBody(
        entries: entries,
        onSelected: (id) => _onSelected(context, id),
        onDismiss: controller.close,
      ),
    );
  }

  List<MuseContextMenuEntry> _pluginEntries(
    BuildContext context,
    PluginTabMenuContributor contributor,
  ) {
    final actions = contributor.tabMenuActions(context);
    final entries = <MuseContextMenuEntry>[];
    int? previousGroup;
    for (final action in actions) {
      if (previousGroup != null && previousGroup != action.group) {
        entries.add(const MuseContextMenuDivider());
      }
      previousGroup = action.group;
      entries.add(
        MuseContextMenuAction(
          id: 'plugin:${action.id}',
          label: action.label,
          icon: action.icon,
          enabled: action.enabled,
          submenuBuilder: action.submenuBuilder == null
              ? null
              : (ctx, close) => action.submenuBuilder!(ctx, close),
        ),
      );
    }
    return entries;
  }

  void _onSelected(BuildContext context, String id) {
    switch (id) {
      case 'tab.close':
        _closeTab(context);
        return;
      case 'tab.close-others':
        _closeOtherTabs(context);
        return;
      case 'tab.pin':
        _togglePin(context);
        return;
    }
    if (!id.startsWith('plugin:')) return;
    final pluginId = id.substring('plugin:'.length);
    if (plugin case final PluginTabMenuContributor contributor) {
      final action = contributor
          .tabMenuActions(context)
          .where((candidate) => candidate.id == pluginId)
          .firstOrNull;
      if (action == null) return;
      controller.close();
      unawaited(Future<void>.value(action.invoke(context)));
    }
  }

  void _closeTab(BuildContext context) {
    context.read<TabsBloc>().add(TabsEvent.closeTab(pageId));
    controller.close();
  }

  void _closeOtherTabs(BuildContext context) {
    context.read<TabsBloc>().add(TabsEvent.closeOtherTabs(pageId));
    controller.close();
  }

  void _togglePin(BuildContext context) {
    context.read<TabsBloc>().add(TabsEvent.togglePin(pageId));
    controller.close();
  }
}
