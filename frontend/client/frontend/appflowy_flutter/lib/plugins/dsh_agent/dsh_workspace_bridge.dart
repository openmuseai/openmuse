import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';

/// Publishes the current AppFlowy workspace so the DSH Cordis plugin can
/// project it onto a DSH workspace cwd without forking DSH.
class DshWorkspaceBridge {
  static File get hintFile {
    return File(
      '${DshRuntimeLayout.defaultDshHome}/bindings/current-appflowy-workspace.json',
    );
  }

  static Future<void> publish(UserWorkspacePB workspace) async {
    final file = hintFile;
    await file.parent.create(recursive: true);
    final payload = jsonEncode({
      'appflowyWorkspaceId': workspace.workspaceId,
      'title': workspace.name,
      'updatedAt': DateTime.now().millisecondsSinceEpoch,
    });
    await file.writeAsString(payload);
  }
}
