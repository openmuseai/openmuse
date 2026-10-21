import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:video_player/video_player.dart';

const marketCategories = ['精选', '办公', 'UI', '编程', '生活'];

String marketDisplayName(MobileMarketEntry entry) =>
    entry.name.replaceFirst(' Mobile 适配演示', '');

bool marketSupportsCurrentMobile(MobileMarketEntry entry) =>
    switch (defaultTargetPlatform) {
      TargetPlatform.android => entry.platforms.contains('android'),
      TargetPlatform.iOS => entry.platforms.contains('ios'),
      _ => true,
    };

class MarketDiscoverView extends StatelessWidget {
  const MarketDiscoverView({
    super.key,
    required this.entries,
    required this.store,
    required this.category,
    required this.mine,
    required this.query,
    required this.onSection,
    required this.onCategory,
    required this.onSearch,
    required this.onDetail,
    required this.onInstall,
    required this.onOpen,
    required this.onPreview,
    required this.desktopSection,
    this.mcpSection,
    this.builtInPlugins = const [],
    this.builtInState,
    this.onOpenBuiltIn,
    this.onManageWorkbench,
    this.error,
    this.busy,
  });

  final List<MobileMarketEntry> entries;
  final MobilePluginStore store;
  final String category;
  final bool mine;
  final String query;
  final ValueChanged<bool> onSection;
  final ValueChanged<String> onCategory;
  final ValueChanged<String> onSearch;
  final ValueChanged<MobileMarketEntry> onDetail;
  final ValueChanged<MobileMarketEntry> onInstall;
  final ValueChanged<MobileMarketEntry> onOpen;
  final ValueChanged<MobileMarketEntry> onPreview;
  final Widget desktopSection;
  final Widget? mcpSection;
  final List<OpenMusePlugin> builtInPlugins;
  final OpenMusePluginState? Function(OpenMusePlugin)? builtInState;
  final ValueChanged<OpenMusePlugin>? onOpenBuiltIn;
  final VoidCallback? onManageWorkbench;
  final String? error;
  final String? busy;

