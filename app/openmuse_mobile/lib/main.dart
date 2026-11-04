import 'dart:async';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_client_plugins/openmuse_client_plugins.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:openmuse_dsh_mobile_agent/openmuse_dsh_mobile_agent.dart';
import 'host/mobile_resource_extensions.dart';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_io.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';
import 'package:openmuse_speech_input/openmuse_speech_input.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path_provider/path_provider.dart';
import 'package:url_launcher/url_launcher.dart';

import 'account_deletion.dart';
import 'ai_content_reporting.dart';
import 'native_dsh_page.dart';
import 'native_desktop_shell.dart';
import 'office_viewer_screen.dart';
import 'docx_editor_screen.dart';
import 'mobile_plugins/scheduled_tasks_plugin.dart';
import 'mobile_plugins/workspace_files_plugin.dart';
import 'workbuddy/workbuddy_controller.dart';
import 'workbuddy/workbuddy_models.dart';
import 'workbuddy/workbuddy_shell.dart';
import 'workbuddy/workbuddy_theme.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(
    OpenMuseRegionBootstrap(
      store: (release) async {
        final support = await getApplicationSupportDirectory();
        return FileConfigStore(
          File('${support.path}/region-${release.id}.json'),
        );
      },
      boot: bootOpenMuseMobile,
    ),
  );
}

Future<Widget> bootOpenMuseMobile(
  DeploymentBundle deploymentBundle,
  String? initialLanguage,
) async {
  var runtimeConfig = deploymentBundle.snapshot;
  final support = await getApplicationSupportDirectory();
  final localeStore = FileConfigStore(
    File('${support.path}/locale-${runtimeConfig.storageIdentity}.json'),
  );
  final localeController = OpenMuseUiLocale(
    defaultTag: runtimeConfig.defaultLocale,
    initialTag: runtimeConfig.defaultLocale,
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

  final endpoints = MobileEndpointConfig.fromEnvironment(config: runtimeConfig);
  const secureValues = FlutterSecureValueStore();
  final deviceId = await _persistentMobileDeviceId(secureValues);
  final authentication = GoTrueAuthenticationController(
    networkGuard: runtime.checkNetwork,
    requirePersistedSession: true,
    provider: GoTrueHttpClient(
      networkGuard: runtime.checkNetwork,
      config: GoTrueClientConfig(
        origin: endpoints.gotrueOrigin,
        allowInsecureLoopback: endpoints.allowInsecureLoopback,
      ),
    ),
    store: SecureAuthSessionStore(
      values: secureValues,
      key: runtimeConfig.authSessionStorageKey,
    ),
    bootstrapper: AppFlowyAccountBootstrapper(
      networkGuard: runtime.checkNetwork,
      cloudOrigin: endpoints.cloudOrigin,
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
    ),
  );
  final authPlugin = OpenMuseGoTruePlugin(
    authentication: authentication,
    cloudLabel: endpoints.cloudOrigin.toString(),
    termsUri: runtimeConfig.linkUri(LinkId.terms, locale: localeController.tag),
    privacyUri: runtimeConfig.linkUri(
      LinkId.privacy,
      locale: localeController.tag,
    ),
  );
  final cloudPlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authentication,
    cloudOrigin: endpoints.cloudOrigin,
    deviceId: deviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
  );
  final deviceDirectory = AccountDeviceDirectoryController(
    authentication: authentication,
    client: AccountDeviceDirectoryClient(
      cloudOrigin: endpoints.cloudOrigin,
      accessToken: authentication.accessToken,
      refreshAccessToken: () => authentication.accessToken(forceRefresh: true),
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
    ),
    registration: () => AccountDeviceRegistration(
      deviceRef: deviceId,
      displayName: 'OpenMuse Mobile',
      platform: defaultTargetPlatform.name,
      kind: AccountDeviceKind.mobile,
      capabilities: const {'workspace.cloud', 'paired-desktop.client'},
    ),
  );
  final pairedPlugin = OpenMusePairedDesktopMobilePlugin.discovered(
    directory: deviceDirectory,
    accessToken: authentication.accessToken,
    deviceRef: deviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
    allowInsecurePrivateNetworkForTesting:
        endpoints.allowInsecurePrivateNetworkForTesting,
  );
  const configuredSpeechModelDirectory = String.fromEnvironment(
    'OPENMUSE_SPEECH_MODEL_DIR',
  );
  final speechModelDirectory = configuredSpeechModelDirectory.isNotEmpty
      ? configuredSpeechModelDirectory
      : '${(await getApplicationSupportDirectory()).path}/speech-model';
  final speechPlugin = OpenMuseSpeechInputPlugin(
    localModel: SherpaZipformerConfig.zh14m(speechModelDirectory),
  );
  final mobileMarketStore = MobilePluginStore(
    Directory(
      '${(await getApplicationSupportDirectory()).path}/mobile-plugins',
    ),
    allowInstall: () => runtime.presentation.isEnabled(AppFeature.pluginMarket),
    allowDownloadedPlugins: false,
  );
  await mobileMarketStore.restore();
  const marketSource = BundledMobileMarketSource(officialRelease: true);
  final launchRecord =
      jsonDecode(await rootBundle.loadString('assets/launch/slides.json'))
          as Map;
  final launchEntry = RemoteMcpEntry.fromRecord(launchRecord);
  final launchPackages = Map<String, String>.unmodifiable({
    '${launchEntry.name}@${launchEntry.version}': launchEntry.packageSha256!,
  });
  final mobileMarketPlugin = OpenMuseMobilePluginMarket(
    mobileMarketStore,
    source: marketSource,
    allowedMcpPackages: launchPackages,
    registryProviders: await () async {
      const registryOverride = String.fromEnvironment(
        'OPENMUSE_MCP_REGISTRY_URL',
      );
      final configured = !kDebugMode || registryOverride.isEmpty
          ? runtimeConfig.serviceUri(ServiceId.pluginRegistry)
          : Uri.parse(registryOverride);
      final providers = McpRegistryProviders(
        file: File(
          '${(await getApplicationSupportDirectory()).path}/mcp-registries.json',
        ),
        defaultOrigin: configured,
      );
      await providers.load();
      return providers;
    }(),
  );
  // This reviewed offline plugin is shipped with the app. It is installed
  // through the same digest and archive checks as other MCP bundles.
  final launchMarket = RemoteMcpMarket(
    runtimeConfig.serviceUri(ServiceId.pluginRegistry),
    File('${support.path}/mcp-remote-installations.json'),
    allowedPackages: launchPackages,
  );
  final launchMarker = File(
    '${support.path}/launch-slides-${launchEntry.version}.seeded',
  );
  if (!await launchMarker.exists()) {
    final data = await rootBundle.load('assets/launch/slides.mcpb');
    await launchMarket.install(
      launchEntry,
      bundledArtifact: data.buffer.asUint8List(
        data.offsetInBytes,
        data.lengthInBytes,
      ),
    );
    await launchMarker.writeAsString(launchEntry.version, flush: true);
  }
  const speechTestAudio = String.fromEnvironment('OPENMUSE_SPEECH_TEST_AUDIO');
  const distributionChannel = String.fromEnvironment(
    'OPENMUSE_DISTRIBUTION_CHANNEL',
  );
  OpenMuseUpdateController? updates;
  // The current Android update feed only accepts a Google Play listing. A
  // Chinese-market build must rely on its own store until a channel-specific
  // verified update URL is implemented, rather than sending users to Play.
  if (distributionChannel != 'android-cn') {
    try {
      updates = await bootOpenMuseUpdates(config: runtimeConfig);
    } catch (error) {
      debugPrint('App update check unavailable: $error');
    }
  }
  return OpenMuseRuntimeHost(
    runtime: runtime,
    child: OpenMuseMobileApplication(
      localeController: localeController,
      runtime: runtime,
      updates: updates,
      authenticationPlugin: authPlugin,
      cloudWorkspacePlugin: cloudPlugin,
      pairedDesktopPlugin: pairedPlugin,
      speechInputPlugin: speechPlugin,
      mobileMarketPlugin: mobileMarketPlugin,
      debugSpeechSource:
          !kReleaseMode &&
              const bool.fromEnvironment('OPENMUSE_SPEECH_E2E_AUTORUN') &&
              speechTestAudio.isNotEmpty
          ? const SpeechFileSource(speechTestAudio)
          : null,
    ),
  );
}

