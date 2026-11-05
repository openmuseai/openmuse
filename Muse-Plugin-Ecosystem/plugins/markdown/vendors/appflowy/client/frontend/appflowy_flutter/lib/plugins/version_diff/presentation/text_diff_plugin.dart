import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/presentation/diff_tab_title.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_viewer.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pbenum.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

final class MuseTextDiffPluginConfig implements PluginConfig {
  @override
  bool get creatable => false;
}

final class MuseTextDiffPluginBuilder extends PluginBuilder {
  @override
  Plugin build(dynamic data) {
    if (data is MuseTextComparisonDocument) return MuseTextDiffPlugin(data);
    throw ArgumentError.value(
      data,
      'data',
      'MuseTextComparisonDocument required',
    );
  }

  @override
  String get menuName => 'Text comparison';

  @override
  FlowySvgData get icon => const FlowySvgData('');

  @override
  PluginType get pluginType => PluginType.diff;

  @override
  ViewLayoutPB? get layoutType => null;
}

final class MuseTextDiffPlugin extends Plugin {
  MuseTextDiffPlugin(
    this.document, {
    String? pluginId,
    String? title,
    this.initialLayout = MuseDiffLayout.split,
    this.isCompare = false,
  })  : pluginId = pluginId ?? 'diff:${document.diff.comparison.id}',
        title = title ?? '${p.basename(document.file.path)} · Diff';

  MuseTextDiffPlugin.view({
    required MuseTextComparisonDocument document,
    required String fileName,
  }) : this(
          document,
          pluginId:
              'diff-view:${document.file.absolute.path}:${document.target.ref.id}',
          title: museDiffViewTabTitle(fileName, document.target),
          initialLayout: MuseDiffLayout.unified,
        );

  MuseTextDiffPlugin.compare({
    required MuseTextComparisonDocument document,
    required String fileName,
  }) : this(
          document,
          pluginId:
              'diff-compare:${document.file.absolute.path}:${document.base.ref.id}:${document.target.ref.id}',
          title: museDiffCompareTabTitle(
            fileName,
            document.base,
            document.target,
          ),
          initialLayout: MuseDiffLayout.split,
          isCompare: true,
        );

  final MuseTextComparisonDocument document;
  final String pluginId;
  final String title;
  final MuseDiffLayout initialLayout;
  final bool isCompare;

  @override
  PluginId get id => pluginId;

  @override
  PluginType get pluginType => PluginType.diff;

  @override
  PluginWidgetBuilder get widgetBuilder =>
      MuseTextDiffPluginWidgetBuilder(this);
}

final class MuseTextDiffPluginWidgetBuilder extends PluginWidgetBuilder
    with NavigationItem {
  MuseTextDiffPluginWidgetBuilder(this.plugin);

  final MuseTextDiffPlugin plugin;

  @override
  EdgeInsets get contentPadding => EdgeInsets.zero;

  @override
  bool get showNavigationTitle => false;

  @override
  String get viewName => plugin.title;

  @override
  Widget get leftBarItem => const SizedBox.shrink();

  @override
  Widget tabBarItem(String pluginId, [bool shortForm = false]) =>
      _DiffTitle(title: plugin.title, isCompare: plugin.isCompare);

  @override
  List<NavigationItem> get navigationItems => const [];

  @override
  Widget buildWidget({
    required PluginContext context,
    required bool shrinkWrap,
    Map<String, dynamic>? data,
  }) =>
      RepaintBoundary(
        key: const ValueKey('diff-host-workbench'),
        child: MuseTextDiffViewer(
          document: plugin.document,
          initialLayout: plugin.initialLayout,
        ),
      );
}

final class _DiffTitle extends StatelessWidget {
  const _DiffTitle({required this.title, required this.isCompare});

  final String title;
  final bool isCompare;

  @override
  Widget build(BuildContext context) {
    final icon = Icon(
      isCompare ? Icons.compare_arrows : Icons.difference_outlined,
      size: 16,
    );
    final text = Text(
      title,
      maxLines: 1,
      softWrap: false,
      overflow: TextOverflow.ellipsis,
    );
    return LayoutBuilder(
      builder: (context, constraints) => Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          icon,
          const SizedBox(width: 7),
          if (constraints.hasBoundedWidth)
            Flexible(child: text)
          else
            text,
        ],
      ),
    );
  }
}
