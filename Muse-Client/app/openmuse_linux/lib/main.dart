import 'dart:async';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_io.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import 'src/host/design_system.dart';
import 'src/host/account_storage.dart';
import 'src/host/endpoint_config.dart';
import 'src/host/layout/layout.dart';
import 'src/host/layout/surface_mutation_guard.dart';
import 'src/host/local_settings.dart';
import 'src/host/openmuse_app.dart';
import 'src/host/plugin_distribution.dart';
import 'src/host/mcp_file_plugin.dart';
import 'src/host/mcp_resource_extensions.dart';
import 'src/host/mcp_market.dart';
import 'src/host/mcp_installed_app.dart';
import 'src/host/mcp_terminal_service.dart';
import 'src/host/mcp_capability_gateway.dart';
import 'src/host/mcp_bin_registry.dart';
import 'src/host/plugin_cli_broker.dart';
import 'src/host/plugin_market.dart';
import 'src/host/workspace_controller.dart';
import 'src/host/workspace_mirror_service.dart';
import 'src/host/scheduled_task_service.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Paint immediately. Waiting on workspace/plugin boot before runApp leaves
  // Finder-launched windows blank while Dart is still in main().
  runApp(
    OpenMuseRegionBootstrap(
      store: (release) async {
        final support = await getApplicationSupportDirectory();
        return FileConfigStore(
          File('${support.path}/region-${release.id}.json'),
        );
      },
      boot: (bundle, language) =>
          bootOpenMuseHost(installation: bundle, initialLanguage: language),
    ),
  );
}