Future<String> _persistentMobileDeviceId(SecureValueStore values) async {
  const key = 'openmuse.device.id.v1';
  final existing = await values.read(key);
  if (existing != null &&
      existing.startsWith('mobile.') &&
      existing.length <= 160) {
    return existing;
  }
  final random = Random.secure();
  final suffix = List.generate(
    20,
    (_) => random.nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();
  final value = 'mobile.$suffix';
  await values.write(key, value);
  return value;
}

@immutable
final class MobileEndpointConfig {
  const MobileEndpointConfig({
    required this.gotrueOrigin,
    required this.cloudOrigin,
    required this.allowInsecureLoopback,
    required this.pairedDesktopOrigin,
    required this.allowInsecurePrivateNetworkForTesting,
  });

  factory MobileEndpointConfig.fromEnvironment({
    required RuntimeConfig config,
  }) => MobileEndpointConfig(
    gotrueOrigin: Uri.parse(
      _mobileEndpoint(
        key: 'OPENMUSE_GOTRUE_ORIGIN',
        dartDefine: const String.fromEnvironment('OPENMUSE_GOTRUE_ORIGIN'),
        // A physical Mobile device resolves loopback to itself. Local backend
        // development remains available through an explicit dart-define.
        debugDefault: config.serviceUri(ServiceId.auth).toString(),
        releaseDefault: config.serviceUri(ServiceId.auth).toString(),
      ),
    ),
    cloudOrigin: Uri.parse(
      _mobileEndpoint(
        key: 'OPENMUSE_CLOUD_ORIGIN',
        dartDefine: const String.fromEnvironment('OPENMUSE_CLOUD_ORIGIN'),
        debugDefault: config.serviceUri(ServiceId.cloud).toString(),
        releaseDefault: config.serviceUri(ServiceId.cloud).toString(),
      ),
    ),
    allowInsecureLoopback: _mobileBool(
      key: 'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
      dartDefine: const String.fromEnvironment(
        'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
      ),
      fallback: !kReleaseMode,
    ),
    pairedDesktopOrigin: Uri.parse(
      const String.fromEnvironment(
        'OPENMUSE_PAIRED_DESKTOP_ORIGIN',
        defaultValue: 'http://127.0.0.1:13180',
      ),
    ),
    allowInsecurePrivateNetworkForTesting:
        !kReleaseMode &&
        const bool.fromEnvironment(
          'OPENMUSE_ALLOW_INSECURE_PRIVATE_NETWORK_FOR_TESTING',
          defaultValue: false,
        ),
  );

  final Uri gotrueOrigin;
  final Uri cloudOrigin;
  final bool allowInsecureLoopback;
  final Uri pairedDesktopOrigin;
  final bool allowInsecurePrivateNetworkForTesting;
}

String _mobileEndpoint({
  required String key,
  required String dartDefine,
  required String debugDefault,
  required String releaseDefault,
}) {
  final compiled = dartDefine.trim();
  if (kDebugMode && compiled.isNotEmpty) return compiled;
  final environment = kDebugMode ? Platform.environment[key]?.trim() : null;
  if (environment != null && environment.isNotEmpty) return environment;
  return kReleaseMode ? releaseDefault : debugDefault;
}

bool _mobileBool({
  required String key,
  required String dartDefine,
  required bool fallback,
}) {
  final raw =
      (dartDefine.trim().isNotEmpty
              ? dartDefine
              : kDebugMode
              ? Platform.environment[key]
              : null)
          ?.trim()
          .toLowerCase();
  if (raw == 'true' || raw == '1') return true;
  if (raw == 'false' || raw == '0') return false;
  return fallback;
}

final class OpenMuseMobileApplication extends StatefulWidget {
  const OpenMuseMobileApplication({
    super.key,
    required this.authenticationPlugin,
    required this.cloudWorkspacePlugin,
    required this.pairedDesktopPlugin,
    this.speechInputPlugin,
    this.mobileMarketPlugin,
    this.remoteWorkbenchPlugin,
    this.officeEngine,
    this.debugSpeechSource,
    this.updates,
    this.localeController,
    this.runtime,
  });

  final OpenMuseUiLocale? localeController;
  final OpenMuseClientRuntime? runtime;
  final OpenMuseGoTruePlugin authenticationPlugin;
  final OpenMuseCloudWorkspacePlugin cloudWorkspacePlugin;
  final OpenMusePairedDesktopMobilePlugin pairedDesktopPlugin;
  final OpenMuseSpeechInputPlugin? speechInputPlugin;
  final OpenMuseMobilePluginMarket? mobileMarketPlugin;
  final OpenMuseRemoteWorkbenchPlugin? remoteWorkbenchPlugin;
  final OfficeEnginePort? officeEngine;
  final SpeechAudioSource? debugSpeechSource;
  final OpenMuseUpdateController? updates;

  @override
  State<OpenMuseMobileApplication> createState() =>
      OpenMuseMobileApplicationState();
}

final class OpenMuseMobileApplicationState
    extends State<OpenMuseMobileApplication>
    with WidgetsBindingObserver {
  late final OpenMusePluginRegistry _plugins;
  late final _contentReports = AiContentReportClient(
    origin: widget.cloudWorkspacePlugin.service.baseUri,
    accessToken: () => widget.authenticationPlugin.authentication.accessToken(),
    currentAccount: () =>
        widget.authenticationPlugin.authentication.snapshot.identity?.subject,
    networkGuard: () => widget.runtime?.checkNetwork(),
    allowInsecureLoopback: !kReleaseMode,
  );
  late final List<OpenMusePlugin> _compiledClientPlugins =
      createOpenMuseClientPlugins();
  late final OpenMuseRemoteWorkbenchPlugin _workbench;
  late final MobileScheduledTasksPlugin _scheduledTasks;
  late final MobileWorkspaceFilesPlugin _workspaceFiles;
  late final OpenMuseDshMobileAgent _agent;
  late final MobileResourceExtensions _extensions;
  String? _extensionIdentity, _extensionGrant;
  void _extensionScopeChanged() {
    final identity =
        widget.authenticationPlugin.authentication.snapshot.identity?.subject;
    final grant =
        widget.pairedDesktopPlugin.controller.snapshot.connection?.grantRef;
    if (identity != _extensionIdentity || grant != _extensionGrant) {
      _extensionIdentity = identity;
      _extensionGrant = grant;
      unawaited(_extensions.clear());
    }
  }

  Future<bool> _approveExtension(String title, String details) async {
    final localeStrings = _locale.strings;

    final navigationContext = _navigatorKey.currentContext;
    if (navigationContext == null || !navigationContext.mounted) return false;
    return await showDialog<bool>(
          context: navigationContext,
          builder: (dialog) => AlertDialog(
            title: Text(title),
            content: Text(details),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(dialog, false),
                child: Text((localeStrings.mobileCancelCda6010b())),
              ),
              FilledButton(
                onPressed: () => Navigator.pop(dialog, true),
                child: Text((localeStrings.mobileAllow295ba601())),
              ),
            ],
          ),
        ) ??
        false;
  }

  final _navigatorKey = GlobalKey<NavigatorState>();
  final Set<String> _registeredMobilePlugins = {};
  Future<void> _marketSync = Future<void>.value();
  late final _locale = widget.localeController ?? OpenMuseUiLocale();
  ThemeMode _themeMode = ThemeMode.system;
  static const _themeKey = 'openmuse.mobile.theme-mode.v1';

  Future<void> _restoreThemeMode() async {
    try {
      final saved = await const FlutterSecureValueStore().read(_themeKey);
      final mode = ThemeMode.values.where((item) => item.name == saved);
      if (mounted && mode.isNotEmpty) setState(() => _themeMode = mode.first);
    } on Object {
      // Preferences failure should never block app launch.
    }
  }

  void _setThemeMode(ThemeMode mode) {
    setState(() => _themeMode = mode);
    unawaited(const FlutterSecureValueStore().write(_themeKey, mode.name));
  }

  @visibleForTesting
  List<String> get installedPluginIds =>
      _plugins.descriptors.map((descriptor) => descriptor.id).toList();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    unawaited(_restoreThemeMode());
    _workbench =
        widget.remoteWorkbenchPlugin ?? OpenMuseRemoteWorkbenchPlugin();
    _scheduledTasks = MobileScheduledTasksPlugin(strings: _locale.strings);
    _workspaceFiles = MobileWorkspaceFilesPlugin(strings: _locale.strings);
    _workspaceFiles.onOpenResource = _openWorkspaceResource;
    unawaited(
      _localMcpMarket()
          .then((market) => market.refreshInstalledCapabilities())
          .catchError((Object error) {
            debugPrint('Installed plugin capability refresh unavailable');
          }),
    );
    _plugins =
        OpenMusePluginRegistry(
            context: OpenMusePluginContext(
              localizations: OpenMusePluginLocalizationAdapter(_locale),
              configuration: widget.runtime == null
                  ? null
                  : OpenMusePluginConfigurationAdapter(widget.runtime!),
              executeHostCommand: (_, _) async => null,
            ),
          )
          ..install(widget.authenticationPlugin)
          ..install(widget.cloudWorkspacePlugin)
          ..install(widget.pairedDesktopPlugin)
          ..install(_workbench)
          ..install(_scheduledTasks)
          ..install(_workspaceFiles);
    _agent = OpenMuseDshMobileAgent(
      context: () => _navigatorKey.currentContext,
      connection: () =>
          widget.pairedDesktopPlugin.controller.snapshot.connection,
    );
    _plugins.install(_agent);
    for (final plugin in _compiledClientPlugins) {
      _plugins.install(plugin);
    }
    _extensions = MobileResourceExtensions(
      market: _localMcpMarket,
      contributors: () => [
        for (final d in _plugins.descriptors)
          if (_plugins.plugin(d.id)
              case final OpenMuseToolContributor contributor)
            (id: d.id, plugin: contributor),
      ],
      approve: _approveExtension,
    );
    _extensionIdentity =
        widget.authenticationPlugin.authentication.snapshot.identity?.subject;
    _extensionGrant =
        widget.pairedDesktopPlugin.controller.snapshot.connection?.grantRef;
    widget.authenticationPlugin.authentication.addListener(
      _extensionScopeChanged,
    );
    widget.pairedDesktopPlugin.controller.addListener(_extensionScopeChanged);
    final speech = widget.speechInputPlugin;
    final market = widget.mobileMarketPlugin;
    if (speech != null) _plugins.install(speech);
    if (market != null) _plugins.install(market);
    market?.store.addListener(_onMobileMarketChanged);
    _onMobileMarketChanged();
    unawaited(() async {
      await _plugins.activate(widget.authenticationPlugin.descriptor.id);
      if (speech != null) await _plugins.activate(speech.descriptor.id);
      if (market != null) await _plugins.activate(market.descriptor.id);
      await _plugins.activate(_workbench.descriptor.id);
      if (market == null ||
          await market.builtInPreferences.isEnabled(
            _scheduledTasks.descriptor.id,
          )) {
        await _plugins.activate(_scheduledTasks.descriptor.id);
      }
      if (market == null ||
          await market.builtInPreferences.isEnabled(
            _workspaceFiles.descriptor.id,
          )) {
        await _plugins.activate(_workspaceFiles.descriptor.id);
      }
      await _plugins.activate(widget.cloudWorkspacePlugin.descriptor.id);
      await _plugins.activate(widget.pairedDesktopPlugin.descriptor.id);
      for (final plugin in _compiledClientPlugins) {
        if (market == null ||
            !mobilePluginCanBeManaged(plugin) ||
            await market.builtInPreferences.isEnabled(plugin.descriptor.id)) {
          await _plugins.activate(plugin.descriptor.id);
        }
      }
    }());
  }

  @override
  void didChangeLocales(List<Locale>? locales) => _locale.updateSystemLocales(
    (locales ?? []).map((v) => v.toLanguageTag()),
  );

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    widget.mobileMarketPlugin?.store.removeListener(_onMobileMarketChanged);
    widget.authenticationPlugin.authentication.removeListener(
      _extensionScopeChanged,
    );
    widget.pairedDesktopPlugin.controller.removeListener(
      _extensionScopeChanged,
    );
    _extensions.dispose();
    unawaited(_agent.deactivate());
    unawaited(
      _marketSync
          .then(
            (_) => Future.wait([
              _plugins.deactivate(widget.cloudWorkspacePlugin.descriptor.id),
              _plugins.deactivate(widget.pairedDesktopPlugin.descriptor.id),
              _plugins.deactivate(widget.authenticationPlugin.descriptor.id),
              if (widget.speechInputPlugin case final speech?)
                _plugins.deactivate(speech.descriptor.id),
              if (widget.mobileMarketPlugin case final market?)
                _plugins.deactivate(market.descriptor.id),
              for (final id in _registeredMobilePlugins)
                _plugins.deactivate(id),
              _plugins.deactivate(_scheduledTasks.descriptor.id),
              _plugins.deactivate(_workspaceFiles.descriptor.id),
              _plugins.deactivate(_workbench.descriptor.id),
              for (final plugin in _compiledClientPlugins)
                _plugins.deactivate(plugin.descriptor.id),
            ]),
          )
          .whenComplete(() {
            _scheduledTasks.dispose();
            _workspaceFiles.dispose();
            for (final id in _registeredMobilePlugins) {
              final plugin = _plugins.plugin(id);
              if (plugin is OpenMuseWebViewInstalledPlugin) plugin.dispose();
            }
            _plugins.dispose();
          }),
    );
    _locale.dispose();
    widget.updates?.dispose();
    super.dispose();
  }

  void _onMobileMarketChanged() {
    _marketSync = _marketSync
        .then((_) async {
          final store = widget.mobileMarketPlugin?.store;
          if (store == null) return;
          final current = {
            for (final bundle in store.installedBundles) bundle.id: bundle,
          };
          for (final stale in _registeredMobilePlugins.difference(
            current.keys.toSet(),
          )) {
            final removed = _plugins.plugin(stale);
            await _plugins.uninstall(stale);
            if (removed is OpenMuseWebViewInstalledPlugin) removed.dispose();
            _registeredMobilePlugins.remove(stale);
          }
          for (final bundle in current.values) {
            final existing = _plugins.plugin(bundle.id);
            if (existing != null &&
                (existing.descriptor.version != bundle.version ||
                    (bundle.isWebView !=
                        (existing is OpenMuseWebViewInstalledPlugin)))) {
              await _plugins.uninstall(bundle.id);
              if (existing is OpenMuseWebViewInstalledPlugin) {
                existing.dispose();
              }
              _registeredMobilePlugins.remove(bundle.id);
            }
            if (!_registeredMobilePlugins.contains(bundle.id)) {
              _plugins.install(
                bundle.isWebView
                    ? OpenMuseWebViewInstalledPlugin(
                        bundle,
                        bundle.manifest.name,
                      )
                    : OpenMuseQuickJsInstalledPlugin(bundle, bundle.id),
              );
              _registeredMobilePlugins.add(bundle.id);
            }
            if (store.isEnabled(bundle.id)) {
              await _plugins.activate(bundle.id);
            } else {
              await _plugins.deactivate(bundle.id);
            }
          }
        })
        .catchError((Object error, StackTrace stack) {
          debugPrint('Mobile plugin registry sync failed: $error\n$stack');
        });
  }

  Future<RemoteMcpMarket> _localMcpMarket() async {
    final provider = widget.mobileMarketPlugin?.registryProviders?.selected;
    if (provider == null) {
      throw StateError('No MCP Registry provider is configured');
    }
    final support = await getApplicationSupportDirectory();
    return RemoteMcpMarket(
      provider,
      File('${support.path}/mcp-remote-installations.json'),
      allowedPackages: widget.mobileMarketPlugin?.allowedMcpPackages,
    );
  }

  Future<bool> _openWorkspaceResource(
    OpenMuseResource resource,
    Future<Map<String, Object?>> Function() read,
    ValueChanged<String> reportStep,
  ) async {
    final localeStrings = _locale.strings;

    final payload = await read();
    if (payload['bytes'] is! Uint8List) {
      throw FormatException(
        (localeStrings.mobileUnableToReadTheFile8c868ffb()),
      );
    }
    final bytes = payload['bytes'] as Uint8List;
    if (bytes.length > 64 * 1024 * 1024) {
      throw FormatException(
        (localeStrings.mobileTheFileExceedsThe64MibLimitAd41f580()),
      );
    }
    final mediaType =
        ((payload['mediaType'] as String?) ??
                resource.mediaType ??
                'application/octet-stream')
            .split(';')
            .first
            .trim()
            .toLowerCase();
    resource = OpenMuseResource(
      uri: resource.uri,
      displayName: resource.displayName,
      mediaType: mediaType,
      reference: resource.reference,
    );
    reportStep((localeStrings.mobileFindingAnInstalledEditor78027f4d()));
    final market = await _localMcpMarket();
    final candidates = <RemoteMcpEntry>[];
    for (final entry in await market.installedEntries()) {
      if (!market.hasOfflineUi(entry) &&
          !market.nativeMediaTypes.containsKey(entry.name)) {
        continue;
      }
      final types = await market.resourceMediaTypes(entry);
      if (resource.mediaType != null && types.contains(resource.mediaType)) {
        candidates.add(entry);
      }
    }
    final navigationContext = _navigatorKey.currentContext;
    if (navigationContext == null || !navigationContext.mounted) return false;
    RemoteMcpEntry? editor;
    final saved = await const FlutterSecureValueStore().read(
      'openmuse.mobile.default-editor.${resource.extension}',
    );
    if (!navigationContext.mounted) return false;
    for (final candidate in candidates) {
      if (candidate.name == saved) editor = candidate;
    }
    if (editor == null && candidates.length == 1) editor = candidates.single;
    if (editor == null && candidates.length > 1) {
      editor = await showDialog<RemoteMcpEntry>(
        context: navigationContext,
        builder: (dialog) => AlertDialog(
          title: Text(
            (localeStrings.mobileWhichEditorShouldOpenBd34ddac(
              value1: resource.displayName,
            )),
          ),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final candidate in candidates)
                ListTile(
                  title: Text(candidate.title),
                  onTap: () => Navigator.pop(dialog, candidate),
                  trailing: IconButton(
                    tooltip: (localeStrings
                        .mobileSetAsTheDefaultEditor310a2582()),
                    icon: const Icon(Icons.push_pin_outlined),
                    onPressed: () async {
                      await const FlutterSecureValueStore().write(
                        'openmuse.mobile.default-editor.${resource.extension}',
                        candidate.name,
                      );
                      if (dialog.mounted) Navigator.pop(dialog, candidate);
                    },
                  ),
                ),
            ],
          ),
        ),
      );
      if (editor == null) return false;
    }
    if (editor == null) {
      await const MethodChannel('io.openmuse/resource_open').invokeMethod<void>(
        'open',
        {'name': resource.displayName, 'mediaType': mediaType, 'bytes': bytes},
      );
      return true;
    }
    reportStep((localeStrings.mobilePreparingTheLocalEditor139ca003()));
    final appHtml = await market.mcpAppDocument(editor);
    if (!mounted) return false;
    reportStep((localeStrings.mobileOpeningTheLocalEditor59b2af8c()));
    await _navigatorKey.currentState?.push<void>(
      MaterialPageRoute(
        settings: const RouteSettings(name: openMuseResourceRouteName),
        builder: (_) => RemoteMcpOfflineAppPage(
          title: resource.displayName,
          digest: editor!.packageSha256!.toLowerCase(),
          fileName: resource.displayName,
          mediaType: mediaType,
          fileBytes: bytes,
          appHtml: appHtml,
        ),
      ),
    );
    return true;
  }

  Future<void> _getUpdate(OpenMuseRelease release) async {
    await launchUrl(release.downloadUri, mode: LaunchMode.externalApplication);
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: _locale,
    builder: (context, _) => OpenMuseUiLocaleScope(
      controller: _locale,
      child: MaterialApp(
        navigatorKey: _navigatorKey,
        debugShowCheckedModeBanner: false,
        title: 'OpenMuse AI',
        locale: _locale.locale,
        theme: workBuddyTheme(brightness: Brightness.light),
        darkTheme: workBuddyTheme(brightness: Brightness.dark),
        themeMode: _themeMode,
        localizationsDelegates: [
          OpenMuseStringsDelegate(_locale.catalog),
          ...OpenMuseAuthLocalizations.localizationsDelegates,
        ],
        supportedLocales: _locale.supportedLocales,
        builder: (context, child) => AiContentReportScope(
          client: _contentReports,
          child: MobileResourceExtensionLayer(
            controller: _extensions,
            child: OpenMuseRuntimeNotice(
              child: child ?? const SizedBox.shrink(),
            ),
          ),
        ),
        home: OpenMuseStartupUpdatePrompt(
          controller: widget.updates,
          onUpdate: _getUpdate,
          child: _WorkBuddyHost(
            plugins: _plugins,
            canOpenResource: (record) async {
              final market = await _localMcpMarket();
              for (final entry in await market.installedEntries()) {
                if ((market.hasOfflineUi(entry) ||
                        market.nativeMediaTypes.containsKey(entry.name)) &&
                    (await market.resourceMediaTypes(
                      entry,
                    )).contains(record.mediaType)) {
                  return true;
                }
              }
              return false;
            },
            openResource: (record, bytes) => _openWorkspaceResource(
              OpenMuseResource(
                uri: Uri(scheme: 'resource', path: '/${record.name}'),
                displayName: record.name,
                mediaType: record.mediaType,
                reference: record.reference(),
              ),
              () async => {
                'bytes': Uint8List.fromList(bytes),
                'mediaType': record.mediaType,
              },
              (_) {},
            ),
            themeMode: _themeMode,
            onThemeMode: _setThemeMode,
            authentication: widget.authenticationPlugin.authentication,
            cloudLabel: widget.authenticationPlugin.cloudLabel,
            cloudService: widget.cloudWorkspacePlugin.service,
            pairedDesktop: widget.pairedDesktopPlugin.controller,
            speechRecognition: widget.speechInputPlugin,
            officeEngine: widget.officeEngine,
            debugSpeechSource: widget.debugSpeechSource,
            mobileMarketPlugin: widget.mobileMarketPlugin,
          ),
        ),
      ),
    ),
  );
}

