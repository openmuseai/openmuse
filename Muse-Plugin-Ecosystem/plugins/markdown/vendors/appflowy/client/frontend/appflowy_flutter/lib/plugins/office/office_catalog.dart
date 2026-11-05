import 'dart:typed_data';

import 'package:appflowy/plugins/office/office_manifest.dart';
import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_page.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/user_profile.pb.dart';

/// Built-in catalog. Downloaded JSON can call [OfficePluginRegistry.register]
/// later (content-dynamic); Flutter UI Facets stay precompiled.
class OfficePluginCatalog {
  OfficePluginCatalog._();

  static bool _installed = false;

  static void ensureInstalled() {
    if (_installed && OfficePluginRegistry.instance.isInstalled) {
      return;
    }
    final registry = OfficePluginRegistry.instance;
    registry.resetForTest();
    registry.register(_word());
    registry.register(_excel());
    registry.register(_slides());
    registry.register(_pdf());
    _installed = true;
  }

  static OfficeManifest _word() => OfficeManifest(
        id: 'word',
        pluginId: 'muse.appflowy.word',
        familyId: 'muse.word',
        pluginType: PluginType.word,
        layout: ViewLayoutPB.Word,
        menuName: 'Word',
        importExtensions: const ['docx'],
        defaultPageName: 'Word',
        blobSubdir: 'word',
        fileExtension: 'docx',
        zipMagic: true,
        engineBound: true,
        creatable: true,
        desktopOnly: true,
        emptyTemplate: buildMinimalDocx,
        validateBytes: WordBlobStore.validateDocx,
        pageBuilder: ({
          required view,
          required user,
          required onDeleted,
        }) =>
            WordPage(
          view: view,
          user: user as UserProfilePB,
          onDeleted: onDeleted,
        ),
      );

  static OfficeManifest _excel() => OfficeManifest(
        id: 'excel',
        pluginId: 'muse.appflowy.excel',
        familyId: 'muse.excel',
        pluginType: PluginType.excel,
        layout: ViewLayoutPB.Excel,
        menuName: 'Excel',
        importExtensions: const ['xlsx'],
        defaultPageName: 'Excel',
        blobSubdir: 'excel',
        fileExtension: 'xlsx',
        zipMagic: true,
        engineBound: false,
        creatable: true,
        desktopOnly: true,
        emptyTemplate: emptyZipOfficePackage,
      );

  static OfficeManifest _slides() => OfficeManifest(
        id: 'slides',
        pluginId: 'muse.appflowy.slides',
        familyId: 'muse.slides',
        pluginType: PluginType.slides,
        layout: ViewLayoutPB.Slides,
        menuName: 'Slides',
        importExtensions: const ['pptx'],
        defaultPageName: 'Slides',
        blobSubdir: 'slides',
        fileExtension: 'pptx',
        zipMagic: true,
        engineBound: false,
        creatable: true,
        desktopOnly: true,
        emptyTemplate: emptyZipOfficePackage,
      );

  static OfficeManifest _pdf() => OfficeManifest(
        id: 'pdf',
        pluginId: 'muse.appflowy.pdf',
        familyId: 'muse.pdf',
        pluginType: PluginType.pdf,
        layout: ViewLayoutPB.Pdf,
        menuName: 'PDF',
        importExtensions: const ['pdf'],
        defaultPageName: 'PDF',
        blobSubdir: 'pdf',
        fileExtension: 'pdf',
        zipMagic: false,
        engineBound: false,
        creatable: true,
        desktopOnly: true,
        emptyTemplate: emptyPdf,
        validateBytes: validatePdf,
      );
}

/// Minimal PK zip so Folder can persist a blob before the engine exists.
Uint8List emptyZipOfficePackage() => Uint8List.fromList(const [
      0x50, 0x4b, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

Uint8List emptyPdf() => Uint8List.fromList(
      '%PDF-1.4\n1 0 obj<</Type/Catalog>>endobj\ntrailer<</Root 1 0 R>>\n%%EOF\n'
          .codeUnits,
    );

void validatePdf(Uint8List bytes) {
  if (bytes.length < 5 ||
      bytes[0] != 0x25 ||
      bytes[1] != 0x50 ||
      bytes[2] != 0x44 ||
      bytes[3] != 0x46) {
    throw const FormatException('PDF blob must start with %PDF');
  }
}
