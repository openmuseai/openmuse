import 'package:appflowy/features/page_access_level/logic/page_access_level_bloc.dart';
import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/office/office_catalog.dart';
import 'package:appflowy/plugins/office/office_manifest.dart';
import 'package:appflowy/plugins/office/office_placeholder_page.dart';
import 'package:appflowy/plugins/util.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/application/view/view_ext.dart';
import 'package:appflowy/workspace/application/view_info/view_info_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:appflowy/workspace/presentation/widgets/favorite_button.dart';
import 'package:appflowy/workspace/presentation/widgets/more_view_actions/more_view_actions.dart';
import 'package:appflowy/workspace/presentation/widgets/tab_bar_item.dart';
import 'package:appflowy/workspace/presentation/widgets/view_title_bar.dart';
import 'package:appflowy_backend/log.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/user_profile.pb.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class OfficePluginBuilder extends PluginBuilder {
  OfficePluginBuilder(this.manifest);

  factory OfficePluginBuilder.forId(String id) {
    OfficePluginCatalog.ensureInstalled();
    return OfficePluginBuilder(OfficePluginRegistry.instance.mustGet(id));
  }

  final OfficeManifest manifest;

  @override
  Plugin build(dynamic data) {
    if (data is ViewPB) {
      return OfficePlugin(view: data);
    }
    throw FlowyPluginException.invalidData;
  }

  @override
  String get menuName => manifest.menuName;

  @override
  FlowySvgData get icon => FlowySvgs.icon_document_s;

  @override
  PluginType get pluginType => manifest.pluginType;

  @override
  ViewLayoutPB get layoutType => manifest.layout;
}

class OfficePluginConfig implements PluginConfig {
  OfficePluginConfig(this.manifest);
  final OfficeManifest manifest;

  @override
  bool get creatable => manifest.creatable;
}

class OfficePlugin extends Plugin {
  OfficePlugin({required ViewPB view})
      : notifier = ViewPluginNotifier(view: view) {
    OfficePluginCatalog.ensureInstalled();
  }

  late final ViewInfoBloc _viewInfoBloc;
  late final PageAccessLevelBloc _pageAccessLevelBloc;

  @override
  final ViewPluginNotifier notifier;

  OfficeManifest get manifest {
    final found = OfficePluginRegistry.instance.byLayout(notifier.view.layout);
    if (found == null) {
      throw StateError('No office plugin for ${notifier.view.layout}');
    }
    return found;
  }

  @override
  PluginWidgetBuilder get widgetBuilder => OfficePluginWidgetBuilder(
        manifest: manifest,
        viewInfoBloc: _viewInfoBloc,
        pageAccessLevelBloc: _pageAccessLevelBloc,
        notifier: notifier,
      );

  @override
  PluginId get id => notifier.view.id;

  @override
  PluginType get pluginType => manifest.pluginType;

  @override
  void init() {
    _viewInfoBloc = ViewInfoBloc(view: notifier.view)
      ..add(const ViewInfoEvent.started());
    _pageAccessLevelBloc = PageAccessLevelBloc(view: notifier.view)
      ..add(const PageAccessLevelEvent.initial());
  }

  @override
  void dispose() {
    _viewInfoBloc.close();
    _pageAccessLevelBloc.close();
    notifier.dispose();
  }
}

class OfficePluginWidgetBuilder extends PluginWidgetBuilder with NavigationItem {
  OfficePluginWidgetBuilder({
    required this.manifest,
    required this.viewInfoBloc,
    required this.pageAccessLevelBloc,
    required this.notifier,
  });

  final OfficeManifest manifest;
  final ViewInfoBloc viewInfoBloc;
  final PageAccessLevelBloc pageAccessLevelBloc;
  final ViewPluginNotifier notifier;
  int? deletedViewIndex;

  ViewPB get view => notifier.view;

  @override
  EdgeInsets get contentPadding => EdgeInsets.zero;

  @override
  String? get viewName => view.nameOrDefault;

  @override
  Widget get leftBarItem => BlocProvider.value(
        value: pageAccessLevelBloc,
        child: ViewTitleBar(key: ValueKey(view.id), view: view),
      );

  @override
  Widget tabBarItem(String pluginId, [bool shortForm = false]) =>
      ViewTabBarItem(view: view, shortForm: shortForm);

  @override
  Widget? get rightBarItem => MultiBlocProvider(
        providers: [
          BlocProvider<ViewInfoBloc>.value(value: viewInfoBloc),
          BlocProvider<PageAccessLevelBloc>.value(value: pageAccessLevelBloc),
        ],
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            ViewFavoriteButton(
              key: ValueKey('favorite_button_${view.id}'),
              view: view,
            ),
            const HSpace(4),
            MoreViewActions(key: ValueKey(view.id), view: view),
          ],
        ),
      );

  @override
  Widget buildWidget({
    required PluginContext context,
    required bool shrinkWrap,
    Map<String, dynamic>? data,
  }) {
    notifier.isDeleted.addListener(_onDeleted);
    final UserProfilePB? user = context.userProfile;
    if (user == null) {
      Log.error('User profile is null when opening office plugin');
      return const SizedBox();
    }
    final body = manifest.pageBuilder?.call(
          view: view,
          user: user,
          onDeleted: () => context.onDeleted?.call(view, deletedViewIndex),
        ) ??
        OfficePlaceholderPage(manifest: manifest);
    return MultiBlocProvider(
      providers: [
        BlocProvider.value(value: viewInfoBloc),
        BlocProvider.value(value: pageAccessLevelBloc),
      ],
      child: body,
    );
  }

  void _onDeleted() {
    final deletedView = notifier.isDeleted.value;
    if (deletedView != null && deletedView.hasIndex()) {
      deletedViewIndex = deletedView.index;
    }
  }

  @override
  List<NavigationItem> get navigationItems => [this];
}
