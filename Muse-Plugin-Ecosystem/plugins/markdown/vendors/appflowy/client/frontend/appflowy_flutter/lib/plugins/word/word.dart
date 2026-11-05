library;

import 'package:appflowy/plugins/office/office.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';

/// Compatibility wrappers. New host code should use [OfficePluginBuilder].
class WordPluginBuilder extends OfficePluginBuilder {
  WordPluginBuilder() : super(_manifest());

  static OfficeManifest _manifest() {
    OfficePluginCatalog.ensureInstalled();
    return OfficePluginRegistry.instance.mustGet('word');
  }

  @override
  Plugin build(dynamic data) {
    if (data is ViewPB) {
      return WordPlugin(view: data);
    }
    throw FlowyPluginException.invalidData;
  }
}

class WordPluginConfig extends OfficePluginConfig {
  WordPluginConfig() : super(_manifest());

  static OfficeManifest _manifest() {
    OfficePluginCatalog.ensureInstalled();
    return OfficePluginRegistry.instance.mustGet('word');
  }
}

class WordPlugin extends OfficePlugin {
  WordPlugin({required super.view});
}
