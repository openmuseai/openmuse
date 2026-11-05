/// Office view-plugin protocol. Host code depends on this file, not on
/// word_editor / word_render / wasm-bindgen.
library;

import 'dart:typed_data';

import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flutter/widgets.dart';

typedef OfficePageBuilder = Widget Function({
  required ViewPB view,
  required dynamic user,
  required VoidCallback onDeleted,
});

/// One discoverable office slot. Layout values are the Folder contract:
/// Word=9, Excel=10, Slides=11, Pdf=12 (never 0–8).
class OfficeManifest {
  const OfficeManifest({
    required this.id,
    required this.pluginId,
    required this.familyId,
    required this.pluginType,
    required this.layout,
    required this.menuName,
    required this.importExtensions,
    required this.defaultPageName,
    required this.blobSubdir,
    required this.fileExtension,
    required this.zipMagic,
    required this.engineBound,
    required this.creatable,
    required this.desktopOnly,
    required this.emptyTemplate,
    this.validateBytes,
    this.pageBuilder,
  });

  final String id;
  final String pluginId;
  final String familyId;
  final PluginType pluginType;
  final ViewLayoutPB layout;
  final String menuName;
  final List<String> importExtensions;
  final String defaultPageName;
  final String blobSubdir;
  final String fileExtension;
  /// When true, bytes must start with `PK`. PDF uses false and `%PDF`.
  final bool zipMagic;
  final bool engineBound;
  final bool creatable;
  final bool desktopOnly;
  final Uint8List Function() emptyTemplate;
  final void Function(Uint8List bytes)? validateBytes;
  final OfficePageBuilder? pageBuilder;
}

class OfficePluginRegistry {
  OfficePluginRegistry._();
  static final OfficePluginRegistry instance = OfficePluginRegistry._();

  final Map<String, OfficeManifest> _byId = {};
  final Map<int, OfficeManifest> _byLayout = {};

  void register(OfficeManifest manifest) {
    _byId[manifest.id] = manifest;
    _byLayout[manifest.layout.value] = manifest;
  }

  OfficeManifest? byId(String id) => _byId[id];

  OfficeManifest? byLayout(ViewLayoutPB layout) => _byLayout[layout.value];

  OfficeManifest mustGet(String id) {
    final found = _byId[id];
    if (found == null) {
      throw StateError('Office plugin "$id" is not installed');
    }
    return found;
  }

  List<OfficeManifest> get all => List.unmodifiable(_byId.values);

  List<OfficeManifest> get creatable =>
      all.where((m) => m.creatable).toList(growable: false);

  bool get isInstalled => _byId.isNotEmpty;

  void resetForTest() {
    _byId.clear();
    _byLayout.clear();
  }
}

bool isOfficeLayout(ViewLayoutPB layout) =>
    OfficePluginRegistry.instance.byLayout(layout) != null;
