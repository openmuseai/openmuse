import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';

import 'src/paired_desktop_market_client.dart';
import 'src/market_discover_view.dart';
import 'src/mcp_remote_market.dart';

export 'src/paired_desktop_market_client.dart';
export 'src/mcp_remote_market.dart'
    show
        RemoteMcpEntry,
        RemoteMcpMarket,
        RemoteMcpMarketPage,
        RemoteMcpAppPage,
        RemoteMcpOfflineAppPage,
        RemoteMcpOfflineView,
        RemoteMcpViewConnection,
        RemoteMcpResourceHostScope;

enum MobileMarketMode { discover, installed }

final class OpenMuseMobilePluginMarket implements OpenMusePlugin {
  OpenMuseMobilePluginMarket(
    this.store, {
    this.source = const BundledMobileMarketSource(),
    this.mcpRegistryOrigin,
    this.registryProviders,
    this.nativeMediaTypes = const {},
    this.allowedMcpPackages,
    this.openNativeFile,
  });
  late final builtInPreferences = MobileBuiltInPreferences(
    File('${store.directory.path}/built-in-enablement.json'),
  );
  final MobilePluginStore store;
  final MobileMarketSource source;
  final Uri? mcpRegistryOrigin;
  final McpRegistryProviders? registryProviders;
  final Map<String, Set<String>> nativeMediaTypes;
  final Map<String, String>? allowedMcpPackages;
  final Future<void> Function(BuildContext, RemoteMcpEntry, File)?
  openNativeFile;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.mobile-plugin-market',
    name: 'Mobile 插件市场',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup', 'onPanel:mobile.plugin.market'],
    panels: [
      OpenMusePanelContribution(
        id: 'mobile.plugin.market',
        region: OpenMuseSurfaceRegion.dialog,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      Center(child: Text(stringsOf(context).marketMarketNoEditor()));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'mobile.plugin.market'
      ? MobileMarketPage(
          store: store,
          source: source,
          mcpRegistryOrigin: mcpRegistryOrigin,
          registryProviders: registryProviders,
          nativeMediaTypes: nativeMediaTypes,
          allowedMcpPackages: allowedMcpPackages,
          openNativeFile: openNativeFile,
        )
      : null;

  Widget buildMarketPage({
    PairedDesktopMarketClient? desktop,
    bool embedded = false,
    bool initialMine = false,
    MobileMarketMode? mode,
    VoidCallback? onDiscover,
    OpenMusePluginRegistry? pluginRegistry,
    ValueChanged<OpenMusePlugin>? onOpenBuiltIn,
    VoidCallback? onManageWorkbench,
  }) => MobileMarketPage(
    store: store,
    source: source,
    desktop: desktop,
    embedded: embedded,
    initialMine: initialMine,
    mode: mode,
    onDiscover: onDiscover,
    builtInPreferences: builtInPreferences,
    pluginRegistry: pluginRegistry,
    onOpenBuiltIn: onOpenBuiltIn,
    onManageWorkbench: onManageWorkbench,
    mcpRegistryOrigin: mcpRegistryOrigin,
    registryProviders: registryProviders,
    nativeMediaTypes: nativeMediaTypes,
    allowedMcpPackages: allowedMcpPackages,
    openNativeFile: openNativeFile,
  );
}

/// Registry-visible adapter for a downloaded JS/A2UI artifact.
final class OpenMuseQuickJsInstalledPlugin implements OpenMusePlugin {
  OpenMuseQuickJsInstalledPlugin(this.bundle, this.title);
  final MobilePluginBundle bundle;
  final String title;

  @override
  late final descriptor = OpenMusePluginDescriptor(
    id: bundle.id,
    name: title,
    version: bundle.version,
    runtime: OpenMusePluginRuntime.quickJs,
    activationEvents: const ['onPanel:mobile.plugin'],
    panels: [
      OpenMusePanelContribution(
        id: 'mobile.plugin.${bundle.id}',
        region: OpenMuseSurfaceRegion.dialog,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      Center(child: Text(stringsOf(context).marketNoEditor()));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'mobile.plugin.${bundle.id}'
      ? _PluginRunPage(bundle: bundle, title: title, preview: false)
      : null;
}

/// A verified downloadable presentation. Its bridge is closed until a
/// separately reviewed capability dispatcher grants a specific permission.
final class OpenMuseWebViewInstalledPlugin extends ChangeNotifier
    implements OpenMusePlugin, OpenMuseMobileWorkbenchContributor {
  OpenMuseWebViewInstalledPlugin(this.bundle, this.title)
    : assert(bundle.isWebView);

  final MobilePluginBundle bundle;
  final String title;

  @override
  late final descriptor = OpenMusePluginDescriptor(
    id: bundle.id,
    name: title,
    version: bundle.version,
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: const ['onStartup'],
  );

  @override
  OpenMuseWorkbenchCard get workbenchCard => bundle.card!;

  @override
  Listenable get workbenchChanges => this;

  @override
  List<Map<String, Object?>> buildWorkbenchPreview() => bundle.preview;

  @override
  String buildMobileDetailDocument(Brightness brightness) => bundle.html!;

  @override
  Future<Object?> handleMobileDetailRequest(
    String method,
    Object? arguments,
  ) async => throw const FormatException('此插件未获 Host 能力授权');

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class MobileMarketPage extends StatefulWidget {
  const MobileMarketPage({
    super.key,
    required this.store,
    this.source = const BundledMobileMarketSource(),
    this.desktop,
    this.embedded = false,
    this.initialMine = false,
    this.mode,
    this.onDiscover,
    this.builtInPreferences,
    this.pluginRegistry,
    this.onOpenBuiltIn,
    this.onManageWorkbench,
    this.mcpRegistryOrigin,
    this.registryProviders,
    this.nativeMediaTypes = const {},
    this.allowedMcpPackages,
    this.openNativeFile,
  });
  final MobilePluginStore store;
  final MobileMarketSource source;
  final PairedDesktopMarketClient? desktop;
  final bool embedded;
  final bool initialMine;
  final MobileMarketMode? mode;
  final VoidCallback? onDiscover;
  final MobileBuiltInPreferences? builtInPreferences;
  final OpenMusePluginRegistry? pluginRegistry;
  final ValueChanged<OpenMusePlugin>? onOpenBuiltIn;
  final VoidCallback? onManageWorkbench;
  final Uri? mcpRegistryOrigin;
  final McpRegistryProviders? registryProviders;
  final Map<String, Set<String>> nativeMediaTypes;
  final Map<String, String>? allowedMcpPackages;
  final Future<void> Function(BuildContext, RemoteMcpEntry, File)?
  openNativeFile;

  @override
  State<MobileMarketPage> createState() => _MobileMarketPageState();
}

final class _MobileMarketPageState extends State<MobileMarketPage> {
  List<MobileMarketEntry>? entries;
  String? error;
  String? busy;
  late String category = mine ? '全部' : '精选';
  String query = '';
  final _search = TextEditingController();
  final _operation = ValueNotifier<String?>(null);
  final _installFailure = ValueNotifier<String?>(null);
  late bool mine =
      widget.mode == MobileMarketMode.installed || widget.initialMine;

  @override
  void initState() {
    super.initState();
    if (mine) entries = [];
    widget.store.addListener(_refresh);
    widget.pluginRegistry?.addListener(_refresh);
    unawaited(_load());
  }

  void _refresh() {
    if (mounted) setState(() {});
  }

  Future<void> _load() async {
    if (mounted) setState(() => error = null);
    try {
      final loaded = await widget.source.list();
      if (mounted)
        setState(() {
          entries = loaded;
          error = null;
        });
    } catch (exception) {
      if (mounted)
        setState(() {
          entries ??= [];
          error = stringsOf(context).marketOpenFailed(
            title: stringsOf(context).marketCategoryDiscover(),
          );
        });
    }
  }

  Future<void> _install(MobileMarketEntry entry) async {
    if (busy != null) return;
    _operation.value = entry.id;
    _installFailure.value = null;
    setState(() {
      busy = entry.id;
      error = null;
    });
    try {
      await widget.store.install(entry, await widget.source.artifact(entry));
    } catch (exception) {
      if (mounted)
        setState(() {
          error = stringsOf(context).marketOpenFailed(title: entry.name);
          _installFailure.value = error;
        });
    } finally {
      if (mounted) {
        _operation.value = null;
        setState(() => busy = null);
      }
    }
  }

  Future<void> _open(MobileMarketEntry entry, {required bool preview}) async {
    try {
      MobilePluginBundle bundle;
      if (preview) {
        final bytes = await widget.source.artifact(entry);
        final previewDirectory = await Directory.systemTemp.createTemp(
          'openmuse-plugin-preview-',
        );
        try {
          final temporary = MobilePluginStore(previewDirectory);
          await temporary.install(entry, bytes);
          bundle = temporary.installed(entry.id)!;
        } finally {
          await previewDirectory.delete(recursive: true);
        }
      } else {
        bundle =
            widget.store.installed(entry.id) ?? (throw StateError('插件尚未安装'));
      }
      if (!mounted) return;
      if (bundle.isWebView) {
        final plugin = OpenMuseWebViewInstalledPlugin(bundle, entry.name);
        try {
          await Navigator.of(context).push<void>(
            MaterialPageRoute(
              builder: (_) =>
                  MobilePluginDetailPage(plugin: plugin, contribution: plugin),
            ),
          );
        } finally {
          plugin.dispose();
        }
        return;
      }
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => _PluginRunPage(
            bundle: bundle,
            title: entry.name,
            preview: preview,
          ),
        ),
      );
    } on Object {
      if (mounted)
        setState(
          () => error = stringsOf(context).marketOpenFailed(title: entry.name),
        );
    }
  }

  Future<void> _detail(MobileMarketEntry entry) async {
    await Navigator.of(context).push<void>(
      MaterialPageRoute(
        builder: (_) => MarketPluginDetailPage(
          entry: entry,
          store: widget.store,
          operation: _operation,
          installationFailure: _installFailure,
          onShowOverview: entry.runtime == 'webview'
              ? widget.onManageWorkbench
              : null,
          onInstall: () => unawaited(_install(entry)),
          onOpen: () => unawaited(_open(entry, preview: false)),
          onPreview: () => unawaited(_open(entry, preview: true)),
          onToggle: () => unawaited(
            widget.store.setEnabled(
              entry.id,
              !widget.store.isEnabled(entry.id),
            ),
          ),
          onRemove: () => unawaited(widget.store.uninstall(entry.id)),
        ),
      ),
    );
  }

  @override
  void dispose() {
    _search.dispose();
    _operation.dispose();
    _installFailure.dispose();
    widget.store.removeListener(_refresh);
    widget.pluginRegistry?.removeListener(_refresh);
    super.dispose();
  }

  List<OpenMusePlugin> get _builtIns {
    final registry = widget.pluginRegistry;
    if (registry == null) return const [];
    return [
      for (final descriptor in registry.descriptors)
        if (registry.plugin(descriptor.id) case final OpenMusePlugin plugin)
          if (!widget.store.isInstalled(plugin.descriptor.id) &&
              (widget.mode == MobileMarketMode.installed
                  ? mobilePluginCanBeManaged(plugin)
                  : plugin is OpenMuseMobileWorkbenchContributor))
            plugin,
    ];
  }

  List<MobileMarketEntry> get _visibleEntries {
    if (!mine) return entries ?? [];
    final available = {
      for (final entry in entries ?? <MobileMarketEntry>[]) entry.id: entry,
    };
    return [
      for (final bundle in widget.store.installedBundles)
        available[bundle.id] ??
            MobileMarketEntry(
              id: bundle.id,
              name: bundle.manifest.name,
              version: bundle.version,
              description: '',
              author: '',
              platforms: [
                for (final target in bundle.manifest.compatibility)
                  if (target.status == OpenMuseTargetStatus.supported)
                    target.target.os.name,
              ],
              bundle: '',
              sha256: '',
              runtime: bundle.isWebView ? 'webview' : 'quickjs',
              permissions: bundle.manifest.requestedPermissions.toList(),
            ),
    ];
  }

  @override
  Widget build(BuildContext context) => entries == null
      ? const MarketLoadingView()
      : MarketDiscoverView(
          entries: _visibleEntries,
          onRetry: () => unawaited(_load()),
          store: widget.store,
          category: category,
          mine: mine,
          showSections: widget.mode == null,
          searchController: _search,
          showAppBar: widget.mode != null && !widget.embedded,
          onDiscover: widget.onDiscover,
          onManageBuiltIn: widget.mode == MobileMarketMode.installed
              ? (plugin) => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) => BuiltInPluginManagementPage(
                      plugin: plugin,
                      registry: widget.pluginRegistry!,
                      preferences: widget.builtInPreferences,
                      onOpen: widget.onOpenBuiltIn,
                    ),
                  ),
                )
              : null,
          query: query,
          onSection: (value) => setState(() {
            mine = value;
            category = value ? '全部' : '精选';
            query = '';
            _search.clear();
          }),
          error: error,
          busy: busy,
          onCategory: (value) => setState(() => category = value),
          onSearch: (value) => setState(() => query = value),
          onDetail: (entry) => unawaited(_detail(entry)),
          onInstall: (entry) => unawaited(_install(entry)),
          onOpen: (entry) => unawaited(_open(entry, preview: false)),
          onPreview: (entry) => unawaited(_open(entry, preview: true)),
          builtInPlugins: _builtIns,
          builtInState: (plugin) =>
              widget.pluginRegistry?.stateOf(plugin.descriptor.id),
          onOpenBuiltIn: widget.onOpenBuiltIn,
          onManageWorkbench: widget.onManageWorkbench,
          mcpSection:
              widget.registryProviders == null &&
                  widget.mcpRegistryOrigin == null
              ? null
              : RemoteMcpMarketPage(
                  key: const ValueKey('mobile-mcp-online-market'),
                  origin: widget.mcpRegistryOrigin,
                  providers: widget.registryProviders,
                  nativeMediaTypes: widget.nativeMediaTypes,
                  allowedPackages: widget.allowedMcpPackages,
                  openNativeFile: widget.openNativeFile,
                  embedded: true,
                  query: query,
                  installedOnly: mine,
                  visibleCategory: category,
                ),
          desktopSection: widget.desktop == null
              ? Padding(
                  padding: const EdgeInsets.all(14),
                  child: Text(stringsOf(context).marketConnectDesktop()),
                )
              : _DesktopMarketSection(client: widget.desktop!),
        );
}

final class _DesktopMarketSection extends StatefulWidget {
  const _DesktopMarketSection({required this.client});
  final PairedDesktopMarketClient client;