final class _WorkBuddyHost extends StatefulWidget {
  const _WorkBuddyHost({
    required this.plugins,
    this.canOpenResource,
    this.openResource,
    required this.themeMode,
    required this.onThemeMode,
    required this.authentication,
    required this.cloudService,
    required this.pairedDesktop,
    this.speechRecognition,
    this.cloudLabel,
    this.officeEngine,
    this.debugSpeechSource,
    this.mobileMarketPlugin,
  });

  final OpenMuseAuthenticationController authentication;
  final OpenMusePluginRegistry plugins;
  final Future<bool> Function(ResourceRecord)? canOpenResource;
  final Future<bool> Function(ResourceRecord, List<int>)? openResource;
  final ThemeMode themeMode;
  final ValueChanged<ThemeMode> onThemeMode;
  final String? cloudLabel;
  final AppFlowyCloudWorkspaceService cloudService;
  final PairedDesktopMobileController pairedDesktop;
  final SpeechRecognitionPort? speechRecognition;
  final OfficeEnginePort? officeEngine;
  final SpeechAudioSource? debugSpeechSource;
  final OpenMuseMobilePluginMarket? mobileMarketPlugin;

  @override
  State<_WorkBuddyHost> createState() => _WorkBuddyHostState();
}

final class _WorkBuddyHostState extends State<_WorkBuddyHost>
    with WidgetsBindingObserver {
  WorkBuddyController _buddy = WorkBuddyController()..selectTab(WbTab.home);
  String? _accountSubject;
  final AcceptanceDesktop? _acceptance = debugAcceptanceDesktop();
  late OpenMuseHostComposition _composition;
  NativeResourceCenter? _resources;
  Object? _resourceFailure;
  int _resourceEpoch = 0;
  Future<void> _resourceWork = Future.value();
  Future<void> _openResources() async {
    final epoch = ++_resourceEpoch,
        scope = _accountSubject ?? 'signed-out-local';
    final previous = _resources;
    _resources = null;
    _resourceFailure = null;
    if (mounted) setState(() {});
    final work = _resourceWork.catchError((Object _) {}).then((_) async {
      await previous?.close();
      if (!mounted || epoch != _resourceEpoch) return;
      try {
        const values = FlutterSecureValueStore();
        var device = await values.read('openmuse.resource.device.v1');
        if (device == null) {
          device = opaqueResourceId('mobile');
          await values.write('openmuse.resource.device.v1', device);
        }
        final resources = await NativeResourceCenter.open(
          seedWelcomeResources: true,
          scope: scope,
          deviceRef: device,
          deviceLabel: Platform.isIOS ? 'iPhone' : 'Android',
          registry: widget.plugins,
          canOpenLegacy: widget.canOpenResource,
          openLegacy: widget.openResource,
        );
        if (!mounted || epoch != _resourceEpoch) {
          await resources.close();
          return;
        }
        setState(() {
          _resources = resources;
          _resourceFailure = null;
        });
        _resourcePairChanged();
      } on Object catch (error) {
        if (mounted && epoch == _resourceEpoch) {
          setState(() => _resourceFailure = error);
        }
      }
    });
    _resourceWork = work;
    await work;
  }

  void _resourcePairChanged() {
    final resources = _resources;
    if (resources == null) return;
    final connection = widget.pairedDesktop.snapshot.connection;
    if (connection == null) {
      unawaited(resources.disconnect());
      return;
    }
    final origin = Uri.parse(connection.session.origin);
    unawaited(
      resources
          .connect(
            key: connection.grantRef,
            peerRef: connection.deviceRef,
            endpoint: origin.resolve('/openmuse/resource-center/v1'),
            headers: () => {
              HttpHeaders.cookieHeader:
                  'OpenMuse-Paired=${connection.grantRef}',
            },
            allowPrivateNetwork:
                connection.session.allowInsecurePrivateNetworkForTesting,
          )
          .catchError((Object error) {
            if (mounted) {
              debugPrint(
                'Resource peer unavailable: ${error is ResourceFailure ? error.code : error.runtimeType}',
              );
            }
          }),
    );
  }

  @override
  void initState() {
    super.initState();
    _accountSubject = widget.authentication.snapshot.identity?.subject;
    widget.pairedDesktop.addListener(_resourcePairChanged);
    unawaited(_openResources());
    WidgetsBinding.instance.addObserver(this);
    widget.authentication.addListener(_onAuth);
    widget.mobileMarketPlugin?.store.addListener(_onMarketStoreChanged);
    _recompose();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _buddy.attachStrings(stringsOf(context));
    _recompose();
  }

  @override
  void didUpdateWidget(covariant _WorkBuddyHost oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.authentication != widget.authentication) {
      oldWidget.authentication.removeListener(_onAuth);
      widget.authentication.addListener(_onAuth);
    }
    if (oldWidget.mobileMarketPlugin?.store !=
        widget.mobileMarketPlugin?.store) {
      oldWidget.mobileMarketPlugin?.store.removeListener(_onMarketStoreChanged);
      widget.mobileMarketPlugin?.store.addListener(_onMarketStoreChanged);
    }
    _recompose();
  }

  void _onMarketStoreChanged() {
    if (mounted) setState(_recompose);
  }

  void _onAuth() {
    if (!mounted) return;
    final subject = widget.authentication.snapshot.identity?.subject;
    if (subject != _accountSubject) {
      final previous = _buddy;
      _buddy = WorkBuddyController(strings: stringsOf(context))
        ..selectTab(WbTab.home);
      _accountSubject = subject;
      unawaited(_openResources());
      widget.pairedDesktop.disconnect();
      WidgetsBinding.instance.addPostFrameCallback((_) => previous.dispose());
    }
    setState(_recompose);
  }

  void _recompose() {
    final identity = widget.authentication.snapshot.identity;
    _composition = connectedCloudMobileComposition(
      strings: _buddy.strings,
      session: identity == null
          ? const MobileAccountSession.signedOut()
          : MobileAccountSession.authenticated(identity.email),
      apiOrigin: widget.cloudService.baseUri,
      accessToken: () => widget.authentication.accessToken(),
      refreshAccessToken: () =>
          widget.authentication.accessToken(forceRefresh: true),
      service: widget.cloudService,
      officeEngine: widget.officeEngine,
      pluginStore: widget.mobileMarketPlugin?.store,
      pairedDesktop: widget.pairedDesktop,
      onSignOut: widget.authentication.signOut,
    );
  }

  Future<void> _deleteAccount() async {
    final reports = AiContentReportScope.maybeOf(context);
    final subject = widget.authentication.snapshot.identity?.subject;
    if (reports != null && subject != null) {
      try {
        await reports.eraseForAccount(account: subject);
      } on AiReportFailure {
        if (!mounted) return;
        throw AccountDeletionException(
          stringsOf(context).commonAiReportDeleteFailed(),
        );
      }
    }
    if (!mounted) return;
    await deleteOpenMuseAccount(
      strings: stringsOf(context),
      cloudOrigin: widget.cloudService.baseUri,
      accessToken: widget.authentication.accessToken,
    );
    await widget.authentication.signOut();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    widget.authentication.removeListener(_onAuth);
    widget.mobileMarketPlugin?.store.removeListener(_onMarketStoreChanged);
    widget.pairedDesktop.removeListener(_resourcePairChanged);
    _resourceEpoch++;
    final resources = _resources;
    unawaited(
      _resourceWork.catchError((Object _) {}).then((_) async {
        await resources?.close();
      }),
    );
    _buddy.dispose();
    _acceptance?.controller.dispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed) {
      unawaited(widget.pairedDesktop.directory?.reconcile());
      _resourcePairChanged();
    } else if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.detached) {
      unawaited(_resources?.disconnect());
    }
  }

  @override
  Widget build(BuildContext context) => ResourceLinkRouting(
    open: (uri) async {
      final center = _resources;
      if (center == null) return;
      try {
        await openResourcePage(
          context,
          center.controller,
          ResourceReference.fromUri(uri),
        );
      } on Object catch (error) {
        if (context.mounted) showResourceError(context, error);
      }
    },
    child: WorkBuddyShell(
      key: ValueKey('workbuddy.${_accountSubject ?? 'signed-out'}'),
      controller: _buddy,
      pluginRegistry: widget.plugins,
      resources: _resources,
      resourceFailure: _resourceFailure,
      onRetryResources: () => unawaited(_openResources()),
      themeMode: widget.themeMode,
      onThemeMode: widget.onThemeMode,
      authentication: widget.authentication,
      pairedDesktop: widget.pairedDesktop,
      deviceSelectionStore: const FlutterSecureValueStore(),
      catalog: _composition.workspaceCatalog,
      speechRecognition: widget.speechRecognition,
      debugSpeechSource: widget.debugSpeechSource,
      remoteWorkbench: _acceptance?.controller,
      mobileMarketPlugin: widget.mobileMarketPlugin,
      cloudLabel: widget.cloudLabel,
      onSignOut: widget.authentication.signOut,
      onDeleteAccount: _deleteAccount,
    ),
  );
}

