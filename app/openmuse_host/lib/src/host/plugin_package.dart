import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:openmuse_plugin_sdk/manifest.dart';
import 'package:path/path.dart' as p;

const distributedPackageLimit = 32 * 1024 * 1024;

final class DistributedPluginReceipt {
  const DistributedPluginReceipt({
    required this.pluginId,
    required this.name,
    required this.version,
    required this.workspacePath,
    required this.packageSha256,
  });

  factory DistributedPluginReceipt.fromJson(Map<dynamic, dynamic> json) {
    final pluginId = json['pluginId'];
    final version = json['version'];
    final workspacePath = json['workspacePath'];
    final packageSha256 = json['packageSha256'];
    if (pluginId is! String ||
        version is! String ||
        workspacePath is! String ||
        packageSha256 is! String) {
      throw const FormatException('无效插件安装回执');
    }
    final name = json['name'];
    return DistributedPluginReceipt(
      pluginId: pluginId,
      name: name is String && name.isNotEmpty ? name : pluginId,
      version: version,
      workspacePath: workspacePath,
      packageSha256: packageSha256,
    );
  }

  final String pluginId;
  final String name;
  final String version;
  final String workspacePath;
  final String packageSha256;

  Map<String, Object?> toJson() => {
    'pluginId': pluginId,
    'name': name,
    'version': version,
    'workspacePath': workspacePath,
    'packageSha256': packageSha256,
  };
}

/// Downloads a plugin package from a catalog, checks the catalog digest, and
/// runs the manifest's install steps. Downloaded bytes are never executed
/// as Dart. Mounting the workspace stays with the host.
Future<DistributedPluginReceipt> installDistributedPlugin({
  required Uri catalogUri,
  required String pluginId,
  required String workspacePath,
  required Directory installRoot,
  required OpenMuseTarget target,
  HttpClient? httpClient,
}) async {
  if (!p.isAbsolute(workspacePath)) {
    throw const FormatException('插件工作区必须是绝对路径');
  }
  if (catalogUri.scheme == 'file') {
    return _installLocalCatalog(
      catalogFile: File.fromUri(catalogUri),
      pluginId: pluginId,
      workspacePath: workspacePath,
      installRoot: installRoot,
      target: target,
    );
  }
  final client = httpClient ?? HttpClient();
  final ownsClient = httpClient == null;
  try {
    final catalogBytes = await _get(
      client,
      catalogUri,
      distributedPackageLimit,
    );
    final catalog = jsonDecode(utf8.decode(catalogBytes));
    if (catalog is! Map || catalog['packages'] is! List) {
      throw const FormatException('无效插件目录');
    }
    final entry = (catalog['packages'] as List)
        .cast<Object?>()
        .where((item) {
          return item is Map && item['id'] == pluginId;
        })
        .cast<Map>()
        .firstOrNull;
    if (entry == null) throw FormatException('目录中没有 $pluginId');
    final version = entry['version'];
    final url = entry['url'];
    final sha = entry['sha256'];
    final size = entry['size'];
    if (version is! String ||
        url is! String ||
        sha is! String ||
        size is! int) {
      throw const FormatException('无效插件目录项');
    }
    final packageUri = catalogUri.resolve(url);
    if (!_sameOrigin(catalogUri, packageUri)) {
      throw const FormatException('插件包必须和目录同一来源');
    }
    final packageBytes = await _get(client, packageUri, size);
    final actual = sha256.convert(packageBytes).toString();
    if (packageBytes.length != size || actual != sha) {
      throw const FormatException('插件包摘要不一致');
    }
    final receipt = await _installPackageBytes(
      bytes: packageBytes,
      pluginId: pluginId,
      version: version,
      workspacePath: workspacePath,
      installRoot: installRoot,
      target: target,
      packageSha256: actual,
    );
    return receipt;
  } finally {
    if (ownsClient) client.close(force: true);
  }
}

