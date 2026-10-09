import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;

/// Local data for one GoTrue subject. The subject never becomes a path segment.
final class HostAccountStorage {
  HostAccountStorage({required this.supportPath, required this.subject});

  final String supportPath;
  final String subject;

  String get root => p.join(
    supportPath,
    'OpenMuse',
    'accounts',
    sha256.convert(utf8.encode(subject)).toString(),
  );

  String get workspacePath => p.join(root, 'Workspace');
  String get mountsPath => p.join(root, 'workspace-mounts-v1.json');
  String get versionsPath => p.join(root, 'versions-v1');
  String get pluginInteractionsPath => p.join(root, 'plugin-interactions');

  String get dshHome {
    final configured = Platform.environment['MUSE_DSH_HOME'];
    if (configured != null && p.isAbsolute(configured)) {
      return p.join(configured, 'accounts', p.basename(root), 'dsh');
    }
    return p.join(root, 'dsh');
  }
}
