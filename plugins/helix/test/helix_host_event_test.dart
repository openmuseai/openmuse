import 'dart:async';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test('active Helix resource requests an authorized Host tab', () async {
    final commands = <String, Object?>{};
    final opened = Completer<void>();
    final runtime = HelixRuntimePool(
      executable: Platform.isMacOS
          ? File('assets/engines/helix/hx').absolute.path
          : 'hx',
    );
    final plugin = OpenMuseHelixPlugin(runtime: runtime);
    await plugin.activate(
      OpenMusePluginContext(
        executeHostCommand: (command, arguments) async {
          if (command == 'workspace.openResource') {
            commands[command] = arguments;
            opened.complete();
          }
          return null;
        },
      ),
    );
    addTearDown(plugin.deactivate);

    runtime.onActiveResourceChanged?.call('/workspace/definition.rs');
    await opened.future.timeout(const Duration(seconds: 2));
    expect(commands['workspace.openResource'], {
      'path': '/workspace/definition.rs',
      'editorId': 'helix.editor',
    });
  });
}
