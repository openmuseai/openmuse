import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;

final class MuseLocalWorkspaceProvider implements MuseWorkspaceProvider {
  static const providerId = 'muse.workspace.local.v1';

  @override
  String get id => providerId;

  @override
  Set<MuseWorkspaceCapability> get capabilities => const {
        MuseWorkspaceCapability.metadataRead,
        MuseWorkspaceCapability.childrenList,
        MuseWorkspaceCapability.contentRead,
        MuseWorkspaceCapability.contentWrite,
        MuseWorkspaceCapability.createFile,
        MuseWorkspaceCapability.createDirectory,
        MuseWorkspaceCapability.rename,
        MuseWorkspaceCapability.delete,
        MuseWorkspaceCapability.import,
        MuseWorkspaceCapability.nativeReveal,
        MuseWorkspaceCapability.changesWatch,
      };

  @override
  Future<MuseWorkspaceEntry> bind(MuseWorkspaceMount mount) async {
    final root = Directory(mount.rootLocator);
    if (!await root.exists()) {
      throw FileSystemException(
        'Workspace directory does not exist',
        root.path,
      );
    }
    final canonical = await root.resolveSymbolicLinks();
    return _entry(
      mount: mount,
      locator: canonical,
      name: mount.displayName,
      kind: MuseWorkspaceEntryKind.directory,
      parentEntryRef: null,
    );
  }

  @override
  Future<List<MuseWorkspaceEntry>> listChildren({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
  }) async {
    _requireDirectory(parent);
    final parentPath = await _contained(mount, parent.locator);
    final entries = <MuseWorkspaceEntry>[];
    await for (final entity in Directory(parentPath).list(followLinks: false)) {
      final type = await FileSystemEntity.type(entity.path, followLinks: false);
      final kind = switch (type) {
        FileSystemEntityType.directory => MuseWorkspaceEntryKind.directory,
        FileSystemEntityType.file => MuseWorkspaceEntryKind.file,
        FileSystemEntityType.link => MuseWorkspaceEntryKind.symlink,
        _ => MuseWorkspaceEntryKind.unknown,
      };
      FileStat? stat;
      try {
        stat = await entity.stat();
      } on FileSystemException {
        // The entry can disappear between list and stat. Keep it navigable.
      }
      entries.add(
        _entry(
          mount: mount,
          locator: p.normalize(entity.absolute.path),
          name: p.basename(entity.path),
          kind: kind,
          parentEntryRef: parent.entryRef,
          size: kind == MuseWorkspaceEntryKind.file ? stat?.size : null,
          modifiedAt: stat?.modified,
        ),
      );
    }
    entries.sort((a, b) {
      if (a.isDirectory != b.isDirectory) return a.isDirectory ? -1 : 1;
      return a.name.toLowerCase().compareTo(b.name.toLowerCase());
    });
    return entries;
  }

  @override
  Future<MuseWorkspaceEntry> createFile({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required String name,
  }) async {
    final safeName = _validateName(name);
    final parentPath = await _contained(mount, parent.locator);
    final path = p.join(parentPath, safeName);
    await _ensureAbsent(path);
    await File(path).writeAsBytes(const [], flush: true);
    return _entry(
      mount: mount,
      locator: path,
      name: safeName,
      kind: MuseWorkspaceEntryKind.file,
      parentEntryRef: parent.entryRef,
      size: 0,
      modifiedAt: DateTime.now(),
    );
  }

  @override
  Future<MuseWorkspaceEntry> createDirectory({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required String name,
  }) async {
    final safeName = _validateName(name);
    final parentPath = await _contained(mount, parent.locator);
    final path = p.join(parentPath, safeName);
    await _ensureAbsent(path);
    await Directory(path).create();
    return _entry(
      mount: mount,
      locator: path,
      name: safeName,
      kind: MuseWorkspaceEntryKind.directory,
      parentEntryRef: parent.entryRef,
      modifiedAt: DateTime.now(),
    );
  }

  @override
  Future<MuseWorkspaceEntry> rename({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry entry,
    required String newName,
  }) async {
    final safeName = _validateName(newName);
    final current = await _contained(mount, entry.locator);
    final target = p.join(p.dirname(current), safeName);
    await _contained(mount, p.dirname(target));
    await _ensureAbsent(target);
    final renamed = await FileSystemEntity.type(current, followLinks: false) ==
            FileSystemEntityType.directory
        ? await Directory(current).rename(target)
        : await File(current).rename(target);
    return _entry(
      mount: mount,
      locator: renamed.path,
      name: safeName,
      kind: entry.kind,
      parentEntryRef: entry.parentEntryRef,
      size: entry.size,
      modifiedAt: DateTime.now(),
    );
  }

