import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:path/path.dart' as p;

typedef ScheduledPromptExecutor =
    Future<void> Function(String workspaceId, String requestId, String message);

/// Desktop-owned durable one-shot scheduled tasks. Mobile is only an editor;
/// the Desktop Host executes while running and resumes overdue work on boot.
final class DesktopScheduledTaskService {
  DesktopScheduledTaskService({
    required this.storeFile,
    required this.executor,
    DateTime Function()? clock,
  }) : _clock = clock ?? DateTime.now;

  final File storeFile;
  final ScheduledPromptExecutor executor;
  final DateTime Function() _clock;
  final Random _random = Random.secure();
  final Map<String, Map<String, Object?>> _tasks = {};
  Timer? _timer;
  bool _running = false;
  Future<void> _persistTail = Future<void>.value();

  Future<void> restore() async {
    _timer?.cancel();
    _tasks.clear();
    if (await storeFile.exists()) {
      final decoded = jsonDecode(await storeFile.readAsString());
      if (decoded is! Map ||
          decoded['version'] != 1 ||
          decoded['tasks'] is! List) {
        throw const FormatException('scheduled task store is invalid');
      }
      for (final raw in decoded['tasks'] as List) {
        if (raw is! Map) continue;
        final task = Map<String, Object?>.from(raw);
        if (task['id'] is! String ||
            task['workspaceId'] is! String ||
            task['message'] is! String ||
            task['dueAt'] is! String ||
            task['state'] is! String) {
          continue;
        }
        if (task['state'] == 'running') {
          // The process may have sent the prompt before it stopped. Retrying
          // automatically could send the same user message twice.
          task['state'] = 'unknown';
          task['error'] = 'Desktop 在执行期间中断，请确认 Workspace 后重新创建任务';
        }
        _tasks[task['id'] as String] = task;
      }
    }
    _arm();
  }

  List<Map<String, Object?>> list() {
    final items = _tasks.values
        .map((task) => Map<String, Object?>.from(task))
        .toList(growable: false);
    items.sort(
      (a, b) => (a['dueAt'] as String).compareTo(b['dueAt'] as String),
    );
    return items;
  }

  Future<Map<String, Object?>> create({
    required String workspaceId,
    required String message,
    required DateTime dueAt,
  }) async {
    final clean = message.trim();
    final utc = dueAt.toUtc();
    if (workspaceId.isEmpty ||
        workspaceId.length > 128 ||
        clean.isEmpty ||
        clean.length > 4000 ||
        !utc.isAfter(_clock().toUtc().add(const Duration(seconds: 10))) ||
        utc.isAfter(_clock().toUtc().add(const Duration(days: 365))) ||
        _tasks.length >= 128) {
      throw const FormatException('scheduled task request is invalid');
    }
    final id = List<int>.generate(
      16,
      (_) => _random.nextInt(256),
    ).map((byte) => byte.toRadixString(16).padLeft(2, '0')).join();
    final task = <String, Object?>{
      'id': id,
      'workspaceId': workspaceId,
      'message': clean,
      'dueAt': utc.toIso8601String(),
      'state': 'pending',
      'attempts': 0,
    };
    _tasks[id] = task;
    await _save();
    _arm();
    return Map<String, Object?>.from(task);
  }

  Future<void> cancel(String id) async {
    final task = _tasks[id];
    if (task == null || task['state'] != 'pending') {
      throw const FormatException('scheduled task is not pending');
    }
    task['state'] = 'cancelled';
    await _save();
    _arm();
  }

