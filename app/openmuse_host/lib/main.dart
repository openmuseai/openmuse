import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import 'src/host/design_system.dart';
import 'src/host/layout/layout.dart';
import 'src/host/layout/surface_mutation_guard.dart';
import 'src/host/local_settings.dart';
import 'src/host/openmuse_app.dart';
import 'src/host/workspace_controller.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Paint immediately. Waiting on workspace/plugin boot before runApp leaves
  // Finder-launched windows blank while Dart is still in main().
  runApp(const OpenMuseLaunchApp());
}

Future<Widget> bootOpenMuseHost() async {
  final support = await getApplicationSupportDirectory();
  final settings = OpenMuseLocalSettings(
    file: File(p.join(support.path, 'OpenMuse', 'settings-v1.json')),
  );
  await settings.load();
  final layoutStore = LayoutStore(
    File(p.join(support.path, 'OpenMuse', 'layout-v1.json')),
    fallback: _layoutFallback(settings),
  );
  final layoutController = WorkbenchLayoutController(await layoutStore.load());
  final mountStore = WorkspaceMountStore(
    File(p.join(support.path, 'OpenMuse', 'workspace-mounts-v1.json')),
  );
  final savedMounts = await mountStore.load();
  // Application Support is always accessible at launch. External project
  // mounts are restored separately and scanned after the first frame, since
  // macOS Documents/iCloud/FileProvider may block directory enumeration.
  final rootPath = p.join(support.path, 'OpenMuse', 'Workspace');
  await Directory(rootPath).create(recursive: true);
  final controller = LocalWorkspaceController(
    rootPath: rootPath,
    initialResources: const [],
    versionStore: LocalVersionStore(
      Directory(p.join(support.path, 'OpenMuse', 'versions-v1')),
    ),
    mountStore: mountStore,
    additionalMountPaths: savedMounts,
  );
  unawaited(
    controller.initialize().catchError((Object error) {
      debugPrint('Workspace scan deferred/failed: $error');
    }),
  );
  final registry = OpenMusePluginRegistry(
    context: OpenMusePluginContext(
      hostChanges: controller,
      executeHostCommand: (command, arguments) async {
        switch (command) {
          case 'workspace.snapshot':
            return {
              'workspaceRef': 'openmuse.local.default',
              'title': 'Project Workspace',
              'activeMountPath': controller.activeMountPath,
              'dshHome': _dshHome(support.path),
              'mounts': [
                for (final mount in controller.mounts)
                  {'path': mount.path, 'name': mount.name},
              ],
            };
          case 'workspace.openResource':
            if (arguments is! Map) {
              throw const FormatException('无效文件打开请求');
            }
            final path = arguments['path'];
            final cwd = arguments['cwd'];
            final editorId = arguments['editorId'];
            if (path is! String ||
                (cwd != null && cwd is! String) ||
                (editorId != null && editorId is! String)) {
              throw const FormatException('无效文件路径');
            }
            final resource = await controller.openHostResource(
              requestedPath: path,
              cwd: cwd as String?,
              editorId: editorId as String?,
            );
            return {'uri': resource.uri.toString()};
          case 'workspace.activateMount':
            if (arguments is! Map || arguments['path'] is! String) {
              throw const FormatException('无效工作区切换请求');
            }
            final path = arguments['path'] as String;
            final matches = controller.mounts.where(
              (mount) => mount.path == path,
            );
            if (matches.isEmpty) {
              throw const FormatException('DSH 工作区未获 Host 授权');
            }
            controller.activateMount(matches.single);
            return {'activeMountPath': controller.activeMountPath};
          case 'settings.plugin.read':
            if (arguments is! String) throw const FormatException('无效插件 ID');
            return settings.pluginValues(arguments);
          case 'settings.plugin.write':
            if (arguments is! Map ||
                arguments['pluginId'] is! String ||
                arguments['values'] is! Map) {
              throw const FormatException('无效插件设置');
            }
            await settings.updatePluginValues(
              arguments['pluginId'] as String,
              Map<String, Object?>.from(arguments['values'] as Map),
            );
            return null;
          default:
            throw UnsupportedError('未知 Host 命令：$command');
        }
      },
    ),
  );
  final gotrueOrigin = Uri.parse(
    const String.fromEnvironment(
      'OPENMUSE_GOTRUE_ORIGIN',
      defaultValue: 'http://127.0.0.1:9999',
    ),
  );
  final cloudOrigin = Uri.parse(
    const String.fromEnvironment(
      'OPENMUSE_CLOUD_ORIGIN',
      defaultValue: 'http://127.0.0.1:8000',
    ),
  );
  final allowInsecureLoopback = const bool.fromEnvironment(
    'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
    defaultValue: !kReleaseMode,
  );
  const authSessionStore = SecureAuthSessionStore(
    values: FlutterSecureValueStore.macOsCompatible(),
  );
  final authenticationController = GoTrueAuthenticationController(
    provider: GoTrueHttpClient(
      config: GoTrueClientConfig(
        origin: gotrueOrigin,
        allowInsecureLoopback: allowInsecureLoopback,
      ),
    ),
    store: authSessionStore,
  );
  final authenticationPlugin = OpenMuseGoTruePlugin(
    authentication: authenticationController,
    cloudLabel: cloudOrigin.toString(),
  );
  final cloudWorkspacePlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authenticationController,
    cloudOrigin: cloudOrigin,
    deviceId: 'desktop.flutter.${rootPath.hashCode.toUnsigned(32)}',
    allowInsecureLoopback: allowInsecureLoopback,
  );
  registry.install(authenticationPlugin);
  registry.install(cloudWorkspacePlugin);
  for (final plugin in createOpenMuseBuiltInPlugins()) {
    registry.install(plugin);
  }
  unawaited(() async {
    try {
      await registry.activate(authenticationPlugin.descriptor.id);
    } catch (error) {
      debugPrint('Authentication plugin activation failed: $error');
    }
  }());
  unawaited(() async {
    try {
      await registry.activate(cloudWorkspacePlugin.descriptor.id);
    } catch (error) {
      debugPrint('Cloud Workspace plugin activation failed: $error');
    }
  }());
  unawaited(() async {
    try {
      await registry.activate('com.openmuse.dsh-agent');
    } catch (error) {
      debugPrint('DSH plugin activate deferred: $error');
    }
  }());
  controller.flushBeforeDiskRead = (resource) async {
    for (final descriptor in registry.descriptors) {
      final plugin = registry.plugin(descriptor.id);
      if (plugin is OpenMuseBufferFlushContributor) {
        await (plugin as OpenMuseBufferFlushContributor).flushResource(
          resource,
        );
      }
    }
  };
  return OpenMuseHostApp(
    registry: registry,
    workspace: controller,
    settings: settings,
    layoutController: layoutController,
    layoutStore: layoutStore,
    mutationGuards: SurfaceMutationGuards(),
    authentication: authenticationPlugin,
  );
}

