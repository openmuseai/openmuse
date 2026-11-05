import 'package:appflowy/plugins/ai_chat/chat.dart';
import 'package:appflowy/plugins/database/calendar/calendar.dart';
import 'package:appflowy/plugins/database/board/board.dart';
import 'package:appflowy/plugins/database/grid/grid.dart';
import 'package:appflowy/plugins/database_document/database_document_plugin.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/plugins/blank/blank.dart';
import 'package:appflowy/plugins/document/document.dart';
import 'package:appflowy/plugins/trash/trash.dart';
import 'package:appflowy/plugins/office/office.dart';
import 'package:appflowy/plugins/word/word.dart';
import 'package:appflowy/plugins/resource_surface/resource_file_plugin.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_plugin.dart';
import 'package:appflowy/plugins/version_diff/presentation/image_overlay_diff_plugin.dart';

class PluginLoadTask extends LaunchTask {
  const PluginLoadTask();

  @override
  LaunchTaskType get type => LaunchTaskType.dataProcessing;

  @override
  Future<void> initialize(LaunchContext context) async {
    await super.initialize(context);

    registerPlugin(builder: BlankPluginBuilder(), config: BlankPluginConfig());
    registerPlugin(builder: TrashPluginBuilder(), config: TrashPluginConfig());
    registerPlugin(builder: DocumentPluginBuilder());
    registerPlugin(
      builder: MuseResourcePluginBuilder(),
      config: MuseResourcePluginConfig(),
    );
    registerPlugin(
      builder: MuseTextDiffPluginBuilder(),
      config: MuseTextDiffPluginConfig(),
    );
    registerPlugin(
      builder: MuseImageOverlayDiffPluginBuilder(),
      config: MuseImageOverlayDiffPluginConfig(),
    );
    registerPlugin(builder: GridPluginBuilder(), config: GridPluginConfig());
    registerPlugin(builder: BoardPluginBuilder(), config: BoardPluginConfig());
    registerPlugin(
      builder: CalendarPluginBuilder(),
      config: CalendarPluginConfig(),
    );
    registerPlugin(
      builder: DatabaseDocumentPluginBuilder(),
      config: DatabaseDocumentPluginConfig(),
    );
    registerPlugin(
      builder: DatabaseDocumentPluginBuilder(),
      config: DatabaseDocumentPluginConfig(),
    );
    registerPlugin(
      builder: AIChatPluginBuilder(),
      config: AIChatPluginConfig(),
    );
    OfficePluginCatalog.ensureInstalled();
    for (final manifest in OfficePluginRegistry.instance.creatable) {
      registerPlugin(
        builder: manifest.id == 'word'
            ? WordPluginBuilder()
            : OfficePluginBuilder(manifest),
        config: OfficePluginConfig(manifest),
      );
    }
  }
}
