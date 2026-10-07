import 'dart:convert';
import 'dart:io';

import 'package:path/path.dart' as p;

import 'plugin_command_registry.dart';
import 'plugin_cli_broker.dart';
import 'plugin_package.dart';
import 'workspace_mount_store.dart';

const openMuseDataDirectoryName = 'com.openmuseai.office/OpenMuse';

/// Human CLI. Plugin command dispatch is resolved from verified installed
/// packages and does not require a Flutter application window.
Future<int> runOpenMuseCli(List<String> args) async {
  try {
    if (args.isEmpty || args.first == 'help' || args.first == '--help') {
      stdout.writeln(_openMuseUsage);
      return args.isEmpty ? 64 : 0;
    }
    final dataDir = Directory(
      _optional(args, '--data-dir') ?? _defaultDataDir(),
    );
    final clean = <String>[];
    for (var index = 0; index < args.length; index++) {
      if (args[index] == '--data-dir') {
        if (++index >= args.length) {
          throw const FormatException('缺少 --data-dir');
        }
      } else {
        clean.add(args[index]);
      }
    }
    if (clean.isEmpty) throw const FormatException('缺少命令');
    if (clean.first == 'list') clean.replaceRange(0, 1, ['plugin', 'list']);
    if (clean.first == 'commands') {
      clean.replaceRange(0, 1, ['plugin', 'commands']);
    }
    if (clean.first == 'plugin') {
      if (clean.length < 2) throw const FormatException('缺少 plugin 子命令');
      switch (clean[1]) {
        case 'install':
          await _install([...clean.sublist(2), '--data-dir', dataDir.path]);
          return 0;
        case 'list':
          stdout.writeln('com.openmuse.cli\t0.1.0\tbuilt-in');
          for (final receipt in readInstalledPlugins(
            Directory(p.join(dataDir.path, 'plugins')),
          )) {
            stdout.writeln(
              '${receipt.pluginId}\t${receipt.version}\t${receipt.workspacePath}',
            );
          }
          return 0;
        case 'commands':
          final discovery = discoverInstalledCliCommands(
            Directory(p.join(dataDir.path, 'plugins')),
          );
          if (clean.contains('--json')) {
            stdout.writeln(
              jsonEncode({
                'protocol': 'openmuse.cli-discovery/v1',
                'commands': [
                  for (final entry in discovery.commands.entries)
                    {
                      'identity': entry.key,
                      'pluginId': entry.value.receipt.pluginId,
                      'description': entry.value.contribution.description,
                      'options': entry.value.contribution.options,
                      'switches': entry.value.contribution.switches,
                      'effects': entry.value.contribution.effects.toList(),
                      'inputSchema': entry.value.contribution.inputSchema,
                      'outputSchema': entry.value.contribution.outputSchema,
                      'invocation':
                          'openmuse ${entry.key.replaceAll('/', ' ')}',
                      'io': {
                        'stdout': 'utf8 stream',
                        'stderr': 'utf8 stream',
                        'exitCode': 'integer',
                      },
                    },
                ],
                'errors': discovery.errors,
              }),
            );
            return discovery.errors.isEmpty ? 0 : 1;
          }
          for (final entry in discovery.commands.entries) {
            stdout.writeln('${entry.key}\t${entry.value.receipt.pluginId}');
          }
          for (final entry in discovery.errors.entries) {
            stderr.writeln('插件 ${entry.key} 不可用：${entry.value}');
          }
          return discovery.errors.isEmpty ? 0 : 1;
        case 'uninstall':
          await _uninstall([...clean.sublist(2), '--data-dir', dataDir.path]);
          return 0;
        default:
          throw FormatException('未知 plugin 子命令：${clean[1]}');
      }
    }
    if (clean.length < 3) {
      throw const FormatException('命令地址需要 group namespace command');
    }
    final identity = clean.take(3).join('/');
    final discovery = discoverInstalledCliCommands(
      Directory(p.join(dataDir.path, 'plugins')),
    );
    final command = discovery.commands[identity];
    if (command == null) {
      throw FormatException('命令未安装或不可用：$identity ${discovery.errors}');
    }
    final brokerUrl = Platform.environment['OPENMUSE_CLI_BROKER_URL'] ??
        Platform.environment['DSH_OPENMUSE_CLI_BROKER_URL'];
    final brokerToken = Platform.environment['OPENMUSE_CLI_BROKER_TOKEN'] ??
        Platform.environment['DSH_OPENMUSE_CLI_BROKER_TOKEN'];
    if (brokerUrl != null && brokerToken != null) {
      return invokeBrokeredCliCommand(
        origin: Uri.parse(brokerUrl),
        token: brokerToken,
        identity: identity,
        args: clean.sublist(3),
      );
    }
    return await invokeInstalledCliCommand(command, clean.sublist(3));
  } on Object catch (error) {
    stderr.writeln('OpenMuse CLI 失败：$error');
    return 1;
  }
}

