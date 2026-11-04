import 'dart:convert';
import 'dart:io';

import 'package:path/path.dart' as p;
import 'package:pub_semver/pub_semver.dart';
import 'package:yaml/yaml.dart';

/// The first curated market snapshot ships inside the signed application.
/// Remote catalog updates will require a separately signed index.
final class MarketPlugin {
  const MarketPlugin({
    required this.id,
    required this.name,
    required this.ecosystem,
    required this.packageName,
    required this.version,
    required this.integrity,
    required this.supportedDshVersions,
    required this.description,
    required this.categories,
  });

  final String id;
  final String name;
  final String ecosystem;
  final String packageName;
  final String version;
  final String integrity;
  final Set<String> supportedDshVersions;
  final String description;
  final List<String> categories;

  String get exactPackage => '$packageName@$version';

  Map<String, Object?> toJson() => {
    'id': id,
    'name': name,
    'ecosystem': ecosystem,
    'package': packageName,
    'version': version,
    'integrity': integrity,
    'supportedDshVersions': supportedDshVersions.toList()..sort(),
    'description': description,
    'categories': categories,
  };
}

const builtInMarketPlugins = <MarketPlugin>[
  MarketPlugin(
    id: 'archify',
    name: 'Archify',
    ecosystem: 'dsh',
    packageName: '@tt-a1i/archify-dsh',
    version: '1.0.0',
    // npm dist.integrity for the exact published tarball. The post-install
    // pnpm lockfile must contain this digest for Archify's package snapshot.
    integrity:
        'sha512-ae5L+SyZngy3EzYnWHHfccAYJCZh5e9MTkEBh7mrwTMfpwpFtkIhiOg2vI5xnluCS8CSb+Ne1hi1kOWRQQtGBQ==',
    supportedDshVersions: {'0.2.0-rc.2'},
    description: '在 DSH 会话中把复杂项目结构转为清晰的架构图。',
    categories: ['编程'],
  ),
];

final class DshMarketInstaller {
  DshMarketInstaller({
    required this.dshHome,
    required this.profile,
    required this.cliPath,
    required this.nodeExecutable,
    String? packageManagerExecutable,
    Map<String, String>? baseEnvironment,
  }) : packageManagerExecutable =
           packageManagerExecutable ?? _bundledPnpmFor(cliPath),
       baseEnvironment = baseEnvironment ?? Platform.environment;

  final Directory dshHome;
  final String profile;
  final String cliPath;
  final String nodeExecutable;
  final String packageManagerExecutable;
  final Map<String, String> baseEnvironment;

  static String _bundledPnpmFor(String cliPath) {
    final nodeModules = p.dirname(p.dirname(p.dirname(p.dirname(cliPath))));
    return p.join(
      nodeModules,
      '.bin',
      Platform.isWindows ? 'pnpm.cmd' : 'pnpm',
    );
  }

  Directory get profileDirectory =>
      Directory(p.join(dshHome.path, 'profiles', profile));

  Future<Map<String, Object?>> plan(MarketPlugin plugin) async {
    _checkInputs(plugin);
    final versionResult = await _run(['-V']);
    if (versionResult.exitCode != 0) {
      throw StateError('无法读取 DSH 版本：${versionResult.stderr}');
    }
    final dshVersion = '${versionResult.stdout}'.trim();
    Version.parse(dshVersion);
    if (!plugin.supportedDshVersions.contains(dshVersion)) {
      throw StateError(
        '${plugin.name} ${plugin.version} 尚未通过 DSH $dshVersion 兼容验证；已验证版本：${plugin.supportedDshVersions.join(', ')}',
      );
    }
    final nodeResult = await Process.run(nodeExecutable, ['--version']);
    if (nodeResult.exitCode != 0) throw StateError('无法读取 Node 版本');
    final nodeVersion = Version.parse(
      '${nodeResult.stdout}'.trim().replaceFirst('v', ''),
    );
    if (!VersionConstraint.parse('^22.19.0').allows(nodeVersion) &&
        !VersionConstraint.parse('>=24.0.0').allows(nodeVersion)) {
      throw StateError(
        '${plugin.name} 需要 Node ^22.19.0 或 >=24.0.0，当前为 $nodeVersion',
      );
    }
    final pnpmResult = await Process.run(packageManagerExecutable, [
      '--version',
    ], environment: _environment());
    if (pnpmResult.exitCode != 0 ||
        '${pnpmResult.stdout}'.trim() != '10.34.1') {
      throw StateError('内置 pnpm 运行失败或版本不匹配：${pnpmResult.stderr}');
    }
    final installed = _installedVersion(plugin);
    final enabled = installed == null ? false : _isEnabled(plugin);
    return {
      ...plugin.toJson(),
      'profile': profile,
      'dshVersion': dshVersion,
      'nodeVersion': nodeVersion.toString(),
      'pnpmVersion': '10.34.1',
      'installedVersion': installed,
      'enabled': enabled,
      'action': installed == plugin.version
          ? (enabled ? 'already-installed' : 'enable')
          : 'install',
    };
  }

