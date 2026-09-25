import 'dart:io';

import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

/// The plugin owns the LS catalog. Host persists paths without interpreting it.
final class HelixLanguageServerSpec {
  const HelixLanguageServerSpec(
    this.command,
    this.label,
    this.arguments, {
    this.binaryName,
    this.npmPackages = const [],
  });

  final String command;
  final String label;
  final List<String> arguments;
  final String? binaryName;
  final List<String> npmPackages;
  bool get oneClickInstallable =>
      command == 'rust-analyzer' || npmPackages.isNotEmpty;
}

const helixLanguageServers = <HelixLanguageServerSpec>[
  HelixLanguageServerSpec('dart', 'Dart / Flutter', [
    'language-server',
    '--client-id=helix',
  ]),
  HelixLanguageServerSpec('rust-analyzer', 'Rust', []),
  HelixLanguageServerSpec(
    'typescript-language-server',
    'TypeScript / JavaScript',
    ['--stdio'],
    npmPackages: ['typescript-language-server@6.0.0', 'typescript@6.0.3'],
  ),
  HelixLanguageServerSpec('ruff', 'Python / Ruff', ['server']),
  HelixLanguageServerSpec('gopls', 'Go', []),
  HelixLanguageServerSpec('clangd', 'C / C++ / Objective-C', []),
  HelixLanguageServerSpec(
    'vscode-json-language-server',
    'JSON',
    ['--stdio'],
    npmPackages: ['vscode-langservers-extracted@4.10.0'],
  ),
  HelixLanguageServerSpec(
    'vscode-html-language-server',
    'HTML',
    ['--stdio'],
    npmPackages: ['vscode-langservers-extracted@4.10.0'],
  ),
  HelixLanguageServerSpec(
    'vscode-css-language-server',
    'CSS / SCSS',
    ['--stdio'],
    npmPackages: ['vscode-langservers-extracted@4.10.0'],
  ),
  HelixLanguageServerSpec(
    'bash-language-server',
    'Shell / Bash',
    ['start'],
    npmPackages: ['bash-language-server@5.8.1'],
  ),
  HelixLanguageServerSpec(
    'yaml-language-server',
    'YAML',
    ['--stdio'],
    npmPackages: ['yaml-language-server@1.24.0'],
  ),
  HelixLanguageServerSpec(
    'svelteserver',
    'Svelte',
    ['--stdio'],
    npmPackages: ['svelte-language-server@0.18.4', 'typescript@6.0.3'],
  ),
  HelixLanguageServerSpec(
    'vuels',
    'Vue',
    ['--stdio'],
    binaryName: 'vue-language-server',
    npmPackages: ['@vue/language-server@3.3.11', 'typescript@6.0.3'],
  ),
  HelixLanguageServerSpec('jdtls', 'Java', []),
  HelixLanguageServerSpec('lua-language-server', 'Lua', []),
  HelixLanguageServerSpec('omnisharp', 'C#', [
    '--languageserver',
  ], binaryName: 'OmniSharp'),
  HelixLanguageServerSpec('docker-langserver', 'Dockerfile', ['--stdio']),
];

enum HelixServerPresence { custom, system, missing }

final class HelixServerStatus {
  const HelixServerStatus(this.spec, this.presence, this.path);
  final HelixLanguageServerSpec spec;
  final HelixServerPresence presence;
  final String? path;
}

List<HelixServerStatus> inspectHelixLanguageServers(
  Map<String, String> overrides, {
  String? pathEnvironment,
}) {
  final pathEntries = (pathEnvironment ?? Platform.environment['PATH'] ?? '')
      .split(Platform.isWindows ? ';' : ':')
      .where((item) => item.isNotEmpty);
  return [
    for (final spec in helixLanguageServers)
      _inspect(spec, overrides[spec.command], pathEntries),
  ];
}

HelixServerStatus _inspect(
  HelixLanguageServerSpec spec,
  String? override,
  Iterable<String> pathEntries,
) {
  if (override != null &&
      p.isAbsolute(override) &&
      File(override).existsSync()) {
    return HelixServerStatus(spec, HelixServerPresence.custom, override);
  }
  for (final directory in pathEntries) {
    for (final suffix
        in Platform.isWindows ? const ['', '.exe', '.cmd'] : const ['']) {
      final candidate = p.join(
        directory,
        '${spec.binaryName ?? spec.command}$suffix',
      );
      if (File(candidate).existsSync()) {
        return HelixServerStatus(spec, HelixServerPresence.system, candidate);
      }
    }
  }
  return HelixServerStatus(spec, HelixServerPresence.missing, null);
}

/// Installs only explicitly selected servers, into a versioned local directory.
/// It never runs a shell, overwrites an existing install, or changes PATH.
Future<String> installHelixLanguageServer(
  HelixLanguageServerSpec spec, {
  Directory? root,
}) async {
  if (!spec.oneClickInstallable) {
    throw StateError('${spec.label} 暂无经验证的一键安装源；请配置本机可执行文件。');
  }
  if (spec.command == 'rust-analyzer') {
    final add = await Process.run('rustup', [
      'component',
      'add',
      'rust-analyzer',
    ]);
    if (add.exitCode != 0) throw StateError('rustup 安装失败：${add.stderr}');
    final which = await Process.run('rustup', ['which', 'rust-analyzer']);
    final path = which.stdout.toString().trim();
    if (which.exitCode != 0 || !File(path).existsSync()) {
      throw StateError('rustup 已执行，但未找到 rust-analyzer 可执行文件');
    }
    return path;
  }
  final support =
      root ??
      Directory(
        p.join(
          (await getApplicationSupportDirectory()).path,
          'OpenMuse',
          'language-servers',
        ),
      );
  await support.create(recursive: true);
  final install = Directory(
    p.join(
      support.path,
      '${spec.command}-${DateTime.now().microsecondsSinceEpoch}',
    ),
  );
  await install.create();
  final result = await Process.run('npm', [
    'install',
    '--prefix',
    install.path,
    '--no-audit',
    '--no-fund',
    ...spec.npmPackages,
  ]);
  if (result.exitCode != 0) {
    throw StateError('npm 安装失败：${result.stderr}');
  }
  final suffix = Platform.isWindows ? '.cmd' : '';
  final binary = File(
    p.join(
      install.path,
      'node_modules',
      '.bin',
      '${spec.binaryName ?? spec.command}$suffix',
    ),
  );
  if (!binary.existsSync()) {
    throw StateError('安装结束，但未找到 ${spec.binaryName ?? spec.command}');
  }
  return binary.absolute.path;
}