  @override
  Future<void> delete({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry entry,
  }) async {
    if (entry.parentEntryRef == null) {
      throw StateError('Mount root cannot be deleted');
    }
    final path = await _contained(mount, entry.locator);
    final type = await FileSystemEntity.type(path, followLinks: false);
    if (type == FileSystemEntityType.directory) {
      await Directory(path).delete(recursive: true);
    } else if (type != FileSystemEntityType.notFound) {
      await File(path).delete();
    }
  }

  @override
  Future<List<MuseWorkspaceEntry>> importFiles({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required List<File> files,
  }) async {
    final parentPath = await _contained(mount, parent.locator);
    final imported = <MuseWorkspaceEntry>[];
    for (final source in files) {
      if (!await source.exists()) continue;
      final name = _validateName(p.basename(source.path));
      final target = await _availablePath(parentPath, name);
      await source.copy(target);
      final stat = await File(target).stat();
      imported.add(
        _entry(
          mount: mount,
          locator: target,
          name: p.basename(target),
          kind: MuseWorkspaceEntryKind.file,
          parentEntryRef: parent.entryRef,
          size: stat.size,
          modifiedAt: stat.modified,
        ),
      );
    }
    return imported;
  }

  @override
  Stream<void> watch(MuseWorkspaceMount mount) async* {
    final root = Directory(mount.rootLocator);
    if (!await root.exists()) return;
    yield* root.watch(recursive: true).map((_) {});
  }

  MuseWorkspaceEntry _entry({
    required MuseWorkspaceMount mount,
    required String locator,
    required String name,
    required MuseWorkspaceEntryKind kind,
    required String? parentEntryRef,
    int? size,
    DateTime? modifiedAt,
  }) {
    final normalized = p.normalize(File(locator).absolute.path);
    final digest = sha256.convert(utf8.encode('${mount.mountRef}:$normalized'));
    final writable = !mount.readOnly;
    return MuseWorkspaceEntry(
      entryRef: 'entry:$digest',
      resourceRef: 'resource:$digest',
      mountRef: mount.mountRef,
      parentEntryRef: parentEntryRef,
      name: name,
      kind: kind,
      locator: normalized,
      capabilities: {
        MuseWorkspaceCapability.metadataRead,
        if (kind == MuseWorkspaceEntryKind.directory)
          MuseWorkspaceCapability.childrenList,
        if (kind == MuseWorkspaceEntryKind.file) ...{
          MuseWorkspaceCapability.contentRead,
          if (writable) MuseWorkspaceCapability.contentWrite,
        },
        if (writable) ...{
          MuseWorkspaceCapability.rename,
          MuseWorkspaceCapability.delete,
        },
        if (kind == MuseWorkspaceEntryKind.directory && writable) ...{
          MuseWorkspaceCapability.createFile,
          MuseWorkspaceCapability.createDirectory,
          MuseWorkspaceCapability.import,
        },
        MuseWorkspaceCapability.nativeReveal,
      },
      size: size,
      modifiedAt: modifiedAt,
    );
  }

  Future<String> _contained(MuseWorkspaceMount mount, String candidate) async {
    final root = await Directory(mount.rootLocator).resolveSymbolicLinks();
    final normalized = p.normalize(File(candidate).absolute.path);
    if (normalized != root && !p.isWithin(root, normalized)) {
      throw FileSystemException('Workspace path escapes mount', candidate);
    }
    return normalized;
  }

  void _requireDirectory(MuseWorkspaceEntry entry) {
    if (!entry.isDirectory) {
      throw StateError('${entry.name} is not a directory');
    }
  }

  String _validateName(String value) {
    final name = value.trim();
    if (name.isEmpty ||
        name == '.' ||
        name == '..' ||
        p.basename(name) != name) {
      throw const FormatException('Invalid workspace entry name');
    }
    return name;
  }

  Future<void> _ensureAbsent(String path) async {
    if (await FileSystemEntity.type(path, followLinks: false) !=
        FileSystemEntityType.notFound) {
      throw FileSystemException('Workspace entry already exists', path);
    }
  }

  Future<String> _availablePath(String directory, String name) async {
    var candidate = p.join(directory, name);
    if (await FileSystemEntity.type(candidate, followLinks: false) ==
        FileSystemEntityType.notFound) {
      return candidate;
    }
    final extension = p.extension(name);
    final base = p.basenameWithoutExtension(name);
    for (var index = 2; index < 10000; index++) {
      candidate = p.join(directory, '$base $index$extension');
      if (await FileSystemEntity.type(candidate, followLinks: false) ==
          FileSystemEntityType.notFound) {
        return candidate;
      }
    }
    throw FileSystemException('Unable to allocate import name', name);
  }
}
