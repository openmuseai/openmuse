import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:flutter_test/flutter_test.dart';

/// Prints the cost of every step HelixResourceSurface._boot runs before it can
/// spawn Helix, so the per-open latency can be attributed instead of guessed.
void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  test('helix open-time boot cost', () async {
    final root = Directory.systemTemp.createTempSync('helix-cost');
    final installer = HelixLanguageServerInstaller(root: root);

    final refresh = Stopwatch()..start();
    await installer.refresh();
    refresh.stop();
    // ignore: avoid_print
    print('BOOTCOST refresh                          : ${refresh.elapsedMilliseconds} ms');

    final write1 = Stopwatch()..start();
    await installer.writeLanguagesToml();
    write1.stop();
    // ignore: avoid_print
    print('BOOTCOST writeLanguagesToml (cold)        : ${write1.elapsedMilliseconds} ms');

    final write2 = Stopwatch()..start();
    await installer.writeLanguagesToml();
    write2.stop();
    // ignore: avoid_print
    print('BOOTCOST writeLanguagesToml (again)       : ${write2.elapsedMilliseconds} ms');

    final resolve1 = Stopwatch()..start();
    HelixInstall? install;
    Object? resolveError;
    try {
      install = await HelixInstall.resolve();
    } on Object catch (error) {
      resolveError = error;
    }
    resolve1.stop();
    // ignore: avoid_print
    print('BOOTCOST HelixInstall.resolve             : ${resolve1.elapsedMilliseconds} ms '
        '(${install?.binary ?? resolveError})');

    if (install != null) {
      final grammars = Stopwatch()..start();
      final count = await copyHelixGrammars(
        destRuntime: installer.grammarRuntime,
        extraSearchRuntimes: [install.runtime],
      );
      grammars.stop();
      // ignore: avoid_print
      print('BOOTCOST copyHelixGrammars                : ${grammars.elapsedMilliseconds} ms '
          '(count=$count)');
    }

    final defaults = Stopwatch()..start();
    final settings = installer.statuses.length;
    defaults.stop();
    // ignore: avoid_print
    print('BOOTCOST statuses=$settings, temp dir=${root.path}');
  }, timeout: const Timeout(Duration(minutes: 5)));
}
