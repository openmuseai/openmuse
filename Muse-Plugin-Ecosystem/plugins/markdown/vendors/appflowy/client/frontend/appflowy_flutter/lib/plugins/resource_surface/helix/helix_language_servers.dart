import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

enum HelixLspSource { github, dartSdk, npm, go }

enum HelixLspPresence { missing, system, bundled, custom }

final class HelixLspOverride {
  const HelixLspOverride({this.commandPath, this.configPath});

  final String? commandPath;
  final String? configPath;

  Map<String, String> toJson() => {
        if (commandPath != null && commandPath!.isNotEmpty)
          'commandPath': commandPath!,
        if (configPath != null && configPath!.isNotEmpty)
          'configPath': configPath!,
      };

  static HelixLspOverride fromJson(Map<String, dynamic> json) => HelixLspOverride(
        commandPath: json['commandPath'] as String?,
        configPath: json['configPath'] as String?,
      );
}

final class HelixLspPackage {
  const HelixLspPackage({
    required this.id,
    required this.label,
    required this.languages,
    required this.source,
    required this.helixCommand,
    this.helixArgs = const <String>[],
    this.common = false,
    this.githubRepo,
    this.assetPattern,
    this.npmPackages = const <String>[],
    this.goModule,
    this.sizeHint,
    this.binaryHint =
        'Language Server 可执行文件路径（二进制本身，不是它所在的目录）',
    this.defaultConfigFiles = const <String>[],
  });

  final String id;
  final String label;
  final String languages;
  final HelixLspSource source;
  final String helixCommand;
  final List<String> helixArgs;
  final bool common;
  final String? githubRepo;
  final String? assetPattern;
  final List<String> npmPackages;
  final String? goModule;
  final String? sizeHint;
  final String binaryHint;
  final List<String> defaultConfigFiles;
}

/// Tree-sitter grammars Host fetches/builds for syntax highlighting.
/// LSP diagnostics do not color tokens; these `.dylib`/`.so` files do.
const helixHighlightGrammars = <String>[
  'dart',
  'rust',
  'python',
  'javascript',
  'typescript',
  'tsx',
  'json',
  'toml',
  'yaml',
  'markdown',
  'markdown_inline',
  'comment',
  'html',
  'css',
  'bash',
  'go',
  'c',
  'cpp',
];

/// Language servers Host can install into Application Support and put on PATH.
const helixLspCatalog = <HelixLspPackage>[
  HelixLspPackage(
    id: 'rust-analyzer',
    label: 'rust-analyzer',
    languages: 'Rust',
    source: HelixLspSource.github,
    helixCommand: 'rust-analyzer',
    common: true,
    githubRepo: 'rust-lang/rust-analyzer',
    assetPattern: 'rust-analyzer-{triple}.gz',
    sizeHint: '~25 MB',
    binaryHint: 'rust-analyzer 可执行文件，例如 /opt/homebrew/bin/rust-analyzer',
    defaultConfigFiles: ['rust-analyzer.toml', '.vscode/settings.json'],
  ),
  HelixLspPackage(
    id: 'dart',
    label: 'Dart SDK',
    languages: 'Dart / Flutter',
    source: HelixLspSource.dartSdk,
    helixCommand: 'dart',
    helixArgs: ['language-server', '--client-id=helix'],
    common: true,
    sizeHint: '~200 MB',
    binaryHint: 'dart 可执行文件（SDK 的 bin/dart）',
    defaultConfigFiles: ['analysis_options.yaml'],
  ),
  HelixLspPackage(
    id: 'typescript',
    label: 'TypeScript',
    languages: 'JavaScript / TypeScript',
    source: HelixLspSource.npm,
    helixCommand: 'typescript-language-server',
    helixArgs: ['--stdio'],
    common: true,
    npmPackages: ['typescript-language-server', 'typescript'],
    sizeHint: '需 Node.js',
    binaryHint: 'typescript-language-server 可执行文件',
    defaultConfigFiles: ['tsconfig.json', 'jsconfig.json'],
  ),
  HelixLspPackage(
    id: 'ruff',
    label: 'Ruff',
    languages: 'Python',
    source: HelixLspSource.github,
    helixCommand: 'ruff',
    helixArgs: ['server'],
    common: true,
    githubRepo: 'astral-sh/ruff',
    assetPattern: 'ruff-{triple}.tar.gz',
    sizeHint: '~8 MB',
    binaryHint: 'ruff 可执行文件',
    defaultConfigFiles: ['ruff.toml', 'pyproject.toml'],
  ),
  HelixLspPackage(
    id: 'gopls',
    label: 'gopls',
    languages: 'Go',
    source: HelixLspSource.go,
    helixCommand: 'gopls',
    goModule: 'golang.org/x/tools/gopls@latest',
    sizeHint: '需 Go',
    binaryHint: 'gopls 可执行文件',
    defaultConfigFiles: ['go.mod'],
  ),
  HelixLspPackage(
    id: 'clangd',
    label: 'clangd',
    languages: 'C / C++ / Objective-C',
    source: HelixLspSource.github,
    helixCommand: 'clangd',
    githubRepo: 'clangd/clangd',
    assetPattern: 'clangd-{clangdOs}',
    sizeHint: '~50 MB',
    binaryHint: 'clangd 可执行文件',
    defaultConfigFiles: ['.clangd', 'compile_commands.json'],
  ),
  HelixLspPackage(
    id: 'vscode-langservers',
    label: 'JSON / HTML / CSS',
    languages: 'JSON, HTML, CSS',
    source: HelixLspSource.npm,
    helixCommand: 'vscode-json-language-server',
    helixArgs: ['--stdio'],
    npmPackages: ['vscode-langservers-extracted'],
    sizeHint: '需 Node.js',
    binaryHint: 'vscode-json-language-server 可执行文件',
    defaultConfigFiles: ['.vscode/settings.json'],
  ),
];