  @override
  State<_DesktopMarketSection> createState() => _DesktopMarketSectionState();
}

final class _DesktopMarketSectionState extends State<_DesktopMarketSection> {
  List<Map<String, Object?>>? plugins;
  final Map<String, Map<String, Object?>> plans = {};
  String? error;
  String? busy;

  @override
  void initState() {
    super.initState();
    unawaited(_load());
  }

  Future<void> _load() async {
    try {
      final response = await widget.client.request('list');
      final raw = response['plugins'];
      if (raw is! List) throw const FormatException('Desktop 市场目录无效');
      final loaded = [
        for (final item in raw) Map<String, Object?>.from(item as Map),
      ];
      if (mounted)
        setState(() {
          plugins = loaded;
          error = null;
        });
      for (final entry in loaded) {
        final id = entry['id'];
        if (id is! String) continue;
        try {
          final plan = await widget.client.request('plan', pluginId: id);
          if (mounted) setState(() => plans[id] = plan);
        } catch (exception) {
          if (mounted) setState(() => plans[id] = {'error': '$exception'});
        }
      }
    } catch (exception) {
      if (mounted) setState(() => error = '$exception');
    }
  }

  Future<void> _act(String operation, String id) async {
    setState(() {
      busy = id;
      error = null;
    });
    try {
      await widget.client.request(operation, pluginId: id);
      final plan = await widget.client.request('plan', pluginId: id);
      if (mounted) setState(() => plans[id] = plan);
    } catch (exception) {
      if (mounted) setState(() => error = '$exception');
    } finally {
      if (mounted) setState(() => busy = null);
    }
  }

