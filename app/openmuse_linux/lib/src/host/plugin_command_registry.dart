import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:openmuse_plugin_sdk/manifest.dart';
import 'package:path/path.dart' as p;

import 'plugin_package.dart';

final class InstalledCliCommand {
  const InstalledCliCommand({
    required this.receipt,
    required this.contribution,
    required this.artifact,
    required this.installRoot,
  });

  final DistributedPluginReceipt receipt;
  final OpenMuseCliContributionV2 contribution;
  final OpenMusePluginArtifactV2 artifact;
  final Directory installRoot;

  String get identity => contribution.identity;

  /// Validate the materialized artifact every time before executing it.
  File verifiedEntrypoint() {
    final root = Directory(
      p.join(installRoot.path, receipt.pluginId, 'payload', artifact.id),
    );
    final files = <String, List<int>>{};
    if (!root.existsSync()) {
      throw FormatException('缺少 CLI artifact ${artifact.id}');
    }
    for (final entity in root.listSync(recursive: true, followLinks: false)) {
      if (entity is Link) throw const FormatException('CLI artifact 不允许符号链接');
      if (entity is File) {
        final relative = p
            .relative(entity.path, from: root.path)
            .replaceAll('\\', '/');
        if (relative.split('/').contains('__pycache__')) continue;
        files[relative] = entity.readAsBytesSync();
      }
    }
    if (digestPayload(files) != artifact.digest.value) {
      throw FormatException('CLI artifact ${artifact.id} 摘要不一致');
    }
    final script = File(p.join(root.path, contribution.entrypoint));
    if (!script.existsSync()) throw const FormatException('CLI 入口文件不存在');
    return script;
  }

  List<String> argv(List<String> args) {
    final allowed = contribution.options.toSet();
    final result = <String>[...contribution.argvPrefix];
    for (var index = 0; index < args.length;) {
      final flag = args[index];
      if (contribution.switches.contains(flag)) {
        result.add(flag);
        index++;
        continue;
      }
      if (!allowed.contains(flag) ||
          index + 1 >= args.length ||
          args[index + 1].startsWith('--')) {
        throw FormatException('命令 ${contribution.identity} 不支持参数 $flag');
      }
      result.addAll([flag, args[index + 1]]);
      index += 2;
    }
    return result;
  }
}

final class CliCommandDiscovery {
  const CliCommandDiscovery({required this.commands, required this.errors});

  final Map<String, InstalledCliCommand> commands;
  final Map<String, String> errors;
}

CliCommandDiscovery discoverInstalledCliCommands(Directory installRoot) {
  final commands = <String, InstalledCliCommand>{};
  final errors = <String, String>{};
  final conflicts = <String, Set<String>>{};
  final settingsFile = File(
    p.join(installRoot.parent.path, 'settings-v1.json'),
  );
  Map? pluginSettings;
  if (settingsFile.existsSync()) {
    try {
      final decoded = jsonDecode(settingsFile.readAsStringSync());
      if (decoded is Map && decoded['plugins'] is Map) {
        pluginSettings = decoded['plugins'] as Map;
      }
    } on FormatException {
      // A damaged settings file cannot grant commands.
      return CliCommandDiscovery(
        commands: commands,
        errors: {'settings': '设置文件格式无效'},
      );
    }
  }
  for (final receipt in readInstalledPlugins(installRoot)) {
    try {
      final values = pluginSettings?[receipt.pluginId];
      if (values is Map && values['workspaceEnabled'] == false) continue;
      final packageFile = File(
        p.join(installRoot.path, receipt.pluginId, 'package.omplugin'),
      );
      final bytes = packageFile.readAsBytesSync();
      if (sha256.convert(bytes).toString() != receipt.packageSha256) {
        throw const FormatException('已安装插件包摘要不一致');
      }
      final archive = ZipDecoder().decodeBytes(bytes);
      final manifestEntry = archive.findFile('openmuse.plugin.json');
      if (manifestEntry == null) throw const FormatException('插件包缺少清单');
      final manifest = OpenMusePluginManifestV2.fromJson(
        jsonDecode(utf8.decode(manifestEntry.content as List<int>)),
      );
      if (manifest.id != receipt.pluginId ||
          manifest.version != receipt.version) {
        throw const FormatException('安装回执与插件清单不一致');
      }
      for (final contribution in manifest.contributes.cli) {
        final artifact = manifest.artifacts.singleWhere(
          (item) => item.id == contribution.artifact,
        );
        if (artifact.target != currentDesktopPluginTarget()) continue;
        final identity = contribution.identity;
        if (conflicts.containsKey(identity)) {
          conflicts[identity]!.add(receipt.pluginId);
          continue;
        }
        final previous = commands.remove(identity);
        if (previous != null) {
          conflicts[identity] = {previous.receipt.pluginId, receipt.pluginId};
          continue;
        }
        commands[identity] = InstalledCliCommand(
          receipt: receipt,
          contribution: contribution,
          artifact: artifact,
          installRoot: installRoot,
        );
      }
    } on Object catch (error) {
      errors[receipt.pluginId] = error.toString();
      commands.removeWhere(
        (_, command) => command.receipt.pluginId == receipt.pluginId,
      );
    }
  }
  for (final entry in conflicts.entries) {
    for (final pluginId in entry.value) {
      errors[pluginId] = 'CLI 命令冲突：${entry.key}';
    }
  }
  return CliCommandDiscovery(commands: commands, errors: errors);
}