  Future<Map<String, Object?>> install(MarketPlugin plugin) async {
    final planned = await plan(plugin);
    if (planned['action'] == 'already-installed') {
      _verifyInstalled(plugin);
      return planned;
    }
    if (planned['action'] == 'enable') return setEnabled(plugin, true);
    final result = await _run([
      'plugin',
      '--profile',
      profile,
      'add',
      plugin.exactPackage,
      '--save-exact',
    ], timeout: const Duration(minutes: 5));
    if (result.exitCode != 0) {
      throw StateError(
        'DSH 安装失败（${result.exitCode}）：${result.stderr}\n${result.stdout}',
      );
    }
    _verifyInstalled(plugin);
    return {
      ...planned,
      'action': 'installed',
      'installedVersion': plugin.version,
    };
  }

  Future<Map<String, Object?>> setEnabled(
    MarketPlugin plugin,
    bool enabled,
  ) async {
    _checkInputs(plugin);
    _verifyInstalled(plugin, requireEnabled: false);
    final current = _installedVersion(plugin);
    if (current != plugin.version) {
      throw StateError('插件未按市场固定版本安装，无法切换启用状态');
    }
    final manifest = File(p.join(profileDirectory.path, 'package.json'));
    final data = jsonDecode(await manifest.readAsString());
    if (data is! Map ||
        data['dsh'] is! Map ||
        (data['dsh'] as Map)['profile'] is! Map ||
        ((data['dsh'] as Map)['profile'] as Map)['bundles'] is! List) {
      throw StateError('DSH profile 缺少插件 bundles');
    }
    final bundles = ((data['dsh'] as Map)['profile'] as Map)['bundles'] as List;
    if (enabled) {
      if (!bundles.contains(plugin.packageName)) {
        bundles.add(plugin.packageName);
      }
    } else {
      bundles.removeWhere((item) => item == plugin.packageName);
    }
    final temporary = File('${manifest.path}.openmuse.tmp');
    await temporary.writeAsString(
      '${const JsonEncoder.withIndent('  ').convert(data)}\n',
      flush: true,
    );
    await temporary.rename(manifest.path);
    _verifyInstalled(plugin, requireEnabled: enabled);
    return {
      ...plugin.toJson(),
      'profile': profile,
      'installedVersion': current,
      'enabled': enabled,
      'restartRequired': true,
      'action': enabled ? 'enabled' : 'disabled',
    };
  }

  bool _isEnabled(MarketPlugin plugin) {
    final manifest = File(p.join(profileDirectory.path, 'package.json'));
    if (!manifest.existsSync()) return false;
    final decoded = jsonDecode(manifest.readAsStringSync());
    final bundles =
        decoded is Map &&
            decoded['dsh'] is Map &&
            (decoded['dsh'] as Map)['profile'] is Map
        ? ((decoded['dsh'] as Map)['profile'] as Map)['bundles']
        : null;
    return bundles is List && bundles.contains(plugin.packageName);
  }

  void _checkInputs(MarketPlugin plugin) {
    if (plugin.ecosystem != 'dsh') {
      throw UnsupportedError('尚未接入 ${plugin.ecosystem} 市场适配器');
    }
    if (profile.isEmpty ||
        profile == '.' ||
        profile == '..' ||
        profile == 'node_modules' ||
        profile.contains('/') ||
        profile.contains('\\') ||
        profile.contains('\x00')) {
      throw const FormatException('无效 DSH profile 名称');
    }
    if (!p.isAbsolute(dshHome.path)) {
      throw const FormatException('DSH_HOME 必须是绝对路径');
    }
    if (!File(cliPath).existsSync()) {
      throw FormatException('找不到 DSH CLI：$cliPath');
    }
    if (!File(packageManagerExecutable).existsSync()) {
      throw StateError(
        '应用内缺少 pnpm，请更新或重新安装 OpenMuse：$packageManagerExecutable',
      );
    }
  }

