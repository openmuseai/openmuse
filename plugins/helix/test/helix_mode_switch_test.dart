import 'dart:async';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  test(
    'live mode switch saves the buffer and restores the active file',
    () async {
      if (!Platform.isMacOS && !Platform.isWindows) return;
      final bundled = Platform.environment['OPENMUSE_HELIX_BIN'];
      final executable = Platform.isWindows
          ? bundled ?? ''
          : File('assets/engines/helix/hx').absolute.path;
      final libraryDirectory =
          Platform.environment['OPENMUSE_PTY_LIBRARY_DIR'] ??
          '../../app/openmuse_host/build/macos/Build/Products/Release/'
              'OpenMuse.app/Contents/Frameworks';
      final framework = Directory(
        '$libraryDirectory/flutter_pty.framework',
      ).absolute;
      final windowsLibrary = File('$libraryDirectory/flutter_pty.dll').absolute;
      final available =
          File(executable).existsSync() &&
          (Platform.isMacOS
              ? framework.existsSync()
              : windowsLibrary.existsSync());
      if (!available) {
        if (Platform.environment['OPENMUSE_REQUIRE_PTY_TEST'] == '1') {
          fail('发行门禁缺少 Helix 或 flutter_pty 原生运行库');
        }
        markTestSkipped('先构建桌面 App，才能执行原生 PTY 切换门禁');
        return;
      }
      final directory = await Directory.systemTemp.createTemp('openmuse-mode-');
      addTearDown(() async {
        // ConPTY releases each session's working directory after its exit
        // event, and a mode switch leaves several sessions behind, so give
        // them time to go before cleaning up.
        for (var attempt = 0; attempt < 40; attempt++) {
          try {
            await directory.delete(recursive: true);
            return;
          } on PathAccessException {
            await Future<void>.delayed(const Duration(milliseconds: 250));
          }
        }
      });
      final originalDirectory = Directory.current;
      if (Platform.isMacOS) {
        await Link(
          '${directory.path}/flutter_pty.framework',
        ).create(framework.path);
      } else {
        await windowsLibrary.copy('${directory.path}/flutter_pty.dll');
      }
      Directory.current = directory;
      addTearDown(() => Directory.current = originalDirectory);
      // Helix reports the buffer path it was handed, so these must use the
      // native separator to match the engine's state events.
      final source = File(
        '${directory.path}${Platform.pathSeparator}example.txt',
      );
      await source.writeAsString('hello\n');
      final secondSource = File(
        '${directory.path}${Platform.pathSeparator}second.txt',
      );
      await secondSource.writeAsString('world\n');
      final runtime = HelixRuntimePool(executable: executable);
      addTearDown(() async {
        await runtime.stop();
        runtime.dispose();
      });
      expect(await runtime.probeCapabilities(), isTrue);
      await runtime.configure(
        const HelixPreferences(
          inputProfile: HelixInputProfile.standardNonmodal,
        ),
      );
      await runtime.openDocument(source.path);
      final dirty = Completer<void>();
      runtime.onResourceEvent = (event) {
        if (event.path == source.path &&
            event.dirty == true &&
            !dirty.isCompleted) {
          dirty.complete();
        }
      };
      runtime.terminal.textInput('x');
      await dirty.future.timeout(const Duration(seconds: 10));
      await runtime.openDocument(secondSource.path);
      final secondDirty = Completer<void>();
      runtime.onResourceEvent = (event) {
        if (event.path == secondSource.path &&
            event.dirty == true &&
            !secondDirty.isCompleted) {
          secondDirty.complete();
        }
      };
      runtime.terminal.textInput('y');
      await secondDirty.future.timeout(const Duration(seconds: 10));

      await runtime.configure(
        const HelixPreferences(inputProfile: HelixInputProfile.helixModal),
      );
      expect(runtime.preferences.inputProfile, HelixInputProfile.helixModal);
      expect(runtime.activePath, secondSource.path);
      // The pool reuses a single hx session for every open document (see the
      // reuse gate in helix_windows_environment_test.dart), so a profile switch
      // stops and relaunches that one session.
      expect(runtime.launchCount, 2);
      expect(await source.readAsString(), startsWith('x'));
      expect(await secondSource.readAsString(), startsWith('y'));

      await runtime.configure(
        const HelixPreferences(
          inputProfile: HelixInputProfile.standardNonmodal,
        ),
      );
      expect(
        runtime.preferences.inputProfile,
        HelixInputProfile.standardNonmodal,
      );
      expect(runtime.activePath, secondSource.path);
      expect(runtime.launchCount, 3);
    },
  );
}
