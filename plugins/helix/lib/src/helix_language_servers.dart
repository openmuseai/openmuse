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

/// rustup installs proxy commands in Cargo's bin directory. The proxy selects
/// a toolchain from the opened file's working directory, which may lack the
/// rust-analyzer component even when the proxy file exists.
bool isRustupProxyAnalyzer(String path, {String? cargoHome}) {
  if (p.basename(path).toLowerCase() !=
      (Platform.isWindows ? 'rust-analyzer.exe' : 'rust-analyzer')) {
    return false;
  }
  final parent = p.normalize(p.dirname(path)).toLowerCase();
  final home = cargoHome ?? Platform.environment['CARGO_HOME'];
  if (home != null &&
      parent == p.normalize(p.join(home, 'bin')).toLowerCase()) {
    return true;
  }
  return p.basename(parent).toLowerCase() == 'bin' &&
      p.basename(p.dirname(parent)).toLowerCase() == '.cargo';
}

/// rustup's PATH shim exists even when the selected toolchain has no analyzer.
/// Resolve an installed component to its real binary so opening a workspace
/// with a different toolchain override cannot silently break LSP startup.
Future<String?> resolveRustAnalyzerExecutable() async {
  try {
    final toolchains = await Process.run('rustup', ['toolchain', 'list']);
    if (toolchains.exitCode != 0) return null;
    final lines = toolchains.stdout.toString().split(RegExp(r'\r?\n'));
    final names = <String>[
      for (final line in lines)
        if (line.contains('(active') || line.contains('(default)'))
          line.trim().split(' ').first,
      for (final line in lines)
        if (line.trim().isNotEmpty &&
            !line.contains('(active') &&
            !line.contains('(default)'))
          line.trim().split(' ').first,
    ];
    for (final name in names.toSet()) {
      final found = await Process.run('rustup', [
        'which',
        '--toolchain',
        name,
        'rust-analyzer',
      ]);
      final path = found.stdout.toString().trim();
      if (found.exitCode != 0 ||
          !p.isAbsolute(path) ||
          !File(path).existsSync()) {
        continue;
      }
      final version = await Process.run(path, ['--version']);
      if (version.exitCode == 0) return path;
    }
  } on ProcessException {
    // Rust is optional; Helix can still edit files without an LSP.
  }
  return null;
}

/// Helix roots the Rust language server with the `roots` markers of the bundled
/// runtime definition (`Cargo.toml`, `Cargo.lock`), walking up from the opened
/// document. When no ancestor matches, rust-analyzer starts at the document's
/// own directory and answers with "failed to find any projects", so code
/// navigation silently stays dead.
String? rustLanguageServerRoot(String documentPath) {
  var directory = Directory(p.dirname(p.normalize(p.absolute(documentPath))));
  while (true) {
    for (final marker in const ['Cargo.toml', 'Cargo.lock']) {
      if (File(p.join(directory.path, marker)).existsSync()) {
        return directory.path;
      }
    }
    final parent = directory.parent;
    if (parent.path == directory.path) return null;
    directory = parent;
  }
}

const _rustProjectSkippedDirectories = {
  'target',
  'node_modules',
  '.git',
  'vendor',
};

/// Cargo manifests of the Rust projects inside the mounted workspaces. The
/// result feeds rust-analyzer's `linkedProjects`, so a Rust buffer that lives
/// outside every crate (a scratch file, a repository root, a polyglot folder)
/// can still resolve navigation through the projects the user mounted.
List<String> discoverRustProjectManifests(
  Iterable<String> workspaceRoots, {
  int maxDepth = 3,
  int maxProjects = 8,
}) {
  if (maxDepth < 0 || maxProjects < 1) return const [];
  final manifests = <String>[];
  final seen = <String>{};
  for (final root in workspaceRoots) {
    if (manifests.length >= maxProjects) break;
    final base = Directory(p.normalize(p.absolute(root)));
    if (!base.existsSync()) continue;
    final pending = <(Directory, int)>[(base, 0)];
    while (pending.isNotEmpty && manifests.length < maxProjects) {
      final (directory, depth) = pending.removeAt(0);
      final manifest = File(p.join(directory.path, 'Cargo.toml'));
      if (manifest.existsSync()) {
        final normalized = p.normalize(manifest.path);
        if (seen.add(normalized.toLowerCase())) manifests.add(normalized);
        // A crate manifest already covers everything below it.
        continue;
      }
      if (depth >= maxDepth) continue;
      final List<FileSystemEntity> children;
      try {
        children = directory.listSync(followLinks: false);
      } on FileSystemException {
        continue;
      }
      for (final child in children) {
        if (child is! Directory) continue;
        final name = p.basename(child.path).toLowerCase();
        if (name.startsWith('.') ||
            _rustProjectSkippedDirectories.contains(name)) {
          continue;
        }
        pending.add((child, depth + 1));
      }
    }
  }
  manifests.sort();
  return manifests;
}

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
