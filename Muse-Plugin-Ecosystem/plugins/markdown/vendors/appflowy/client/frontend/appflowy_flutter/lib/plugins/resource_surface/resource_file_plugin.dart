import 'dart:async';
import 'dart:io';

import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/resource_surface/engines/helix.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/helix_resource_surface.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/ioffice_word_resource_surface.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/open_file_viewer_resource_surface.dart';
import 'package:appflowy/plugins/version_diff/presentation/resource_version_pane.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pbenum.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as p;

final class MuseResourcePluginConfig implements PluginConfig {
  @override
  bool get creatable => false;
}

final class MuseResourcePluginBuilder extends PluginBuilder {
  @override
  Plugin build(dynamic data) {
    if (data is MuseResolvedResource) return MuseResourceFilePlugin(data);
    throw ArgumentError.value(data, 'data', 'MuseResolvedResource required');
  }

  @override
  String get menuName => 'Local resource';

  @override
  FlowySvgData get icon => const FlowySvgData('');

  @override
  PluginType get pluginType => PluginType.resource;

  @override
  ViewLayoutPB? get layoutType => null;
}

final class MuseResourceFilePlugin extends Plugin
    implements PluginTabMenuContributor, MuseResourceTabTarget {
  MuseResourceFilePlugin(this.resource)
      : _engine = ValueNotifier(resource.engine);

  @override
  final MuseResolvedResource resource;

  final ValueNotifier<MuseLocalEngine> _engine;

  ValueListenable<MuseLocalEngine> get engineListenable => _engine;

  @override
  MuseLocalEngine get selectedEngine => _engine.value;

  @override
  void selectEngine(MuseLocalEngine engine) {
    if (_engine.value != engine) _engine.value = engine;
  }

  @override
  PluginId get id => 'resource:${resource.file.absolute.path}';

  @override
  PluginType get pluginType => PluginType.resource;

  @override
  PluginWidgetBuilder get widgetBuilder =>
      MuseResourcePluginWidgetBuilder(this);

  @override
  List<PluginTabMenuAction> tabMenuActions(BuildContext context) {
    unawaited(
      getIt<MuseResourceVersionPaneRegistry>()
          .of(resource.file)
          .refreshCanSave(),
    );
    return getIt<MuseResourceTabActionRegistry>().actionsFor(this);
  }

  @override
  void dispose() {
    _engine.dispose();
    super.dispose();
  }
}

final class MuseResourcePluginWidgetBuilder extends PluginWidgetBuilder
    with NavigationItem {
  MuseResourcePluginWidgetBuilder(this.plugin);

  final MuseResourceFilePlugin plugin;

  String get _name => p.basename(plugin.resource.file.path);

  @override
  EdgeInsets get contentPadding => EdgeInsets.zero;

  @override
  bool get showNavigationTitle => false;

  @override
  String get viewName => _name;

  @override
  Widget get leftBarItem => const SizedBox.shrink();

  @override
  Widget tabBarItem(String pluginId, [bool shortForm = false]) =>
      _ResourceTitle(file: plugin.resource.file, showPath: false);

  @override
  List<NavigationItem> get navigationItems => const [];

  @override
  Widget buildWidget({
    required PluginContext context,
    required bool shrinkWrap,
    Map<String, dynamic>? data,
  }) {
    return ValueListenableBuilder<MuseLocalEngine>(
      valueListenable: plugin.engineListenable,
      builder: (context, engine, _) {
        return ClipRect(
          child: switch (engine) {
            MuseLocalEngine.ioffice => IofficeWordResourceSurface(
                key: ValueKey('ioffice:${plugin.resource.file.path}'),
                file: plugin.resource.file,
              ),
            MuseLocalEngine.helix => HelixResourceSurface(
                key: ValueKey('helix:${plugin.resource.file.path}'),
                file: plugin.resource.file,
                initialLine: plugin.resource.line,
                performanceTrace: plugin.resource.performanceTrace,
              ),
            MuseLocalEngine.openFileViewer => OpenFileViewerResourceSurface(
                key: ValueKey('viewer:${plugin.resource.file.path}'),
                file: plugin.resource.file,
                initialLine: plugin.resource.line,
                performanceTrace: plugin.resource.performanceTrace,
              ),
          },
        );
      },
    );
  }
}

class _ResourceTitle extends StatelessWidget {
  const _ResourceTitle({required this.file, required this.showPath});

  final File file;
  final bool showPath;

  @override
  Widget build(BuildContext context) {
    final extension = p.extension(file.path).toLowerCase();
    final label = FlowyText.medium(
      showPath ? file.path : p.basename(file.path),
      overflow: TextOverflow.ellipsis,
    );
    return LayoutBuilder(
      builder: (context, constraints) => Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(_iconFor(extension), size: 16),
          const SizedBox(width: 7),
          if (constraints.hasBoundedWidth) Flexible(child: label) else label,
        ],
      ),
    );
  }

  IconData _iconFor(String extension) {
    if (extension == '.docx') return Icons.description_outlined;
    if ({'.pdf', '.ppt', '.pptx', '.xls', '.xlsx'}.contains(extension)) {
      return Icons.insert_drive_file_outlined;
    }
    if ({'.png', '.jpg', '.jpeg', '.gif', '.webp', '.svg'}
        .contains(extension)) {
      return Icons.image_outlined;
    }
    if ({'.mp3', '.wav', '.m4a', '.flac', '.ogg'}.contains(extension)) {
      return Icons.audio_file_outlined;
    }
    if ({'.mp4', '.mov', '.webm', '.mkv'}.contains(extension)) {
      return Icons.video_file_outlined;
    }
    if (helixSupportedExtensions.contains(extension.replaceFirst('.', ''))) {
      return Icons.code;
    }
    return Icons.insert_drive_file_outlined;
  }
}
