import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  test('parses session/open payload including waiting room', () {
    final ready = DshSessionOpen.fromJson({
      'sessionRef': 'session.1',
      'instanceRef': 'inst.a',
      'webUrl': 'https://app.example.com/u/ab/?token=x',
      'nodeId': 'local',
    });
    expect(ready.isQueued, isFalse);
    expect(ready.webUrl, contains('/u/'));
    final queued = DshSessionOpen.fromJson({
      'sessionRef': 'session.2',
      'queuePosition': 3,
      'retryAfterMs': 10000,
      'nodeId': 'local',
    });
    expect(queued.isQueued, isTrue);
  });

  test('posts open/close/heartbeat to Cloud BFF paths with the account bearer', () async {
    final seen = <String>[];
    final api = DshSessionApi(
      cloudOrigin: Uri.parse('https://app.example.com'),
      accessToken: 'account.jwt',
      post: (uri, headers, body) async {
        seen.add('${uri.path} $body');
        expect(headers['Authorization'], 'Bearer account.jwt');
        return {
          'data': {
            'sessionRef': 'session.1',
            'webUrl': 'https://app.example.com/u/aa/?token=t',
            'nodeId': 'local',
          },
        };
      },
    );
    final opened = await api.open(workspaceRef: 'ws-1', deviceId: 'mobile.1');
    expect(opened.webUrl, contains('token=t'));
    await api.close(sessionRef: opened.sessionRef, deviceId: 'mobile.1');
    await api.heartbeat(sessionRef: opened.sessionRef);
    expect(seen[0], contains('/api/muse/dsh/session/open'));
    expect(seen[1], contains('/api/muse/dsh/session/close'));
    expect(seen[2], contains('/api/muse/dsh/session/heartbeat'));
  });
}