  String? _installedVersion(MarketPlugin plugin) {
    final file = File(
      p.joinAll([
        profileDirectory.path,
        'node_modules',
        ...plugin.packageName.split('/'),
        'package.json',
      ]),
    );
    if (!file.existsSync()) return null;
    final data = jsonDecode(file.readAsStringSync());
    if (data is! Map ||
        data['name'] != plugin.packageName ||
        data['version'] is! String) {
      throw StateError('${plugin.name} 已安装包身份不符');
    }
    return data['version'] as String;
  }

  void _verifyInstalled(MarketPlugin plugin, {bool requireEnabled = true}) {
    if (_installedVersion(plugin) != plugin.version) {
      throw StateError(
        '${plugin.name} 安装命令返回成功，但 profile 中不是 ${plugin.version}',
      );
    }
    final manifest = File(p.join(profileDirectory.path, 'package.json'));
    final decoded = jsonDecode(manifest.readAsStringSync());
    if (decoded is! Map ||
        decoded['dependencies'] is! Map ||
        (decoded['dependencies'] as Map)[plugin.packageName] !=
            plugin.version) {
      throw StateError('${plugin.name} 未以精确版本写入 DSH profile');
    }
    final bundles =
        decoded['dsh'] is Map && (decoded['dsh'] as Map)['profile'] is Map
        ? ((decoded['dsh'] as Map)['profile'] as Map)['bundles']
        : null;
    if (bundles is! List ||
        (requireEnabled && !bundles.contains(plugin.packageName))) {
      throw StateError('${plugin.name} 未加入 DSH profile 组合');
    }
    final lock = File(p.join(profileDirectory.path, 'pnpm-lock.yaml'));
    if (!lock.existsSync()) {
      throw StateError('${plugin.name} 缺少 DSH lockfile');
    }
    final lockData = loadYaml(lock.readAsStringSync());
    final packages = lockData is YamlMap ? lockData['packages'] : null;
    final packageRecord = packages is YamlMap
        ? packages[plugin.exactPackage]
        : null;
    final resolution = packageRecord is YamlMap
        ? packageRecord['resolution']
        : null;
    if (resolution is! YamlMap || resolution['integrity'] != plugin.integrity) {
      throw StateError('${plugin.name} 的 lockfile 缺少市场固定的完整性摘要');
    }
  }

  Future<ProcessResult> _run(
    List<String> args, {
    Duration timeout = const Duration(seconds: 15),
  }) async {
    final executable = cliPath.endsWith('.js') ? nodeExecutable : cliPath;
    final arguments = cliPath.endsWith('.js') ? [cliPath, ...args] : args;
    return Process.run(
      executable,
      arguments,
      workingDirectory: p.dirname(cliPath),
      environment: _environment(),
    ).timeout(timeout);
  }

  Map<String, String> _environment() {
    final pathKey = Platform.isWindows ? 'Path' : 'PATH';
    final separator = Platform.isWindows ? ';' : ':';
    final paths = <String>[
      p.dirname(packageManagerExecutable),
      if (p.isAbsolute(nodeExecutable)) p.dirname(nodeExecutable),
      baseEnvironment[pathKey] ?? '',
    ];
    return {
      ...baseEnvironment,
      'DSH_HOME': dshHome.path,
      'CI': 'true',
      pathKey: paths.join(separator),
    };
  }
}

({String? cliPath, String nodeExecutable}) resolveMarketDshRuntime({
  required String executablePath,
  required Map<String, String> environment,
}) {
  final executableDir = p.dirname(executablePath);
  final bundleParent = p.dirname(executableDir);
  final cliTail = p.join(
    'openmuse',
    'dsh',
    'node_modules',
    '@deepseek-ai',
    'dsh',
    'lib',
    'bin.js',
  );
  final cliCandidates = [
    if (environment['OPENMUSE_DSH_CLI'] case final String explicit
        when explicit.isNotEmpty)
      explicit,
    p.join(bundleParent, 'Resources', cliTail),
    p.join(executableDir, cliTail),
  ];
  final nodeCandidates = [
    p.normalize(p.join(executableDir, '../../dsh/node/bin/node')),
    p.join(bundleParent, 'Resources', 'openmuse', 'dsh', 'node', 'bin', 'node'),
    p.join(executableDir, 'openmuse', 'dsh', 'node', 'node.exe'),
    p.join(executableDir, 'openmuse', 'dsh', 'node', 'bin', 'node'),
  ];
  return (
    cliPath: cliCandidates.where((path) => File(path).existsSync()).firstOrNull,
    nodeExecutable:
        nodeCandidates.where((path) => File(path).existsSync()).firstOrNull ??
        'node',
  );
}
