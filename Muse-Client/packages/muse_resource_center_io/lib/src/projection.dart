import 'dart:io';
import 'dart:convert';
import 'package:muse_resource_center/muse_resource_center.dart';

/// A private execution adapter object. Its path is never serialized into a
/// resource, history entry, clipboard reference, message or network response.
final class ResourceProjection {
  ResourceProjection._(
    this.file,
    this.expiresAtMs,
    this.resource,
    this._directory,
  );
  final File file;
  final int expiresAtMs;
  final OpenedResource resource;
  final Directory _directory;
  bool _disposed = false;
  Future<void> dispose() async {
    if (_disposed) return;
    _disposed = true;
    if (await _directory.exists()) await _directory.delete(recursive: true);
  }
}

final class ResourceProjectionManager {
  ResourceProjectionManager(this.root, {int Function()? clock})
    : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);
  final Directory root;
  final int Function() _clock;
  final _active = <ResourceProjection>[];
  Future<ResourceProjection> materialize(
    OpenedResource resource, {
    Duration ttl = const Duration(minutes: 15),
  }) async {
    if (ttl <= Duration.zero || ttl > const Duration(hours: 1))
      throw const ResourceFailure('INVALID_LEASE');
    await root.create(recursive: true);
    if (await FileSystemEntity.isLink(root.path))
      throw const ResourceFailure('UNSAFE_STORAGE');
    final dir = await root.createTemp('resource-lease-');
    final extension =
        RegExp(
          r'\.([a-zA-Z0-9]{1,10})$',
        ).firstMatch(resource.content.record.name)?.group(1) ??
        'bin';
    final f = File('${dir.path}/input.$extension');
    await f.writeAsBytes(resource.content.bytes, flush: true);
    final expiry = _clock() + ttl.inMilliseconds;
    await File('${dir.path}/lease.json').writeAsString(
      jsonEncode({
        'schema': 'openmuse.execution-lease/v1',
        'expiresAtMs': expiry,
      }),
      flush: true,
    );
    final p = ResourceProjection._(f, expiry, resource, dir);
    _active.add(p);
    return p;
  }

  Future<void> sweep() async {
    for (final p in _active.where((p) => p.expiresAtMs <= _clock()).toList()) {
      await p.dispose();
      _active.remove(p);
    }
    // Recover only our own expired leases after process restart. Unrelated or
    // malformed entries are never recursively removed by the janitor.
    if (!await root.exists() || await FileSystemEntity.isLink(root.path))
      return;
    await for (final entry in root.list(followLinks: false)) {
      if (entry is! Directory ||
          !entry.path
              .split(Platform.pathSeparator)
              .last
              .startsWith('resource-lease-'))
        continue;
      final marker = File('${entry.path}/lease.json');
      try {
        if (!await marker.exists() ||
            await FileSystemEntity.isLink(marker.path) ||
            await marker.length() > 1024)
          continue;
        final data = jsonDecode(await marker.readAsString());
        if (data is Map &&
            data['schema'] == 'openmuse.execution-lease/v1' &&
            data['expiresAtMs'] is int &&
            data['expiresAtMs'] <= _clock())
          await entry.delete(recursive: true);
      } on FileSystemException {
        /* Already removed by a concurrent lease owner. */
      } on FormatException {
        /* Unknown files remain untouched. */
      }
    }
  }

  Future<void> clear() async {
    for (final p in _active) {
      await p.dispose();
    }
    _active.clear();
  }
}
