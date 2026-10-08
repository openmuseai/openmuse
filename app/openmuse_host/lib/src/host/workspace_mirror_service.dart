import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path/path.dart' as p;

import 'workspace_controller.dart';

/// Desktop-owned, read-only directory authority for an authorized pair.
/// References are random handles; browser responses never contain local paths.
final class DesktopWorkspaceMirrorService {
  DesktopWorkspaceMirrorService(this.workspace);

  final LocalWorkspaceController workspace;
  final Random _random = Random.secure();
  final Map<String, _MirrorPath> _paths = {};
  final Map<String, String> _refsByPath = {};

  Future<Map<String, Object?>> handle(WorkspaceMirrorQuery query) async {
    if (query.operation == 'mounts') {
      return {
        'mounts': [
          for (final mount in workspace.mounts)
            {
              'mountRef': _ref(mount.path, mount.path),
              'rootRef': _ref(mount.path, mount.path),
              'title': mount.name,
            },
        ],
      };
    }
    if (query.operation != 'children') throw const FormatException();
    final mount = _paths[query.mountRef];
    final parent = _paths[query.parentRef];
    if (mount == null ||
        mount.path != mount.mountPath ||
        parent == null ||
        parent.mountPath != mount.path ||
        !workspace.mounts.any((value) => value.path == mount.path) ||
        !_inside(mount.path, parent.path)) {
      throw const FormatException();
    }
    if (await FileSystemEntity.type(parent.path, followLinks: false) !=
        FileSystemEntityType.directory) {
      throw const FormatException();
    }
    // A directory can be replaced after its opaque ref was issued. Refuse a
    // symlink hop through any ancestor before enumerating it.
    final resolved = await Directory(parent.path).resolveSymbolicLinks();
    if (!_inside(mount.path, resolved)) throw const FormatException();
    final offset = _decodeCursor(query.cursor, query.parentRef!);
    final entries = <({String path, String name, bool directory})>[];
    await for (final entity in Directory(
      parent.path,
    ).list(followLinks: false)) {
      final type = await FileSystemEntity.type(entity.path, followLinks: false);
      if (type != FileSystemEntityType.file &&
          type != FileSystemEntityType.directory) {
        continue;
      }
      final path = p.normalize(p.absolute(entity.path));
      if (!_inside(mount.path, path)) continue;
      entries.add((
        path: path,
        name: p.basename(path),
        directory: type == FileSystemEntityType.directory,
      ));
    }
    entries.sort((a, b) {
      if (a.directory != b.directory) return a.directory ? -1 : 1;
      return a.name.toLowerCase().compareTo(b.name.toLowerCase());
    });
    if (offset > entries.length) throw const FormatException();
    final end = min(offset + query.limit, entries.length);
    return {
      'entries': [
        for (final entry in entries.sublist(offset, end))
          {
            'nodeRef': _ref(mount.path, entry.path),
            'name': entry.name,
            'isDirectory': entry.directory,
            if (!entry.directory) 'resourceRef': _ref(mount.path, entry.path),
          },
      ],
      'nextCursor': end < entries.length
          ? base64Url.encode(utf8.encode('${query.parentRef}:$end'))
          : null,
    };
  }

  Future<WorkspaceMirrorResource> readResource(
    WorkspaceMirrorResourceQuery query,
  ) async {
    final resource = _paths[query.resourceRef];
    if (resource == null ||
        !workspace.mounts.any((mount) => mount.path == resource.mountPath) ||
        !_inside(resource.mountPath, resource.path) ||
        await FileSystemEntity.type(resource.path, followLinks: false) !=
            FileSystemEntityType.file) {
      throw const FormatException();
    }
    final resolved = await File(resource.path).resolveSymbolicLinks();
    if (!_inside(resource.mountPath, resolved)) throw const FormatException();
    final file = File(resource.path);
    if (await file.length() > 16 * 1024 * 1024) throw const FormatException();
    final extension = p.extension(resource.path).toLowerCase();
    final mediaType = switch (extension) {
      '.md' || '.markdown' || '.mdown' => 'text/markdown; charset=utf-8',
      '.txt' || '.log' => 'text/plain; charset=utf-8',
      '.json' => 'application/json',
      '.png' => 'image/png',
      '.jpg' || '.jpeg' => 'image/jpeg',
      '.gif' => 'image/gif',
      '.webp' => 'image/webp',
      '.svg' => 'image/svg+xml',
      '.pdf' => 'application/pdf',
      _ => 'application/octet-stream',
    };
    return WorkspaceMirrorResource(
      bytes: Uint8List.fromList(await file.readAsBytes()),
      mediaType: mediaType,
    );
  }

  String _ref(String mountPath, String path) {
    final key = '$mountPath\x00$path';
    final existing = _refsByPath[key];
    if (existing != null) return existing;
    final ref = List<int>.generate(
      24,
      (_) => _random.nextInt(256),
    ).map((byte) => byte.toRadixString(16).padLeft(2, '0')).join();
    _paths[ref] = _MirrorPath(mountPath, path);
    _refsByPath[key] = ref;
    return ref;
  }

  int _decodeCursor(String? cursor, String parentRef) {
    if (cursor == null) return 0;
    try {
      final value = utf8.decode(base64Url.decode(base64Url.normalize(cursor)));
      if (!value.startsWith('$parentRef:')) throw const FormatException();
      final offset = int.parse(value.substring(parentRef.length + 1));
      if (offset < 0) throw const FormatException();
      return offset;
    } on Object {
      throw const FormatException();
    }
  }

  bool _inside(String root, String path) =>
      root == path || p.isWithin(root, path);
}

final class _MirrorPath {
  const _MirrorPath(this.mountPath, this.path);

  final String mountPath;
  final String path;
}
