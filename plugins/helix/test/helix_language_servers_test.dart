import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  test('common language server catalog includes additional file types', () {
    final commands = helixLanguageServers.map((item) => item.command).toSet();
    expect(commands, containsAll([
      'rust-analyzer',
      'bash-language-server',
      'yaml-language-server',
      'vscode-html-language-server',
      'vscode-css-language-server',
      'jdtls',
      'lua-language-server',
      'svelteserver',
      'vuels',
    ]));
    expect(helixLanguageServers.firstWhere((item) => item.command == 'rust-analyzer').oneClickInstallable, isTrue);
    expect(helixLanguageServers.firstWhere((item) => item.command == 'jdtls').oneClickInstallable, isFalse);
  });

  test('LS executable and config paths persist and produce Helix TOML', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-helix-ls-');
    addTearDown(() => root.delete(recursive: true));
    final binary = File('${root.path}/rust-analyzer')..writeAsStringSync('binary');
    final config = File('${root.path}/rust-analyzer.json')
      ..writeAsStringSync(jsonEncode({'cargo': {'allFeatures': true}}));
    final value = HelixPreferences(
      languageServerPaths: {'rust-analyzer': binary.path},
      languageServerConfigPaths: {'rust-analyzer': config.path},
    );
    final restored = HelixPreferences.fromJson(value.toJson());
    expect(restored.languageServerConfigPaths['rust-analyzer'], config.path);
    expect(restored.languagesToml, contains('command = ${jsonEncode(binary.path)}'));
    expect(restored.languagesToml, contains('[language-server.rust-analyzer.config]'));
    expect(restored.languagesToml, contains('"cargo"."allFeatures" = true'));
  });
}
