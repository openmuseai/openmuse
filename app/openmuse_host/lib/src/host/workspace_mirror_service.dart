import 'dart:collection';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path/path.dart' as p;

import 'workspace_controller.dart';
import 'resource_inspector.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Desktop-owned directory authority for an authorized pair.
/// References are random handles; browser responses never contain local paths.
final class DesktopWorkspaceMirrorService {
  DesktopWorkspaceMirrorService(this.workspace);

  final LocalWorkspaceController workspace;
  final Random _random = Random.secure();
  final Map<String, _MirrorPath> _paths = {};
  final Map<String, String> _refsByPath = {};

  Future<Map<String, Object?>> act(WorkspaceMirrorAction action) async {
    if (action.operation == 'add-workspace') {
      final input = action.name?.trim() ?? '';
      if (input.isEmpty) throw const FormatException();
      String target;
      if (p.isAbsolute(input)) {
        target = input;
      } else {
        _validateName(input);
        target = p.join(p.dirname(workspace.rootPath), 'Workspaces', input);
        await Directory(target).create(recursive: true);
      }
      await workspace.addWorkspace(target);
      return {'ok': true};
    }
    final ref = _paths[action.nodeRef];
    if (ref == null || !workspace.mounts.any((m) => m.path == ref.mountPath)) {
      throw const FormatException();
    }
    final type = await FileSystemEntity.type(ref.path, followLinks: false);
    if (type != FileSystemEntityType.file &&
        type != FileSystemEntityType.directory) {
      throw const FormatException();
    }
    final resolved = type == FileSystemEntityType.directory
        ? await Directory(ref.path).resolveSymbolicLinks()
        : await File(ref.path).resolveSymbolicLinks();
    if (!_inside(ref.mountPath, resolved)) throw const FormatException();
    if (action.operation == 'mount') {
      if (type != FileSystemEntityType.directory) throw const FormatException();
      await workspace.addWorkspace(ref.path);
    } else if (action.operation == 'unmount') {
      if (ref.path != ref.mountPath || workspace.mounts.length < 2) {
        throw const FormatException();
      }
      await workspace.removeWorkspace(
        workspace.mounts.firstWhere((m) => m.path == ref.path),
      );
    } else {
      final entry = await _entry(ref);
      if (action.operation == 'new-file' || action.operation == 'new-folder') {
        _validateName(action.name ?? '');
        await workspace.createEntry(
          entry,
          action.name!,
          directory: action.operation == 'new-folder',
        );
      } else {
        if (ref.path == ref.mountPath) throw const FormatException();
        if (ref.path == p.join(ref.mountPath, '.openmuse-trash')) {
          throw const FormatException();
        }
        if (action.operation == 'restore') {
          if (p.dirname(ref.path) != p.join(ref.mountPath, '.openmuse-trash')) {
            throw const FormatException();
          }
          final match = RegExp(r'^\d+-(.+)$').firstMatch(p.basename(ref.path));
          if (match == null) throw const FormatException();
          _validateName(match.group(1)!);
          final target = p.join(ref.mountPath, match.group(1)!);
          if (await FileSystemEntity.type(target, followLinks: false) !=
              FileSystemEntityType.notFound) {
            throw const FileSystemException('目标已存在');
          }
          if (entry.isDirectory) {
            await Directory(ref.path).rename(target);
          } else {
            await File(ref.path).rename(target);
          }
          await workspace.refreshDirectory(
            workspace.mounts.firstWhere((m) => m.path == ref.mountPath).root,
          );
        } else if (action.operation == 'rename') {
          _validateName(action.name ?? '');
          await workspace.renameEntry(entry, action.name!);
        } else if (action.operation == 'trash') {
          // Recoverable through the normal workspace tree, never permanent delete.
          final trashPath = p.join(ref.mountPath, '.openmuse-trash');
          if (_inside(trashPath, ref.path)) throw const FormatException();
          if (await FileSystemEntity.type(trashPath, followLinks: false) ==
              FileSystemEntityType.link) {
            throw const FormatException();
          }
          await Directory(trashPath).create();
          final target = p.join(
            trashPath,
            '${DateTime.now().microsecondsSinceEpoch}-${p.basename(ref.path)}',
          );
          if (entry.isDirectory) {
            await Directory(ref.path).rename(target);
          } else {
            await File(ref.path).rename(target);
          }
          await workspace.refreshDirectory(
            workspace.mounts.firstWhere((m) => m.path == ref.mountPath).root,
          );
        } else {
          throw const FormatException();
        }
      }
    }
    return {'ok': true};
  }

