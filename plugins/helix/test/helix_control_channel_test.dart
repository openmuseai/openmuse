import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/src/helix_control_channel.dart';

void main() {
  final statePath = File(
    '${Directory.systemTemp.path}${Platform.pathSeparator}definition.rs',
  ).absolute.path;
  final activePath = File(
    '${Directory.systemTemp.path}${Platform.pathSeparator}active.rs',
  ).absolute.path;

  test(
    'authenticated engine events are parsed without ANSI inspection',
    () async {
      final received = <HelixResourceEvent>[];
      final channel = await HelixControlChannel.bind(received.add);
      channel.expectedPid = 42;
      final socket = await Socket.connect(
        '127.0.0.1',
        int.parse(channel.address.split(':').last),
      );
      addTearDown(() async {
        socket.destroy();
        await channel.close();
      });

      void send(Map<String, Object?> message) =>
          socket.write('${jsonEncode(message)}\n');
      send({'version': 1, 'type': 'hello', 'token': channel.token, 'pid': 42});
      send({
        'version': 1,
        'type': 'state',
        'path': statePath,
        'revision': 7,
        'dirty': true,
      });
      send({'version': 1, 'type': 'saved', 'path': statePath, 'revision': 7});
      await socket.flush();
      await channel.firstState.timeout(const Duration(seconds: 2));
      await Future<void>.delayed(const Duration(milliseconds: 25));
      expect(received, hasLength(2));
      expect(received.first.path, statePath);
      expect(received.first.dirty, true);
      expect(received.last.type, 'saved');
    },
  );

  test('rejects missing authentication and relative paths', () async {
    final received = <HelixResourceEvent>[];
    final channel = await HelixControlChannel.bind(received.add);
    channel.expectedPid = 42;
    addTearDown(channel.close);
    final port = int.parse(channel.address.split(':').last);
    final attacker = await Socket.connect('127.0.0.1', port);
    attacker.write(
      '${jsonEncode({'version': 1, 'type': 'hello', 'token': 'wrong', 'pid': 42})}\n',
    );
    await attacker.flush();
    attacker.destroy();
    await Future<void>.delayed(const Duration(milliseconds: 25));

    final engine = await Socket.connect('127.0.0.1', port);
    addTearDown(engine.destroy);
    engine.write(
      '${jsonEncode({'version': 1, 'type': 'hello', 'token': channel.token, 'pid': 42})}\n',
    );
    engine.write(
      '${jsonEncode({'version': 1, 'type': 'state', 'path': 'relative.rs', 'revision': 1, 'dirty': false})}\n',
    );
    await engine.flush();
    await Future<void>.delayed(const Duration(milliseconds: 25));
    expect(received, isEmpty);
  });

  test('semantic command is correlated to a typed result', () async {
    final channel = await HelixControlChannel.bind((_) {});
    channel.expectedPid = 77;
    addTearDown(channel.close);
    final socket = await Socket.connect(
      '127.0.0.1',
      int.parse(channel.address.split(':').last),
    );
    addTearDown(socket.destroy);
    socket.write(
      '${jsonEncode({'version': 1, 'type': 'hello', 'token': channel.token, 'pid': 77})}\n',
    );
    socket.write(
      '${jsonEncode({'version': 1, 'type': 'state', 'path': activePath, 'revision': 3, 'dirty': true})}\n',
    );
    await socket.flush();
    await channel.firstState.timeout(const Duration(seconds: 2));

    final incoming = socket
        .map<List<int>>((bytes) => bytes)
        .transform(utf8.decoder)
        .transform(const LineSplitter())
        .first;
    final future = channel.command(
      name: 'paste',
      path: activePath,
      revision: 3,
      text: '中文 🧪',
    );
    final request = jsonDecode(await incoming) as Map<String, dynamic>;
    expect(request['type'], 'command');
    expect(request['command'], 'paste');
    expect(request['path'], activePath);
    expect(request['revision'], 3);
    expect(request['text'], '中文 🧪');
    socket.write(
      '${jsonEncode({'version': 1, 'type': 'result', 'id': request['id'], 'ok': true, 'path': activePath, 'revision': 4, 'dirty': false, 'text': 'copied'})}\n',
    );
    await socket.flush();
    final result = await future.timeout(const Duration(seconds: 2));
    expect(result.ok, isTrue);
    expect(result.revision, 4);
    expect(result.dirty, isFalse);
    expect(result.text, 'copied');
  });
}