WorkbenchLayoutSnapshot _layoutFallback(OpenMuseLocalSettings settings) {
  const assumedWindowWidth = 1440.0;
  final sidebarRatio = ((settings.sidebarWidth ?? 300) / assumedWindowWidth)
      .clamp(0.12, 0.36)
      .toDouble();
  final remaining = assumedWindowWidth * (1 - sidebarRatio);
  final assistant = settings.assistantWidth ?? 420;
  final editorRatio = ((remaining - assistant) / remaining)
      .clamp(0.45, 0.82)
      .toDouble();
  return createDefaultWorkbenchLayout(
    sidebarRatio: sidebarRatio,
    editorRatio: editorRatio,
  );
}

String _dshHome(String supportPath) {
  final configured = Platform.environment['MUSE_DSH_HOME'];
  if (configured != null && p.isAbsolute(configured)) {
    return p.normalize(configured);
  }
  return p.join(supportPath, 'OpenMuse', 'dsh');
}

final class OpenMuseLaunchApp extends StatefulWidget {
  const OpenMuseLaunchApp({super.key, this.boot = bootOpenMuseHost});

  final Future<Widget> Function() boot;

  @override
  State<OpenMuseLaunchApp> createState() => _OpenMuseLaunchAppState();
}

final class _OpenMuseLaunchAppState extends State<OpenMuseLaunchApp> {
  Widget? _app;
  Object? _error;

  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() async {
    try {
      final app = await widget.boot();
      if (!mounted) return;
      setState(() => _app = app);
    } catch (error) {
      if (!mounted) return;
      setState(() => _error = error);
    }
  }

  @override
  Widget build(BuildContext context) {
    final app = _app;
    if (app != null) return app;
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'OpenMuse',
      theme: buildOpenMuseTheme(),
      home: Scaffold(
        backgroundColor: OpenMuseTokens.canvas,
        body: Center(
          child: _error == null
              ? const CircularProgressIndicator(strokeWidth: 2)
              : Padding(
                  padding: const EdgeInsets.all(24),
                  child: Text(
                    '无法启动 OpenMuse\n$_error',
                    textAlign: TextAlign.center,
                  ),
                ),
        ),
      ),
    );
  }
}
