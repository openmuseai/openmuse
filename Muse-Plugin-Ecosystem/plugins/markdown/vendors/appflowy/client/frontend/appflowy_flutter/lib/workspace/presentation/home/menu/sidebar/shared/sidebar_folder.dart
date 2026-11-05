import 'package:appflowy/features/workspace/logic/workspace_bloc.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/favorite/favorite_bloc.dart';
import 'package:appflowy/workspace/application/menu/sidebar_sections_bloc.dart';
import 'package:appflowy/workspace/application/sidebar/folder/folder_bloc.dart';
import 'package:appflowy/workspace/presentation/home/menu/menu_shared_state.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/favorites/favorite_folder.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/folder/_section_folder.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class SidebarFolder extends StatelessWidget {
  const SidebarFolder({
    super.key,
    this.isHoverEnabled = true,
    this.includeBottomSpacer = true,
    required this.userProfile,
    this.expanded,
    this.onExpandedChanged,
    this.fillRemaining = false,
    this.onContentHeight,
    this.onActivate,
  });

  final bool isHoverEnabled;
  final bool includeBottomSpacer;
  final UserProfilePB userProfile;
  final bool? expanded;
  final ValueChanged<bool>? onExpandedChanged;
  final bool fillRemaining;
  final ValueChanged<double>? onContentHeight;
  final VoidCallback? onActivate;

  @override
  Widget build(BuildContext context) {
    const sectionPadding = 16.0;
    return ValueListenableBuilder(
      valueListenable: getIt<MenuSharedState>().notifier,
      builder: (context, value, child) {
        final body = Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            BlocBuilder<FavoriteBloc, FavoriteState>(
              builder: (context, state) {
                if (state.views.isEmpty) {
                  return const SizedBox.shrink();
                }
                return FavoriteFolder(
                  views: state.views.map((e) => e.item).toList(),
                );
              },
            ),
            _buildSections(context, sectionPadding: sectionPadding),
            if (includeBottomSpacer) const VSpace(200),
          ],
        );
        return Column(
          mainAxisSize: fillRemaining ? MainAxisSize.max : MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const VSpace(4.0),
            if (fillRemaining)
              Expanded(child: SingleChildScrollView(child: body))
            else
              SingleChildScrollView(child: body),
          ],
        );
      },
    );
  }

  Widget _buildSections(
    BuildContext context, {
    required double sectionPadding,
  }) {
    return BlocBuilder<SidebarSectionsBloc, SidebarSectionsState>(
      builder: (context, state) {
        final isCollaborativeWorkspace =
            context.read<UserWorkspaceBloc>().state.isCollabWorkspaceOn;
        final personal = PersonalSectionFolder(
          views: state.section.publicViews,
          expanded: expanded,
          onExpandedChanged: onExpandedChanged,
          onContentHeight: onContentHeight,
          onActivate: onActivate,
        );
        if (isCollaborativeWorkspace) {
          return Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              VSpace(sectionPadding),
              PublicSectionFolder(
                views: state.section.publicViews,
                expanded: expanded,
                onExpandedChanged: onExpandedChanged,
                onContentHeight: onContentHeight,
                onActivate: onActivate,
              ),
              VSpace(sectionPadding),
              PrivateSectionFolder(
                views: state.section.privateViews,
                expanded: expanded,
                onExpandedChanged: onExpandedChanged,
              ),
            ],
          );
        }
        return Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            VSpace(sectionPadding),
            personal,
          ],
        );
      },
    );
  }
}

class PrivateSectionFolder extends SectionFolder {
  PrivateSectionFolder({
    super.key,
    required super.views,
    super.expanded,
    super.onExpandedChanged,
    super.fillRemaining,
    super.onContentHeight,
    super.onActivate,
  })
      : super(
          title: LocaleKeys.sideBar_private.tr(),
          spaceType: FolderSpaceType.private,
          expandButtonTooltip: LocaleKeys.sideBar_clickToHidePrivate.tr(),
          addButtonTooltip: LocaleKeys.sideBar_addAPageToPrivate.tr(),
        );
}

class PublicSectionFolder extends SectionFolder {
  PublicSectionFolder({
    super.key,
    required super.views,
    super.expanded,
    super.onExpandedChanged,
    super.fillRemaining,
    super.onContentHeight,
    super.onActivate,
  })
      : super(
          title: LocaleKeys.sideBar_workspace.tr(),
          spaceType: FolderSpaceType.public,
          expandButtonTooltip: LocaleKeys.sideBar_clickToHideWorkspace.tr(),
          addButtonTooltip: LocaleKeys.sideBar_addAPageToWorkspace.tr(),
        );
}

class PersonalSectionFolder extends SectionFolder {
  PersonalSectionFolder({
    super.key,
    required super.views,
    super.expanded,
    super.onExpandedChanged,
    super.fillRemaining,
    super.onContentHeight,
    super.onActivate,
  })
      : super(
          title: LocaleKeys.sideBar_personal.tr(),
          spaceType: FolderSpaceType.public,
          expandButtonTooltip: LocaleKeys.sideBar_clickToHidePersonal.tr(),
          addButtonTooltip: LocaleKeys.sideBar_addAPage.tr(),
        );
}
