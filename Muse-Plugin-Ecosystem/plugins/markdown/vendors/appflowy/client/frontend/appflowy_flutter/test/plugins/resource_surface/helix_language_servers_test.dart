import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('catalog ids are unique and common set includes dart and rust-analyzer', () {
    final ids = helixLspCatalog.map((item) => item.id).toList();
    expect(ids.toSet().length, ids.length);
    expect(ids, containsAll(['rust-analyzer', 'dart', 'typescript', 'ruff']));
    expect(
      helixLspCatalog.where((item) => item.common).map((item) => item.id),
      containsAll(['rust-analyzer', 'dart']),
    );
  });

  test('writes absolute rust-analyzer command into Host languages.toml', () async {
    final parent = Directory.systemTemp.createTempSync('muse-lsp-');
    addTearDown(() => parent.deleteSync(recursive: true));
    final root = Directory('${parent.path}/language-servers')
      ..createSync(recursive: true);
    File('${root.path}/bin/rust-analyzer').createSync(recursive: true);

    final installer = HelixLanguageServerInstaller(root: root);
    await installer.writeLanguagesToml();

    final toml = File('${parent.path}/helix-xdg/helix/languages.toml')
        .readAsStringSync();
    expect(toml, contains('[language-server.rust-analyzer]'));
    expect(toml, contains(root.path));
    expect(toml, contains('use-grammars = { only ='));
    expect(toml, contains('"dart"'));
    expect(toml.indexOf('use-grammars'), lessThan(toml.indexOf('[language-server')));
    expect(installer.pathPrefix('/usr/bin'), startsWith('${root.path}/bin:'));
  });

  test('grammar env prefers XDG runtime and strips cargo manifest', () {
    final env = helixGrammarProcessEnvironment(
      helixRuntime: '/tmp/helix-runtime',
      xdgConfigHome: '/tmp/helix-xdg',
      base: {
        'PATH': '/usr/bin',
        'CARGO_MANIFEST_DIR': '/tmp/crate',
      },
    );
    expect(env['HELIX_RUNTIME'], '/tmp/helix-runtime');
    expect(env['XDG_CONFIG_HOME'], '/tmp/helix-xdg');
    expect(env['COLORTERM'], 'truecolor');
    expect(env.containsKey('CARGO_MANIFEST_DIR'), isFalse);
  });

  test('host triple is a rustc-style target', () {
    expect(
      helixLspHostTriple(),
      anyOf(
        'aarch64-apple-darwin',
        'x86_64-apple-darwin',
        'aarch64-unknown-linux-gnu',
        'x86_64-unknown-linux-gnu',
        'x86_64-pc-windows-msvc',
      ),
    );
  });

  test('rust-analyzer download uses GitHub latest/download, not the API', () {
    expect(
      helixLspGithubLatestUrl(
        'rust-lang/rust-analyzer',
        'rust-analyzer-aarch64-apple-darwin.gz',
      ),
      'https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-aarch64-apple-darwin.gz',
    );
    expect(
      helixLspGithubLatestUrl(
        'rust-lang/rust-analyzer',
        'rust-analyzer-aarch64-apple-darwin.gz',
      ),
      isNot(contains('api.github.com')),
    );
  });

  test('custom binary and JSON config are written into languages.toml', () async {
    final parent = Directory.systemTemp.createTempSync('muse-lsp-override-');
    addTearDown(() => parent.deleteSync(recursive: true));
    final root = Directory('${parent.path}/language-servers')
      ..createSync(recursive: true);
    final binary = File('${parent.path}/custom-ra')..writeAsStringSync('');
    final config = File('${parent.path}/ra.json')
      ..writeAsStringSync('{"check":{"command":"clippy"}}');

    final installer = HelixLanguageServerInstaller(root: root);
    await installer.setOverride(
      'rust-analyzer',
      HelixLspOverride(
        commandPath: binary.path,
        configPath: config.path,
      ),
    );

    final toml = File('${parent.path}/helix-xdg/helix/languages.toml')
        .readAsStringSync();
    expect(toml, contains('command = ${jsonEncode(binary.path)}'));
    expect(toml, contains('[language-server.rust-analyzer.config]'));
    expect(toml, contains('check.command = "clippy"'));
    expect(helixLspIsExecutableFile(binary.path), isTrue);
    expect(helixLspIsExecutableFile(parent.path), isFalse);
  });

  test('JSON config maps into a Helix language-server config table', () {
    final file = File('${Directory.systemTemp.path}/muse-lsp-config.json')
      ..writeAsStringSync('{"diagnostics":{"enable":true}}');
    addTearDown(() {
      if (file.existsSync()) file.deleteSync();
    });
    final toml = helixLspConfigToml('rust-analyzer', file);
    expect(toml, contains('[language-server.rust-analyzer.config]'));
    expect(toml, contains('diagnostics.enable = true'));
  });
}