const _openMuseUsage = '''
用法:
  openmuse plugin install --catalog <catalog.json> --workspace <绝对路径> --plugin <id> [--data-dir <目录>]
  openmuse plugin uninstall --plugin <id> [--data-dir <目录>]
  openmuse plugin list | commands [--data-dir <目录>]
  openmuse list | commands [--data-dir <目录>]
  openmuse <group> <namespace> <command> [命令参数] [--data-dir <目录>]
示例: openmuse plugin commands --json
''';

/// Legacy entry point for generic plugin installation.
Future<int> runPluginCli(List<String> args) async {
  try {
    if (args.isEmpty || args.first == 'help' || args.first == '--help') {
      stdout.writeln(_usage);
      return args.isEmpty ? 64 : 0;
    }
    switch (args.first) {
      case 'install':
        await _install(args.sublist(1));
        return 0;
      default:
        stderr.writeln('未知命令：${args.first}\n$_usage');
        return 64;
    }
  } on Object catch (error) {
    stderr.writeln('插件命令失败：$error');
    return 1;
  }
}

const _usage = '''
用法:
  openmuse_plugin install --catalog <catalog.json 或 http(s) URL> --workspace <工作区目录> --plugin <id> [--data-dir <OpenMuse 数据目录>]
''';

Future<void> _install(List<String> args) async {
  final catalog = _required(args, '--catalog');
  final workspace = _required(args, '--workspace');
  final pluginId = _required(args, '--plugin');
  final dataDir = Directory(_optional(args, '--data-dir') ?? _defaultDataDir());
  final catalogUri = catalog.contains('://')
      ? Uri.parse(catalog)
      : Uri.file(p.absolute(catalog));
  Directory(p.join(dataDir.path, 'Workspace')).createSync(recursive: true);
  final receipt = await installDistributedPlugin(
    catalogUri: catalogUri,
    pluginId: pluginId,
    workspacePath: p.absolute(workspace),
    installRoot: Directory(p.join(dataDir.path, 'plugins')),
    target: currentDesktopPluginTarget(),
  );
  final mounted = await _rememberWorkspaceMount(
    pluginId: receipt.pluginId,
    workspacePath: receipt.workspacePath,
    mountsFile: File(p.join(dataDir.path, 'workspace-mounts-v1.json')),
  );
  await _rememberPluginSettings(
    settingsFile: File(p.join(dataDir.path, 'settings-v1.json')),
    pluginId: receipt.pluginId,
    workspacePath: mounted,
  );
  stdout.writeln('已安装 ${receipt.name} ${receipt.version}，工作区 $mounted');
}

Future<void> _uninstall(List<String> args) async {
  final pluginId = _required(args, '--plugin');
  if (!_pluginIdPattern.hasMatch(pluginId)) {
    throw FormatException('无效插件 ID：$pluginId');
  }
  final dataDir = Directory(_optional(args, '--data-dir') ?? _defaultDataDir());
  await uninstallDistributedPlugin(
    installRoot: Directory(p.join(dataDir.path, 'plugins')),
    pluginId: pluginId,
  );
  final settingsFile = File(p.join(dataDir.path, 'settings-v1.json'));
  if (await settingsFile.exists()) {
    final decoded = jsonDecode(await settingsFile.readAsString());
    if (decoded is Map && decoded['plugins'] is Map) {
      final data = Map<String, Object?>.from(decoded);
      final plugins = Map<String, Object?>.from(decoded['plugins'] as Map);
      plugins.remove(pluginId);
      data['plugins'] = plugins;
      await settingsFile.writeAsString(jsonEncode(data), flush: true);
    }
  }
  stdout.writeln('已卸载 $pluginId');
}

