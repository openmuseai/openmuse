import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_linux/src/host/scheduled_task_service.dart';

void main() {
  test('Desktop restores and executes one-shot tasks exactly once', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-schedule-');
    addTearDown(() => root.delete(recursive: true));
    var now = DateTime.utc(2026, 10, 14, 9);
    final sent = <(String, String, String)>[];
    DesktopScheduledTaskService service() => DesktopScheduledTaskService(
      storeFile: File('${root.path}/tasks.json'),
      clock: () => now,
      executor: (workspace, requestId, message) async {
        sent.add((workspace, requestId, message));
      },
    );
    final first = service();
    await first.restore();
    final task = await first.create(
      workspaceId: 'workspace-1',
      message: '  请总结今天的进度  ',
      dueAt: now.add(const Duration(minutes: 5)),
    );
    expect(first.list().single['state'], 'pending');
    first.dispose();

    final restored = service();
    await restored.restore();
    expect(restored.list().single['id'], task['id']);
    now = now.add(const Duration(minutes: 6));
    await restored.runDue();
    await restored.runDue();
    expect(sent, [('workspace-1', task['id'] as String, '请总结今天的进度')]);
    expect(restored.list().single['state'], 'completed');
    restored.dispose();

    final again = service();
    await again.restore();
    expect(again.list().single['state'], 'completed');
    again.dispose();
  });

  test('cancelled task never executes', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-schedule-');
    addTearDown(() => root.delete(recursive: true));
    var now = DateTime.utc(2026, 10, 14, 9);
    var executions = 0;
    final service = DesktopScheduledTaskService(
      storeFile: File('${root.path}/tasks.json'),
      clock: () => now,
      executor: (_, _, _) async {
        executions++;
      },
    );
    await service.restore();
    final task = await service.create(
      workspaceId: 'workspace-1',
      message: 'hello',
      dueAt: now.add(const Duration(minutes: 2)),
    );
    await service.cancel(task['id'] as String);
    now = now.add(const Duration(minutes: 3));
    await service.runDue();
    expect(executions, 0);
    expect(service.list().single['state'], 'cancelled');
    service.dispose();
  });

  test(
    'an interrupted send is marked unknown and never resent automatically',
    () async {
      final root = await Directory.systemTemp.createTemp('openmuse-schedule-');
      addTearDown(() => root.delete(recursive: true));
      final file = File('${root.path}/tasks.json');
      await file.writeAsString(
        jsonEncode({
          'version': 1,
          'tasks': [
            {
              'id': 'request-1',
              'workspaceId': 'workspace-1',
              'message': 'hello',
              'dueAt': DateTime.utc(2026, 10, 13).toIso8601String(),
              'state': 'running',
              'attempts': 1,
            },
          ],
        }),
      );
      var executions = 0;
      final service = DesktopScheduledTaskService(
        storeFile: file,
        clock: () => DateTime.utc(2026, 10, 14),
        executor: (_, _, _) async => executions++,
      );
      await service.restore();
      await service.runDue();
      expect(executions, 0);
      expect(service.list().single['state'], 'unknown');
      service.dispose();
    },
  );

  test(
    'DSH executor creates a workspace session then queues the prompt',
    () async {
      final requests = <Map<String, Object?>>[];
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() => server.close(force: true));
      server.listen((request) async {
        final raw = await utf8.decoder.bind(request).join();
        requests.add({
          'path': request.uri.path,
          'token': request.headers.value('x-openmuse-bridge-token'),
          'body': jsonDecode(raw),
        });
        request.response.headers.contentType = ContentType.json;
        request.response.write(
          jsonEncode(
            request.uri.path.endsWith('/create')
                ? {'sessionId': 'session-1'}
                : {'accepted': true},
          ),
        );
        await request.response.close();
      });
      await DshScheduledPromptExecutor(
        endpoint: () async => Uri.parse('http://127.0.0.1:${server.port}/'),
        bridgeToken: 'test-token',
      )('workspace-1', 'request-1', 'hello');
      expect(requests, hasLength(2));
      expect(requests.first['path'], '/openmuse-native/v1/session/create');
      expect(requests.first['token'], 'test-token');
      expect((requests.first['body'] as Map)['workspaceId'], 'workspace-1');
      expect(requests.last['path'], '/openmuse-native/v1/session/prompt');
      expect((requests.last['body'] as Map)['requestId'], 'request-1');
      expect((requests.last['body'] as Map)['mode'], 'queue');
    },
  );
}