  void _validateName(String name) {
    if (name.trim().isEmpty ||
        name != name.trim() ||
        name == '.' ||
        name == '..' ||
        name.contains('/') ||
        name.contains('\\') ||
        name.contains('\x00')) {
      throw const FormatException();
    }
  }

  Future<WorkspaceEntry> _entry(_MirrorPath ref) async {
    var entry = workspace.mounts
        .firstWhere((m) => m.path == ref.mountPath)
        .root;
    if (ref.path == ref.mountPath) return entry;
    for (final part in p.split(p.relative(ref.path, from: ref.mountPath))) {
      await workspace.refreshDirectory(entry);
      entry = entry.children.firstWhere(
        (e) => e.name == part,
        orElse: () => throw const FormatException(),
      );
    }
    return entry;
  }

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
    if (query.operation == 'search') return _search(query);
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
            'canRestore':
                p.dirname(entry.path) == p.join(mount.path, '.openmuse-trash'),
            if (!entry.directory) 'resourceRef': _ref(mount.path, entry.path),
          },
      ],
      'nextCursor': end < entries.length
          ? base64Url.encode(utf8.encode('${query.parentRef}:$end'))
          : null,
    };
  }

  /// Bounded filename search. Results contain opaque references only, and
  /// symlinks are never followed or exposed as navigable nodes.
  Future<Map<String, Object?>> _search(WorkspaceMirrorQuery query) async {
    final needle = query.searchText?.trim().toLowerCase();
    if (needle == null || needle.isEmpty || needle.length > 120) {
      throw const FormatException();
    }
    final found = <Map<String, Object?>>[];
    var visited = 0;
    for (final mount in workspace.mounts) {
      final root = mount.path;
      final mountRef = _ref(root, root);
      final queue = Queue<Directory>()..add(Directory(root));
      while (queue.isNotEmpty && visited < 2500 && found.length < query.limit) {
        final parent = queue.removeFirst();
        await for (final entity in parent.list(followLinks: false)) {
          if (++visited > 2500 || found.length >= query.limit) break;
          final normalized = p.normalize(p.absolute(entity.path));
          final type = await FileSystemEntity.type(
            normalized,
            followLinks: false,
          );
          if (!_inside(root, normalized) ||
              type == FileSystemEntityType.link ||
              (type != FileSystemEntityType.directory &&
                  type != FileSystemEntityType.file)) {
            continue;
          }
          final name = p.basename(normalized);
          final directory = type == FileSystemEntityType.directory;
          if (directory &&
              !const {
                '.git',
                'node_modules',
                '.dart_tool',
                'build',
              }.contains(name)) {
            queue.add(Directory(normalized));
          }
          if (!name.toLowerCase().contains(needle)) continue;
          found.add({
            'mountRef': mountRef,
            'nodeRef': _ref(root, normalized),
            'name': name,
            'isDirectory': directory,
            if (!directory) 'resourceRef': _ref(root, normalized),
          });
        }
      }
      if (visited > 2500 || found.length >= query.limit) break;
    }
    return {
      'entries': found,
      'truncated': visited > 2500 || found.length >= query.limit,
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
    if (await file.length() > 64 * 1024 * 1024) throw const FormatException();
    final inspected = await inspectLocalResource(
      OpenMuseResource(uri: file.uri, displayName: p.basename(file.path)),
    );
    return WorkspaceMirrorResource(
      bytes: Uint8List.fromList(await file.readAsBytes()),
      mediaType: inspected.mediaType ?? 'application/octet-stream',
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
