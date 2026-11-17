import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path_provider/path_provider.dart';

/// Account/device scoped recent history; Desktop remains authoritative.
final class MobileDshRecentMessages {
  MobileDshRecentMessages(
    this.scope, {
    Future<Directory> Function()? cacheDirectory,
  }) : _cacheDirectory = cacheDirectory ?? getApplicationCacheDirectory {
    ready = _load();
  }
  final Future<Directory> Function() _cacheDirectory;
  final String scope;
  final _pages = <String, DshSnapshotFrame>{};
  late final Future<void> ready;
  File? _file;
  Timer? _saveTimer;
  Future<void> _writes = Future<void>.value();

  DshSnapshotFrame? read(String id) => _pages[id];

  void put(String id, DshSnapshotFrame page) {
    // Do not persist huge tool results or credentials belonging to transport.
    if (utf8.encode(jsonEncode(_encode(page))).length > 128 * 1024) return;
    _pages.remove(id);
    _pages[id] = page;
    while (_pages.length > 12) {
      _pages.remove(_pages.keys.first);
    }
    _saveTimer?.cancel();
    _saveTimer = Timer(const Duration(milliseconds: 500), _save);
  }

  Future<void> _load() async {
    try {
      final root = await _cacheDirectory();
      final key = base64Url.encode(utf8.encode(scope));
      _file = File('${root.path}/dsh-recent-$key.json');
      if (!await _file!.exists()) return;
      if (await _file!.length() > 2 * 1024 * 1024) return;
      final decoded = jsonDecode(await _file!.readAsString());
      if (decoded is! Map) return;
      for (final entry in decoded.entries.take(12)) {
        final frame = DshFollowFrame.fromJson(
          (entry.value as Map).cast<String, Object?>(),
        );
        if (frame is DshSnapshotFrame && frame.header['id'] == entry.key) {
          _pages.putIfAbsent(entry.key as String, () => frame);
        }
      }
    } on Object {
      /* Cache misses never block networking. */
    }
  }

  static JsonMap _encode(DshSnapshotFrame page) => {
    'type': 'snapshot',
    'header': page.header,
    'cursor': page.cursor,
    'hasMore': page.hasMore,
    'projections': page.projections,
    'records': [
      for (final event in page.records) {'type': 'event', 'event': event.raw},
    ],
  };

  void _save() {
    final file = _file;
    if (file == null) return;
    final data = jsonEncode({
      for (final entry in _pages.entries) entry.key: _encode(entry.value),
    });
    _writes = _writes
        .then((_) async {
          final temp = File('${file.path}.tmp');
          await temp.writeAsString(data, flush: true);
          await temp.rename(file.path);
        })
        .catchError((Object _) {});
  }

  Future<void> flush() async {
    _saveTimer?.cancel();
    await ready;
    _save();
    await _writes;
  }

  void dispose() {
    unawaited(flush());
  }
}

/// One HTTP keep-alive pool and cookie/bootstrap lifecycle per Desktop grant.
final class MobileDshConnection {
  MobileDshConnection(PairedDesktopConnection connection, this.messages)
    : client = DshNativeGatewayClient(
        trace: connection.trace,
        origin: Uri.parse(connection.session.origin),
        bootstrapPath: connection.session.path,
        allowInsecureLoopback: connection.session.allowInsecureLoopback,
        allowInsecurePrivateNetworkForTesting:
            connection.session.allowInsecurePrivateNetworkForTesting,
      );
  final DshNativeGatewayClient client;
  final MobileDshRecentMessages messages;
  Future<DshNativeNegotiation>? _negotiation;
  Future<DshNativeNegotiation> prepare() =>
      _negotiation ??= _prepare().catchError((Object error) {
        _negotiation = null;
        throw error;
      });
  Future<DshNativeNegotiation> _prepare() async {
    return client.trace.measure('mobile.prewarm', (_) async {
      final hello = await client.initialize();
      if (!hello.contractSupported) {
        throw const DshNativeGatewayException('Unsupported Desktop protocol');
      }
      return client.negotiate(DshNativeCapabilities.standard());
    });
  }

  void close() => client.close();
}