  @override
  Widget build(BuildContext context) => Column(
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      if (error != null)
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Text(
            error!,
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          ),
        ),
      if (plugins == null && error == null)
        const Padding(
          padding: EdgeInsets.all(12),
          child: CircularProgressIndicator(),
        ),
      for (final entry in plugins ?? const <Map<String, Object?>>[])
        Container(
          margin: const EdgeInsets.only(bottom: 10),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            border: Border.all(
              color: Theme.of(context).colorScheme.outlineVariant,
            ),
            borderRadius: BorderRadius.circular(18),
          ),
          child: Padding(
            padding: const EdgeInsets.all(18),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Container(
                      width: 42,
                      height: 42,
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.primaryContainer,
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Icon(
                        Icons.account_tree_rounded,
                        color: Theme.of(context).colorScheme.primary,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            '${entry['name']}',
                            style: TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.w700,
                              color: Theme.of(context).colorScheme.onSurface,
                            ),
                          ),
                          Text(
                            'v${entry['version']} · ${entry['package']}',
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 11,
                              color: Theme.of(
                                context,
                              ).colorScheme.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const MarketPlatformBadge('Desktop'),
                  ],
                ),
                const SizedBox(height: 12),
                Text(
                  '${entry['description'] ?? stringsOf(context).marketDesktopUsage()}',
                  style: TextStyle(
                    fontSize: 13,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
                if (plans[entry['id']]?['error'] case final problem?)
                  Text('$problem'),
                if (plans[entry['id']]?['action'] case final action?) ...[
                  const SizedBox(height: 12),
                  Text(
                    plans[entry['id']]?['enabled'] == true
                        ? stringsOf(context).marketEnabled()
                        : plans[entry['id']]?['installedVersion'] != null
                        ? stringsOf(context).marketDisabled()
                        : stringsOf(context).marketNotInstalled(),
                  ),
                  const SizedBox(height: 8),
                  if (busy == entry['id'])
                    const CircularProgressIndicator()
                  else
                    Wrap(
                      spacing: 8,
                      children: [
                        if (action == 'install' || action == 'enable')
                          FilledButton(
                            onPressed: () => _act('$action', '${entry['id']}'),
                            child: Text(
                              action == 'install'
                                  ? stringsOf(context).marketInstallDesktop()
                                  : stringsOf(context).marketEnable(),
                            ),
                          ),
                        if (plans[entry['id']]?['enabled'] == true)
                          OutlinedButton(
                            onPressed: () => _act('disable', '${entry['id']}'),
                            child: Text(stringsOf(context).marketDisable()),
                          ),
                      ],
                    ),
                  Text(
                    stringsOf(context).marketRestartDesktop(),
                    style: TextStyle(
                      fontSize: 11,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                  ),
                ],
              ],
            ),
          ),
        ),
    ],
  );
}

