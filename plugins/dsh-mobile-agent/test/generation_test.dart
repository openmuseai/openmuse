import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_mobile_agent/openmuse_dsh_mobile_agent.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  test(
    'mobile DSH owns tasks, ignores snapshot history and reuses an isolated real session',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      HttpResponse? stream;
      var created = 0, prompted = 0, seq = 0;
      void emit(Map<String, Object?> value) {
        stream!.write('data: ${jsonEncode(value)}\n\n');
      }

      void event(String type, Map<String, Object?> data) => emit({
        'type': 'event',
        'event': {
          'type': type,
          'data': data,
          'seq': ++seq,
          'time': DateTime.now().millisecondsSinceEpoch,
        },
      });
      server.listen((request) async {
        final path = request.uri.path;
        if (path.endsWith('/follow')) {
          request.response.headers.contentType = ContentType(
            'text',
            'event-stream',
          );
          request.response.bufferOutput = false;
          stream = request.response;
          emit({
            'type': 'snapshot',
            'header': {},
            'cursor': 0,
            'records': [],
            'hasMore': false,
            'projections': {},
          });
          return;
        }
        Map<String, Object?> value = {};
        if (path.endsWith('/hello'))
          value = {
            'protocolVersion': 1,
            'dshVersion': '0.2.0-rc.2',
            'capabilities': [],
            'compatibility': {
              'nativeConversationCompatible': true,
              'fallbackRequired': false,
              'unsupportedPlugins': [],
            },
          };
        if (path.endsWith('/workspaces'))
          value = {
            'items': [
              {
                'workspaceId': 'workspace',
                'title': 'Workspace',
                'sessionIds': [],
                'createdAt': '2026-10-21T00:00:00Z',
                'updatedAt': '2026-10-21T00:00:00Z',
              },
            ],
          };
        if (path.endsWith('/create')) {
          created++;
          value = {'sessionId': 'owned-session'};
        }
        if (path.endsWith('/prompt')) {
          final body = jsonDecode(await utf8.decodeStream(request));
          expect(body['sessionId'], 'owned-session');
          prompted++;
          value = {'accepted': true};
          event('turn/start', {});
          event('assistant/message', {
            'message': {
              'content': [
                {'type': 'text', 'text': 'Generated page $prompted'},
              ],
            },
          });
          event('turn/end', {
            'reason': {'kind': 'completed'},
          });
        }
        if (path.endsWith('/cancel')) value = {'accepted': true};
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode(value));
        await request.response.close();
      });
      final connection = PairedDesktopConnection(
        accountRef: 'account',
        deviceRef: 'device',
        deviceName: 'Desktop',
        workspaceRef: 'workspace',
        workspaceTitle: 'Workspace',
        grantRef: 'grant',
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 600000,
        session: DshSessionDescriptor(
          sessionRef: 'session',
          origin: 'http://127.0.0.1:${server.port}',
          path: '/u/grant',
          generation: 1,
          allowInsecureLoopback: true,
        ),
      );
      final plugin = OpenMuseDshMobileAgent(
        connection: () => connection,
        context: () => null,
      );
      addTearDown(() async {
        await plugin.deactivate();
        await server.close(force: true);
      });
      Future<Map> call(
        String caller,
        String name,
        Map<String, Object?> args,
      ) async =>
          (await plugin.callContributedTool(
                caller,
                name,
                args,
              ))['structuredContent']
              as Map;
      final first = await call('consumer', 'dsh_generate', {
        'text': 'Page one',
      });
      await expectLater(
        call('other', 'dsh_generation_status', {'taskRef': first['taskRef']}),
        throwsStateError,
      );
      Future<Map> done(Map task) async {
        for (var i = 0; i < 100; i++) {
          final status = await call('consumer', 'dsh_generation_status', {
            'taskRef': task['taskRef'],
          });
          if (status['status'] != 'running') return status;
          await Future<void>.delayed(const Duration(milliseconds: 10));
        }
        throw StateError('generation timed out');
      }

      expect((await done(first))['text'], 'Generated page 1');
      final second = await call('consumer', 'dsh_generate', {
        'text': 'Page two',
        'conversationRef': first['conversationRef'],
      });
      expect((await done(second))['text'], 'Generated page 2');
      expect(created, 1);
      expect(prompted, 2);
      await plugin.disposeToolCaller('consumer');
      await expectLater(
        call('consumer', 'dsh_generation_status', {
          'taskRef': second['taskRef'],
        }),
        throwsStateError,
      );
    },
  );
}