/// Composition entry used by the login/bootstrap layer after it has obtained
/// an account session and a short-lived access-token provider. No credential is
/// compiled into the Mobile artifact or stored by the Cloud adapter.
OpenMuseHostComposition connectedCloudMobileComposition({
  OpenMuseStrings? strings,
  required OpenMuseSessionPort session,
  required Uri apiOrigin,
  required AccessTokenProvider accessToken,
  RefreshAccessTokenProvider? refreshAccessToken,
  String deviceId = 'mobile.openmuse',
  bool allowHttpForTesting = false,
  AppFlowyCloudWorkspaceService? service,
  OfficeEnginePort? officeEngine,
  MobilePluginStore? pluginStore,
  PairedDesktopMobileController? pairedDesktop,
  Future<void> Function()? onSignOut,
}) {
  final cloud =
      service ??
      AppFlowyCloudWorkspaceService(
        baseUri: apiOrigin,
        accessToken: accessToken,
        refreshAccessToken: refreshAccessToken,
        deviceId: deviceId,
        allowHttpForTesting: allowHttpForTesting,
      );
  return mobileComposition(
    strings: strings,
    session: session,
    cloudService: cloud,
    dshConnector: cloud,
    resources: cloud,
    resourceCatalog: cloud,
    officeCommits: cloud,
    officeEngine: officeEngine,
    pluginStore: pluginStore,
    pairedDesktop: pairedDesktop,
    onSignOut: onSignOut,
  );
}