final class _PluginRunPage extends StatefulWidget {
  const _PluginRunPage({
    required this.bundle,
    required this.title,
    required this.preview,
  });
  final MobilePluginBundle bundle;
  final String title;
  final bool preview;

  @override
  State<_PluginRunPage> createState() => _PluginRunPageState();
}

final class _PluginRunPageState extends State<_PluginRunPage> {
  late final MobilePluginSession session;
  String? error;
  bool ready = false;

  @override
  void initState() {
    super.initState();
    session = MobilePluginSession(
      pluginId: widget.bundle.id,
      source: widget.bundle.js,
      capability: (_, service, _) async =>
          throw UnsupportedError('Host capability denied: $service'),
    );
    unawaited(_start());
  }

  Future<void> _start() async {
    try {
      await session.start();
      if (mounted) setState(() => ready = true);
    } catch (exception) {
      if (mounted) setState(() => error = '$exception');
    }
  }

  @override
  void dispose() {
    unawaited(session.close());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: Text(
        widget.preview
            ? stringsOf(context).marketPreviewTitle(title: widget.title)
            : widget.title,
      ),
    ),
    body: error != null
        ? Center(child: Text(error!))
        : !ready
        ? const Center(child: CircularProgressIndicator())
        : SingleChildScrollView(
            padding: const EdgeInsets.all(16),
            child: Column(
              children: [
                ValueListenableBuilder<String?>(
                  valueListenable: session.lastError,
                  builder: (context, message, _) => message == null
                      ? const SizedBox.shrink()
                      : Text(
                          message,
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.error,
                          ),
                        ),
                ),
                MobilePluginSurface(session: session),
              ],
            ),
          ),
  );
}