final class HelixLspStatus {
  const HelixLspStatus({
    required this.package,
    required this.presence,
    this.resolvedPath,
  });

  final HelixLspPackage package;
  final HelixLspPresence presence;
  final String? resolvedPath;

  bool get ready => presence != HelixLspPresence.missing;
}

final class HelixLanguageServerInstaller extends ChangeNotifier {
  HelixLanguageServerInstaller({Directory? root, http.Client? httpClient})
      : _injectedRoot = root,
        _http = httpClient ?? http.Client();

  final Directory? _injectedRoot;
  final http.Client _http;

  Directory? _root;
  String? busyId;
  double progress = 0;
  String? message;
  int revision = 0;
  List<HelixLspStatus> statuses = const [];
  Map<String, HelixLspOverride> overrides = const {};

  Directory get root {
    final injected = _injectedRoot;
    if (injected != null) return injected;
    final cached = _root;
    if (cached != null) return cached;
    throw StateError('HelixLanguageServerInstaller.ensureRoot first');
  }

  Future<Directory> ensureRoot() async {
    if (_injectedRoot != null) return _injectedRoot;
    if (_root != null) return _root!;
    final support = await getApplicationSupportDirectory();
    _root = Directory(p.join(support.path, 'language-servers'));
    await _root!.create(recursive: true);
    await Directory(p.join(_root!.path, 'bin')).create(recursive: true);
    return _root!;
  }

  List<String> get pathEntries {
    final base = (_injectedRoot ?? _root)?.path;
    if (base == null) return const [];
    return [
      p.join(base, 'bin'),
      p.join(base, 'dart-sdk', 'bin'),
      p.join(base, 'npm', 'node_modules', '.bin'),
    ];
  }

  String pathPrefix(String currentPath) {
    final extra = pathEntries.where((dir) => Directory(dir).existsSync());
    if (extra.isEmpty) return currentPath;
    // Windows separates PATH entries with ';', everything else with ':'.
    final separator = Platform.isWindows ? ';' : ':';
    return '${extra.join(separator)}$separator$currentPath';
  }

  Directory get xdgConfigHome {
    final base = (_injectedRoot ?? _root)?.path;
    if (base == null) {
      throw StateError('HelixLanguageServerInstaller.ensureRoot first');
    }
    return Directory(p.join(p.dirname(base), 'helix-xdg'));
  }

  String get grammarRuntime {
    return p.join(xdgConfigHome.path, 'helix', 'runtime');
  }

  List<String> grammarRuntimes(String helixRuntime) => [
        grammarRuntime,
        helixRuntime,
      ];

  Future<void> refresh() async {
    await ensureRoot();
    await _loadOverrides();
    final next = <HelixLspStatus>[];
    for (final package in helixLspCatalog) {
      next.add(await _statusFor(package));
    }
    statuses = next;
    // Resolution results changed, so the generated languages.toml must be
    // rebuilt on the next write instead of being served from the cache.
    _languagesTomlSignature = null;
    notifyListeners();
  }