Future<DistributedPluginReceipt> _installLocalCatalog({
  required File catalogFile,
  required String pluginId,
  required String workspacePath,
  required Directory installRoot,
  required OpenMuseTarget target,
}) async {
  if (!await catalogFile.exists()) {
    throw FormatException('找不到插件目录：${catalogFile.path}');
  }
  final catalog = jsonDecode(await catalogFile.readAsString());
  if (catalog is! Map || catalog['packages'] is! List) {
    throw const FormatException('无效插件目录');
  }
  final entry = (catalog['packages'] as List)
      .cast<Object?>()
      .where((item) {
        return item is Map && item['id'] == pluginId;
      })
      .cast<Map>()
      .firstOrNull;
  if (entry == null) throw FormatException('目录中没有 $pluginId');
  final version = entry['version'];
  final url = entry['url'];
  final sha = entry['sha256'];
  final size = entry['size'];
  if (version is! String || url is! String || sha is! String || size is! int) {
    throw const FormatException('无效插件目录项');
  }
  if (url.contains('://') || p.isAbsolute(url)) {
    throw const FormatException('本地目录里的插件包必须是相对路径');
  }
  final packageFile = File(p.normalize(p.join(catalogFile.parent.path, url)));
  if (!p.isWithin(catalogFile.parent.path, packageFile.path)) {
    throw const FormatException('插件包路径越出目录');
  }
  final packageBytes = await packageFile.readAsBytes();
  final actual = sha256.convert(packageBytes).toString();
  if (packageBytes.length != size || actual != sha) {
    throw const FormatException('插件包摘要不一致');
  }
  return _installPackageBytes(
    bytes: packageBytes,
    pluginId: pluginId,
    version: version,
    workspacePath: workspacePath,
    installRoot: installRoot,
    target: target,
    packageSha256: actual,
  );
}

List<DistributedPluginReceipt> readInstalledPlugins(Directory installRoot) {
  if (!installRoot.existsSync()) return const [];
  final receipts = <DistributedPluginReceipt>[];
  for (final entity in installRoot.listSync()) {
    if (entity is! Directory) continue;
    final file = File(p.join(entity.path, 'receipt.json'));
    if (!file.existsSync()) continue;
    final decoded = jsonDecode(file.readAsStringSync());
    if (decoded is! Map) continue;
    receipts.add(DistributedPluginReceipt.fromJson(decoded));
  }
  receipts.sort((a, b) => a.name.compareTo(b.name));
  return receipts;
}

Future<void> uninstallDistributedPlugin({
  required Directory installRoot,
  required String pluginId,
}) async {
  final directory = Directory(p.join(installRoot.path, pluginId));
  if (await directory.exists()) await directory.delete(recursive: true);
}

String digestPayload(Map<String, List<int>> files) {
  final builder = BytesBuilder(copy: false);
  for (final path in files.keys.toList()..sort()) {
    builder.add(utf8.encode(path));
    builder.add(const [0]);
    builder.add(files[path]!);
    builder.add(const [0]);
  }
  return sha256.convert(builder.takeBytes()).toString();
}