final _pluginIdPattern = RegExp(
  r'^com\.openmuse\.[A-Za-z0-9][A-Za-z0-9._-]{0,100}$',
);

Future<String> _rememberWorkspaceMount({
  required String pluginId,
  required String workspacePath,
  required File mountsFile,
}) async {
  if (!_pluginIdPattern.hasMatch(pluginId)) {
    throw FormatException('无效插件 ID', pluginId);
  }
  final directory = Directory(workspacePath);
  if (!await directory.exists()) {
    throw FileSystemException('插件工作区不存在', workspacePath);
  }
  final canonical = await directory.resolveSymbolicLinks();
  final store = WorkspaceMountStore(mountsFile);
  final mounts = await store.load();
  if (!mounts.contains(canonical)) {
    await store.save([...mounts, canonical]);
  }
  return canonical;
}

Future<void> _rememberPluginSettings({
  required File settingsFile,
  required String pluginId,
  required String workspacePath,
}) async {
  final data = <String, Object?>{
    'version': 1,
    'themeMode': 'system',
    'assistantVisible': true,
    'defaultEditors': <String, Object?>{},
    'plugins': <String, Object?>{},
  };
  if (await settingsFile.exists()) {
    final decoded = jsonDecode(await settingsFile.readAsString());
    if (decoded is Map && decoded['version'] == 1) {
      data
        ..clear()
        ..addAll(Map<String, Object?>.from(decoded));
    }
  }
  final plugins = data['plugins'] is Map
      ? Map<String, Object?>.from(data['plugins'] as Map)
      : <String, Object?>{};
  final current = plugins[pluginId] is Map
      ? Map<String, Object?>.from(plugins[pluginId] as Map)
      : <String, Object?>{};
  current['workspaceEnabled'] = true;
  current['workspacePath'] = workspacePath;
  plugins[pluginId] = current;
  data['version'] = 1;
  data['plugins'] = plugins;
  await settingsFile.parent.create(recursive: true);
  final temporary = File(
    '${settingsFile.path}.tmp.$pid.${DateTime.now().microsecondsSinceEpoch}',
  );
  await temporary.writeAsString(jsonEncode(data), flush: true);
  try {
    await temporary.rename(settingsFile.path);
  } on FileSystemException {
    if (!Platform.isWindows || !await settingsFile.exists()) rethrow;
    await settingsFile.delete();
    await temporary.rename(settingsFile.path);
  }
}

String _defaultDataDir() {
  if (Platform.isWindows) {
    final appData = Platform.environment['APPDATA'];
    if (appData == null || appData.isEmpty) {
      throw const FormatException('无法定位 OpenMuse 数据目录，请传入 --data-dir');
    }
    return p.join(appData, 'OpenMuse');
  }
  final userHome = Platform.environment['HOME'];
  if (userHome == null || userHome.isEmpty) {
    throw const FormatException('无法定位 OpenMuse 数据目录，请传入 --data-dir');
  }
  if (Platform.isLinux) {
    return p.join(
      Platform.environment['XDG_DATA_HOME'] ??
          p.join(userHome, '.local', 'share'),
      'OpenMuse',
    );
  }
  return p.join(
    userHome,
    'Library',
    'Application Support',
    openMuseDataDirectoryName,
  );
}

String _required(List<String> args, String name) {
  final value = _optional(args, name);
  if (value == null || value.isEmpty) {
    throw FormatException('缺少 $name');
  }
  return value;
}

String? _optional(List<String> args, String name) {
  final index = args.indexOf(name);
  if (index < 0 || index + 1 >= args.length) return null;
  return args[index + 1];
}