  File get _overridesFile => File(p.join(root.path, 'overrides.json'));

  String get defaultConfigDirectory =>
      p.join(xdgConfigHome.path, 'helix');

  Future<void> _loadOverrides() async {
    await ensureRoot();
    final file = _overridesFile;
    if (!file.existsSync()) {
      overrides = const {};
      return;
    }
    try {
      final json = jsonDecode(await file.readAsString()) as Map<String, dynamic>;
      overrides = {
        for (final entry in json.entries)
          if (entry.value is Map)
            entry.key: HelixLspOverride.fromJson(
              Map<String, dynamic>.from(entry.value as Map),
            ),
      };
    } on Object {
      overrides = const {};
    }
  }

  Future<void> _saveOverrides() async {
    await ensureRoot();
    final json = {
      for (final entry in overrides.entries)
        if (entry.value.commandPath != null || entry.value.configPath != null)
          entry.key: entry.value.toJson(),
    };
    await _overridesFile.writeAsString(const JsonEncoder.withIndent('  ').convert(json));
  }

  Future<void> setOverride(String id, HelixLspOverride next) async {
    await ensureRoot();
    await _loadOverrides();
    final copy = Map<String, HelixLspOverride>.from(overrides);
    if ((next.commandPath == null || next.commandPath!.isEmpty) &&
        (next.configPath == null || next.configPath!.isEmpty)) {
      copy.remove(id);
    } else {
      copy[id] = next;
    }
    overrides = copy;
    await _saveOverrides();
    await writeLanguagesToml();
    revision += 1;
    notifyListeners();
    await refresh();
  }

  Future<HelixLspStatus> _statusFor(HelixLspPackage package) async {
    final custom = overrides[package.id]?.commandPath;
    if (custom != null && custom.isNotEmpty && File(custom).existsSync()) {
      return HelixLspStatus(
        package: package,
        presence: HelixLspPresence.custom,
        resolvedPath: custom,
      );
    }
    final bundled = _bundledCommand(package);
    if (bundled != null && File(bundled).existsSync()) {
      return HelixLspStatus(
        package: package,
        presence: HelixLspPresence.bundled,
        resolvedPath: bundled,
      );
    }
    final system = await _which(package.helixCommand);
    if (system != null) {
      return HelixLspStatus(
        package: package,
        presence: HelixLspPresence.system,
        resolvedPath: system,
      );
    }
    return HelixLspStatus(package: package, presence: HelixLspPresence.missing);
  }

  String? _bundledCommand(HelixLspPackage package) {
    final base = root.path;
    return switch (package.id) {
      'dart' => p.join(base, 'dart-sdk', 'bin', 'dart'),
      'typescript' => p.join(
          base,
          'npm',
          'node_modules',
          '.bin',
          'typescript-language-server',
        ),
      'vscode-langservers' => p.join(
          base,
          'npm',
          'node_modules',
          '.bin',
          'vscode-json-language-server',
        ),
      _ => p.join(base, 'bin', package.helixCommand),
    };
  }

  Future<void> install(String id) async {
    final package = helixLspCatalog.firstWhere((item) => item.id == id);
    busyId = id;
    progress = 0;
    message = '准备安装 ${package.label}…';
    notifyListeners();
    try {
      await ensureRoot();
      switch (package.source) {
        case HelixLspSource.github:
          if (package.id == 'rust-analyzer') {
            await _installRustAnalyzer(package);
          } else {
            await _installGithub(package);
          }
        case HelixLspSource.dartSdk:
          await _installDartSdk();
        case HelixLspSource.npm:
          await _installNpm(package);
        case HelixLspSource.go:
          await _installGo(package);
      }
      await writeLanguagesToml();
      if (package.id == 'dart') {
        try {
          final install = await HelixInstall.resolve();
          await _compileGrammars(
            hxBinary: install.binary,
            helixRuntime: install.runtime,
          );
        } on Object {
          // Highlight grammars are independent of the Dart SDK install.
        }
      }
      revision += 1;
      message = '${package.label} 已安装';
    } on Object catch (error) {
      message = '$error';
      rethrow;
    } finally {
      busyId = null;
      progress = 0;
      await refresh();
    }
  }

