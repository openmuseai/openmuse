import 'dart:io';

import 'package:openmuse_plugin_sdk/manifest.dart';

import 'local_settings.dart';
import 'plugin_package.dart';
import 'workspace_controller.dart';

export 'plugin_package.dart';

/// Installs a package, then mounts the workspace the caller chose and records
/// that path in host settings.
Future<DistributedPluginReceipt> acceptDistributedPlugin({
  required Uri catalogUri,
  required String pluginId,
  required String workspacePath,
  required Directory installRoot,
  required OpenMuseTarget target,
  required LocalWorkspaceController workspace,
  required OpenMuseLocalSettings settings,
  HttpClient? httpClient,
}) async {
  final receipt = await installDistributedPlugin(
    catalogUri: catalogUri,
    pluginId: pluginId,
    workspacePath: workspacePath,
    installRoot: installRoot,
    target: target,
    httpClient: httpClient,
  );
  final mounted = await workspace.ensurePluginWorkspace(
    pluginId: receipt.pluginId,
    path: receipt.workspacePath,
  );
  final current = settings.pluginValues(receipt.pluginId);
  await settings.updatePluginValues(receipt.pluginId, {
    ...current,
    'workspaceEnabled': true,
    'workspacePath': mounted,
  });
  return DistributedPluginReceipt(
    pluginId: receipt.pluginId,
    name: receipt.name,
    version: receipt.version,
    workspacePath: mounted,
    packageSha256: receipt.packageSha256,
  );
}
