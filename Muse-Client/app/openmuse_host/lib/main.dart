import 'dart:async';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:appflowy_editor_native_plugin/appflowy_editor_native_plugin.dart';
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
import 'src/host/distribution_channel.dart';
import 'src/host/account_storage.dart';
import 'src/host/account_deletion.dart';
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
import 'src/host/word_product_plugin.dart';
import 'src/host/pdf_product_plugin.dart';
import 'src/host/pdf_generated_document_port.dart';
import 'src/host/spreadsheet_product_plugin.dart';
import 'src/host/scheduled_task_service.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Paint immediately. Waiting on workspace/plugin boot before runApp leaves
  // Finder-launched windows blank while Dart is still in main().
  runApp(
    OpenMuseRegionBootstrap(
      requiresLegalConsent: !macAppStoreDistribution,
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
  void bootPhase(String phase) => debugPrint('OpenMuse boot: $phase');
  final supportOverride = kDebugMode
      ? Platform.environment['OPENMUSE_E2E_SUPPORT_DIR']
      : null;
  final support = supportOverride == null
      ? await getApplicationSupportDirectory()
      : Directory(supportOverride);
  bootPhase('support ${support.path}');
  await support.create(recursive: true);
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
  runtimeConfig = runtime.networkExpired
      ? runtime.client.current
      : runtime.network;
  runtime.start();
  bootPhase('updates');
  OpenMuseUpdateController? updates;
  try {
    updates = await bootOpenMuseUpdates(
      config: runtimeConfig,
    ).timeout(const Duration(seconds: 15));
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
  bootPhase('layout loaded');
  final mountStore = WorkspaceMountStore(File(accountStorage.mountsPath));
  final savedMounts = await mountStore.load();
  bootPhase('mounts loaded');
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
  Future<void> openGeneratedPdfDocument(String path) =>
      openGeneratedPdfInWorkspace(controller, path);
  unawaited(
    controller.initialize().catchError((Object error) {
      debugPrint('Workspace scan deferred/failed: $error');
    }),
  );
  late final OpenMusePluginRegistry registry;
  final resourceAgents = ResourceAgentBroker();
  await resourceAgents.start();
  bootPhase('resource agent started');
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
  final bundledWordRoot = Directory(
    p.normalize(
      p.join(
        p.dirname(Platform.resolvedExecutable),
        '../Resources/openmuse/word-product',
      ),
    ),
  );
  final bundledWordIdentity = File(
    p.join(bundledWordRoot.path, 'identity.json'),
  );
  final bundledWordKey = File(p.join(bundledWordRoot.path, 'public.pem'));
  final bundledPdfRoot = Directory(
    p.normalize(
      p.join(
        p.dirname(Platform.resolvedExecutable),
        '../Resources/openmuse/pdf-product',
      ),
    ),
  );
  final bundledPdfIdentity = File(p.join(bundledPdfRoot.path, 'identity.json'));
  final bundledPdfKey = File(p.join(bundledPdfRoot.path, 'public.pem'));
  final bundledSpreadsheetRoot = Directory(
    p.normalize(
      p.join(
        p.dirname(Platform.resolvedExecutable),
        '../Resources/openmuse/spreadsheet-product',
      ),
    ),
  );
  final bundledSpreadsheetIdentity = File(
    p.join(bundledSpreadsheetRoot.path, 'identity.json'),
  );
  final bundledSpreadsheetKey = File(
    p.join(bundledSpreadsheetRoot.path, 'public.pem'),
  );
  final bundledPptxRoot = Directory(
    p.normalize(
      p.join(
        p.dirname(Platform.resolvedExecutable),
        '../Resources/openmuse/genoffice-pptx',
      ),
    ),
  );
  final bundledPptxBundle = File(
    p.join(bundledPptxRoot.path, 'genoffice-pptx.mcpb'),
  );
  final bundledPptxIdentity = File(
    p.join(bundledPptxRoot.path, 'identity.json'),
  );
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
    if (macAppStoreDistribution) {
      final bundled =
          jsonDecode(await bundledPptxIdentity.readAsString()) as Map;
      if (entry.name != 'io.openmuse/genoffice-pptx' ||
          entry.sha256 != bundled['sha256'] ||
          entry.version != bundled['version']) {
        throw UnsupportedError(
          'Only the fixed bundled app is available in this distribution',
        );
      }
    }
    final existing = registry.plugin(entry.name);
    if (existing?.descriptor.version == entry.version) return;
    if (entry.name == wordProductId) {
      final keyPath =
          (kDebugMode ? Platform.environment['WORD_STACK_PUBLIC_KEY'] : null) ??
          Platform.environment['OPENMUSE_WORD_PRODUCT_PUBLIC_KEY'] ??
          (await bundledWordKey.exists() ? bundledWordKey.path : null);
      final bundledIdentity = await bundledWordIdentity.exists()
          ? jsonDecode(await bundledWordIdentity.readAsString()) as Map
          : null;
      final keyId =
          (kDebugMode ? Platform.environment['WORD_STACK_KEY_ID'] : null) ??
          Platform.environment['OPENMUSE_WORD_PRODUCT_KEY_ID'] ??
          bundledIdentity?['keyId'] as String?;
      if (keyPath == null || keyId == null) {
        debugPrint('Word product trust key unavailable; editor not registered');
        return;
      }
      final word = await OpenMuseWordProductPlugin.load(
        entry: entry,
        installedDirectory: receiptMarket.directoryFor(entry),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
      );
      if (existing != null) await registry.uninstall(entry.name);
      registry.install(word);
      return;
    }
    if (entry.name == pdfProductId) {
      final keyPath =
          Platform.environment['OPENMUSE_PDF_PRODUCT_PUBLIC_KEY'] ??
          (await bundledPdfKey.exists() ? bundledPdfKey.path : null);
      final identity = await bundledPdfIdentity.exists()
          ? jsonDecode(await bundledPdfIdentity.readAsString()) as Map
          : null;
      final keyId =
          Platform.environment['OPENMUSE_PDF_PRODUCT_KEY_ID'] ??
          identity?['keyId'] as String?;
      if (keyPath == null || keyId == null) {
        debugPrint('PDF product trust key unavailable; editor not registered');
        return;
      }
      final pdf = await OpenMusePdfProductPlugin.load(
        entry: entry,
        installedDirectory: receiptMarket.directoryFor(entry),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
        settings: settings,
        openGeneratedDocument: openGeneratedPdfDocument,
      );
      if (existing != null) await registry.uninstall(entry.name);
      registry.install(pdf);
      return;
    }
    if (entry.name == spreadsheetProductId) {
      final keyPath =
          Platform.environment['OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY'] ??
          (await bundledSpreadsheetKey.exists()
              ? bundledSpreadsheetKey.path
              : null);
      final identity = await bundledSpreadsheetIdentity.exists()
          ? jsonDecode(await bundledSpreadsheetIdentity.readAsString()) as Map
          : null;
      final keyId =
          Platform.environment['OPENMUSE_SPREADSHEET_PRODUCT_KEY_ID'] ??
          identity?['keyId'] as String?;
      if (keyPath == null || keyId == null) {
        debugPrint(
          'Spreadsheet product trust key unavailable; editor not registered',
        );
        return;
      }
      final spreadsheet = await OpenMuseSpreadsheetProductPlugin.load(
        entry: entry,
        installedDirectory: receiptMarket.directoryFor(entry),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
      );
      if (existing != null) await registry.uninstall(entry.name);
      registry.install(spreadsheet);
      return;
    }
    final node = resolveMarketDshRuntime(
      executablePath: Platform.resolvedExecutable,
      environment: Platform.environment,
    ).nodeExecutable;
    final runtime = McpBundleRuntime(
      bundleDirectory: macAppStoreDistribution
          ? Directory(p.join(bundledPptxRoot.path, 'runtime'))
          : receiptMarket.directoryFor(entry),
      nodeExecutable: node,
    );
    final plugin = await discoverMcpFilePlugin(
      entry: entry,
      runtime: runtime,
      agentBroker: macAppStoreDistribution ? null : resourceAgents,
      extensions: macAppStoreDistribution ? null : resourceExtensions,
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
      await registry.activate(plugin.descriptor.id);
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
    // A marketplace app may expose a no-argument MCP Apps entry. Open that
    // reviewed surface after installation so its own onboarding can run.
    final runtime = McpBundleRuntime(
      bundleDirectory: receiptMarket.directoryFor(entry),
      nodeExecutable: marketNode,
    );
    final tools = await runtime.tools();
    final launchable = tools.any((tool) {
      final schema = tool['inputSchema'];
      return tool['uiResourceUri'] is String &&
          (schema is! Map ||
              schema['required'] is! List ||
              (schema['required'] as List).isEmpty);
    });
    if (launchable) {
      final navigator = mcpNavigatorKey.currentState;
      if (navigator != null) {
        unawaited(
          navigator.push<void>(
            MaterialPageRoute(
              builder: (_) => Scaffold(
                appBar: AppBar(title: Text(entry.title)),
                body: McpInstalledApp(
                  pluginId: entry.name,
                  environment: mcpApps,
                ),
              ),
            ),
          ),
        );
      }
    }
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
  if (!macAppStoreDistribution) {
    await terminalMcp.start();
    await refreshBins();
  }

  Future<Map<String, Object?>> runMarketAction(
    String operation,
    String id,
  ) async {
    if (macAppStoreDistribution) {
      throw UnsupportedError(
        'External plugin actions are unavailable in this distribution',
      );
    }
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
        if (!hostCommandAllowed(command)) {
          throw UnsupportedError(
            'This command is unavailable in this distribution',
          );
        }
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
    onDeleteAccount: () async {
      runtime.checkNetwork();
      final token = await authenticationController.accessToken(
        forceRefresh: true,
      );
      await deleteHostAccount(
        origin: endpoints.cloudOrigin,
        token: token ?? '',
      );
      await authenticationController.signOut();
    },
    cloudLabel: endpoints.cloudOrigin.toString(),
    allowAnonymous: true,
    termsUri: runtimeConfig.linkUri(LinkId.terms),
    privacyUri: runtimeConfig.linkUri(LinkId.privacy),
  );
  final desktopDeviceId = await _persistentDesktopDeviceId(settings);
  settings.setAccountSubject(localAnonymousUserId);
  final desktopDisplayName = Platform.localHostname.isEmpty
      ? 'OpenMuse Desktop'
      : Platform.localHostname;
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
  bootPhase('cli broker started');
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
  if (!macAppStoreDistribution) registry.install(pairedDesktopPlugin);
  for (final plugin in createOpenMuseBuiltInPlugins(
    dshSupervisor: dshSupervisor,
    nativeMcpEditorBuilder: (pluginId) =>
        pluginId == 'io.openmuse/appflowy-editor'
        ? const AppFlowyMarkdownEditor()
        : McpInstalledApp(pluginId: pluginId, environment: mcpApps),
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
    if (macAppStoreDistribution &&
        const {
          'com.openmuse.cli',
          'com.openmuse.plugin-market',
        }.contains(plugin.descriptor.id)) {
      continue;
    }
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
  bootPhase('reading installed plugins');
  McpMarketEntry? fixedBundledPptx;
  if (await bundledPptxBundle.exists() && await bundledPptxIdentity.exists()) {
    try {
      final identity =
          jsonDecode(await bundledPptxIdentity.readAsString()) as Map;
      final digest = identity['sha256'] as String;
      final entry = McpMarketEntry(
        name: 'io.openmuse/genoffice-pptx',
        title: 'GenOffice PPTX',
        description: 'Bundled GenOffice PPTX MCP Apps editor',
        version: identity['version'] as String,
        packageUri: runtimeConfig.servicePath(ServiceId.pluginRegistry, [
          'packages',
          '$digest.mcpb',
        ]),
        sha256: digest,
      );
      fixedBundledPptx = entry;
      if (!macAppStoreDistribution) {
        await receiptMarket.install(
          entry,
          bundleBytes: await bundledPptxBundle.readAsBytes(),
        );
      }
    } catch (error) {
      debugPrint('Bundled PPTX registration failed: $error');
    }
  }
  final installedMcpEntries = macAppStoreDistribution
      ? [?fixedBundledPptx]
      : await receiptMarket.installedEntries();
  // The native Word editor must be available for a restored DOCX tab. Other
  // MCP servers are probed after the Host has painted its first frame: an
  // unresponsive installed plugin must never hold the entire window on the
  // launch spinner.
  for (final entry in installedMcpEntries.where(
    (entry) =>
        entry.name == wordProductId ||
        entry.name == pdfProductId ||
        entry.name == spreadsheetProductId ||
        entry.name == 'io.openmuse/genoffice-pptx',
  )) {
    try {
      await registerMcp(entry);
    } catch (error) {
      debugPrint('Word product registration failed: $error');
    }
  }
  if (registry.plugin(wordProductId) == null &&
      await bundledWordIdentity.exists() &&
      await bundledWordKey.exists()) {
    try {
      final identity =
          jsonDecode(await bundledWordIdentity.readAsString()) as Map;
      registry.install(
        await OpenMuseWordProductPlugin.loadBundled(
          bundleFile: File(p.join(bundledWordRoot.path, 'word.mcpb')),
          trustedPublicKeyPem: await bundledWordKey.readAsString(),
          expectedKeyId: identity['keyId'] as String,
          expectedVersion: identity['version'] as String,
        ),
      );
    } catch (error) {
      debugPrint('Bundled Word product registration failed: $error');
    }
  }
  if (registry.plugin(pdfProductId) == null &&
      await bundledPdfIdentity.exists() &&
      await bundledPdfKey.exists()) {
    try {
      final identity =
          jsonDecode(await bundledPdfIdentity.readAsString()) as Map;
      registry.install(
        await OpenMusePdfProductPlugin.loadBundled(
          bundleFile: File(p.join(bundledPdfRoot.path, 'pdf.mcpb')),
          trustedPublicKeyPem: await bundledPdfKey.readAsString(),
          expectedKeyId: identity['keyId'] as String,
          expectedVersion: identity['version'] as String,
          settings: settings,
          openGeneratedDocument: openGeneratedPdfDocument,
        ),
      );
    } catch (error) {
      debugPrint('Bundled PDF product registration failed: $error');
    }
  }
  if (registry.plugin(spreadsheetProductId) == null &&
      await bundledSpreadsheetIdentity.exists() &&
      await bundledSpreadsheetKey.exists()) {
    try {
      final identity =
          jsonDecode(await bundledSpreadsheetIdentity.readAsString()) as Map;
      registry.install(
        await OpenMuseSpreadsheetProductPlugin.loadBundled(
          bundleFile: File(
            p.join(bundledSpreadsheetRoot.path, 'spreadsheet.mcpb'),
          ),
          trustedPublicKeyPem: await bundledSpreadsheetKey.readAsString(),
          expectedKeyId: identity['keyId'] as String,
          expectedVersion: identity['version'] as String,
        ),
      );
    } catch (error) {
      debugPrint('Bundled Spreadsheet product registration failed: $error');
    }
  }
  unawaited(
    Future<void>(() async {
      for (final entry in installedMcpEntries.where(
        (entry) =>
            entry.name != wordProductId &&
            entry.name != pdfProductId &&
            entry.name != spreadsheetProductId,
      )) {
        try {
          await registerMcp(entry).timeout(const Duration(seconds: 10));
        } catch (error) {
          debugPrint('MCPB activation deferred for ${entry.name}: $error');
        }
      }
    }),
  );
  final wordE2eDocument = kDebugMode && supportOverride != null
      ? Platform.environment['OPENMUSE_E2E_OPEN_DOCX']
      : null;
  if (wordE2eDocument != null) {
    final document = File(wordE2eDocument).absolute;
    if (await document.exists() && registry.plugin(wordProductId) != null) {
      controller.openResource(
        OpenMuseResource(
          uri: document.uri,
          displayName: p.basename(document.path),
          mediaType: wordDocxMediaType,
        ),
      );
    }
  }
  final pdfE2eDocument = kDebugMode && supportOverride != null
      ? Platform.environment['OPENMUSE_E2E_OPEN_PDF']
      : null;
  if (pdfE2eDocument != null) {
    final document = File(pdfE2eDocument).absolute;
    if (await document.exists() && registry.plugin(pdfProductId) != null) {
      controller.openResource(
        OpenMuseResource(
          uri: document.uri,
          displayName: p.basename(document.path),
          mediaType: 'application/pdf',
        ),
      );
    }
  }
  final spreadsheetE2eDocument = kDebugMode && supportOverride != null
      ? Platform.environment['OPENMUSE_E2E_OPEN_XLSX']
      : null;
  if (spreadsheetE2eDocument != null) {
    final document = File(spreadsheetE2eDocument).absolute;
    if (await document.exists() &&
        registry.plugin(spreadsheetProductId) != null) {
      controller.openResource(
        OpenMuseResource(
          uri: document.uri,
          displayName: p.basename(document.path),
          mediaType:
              'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
        ),
      );
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
      if (!macAppStoreDistribution) {
        await registry.activate(pairedDesktopPlugin.descriptor.id);
      }
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
      if (!macAppStoreDistribution) {
        await registry.activate('com.openmuse.plugin-market');
      }
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
        backgroundColor: Colors.white,
        body: Center(
          child: _error == null
              ? const Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    CircularProgressIndicator(strokeWidth: 2),
                    SizedBox(height: 20),
                    Text('OpenMuse 正在启动…'),
                  ],
                )
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