  Future<void> runDue() async {
    if (_running) return;
    _running = true;
    try {
      final due = _tasks.values
          .where(
            (task) =>
                task['state'] == 'pending' &&
                !DateTime.parse(
                  task['dueAt'] as String,
                ).isAfter(_clock().toUtc()),
          )
          .toList();
      due.sort(
        (a, b) => (a['dueAt'] as String).compareTo(b['dueAt'] as String),
      );
      for (final task in due) {
        task['state'] = 'running';
        task['attempts'] = (task['attempts'] as int? ?? 0) + 1;
        await _save();
        try {
          await executor(
            task['workspaceId'] as String,
            task['id'] as String,
            task['message'] as String,
          );
          task['state'] = 'completed';
          task['completedAt'] = _clock().toUtc().toIso8601String();
          task.remove('error');
        } on Object catch (error) {
          task['state'] = 'failed';
          task['error'] = '$error'.substring(0, min(200, '$error'.length));
        }
        await _save();
      }
    } finally {
      _running = false;
      _arm();
    }
  }

  void _arm() {
    _timer?.cancel();
    final pending = _tasks.values
        .where((task) => task['state'] == 'pending')
        .toList();
    if (pending.isEmpty) return;
    pending.sort(
      (a, b) => (a['dueAt'] as String).compareTo(b['dueAt'] as String),
    );
    final due = DateTime.parse(pending.first['dueAt'] as String);
    final delay = due.difference(_clock().toUtc());
    _timer = Timer(
      delay.isNegative ? Duration.zero : delay,
      () => unawaited(runDue()),
    );
  }

  Future<void> _save() async {
    final snapshot = jsonEncode({
      'version': 1,
      'tasks': _tasks.values.toList(),
    });
    final write = _persistTail.then((_) async {
      await storeFile.parent.create(recursive: true);
      final temporary = File(
        p.join(storeFile.parent.path, '${p.basename(storeFile.path)}.tmp'),
      );
      await temporary.writeAsString(snapshot, flush: true);
      await temporary.rename(storeFile.path);
    });
    _persistTail = write.catchError((Object _) {});
    await write;
  }

  void dispose() => _timer?.cancel();
}

/// Invokes the already-running local DSH bridge. This token never leaves the
/// Desktop process or enters the paired Mobile response.
final class DshScheduledPromptExecutor {
  const DshScheduledPromptExecutor({
    required this.endpoint,
    required this.bridgeToken,
  });

  final Future<Uri> Function() endpoint;
  final String bridgeToken;

  Future<void> call(
    String workspaceId,
    String requestId,
    String message,
  ) async {
    final origin = await endpoint();
    if (!const {'127.0.0.1', 'localhost', '::1'}.contains(origin.host)) {
      throw StateError('DSH scheduled endpoint must be loopback');
    }
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 12);
    try {
      Future<Map<String, Object?>> post(
        String path,
        Map<String, Object?> body,
      ) async {
        final request = await client.postUrl(origin.resolve(path));
        request.headers
          ..contentType = ContentType.json
          ..set('x-openmuse-bridge-token', bridgeToken);
        request.add(utf8.encode(jsonEncode(body)));
        final response = await request.close().timeout(
          const Duration(seconds: 30),
        );
        final bytes = await response.fold<List<int>>(
          <int>[],
          (out, chunk) => out..addAll(chunk),
        );
        if (response.statusCode != HttpStatus.ok) {
          throw StateError(
            'DSH scheduled request failed: HTTP ${response.statusCode}',
          );
        }
        final value = jsonDecode(utf8.decode(bytes));
        if (value is! Map) throw const FormatException('DSH response invalid');
        return Map<String, Object?>.from(value);
      }

      final session = await post('/openmuse-native/v1/session/create', {
        'workspaceId': workspaceId,
        'agentPreset': 'standard',
      });
      final sessionId = session['sessionId'];
      if (sessionId is! String || sessionId.isEmpty) {
        throw const FormatException('DSH scheduled session invalid');
      }
      await post('/openmuse-native/v1/session/prompt', {
        'sessionId': sessionId,
        'requestId': requestId,
        'mode': 'queue',
        'content': [
          {'type': 'text', 'text': message},
        ],
      });
    } finally {
      client.close(force: true);
    }
  }
}