bool mobilePluginCanBeManaged(OpenMusePlugin plugin) =>
    plugin is OpenMuseMobileWorkbenchContributor ||
    plugin is OpenMuseResourceHandlerContributor ||
    plugin is OpenMuseResourceCreatorContributor;

/// Device/region installation state, independent of account document data.
final class MobileBuiltInPreferences {
  MobileBuiltInPreferences(this.file);
  final File file;
  Map<String, Object?>? _values;
  Future<Map<String, Object?>> _read() async {
    if (_values != null) return _values!;
    if (!await file.exists()) return _values = {};
    try {
      return _values = Map<String, Object?>.from(
        jsonDecode(await file.readAsString()) as Map,
      );
    } on Object {
      return _values = {};
    }
  }

  Future<bool> isEnabled(String id) async => (await _read())[id] != false;
  Future<void> setEnabled(
    OpenMusePluginRegistry registry,
    OpenMusePlugin plugin,
    bool enabled,
  ) async {
    if (!mobilePluginCanBeManaged(plugin))
      throw StateError('Plugin is managed by the application');
    final previous =
        registry.stateOf(plugin.descriptor.id) == OpenMusePluginState.active;
    if (enabled) {
      await registry.activate(plugin.descriptor.id);
    } else {
      await registry.deactivate(plugin.descriptor.id);
    }
    final values = {...await _read(), plugin.descriptor.id: enabled};
    try {
      await file.parent.create(recursive: true);
      final temporary = File(
        '${file.path}.${DateTime.now().microsecondsSinceEpoch}.tmp',
      );
      await temporary.writeAsString(jsonEncode(values), flush: true);
      await temporary.rename(file.path);
      _values = values;
    } on Object {
      if (previous) {
        await registry.activate(plugin.descriptor.id);
      } else {
        await registry.deactivate(plugin.descriptor.id);
      }
      rethrow;
    }
  }
}

