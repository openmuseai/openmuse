import 'dart:io';

import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

import 'workspace_controller.dart';

/// Host navigation adapter for a durable PDF product output.
Future<void> openGeneratedPdfInWorkspace(
  LocalWorkspaceController workspace,
  String path,
) async {
  if (!p.isAbsolute(path) || p.extension(path).toLowerCase() != '.pdf') {
    throw const FormatException(
      'Generated PDF path must be an absolute PDF file',
    );
  }
  final file = File(path);
  if (!await file.exists()) {
    throw FileSystemException('Generated PDF is missing', path);
  }
  workspace.openResource(
    OpenMuseResource(
      uri: file.uri,
      displayName: p.basename(path),
      mediaType: 'application/pdf',
    ),
  );
}
