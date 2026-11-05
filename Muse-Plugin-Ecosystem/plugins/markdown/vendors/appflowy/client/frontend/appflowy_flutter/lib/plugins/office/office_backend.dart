import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/office/office_blob_store.dart';
import 'package:appflowy/plugins/office/office_manifest.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_file_open.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/settings/application_data_storage.dart';
import 'package:appflowy/workspace/application/view/view_service.dart';
import 'package:appflowy_backend/dispatch/dispatch.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/workspace.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/user_profile.pb.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:path/path.dart' as p;

class OfficeBackendService {
  static Future<OfficeBlobStore> storeFor(
    UserProfilePB user,
    OfficeManifest manifest,
  ) async {
    final root = await getIt<ApplicationDataStorage>().getPath();
    var workspaceId = 'unknown';
    final result = await FolderEventReadCurrentWorkspace().send();
    result.fold((WorkspacePB ws) => workspaceId = ws.id, (_) {});
    return OfficeBlobStore(
      root: Directory(
        p.join(root, user.id.toString(), manifest.blobSubdir, workspaceId),
      ),
      extension: manifest.fileExtension,
      validate: (bytes) => _validate(manifest, bytes),
    );
  }

  static String pageNameFromFileName(OfficeManifest manifest, String fileName) {
    final base = p.basenameWithoutExtension(fileName);
    return base.isEmpty ? manifest.defaultPageName : base;
  }

  static Future<ViewPB?> createView({
    required String parentViewId,
    required OfficeManifest manifest,
    required String name,
    required Uint8List bytes,
    bool openAfterCreate = true,
  }) async {
    _validate(manifest, bytes);
    final result = await ViewBackendService.createView(
      parentViewId: parentViewId,
      name: name,
      layoutType: manifest.layout,
      initialDataBytes: bytes,
      openAfterCreate: openAfterCreate,
    );
    return result.fold<ViewPB?>((value) => value, (_) => null);
  }

  static Future<List<ViewPB>> pickAndImport({
    required String parentViewId,
    required OfficeManifest manifest,
    bool allowMultiple = true,
  }) async {
    final safeDir = await wordSafeOpenDirectory();
    final result = await getIt<FilePickerService>().pickFiles(
      type: FileType.custom,
      allowMultiple: allowMultiple,
      allowedExtensions: manifest.importExtensions,
      withData: true,
      initialDirectory: safeDir.path,
    );
    if (result == null || result.files.isEmpty) {
      return const [];
    }
    final created = <ViewPB>[];
    for (var i = 0; i < result.files.length; i++) {
      final file = result.files[i];
      final bytes = file.bytes;
      if (bytes == null) continue;
      try {
        _validate(manifest, bytes);
      } catch (_) {
        continue;
      }
      final view = await createView(
        parentViewId: parentViewId,
        manifest: manifest,
        name: pageNameFromFileName(manifest, file.name),
        bytes: bytes,
        openAfterCreate: i == result.files.length - 1,
      );
      if (view != null) created.add(view);
    }
    return created;
  }

  static void _validate(OfficeManifest manifest, Uint8List bytes) {
    if (manifest.validateBytes != null) {
      manifest.validateBytes!(bytes);
      return;
    }
    if (manifest.zipMagic) {
      WordBlobStore.validateDocx(bytes);
    }
  }
}
