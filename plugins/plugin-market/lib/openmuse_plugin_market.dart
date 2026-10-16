import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:url_launcher/url_launcher.dart';

final class OpenMusePluginMarket implements OpenMusePlugin {
  OpenMusePluginContext? _context;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.plugin-market',
    name: '插件市场',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup', 'onPanel:plugin.market'],
    permissions: {'plugin.catalog.read', 'plugin.install.request'},
    panels: [
      OpenMusePanelContribution(
        id: 'plugin.market',
        region: OpenMuseSurfaceRegion.dialog,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async =>
      _context = context;

  @override
  Future<void> deactivate() async => _context = null;

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const Center(child: Text('插件市场不编辑文件'));

  @override
  Widget? buildPanel(BuildContext context, String panelId) => switch (panelId) {
    'plugin.market' => _MarketPanel(host: _context),
    'plugin.market.installed' => _MarketPanel(
      host: _context,
      installedOnly: true,
    ),
    _ => null,
  };
}

/// Host-owned macOS surface for a standard MCP App HTML resource link.
/// The plugin receives no Flutter or native bridge capability.
final class McpAppPage extends StatelessWidget {
  const McpAppPage({super.key, required this.title, required this.uri});

  final String title;
  final Uri uri;

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(title)),
    body: AppKitView(
      key: ValueKey('mcp-app-${uri.toString()}'),
      viewType: 'com.openmuse.mcp/app-view',
      creationParams: {'url': uri.toString()},
      creationParamsCodec: const StandardMessageCodec(),
    ),
  );
}

final class _MarketPanel extends StatefulWidget {
  const _MarketPanel({required this.host, this.installedOnly = false});
  final OpenMusePluginContext? host;
  final bool installedOnly;

  @override
  State<_MarketPanel> createState() => _MarketPanelState();
}