Future<int> invokeInstalledCliCommand(
  InstalledCliCommand command,
  List<String> args, {
  String? python,
  IOSink? output,
  IOSink? errors,
  FutureOr<void> Function(List<int>)? onStdout,
  FutureOr<void> Function(List<int>)? onStderr,
}) async {
  final script = command.verifiedEntrypoint();
  final argv = command.argv(args);
  final venv = File(
    p.join(
      command.receipt.workspacePath,
      'runtime',
      'venv',
      Platform.isWindows ? 'Scripts/python.exe' : 'bin/python',
    ),
  );
  final executable = python != null
      ? await _resolvePython(python)
      : venv.existsSync()
      ? venv.path
      : await _resolvePython(Platform.environment['OPENMUSE_PYTHON']);
  final process = await Process.start(
    executable,
    [script.path, ...argv],
    workingDirectory: command.receipt.workspacePath,
    environment: {
      ...Platform.environment,
      'HOME': command.receipt.workspacePath,
      'USERPROFILE': command.receipt.workspacePath,
      'XDG_CACHE_HOME': p.join(command.receipt.workspacePath, 'cache'),
      'PYTHONPYCACHEPREFIX': p.join(
        command.receipt.workspacePath,
        'cache',
        'openmuse-pycache',
      ),
      // Generic plugin interaction inbox. The plugin owns the content;
      // desktop/mobile surfaces only render the declared protocol.
      'OPENMUSE_PLUGIN_INTERACTION_DIR': p.join(
        command.receipt.workspacePath,
        'state',
        'interactions',
      ),
      'OPENMUSE_PLUGIN_ID': command.receipt.pluginId,
      'OPENMUSE_PLUGIN_WORKSPACE': command.receipt.workspacePath,
    },
    runInShell: false,
  );
  await Future.wait([
    process.stdout.forEach((bytes) async {
      if (onStdout != null) {
        await onStdout(bytes);
      } else {
        (output ?? stdout).add(bytes);
      }
    }),
    process.stderr.forEach((bytes) async {
      if (onStderr != null) {
        await onStderr(bytes);
      } else {
        (errors ?? stderr).add(bytes);
      }
    }),
  ]);
  return process.exitCode;
}

Future<String> _resolvePython(String? selected) async {
  final candidates = selected == null
      ? ['python3', 'python3.13', 'python3.12', 'python3.11', 'python3.10']
      : [selected];
  for (final candidate in candidates) {
    try {
      final version = await Process.run(candidate, [
        '-c',
        'import sys; print(sys.version_info[:2] >= (3, 10))',
      ]);
      if (version.exitCode == 0 &&
          (version.stdout as String).trim() == 'True') {
        return candidate;
      }
    } on ProcessException {
      // Keep looking for another supported interpreter on PATH.
    }
  }
  throw FormatException('CLI 需要 Python >= 3.10：${candidates.join(', ')}');
}
