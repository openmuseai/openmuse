import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'src/paired_desktop_market_client.dart';
import 'src/market_discover_view.dart';
import 'src/mcp_remote_market.dart';

export 'src/paired_desktop_market_client.dart';
export 'src/mcp_remote_market.dart'
    show RemoteMcpEntry, RemoteMcpMarket, RemoteMcpMarketPage;

final class OpenMuseMobilePluginMarket implements OpenMusePlugin {
  OpenMuseMobilePluginMarket(
    this.store, {
    this.source = const BundledMobileMarketSource(),
    this.mcpRegistryOrigin,
  });
  final MobilePluginStore store;
  final MobileMarketSource source;
  final Uri? mcpRegistryOrigin;

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
      const Center(child: Text('插件市场不编辑文件'));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'mobile.plugin.market'
      ? MobileMarketPage(
          store: store,
          source: source,
          mcpRegistryOrigin: mcpRegistryOrigin,
        )
      : null;

  Widget buildMarketPage({
    PairedDesktopMarketClient? desktop,
    bool embedded = false,
    bool initialMine = false,
    OpenMusePluginRegistry? pluginRegistry,
    ValueChanged<OpenMusePlugin>? onOpenBuiltIn,
  }) => MobileMarketPage(
    store: store,
    source: source,
    desktop: desktop,
    embedded: embedded,
    initialMine: initialMine,
    pluginRegistry: pluginRegistry,
    onOpenBuiltIn: onOpenBuiltIn,
    mcpRegistryOrigin: mcpRegistryOrigin,
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
      const Center(child: Text('该插件没有文件编辑器'));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'mobile.plugin.${bundle.id}'
      ? _PluginRunPage(bundle: bundle, title: title, preview: false)
      : null;
}

final class MobileMarketPage extends StatefulWidget {
  const MobileMarketPage({
    super.key,
    required this.store,
    this.source = const BundledMobileMarketSource(),
    this.desktop,
    this.embedded = false,
    this.initialMine = false,
    this.pluginRegistry,
    this.onOpenBuiltIn,
    this.mcpRegistryOrigin,
  });
  final MobilePluginStore store;
  final MobileMarketSource source;
  final PairedDesktopMarketClient? desktop;
  final bool embedded;
  final bool initialMine;
  final OpenMusePluginRegistry? pluginRegistry;
  final ValueChanged<OpenMusePlugin>? onOpenBuiltIn;
  final Uri? mcpRegistryOrigin;

  @override
  State<MobileMarketPage> createState() => _MobileMarketPageState();
}

final class _MobileMarketPageState extends State<MobileMarketPage> {
  List<MobileMarketEntry>? entries;
  String? error;
  String? busy;
  String category = '精选';
  String query = '';
  late bool mine = widget.initialMine;

  @override
  void initState() {
    super.initState();
    widget.store.addListener(_refresh);
    widget.pluginRegistry?.addListener(_refresh);
    unawaited(_load());
  }

  void _refresh() {
    if (mounted) setState(() {});
  }

  Future<void> _load() async {
    try {
      final loaded = await widget.source.list();
      if (mounted) setState(() => entries = loaded);
    } catch (exception) {
      if (mounted) setState(() { entries = []; error = '$exception'; });
    }
  }

  Future<void> _install(MobileMarketEntry entry) async {
    setState(() {
      busy = entry.id;
      error = null;
    });
    try {
      await widget.store.install(entry, await widget.source.artifact(entry));
    } catch (exception) {
      if (mounted) setState(() => error = '$exception');
    } finally {
      if (mounted) setState(() => busy = null);
    }
  }

  Future<void> _open(MobileMarketEntry entry, {required bool preview}) async {
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
      bundle = widget.store.installed(entry.id)!;
    }
    if (!mounted) return;
    await Navigator.of(context).push<void>(
      MaterialPageRoute(
        builder: (_) =>
            _PluginRunPage(bundle: bundle, title: entry.name, preview: preview),
      ),
    );
  }

  Future<void> _detail(MobileMarketEntry entry) async {
    await Navigator.of(context).push<void>(
      MaterialPageRoute(
        builder: (_) => MarketPluginDetailPage(
          entry: entry,
          store: widget.store,
          onInstall: () => unawaited(_install(entry)),
          onOpen: () => unawaited(_open(entry, preview: false)),
          onPreview: () => unawaited(_open(entry, preview: true)),
          onToggle: () => unawaited(
            widget.store.setEnabled(
              entry.id,
              !widget.store.isEnabled(entry.id),
            ),
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    widget.store.removeListener(_refresh);
    widget.pluginRegistry?.removeListener(_refresh);
    super.dispose();
  }

  List<OpenMusePlugin> get _builtIns {
    final registry = widget.pluginRegistry;
    if (registry == null) return const [];
    return [
      for (final descriptor in registry.descriptors)
        if (registry.plugin(descriptor.id)
            case final OpenMuseMobileWorkbenchContributor contributor)
          contributor as OpenMusePlugin,
    ];
  }

  @override
  Widget build(BuildContext context) => entries == null
      ? Scaffold(
          body: Center(
            child: error == null
                ? const CircularProgressIndicator()
                : Text(error!),
          ),
        )
      : MarketDiscoverView(
          entries: entries!,
          store: widget.store,
          category: category,
          mine: mine,
          query: query,
          onSection: (value) => setState(() {
            mine = value;
            category = value ? '全部' : '精选';
            query = '';
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
          mcpSection: widget.mcpRegistryOrigin == null
              ? null
              : Card(
                  child: ListTile(
                    key: const ValueKey('mobile-mcp-online-market'),
                    leading: const Icon(Icons.cloud_outlined),
                    title: const Text('在线 MCP 插件'),
                    subtitle: const Text('标准 MCP Registry 与远程工具'),
                    onTap: () => Navigator.of(context).push<void>(
                      MaterialPageRoute(
                        builder: (_) => RemoteMcpMarketPage(
                          origin: widget.mcpRegistryOrigin!,
                        ),
                      ),
                    ),
                  ),
                ),
          desktopSection: widget.desktop == null
              ? const Padding(
                  padding: EdgeInsets.all(14),
                  child: Text('连接 Desktop 后可管理其插件。'),
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
                  '${entry['description'] ?? '在配对的 Desktop DSH 中使用。'}',
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
                        ? '已启用'
                        : plans[entry['id']]?['installedVersion'] != null
                        ? '已禁用'
                        : '未安装',
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
                              action == 'install' ? '安装到 Desktop' : '启用',
                            ),
                          ),
                        if (plans[entry['id']]?['enabled'] == true)
                          OutlinedButton(
                            onPressed: () => _act('disable', '${entry['id']}'),
                            child: Text('禁用'),
                          ),
                      ],
                    ),
                  Text(
                    '启停后需重启 Desktop DSH。',
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
      title: Text(widget.preview ? '${widget.title} · 预览' : widget.title),
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