final class _MarketPanelState extends State<_MarketPanel> {
  List<Map<String, Object?>>? _plugins;
  String? _error;
  String? _busy;
  String _query = '';
  String _category = '精选';
  String? _selectedId;
  final Map<String, Map<String, Object?>> _plans = {};
  final Map<String, String> _planErrors = {};

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final result = await widget.host?.executeHostCommand(
        'plugin.market.list',
        null,
      );
      if (result is! List) throw StateError('市场目录不可用');
      if (mounted) {
        setState(() {
          _plugins = [
            for (final item in result) Map<String, Object?>.from(item as Map),
          ];
          _error = null;
          _planErrors.clear();
        });
      }
      for (final item in result) {
        final id = (item as Map)['id'];
        if (id is! String) continue;
        try {
          final plan = await widget.host?.executeHostCommand(
            'plugin.market.plan',
            {'id': id},
          );
          if (mounted && plan is Map) {
            setState(() => _plans[id] = Map<String, Object?>.from(plan));
          }
        } catch (error) {
          if (mounted) setState(() => _planErrors[id] = '$error');
        }
      }
    } catch (error) {
      if (mounted) setState(() => _error = '$error');
    }
  }

  Future<void> _install(String id) async {
    if (_busy != null) return;
    setState(() {
      _busy = id;
      _error = null;
      _planErrors.remove(id);
    });
    try {
      final plan = await widget.host?.executeHostCommand('plugin.market.plan', {
        'id': id,
      });
      if (plan is! Map) throw StateError('无法计算安装计划');
      if (mounted) setState(() => _plans[id] = Map<String, Object?>.from(plan));
      if (plan['action'] != 'already-installed') {
        final result = await widget.host?.executeHostCommand(
          'plugin.market.install',
          {'id': id},
        );
        if (result is! Map || result['installedVersion'] != plan['version']) {
          throw StateError('安装后版本与计划不一致');
        }
        if (mounted)
          setState(() => _plans[id] = Map<String, Object?>.from(result));
      }
    } catch (error) {
      if (mounted) setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = null);
    }
  }

  Future<void> _openMcp(String id) async {
    if (_busy != null) return;
    setState(() {
      _busy = id;
      _error = null;
    });
    try {
      final result = await widget.host?.executeHostCommand(
        'plugin.market.open',
        {'id': id},
      );
      if (result is! Map || result['uri'] is! String) {
        throw StateError('MCP App 没有返回可打开的资源');
      }
      final uri = Uri.parse(result['uri'] as String);
      if (Platform.isMacOS) {
        if (mounted) {
          await Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (_) => McpAppPage(title: id, uri: uri),
            ),
          );
        }
      } else if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
        throw StateError('无法打开 MCP App');
      }
    } catch (error) {
      if (mounted) setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = null);
    }
  }

  Future<void> _setEnabled(String id, bool enabled) async {
    if (_busy != null) return;
    setState(() {
      _busy = id;
      _error = null;
    });
    try {
      final result = await widget.host?.executeHostCommand(
        enabled ? 'plugin.market.enable' : 'plugin.market.disable',
        {'id': id},
      );
      if (result is! Map || result['enabled'] != enabled) {
        throw StateError('Desktop 插件状态更新失败');
      }
      if (mounted)
        setState(() => _plans[id] = Map<String, Object?>.from(result));
    } catch (error) {
      if (mounted) setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = null);
    }
  }

  Widget _searchField() => TextField(
    key: const ValueKey('plugin-market-search'),
    onChanged: (value) => setState(() => _query = value),
    decoration: const InputDecoration(
      prefixIcon: Icon(Icons.search_rounded, size: 20),
      hintText: '搜索插件、功能或作者',
    ),
  );

  Widget _categoryChips() => Wrap(
    spacing: 8,
    runSpacing: 8,
    children: [
      for (final category in const ['精选', '办公', '工具', 'UI', '编程', '生活'])
        ChoiceChip(
          key: ValueKey('plugin-market-category-$category'),
          label: Text(category),
          selected: _category == category,
          showCheckmark: false,
          selectedColor: MarketColors.paleBlue,
          backgroundColor: MarketColors.surface,
          side: const BorderSide(color: MarketColors.line),
          labelStyle: TextStyle(
            color: _category == category
                ? MarketColors.blue
                : MarketColors.secondary,
            fontWeight: _category == category
                ? FontWeight.w700
                : FontWeight.w500,
          ),
          onSelected: (_) => setState(() => _category = category),
        ),
    ],
  );

  @override
  Widget build(BuildContext context) {
    final plugins = _plugins;
    final visible = plugins?.where((plugin) {
      final id = plugin['id'] as String;
      if (widget.installedOnly) return _plans[id]?['installedVersion'] != null;
      final query = _query.trim().toLowerCase();
      final categories = plugin['categories'];
      final categoryMatch =
          _category == '精选' ||
          (categories is List && categories.contains(_category));
      return categoryMatch &&
          (query.isEmpty ||
              '${plugin['name']} ${plugin['package']} ${plugin['description'] ?? ''}'
                  .toLowerCase()
                  .contains(query));
    }).toList();
    final selected = plugins
        ?.where((item) => item['id'] == _selectedId)
        .firstOrNull;
    return Theme(
      data: openMuseMarketTheme(),
      child: ColoredBox(
        color: MarketColors.canvas,
        child: LayoutBuilder(
          builder: (context, constraints) => ListView(
            key: ValueKey(
              widget.installedOnly
                  ? 'plugin-market-installed'
                  : 'plugin-market-catalog',
            ),
            padding: const EdgeInsets.fromLTRB(28, 26, 28, 32),
            children: [
              ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 1040),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      widget.installedOnly ? '已安装插件' : '发现更好的工作方式',
                      style: const TextStyle(
                        fontSize: 29,
                        fontWeight: FontWeight.w800,
                        letterSpacing: -0.8,
                        color: MarketColors.ink,
                      ),
                    ),
                    const SizedBox(height: 6),
                    Text(
                      widget.installedOnly
                          ? '管理已安装到本机的插件。'
                          : '发现经过兼容性验证的插件并安装到当前设备。',
                      style: const TextStyle(
                        fontSize: 13,
                        color: MarketColors.secondary,
                      ),
                    ),
                    if (!widget.installedOnly) ...[
                      const SizedBox(height: 22),
                      if (constraints.maxWidth >= 820)
                        Row(
                          children: [
                            Expanded(child: _categoryChips()),
                            const SizedBox(width: 20),
                            SizedBox(width: 280, child: _searchField()),
                          ],
                        )
                      else
                        Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            _categoryChips(),
                            const SizedBox(height: 12),
                            _searchField(),
                          ],
                        ),
                      const SizedBox(height: 22),
                      if (_query.isEmpty &&
                          _category == '精选' &&
                          selected == null &&
                          plugins?.any((item) => item['id'] == 'archify') ==
                              true) ...[
                        MarketHeroArt(
                          asset: MarketArt.archify,
                          height: constraints.maxHeight < 700 ? 190 : 250,
                          onTap: () => setState(() => _selectedId = 'archify'),
                          child: const Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                '编辑精选  ·  编程',
                                style: TextStyle(
                                  color: MarketColors.blue,
                                  fontWeight: FontWeight.w700,
                                  fontSize: 12,
                                ),
                              ),
                              SizedBox(height: 12),
                              Text(
                                '让复杂结构，\n一眼看清',
                                style: TextStyle(
                                  color: MarketColors.ink,
                                  fontSize: 27,
                                  fontWeight: FontWeight.w800,
                                  height: 1.18,
                                ),
                              ),
                              Spacer(),
                              Text(
                                'Archify · 架构图生成技能',
                                style: TextStyle(
                                  color: MarketColors.ink,
                                  fontWeight: FontWeight.w600,
                                ),
                              ),
                            ],
                          ),
                        ),
                        const SizedBox(height: 25),
                      ],
                    ] else
                      const SizedBox(height: 20),
                    if (_error != null) ...[
                      _notice(context, _error!),
                      const SizedBox(height: 8),
                      Align(
                        alignment: Alignment.centerLeft,
                        child: TextButton(
                          onPressed: _load,
                          child: const Text('重试'),
                        ),
                      ),
                    ],
                    if (plugins == null ||
                        (widget.installedOnly &&
                            _plans.length + _planErrors.length <
                                plugins.length))
                      const Center(child: CircularProgressIndicator())
                    else if (widget.installedOnly && _planErrors.isNotEmpty)
                      _notice(context, _planErrors.values.first)
                    else if (visible!.isEmpty)
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 28),
                        child: Center(
                          child: Text(
                            widget.installedOnly ? '尚未安装插件' : '没有匹配的插件',
                            style: const TextStyle(
                              color: MarketColors.secondary,
                            ),
                          ),
                        ),
                      )
                    else ...[
                      if (selected != null) ...[
                        _detail(context, selected),
                        const SizedBox(height: 24),
                      ],
                      Text(
                        widget.installedOnly ? '我的插件' : '为你推荐',
                        style: const TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.w800,
                          color: MarketColors.ink,
                        ),
                      ),
                      const SizedBox(height: 14),
                      Wrap(
                        spacing: 12,
                        runSpacing: 12,
                        children: [
                          for (final plugin in visible)
                            SizedBox(
                              width:
                                  widget.installedOnly ||
                                      constraints.maxWidth < 650
                                  ? constraints.maxWidth - 56
                                  : (constraints.maxWidth - 68) / 2,
                              child: _card(context, plugin),
                            ),
                        ],
                      ),
                    ],
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _notice(BuildContext context, String message) {
    final colors = Theme.of(context).colorScheme;
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: colors.errorContainer,
        borderRadius: BorderRadius.circular(9),
      ),
      child: Text(
        message.replaceFirst('Bad state: ', '').split('\n').first,
        style: TextStyle(fontSize: 12, color: colors.onErrorContainer),
      ),
    );
  }

  Widget _card(BuildContext context, Map<String, Object?> plugin) {
    final id = plugin['id'] as String;
    final plan = _plans[id];
    final installed = plan?['installedVersion'] != null;
    final busy = _busy == id;
    return Card(
      key: ValueKey('plugin-market-card-$id'),
      margin: EdgeInsets.zero,
      elevation: 0,
      color: MarketColors.surface,
      shape: RoundedRectangleBorder(
        side: const BorderSide(color: MarketColors.border),
        borderRadius: BorderRadius.circular(18),
      ),
      child: InkWell(
        borderRadius: BorderRadius.circular(18),
        onTap: () => setState(() => _selectedId = id),
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
                      color: const Color(0xFFE9F1FC),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: const Icon(
                      Icons.account_tree_rounded,
                      size: 22,
                      color: MarketColors.blue,
                    ),
                  ),
                  const SizedBox(width: 10),
                  Expanded(
                    child: Text(
                      '${plugin['name']} ${plugin['version']}',
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        fontSize: 15,
                        color: MarketColors.ink,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  if (installed && plugin['ecosystem'] == 'mcp')
                    const Text('已安装')
                  else if (installed)
                    TextButton(
                      key: ValueKey('plugin-market-toggle-$id'),
                      onPressed: busy
                          ? null
                          : () => _setEnabled(id, plan?['enabled'] != true),
                      child: Text(plan?['enabled'] == true ? '禁用' : '启用'),
                    )
                  else
                    FilledButton(
                      onPressed: busy ? null : () => _install(id),
                      child: Text(busy ? '安装中…' : '安装'),
                    ),
                ],
              ),
              const SizedBox(height: 15),
              Text(
                '${plugin['description'] ?? '在 DSH 中扩展工作能力。'}',
                style: const TextStyle(
                  fontSize: 13,
                  color: MarketColors.secondary,
                ),
              ),
              const SizedBox(height: 12),
              Row(
                children: [
                  const MarketPlatformBadge('Desktop'),
                  const SizedBox(width: 7),
                  if (plugin['ecosystem'] == 'dsh')
                    const MarketPlatformBadge('DSH')
                  else
                    const MarketPlatformBadge('MCP'),
                  const Spacer(),
                  const Icon(
                    Icons.arrow_forward_rounded,
                    size: 16,
                    color: MarketColors.secondary,
                  ),
                ],
              ),
              const SizedBox(height: 11),
              Text(
                '${plugin['ecosystem']} · ${plugin['package']}',
                style: const TextStyle(
                  fontSize: 11,
                  color: MarketColors.secondary,
                ),
              ),
              const SizedBox(height: 10),
              if (plan != null)
                Text(
                  plugin['ecosystem'] == 'mcp'
                      ? (installed
                            ? '已安装 ${plan['installedVersion']} · 可使用 MCP 工具'
                            : '在线安装 MCP Bundle')
                      : (installed
                            ? '已安装 ${plan['installedVersion']} · ${plan['enabled'] == true ? '已启用' : '已禁用'} · 重启 DSH 后生效'
                            : '待安装到 ${plan['profile']}'),
                  style: const TextStyle(
                    fontSize: 11,
                    color: MarketColors.secondary,
                  ),
                ),
              if (_planErrors[id] case final error?) ...[
                const SizedBox(height: 10),
                _notice(context, error),
              ],
            ],
          ),
        ),
      ),
    );
  }

  Widget _detail(BuildContext context, Map<String, Object?> plugin) {
    final id = plugin['id'] as String;
    final plan = _plans[id];
    final installed = plan?['installedVersion'] != null;
    return Container(
      key: ValueKey('plugin-market-detail-$id'),
      padding: const EdgeInsets.all(22),
      decoration: BoxDecoration(
        color: MarketColors.surface,
        border: Border.all(color: MarketColors.border),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              TextButton.icon(
                onPressed: () => setState(() => _selectedId = null),
                icon: const Icon(Icons.arrow_back_rounded, size: 18),
                label: const Text('返回发现'),
              ),
              const Spacer(),
              const MarketPlatformBadge('Desktop'),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            '${plugin['name']}',
            style: const TextStyle(
              fontSize: 25,
              fontWeight: FontWeight.w800,
              color: MarketColors.ink,
            ),
          ),
          const SizedBox(height: 5),
          Text(
            'v${plugin['version']} · ${plugin['package']}',
            style: const TextStyle(fontSize: 12, color: MarketColors.secondary),
          ),
          const SizedBox(height: 20),
          if (plugin['ecosystem'] == 'dsh')
            ClipRRect(
              borderRadius: BorderRadius.circular(15),
              child: Image.asset(
                MarketArt.archify,
                fit: BoxFit.cover,
                height: 220,
                width: double.infinity,
              ),
            ),
          const SizedBox(height: 20),
          Text(
            '${plugin['description'] ?? '在 DSH 中扩展工作能力。'}',
            style: const TextStyle(fontSize: 14, color: MarketColors.ink),
          ),
          const SizedBox(height: 14),
          Text(
            plugin['ecosystem'] == 'mcp'
                ? (installed
                      ? '已安装 ${plan?['installedVersion']} · MCP 工具已就绪'
                      : '从在线 MCP Registry 安装已审核的 MCP Bundle')
                : (installed
                      ? '已安装 ${plan?['installedVersion']} · ${plan?['enabled'] == true ? '已启用' : '已禁用'} · 重启 DSH 后生效'
                      : '安装位置：${plan?['profile'] ?? '正在读取…'}'),
            style: const TextStyle(fontSize: 12, color: MarketColors.secondary),
          ),
          const SizedBox(height: 15),
          if (installed && plugin['ecosystem'] == 'mcp')
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                OutlinedButton.icon(
                  onPressed: _busy == id ? null : () => _openMcp(id),
                  icon: const Icon(Icons.open_in_new_rounded, size: 17),
                  label: const Text('打开 MCP App'),
                ),
                const SizedBox(height: 8),
                const Text('文件型 MCP 工具也可从工作区打开兼容文件。'),
              ],
            )
          else if (installed)
            OutlinedButton(
              onPressed: _busy == id
                  ? null
                  : () => _setEnabled(id, plan?['enabled'] != true),
              child: Text(plan?['enabled'] == true ? '禁用插件' : '启用插件'),
            )
          else
            FilledButton(
              onPressed: _busy == id ? null : () => _install(id),
              child: Text(_busy == id ? '安装中…' : '安装到 Desktop'),
            ),
        ],
      ),
    );
  }
}
