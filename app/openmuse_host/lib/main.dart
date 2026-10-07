import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import 'src/host/design_system.dart';
import 'src/host/layout/layout.dart';
import 'src/host/layout/surface_mutation_guard.dart';
import 'src/host/local_settings.dart';
import 'src/host/openmuse_app.dart';
import 'src/host/plugin_distribution.dart';
import 'src/host/plugin_cli_broker.dart';
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
              'pluginInteractionDir': p.join(
                support.path,
                'OpenMuse',
                'plugin-interactions',
              ),
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
          case 'workspace.plugin.ensure':
            if (arguments is! Map ||
                arguments['pluginId'] is! String ||
                arguments['path'] is! String) {
              throw const FormatException('无效插件工作区');
            }
            final pluginId = arguments['pluginId'] as String;
            final path = await controller.ensurePluginWorkspace(
              pluginId: pluginId,
              path: arguments['path'] as String,
            );
            final current = settings.pluginValues(pluginId);
            await settings.updatePluginValues(pluginId, {
              ...current,
              'workspaceEnabled': true,
              'workspacePath': path,
            });
            return {'enabled': true, 'path': path};
          case 'plugin.installFromCatalog':
            if (arguments is! Map ||
                arguments['catalogUri'] is! String ||
                arguments['pluginId'] is! String ||
                arguments['workspacePath'] is! String) {
              throw const FormatException('无效插件安装请求');
            }
            final receipt = await acceptDistributedPlugin(
              catalogUri: Uri.parse(arguments['catalogUri'] as String),
              pluginId: arguments['pluginId'] as String,
              workspacePath: arguments['workspacePath'] as String,
              installRoot: Directory(
                p.join(support.path, 'OpenMuse', 'plugins'),
              ),
              target: currentDesktopPluginTarget(),
              workspace: controller,
              settings: settings,
            );
            return receipt.toJson();
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
    _configuredEndpoint(
      key: 'OPENMUSE_GOTRUE_ORIGIN',
      dartDefine: const String.fromEnvironment('OPENMUSE_GOTRUE_ORIGIN'),
      debugDefault: 'http://127.0.0.1:9999',
      releaseDefault: 'https://openmuseai.com/gotrue',
    ),
  );
  final cloudOrigin = Uri.parse(
    _configuredEndpoint(
      key: 'OPENMUSE_CLOUD_ORIGIN',
      dartDefine: const String.fromEnvironment('OPENMUSE_CLOUD_ORIGIN'),
      debugDefault: 'http://127.0.0.1:8000',
      releaseDefault: 'https://openmuseai.com',
    ),
  );
  final allowInsecureLoopback = _configuredBool(
    key: 'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
    dartDefine: const String.fromEnvironment(
      'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
    ),
    fallback: !kReleaseMode,
  );
  final authSessionStore = SecureAuthSessionStore(
    values: FlutterSecureValueStore.macOsCompatible(
      accountName: String.fromEnvironment(
        'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT',
        defaultValue: 'flutter_secure_storage_service',
      ),
    ),
  );
  final goTrueClient = GoTrueHttpClient(
    config: GoTrueClientConfig(
      origin: gotrueOrigin,
      allowInsecureLoopback: allowInsecureLoopback,
    ),
  );
  final authenticationController = GoTrueAuthenticationController(
    provider: goTrueClient,
    store: authSessionStore,
    bootstrapper: AppFlowyAccountBootstrapper(
      cloudOrigin: cloudOrigin,
      allowInsecureLoopback: allowInsecureLoopback,
    ),
  );
  if (kDebugMode) {
    authenticationController.addListener(() {
      final snapshot = authenticationController.snapshot;
      debugPrint(
        'OpenMuse authentication: phase=${snapshot.phase.name} code=${snapshot.failureCode} message=${snapshot.failureMessage}',
      );
    });
  }
  final authenticationPlugin = OpenMuseGoTruePlugin(
    authentication: authenticationController,
    cloudLabel: cloudOrigin.toString(),
    allowAnonymous: true,
  );
  final desktopDeviceId = await _persistentDesktopDeviceId(settings);
  final desktopDisplayName = Platform.localHostname.isEmpty
      ? 'OpenMuse Desktop'
      : Platform.localHostname;
  final cloudWorkspacePlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authenticationController,
    cloudOrigin: cloudOrigin,
    deviceId: desktopDeviceId,
    allowInsecureLoopback: allowInsecureLoopback,
  );
  final cliBroker = PluginCliBroker(
    installRoot: Directory(p.join(support.path, 'OpenMuse', 'plugins')),
  );
  await cliBroker.start();
  final dshSupervisor = DshSidecarSupervisor(
    environment: {
      ...Platform.environment,
      'DSH_HOME': _dshHome(support.path),
      'OPENMUSE_CLI_BROKER_URL': cliBroker.origin.toString(),
      'OPENMUSE_CLI_BROKER_TOKEN': cliBroker.token,
    },
  );
  const relayFromDefine = String.fromEnvironment(
    'OPENMUSE_RELAY_PUBLIC_ORIGIN',
  );
  final relayFromEnvironment =
      Platform.environment['OPENMUSE_RELAY_PUBLIC_ORIGIN'] ?? '';
  final relayConfigured = relayFromDefine.isNotEmpty
      ? relayFromDefine
      : relayFromEnvironment.isNotEmpty
      ? relayFromEnvironment
      : cloudOrigin.host == 'openmuseai.com'
      ? 'https://openmuseai.com:8443'
      : '';
  final relayOrigin = desktopRelayPublicOrigin(
    cloudOrigin: cloudOrigin,
    configured: relayConfigured,
  );
  final surfaceLab = _acceptanceSurface(
    workspaceRef: 'openmuse.local.default',
    desktopDeviceRef: desktopDeviceId,
  );
  final pairedGateway = PairedDesktopGateway(
    currentAccountRef: () =>
        authenticationController.snapshot.identity?.subject,
    validateToken: (token) async {
      final user = await goTrueClient.currentUser(token);
      return user.id;
    },
    dshEndpoint: () async {
      await dshSupervisor.ensureStarted();
      final endpoint = dshSupervisor.endpoint;
      if (endpoint == null) throw StateError('DSH endpoint unavailable');
      return endpoint;
    },
    workspaceRef: 'openmuse.local.default',
    workspaceTitle: 'Project Workspace',
    nativeApiToken: dshSupervisor.bridgeToken,
    deviceRef: desktopDeviceId,
    deviceName: desktopDisplayName,
    port:
        int.tryParse(
          Platform.environment['OPENMUSE_PAIRED_DESKTOP_PORT'] ?? '',
        ) ??
        13180,
    fixedPairingCode:
        Platform.environment['OPENMUSE_PAIRED_DESKTOP_PAIRING_CODE'],
    remoteSurface: (request) => dispatchRemoteSurface(
      host: surfaceLab.host,
      operation: request.operation,
      body: request.body,
      context: RemoteConnectionContext(
        actorRef: request.accountRef,
        mobileDeviceRef: request.deviceRef,
        desktopDeviceRef: desktopDeviceId,
        workspaceRef: request.workspaceRef,
        permissions: surfaceLab.permissions,
      ),
    ),
    remoteMedia: (query) async => _mediaSlice(surfaceLab.media, query),
  );
  final deviceDirectory = AccountDeviceDirectoryController(
    authentication: authenticationController,
    client: AccountDeviceDirectoryClient(
      cloudOrigin: cloudOrigin,
      accessToken: authenticationController.accessToken,
      allowInsecureLoopback: allowInsecureLoopback,
    ),
    registration: () => AccountDeviceRegistration(
      deviceRef: desktopDeviceId,
      displayName: desktopDisplayName,
      platform: Platform.operatingSystem,
      kind: AccountDeviceKind.desktop,
      capabilities: const {
        'workspace.local',
        'dsh.local',
        'paired-desktop.transport',
      },
      transportOrigin: relayOrigin ?? pairedGateway.origin,
    ),
  );
  final relay = relayOrigin == null
      ? null
      : DesktopOutboundRelay(
          attachUri: relayOrigin.replace(scheme: 'wss', path: '/attach'),
          accessToken: () => authenticationController.accessToken(),
          deviceId: desktopDeviceId,
          localGateway: () => pairedGateway.origin,
          publicOrigin: relayOrigin,
        );
  final pairedDesktopPlugin = OpenMusePairedDesktopHostPlugin(
    pairedGateway,
    directory: deviceDirectory,
    relay: relay,
  );
  registry.install(authenticationPlugin);
  registry.install(cloudWorkspacePlugin);
  registry.install(pairedDesktopPlugin);
  for (final plugin in createOpenMuseBuiltInPlugins(
    dshSupervisor: dshSupervisor,
    routeInteraction: (interaction) async {
      try {
        final image = await File(interaction.imagePath).readAsBytes();
        if (image.isEmpty || image.length > 2 * 1024 * 1024) return false;
        final id = List.generate(
          16,
          (_) => Random.secure().nextInt(256).toRadixString(16).padLeft(2, '0'),
        ).join();
        return pairedGateway.offerPluginInteraction(
          PairedPluginInteraction(
            id: id,
            pluginId: interaction.pluginId,
            title: interaction.title,
            imageBytes: Uint8List.fromList(image),
            readStatus: () async {
              final file = File(interaction.statusPath);
              if (await file.length() > 4096) return {'state': 'error'};
              final decoded = jsonDecode(await file.readAsString());
              return decoded is Map
                  ? {'state': decoded['state'], 'message': decoded['message']}
                  : {'state': 'error'};
            },
          ),
        );
      } catch (_) {
        return false;
      }
    },
  )) {
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
      await registry.activate(pairedDesktopPlugin.descriptor.id);
    } catch (error) {
      debugPrint('Paired Desktop plugin activation failed: $error');
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

String _configuredEndpoint({
  required String key,
  required String dartDefine,
  required String debugDefault,
  required String releaseDefault,
}) {
  final compiled = dartDefine.trim();
  if (compiled.isNotEmpty) return compiled;
  final environment = Platform.environment[key]?.trim();
  if (environment != null && environment.isNotEmpty) return environment;
  return kReleaseMode ? releaseDefault : debugDefault;
}

bool _configuredBool({
  required String key,
  required String dartDefine,
  required bool fallback,
}) {
  final raw = dartDefine.trim().isNotEmpty
      ? dartDefine.trim()
      : Platform.environment[key]?.trim().toLowerCase();
  if (raw == 'true' || raw == '1') return true;
  if (raw == 'false' || raw == '0') return false;
  return fallback;
}

Future<String> _persistentDesktopDeviceId(
  OpenMuseLocalSettings settings,
) async {
  const namespace = 'com.openmuse.device.identity';
  final existing = settings.pluginValues(namespace)['deviceId'];
  if (existing is String &&
      existing.startsWith('desktop.') &&
      existing.length <= 160) {
    return existing;
  }
  final random = Random.secure();
  final suffix = List.generate(
    20,
    (_) => random.nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();
  final value = 'desktop.$suffix';
  await settings.updatePluginValues(namespace, {'deviceId': value});
  return value;
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

final class _DesktopSurface {
  _DesktopSurface.lab(AcceptanceDesktop desktop)
    : host = desktop.host,
      media = desktop.media,
      permissions = desktop.connection.permissions;

  _DesktopSurface.empty()
    : host = RemoteSurfaceHost(),
      media = RemoteMediaAuthority(),
      permissions = const {'workspace.resource.read'};

  final RemoteSurfaceHost host;
  final RemoteMediaAuthority media;
  final Set<String> permissions;
}

_DesktopSurface _acceptanceSurface({
  required String workspaceRef,
  required String desktopDeviceRef,
}) {
  const enabled = bool.fromEnvironment('OPENMUSE_REMOTE_SURFACE_ACCEPTANCE');
  if (kReleaseMode && !enabled) return _DesktopSurface.empty();
  return _DesktopSurface.lab(
    AcceptanceDesktop(
      workspaceRef: workspaceRef,
      desktopDeviceRef: desktopDeviceRef,
    ),
  );
}

Future<RemoteMediaSlice> _mediaSlice(
  RemoteMediaAuthority media,
  RemoteMediaQuery query,
) async {
  final read = media.readRange(
    handle: query.handle,
    workspaceRef: query.workspaceRef,
    deviceRef: query.desktopDeviceRef,
    now: DateTime.now(),
    start: query.start,
    endInclusive: query.endInclusive,
  );
  return switch (read) {
    RemoteMediaDenied() => const RemoteMediaSliceDenied(),
    RemoteMediaUnsatisfiable() => const RemoteMediaSliceUnsatisfiable(),
    RemoteMediaBytes(:final bytes, :final total, :final start) =>
      RemoteMediaSliceBody(bytes: bytes, total: total, start: start),
  };
}