  Future<void> installCommon() async {
    for (final package in helixLspCatalog.where((item) => item.common)) {
      final status = await _statusFor(package);
      if (status.ready) continue;
      await install(package.id);
    }
  }

  Future<void> buildGrammars({
    required String hxBinary,
    required String helixRuntime,
  }) async {
    if (busyId != null && busyId != 'grammars') {
      throw StateError('正在安装 $busyId');
    }
    busyId = 'grammars';
    progress = 0;
    message = '准备语法高亮…';
    notifyListeners();
    try {
      await _compileGrammars(hxBinary: hxBinary, helixRuntime: helixRuntime);
      revision += 1;
      message = '语法高亮已安装';
    } on Object catch (error) {
      message = '$error';
      rethrow;
    } finally {
      busyId = null;
      progress = 0;
      await refresh();
    }
  }

  /// Signature of the last written languages.toml, so repeated calls on the
  /// file-open path do not re-resolve every language server.
  String? _languagesTomlSignature;

  Future<void> writeLanguagesToml({bool force = false}) async {
    await ensureRoot();
    final file = File(p.join(xdgConfigHome.path, 'helix', 'languages.toml'));
    // Resolving the catalog walks PATH for each language server (~340 ms with
    // 7 packages here) and this sits on the path of every Helix file open, so
    // skip the whole rebuild unless the installer state actually changed.
    final signature = Object.hash(
      revision,
      Object.hashAll([
        for (final entry in overrides.entries)
          Object.hash(
            entry.key,
            entry.value.commandPath,
            entry.value.configPath,
          ),
      ]),
    ).toString();
    if (!force && signature == _languagesTomlSignature && file.existsSync()) {
      return;
    }
    final blocks = <String>[
      'use-grammars = { only = ${jsonEncode(helixHighlightGrammars)} }',
    ];
    for (final status in [
      for (final package in helixLspCatalog) await _statusFor(package),
    ]) {
      if (!status.ready || status.resolvedPath == null) continue;
      final command = status.resolvedPath!;
      final args = status.package.helixArgs.isEmpty
          ? ''
          : '\nargs = ${jsonEncode(status.package.helixArgs)}';
      blocks.add(
        '[language-server.${status.package.helixCommand}]\n'
        'command = ${jsonEncode(command)}$args',
      );
      final configPath = overrides[status.package.id]?.configPath;
      if (configPath != null &&
          configPath.isNotEmpty &&
          File(configPath).existsSync()) {
        final configBlock = helixLspConfigToml(
          status.package.helixCommand,
          File(configPath),
        );
        if (configBlock != null) blocks.add(configBlock);
      }
      if (status.package.id == 'vscode-langservers') {
        final bin = p.dirname(command);
        blocks.add(
          '[language-server.vscode-html-language-server]\n'
          'command = ${jsonEncode(p.join(bin, 'vscode-html-language-server'))}\n'
          'args = ["--stdio"]',
        );
        blocks.add(
          '[language-server.vscode-css-language-server]\n'
          'command = ${jsonEncode(p.join(bin, 'vscode-css-language-server'))}\n'
          'args = ["--stdio"]',
        );
      }
    }
    final text = '${blocks.join('\n\n')}\n';
    await file.parent.create(recursive: true);
    await file.writeAsString(text);
    _languagesTomlSignature = signature;
  }

  Future<void> _compileGrammars({
    required String hxBinary,
    required String helixRuntime,
  }) async {
    await ensureRoot();
    final git = await _which('git');
    if (git == null) {
      throw StateError('未找到 git。语法高亮需要 git 拉取 tree-sitter 源码。');
    }
    final cc = await _which('clang') ?? await _which('cc');
    if (cc == null) {
      throw StateError(
        '未找到 C 编译器。macOS 请先安装 Xcode Command Line Tools（xcode-select --install）。',
      );
    }
    await writeLanguagesToml();
    final env = helixGrammarProcessEnvironment(
      helixRuntime: helixRuntime,
      xdgConfigHome: xdgConfigHome.path,
    );
    progress = 0.2;
    message = '正在下载 tree-sitter 源码…';
    notifyListeners();
    await _runHxGrammar(hxBinary, 'fetch', env);
    progress = 0.6;
    message = '正在编译语法高亮…';
    notifyListeners();
    await _runHxGrammar(hxBinary, 'build', env);
    progress = 1;
    notifyListeners();
  }