Future<Widget> bootOpenMuseHost({
  DeploymentBundle? installation,
  String? initialLanguage,
}) async {
  final deploymentBundle = installation ?? await loadDeploymentBundle();
  var runtimeConfig = deploymentBundle.snapshot;
  final support = await getApplicationSupportDirectory();
  final localeStore = FileConfigStore(
    File(
      p.join(
        support.path,
        'OpenMuse',
        'locale-${runtimeConfig.storageIdentity}.json',
      ),
    ),
  );
  final localeController = OpenMuseUiLocale(
    defaultTag: runtimeConfig.defaultLocale,
    enabledLocales: runtimeConfig.enabledLocales,
    aliases: runtimeConfig.localeAliases,
    systemLocales: WidgetsBinding.instance.platformDispatcher.locales.map(
      (v) => v.toLanguageTag(),
    ),
    preferences: CallbackLocalePreferences(
      readValue: localeStore.read,
      writeValue: localeStore.commit,
    ),
  );
  await localeController.restore();
  if (initialLanguage != null) {
    localeController.selectTag(initialLanguage);
    await localeController.saved;
  }
  final package = await PackageInfo.fromPlatform();
  final runtime = OpenMuseClientRuntime(
    bundle: deploymentBundle,
    transport: createConfigTransport(),
    store: FileConfigStore(
      File('${support.path}/runtime-${runtimeConfig.storageIdentity}.json'),
    ),
    build: int.tryParse(package.buildNumber) ?? 0,
    locale: localeController,
  );
  await runtime.bootstrap();
  runtimeConfig = runtime.client.current;
  runtime.start();
  OpenMuseUpdateController? updates;
  try {
    updates = await bootOpenMuseUpdates(config: runtimeConfig);
    updates?.startPolling(const Duration(minutes: 5));
  } catch (error) {
    // A release-feed failure never blocks an offline workspace.
    debugPrint('App update check unavailable: $error');
  }

  var accountStorage = HostAccountStorage(
    supportPath: support.path,
    subject: localAnonymousUserId,
  );
  final readyAccount = ValueNotifier<String?>(null);
  final settings = OpenMuseLocalSettings(
    file: File(p.join(support.path, 'OpenMuse', 'settings-v1.json')),
  );
  await settings.load();
  final layoutStore = LayoutStore(
    File(p.join(support.path, 'OpenMuse', 'layout-v1.json')),
    fallback: _layoutFallback(settings),
  );
  final layoutController = WorkbenchLayoutController(await layoutStore.load());
  final mountStore = WorkspaceMountStore(File(accountStorage.mountsPath));
  final savedMounts = await mountStore.load();
  // Application Support is always accessible at launch. External project
  // mounts are restored separately and scanned after the first frame, since
  // macOS Documents/iCloud/FileProvider may block directory enumeration.
  final rootPath = accountStorage.workspacePath;
  await Directory(rootPath).create(recursive: true);
  final controller = LocalWorkspaceController(
    rootPath: rootPath,
    initialResources: const [],
    versionStore: LocalVersionStore(Directory(accountStorage.versionsPath)),
    mountStore: mountStore,
    additionalMountPaths: savedMounts,
  );
  unawaited(
    controller.initialize().catchError((Object error) {
      debugPrint('Workspace scan deferred/failed: $error');
    }),
  );
  late final OpenMusePluginRegistry registry;
  final resourceAgents = ResourceAgentBroker();
  await resourceAgents.start();
  controller.addListener(() {
    resourceAgents.activeUri = controller
        .activeTabFor(controller.focusedEditorGroupId)
        ?.resource
        .uri
        .toString();
  });
  resourceAgents.sendAgentMessage = (content) async {
    for (final descriptor in registry.descriptors) {
      final plugin = registry.plugin(descriptor.id);
      if (plugin is OpenMuseAgentContributor) {
        await registry.ensureActive(plugin!);
        await (plugin as OpenMuseAgentContributor).sendAgentMessage(content);
        return;
      }
    }
    throw StateError('No Agent plugin installed');
  };
  const registryOverride = String.fromEnvironment('OPENMUSE_MCP_REGISTRY_URL');
  final mcpRegistryUrl = registryOverride.isNotEmpty
      ? registryOverride
      : (kDebugMode
                ? Platform.environment['OPENMUSE_MCP_REGISTRY_URL']
                : null) ??
            runtimeConfig.serviceUri(ServiceId.pluginRegistry).toString();
  final providers = McpRegistryProviders(
    file: File(p.join(support.path, 'OpenMuse', 'mcp-registries.json')),
    defaultOrigin: mcpRegistryUrl.isEmpty ? null : Uri.parse(mcpRegistryUrl),
  );
  await providers.load();
  final mcpInstallRoot = Directory(p.join(support.path, 'OpenMuse', 'mcpb'));
  final marketNode = resolveMarketDshRuntime(
    executablePath: Platform.resolvedExecutable,
    environment: Platform.environment,
  ).nodeExecutable;
  McpRegistryMarket makeMarket(Uri origin) => McpRegistryMarket(
    origin: origin,
    installRoot: mcpInstallRoot,
    nodeExecutable: marketNode,
  );
  McpRegistryMarket? selectedMarket() =>
      providers.selected == null ? null : makeMarket(providers.selected!);
  final receiptMarket = makeMarket(Uri());
  final mcpNavigatorKey = GlobalKey<NavigatorState>();
  final mcpApps = McpAppsEnvironment(
    market: receiptMarket,
    nodeExecutable: marketNode,
    scope: () => accountStorage.subject,
    navigatorKey: mcpNavigatorKey,
    workspaceDirectory: () => controller.activeMountPath,
    selectedMarket: selectedMarket,
  );
  final resourceExtensions = McpResourceExtensions(
    market: receiptMarket,
    nodeExecutable: marketNode,
  );
  mcpApps.onDevelopmentBundle = (caller, entry, directory, valid) =>
      resourceExtensions.addDevelopment(
        caller,
        entry,
        McpBundleRuntime(
          bundleDirectory: directory,
          nodeExecutable: marketNode,
        ),
        valid,
      );
  mcpApps.onDevelopmentClosed = resourceExtensions.removeDevelopment;
  Future<void> registerMcp(McpMarketEntry entry) async {
    final existing = registry.plugin(entry.name);
    if (existing?.descriptor.version == entry.version) return;
    final node = resolveMarketDshRuntime(
      executablePath: Platform.resolvedExecutable,
      environment: Platform.environment,
    ).nodeExecutable;
    final runtime = McpBundleRuntime(
      bundleDirectory: receiptMarket.directoryFor(entry),
      nodeExecutable: node,
    );
    final plugin = await discoverMcpFilePlugin(
      entry: entry,
      runtime: runtime,
      agentBroker: resourceAgents,
      extensions: resourceExtensions,
      contributors: () => [
        for (final descriptor in registry.descriptors)
          if (registry.plugin(descriptor.id)
              case final OpenMuseToolContributor plugin)
            (id: descriptor.id, plugin: plugin),
      ],
      approve: mcpApps.approve,
    );
    if (plugin != null) {
      if (existing != null) await registry.uninstall(entry.name);
      registry.install(plugin);
    }
  }

  final mcpBinDirectory = Directory(
    p.join(support.path, 'OpenMuse', 'mcp-bin'),
  );
  final cliLauncher = p.normalize(
    p.join(
      p.dirname(Platform.resolvedExecutable),
      '../Resources/openmuse/cli/bin/openmuse',
    ),
  );
  Future<void> refreshBins() async {
    await refreshMcpBinShims(receiptMarket, mcpBinDirectory, cliLauncher);
  }

  mcpApps.onInstalled = (entry) async {
    resourceExtensions.invalidate();
    await registerMcp(entry);
    await refreshBins();
  };
  final terminalMcp = McpTerminalService(
    onRevoke: mcpApps.closeCaller,
    scope: () => accountStorage.subject,
    tools: [
      ...McpAppsEnvironment.workspaceTools,
      ...McpCapabilityGateway.tools.where(
        (t) => {
          'host_describe',
          'capabilities_list',
          'development_open_bundle',
          'development_read_resource',
        }.contains(t['name']),
      ),
    ],
    call: (caller, root, name, args) async {
      switch (name) {
        case 'host_describe':
          return mcpApps.identity;
        case 'capabilities_list':
          final installed = await mcpApps.providers();
          try {
            return {'providers': installed.map((p) => p.toJson()).toList()};
          } finally {
            for (final p in installed) {
              await p.close?.call();
            }
          }
        case 'development_open_bundle':
          return mcpApps.openDevelopment(caller, args);
        case 'development_read_resource':
          return mcpApps.readDevelopment(caller, args);
        default:
          return mcpApps.workspaceCall(name, args, root);
      }
    },
  );
  await terminalMcp.start();
  await refreshBins();

  Future<Map<String, Object?>> runMarketAction(
    String operation,
    String id,
  ) async {
    if (operation == 'install' &&
        !runtime.presentation.isEnabled(AppFeature.pluginMarket)) {
      throw StateError('Plugin installation is currently disabled');
    }
    final mcpMarket = selectedMarket();
    List<McpMarketEntry> listed = [];
    if (mcpMarket != null) {
      try {
        listed = await mcpMarket.list();
      } on Object {
        if (operation == 'install') rethrow;
      }
      final matches = listed.where((entry) => entry.name == id);
      if (matches.isNotEmpty) {
        final entry = matches.first;
        if (operation == 'plan') return mcpMarket.plan(entry);
        if (operation == 'install') {
          final installed = await mcpMarket.install(entry);
          await registerMcp(entry);
          await refreshBins();
          return installed;
        }
        if (operation == 'open') {
          final installed = await mcpMarket.installedEntries();
          if (!installed.any(
            (item) => item.name == entry.name && item.version == entry.version,
          )) {
            throw StateError('请先安装此 MCP 插件');
          }
          final uri = await McpBundleRuntime(
            bundleDirectory: mcpMarket.directoryFor(entry),
            nodeExecutable: mcpMarket.nodeExecutable,
          ).openUi();
          if (uri == null) throw StateError('插件没有可打开的 MCP App');
          return {'uri': uri.toString()};
        }
        throw UnsupportedError('MCPB enable/disable is not available');
      }
    }
    if (operation == 'plan' || operation == 'open') {
      final installed = (await receiptMarket.installedEntries()).where(
        (entry) => entry.name == id,
      );
      if (installed.isNotEmpty) {
        final entry = installed.first;
        final localMarket = makeMarket(Uri.parse(entry.packageUri.origin));
        if (operation == 'plan') {
          return {...await localMarket.plan(entry), 'listed': false};
        }
        final uri = await McpBundleRuntime(
          bundleDirectory: localMarket.directoryFor(entry),
          nodeExecutable: localMarket.nodeExecutable,
        ).openUi();
        if (uri == null) throw StateError('插件没有可打开的 MCP App');
        return {'uri': uri.toString()};
      }
    }
    final entries = builtInMarketPlugins.where((entry) => entry.id == id);
    if (entries.isEmpty) throw const FormatException('市场中没有该插件');
    final marketRuntime = resolveMarketDshRuntime(
      executablePath: Platform.resolvedExecutable,
      environment: Platform.environment,
    );
    final cli = marketRuntime.cliPath;
    if (cli == null) throw StateError('未找到 DSH CLI');
    final installer = DshMarketInstaller(
      dshHome: Directory(accountStorage.dshHome),
      profile: 'web',
      cliPath: cli,
      nodeExecutable: marketRuntime.nodeExecutable,
    );
    return switch (operation) {
      'plan' => installer.plan(entries.single),
      'install' => installer.install(entries.single),
      'enable' => installer.setEnabled(entries.single, true),
      'disable' => installer.setEnabled(entries.single, false),
      _ => throw const FormatException('无效市场操作'),
    };
  }

  registry = OpenMusePluginRegistry(
    context: OpenMusePluginContext(
      localizations: OpenMusePluginLocalizationAdapter(localeController),
      configuration: OpenMusePluginConfigurationAdapter(runtime),
      hostChanges: controller,
      executeHostCommand: (command, arguments) async {
        switch (command) {
          case 'terminal.session.release':
            await terminalMcp.revoke((arguments as Map)['token'] as String);
            return null;
          case 'terminal.session.create':
            final directory = (arguments as Map)['directory'] as String;
            final root = await Directory(directory).resolveSymbolicLinks();
            final roots = [
              controller.rootPath,
              ...controller.mounts.map((m) => m.path),
            ];
            final authorized = await Future.wait(
              roots.map((r) async {
                try {
                  final real = await Directory(r).resolveSymbolicLinks();
                  return root == real || p.isWithin(real, root);
                } on FileSystemException {
                  return false;
                }
              }),
            );
            if (!authorized.any((v) => v)) {
              throw StateError('Terminal workspace not authorized');
            }
            await refreshBins();
            return {
              ...terminalMcp.issue(root),
              'MCP_BIN_PATH': mcpBinDirectory.path,
              'MCP_REGISTRY_URL':
                  providers.selected?.toString() ??
                  runtimeConfig.serviceUri(ServiceId.pluginRegistry).toString(),
            };
          case 'workspace.snapshot':
            return {
              'workspaceRef': 'openmuse.local.default',
              'title': 'Project Workspace',
              'activeMountPath': controller.activeMountPath,
              'dshHome': accountStorage.dshHome,
              'pluginInteractionDir': accountStorage.pluginInteractionsPath,
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
          case 'plugin.market.list':
            final market = selectedMarket();
            var catalogAvailable = market != null;
            List<McpMarketEntry> listed = [];
            if (market != null) {
              try {
                listed = await market.list();
              } on Object {
                catalogAvailable = false;
              }
            }
            final names = listed.map((entry) => entry.name).toSet();
            return [
              for (final entry in builtInMarketPlugins) entry.toJson(),
              for (final entry in listed)
                {...entry.toMarketCard(), 'listed': true},
              for (final entry in await receiptMarket.installedEntries())
                if (!names.contains(entry.name))
                  {
                    ...entry.toMarketCard(),
                    'listed': catalogAvailable ? false : null,
                  },
            ];
          case 'plugin.market.providers':
            return {
              'selected': providers.selected?.toString(),
              'providers': [
                for (final uri in providers.origins) uri.toString(),
              ],
            };
          case 'plugin.market.selectProvider':
            if (arguments is! String) {
              throw const FormatException('无效市场地址');
            }
            await providers.select(arguments);
            return providers.selected?.toString();
          case 'plugin.market.plan':
          case 'plugin.market.install':
          case 'plugin.market.open':
          case 'plugin.market.enable':
          case 'plugin.market.disable':
            if (arguments is! Map || arguments['id'] is! String) {
              throw const FormatException('无效市场插件 ID');
            }
            return runMarketAction(
              command.substring('plugin.market.'.length),
              arguments['id'] as String,
            );
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
  // Every host, port and loopback address comes from one configuration object.
  final endpoints = HostEndpointConfig.fromEnvironment(config: runtimeConfig);
  final authSessionStore = SecureAuthSessionStore(
    key: runtimeConfig.authSessionStorageKey,
    values: FlutterSecureValueStore.macOsCompatible(
      accountName: endpoints.authKeychainAccount,
    ),
  );
  final goTrueClient = GoTrueHttpClient(
    networkGuard: runtime.checkNetwork,
    config: GoTrueClientConfig(
      origin: endpoints.gotrueOrigin,
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
    ),
  );
  final authenticationController = GoTrueAuthenticationController(
    networkGuard: runtime.checkNetwork,
    provider: goTrueClient,
    store: authSessionStore,
    bootstrapper: AppFlowyAccountBootstrapper(
      networkGuard: runtime.checkNetwork,
      cloudOrigin: endpoints.cloudOrigin,
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
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
    cloudLabel: endpoints.cloudOrigin.toString(),
    allowAnonymous: true,
    termsUri: runtimeConfig.linkUri(LinkId.terms),
    privacyUri: runtimeConfig.linkUri(LinkId.privacy),
  );
  final desktopDeviceId = await _persistentDesktopDeviceId(settings);
  settings.setAccountSubject(localAnonymousUserId);
  final desktopDisplayName =
      Platform.environment['OPENMUSE_DEVICE_NAME'] ??
      (Platform.localHostname.isEmpty
          ? 'OpenMuse Linux'
          : 'OpenMuse Linux · ${Platform.localHostname}');
  final cloudWorkspacePlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authenticationController,
    cloudOrigin: endpoints.cloudOrigin,
    deviceId: desktopDeviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
  );
  final cliBroker = PluginCliBroker(
    installRoot: Directory(p.join(support.path, 'OpenMuse', 'plugins')),
  );
  await cliBroker.start();
  final dshSupervisor = DshSidecarSupervisor(
    environment: {
      ...Platform.environment,
      'DSH_HOME': accountStorage.dshHome,
      'OPENMUSE_RESOURCE_MCP_URL': resourceAgents.endpoint.toString(),
      'OPENMUSE_RESOURCE_MCP_TOKEN': resourceAgents.token,
      'OPENMUSE_CLI_BROKER_URL': cliBroker.origin.toString(),
      'OPENMUSE_CLI_BROKER_TOKEN': cliBroker.token,
    },
  );
  Future<void> executeScheduledPrompt(
    String workspaceId,
    String requestId,
    String message,
  ) async {
    await dshSupervisor.ensureStarted();
    await DshScheduledPromptExecutor(
      endpoint: () async {
        final endpoint = dshSupervisor.endpoint;
        if (endpoint == null) throw StateError('DSH endpoint unavailable');
        return endpoint;
      },
      bridgeToken: dshSupervisor.bridgeToken,
    )(workspaceId, requestId, message);
  }

  var scheduledTasks = DesktopScheduledTaskService(
    storeFile: File(accountStorage.scheduledTasksPath),
    executor: executeScheduledPrompt,
  );
  Future<void> scopeTransition = Future.value();
  String? requestedSubject;
  void syncAccountStorage() {
    final snapshot = authenticationController.snapshot;
    final subject = snapshot.isAuthenticated
        ? snapshot.identity?.subject
        : null;
    if (subject == null) {
      final hadAccount = requestedSubject != null;
      requestedSubject = null;
      readyAccount.value = null;
      scheduledTasks.dispose();
      if (hadAccount) {
        resourceAgents.clear();
        scopeTransition = scopeTransition
            .then((_) => dshSupervisor.stop())
            .catchError((Object error) {
              debugPrint('Account DSH stop failed: $error');
            });
      }
      return;
    }
    if (subject == requestedSubject) return;
    requestedSubject = subject;
    readyAccount.value = null;
    scopeTransition = scopeTransition
        .then((_) async {
          if (requestedSubject != subject) return;
          if (accountStorage.subject != subject) {
            await dshSupervisor.stop();
            settings.setAccountSubject(subject);
            final next = HostAccountStorage(
              supportPath: support.path,
              subject: subject,
            );
            await controller.switchStorage(
              rootPath: next.workspacePath,
              versionStore: LocalVersionStore(Directory(next.versionsPath)),
              mountStore: WorkspaceMountStore(File(next.mountsPath)),
            );
            dshSupervisor.environment['DSH_HOME'] = next.dshHome;
            accountStorage = next;
          }
          scheduledTasks.dispose();
          scheduledTasks = DesktopScheduledTaskService(
            storeFile: File(accountStorage.scheduledTasksPath),
            executor: executeScheduledPrompt,
          );
          await scheduledTasks.restore();
          if (requestedSubject == subject) readyAccount.value = subject;
        })
        .catchError((Object error) {
          debugPrint('Account workspace switch failed: $error');
        });
  }

  authenticationController.addListener(syncAccountStorage);
  final relayOrigin = desktopRelayPublicOrigin(
    cloudOrigin: endpoints.cloudOrigin,
    configured: endpoints.relayPublicOrigin,
  );
  final surfaceLab = _acceptanceSurface(
    workspaceRef: 'openmuse.local.default',
    desktopDeviceRef: desktopDeviceId,
  );
  final resourceCenter = ValueNotifier<NativeResourceCenter?>(null);
  Future<void> resourceTransition = Future.value();
  int resourceEpoch = 0;
  void syncResourceAccount() {
    final epoch = ++resourceEpoch,
        scope = readyAccount.value ?? 'signed-out-local';
    final previous = resourceCenter.value;
    resourceCenter.value = null;
    resourceTransition = resourceTransition
        .catchError((Object _) {})
        .then((_) async {
          await previous?.close();
          if (epoch != resourceEpoch) return;
          final center = await NativeResourceCenter.open(
            seedWelcomeResources: true,
            scope: scope,
            deviceRef: desktopDeviceId,
            deviceLabel: Platform.localHostname,
            registry: registry,
            workspace: 'openmuse.local.default',
            workspaceLabel: 'Workspace',
            storageRoot: Directory('${support.path}/resource-center'),
          );
          if (epoch != resourceEpoch) {
            await center.close();
            return;
          }
          await center.mountDirectories({
            for (final mount in controller.mounts) mount.path: mount.name,
          });
          resourceCenter.value = center;
        })
        .catchError((Object error) {
          debugPrint('Resource center unavailable: ${error.runtimeType}');
        });
  }

  readyAccount.addListener(syncResourceAccount);
  syncResourceAccount();
  controller.addListener(() {
    final center = resourceCenter.value;
    if (center == null) return;
    unawaited(
      center
          .mountDirectories({
            for (final mount in controller.mounts) mount.path: mount.name,
          })
          .catchError((Object error) {
            debugPrint('Resource mounts unavailable: ${error.runtimeType}');
          }),
    );
  });
  final workspaceMirrorService = DesktopWorkspaceMirrorService(controller);
  final pairedGateway = PairedDesktopGateway(
    currentAccountRef: () => readyAccount.value,
    resources: (request) async {
      await resourceTransition;
      final center = resourceCenter.value;
      if (center == null || center.hub.scope != request.accountRef) {
        return {'error': 'DENIED'};
      }
      try {
        return await center.rpc.dispatch(
          request.body,
          ResourcePeer(
            scope: request.accountRef,
            deviceRef: request.deviceRef,
            workspaceRef: request.workspaceRef,
            expiresAtMs: request.expiresAtMs,
          ),
        );
      } on ResourceFailure catch (error) {
        return {'error': error.code};
      } on Object {
        return {'error': 'INVALID_REQUEST'};
      }
    },
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
    workspaceMirror: workspaceMirrorService.handle,
    workspaceMirrorAction: workspaceMirrorService.act,
    workspaceMirrorResource: workspaceMirrorService.readResource,
    scheduledTasks: (request) async {
      if (request.accountRef != readyAccount.value) {
        throw StateError('Desktop 登录状态已变化');
      }
      return switch (request.operation) {
        'list' => {'tasks': scheduledTasks.list()},
        'create' => {
          'task': await scheduledTasks.create(
            workspaceId: request.workspaceId!,
            message: request.message!,
            dueAt: request.dueAt!,
          ),
        },
        'cancel' => (() async {
          await scheduledTasks.cancel(request.id!);
          return <String, Object?>{'cancelled': true};
        })(),
        _ => throw const FormatException('无效定时任务操作'),
      };
    },
    pluginMarket: (request) async {
      if (request.accountRef != readyAccount.value) {
        throw StateError('Desktop 登录状态已变化');
      }
      if (request.operation == 'list') {
        return {
          'plugins': [for (final entry in builtInMarketPlugins) entry.toJson()],
        };
      }
      return runMarketAction(request.operation, request.pluginId!);
    },
    nativeApiToken: dshSupervisor.bridgeToken,
    deviceRef: desktopDeviceId,
    deviceName: desktopDisplayName,
    port: endpoints.pairedDesktopPort,
    fixedPairingCode: endpoints.fixedPairedDesktopPairingCode,
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
      cloudOrigin: endpoints.cloudOrigin,
      accessToken: authenticationController.accessToken,
      refreshAccessToken: () =>
          authenticationController.accessToken(forceRefresh: true),
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
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
    nativeMcpEditorBuilder: (pluginId) =>
        McpInstalledApp(pluginId: pluginId, environment: mcpApps),
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
    if (plugin.descriptor.activationEvents.contains('onStartup') &&
        (plugin is OpenMuseResourceHandlerContributor ||
            plugin is OpenMuseResourceCreatorContributor ||
            plugin is OpenMuseResourceContextContributor)) {
      try {
        await registry.activate(plugin.descriptor.id);
      } on Object catch (error) {
        debugPrint(
          'Resource plugin startup failed: ${plugin.descriptor.id} (${error.runtimeType})',
        );
      }
    }
  }
  for (final entry in await receiptMarket.installedEntries()) {
    try {
      await registerMcp(entry);
    } catch (error) {
      debugPrint('MCPB activation deferred: $error');
    }
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
  unawaited(() async {
    try {
      await registry.activate('com.openmuse.plugin-market');
    } catch (error) {
      debugPrint('Plugin market activation failed: $error');
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
  return OpenMuseRuntimeHost(
    runtime: runtime,
    child: OpenMuseHostApp(
      localeController: localeController,
      navigatorKey: mcpNavigatorKey,
      updates: updates,
      registry: registry,
      workspace: controller,
      settings: settings,
      layoutController: layoutController,
      layoutStore: layoutStore,
      mutationGuards: SurfaceMutationGuards(),
      authentication: authenticationPlugin,
      readyAccount: readyAccount,
      resources: resourceCenter,
    ),
  );
}

WorkbenchLayoutSnapshot _layoutFallback(OpenMuseLocalSettings settings) {
  const assumedWindowWidth = 1440.0;
  final sidebarRatio = ((settings.sidebarWidth ?? 300) / assumedWindowWidth)
      .clamp(0.12, 0.36)
      .toDouble();
  final remaining = assumedWindowWidth * (1 - sidebarRatio);
  final assistant = settings.assistantWidth ?? 420;
  final agentRatio = (assistant / remaining).clamp(0.32, 0.7).toDouble();
  return createDefaultWorkbenchLayout(
    sidebarRatio: sidebarRatio,
    agentRatio: agentRatio,
  );
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
      localizationsDelegates: OpenMuseAuthLocalizations.localizationsDelegates,
      supportedLocales: OpenMuseAuthLocalizations.supportedLocales,
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
