import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/version_diff/application/image_overlay_diff_service.dart';
import 'package:appflowy/plugins/version_diff/presentation/image_overlay_diff_viewer.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pbenum.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

final class MuseImageOverlayDiffPluginConfig implements PluginConfig {
  @override
  bool get creatable => false;
}

final class MuseImageOverlayDiffPluginBuilder extends PluginBuilder {
  @override
  Plugin build(dynamic data) {
    if (data is MuseImageComparisonDocument) {
      return MuseImageOverlayDiffPlugin(data);
    }
    throw ArgumentError.value(
      data,
      'data',
      'MuseImageComparisonDocument required',
    );
  }

  @override
  String get menuName => 'Image comparison';

  @override
  FlowySvgData get icon => const FlowySvgData('');

  @override
  PluginType get pluginType => PluginType.diff;

  @override
  ViewLayoutPB? get layoutType => null;
}

final class MuseImageOverlayDiffPlugin extends Plugin {
  MuseImageOverlayDiffPlugin(this.document);

  final MuseImageComparisonDocument document;

  @override
  PluginId get id => 'diff:${document.diff.comparison.id}';

  @override
  PluginType get pluginType => PluginType.diff;

  @override
  PluginWidgetBuilder get widgetBuilder =>
      MuseImageOverlayDiffPluginWidgetBuilder(document);
}

final class MuseImageOverlayDiffPluginWidgetBuilder extends PluginWidgetBuilder
    with NavigationItem {
  MuseImageOverlayDiffPluginWidgetBuilder(this.document);

  final MuseImageComparisonDocument document;

  String get _title => '${p.basename(document.file.path)} · Image Diff';

  @override
  EdgeInsets get contentPadding => EdgeInsets.zero;

  @override
  String get viewName => _title;

  @override
  Widget get leftBarItem => Text(_title);

  @override
  Widget tabBarItem(String pluginId, [bool shortForm = false]) =>
      Text(_title, overflow: TextOverflow.ellipsis);

  @override
  List<NavigationItem> get navigationItems => [this];

  @override
  Widget buildWidget({
    required PluginContext context,
    required bool shrinkWrap,
    Map<String, dynamic>? data,
  }) =>
      MuseImageOverlayDiffViewer(document: document);
}