  Future<void> _runHxGrammar(
    String hxBinary,
    String action,
    Map<String, String> environment,
  ) async {
    final result = await Process.run(
      hxBinary,
      ['--grammar', action],
      environment: environment,
    );
    if (result.exitCode != 0) {
      final err = '${result.stderr}\n${result.stdout}'.trim();
      throw StateError('hx --grammar $action 失败: $err');
    }
  }

  Future<void> _installRustAnalyzer(HelixLspPackage package) async {
    final rustup = await _which('rustup');
    if (rustup != null) {
      message = '通过 rustup 安装 rust-analyzer…';
      notifyListeners();
      final add = await Process.run(
        rustup,
        ['component', 'add', 'rust-analyzer'],
      );
      if (add.exitCode == 0) {
        final which = await Process.run(rustup, ['which', 'rust-analyzer']);
        final path = which.stdout.toString().trim();
        if (which.exitCode == 0 && path.isNotEmpty && File(path).existsSync()) {
          final dest = File(p.join(root.path, 'bin', 'rust-analyzer'));
          await dest.parent.create(recursive: true);
          await File(path).copy(dest.path);
          await _markExecutable(dest);
          return;
        }
      }
    }
    await _installGithub(package);
  }

  Future<void> _installGithub(HelixLspPackage package) async {
    final triple = helixLspHostTriple();
    final repo = package.githubRepo!;
    final dest = File(p.join(root.path, 'bin', package.helixCommand));
    await dest.parent.create(recursive: true);

    final assetName = switch (package.id) {
      'clangd' => null,
      _ => package.assetPattern
          ?.replaceAll('{triple}', triple)
          .replaceAll('{clangdOs}', Platform.isMacOS ? 'mac' : 'linux'),
    };
    if (assetName != null) {
      final direct = Uri.parse(helixLspGithubLatestUrl(repo, assetName));
      message = '下载 $assetName…';
      notifyListeners();
      try {
        final archive = File(p.join(root.path, 'tmp-$assetName'));
        await _download(direct, archive);
        await _unpackGithubAsset(package, archive, dest, assetName);
        return;
      } on Object catch (error) {
        message = '直链失败，改用 GitHub API（$error）';
        notifyListeners();
      }
    }

    message = '解析 $repo 最新版本…';
    notifyListeners();
    final release = jsonDecode(
      await _getText(
        Uri.parse('https://api.github.com/repos/$repo/releases/latest'),
      ),
    ) as Map<String, dynamic>;
    final assets = (release['assets'] as List<dynamic>).cast<Map<String, dynamic>>();
    final needle = switch (package.id) {
      'clangd' => Platform.isMacOS ? 'clangd-mac' : 'clangd-linux',
      'ruff' => 'ruff-$triple',
      _ => package.assetPattern!.replaceAll('{triple}', triple),
    };
    final asset = assets.firstWhere(
      (item) {
        final name = item['name'] as String? ?? '';
        return name.contains(needle.replaceAll('.gz', '')) ||
            name.contains(needle);
      },
      orElse: () => throw StateError('没有匹配 $needle 的安装包'),
    );
    final url = asset['browser_download_url'] as String;
    final name = asset['name'] as String;
    final archive = File(p.join(root.path, 'tmp-$name'));
    await _download(Uri.parse(url), archive);
    await _unpackGithubAsset(package, archive, dest, name);
  }

  Future<void> _unpackGithubAsset(
    HelixLspPackage package,
    File archive,
    File dest,
    String name,
  ) async {
    message = '解压 $name…';
    notifyListeners();
    if (name.endsWith('.gz') && !name.endsWith('.tar.gz')) {
      final process = await Process.start('gzip', ['-dc', archive.path]);
      final bytes = await process.stdout.fold<List<int>>(
        <int>[],
        (buffer, chunk) => buffer..addAll(chunk),
      );
      final code = await process.exitCode;
      if (code != 0) {
        final err = await utf8.decodeStream(process.stderr);
        throw StateError('gzip 失败: $err');
      }
      await dest.writeAsBytes(bytes, flush: true);
    } else {
      final unpackDir = Directory(p.join(root.path, 'tmp-${package.id}'));
      if (unpackDir.existsSync()) {
        await unpackDir.delete(recursive: true);
      }
      await unpackDir.create(recursive: true);
      final extract = name.endsWith('.zip')
          ? await Process.run('unzip', ['-o', archive.path, '-d', unpackDir.path])
          : await Process.run(
              'tar',
              ['-xf', archive.path, '-C', unpackDir.path],
            );
      if (extract.exitCode != 0) {
        throw StateError('解压失败: ${extract.stderr}');
      }
      final binary = await _findNamedBinary(unpackDir, package.helixCommand);
      if (binary == null) {
        throw StateError('压缩包里没有 ${package.helixCommand}');
      }
      await binary.copy(dest.path);
      await unpackDir.delete(recursive: true);
    }
    if (archive.existsSync()) await archive.delete();
    await _markExecutable(dest);
  }

