import 'package:appflowy/features/workspace/logic/workspace_bloc.dart';
import 'package:appflowy/workspace/application/menu/sidebar_sections_bloc.dart';
import 'package:appflowy/workspace/application/sidebar/folder/folder_bloc.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_sizes.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/folder/_folder_header.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/shared/sidebar_stack_panes.dart';
import 'package:appflowy/workspace/presentation/home/menu/view/view_item.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class SectionFolder extends StatefulWidget {
  const SectionFolder({
    super.key,
    required this.title,
    required this.spaceType,
    required this.views,
    this.isHoverEnabled = true,
    required this.expandButtonTooltip,
    required this.addButtonTooltip,
    this.expanded,
    this.onExpandedChanged,
    this.fillRemaining = false,
    this.onContentHeight,
    this.onActivate,
  });

  final String title;
  final FolderSpaceType spaceType;
  final List<ViewPB> views;
  final bool isHoverEnabled;
  final String expandButtonTooltip;
  final String addButtonTooltip;
  final bool? expanded;
  final ValueChanged<bool>? onExpandedChanged;
  final bool fillRemaining;
  final ValueChanged<double>? onContentHeight;
  final VoidCallback? onActivate;

  @override
  State<SectionFolder> createState() => _SectionFolderState();
}

class _SectionFolderState extends State<SectionFolder> {
  final isHovered = ValueNotifier(false);

  @override
  void dispose() {
    isHovered.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => isHovered.value = true,
      onExit: (_) => isHovered.value = false,
      child: BlocProvider<FolderBloc>(
        create: (_) => FolderBloc(type: widget.spaceType)
          ..add(const FolderEvent.initial()),
        child: BlocBuilder<FolderBloc, FolderState>(
          builder: (context, state) {
            final expanded = widget.expanded ?? state.isExpanded;
            final views = _buildViews(context, expanded, isHovered);
            final list = Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                ...views,
                _buildDraggablePlaceholder(context),
              ],
            );
            return Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                _buildHeader(context, expanded),
                if (expanded) const VSpace(4.0),
                if (expanded)
                  SidebarContentHeightReporter(
                    onHeight: (height) => widget.onContentHeight?.call(height),
                    child: list,
                  )
                else
                  SidebarContentHeightReporter(
                    onHeight: (height) => widget.onContentHeight?.call(0),
                    child: const SizedBox.shrink(),
                  ),
              ],
            );
          },
        ),
      ),
    );
  }

  Widget _buildHeader(BuildContext context, bool expanded) {
    return FolderHeader(
      title: widget.title,
      isExpanded: expanded,
      expandButtonTooltip: widget.expandButtonTooltip,
      addButtonTooltip: widget.addButtonTooltip,
      onPressed: () {
        final next = !expanded;
        widget.onActivate?.call();
        final onChanged = widget.onExpandedChanged;
        if (onChanged != null) {
          onChanged(next);
        } else {
          context.read<FolderBloc>().add(const FolderEvent.expandOrUnExpand());
        }
      },
      onAdded: () {
        context.read<SidebarSectionsBloc>().add(
              SidebarSectionsEvent.createRootViewInSection(
                name: '',
                index: 0,
                viewSection: widget.spaceType.toViewSectionPB,
              ),
            );

        final onChanged = widget.onExpandedChanged;
        if (onChanged != null) {
          onChanged(true);
        } else {
          context
              .read<FolderBloc>()
              .add(const FolderEvent.expandOrUnExpand(isExpanded: true));
        }
      },
    );
  }

  Iterable<Widget> _buildViews(
    BuildContext context,
    bool expanded,
    ValueNotifier<bool> isHovered,
  ) {
    if (!expanded) {
      return [];
    }

    return widget.views.map(
      (view) => ViewItem(
        key: ValueKey('${widget.spaceType.name} ${view.id}'),
        spaceType: widget.spaceType,
        engagedInExpanding: true,
        isFirstChild: view.id == widget.views.first.id,
        view: view,
        level: 0,
        leftPadding: HomeSpaceViewSizes.leftPadding,
        isFeedback: false,
        isHovered: isHovered,
        enableRightClickContext: true,
        onSelected: (viewContext, view) {
          if (HardwareKeyboard.instance.isControlPressed) {
            context.read<TabsBloc>().openTab(view);
          }

          context.read<TabsBloc>().openPlugin(view);
        },
        onTertiarySelected: (viewContext, view) =>
            context.read<TabsBloc>().openTab(view),
        isHoverEnabled: widget.isHoverEnabled,
      ),
    );
  }

  Widget _buildDraggablePlaceholder(BuildContext context) {
    if (widget.views.isNotEmpty) {
      return const SizedBox.shrink();
    }
    final parentViewId =
        context.read<UserWorkspaceBloc>().state.currentWorkspace?.workspaceId;
    return ViewItem(
      spaceType: widget.spaceType,
      view: ViewPB(parentViewId: parentViewId ?? ''),
      level: 0,
      leftPadding: HomeSpaceViewSizes.leftPadding,
      isFeedback: false,
      onSelected: (_, __) {},
      isHoverEnabled: widget.isHoverEnabled,
      isPlaceholder: true,
    );
  }
}
