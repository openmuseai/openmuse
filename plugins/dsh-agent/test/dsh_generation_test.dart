import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/dsh_generation.dart';

void main() {
  test(
    'real HTTP task returns only after a completed visible conversation turn; ownership and cancel are enforced',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      HttpResponse? stream;
      int created = 0, prompts = 0, cancels = 0;
      String? opened;
      final requests = server.listen((r) async {
        expect(r.headers.value('x-openmuse-bridge-token'), 'test-token');
        final path = r.uri.path;
        if (path.endsWith('/follow')) {
          stream = r.response;
          stream!.bufferOutput = false;
          stream!.headers.contentType = ContentType('text', 'event-stream');
          stream!.write(
            'event: frame\ndata: ${jsonEncode({'type': 'snapshot', 'records': []})}\n\n',
          );
          await stream!.flush();
          return;
        }
        final body = jsonDecode(await utf8.decoder.bind(r).join()) as Map;
        Object result = {};
        if (path == '/openmuse-bridge/workspaces')
          result = {
            'items': [
              {'path': '/workspace', 'workspaceId': 'w'},
            ],
          };
        if (path.endsWith('/create')) {
          created++;
          result = {'sessionId': 's'};
        }
        if (path.endsWith('/prompt')) {
          prompts++;
          expect(body['sessionId'], 's');
          result = {'accepted': true};
        }
        if (path.endsWith('/cancel')) {
          cancels++;
          result = {'cancelled': true};
        }
        r.response.headers.contentType = ContentType.json;
        r.response.write(jsonEncode(result));
        await r.response.close();
      });
      final service = DshGeneration(
        endpoint: () => Uri.parse('http://127.0.0.1:${server.port}'),
        token: 'test-token',
        mount: () => '/workspace',
        ready: () async {},
        openSession: (id) => opened = id,
      );
      Future<Map> call(
        String tool,
        Map<String, Object?> args, [
        String owner = 'one',
      ]) async =>
          (await service.call(owner, tool, args))['structuredContent'] as Map;
      Future<void> until(bool Function() condition) async {
        for (int i = 0; i < 200 && !condition(); i++) {
          await Future<void>.delayed(const Duration(milliseconds: 10));
        }
        expect(condition(), true);
      }

      final first = await call('dsh_generate', {'text': 'Explain page 1'});
      try {
        await until(() => prompts == 1);
      } catch (_) {
        fail(
          'Task: ${await call("dsh_generation_status", {"taskRef": first["taskRef"]})}; created=$created, stream=${stream != null}',
        );
      }
      expect(opened, 's');
      expect(created, 1);
      await expectLater(
        service.call('two', 'dsh_generation_status', {
          'taskRef': first['taskRef'],
        }),
        throwsStateError,
      );
      await expectLater(
        service.call('one', 'dsh_generate', {'text': 'Duplicate'}),
        throwsStateError,
      );
      expect(
        (await call('dsh_generation_status', {
          'taskRef': first['taskRef'],
        }))['status'],
        'running',
      );
      void event(String type, Map data) {
        stream!.write(
          'event: frame\ndata: ${jsonEncode({
            'type': 'event',
            'event': {'type': type, 'data': data},
          })}\n\n',
        );
      }

      event('turn/start', {});
      event('assistant/message', {
        'message': {
          'content': [
            {'type': 'text', 'text': 'Actual model output'},
          ],
        },
      });
      event('turn/end', {
        'reason': {'kind': 'completed'},
      });
      await stream!.flush();
      await until(() => stream != null);
      Map status = {};
      for (int i = 0; i < 100; i++) {
        status = await call('dsh_generation_status', {
          'taskRef': first['taskRef'],
        });
        if (status['status'] == 'completed') break;
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }
      expect(status['status'], 'completed');
      expect(status['text'], 'Actual model output');
      final second = await call('dsh_generate', {
        'text': 'Explain page 2',
        'conversationRef': first['conversationRef'],
      });
      await until(() => prompts == 2);
      expect(created, 1);
      await call('dsh_generation_cancel', {'taskRef': second['taskRef']});
      expect(cancels, 1);
      expect(
        (await call('dsh_generation_status', {
          'taskRef': second['taskRef'],
        }))['status'],
        'cancelled',
      );
      await service.disposeCaller('one');
      await expectLater(
        service.call('one', 'dsh_generation_status', {
          'taskRef': second['taskRef'],
        }),
        throwsStateError,
      );
      await requests.cancel();
      await server.close(force: true);
    },
  );
}