  Future<void> _installDartSdk() async {
    final triple = helixLspHostTriple();
    final sdkName = switch (triple) {
      'aarch64-apple-darwin' => 'dartsdk-macos-arm64-release.zip',
      'x86_64-apple-darwin' => 'dartsdk-macos-x64-release.zip',
      'aarch64-unknown-linux-gnu' => 'dartsdk-linux-arm64-release.zip',
      _ => 'dartsdk-linux-x64-release.zip',
    };
    final url = Uri.parse(
      'https://storage.googleapis.com/dart-archive/channels/stable/release/latest/sdk/$sdkName',
    );
    final archive = File(p.join(root.path, sdkName));
    await _download(url, archive);
    message = '解压 Dart SDK…';
    notifyListeners();
    final extract = await Process.run(
      'unzip',
      ['-o', archive.path, '-d', root.path],
    );
    if (extract.exitCode != 0) {
      throw StateError('解压 Dart SDK 失败: ${extract.stderr}');
    }
    await archive.delete();
    await _markExecutable(File(p.join(root.path, 'dart-sdk', 'bin', 'dart')));
  }

  Future<void> _installNpm(HelixLspPackage package) async {
    final npm = await _which('npm');
    if (npm == null) {
      throw StateError('未找到 npm。请先安装 Node.js（https://nodejs.org）后再点安装。');
    }
    final prefix = Directory(p.join(root.path, 'npm'));
    await prefix.create(recursive: true);
    message = 'npm install ${package.npmPackages.join(' ')}…';
    notifyListeners();
    final result = await Process.run(
      npm,
      ['install', '--prefix', prefix.path, ...package.npmPackages],
      environment: Platform.environment,
    );
    if (result.exitCode != 0) {
      throw StateError('npm 安装失败: ${result.stderr}');
    }
  }

  Future<void> _installGo(HelixLspPackage package) async {
    final go = await _which('go');
    if (go == null) {
      throw StateError('未找到 go。请先安装 Go（https://go.dev/dl）后再点安装。');
    }
    final bin = Directory(p.join(root.path, 'bin'));
    await bin.create(recursive: true);
    message = 'go install ${package.goModule}…';
    notifyListeners();
    final result = await Process.run(
      go,
      ['install', package.goModule!],
      environment: {
        ...Platform.environment,
        'GOBIN': bin.path,
      },
    );
    if (result.exitCode != 0) {
      throw StateError('go install 失败: ${result.stderr}');
    }
  }

  Future<void> _download(Uri url, File dest) async {
    message = '下载 ${p.basename(dest.path)}…';
    notifyListeners();
    var current = url;
    http.StreamedResponse? response;
    for (var hop = 0; hop < 8; hop++) {
      final request = http.Request('GET', current);
      request.followRedirects = false;
      request.headers['User-Agent'] = 'OpenMuseHost/1.0';
      request.headers['Accept'] = '*/*';
      response = await _http.send(request);
      final code = response.statusCode;
      if (code >= 300 && code < 400) {
        final location = response.headers['location'];
        await response.stream.drain();
        if (location == null || location.isEmpty) {
          throw StateError('下载失败 HTTP $code');
        }
        current = current.resolve(location);
        continue;
      }
      break;
    }
    final streamed = response!;
    if (streamed.statusCode < 200 || streamed.statusCode >= 300) {
      throw StateError('下载失败 HTTP ${streamed.statusCode}');
    }
    final total = streamed.contentLength ?? 0;
    final sink = dest.openWrite();
    var received = 0;
    await for (final chunk in streamed.stream) {
      sink.add(chunk);
      received += chunk.length;
      if (total > 0) {
        progress = received / total;
        notifyListeners();
      }
    }
    await sink.close();
    progress = 1;
    notifyListeners();
  }