  @override
  Widget build(BuildContext context) {
    final filtered = entries.where((entry) {
      final categoryMatch = mine
          ? category == '全部' ||
                (category == '已启用' && store.isEnabled(entry.id)) ||
                (category == '已停用' && !store.isEnabled(entry.id)) ||
                (category == '更新' &&
                    store.installed(entry.id)?.version != entry.version)
          : category == '精选' || entry.categories.contains(category);
      final text = '$query'.trim().toLowerCase();
      return (!mine || store.isInstalled(entry.id)) &&
          categoryMatch &&
          (text.isEmpty ||
              '${entry.name} ${entry.description} ${entry.author}'
                  .toLowerCase()
                  .contains(text));
    }).toList();
    final visibleBuiltIns = builtInPlugins.where((plugin) {
      final active = builtInState?.call(plugin) == OpenMusePluginState.active;
      if (mine && category == '已启用' && !active) return false;
      if (mine && category == '已停用' && active) return false;
      if (mine && category == '更新') return false;
      if (!mine && category != '精选') return false;
      final text = query.trim().toLowerCase();
      return text.isEmpty ||
          '${plugin.descriptor.name} ${plugin.descriptor.id}'
              .toLowerCase()
              .contains(text);
    }).toList();
    final featured = filtered
        .where(
          (entry) =>
              marketSupportsCurrentMobile(entry) &&
              entry.media.any(
                (medium) =>
                    medium.type != 'video' && medium.uri.startsWith('asset:'),
              ),
        )
        .firstOrNull;
    final featuredArt = featured?.media
        .where(
          (medium) => medium.type != 'video' && medium.uri.startsWith('asset:'),
        )
        .firstOrNull;
    return Scaffold(
      backgroundColor: Theme.of(context).scaffoldBackgroundColor,
      body: CustomScrollView(
        slivers: [
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 20, 20, 8),
              child: Row(
                children: [
                  for (final section in [(false, '发现'), (true, '我的')])
                    Padding(
                      padding: const EdgeInsets.only(right: 18),
                      child: Semantics(
                        label: section.$1 ? '我的插件' : '发现插件',
                        selected: mine == section.$1,
                        button: true,
                        child: InkWell(
                          key: ValueKey('mobile-market-section-${section.$2}'),
                          onTap: () => onSection(section.$1),
                          child: SizedBox(
                            height: 48,
                            child: Align(
                              alignment: Alignment.centerLeft,
                              child: AnimatedDefaultTextStyle(
                                duration:
                                    MediaQuery.disableAnimationsOf(context)
                                    ? Duration.zero
                                    : const Duration(milliseconds: 200),
                                style: TextStyle(
                                  fontSize: mine == section.$1 ? 32 : 24,
                                  fontWeight: FontWeight.w800,
                                  letterSpacing: -0.9,
                                  color: mine == section.$1
                                      ? Theme.of(context).colorScheme.onSurface
                                      : Theme.of(
                                          context,
                                        ).colorScheme.onSurfaceVariant,
                                ),
                                child: Text(section.$2),
                              ),
                            ),
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 6, 20, 10),
              child: TextField(
                key: const ValueKey('mobile-market-search'),
                onChanged: onSearch,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.search_rounded),
                  hintText: mine ? '搜索已安装插件' : '搜索插件与功能',
                  isDense: true,
                  filled: true,
                  fillColor: Theme.of(context).colorScheme.surface,
                  border: OutlineInputBorder(
                    borderSide: BorderSide.none,
                    borderRadius: BorderRadius.circular(16),
                  ),
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 15,
                    vertical: 15,
                  ),
                ),
              ),
            ),
          ),
          SliverToBoxAdapter(
            child: SizedBox(
              height: 48,
              child: ListView(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                scrollDirection: Axis.horizontal,
                children: [
                  for (final item
                      in mine
                          ? const ['全部', '已启用', '已停用', '更新']
                          : marketCategories)
                    Padding(
                      padding: const EdgeInsets.only(right: 23),
                      child: Semantics(
                        label: '$item类别',
                        selected: category == item,
                        button: true,
                        child: InkWell(
                          key: ValueKey('market-category-$item'),
                          onTap: () => onCategory(item),
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              Text(
                                item,
                                style: TextStyle(
                                  fontSize: 15,
                                  fontWeight: category == item
                                      ? FontWeight.w700
                                      : FontWeight.w500,
                                  color: category == item
                                      ? Theme.of(context).colorScheme.primary
                                      : Theme.of(
                                          context,
                                        ).colorScheme.onSurfaceVariant,
                                ),
                              ),
                              const SizedBox(height: 5),
                              AnimatedContainer(
                                duration:
                                    MediaQuery.disableAnimationsOf(context)
                                    ? Duration.zero
                                    : const Duration(milliseconds: 180),
                                height: 3,
                                width: category == item ? 23 : 0,
                                decoration: BoxDecoration(
                                  color: Theme.of(context).colorScheme.primary,
                                  borderRadius: BorderRadius.circular(2),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
          SliverPadding(
            padding: const EdgeInsets.fromLTRB(20, 8, 20, 24),
            sliver: SliverList(
              delegate: SliverChildListDelegate([
                if (error != null)
                  Padding(
                    padding: const EdgeInsets.only(bottom: 12),
                    child: Text(
                      error!,
                      style: TextStyle(color: Color(0xFFB42318)),
                    ),
                  ),
                if (!mine &&
                    featured != null &&
                    featuredArt != null &&
                    category == '精选' &&
                    query.isEmpty) ...[
                  MarketHeroArt(
                    asset: featuredArt.uri.substring('asset:'.length),
                    height: 300,
                    onTap: () => onDetail(featured),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '本周精选 · ${featured.categories.firstOrNull ?? '插件'}',
                          style: TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.w700,
                            color: const Color(0xFF1765E9),
                          ),
                        ),
                        const SizedBox(height: 14),
                        Text(
                          '把好用的工具\n放进工作台',
                          style: TextStyle(
                            color: const Color(0xFF25303E),
                            fontSize: 25,
                            height: 1.17,
                            fontWeight: FontWeight.w800,
                            letterSpacing: -0.6,
                          ),
                        ),
                        const Spacer(),
                        FilledButton.icon(
                          onPressed: () => onDetail(featured),
                          icon: const Icon(
                            Icons.arrow_forward_rounded,
                            size: 17,
                          ),
                          label: Text('查看${marketDisplayName(featured)}'),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 24),
                ],
                Row(
                  children: [
                    Expanded(
                      child: Text(
                        mine
                            ? '我的插件'
                            : category == '精选'
                            ? '编辑推荐'
                            : '$category插件',
                        style: TextStyle(
                          fontSize: 21,
                          fontWeight: FontWeight.w800,
                          color: Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                    ),
                    Text(
                      mcpSection == null
                          ? '${filtered.length + visibleBuiltIns.length} 个'
                          : '',
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 11),
                if (filtered.isEmpty &&
                    visibleBuiltIns.isEmpty &&
                    mcpSection == null)
                  Container(
                    key: const ValueKey('market-empty-category'),
                    padding: const EdgeInsets.all(26),
                    decoration: BoxDecoration(
                      color: Theme.of(context).colorScheme.surface,
                      borderRadius: BorderRadius.circular(20),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Icon(
                          Icons.auto_awesome_outlined,
                          color: Theme.of(context).colorScheme.primary,
                          size: 30,
                        ),
                        SizedBox(height: 12),
                        Text(
                          '这个分类还没有已验证的插件',
                          style: TextStyle(
                            fontWeight: FontWeight.w700,
                            fontSize: 16,
                          ),
                        ),
                        SizedBox(height: 5),
                        Text(
                          '完成兼容验证后会出现在这里。',
                          style: TextStyle(
                            color: Theme.of(
                              context,
                            ).colorScheme.onSurfaceVariant,
                          ),
                        ),
                      ],
                    ),
                  ),
                if (mcpSection != null) mcpSection!,
                for (final entry in filtered)
                  _MarketEntryTile(
                    entry: entry,
                    store: store,
                    busy: busy == entry.id,
                    onDetail: () => onDetail(entry),
                    onInstall: () => onInstall(entry),
                    onOpen: () => onOpen(entry),
                    onPreview: () => onPreview(entry),
                  ),
                if (visibleBuiltIns.isNotEmpty) ...[
                  const SizedBox(height: 14),
                  Text(
                    '预置工具',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.w800,
                      color: Theme.of(context).colorScheme.onSurface,
                    ),
                  ),
                  const SizedBox(height: 10),
                  for (final plugin in visibleBuiltIns)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 10),
                      child: Material(
                        color: Theme.of(context).colorScheme.surface,
                        borderRadius: BorderRadius.circular(18),
                        child: ListTile(
                          key: ValueKey(
                            'market-built-in-${plugin.descriptor.id}',
                          ),
                          leading: const Icon(Icons.widgets_outlined),
                          title: Text(plugin.descriptor.name),
                          subtitle: Text(
                            builtInState?.call(plugin) ==
                                    OpenMusePluginState.active
                                ? '已预置 · 可在工作台打开'
                                : '已预置 · 当前未启用',
                          ),
                          trailing: const Icon(Icons.chevron_right),
                          onTap:
                              builtInState?.call(plugin) ==
                                  OpenMusePluginState.active
                              ? () => onOpenBuiltIn?.call(plugin)
                              : null,
                        ),
                      ),
                    ),
                ],
                if (mine && onManageWorkbench != null) ...[
                  const SizedBox(height: 20),
                  Material(
                    color: Theme.of(context).colorScheme.surface,
                    borderRadius: BorderRadius.circular(18),
                    child: ListTile(
                      key: const ValueKey('market-manage-workbench'),
                      leading: const Icon(Icons.dashboard_customize_outlined),
                      title: const Text('工作台展示'),
                      subtitle: const Text('管理已安装插件的卡片和位置'),
                      trailing: const Icon(Icons.chevron_right),
                      onTap: onManageWorkbench,
                    ),
                  ),
                ],
                if (!mine && category == '精选' && query.isEmpty) ...[
                  const SizedBox(height: 18),
                  MarketHeroArt(
                    asset: MarketArt.archify,
                    height: 165,
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '跨设备协作',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.primary,
                            fontSize: 12,
                            fontWeight: FontWeight.w700,
                          ),
                        ),
                        SizedBox(height: 11),
                        Text(
                          '你的桌面，\n也在这里',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface,
                            fontSize: 23,
                            height: 1.18,
                            fontWeight: FontWeight.w800,
                          ),
                        ),
                        Spacer(),
                        Text(
                          '连接 Desktop 后管理插件',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 25),
                  Text(
                    'Desktop 插件',
                    style: TextStyle(
                      fontSize: 21,
                      fontWeight: FontWeight.w800,
                      color: Theme.of(context).colorScheme.onSurface,
                    ),
                  ),
                  const SizedBox(height: 10),
                  desktopSection,
                ],
              ]),
            ),
          ),
        ],
      ),
    );
  }
}

class _MarketEntryTile extends StatelessWidget {
  const _MarketEntryTile({
    required this.entry,
    required this.store,
    required this.busy,
    required this.onDetail,
    required this.onInstall,
    required this.onOpen,
    required this.onPreview,
  });
  final MobileMarketEntry entry;
  final MobilePluginStore store;
  final bool busy;
  final VoidCallback onDetail;
  final VoidCallback onInstall;
  final VoidCallback onOpen;
  final VoidCallback onPreview;

  @override
  Widget build(BuildContext context) {
    final installed = store.isInstalled(entry.id);
    final enabled = store.isEnabled(entry.id);
    final supported = marketSupportsCurrentMobile(entry);
    final updateAvailable =
        installed && store.installed(entry.id)!.version != entry.version;
    return Padding(
      padding: const EdgeInsets.only(bottom: 9),
      child: Material(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: BorderRadius.circular(17),
        child: InkWell(
          key: ValueKey('market-entry-${entry.id}'),
          borderRadius: BorderRadius.circular(17),
          onTap: onDetail,
          child: Padding(
            padding: const EdgeInsets.all(12),
            child: Row(
              children: [
                MarketPluginIcon(entry.id, size: 57),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        marketDisplayName(entry),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                          fontWeight: FontWeight.w700,
                          fontSize: 16,
                          color: Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        entry.description,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                          fontSize: 12,
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                        ),
                      ),
                      const SizedBox(height: 6),
                      Wrap(
                        spacing: 5,
                        children: [
                          for (final platform in entry.platforms)
                            MarketPlatformBadge(
                              platform == 'ios'
                                  ? 'iOS'
                                  : platform == 'android'
                                  ? 'Android'
                                  : platform,
                            ),
                        ],
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 8),
                Column(
                  children: [
                    if (!installed || updateAvailable)
                      FilledButton(
                        key: ValueKey('install-${entry.id}'),
                        onPressed: busy || !supported ? null : onInstall,
                        child: Text(
                          !supported
                              ? '不支持'
                              : busy
                              ? '处理中'
                              : updateAvailable
                              ? '更新'
                              : '安装',
                        ),
                      )
                    else
                      FilledButton(
                        key: ValueKey('open-${entry.id}'),
                        onPressed: enabled && supported ? onOpen : null,
                        child: Text(
                          !supported
                              ? '不支持'
                              : enabled
                              ? '打开'
                              : '已禁用',
                        ),
                      ),
                    TextButton(
                      key: ValueKey('preview-${entry.id}'),
                      onPressed: supported ? onPreview : null,
                      child: Text('预览'),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class MarketPluginIcon extends StatelessWidget {
  const MarketPluginIcon(this.id, {super.key, this.size = 68});
  final String id;
  final double size;

  @override
  Widget build(BuildContext context) {
    final usage = id.contains('usage');
    return Container(
      width: size,
      height: size,
      decoration: BoxDecoration(
        color: usage ? const Color(0xFFFFE8DB) : const Color(0xFFDCEAFF),
        borderRadius: BorderRadius.circular(size * .25),
      ),
      child: Icon(
        usage ? Icons.bar_chart_rounded : Icons.view_kanban_rounded,
        size: size * .53,
        color: usage
            ? const Color(0xFFDE7256)
            : Theme.of(context).colorScheme.primary,
      ),
    );
  }
}

class MarketPluginDetailPage extends StatelessWidget {
  const MarketPluginDetailPage({
    super.key,
    required this.entry,
    required this.store,
    required this.onInstall,
    required this.onOpen,
    required this.onPreview,
    required this.onToggle,
    required this.onRemove,
  });
  final MobileMarketEntry entry;
  final MobilePluginStore store;
  final VoidCallback onInstall;
  final VoidCallback onOpen;
  final VoidCallback onPreview;
  final VoidCallback onToggle;
  final VoidCallback onRemove;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: store,
    builder: (context, _) {
      final installed = store.isInstalled(entry.id);
      final enabled = store.isEnabled(entry.id);
      final supported = marketSupportsCurrentMobile(entry);
      final updateAvailable =
          installed && store.installed(entry.id)!.version != entry.version;
      final name = marketDisplayName(entry);
      final permissions =
          store.installed(entry.id)?.manifest.requestedPermissions.toList() ??
          entry.permissions;
      return Scaffold(
        appBar: AppBar(title: Text(name)),
        bottomNavigationBar: SafeArea(
          top: false,
          child: Container(
            padding: const EdgeInsets.fromLTRB(16, 10, 16, 10),
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surface,
              border: Border(
                top: BorderSide(
                  color: Theme.of(context).colorScheme.outlineVariant,
                ),
              ),
            ),
            child: Row(
              children: [
                MarketPluginIcon(entry.id, size: 42),
                const SizedBox(width: 10),
                Expanded(
                  child: Text(
                    name,
                    style: TextStyle(fontWeight: FontWeight.w700),
                  ),
                ),
                FilledButton(
                  key: ValueKey('detail-action-${entry.id}'),
                  onPressed: !supported
                      ? null
                      : (!installed || updateAvailable)
                      ? onInstall
                      : enabled
                      ? onOpen
                      : onToggle,
                  child: Text(
                    !supported
                        ? '此设备不支持'
                        : updateAvailable
                        ? '更新'
                        : !installed
                        ? '安装'
                        : enabled
                        ? '打开'
                        : '启用',
                  ),
                ),
              ],
            ),
          ),
        ),
        body: ListView(
          padding: const EdgeInsets.fromLTRB(20, 12, 20, 28),
          children: [
            Row(
              children: [
                MarketPluginIcon(entry.id),
                const SizedBox(width: 14),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        name,
                        style: TextStyle(
                          fontSize: 23,
                          fontWeight: FontWeight.w800,
                        ),
                      ),
                      Text(
                        '${entry.author} · ${entry.categories.join(' / ')}',
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                        ),
                      ),
                      const SizedBox(height: 6),
                      Wrap(
                        spacing: 6,
                        children: [
                          for (final platform in entry.platforms)
                            MarketPlatformBadge(
                              platform == 'ios'
                                  ? 'iOS'
                                  : platform == 'android'
                                  ? 'Android'
                                  : platform,
                            ),
                        ],
                      ),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 22),
            SizedBox(
              height: 216,
              child: entry.media.isEmpty
                  ? DecoratedBox(
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.surfaceContainer,
                        borderRadius: BorderRadius.circular(20),
                      ),
                      child: Center(
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            MarketPluginIcon(entry.id, size: 64),
                            const SizedBox(height: 12),
                            Text(
                              marketDisplayName(entry),
                              style: const TextStyle(
                                fontSize: 19,
                                fontWeight: FontWeight.w700,
                              ),
                            ),
                          ],
                        ),
                      ),
                    )
                  : PageView(
                      children: [
                        for (final medium in entry.media)
                          MarketMediaView(media: medium),
                      ],
                    ),
            ),
            const SizedBox(height: 24),
            Text(
              entry.name,
              style: TextStyle(fontSize: 23, fontWeight: FontWeight.w800),
            ),
            const SizedBox(height: 8),
            Text(
              entry.description,
              style: TextStyle(
                fontSize: 15,
                height: 1.5,
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
            ),
            const SizedBox(height: 24),
            Text(
              '功能亮点',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.w800),
            ),
            const SizedBox(height: 9),
            _DetailInfoRow(
              icon: Icons.bolt_outlined,
              title: '工作台预览',
              detail: '使用共享 A2UI 组件显示关键状态',
            ),
            _DetailInfoRow(
              icon: Icons.layers_outlined,
              title: '完整页面',
              detail: entry.runtime == 'webview'
                  ? '平台 WebView 中的独立插件页面'
                  : 'Flutter A2UI 兼容页面',
            ),
            const SizedBox(height: 22),
            Text(
              '权限与兼容性',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.w800),
            ),
            const SizedBox(height: 9),
            _DetailInfoRow(
              icon: Icons.devices_outlined,
              title: '支持平台',
              detail: entry.platforms.join(' / '),
            ),
            _DetailInfoRow(
              icon: Icons.verified_user_outlined,
              title: '插件权限',
              detail: permissions == null
                  ? '以安装包清单为准'
                  : permissions.isEmpty
                  ? '不请求 Host 业务能力'
                  : permissions.join('、'),
            ),
            const SizedBox(height: 8),
            Text(
              '版本 ${entry.version} · ${entry.author}',
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
            ),
            if (entry.source != null)
              Text(
                '功能参考：${entry.source}',
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                  fontSize: 11,
                ),
              ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 8,
              children: [
                OutlinedButton(
                  onPressed: supported ? onPreview : null,
                  child: Text('预览'),
                ),
                if (installed)
                  TextButton(
                    onPressed: onToggle,
                    child: Text(enabled ? '禁用' : '启用'),
                  ),
                if (installed)
                  TextButton(
                    key: ValueKey('remove-${entry.id}'),
                    onPressed: () async {
                      final confirmed = await showDialog<bool>(
                        context: context,
                        builder: (dialogContext) => AlertDialog(
                          title: const Text('移除插件？'),
                          content: Text('将从本机移除 ${entry.name}。'),
                          actions: [
                            TextButton(
                              onPressed: () =>
                                  Navigator.pop(dialogContext, false),
                              child: const Text('取消'),
                            ),
                            FilledButton(
                              onPressed: () =>
                                  Navigator.pop(dialogContext, true),
                              child: const Text('移除'),
                            ),
                          ],
                        ),
                      );
                      if (confirmed == true) onRemove();
                    },
                    child: const Text('移除'),
                  ),
              ],
            ),
          ],
        ),
      );
    },
  );
}

class _DetailInfoRow extends StatelessWidget {
  const _DetailInfoRow({
    required this.icon,
    required this.title,
    required this.detail,
  });
  final IconData icon;
  final String title;
  final String detail;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: 8),
    child: Row(
      children: [
        Container(
          width: 39,
          height: 39,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.primaryContainer,
            borderRadius: BorderRadius.circular(11),
          ),
          child: Icon(
            icon,
            color: Theme.of(context).colorScheme.primary,
            size: 20,
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(title, style: TextStyle(fontWeight: FontWeight.w700)),
              Text(
                detail,
                style: TextStyle(
                  fontSize: 12,
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
        ),
      ],
    ),
  );
}

class MarketMediaView extends StatefulWidget {
  const MarketMediaView({super.key, required this.media});
  final MobileMarketMedia media;

  @override
  State<MarketMediaView> createState() => _MarketMediaViewState();
}

class _MarketMediaViewState extends State<MarketMediaView> {
  VideoPlayerController? _video;
  String? _error;

  Future<void> _play() async {
    if (_video != null) {
      await (_video!.value.isPlaying ? _video!.pause() : _video!.play());
      if (mounted) setState(() {});
      return;
    }
    final uri = Uri.tryParse(widget.media.uri);
    if (uri == null || uri.scheme != 'https') {
      setState(() => _error = '视频地址不可用');
      return;
    }
    final controller = VideoPlayerController.networkUrl(uri);
    try {
      await controller.initialize();
      await controller.play();
      if (!mounted) {
        await controller.dispose();
        return;
      }
      setState(() => _video = controller);
    } catch (_) {
      await controller.dispose();
      if (mounted) setState(() => _error = '视频暂时无法播放');
    }
  }

  @override
  void dispose() {
    unawaited(_video?.dispose());
    super.dispose();
  }

  Widget _image(String location) {
    if (location.startsWith('asset:')) {
      return Image.asset(
        location.substring('asset:'.length),
        fit: BoxFit.cover,
        width: double.infinity,
        height: double.infinity,
        errorBuilder: (_, _, _) => const Center(child: Text('图片暂时无法加载')),
      );
    }
    final uri = Uri.tryParse(location);
    if (uri?.scheme != 'https') return const Center(child: Text('媒体不可用'));
    return Image.network(
      location,
      fit: BoxFit.cover,
      width: double.infinity,
      height: double.infinity,
      errorBuilder: (_, _, _) => const Center(child: Text('图片暂时无法加载')),
    );
  }

  @override
  Widget build(BuildContext context) => ClipRRect(
    borderRadius: BorderRadius.circular(20),
    child: ColoredBox(
      color: Theme.of(context).colorScheme.primaryContainer,
      child: Stack(
        fit: StackFit.expand,
        children: [
          if (_video case final video?)
            VideoPlayer(video)
          else if (widget.media.type != 'video')
            _image(widget.media.uri)
          else if (widget.media.poster case final poster?)
            _image(poster),
          if (widget.media.type == 'video')
            Center(
              child: IconButton.filled(
                tooltip: _video?.value.isPlaying == true ? '暂停视频' : '播放视频',
                onPressed: _play,
                icon: Icon(
                  _video?.value.isPlaying == true
                      ? Icons.pause
                      : Icons.play_arrow,
                ),
              ),
            ),
          if (widget.media.type == 'gif')
            const Positioned(
              right: 10,
              bottom: 10,
              child: MarketPlatformBadge('动图'),
            ),
          if (_error != null)
            Align(
              alignment: Alignment.bottomCenter,
              child: Text(
                _error!,
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                ),
              ),
            ),
        ],
      ),
    ),
  );
}
