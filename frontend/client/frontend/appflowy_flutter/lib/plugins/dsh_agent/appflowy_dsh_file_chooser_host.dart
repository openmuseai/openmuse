import 'package:appflowy/shared/permission/permission_checker.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

/// AppFlowy Photo Picker / Camera / SAF adapter. Returns URI strings only.
class AppFlowyDshFileChooserHost implements DshFileChooserHost {
  AppFlowyDshFileChooserHost({required this.context});

  final BuildContext Function() context;

  @override
  Future<List<String>> chooseFiles(DshFileChooserRequest request) async {
    final ctx = context();
    if (!ctx.mounted) return const [];
    final choice = await showModalBottomSheet<_ChooserAction>(
      context: ctx,
      builder: (sheet) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.photo_library_outlined),
              title: const Text('照片'),
              onTap: () => Navigator.pop(sheet, _ChooserAction.photo),
            ),
            ListTile(
              leading: const Icon(Icons.photo_camera_outlined),
              title: const Text('拍照'),
              onTap: () => Navigator.pop(sheet, _ChooserAction.camera),
            ),
            ListTile(
              leading: const Icon(Icons.folder_open_outlined),
              title: const Text('文件'),
              onTap: () => Navigator.pop(sheet, _ChooserAction.file),
            ),
            ListTile(
              title: const Text('取消'),
              onTap: () => Navigator.pop(sheet),
            ),
          ],
        ),
      ),
    );
    if (choice == null) return const [];
    final next = context();
    if (!next.mounted) return const [];
    switch (choice) {
      case _ChooserAction.photo:
        return _pickImages(next, request: request, camera: false);
      case _ChooserAction.camera:
        return _pickImages(next, request: request, camera: true);
      case _ChooserAction.file:
        return _pickFiles(request);
    }
  }

  Future<List<String>> _pickImages(
    BuildContext ctx, {
    required DshFileChooserRequest request,
    required bool camera,
  }) async {
    final allowed = camera
        ? await PermissionChecker.checkCameraPermission(ctx)
        : await PermissionChecker.checkPhotoPermission(ctx);
    if (!allowed || !ctx.mounted) return const [];
    final picker = ImagePicker();
    if (camera) {
      final file = await picker.pickImage(source: ImageSource.camera);
      return _urisOf(file == null ? const [] : [file]);
    }
    if (request.mode == DshFileChooserMode.openMultiple) {
      final files = await picker.pickMultiImage();
      return _urisOf(files);
    }
    final file = await picker.pickImage(source: ImageSource.gallery);
    return _urisOf(file == null ? const [] : [file]);
  }

  Future<List<String>> _pickFiles(DshFileChooserRequest request) async {
    final result = await getIt<FilePickerService>().pickFiles(
      allowMultiple: request.mode == DshFileChooserMode.openMultiple,
      type: FileType.custom,
      allowedExtensions: const ['png', 'jpg', 'jpeg', 'webp', 'gif'],
    );
    if (result == null) return const [];
    return [
      for (final file in result.files)
        if (file.path != null) _toChooserUri(file.path!),
    ];
  }

  List<String> _urisOf(List<XFile> files) => [
        for (final file in files) _toChooserUri(file.path),
      ];

  static String _toChooserUri(String path) {
    if (path.startsWith('content:') || path.startsWith('file:')) return path;
    return Uri.file(path).toString();
  }
}

enum _ChooserAction { photo, camera, file }