OpenMuseHostComposition mobileComposition({
  OpenMuseStrings? strings,
  OpenMuseSessionPort session = const MobileAccountSession.signedOut(),
  CloudWorkspaceService? cloudService,
  DshRuntimeConnector? dshConnector,
  ResourceRangePort? resources,
  CloudResourceCatalogPort? resourceCatalog,
  OfficeResourceCommitPort? officeCommits,
  OfficeEnginePort? officeEngine,
  MobilePluginStore? pluginStore,
  PairedDesktopMobileController? pairedDesktop,
  Future<void> Function()? onSignOut,
}) {
  final catalog = _MobileCatalogAdapter(
    cloudService,
    pairedDesktop,
    strings: strings,
  );
  return OpenMuseHostComposition(
    platform: OpenMuseHostPlatform.mobile,
    session: session,
    workspaceCatalog: catalog,
    capabilitySnapshot: _MobileCapabilities(
      cloudService != null,
      pairedDesktop != null,
      _supportsFormat(officeEngine, OfficeFormat.word),
      _supportsFormat(officeEngine, OfficeFormat.sheet),
      _supportsFormat(officeEngine, OfficeFormat.slides) &&
          (pluginStore == null || pluginStore.isEnabled('com.openmuse.pptx')),
      _supportsFormat(officeEngine, OfficeFormat.pdf),
    ),
    workspaceBuilder: (context, workspace) {
      if (workspace.placement == WorkspacePlacement.pairedDesktop) {
        final controller = pairedDesktop;
        final connection = controller?.snapshot.connection;
        if (controller == null) {
          return Scaffold(
            body: Center(
              child: Text(
                (stringsOf(
                  context,
                ).mobileThePairedDesktopPluginIsUnavailableD368a69e()),
              ),
            ),
          );
        }
        if (connection == null) {
          return PairedDesktopConnectScreen(controller: controller);
        }
        return PairedDesktopWorkspaceScreen(connection: connection);
      }
      final record = catalog.cloudRecord(workspace.workspaceRef);
      if (record == null ||
          cloudService == null ||
          dshConnector == null ||
          resources == null) {
        return Scaffold(
          appBar: AppBar(title: Text(workspace.title)),
          body: Center(
            child: Text(
              (stringsOf(context).mobileCloudWorkspaceIsNotConnected9de62d6c()),
            ),
          ),
        );
      }
      return CloudWorkspaceScreen(
        record: record,
        service: cloudService,
        connector: dshConnector,
        resources: resources,
        resourceCatalog: resourceCatalog,
        officeEngine: officeEngine,
        pluginStore: pluginStore,
        officeCommits: officeCommits,
      );
    },
    accountDevicesBuilder: pairedDesktop?.directory == null
        ? null
        : (_) => AccountDevicesScreen(controller: pairedDesktop!),
    onSignOut: onSignOut,
  );
}

