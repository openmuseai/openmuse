import 'dart:typed_data';

final class RemoteMediaAuthority {
  final _grants = <String, _MediaGrant>{};
  var _serial = 0;

  static final _handle = RegExp(r'^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$');

  String issue({
    required Uint8List bytes,
    required String workspaceRef,
    required String deviceRef,
    required DateTime expiresAt,
    String? handle,
  }) {
    final id = handle ?? 'media${++_serial}';
    if (!_handle.hasMatch(id)) {
      throw const FormatException('media handle is invalid');
    }
    _grants[id] = _MediaGrant(
      bytes: Uint8List.fromList(bytes),
      workspaceRef: workspaceRef,
      deviceRef: deviceRef,
      expiresAt: expiresAt,
    );
    return id;
  }

  Uint8List? read({
    required String handle,
    required String workspaceRef,
    required String deviceRef,
    required DateTime now,
  }) {
    final grant = _grants[handle];
    if (grant == null || !now.isBefore(grant.expiresAt)) return null;
    if (grant.workspaceRef != workspaceRef || grant.deviceRef != deviceRef) {
      return null;
    }
    return Uint8List.fromList(grant.bytes);
  }

  RemoteMediaRead readRange({
    required String handle,
    required String workspaceRef,
    required String deviceRef,
    required DateTime now,
    required int start,
    int? endInclusive,
  }) {
    final grant = _grants[handle];
    if (grant == null ||
        !now.isBefore(grant.expiresAt) ||
        grant.workspaceRef != workspaceRef ||
        grant.deviceRef != deviceRef) {
      return const RemoteMediaDenied();
    }
    final total = grant.bytes.length;
    if (start < 0 || start > total) return const RemoteMediaUnsatisfiable();
    final end = endInclusive ?? (total == 0 ? -1 : total - 1);
    if (total == 0) {
      return RemoteMediaBytes(bytes: Uint8List(0), total: 0, start: 0);
    }
    if (end < start || end >= total) return const RemoteMediaUnsatisfiable();
    return RemoteMediaBytes(
      bytes: Uint8List.sublistView(grant.bytes, start, end + 1),
      total: total,
      start: start,
    );
  }
}

sealed class RemoteMediaRead {
  const RemoteMediaRead();
}

final class RemoteMediaDenied extends RemoteMediaRead {
  const RemoteMediaDenied();
}

final class RemoteMediaUnsatisfiable extends RemoteMediaRead {
  const RemoteMediaUnsatisfiable();
}

final class RemoteMediaBytes extends RemoteMediaRead {
  RemoteMediaBytes({
    required this.bytes,
    required this.total,
    required this.start,
  });

  final Uint8List bytes;
  final int total;
  final int start;
}

final class _MediaGrant {
  const _MediaGrant({
    required this.bytes,
    required this.workspaceRef,
    required this.deviceRef,
    required this.expiresAt,
  });

  final Uint8List bytes;
  final String workspaceRef;
  final String deviceRef;
  final DateTime expiresAt;
}