  Future<String> _getText(Uri url) async {
    final response = await _http.get(
      url,
      headers: {
        'User-Agent': 'OpenMuseHost/1.0',
        'Accept': 'application/vnd.github+json',
        'X-GitHub-Api-Version': '2022-11-28',
      },
    );
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw StateError('请求失败 HTTP ${response.statusCode}');
    }
    return response.body;
  }
}

Future<File?> _findNamedBinary(Directory root, String name) async {
  await for (final entity in root.list(recursive: true, followLinks: false)) {
    if (entity is File && p.basename(entity.path) == name) {
      return entity;
    }
  }
  return null;
}

Future<void> _markExecutable(File file) async {
  if (!file.existsSync()) return;
  if (Platform.isWindows) return; // Windows has no executable bit.
  await Process.run('chmod', ['+x', file.path]);
  if (Platform.isMacOS) {
    await Process.run('xattr', ['-dr', 'com.apple.quarantine', file.path]);
  }
}

/// Resolves a command against `PATH` without shelling out: Windows ships
/// neither `which` nor a POSIX-like lookup, and spawning a missing helper
/// throws instead of returning null.
Future<String?> _which(String command) async {
  if (command.trim().isEmpty) return null;
  final separator = Platform.isWindows ? ';' : ':';
  final entries = (Platform.environment['PATH'] ?? '').split(separator);
  final suffixes = <String>[''];
  if (Platform.isWindows && p.extension(command).isEmpty) {
    final pathExt = Platform.environment['PATHEXT'] ?? '.COM;.EXE;.BAT;.CMD';
    for (final ext in pathExt.split(';')) {
      final trimmed = ext.trim().toLowerCase();
      if (trimmed.isNotEmpty) suffixes.add(trimmed);
    }
  }
  for (final entry in entries) {
    final dir = entry.trim().replaceAll('"', '');
    if (dir.isEmpty) continue;
    for (final suffix in suffixes) {
      final candidate = File(p.join(dir, '$command$suffix'));
      if (candidate.existsSync()) return candidate.path;
    }
  }
  return null;
}

String helixLspHostTriple() {
  final cpu = _cpuName();
  if (Platform.isMacOS) {
    return cpu == 'arm64' || cpu == 'aarch64'
        ? 'aarch64-apple-darwin'
        : 'x86_64-apple-darwin';
  }
  if (Platform.isLinux) {
    return cpu == 'arm64' || cpu == 'aarch64'
        ? 'aarch64-unknown-linux-gnu'
        : 'x86_64-unknown-linux-gnu';
  }
  return 'x86_64-pc-windows-msvc';
}

String _cpuName() {
  try {
    final result = Process.runSync('uname', ['-m']);
    if (result.exitCode == 0) {
      return result.stdout.toString().trim();
    }
  } on Object {
    // Fall through.
  }
  return 'x86_64';
}

String helixLspGithubLatestUrl(String repo, String asset) =>
    'https://github.com/$repo/releases/latest/download/$asset';

String? helixLspConfigToml(String serverId, File file) {
  final raw = file.readAsStringSync().trim();
  if (raw.isEmpty) return null;
  try {
    final decoded = jsonDecode(raw);
    if (decoded is Map<String, dynamic>) {
      final buffer = StringBuffer('[language-server.$serverId.config]\n');
      _writeTomlMap(buffer, decoded, '');
      return buffer.toString();
    }
  } on Object {
    // Fall through: treat as raw TOML under the config table.
  }
  return '[language-server.$serverId.config]\n$raw\n';
}

void _writeTomlMap(StringBuffer buffer, Map<String, dynamic> map, String prefix) {
  for (final entry in map.entries) {
    final key = prefix.isEmpty ? entry.key : '$prefix.${entry.key}';
    final value = entry.value;
    if (value is Map<String, dynamic>) {
      _writeTomlMap(buffer, value, key);
    } else if (value is Map) {
      _writeTomlMap(buffer, Map<String, dynamic>.from(value), key);
    } else {
      buffer.writeln('$key = ${jsonEncode(value)}');
    }
  }
}

bool helixLspIsExecutableFile(String path) {
  if (path.trim().isEmpty) return false;
  return FileSystemEntity.typeSync(path) == FileSystemEntityType.file;
}