final class MobileAccountSession implements OpenMuseSessionPort {
  const MobileAccountSession.authenticated(this.accountLabel) : signedIn = true;
  const MobileAccountSession.signedOut()
    : signedIn = false,
      accountLabel = null;
  @override
  final bool signedIn;
  @override
  final String? accountLabel;
}

final class _MobileCatalogAdapter implements WorkspaceCatalogPort {
  _MobileCatalogAdapter(
    this.service,
    this.pairedDesktop, {
    OpenMuseStrings? strings,
  }) : _strings = strings ?? OpenMuseStrings.fallback;
  final OpenMuseStrings _strings;
  final CloudWorkspaceService? service;
  final PairedDesktopMobileController? pairedDesktop;
  final Map<String, CloudWorkspaceRecord> _records = {};

  CloudWorkspaceRecord? cloudRecord(String workspaceRef) =>
      _records[workspaceRef];

  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async {
    final provider = service;
    final values = provider == null
        ? const <CloudWorkspaceRecord>[]
        : await provider.listWorkspaces();
    var sessions = const <DshSessionSummary>[];
    if (provider is DshSessionCatalogPort) {
      try {
        sessions = await (provider as DshSessionCatalogPort).listSessions();
      } on Object {
        // Session presence enriches the catalog but does not own it. A DSH
        // pool outage must not hide otherwise usable Cloud workspaces.
      }
    }
    final runningByWorkspace = {
      for (final session in sessions.where((value) => value.isRunning))
        session.workspaceRef: session.sessionRef,
    };
    _records
      ..clear()
      ..addEntries(values.map((value) => MapEntry(value.workspaceRef, value)));
    final result = values
        .map(
          (value) => WorkspaceSummary(
            workspaceRef: value.workspaceRef,
            title: value.title,
            placement: WorkspacePlacement.cloud,
            writable:
                value.writable &&
                value.storageState == CloudStorageState.available,
            runningSessionRef: runningByWorkspace[value.workspaceRef],
          ),
        )
        .toList(growable: true);
    final paired = pairedDesktop;
    if (paired != null) {
      final connection = paired.snapshot.connection;
      result.add(
        WorkspaceSummary(
          workspaceRef: connection?.workspaceRef ?? 'openmuse.paired.connect',
          title:
              connection?.workspaceTitle ??
              (_strings.mobileConnectToDesktop60b27881()),
          placement: WorkspacePlacement.pairedDesktop,
          writable: false,
          runningSessionRef: connection?.session.sessionRef,
        ),
      );
    }
    return result;
  }
}