final class BuiltInPluginManagementPage extends StatefulWidget {
  const BuiltInPluginManagementPage({
    super.key,
    required this.plugin,
    required this.registry,
    this.preferences,
    this.onOpen,
  });
  final OpenMusePlugin plugin;
  final OpenMusePluginRegistry registry;
  final MobileBuiltInPreferences? preferences;
  final ValueChanged<OpenMusePlugin>? onOpen;
  @override
  State<BuiltInPluginManagementPage> createState() =>
      _BuiltInPluginManagementPageState();
}

final class _BuiltInPluginManagementPageState
    extends State<BuiltInPluginManagementPage> {
  bool busy = false;
  String? failure;
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: widget.registry,
    builder: (context, _) {
      final s = stringsOf(context), colors = Theme.of(context).colorScheme;
      final active =
          widget.registry.stateOf(widget.plugin.descriptor.id) ==
          OpenMusePluginState.active;
      final title = builtinPluginDisplayName(
        context,
        widget.plugin.descriptor.id,
        widget.plugin.descriptor.name,
      );
      return Scaffold(
        appBar: AppBar(title: Text(s.marketMyPlugins())),
        body: ListView(
          padding: const EdgeInsets.all(20),
          children: [
            Row(
              children: [
                Container(
                  width: 64,
                  height: 64,
                  decoration: BoxDecoration(
                    color: colors.primary.withValues(alpha: .09),
                    borderRadius: BorderRadius.circular(18),
                  ),
                  child: Icon(
                    Icons.widgets_outlined,
                    color: colors.primary,
                    size: 30,
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Text(
                    title,
                    style: const TextStyle(
                      fontSize: 23,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 24),
            Text(
              active ? s.mobileEnabledHere() : s.mobileDisabledHere(),
              style: TextStyle(color: colors.onSurfaceVariant),
            ),
            const SizedBox(height: 24),
            if (widget.plugin is OpenMuseMobileWorkbenchContributor &&
                widget.onOpen != null)
              FilledButton(
                key: const ValueKey('installed-plugin-open'),
                onPressed: active ? () => widget.onOpen!(widget.plugin) : null,
                child: Text(s.mobileOpenApp()),
              ),
            const SizedBox(height: 12),
            if (failure != null)
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 12),
                child: Text(failure!, style: TextStyle(color: colors.error)),
              ),
            OutlinedButton(
              key: const ValueKey('installed-plugin-toggle'),
              onPressed: busy
                  ? null
                  : () async {
                      setState(() {
                        busy = true;
                        failure = null;
                      });
                      try {
                        final preferences = widget.preferences;
                        if (preferences != null) {
                          await preferences.setEnabled(
                            widget.registry,
                            widget.plugin,
                            !active,
                          );
                        } else if (active) {
                          await widget.registry.deactivate(
                            widget.plugin.descriptor.id,
                          );
                        } else {
                          await widget.registry.activate(
                            widget.plugin.descriptor.id,
                          );
                        }
                      } on Object {
                        if (mounted)
                          setState(
                            () => failure = s.marketOpenFailed(title: title),
                          );
                      } finally {
                        if (mounted) setState(() => busy = false);
                      }
                    },
              child: Text(
                active ? s.mobileDisablePlugin() : s.mobileEnablePlugin(),
              ),
            ),
          ],
        ),
      );
    },
  );
}