Future<DistributedPluginReceipt> _installPackageBytes({
  required List<int> bytes,
  required String pluginId,
  required String version,
  required String workspacePath,
  required Directory installRoot,
  required OpenMuseTarget target,
  required String packageSha256,
}) async {
  final archive = ZipDecoder().decodeBytes(bytes);
  final files = <String, List<int>>{};
  for (final entry in archive) {
    final name = entry.name.replaceAll('\\', '/');
    if (name.startsWith('/') || name.split('/').contains('..')) {
      throw FormatException('插件包路径越界：$name');
    }
    if (!entry.isFile) continue;
    files[name] = _fileBytes(entry);
  }
  final manifestBytes = files['openmuse.plugin.json'];
  if (manifestBytes == null) throw const FormatException('插件包缺少清单');
  final manifest = OpenMusePluginManifestV2.fromJson(
    jsonDecode(utf8.decode(manifestBytes)),
  );
  if (manifest.id != pluginId || manifest.version != version) {
    throw const FormatException('插件包和目录不一致');
  }
  final decision = manifest.compatibility.where((item) {
    final itemTarget = item.target;
    return itemTarget.os == target.os &&
        itemTarget.arch == target.arch &&
        itemTarget.libc == target.libc;
  }).firstOrNull;
  if (decision?.status != OpenMuseTargetStatus.supported) {
    throw FormatException('插件不支持当前平台 $target');
  }
  final install = manifest.install;
  if (install == null) throw const FormatException('插件没有安装步骤');
  for (final artifact in manifest.artifacts) {
    final payload = _payload(files, artifact.id);
    final actual = digestPayload(payload);
    if (actual != artifact.digest.value) {
      throw FormatException('artifact ${artifact.id} 摘要不一致');
    }
  }
  final stored = File(p.join(installRoot.path, pluginId, 'receipt.json'));
  if (stored.existsSync()) {
    final previous = jsonDecode(stored.readAsStringSync());
    if (previous is Map &&
        previous['version'] == version &&
        previous['workspacePath'] == workspacePath &&
        previous['packageSha256'] == packageSha256) {
      return DistributedPluginReceipt(
        pluginId: pluginId,
        name: previous['name'] is String
            ? previous['name'] as String
            : pluginId,
        version: version,
        workspacePath: workspacePath,
        packageSha256: packageSha256,
      );
    }
    if (previous is Map &&
        previous['version'] == version &&
        previous['workspacePath'] != workspacePath) {
      throw const FormatException('同一版本不能改挂到另一个工作区');
    }
  }
  final workspace = Directory(workspacePath);
  workspace.createSync(recursive: true);
  final packageDir = Directory(p.join(installRoot.path, pluginId, 'payload'));
  packageDir.createSync(recursive: true);
  for (final artifact in manifest.artifacts) {
    final payload = _payload(files, artifact.id);
    for (final entry in payload.entries) {
      final destination = _within(
        Directory(p.join(packageDir.path, artifact.id)),
        entry.key,
      );
      destination.parent.createSync(recursive: true);
      destination.writeAsBytesSync(entry.value, flush: true);
    }
  }
  for (final step in install.steps) {
    switch (step.type) {
      case OpenMuseInstallStepType.workspaceChoose:
        continue;
      case OpenMuseInstallStepType.workspaceLayout:
        final directories = step.fields['directories'];
        if (directories is! List) {
          throw const FormatException('无效安装步骤');
        }
        for (final name in directories.whereType<String>()) {
          Directory(p.join(workspace.path, name)).createSync(recursive: true);
        }
      case OpenMuseInstallStepType.workspaceMaterialize:
      case OpenMuseInstallStepType.runtimePrepare:
        final artifact = step.fields['artifact'];
        final into = step.fields['into'];
        if (artifact is! String || into is! String) {
          throw const FormatException('无效安装步骤');
        }
        final payload = _payload(files, artifact);
        final destinationRoot = Directory(p.join(workspace.path, into));
        for (final entry in payload.entries) {
          final destination = _within(destinationRoot, entry.key);
          destination.parent.createSync(recursive: true);
          destination.writeAsBytesSync(entry.value, flush: true);
        }
        if (step.type == OpenMuseInstallStepType.runtimePrepare &&
            step.fields['entrypoint'] is String) {
          final relative = step.fields['entrypoint'] as String;
          final script = _within(destinationRoot, relative);
          if (!script.existsSync() || !payload.containsKey(relative)) {
            throw FormatException('安装引导入口不存在：$relative');
          }
          final python = await _installPython();
          final result = await Process.start(
            python,
            [script.path],
            workingDirectory: workspace.path,
            environment: {
              ...Platform.environment,
              'HOME': workspace.path,
              'USERPROFILE': workspace.path,
              'XDG_CACHE_HOME': p.join(workspace.path, 'cache'),
              'PYTHONPYCACHEPREFIX': p.join(workspace.path, 'cache', 'python'),
              'OPENMUSE_PLUGIN_WORKSPACE': workspace.path,
              'OPENMUSE_PLUGIN_ID': pluginId,
              'OPENMUSE_PLUGIN_ARTIFACT_ROOT': destinationRoot.path,
            },
            runInShell: false,
          );
          await Future.wait([
            result.stdout.forEach(stdout.add),
            result.stderr.forEach(stderr.add),
          ]);
          final code = await result.exitCode;
          if (code != 0) {
            throw ProcessException(python, [script.path], '安装引导失败', code);
          }
        }
    }
  }
  final receipt = DistributedPluginReceipt(
    pluginId: pluginId,
    name: manifest.name,
    version: version,
    workspacePath: workspace.path,
    packageSha256: packageSha256,
  );
  stored.parent.createSync(recursive: true);
  File(
    p.join(stored.parent.path, 'package.omplugin'),
  ).writeAsBytesSync(bytes, flush: true);
  stored.writeAsStringSync(jsonEncode(receipt.toJson()), flush: true);
  return receipt;
}