final class AccountDevicesScreen extends StatelessWidget {
  const AccountDevicesScreen({super.key, required this.controller});

  final PairedDesktopMobileController controller;

  @override
  Widget build(BuildContext context) {
    final directory = controller.directory;
    return Scaffold(
      appBar: AppBar(
        title: Text((stringsOf(context).mobileAccountDevicesB456d767())),
      ),
      body: directory == null
          ? Center(
              child: Text(
                (stringsOf(
                  context,
                ).mobileTheDeviceDirectoryIsUnavailableF874e876()),
              ),
            )
          : ListView(
              padding: const EdgeInsets.all(20),
              children: [
                Text(
                  (stringsOf(context).mobileDevicesOnYourAccountE2737cc7()),
                  style: TextStyle(fontSize: 20),
                ),
                const SizedBox(height: 6),
                Text(
                  (stringsOf(
                    context,
                  ).mobileConnectToAnOnlineDesktopOfflineDevic84faed1b()),
                ),
                const SizedBox(height: 16),
                AccountDeviceList(
                  controller: directory,
                  excludeDeviceRef: controller.requesterDeviceRef,
                  onSelect: (device) => Navigator.of(context).push<void>(
                    MaterialPageRoute(
                      builder: (_) => PairedDesktopConnectScreen(
                        controller: controller,
                        initialDevice: device,
                      ),
                    ),
                  ),
                ),
              ],
            ),
    );
  }
}

final class PairedDesktopConnectScreen extends StatefulWidget {
  const PairedDesktopConnectScreen({
    super.key,
    required this.controller,
    this.initialDevice,
  });

  final PairedDesktopMobileController controller;
  final AccountDevice? initialDevice;

  @override
  State<PairedDesktopConnectScreen> createState() =>
      _PairedDesktopConnectScreenState();
}

