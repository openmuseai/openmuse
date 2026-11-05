import 'dart:convert';
import 'dart:io';

import 'package:appflowy/env/backend_env.dart';
import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/user/application/auth/device_id.dart';
import 'package:appflowy_backend/appflowy_backend.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';

import '../startup.dart';

class InitRustSDKTask extends LaunchTask {
  const InitRustSDKTask({
    this.customApplicationPath,
  });

  // Customize the RustSDK initialization path
  final Directory? customApplicationPath;

  @override
  LaunchTaskType get type => LaunchTaskType.dataProcessing;

  @override
  Future<void> initialize(LaunchContext context) async {
    await super.initialize(context);

    final root = await museApplicationSupportDirectory();
    final applicationPath = await appFlowyApplicationDataDirectory();
    final dir = customApplicationPath ?? applicationPath;
    final deviceId = await getDeviceId();

    // Pass the environment variables to the Rust SDK
    final env = _makeAppFlowyConfiguration(
      root.path,
      context.config.version,
      dir.path,
      applicationPath.path,
      deviceId,
      rustEnvs: context.config.rustEnvs,
    );
    await context.getIt<FlowySDK>().init(jsonEncode(env.toJson()));
  }
}

AppFlowyConfiguration _makeAppFlowyConfiguration(
  String root,
  String appVersion,
  String customAppPath,
  String originAppPath,
  String deviceId, {
  required Map<String, String> rustEnvs,
}) {
  final env = getIt<AppFlowyCloudSharedEnv>();
  return AppFlowyConfiguration(
    root: root,
    app_version: appVersion,
    custom_app_path: customAppPath,
    origin_app_path: originAppPath,
    device_id: deviceId,
    platform: Platform.operatingSystem,
    authenticator_type: env.authenticatorType.value,
    appflowy_cloud_config: env.appflowyCloudConfig,
    envs: rustEnvs,
  );
}

/// The default directory to store the user data. The directory can be
/// customized by the user via the [ApplicationDataStorage]
Future<Directory> appFlowyApplicationDataDirectory() async {
  switch (integrationMode()) {
    case IntegrationMode.develop:
      final Directory documentsDir = await museApplicationSupportDirectory()
          .then((directory) => directory.create());
      return Directory(path.join(documentsDir.path, 'data_dev'));
    case IntegrationMode.release:
      final Directory documentsDir = await museApplicationSupportDirectory();
      return Directory(path.join(documentsDir.path, 'data'));
    case IntegrationMode.unitTest:
    case IntegrationMode.integrationTest:
      return Directory(path.join(Directory.current.path, '.sandbox'));
  }
}

/// Desktop support dir follows the current product name. First launch copies
/// leftover AppFlowy / DSH Office / OpenMuse AI folders so libraries survive.
Future<Directory> museApplicationSupportDirectory() async {
  final current = await getApplicationSupportDirectory();
  if (Platform.isAndroid || Platform.isIOS) {
    return current;
  }
  await current.create(recursive: true);
  if (await _directoryHasEntries(current)) {
    return current;
  }
  for (final legacy in _legacyApplicationSupportPaths()) {
    final dir = Directory(legacy);
    if (!await _directoryHasEntries(dir)) continue;
    try {
      await dir.rename(current.path);
    } catch (_) {
      await _copyDirectory(dir, current);
    }
    break;
  }
  return current;
}

Future<bool> _directoryHasEntries(Directory dir) async {
  if (!await dir.exists()) return false;
  try {
    return dir.listSync(followLinks: false).isNotEmpty;
  } catch (_) {
    return false;
  }
}

List<String> _legacyApplicationSupportPaths() {
  final paths = <String>[];
  if (Platform.isWindows) {
    final appData = Platform.environment['APPDATA']?.trim();
    if (appData != null && appData.isNotEmpty) {
      paths.addAll([
        '$appData/openmuseai/Muse',
        '$appData/openmuseai/OpenMuse AI',
        '$appData/openmuseai/DSH Office',
        '$appData/AppFlowy/AppFlowy',
        '$appData/DSH Office/DSH Office',
      ]);
    }
  } else if (Platform.isLinux) {
    final home = Platform.environment['HOME']?.trim();
    final xdg = Platform.environment['XDG_DATA_HOME']?.trim();
    final base = (xdg != null && xdg.isNotEmpty)
        ? xdg
        : (home != null && home.isNotEmpty ? '$home/.local/share' : null);
    if (base != null) {
      paths.addAll([
        '$base/muse',
        '$base/dsh-office',
        '$base/appflowy',
        '$base/OpenMuse AI',
      ]);
    }
  } else {
    final home = Platform.environment['HOME']?.trim();
    if (home != null && home.isNotEmpty) {
      final support = '$home/Library/Application Support';
      paths.addAll([
        '$support/Muse',
        '$support/OpenMuse AI',
        '$support/DSH Office',
        '$support/AppFlowy',
      ]);
    }
  }
  return paths;
}

Future<void> _copyDirectory(Directory source, Directory dest) async {
  await dest.create(recursive: true);
  await for (final entity in source.list(recursive: true, followLinks: false)) {
    final relative = entity.path.substring(source.path.length);
    final targetPath = '${dest.path}$relative';
    if (entity is Directory) {
      await Directory(targetPath).create(recursive: true);
    } else if (entity is File) {
      await File(targetPath).parent.create(recursive: true);
      await entity.copy(targetPath);
    }
  }
}