Future<String> _installPython() async {
  final preferred = Platform.environment['OPENMUSE_PYTHON'];
  final candidates = preferred == null
      ? ['python3.13', 'python3.12', 'python3.11', 'python3.10', 'python3']
      : [preferred];
  for (final candidate in candidates) {
    try {
      final result = await Process.run(candidate, [
        '-c',
        'import sys; print(sys.version_info >= (3, 10))',
      ]);
      if (result.exitCode == 0 && '${result.stdout}'.trim() == 'True') {
        return candidate;
      }
    } on ProcessException {
      // Try another installed interpreter.
    }
  }
  throw const FormatException('插件安装引导需要 Python >= 3.10');
}

Map<String, List<int>> _payload(
  Map<String, List<int>> files,
  String artifactId,
) {
  final prefix = 'payload/$artifactId/';
  final payload = <String, List<int>>{};
  for (final entry in files.entries) {
    if (!entry.key.startsWith(prefix)) continue;
    final relative = entry.key.substring(prefix.length);
    if (relative.isEmpty) continue;
    payload[relative] = entry.value;
  }
  if (payload.isEmpty) throw FormatException('缺少 artifact $artifactId');
  return payload;
}

List<int> _fileBytes(ArchiveFile entry) {
  final content = entry.content;
  if (content is Uint8List) return content;
  if (content is List<int>) return content;
  throw FormatException('无法读取 ${entry.name}');
}

File _within(Directory root, String relative) {
  final normalized = p.normalize(p.join(root.path, relative));
  if (!p.isWithin(root.path, normalized)) {
    throw FormatException('安装路径越界：$relative');
  }
  return File(normalized);
}

Future<List<int>> _get(HttpClient client, Uri uri, int maxBytes) async {
  if (!_allowed(uri)) throw FormatException('拒绝的插件地址：$uri');
  if (maxBytes <= 0 || maxBytes > distributedPackageLimit) {
    throw const FormatException('插件包尺寸无效');
  }
  final request = await client.getUrl(uri).timeout(const Duration(seconds: 10));
  request.followRedirects = false;
  final response = await request.close().timeout(const Duration(seconds: 20));
  if (response.statusCode != HttpStatus.ok) {
    await response.drain<void>();
    throw FormatException('下载失败：HTTP ${response.statusCode}');
  }
  final builder = BytesBuilder(copy: false);
  await for (final chunk in response) {
    if (builder.length + chunk.length > maxBytes) {
      throw const FormatException('插件包超过目录声明的大小');
    }
    builder.add(chunk);
  }
  return builder.takeBytes();
}

bool _allowed(Uri uri) {
  if (uri.scheme == 'https' && uri.host.isNotEmpty) return true;
  if (uri.scheme == 'http' &&
      (uri.host == '127.0.0.1' || uri.host == 'localhost')) {
    return true;
  }
  return false;
}

bool _sameOrigin(Uri catalog, Uri package) =>
    catalog.scheme == package.scheme &&
    catalog.host == package.host &&
    catalog.port == package.port;

OpenMuseTarget currentDesktopPluginTarget() {
  final abi = Abi.current();
  return switch (abi) {
    Abi.macosArm64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.macos,
      arch: OpenMuseTargetArch.aarch64,
      libc: OpenMuseTargetLibc.darwin,
    ),
    Abi.macosX64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.macos,
      arch: OpenMuseTargetArch.x86_64,
      libc: OpenMuseTargetLibc.darwin,
    ),
    Abi.linuxArm64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.linux,
      arch: OpenMuseTargetArch.aarch64,
      libc: OpenMuseTargetLibc.gnu,
    ),
    Abi.linuxX64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.linux,
      arch: OpenMuseTargetArch.x86_64,
      libc: OpenMuseTargetLibc.gnu,
    ),
    Abi.windowsArm64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.windows,
      arch: OpenMuseTargetArch.aarch64,
      libc: OpenMuseTargetLibc.msvc,
    ),
    Abi.windowsX64 => const OpenMuseTarget(
      os: OpenMuseTargetOs.windows,
      arch: OpenMuseTargetArch.x86_64,
      libc: OpenMuseTargetLibc.msvc,
    ),
    _ => throw UnsupportedError('不支持的 Desktop 插件平台：$abi'),
  };
}