final class _PairedDesktopConnectScreenState
    extends State<PairedDesktopConnectScreen> {
  AccountDevice? _selectedDevice;

  @override
  void initState() {
    super.initState();
    _selectedDevice = widget.initialDevice;
  }

  Future<void> _connect() async {
    final target = _selectedDevice;
    if (target == null) return;
    final connected = await widget.controller.connectDevice(target);
    final connection = widget.controller.snapshot.connection;
    if (connected && connection != null && mounted) {
      await Navigator.of(context).pushReplacement<void, void>(
        MaterialPageRoute(
          builder: (_) => PairedDesktopWorkspaceScreen(connection: connection),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: Text((stringsOf(context).mobileChooseAComputer7f4e6ff3())),
    ),
    body: ListenableBuilder(
      listenable: widget.controller,
      builder: (context, _) {
        final snapshot = widget.controller.snapshot;
        return ListView(
          padding: const EdgeInsets.all(24),
          children: [
            const Icon(Icons.devices_outlined, size: 48),
            const SizedBox(height: 18),
            Text(
              (stringsOf(
                context,
              ).mobileChooseAnOnlineComputerSignedInToThe0a48865d()),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 18),
            if (widget.controller.directory case final directory?)
              AccountDeviceList(
                controller: directory,
                excludeDeviceRef: widget.controller.requesterDeviceRef,
                selectedDeviceRef: _selectedDevice?.deviceRef,
                onSelect: (device) => setState(() => _selectedDevice = device),
              ),
            const SizedBox(height: 12),
            FilledButton.icon(
              key: const ValueKey('paired-desktop-connect'),
              onPressed: snapshot.connecting || _selectedDevice?.online != true
                  ? null
                  : _connect,
              icon: snapshot.connecting
                  ? const SizedBox(
                      width: 18,
                      height: 18,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.arrow_forward),
              label: Text(
                _selectedDevice == null
                    ? (stringsOf(
                        context,
                      ).mobileChooseAnOnlineComputer6c104504())
                    : (stringsOf(context).mobileOpen9cdb481b(
                        value1: _selectedDevice!.displayName,
                      )),
              ),
            ),
            if (snapshot.failureMessage case final message?) ...[
              const SizedBox(height: 16),
              Text(
                message,
                key: const ValueKey('paired-desktop-error'),
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ],
          ],
        );
      },
    ),
  );
}

final class PairedDesktopWorkspaceScreen extends StatelessWidget {
  const PairedDesktopWorkspaceScreen({super.key, required this.connection});

  final PairedDesktopConnection connection;

  @override
  Widget build(BuildContext context) =>
      NativeDesktopShell(connection: connection);
}

bool _supportsFormat(OfficeEnginePort? engine, OfficeFormat format) {
  if (engine == null) return false;
  if (engine is MultiFormatOfficeEngine) return engine.formats.contains(format);
  return format == OfficeFormat.word;
}

final class _MobileCapabilities implements CapabilitySnapshotPort {
  const _MobileCapabilities(
    this.cloudConnected,
    this.pairedDesktopConnected,
    this.docxEngineConnected,
    this.xlsxEngineConnected,
    this.pptxEngineConnected,
    this.pdfEngineConnected,
  );
  final bool cloudConnected;
  final bool pairedDesktopConnected;
  final bool docxEngineConnected;
  final bool xlsxEngineConnected;
  final bool pptxEngineConnected;
  final bool pdfEngineConnected;
  @override
  Set<String> get capabilities => {
    'resource.viewer',
    if (cloudConnected) 'workspace.cloud',
    if (cloudConnected) 'dsh.remote',
    if (pairedDesktopConnected) 'workspace.paired',
    if (pairedDesktopConnected) 'dsh.paired',
    if (docxEngineConnected) 'office.docx.engine',
    if (xlsxEngineConnected) 'office.xlsx.engine',
    if (pptxEngineConnected) 'office.pptx.engine',
    if (pdfEngineConnected) 'office.pdf.engine',
  };
}

final class CloudWorkspaceScreen extends StatefulWidget {
  const CloudWorkspaceScreen({
    super.key,
    required this.record,
    required this.service,
    required this.connector,
    required this.resources,
    this.resourceCatalog,
    this.officeEngine,
    this.pluginStore,
    this.officeCommits,
  });
  final CloudWorkspaceRecord record;
  final CloudWorkspaceService service;
  final DshRuntimeConnector connector;
  final ResourceRangePort resources;
  final CloudResourceCatalogPort? resourceCatalog;
  final OfficeEnginePort? officeEngine;
  final MobilePluginStore? pluginStore;
  final OfficeResourceCommitPort? officeCommits;

  @override
  State<CloudWorkspaceScreen> createState() => _CloudWorkspaceScreenState();
}

final class _CloudWorkspaceScreenState extends State<CloudWorkspaceScreen> {
  late final CloudWorkspaceCoordinator coordinator;
  late final Future<List<CloudResourceRecord>> opening;

  @override
  void initState() {
    super.initState();
    widget.pluginStore?.addListener(_onPluginStoreChanged);
    coordinator = CloudWorkspaceCoordinator(
      service: widget.service,
      connector: widget.connector,
      resources: widget.resources,
    );
    opening = coordinator.select(widget.record).then((_) {
      final catalog = widget.resourceCatalog;
      if (catalog == null) return const <CloudResourceRecord>[];
      return catalog.listResources(
        workspaceRef: widget.record.workspaceRef,
        revision: widget.record.revision,
        generation: coordinator.flow.generation,
      );
    });
  }

  void _onPluginStoreChanged() {
    if (mounted) setState(() {});
  }

  @override
  void dispose() {
    widget.pluginStore?.removeListener(_onPluginStoreChanged);
    super.dispose();
  }

  bool get _pptxPluginEnabled =>
      widget.pluginStore == null ||
      widget.pluginStore!.isEnabled('com.openmuse.pptx');

  bool _isDocx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.wordprocessingml.document' ||
      value.mediaType == 'application/docx';

  bool _isXlsx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' ||
      value.mediaType == 'application/xlsx';

  bool _isPptx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.presentationml.presentation' ||
      value.mediaType == 'application/pptx';

  bool _isPdf(CloudResourceRecord value) =>
      value.mediaType == 'application/pdf';

  // Cloud resources must use the injected engine and revision transaction.
  // The standalone product screens save local copies and are used only by the
  // local-file entry points, not for cloud writes.
  Future<void> _openPdf(CloudResourceRecord resource) =>
      _openViewer(resource, OfficeFormat.pdf);

  Future<void> _openSpreadsheet(CloudResourceRecord resource) =>
      _openViewer(resource, OfficeFormat.sheet);

  Future<void> _openDocx(CloudResourceRecord resource) async {
    final localeStrings = stringsOf(context);

    final engine = widget.officeEngine;
    final commits = widget.officeCommits;
    if (engine == null || commits == null || !_isDocx(resource)) return;
    try {
      final generation = coordinator.flow.generation;
      final handle = await widget.service.issueResourceHandle(
        workspaceRef: widget.record.workspaceRef,
        resourceRef: resource.resourceRef,
        revision: resource.revision,
        audience: 'openmuse-mobile-office',
        generation: generation,
      );
      if (!mounted ||
          handle.resourceRef != resource.resourceRef ||
          handle.revision != resource.revision ||
          handle.generation != generation) {
        throw StateError('late or cross-resource Office handle');
      }
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => DocxEditorScreen(
            title: resource.title,
            handle: handle,
            engine: engine,
            ranges: widget.resources,
            commits: commits,
            generation: generation,
            nowMs: () => DateTime.now().millisecondsSinceEpoch,
          ),
        ),
      );
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text((localeStrings.mobileUnableToOpenDocxE1967d9f())),
          ),
        );
      }
    }
  }

  Future<void> _openViewer(
    CloudResourceRecord resource,
    OfficeFormat format,
  ) async {
    final localeStrings = stringsOf(context);

    final engine = widget.officeEngine;
    if (engine == null ||
        (format == OfficeFormat.slides && !_pptxPluginEnabled) ||
        (format == OfficeFormat.sheet && !_isXlsx(resource)) ||
        (format == OfficeFormat.slides && !_isPptx(resource)) ||
        (format == OfficeFormat.pdf && !_isPdf(resource))) {
      return;
    }
    try {
      final generation = coordinator.flow.generation;
      final handle = await widget.service.issueResourceHandle(
        workspaceRef: widget.record.workspaceRef,
        resourceRef: resource.resourceRef,
        revision: resource.revision,
        audience: 'openmuse-mobile-office',
        generation: generation,
      );
      if (!mounted ||
          handle.resourceRef != resource.resourceRef ||
          handle.revision != resource.revision ||
          handle.generation != generation) {
        throw StateError('late or cross-resource Office handle');
      }
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => OfficeViewerScreen(
            title: resource.title,
            format: format,
            handle: handle,
            engine: engine,
            ranges: widget.resources,
            generation: generation,
            nowMs: () => DateTime.now().millisecondsSinceEpoch,
          ),
        ),
      );
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              (localeStrings.mobileUnableToOpenThisDocumentA6fd0081()),
            ),
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.record.title)),
    body: FutureBuilder<List<CloudResourceRecord>>(
      future: opening,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return const Center(child: Text('Cloud Workspace · Degraded'));
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(child: Text('Cloud Workspace · Queued'));
        }
        final session = coordinator.presentation.session;
        final resources = snapshot.data ?? const <CloudResourceRecord>[];
        return ListView(
          padding: const EdgeInsets.all(20),
          children: [
            const Text(
              'Cloud Workspace · Remote DSH',
              key: ValueKey('workspace-placement'),
            ),
            const SizedBox(height: 8),
            Text('Revision ${coordinator.flow.revision}'),
            Text(
              widget.record.storageState == CloudStorageState.available
                  ? (widget.record.writable
                        ? (stringsOf(
                            context,
                          ).mobileStorageReadAndWrite34cf9652())
                        : (stringsOf(context).mobileStorageReadOnlyE72ee688()))
                  : 'Storage · Unavailable',
            ),
            Text('DSH · ${coordinator.flow.state.name}'),
            if (session != null)
              Text(
                (stringsOf(context).mobileRemoteDshConnectedF83ee6e6()),
                key: const ValueKey('remote-dsh-session'),
              ),
            if (session != null) ...[
              const SizedBox(height: 12),
              FilledButton.icon(
                key: const ValueKey('open-remote-dsh'),
                onPressed: () => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) => NativeDshPage(
                      session: session,
                      workspaceTitle: widget.record.title,
                    ),
                  ),
                ),
                icon: const Icon(Icons.auto_awesome_outlined),
                label: Text((stringsOf(context).mobileOpenAssistant018b4e66())),
              ),
            ],
            if (resources.isNotEmpty) const Divider(),
            for (final resource in resources)
              ListTile(
                key: ValueKey('resource-${resource.resourceRef}'),
                title: Text(resource.title),
                subtitle: Text(
                  _isPptx(resource) && !_pptxPluginEnabled
                      ? (stringsOf(
                          context,
                        ).mobileInstallThePptxViewerToOpenThisFile8e07e3cd())
                      : resource.mediaType,
                ),
                enabled:
                    (_isDocx(resource) &&
                        widget.officeEngine != null &&
                        widget.officeCommits != null) ||
                    (_isXlsx(resource) && widget.officeEngine != null) ||
                    (_isPptx(resource) &&
                        widget.officeEngine != null &&
                        _pptxPluginEnabled) ||
                    (_isPdf(resource) && widget.officeEngine != null),
                onTap: () {
                  if (_isXlsx(resource)) {
                    _openSpreadsheet(resource);
                  } else if (_isPptx(resource)) {
                    _openViewer(resource, OfficeFormat.slides);
                  } else if (_isPdf(resource)) {
                    _openPdf(resource);
                  } else {
                    _openDocx(resource);
                  }
                },
              ),
          ],
        );
      },
    ),
  );
}
