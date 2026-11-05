import 'dart:convert';
import 'dart:io';

import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

typedef MuseWorkspaceStorageRoot = Future<Directory> Function();

final class MuseWorkspacePersistence {
  MuseWorkspacePersistence({MuseWorkspaceStorageRoot? rootResolver})
      : _rootResolver = rootResolver ?? _defaultRoot;

  final MuseWorkspaceStorageRoot _rootResolver;

  static Future<Directory> _defaultRoot() async {
    final support = await getApplicationSupportDirectory();
    return Directory(p.join(support.path, 'OpenMuse', 'workspace-platform-v1'));
  }

  Future<MuseWorkspaceSnapshot> load(String accountSpaceRef) async {
    final file = await _file(accountSpaceRef);
    if (!await file.exists()) {
      return MuseWorkspaceSnapshot(
        accountSpaceRef: accountSpaceRef,
        mounts: const [],
        expandedEntryRefs: const {},
      );
    }
    try {
      final json =
          jsonDecode(await file.readAsString()) as Map<String, dynamic>;
      final mounts = (json['mounts'] as List<dynamic>? ?? const [])
          .whereType<Map<String, dynamic>>()
          .map(MuseWorkspaceMount.fromJson)
          .toList(growable: false)
        ..sort((a, b) => a.order.compareTo(b.order));
      final expanded = (json['expandedEntryRefs'] as List<dynamic>? ?? const [])
          .whereType<String>()
          .toSet();
      return MuseWorkspaceSnapshot(
        accountSpaceRef: accountSpaceRef,
        mounts: mounts,
        expandedEntryRefs: expanded,
        activeMountRef: json['activeMountRef'] as String?,
      );
    } on Object {
      return MuseWorkspaceSnapshot(
        accountSpaceRef: accountSpaceRef,
        mounts: const [],
        expandedEntryRefs: const {},
      );
    }
  }

  Future<void> save(MuseWorkspaceSnapshot snapshot) async {
    final file = await _file(snapshot.accountSpaceRef);
    await file.parent.create(recursive: true);
    final temporary = File('${file.path}.tmp');
    await temporary.writeAsString(
      const JsonEncoder.withIndent('  ').convert({
        'protocol': 'muse.workspace/definition/v1',
        'accountSpaceRef': snapshot.accountSpaceRef,
        'mounts': snapshot.mounts.map((mount) => mount.toJson()).toList(),
        'expandedEntryRefs': snapshot.expandedEntryRefs.toList()..sort(),
        if (snapshot.activeMountRef != null)
          'activeMountRef': snapshot.activeMountRef,
      }),
      flush: true,
    );
    if (await file.exists()) await file.delete();
    await temporary.rename(file.path);
  }

  Future<File> _file(String accountSpaceRef) async {
    final root = await _rootResolver();
    final safe = accountSpaceRef.replaceAll(RegExp('[^a-zA-Z0-9._-]'), '_');
    return File(p.join(root.path, 'definitions', '$safe.json'));
  }
}
