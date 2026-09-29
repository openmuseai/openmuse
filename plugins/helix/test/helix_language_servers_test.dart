import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  test('common language server catalog includes additional file types', () {
    final commands = helixLanguageServers.map((item) => item.command).toSet();
    expect(
      commands,
      containsAll([
        'rust-analyzer',
        'bash-language-server',
        'yaml-language-server',
        'vscode-html-language-server',
        'vscode-css-language-server',
        'jdtls',
        'lua-language-server',
        'svelteserver',
        'vuels',
      ]),
    );
    expect(
      helixLanguageServers
          .firstWhere((item) => item.command == 'rust-analyzer')
          .oneClickInstallable,
      isTrue,
    );
    expect(
      helixLanguageServers
          .firstWhere((item) => item.command == 'jdtls')
          .oneClickInstallable,
      isFalse,
    );
  });

  test(
    'LS executable and config paths persist and produce Helix TOML',
    () async {
      final root = await Directory.systemTemp.createTemp('openmuse-helix-ls-');
      addTearDown(() => root.delete(recursive: true));
      final binary = File('${root.path}/rust-analyzer')
        ..writeAsStringSync('binary');
      final config = File('${root.path}/rust-analyzer.json')
        ..writeAsStringSync(
          jsonEncode({
            'cargo': {'allFeatures': true},
          }),
        );
      final value = HelixPreferences(
        languageServerPaths: {'rust-analyzer': binary.path},
        languageServerConfigPaths: {'rust-analyzer': config.path},
      );
      final restored = HelixPreferences.fromJson(value.toJson());
      expect(restored.languageServerConfigPaths['rust-analyzer'], config.path);
      expect(
        restored.languagesToml,
        contains('command = ${jsonEncode(binary.path)}'),
      );
      expect(
        restored.languagesToml,
        contains('[language-server.rust-analyzer.config]'),
      );
      expect(restored.languagesToml, contains('"cargo"."allFeatures" = true'));
    },
  );

  test('Rust analyzer resolves to a runnable toolchain binary', () async {
    final path = await resolveRustAnalyzerExecutable();
    if (path == null) return; // Rust is optional on developer machines.
    expect(File(path).existsSync(), isTrue);
    expect(path.toLowerCase(), isNot(contains('.cargo\\bin\\rust-analyzer')));
    final version = await Process.run(path, ['--version']);
    expect(version.exitCode, 0);
    expect(version.stdout.toString(), contains('rust-analyzer'));
  });

  test('configured rustup proxy is recognized before writing Helix config', () {
    if (!Platform.isWindows) return;
    expect(
      isRustupProxyAnalyzer(r'D:\Rust\.cargo\bin\rust-analyzer.exe'),
      isTrue,
    );
    expect(
      isRustupProxyAnalyzer(
        r'D:\Rust\.rustup\toolchains\nightly\bin\rust-analyzer.exe',
      ),
      isFalse,
    );
  });

  test('Helix replaces configured proxy with a working Rust binary', () async {
    if (!Platform.isWindows) return;
    final actual = await resolveRustAnalyzerExecutable();
    if (actual == null) return;
    final runtime = HelixRuntimePool(
      executable: 'missing-hx-for-config-test.exe',
    );
    addTearDown(() async {
      final directory = runtime.generatedLanguagesFile.parent.parent;
      if (await directory.exists()) await directory.delete(recursive: true);
      runtime.dispose();
    });
    await runtime.configure(
      const HelixPreferences(
        languageServerPaths: {
          'rust-analyzer': r'D:\Rust\.cargo\bin\rust-analyzer.exe',
        },
      ),
    );
    final languages = await runtime.generatedLanguagesFile.readAsString();
    expect(languages, contains(jsonEncode(actual)));
    expect(languages, isNot(contains(r'.cargo\\bin\\rust-analyzer.exe')));
  });
}
