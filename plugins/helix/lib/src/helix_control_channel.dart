import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

/// Versioned engine event, independent of the terminal's ANSI drawing bytes.
final class HelixResourceEvent {
  const HelixResourceEvent({
    required this.type,
    required this.path,
    required this.revision,
    this.dirty,
  });

  final String type;
  final String path;
  final int revision;
  final bool? dirty;
}

final class HelixCommandResult {
  const HelixCommandResult({
    required this.ok,
    required this.revision,
    required this.dirty,
    this.path,
    this.error,
    this.text,
  });

  final bool ok;
  final String? path;
  final int revision;
  final bool dirty;
  final String? error;
  final String? text;
}

/// One loopback listener per PTY. A random token and the spawned PID authenticate
/// the child; the Host independently authorizes every resource path.
final class HelixControlChannel {
  HelixControlChannel._(this._server, this.token, this.onEvent) {
    _serverSubscription = _server.listen(_accept);
  }

  final ServerSocket _server;
  final String token;
  final void Function(HelixResourceEvent) onEvent;
  late final StreamSubscription<Socket> _serverSubscription;
  Socket? _peer;
  StreamSubscription<List<int>>? _peerSubscription;
  int? expectedPid;
  bool _authenticated = false;
  bool _everAuthenticated = false;
  final List<int> _pending = [];
  final Completer<void> _firstState = Completer<void>();
  final Map<int, Completer<HelixCommandResult>> _pendingCommands = {};
  int _nextRequestId = 1;
  bool _closed = false;

  String get address => '127.0.0.1:${_server.port}';
  Future<void> get firstState => _firstState.future;

  static Future<HelixControlChannel> bind(
    void Function(HelixResourceEvent) onEvent,
  ) async {
    final server = await ServerSocket.bind(InternetAddress.loopbackIPv4, 0);
    final random = Random.secure();
    final token = List<int>.generate(
      32,
      (_) => random.nextInt(256),
    ).map((byte) => byte.toRadixString(16).padLeft(2, '0')).join();
    return HelixControlChannel._(server, token, onEvent);
  }

  void _accept(Socket socket) {
    if (_peer != null || _everAuthenticated) {
      socket.destroy();
      return;
    }
    _peer = socket;
    _peerSubscription = socket.listen(
      _consume,
      onError: (_) => socket.destroy(),
      onDone: () {
        socket.destroy();
        if (identical(_peer, socket)) {
          _peer = null;
          _authenticated = false;
          _pending.clear();
          _failPending(StateError('Helix 控制通道已断开'));
        }
      },
      cancelOnError: true,
    );
  }

  void _consume(List<int> bytes) {
    final socket = _peer;
    if (socket == null) return;
    for (final byte in bytes) {
      if (byte == 10) {
        try {
          _line(utf8.decode(_pending));
        } on FormatException {
          socket.destroy();
        } finally {
          _pending.clear();
        }
      } else {
        _pending.add(byte);
        if (_pending.length > 2 * 1024 * 1024) {
          socket.destroy();
          _pending.clear();
          return;
        }
      }
    }
  }

  void _line(String line) {
    final decoded = jsonDecode(line);
    if (decoded is! Map || decoded['version'] != 1) {
      _peer?.destroy();
      return;
    }
    if (!_authenticated) {
      if (decoded['type'] != 'hello' ||
          decoded['token'] != token ||
          decoded['pid'] != expectedPid) {
        _peer?.destroy();
        return;
      }
      _authenticated = true;
      _everAuthenticated = true;
      return;
    }
    final type = decoded['type'];
    if (type == 'result') {
      final id = decoded['id'];
      final ok = decoded['ok'];
      final revision = decoded['revision'];
      final dirty = decoded['dirty'];
      final path = decoded['path'];
      final error = decoded['error'];
      final text = decoded['text'];
      if (id is! int ||
          ok is! bool ||
          revision is! int ||
          revision < 0 ||
          dirty is! bool ||
          (path != null && (path is! String || !File(path).isAbsolute)) ||
          (error != null && error is! String) ||
          (text != null &&
              (text is! String || utf8.encode(text).length > 1024 * 1024))) {
        return;
      }
      _pendingCommands
          .remove(id)
          ?.complete(
            HelixCommandResult(
              ok: ok,
              path: path as String?,
              revision: revision,
              dirty: dirty,
              error: error as String?,
              text: text as String?,
            ),
          );
      return;
    }
    final path = decoded['path'];
    final revision = decoded['revision'];
    if ((type != 'state' && type != 'saved') ||
        path is! String ||
        path.isEmpty ||
        path.length > 4096 ||
        !File(path).isAbsolute ||
        revision is! int ||
        revision < 0 ||
        (type == 'state' && decoded['dirty'] is! bool)) {
      return;
    }
    onEvent(
      HelixResourceEvent(
        type: type as String,
        path: path,
        revision: revision,
        dirty: decoded['dirty'] as bool?,
      ),
    );
    if (type == 'state' && !_firstState.isCompleted) _firstState.complete();
  }

  Future<HelixCommandResult> command({
    required String name,
    required String path,
    required int revision,
    String? text,
  }) async {
    final peer = _peer;
    if (_closed || !_authenticated || peer == null) {
      throw StateError('Helix 控制通道尚未就绪');
    }
    if (name.length > 32 ||
        !File(path).isAbsolute ||
        revision < 0 ||
        (text != null && utf8.encode(text).length > 1024 * 1024)) {
      throw const FormatException('无效 Helix 语义命令');
    }
    final id = _nextRequestId++;
    final pending = Completer<HelixCommandResult>();
    _pendingCommands[id] = pending;
    try {
      peer.write(
        '${jsonEncode({'version': 1, 'type': 'command', 'id': id, 'command': name, 'path': path, 'revision': revision, if (text != null) 'text': text})}\n',
      );
      await peer.flush();
      return await pending.future.timeout(const Duration(seconds: 30));
    } finally {
      _pendingCommands.remove(id);
    }
  }

  void _failPending(Object error) {
    for (final pending in _pendingCommands.values) {
      if (!pending.isCompleted) pending.completeError(error);
    }
    _pendingCommands.clear();
  }

  Future<void> close() async {
    _closed = true;
    _failPending(StateError('Helix 控制通道已关闭'));
    _peer?.destroy();
    await _peerSubscription?.cancel();
    await _serverSubscription.cancel();
    await _server.close();
  }
}
