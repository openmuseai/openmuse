import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_file_open.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/settings/application_data_storage.dart';
import 'package:appflowy/workspace/application/view/view_service.dart';
import 'package:appflowy_backend/dispatch/dispatch.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/workspace.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/user_profile.pb.dart';
import 'package:path/path.dart' as p;

class WordBackendService {
  static Future<WordBlobStore> storeFor(UserProfilePB user) async {
    final root = await getIt<ApplicationDataStorage>().getPath();
    var workspaceId = 'unknown';
    final result = await FolderEventReadCurrentWorkspace().send();
    result.fold((WorkspacePB ws) => workspaceId = ws.id, (_) {});
    return WordBlobStore(
      root: Directory(p.join(root, user.id.toString(), 'word', workspaceId)),
    );
  }

  static Uint8List emptyTemplate() => buildMinimalDocx();

  static String pageNameFromFileName(String fileName) {
    final base = p.basenameWithoutExtension(fileName);
    return base.isEmpty ? 'Word' : base;
  }

  /// F1.2: pick local `.docx` files and create Word pages (blob, not Document).
  static Future<List<ViewPB>> pickAndImportDocx({
    required String parentViewId,
    bool allowMultiple = true,
    bool openLast = true,
  }) async {
    final files = await pickLocalDocxFiles(allowMultiple: allowMultiple);
    if (files.isEmpty) {
      return const [];
    }
    final created = <ViewPB>[];
    for (var i = 0; i < files.length; i++) {
      final file = files[i];
      try {
        WordBlobStore.validateDocx(file.bytes);
      } catch (_) {
        continue;
      }
      final view = await createWordView(
        parentViewId: parentViewId,
        name: pageNameFromFileName(file.name),
        bytes: file.bytes,
        openAfterCreate: openLast && i == files.length - 1,
      );
      if (view != null) {
        created.add(view);
      }
    }
    return created;
  }

  static Future<ViewPB?> createWordView({
    required String parentViewId,
    required String name,
    required Uint8List bytes,
    bool openAfterCreate = true,
  }) async {
    WordBlobStore.validateDocx(bytes);
    final result = await ViewBackendService.createView(
      parentViewId: parentViewId,
      name: name,
      layoutType: ViewLayoutPB.Word,
      initialDataBytes: bytes,
      openAfterCreate: openAfterCreate,
    );
    final view = result.fold<ViewPB?>((value) => value, (_) => null);
    if (view == null) {
      return null;
    }
    try {
      final profile = await UserEventGetUserProfile().send();
      final user = profile.fold<UserProfilePB?>((value) => value, (_) => null);
      if (user != null) {
        final store = await storeFor(user);
        await store.put(view.id, bytes);
      }
    } catch (_) {}
    return view;
  }
}
