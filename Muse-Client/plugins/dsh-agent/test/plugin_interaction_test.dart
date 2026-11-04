import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/plugin_interaction.dart';
import 'package:path/path.dart' as p;

void main() {
  test('installed plugin challenge resolves within its workspace', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-interaction-');
    addTearDown(() => root.delete(recursive: true));
    final inbox = Directory(p.join(root.path, 'plugin-interactions'))
      ..createSync();
    final workspace = Directory(p.join(root.path, 'workspace'))..createSync();
    final receipt = File(
      p.join(root.path, 'plugins', 'com.openmuse.easel', 'receipt.json'),
    );
    receipt.parent.createSync(recursive: true);
    receipt.writeAsStringSync(
      jsonEncode({
        'pluginId': 'com.openmuse.easel',
        'workspacePath': workspace.path,
      }),
    );
    final image = File(p.join(workspace.path, 'qr.png'))
      ..writeAsBytesSync([1, 2, 3]);
    final status = File(p.join(workspace.path, 'login.json'))
      ..writeAsStringSync('{"state":"qr_ready"}');
    final event = File(p.join(inbox.path, 'event.json'));
    Map<String, Object> envelope(String imagePath) => {
      'protocol': 'openmuse.plugin-interaction/v1',
      'type': 'image.challenge',
      'pluginId': 'com.openmuse.easel',
      'title': '扫码登录',
      'imagePath': imagePath,
      'statusPath': status.path,
      'issuedAt': DateTime.now().millisecondsSinceEpoch ~/ 1000,
    };
    event.writeAsStringSync(jsonEncode(envelope(image.path)));
    final parsed = await PluginInteraction.read(event, inbox.path);
    expect(parsed.pluginId, 'com.openmuse.easel');
    expect(parsed.imagePath, await image.resolveSymbolicLinks());
    event.writeAsStringSync(
      jsonEncode(envelope(p.join(root.path, 'outside.png'))),
    );
    File(p.join(root.path, 'outside.png')).writeAsBytesSync([1]);
    expect(PluginInteraction.read(event, inbox.path), throwsFormatException);

    final workspaceInbox = Directory(
      p.join(workspace.path, 'state', 'interactions'),
    )..createSync(recursive: true);
    final fromWorkspace = File(p.join(workspaceInbox.path, 'event.json'))
      ..writeAsStringSync(jsonEncode(envelope(image.path)));
    PluginInteraction? received;
    final listener = PluginInteractionListener(
      inbox.path,
      (value) => received = value,
    );
    await listener.scan();
    expect(received?.imagePath, await image.resolveSymbolicLinks());
    expect(await File('${fromWorkspace.path}.seen').exists(), isTrue);
  });
}
