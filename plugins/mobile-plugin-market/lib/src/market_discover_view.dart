import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:video_player/video_player.dart';

const marketCategories = ['精选', '办公', 'UI', '编程', '生活'];

String marketDisplayName(MobileMarketEntry entry) =>
    entry.name.replaceFirst(' Mobile 适配演示', '');

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
    this.builtInPlugins = const [],
    this.builtInState,
    this.onOpenBuiltIn,
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
  final List<OpenMusePlugin> builtInPlugins;
  final OpenMusePluginState? Function(OpenMusePlugin)? builtInState;
  final ValueChanged<OpenMusePlugin>? onOpenBuiltIn;
  final String? error;
  final String? busy;

  @override
  Widget build(BuildContext context) {
    final filtered = entries.where((entry) {
      final categoryMatch = mine
          ? category == '全部' ||
                (category == '已启用' && store.isEnabled(entry.id)) ||
                (category == '已停用' && !store.isEnabled(entry.id))
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
      if (!mine && category != '精选') return false;
      final text = query.trim().toLowerCase();
      return text.isEmpty ||
          '${plugin.descriptor.name} ${plugin.descriptor.id}'
              .toLowerCase()
              .contains(text);
    }).toList();
    final featured = filtered.isEmpty ? null : filtered.first;
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
                      child: InkWell(
                        key: ValueKey('mobile-market-section-${section.$2}'),
                        onTap: () => onSection(section.$1),
                        child: AnimatedDefaultTextStyle(
                          duration: const Duration(milliseconds: 200),
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
                      in mine ? const ['全部', '已启用', '已停用'] : marketCategories)
                    Padding(
                      padding: const EdgeInsets.only(right: 23),
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
                              duration: const Duration(milliseconds: 180),
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
                    category == '精选' &&
                    query.isEmpty) ...[
                  MarketHeroArt(
                    asset: MarketArt.taskBoard,
                    height: 220,
                    onTap: () => onDetail(featured),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '本周精选 · 办公',
                          style: TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.w700,
                            color: Theme.of(context).colorScheme.primary,
                          ),
                        ),
                        const SizedBox(height: 14),
                        Text(
                          '把一天，\n安排得刚刚好',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface,
                            fontSize: 25,
                            height: 1.17,
                            fontWeight: FontWeight.w800,
                            letterSpacing: -0.6,
                          ),
                        ),
                        const Spacer(),
                        Text(
                          '${marketDisplayName(featured)} · 图文介绍',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface,
                            fontSize: 12,
                            fontWeight: FontWeight.w600,
                          ),
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
                      '${filtered.length + visibleBuiltIns.length} 个',
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 11),
                if (filtered.isEmpty && visibleBuiltIns.isEmpty)
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
                if (category == '精选' && query.isEmpty) ...[
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
                      const MarketPlatformBadge('Mobile'),
                    ],
                  ),
                ),
                const SizedBox(width: 8),
                Column(
                  children: [
                    if (!installed)
                      FilledButton(
                        key: ValueKey('install-${entry.id}'),
                        onPressed: busy ? null : onInstall,
                        child: Text(busy ? '安装中' : '安装'),
                      )
                    else
                      FilledButton(
                        key: ValueKey('open-${entry.id}'),
                        onPressed: enabled ? onOpen : null,
                        child: Text(enabled ? '打开' : '已禁用'),
                      ),
                    TextButton(
                      key: ValueKey('preview-${entry.id}'),
                      onPressed: onPreview,
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
  });
  final MobileMarketEntry entry;
  final MobilePluginStore store;
  final VoidCallback onInstall;
  final VoidCallback onOpen;
  final VoidCallback onPreview;
  final VoidCallback onToggle;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: store,
    builder: (context, _) {
      final installed = store.isInstalled(entry.id);
      final enabled = store.isEnabled(entry.id);
      final name = marketDisplayName(entry);
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
                  onPressed: !installed
                      ? onInstall
                      : enabled
                      ? onOpen
                      : onToggle,
                  child: Text(
                    !installed
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
                  ? MarketHeroArt(
                      asset: MarketArt.taskBoard,
                      height: 216,
                      child: const SizedBox.shrink(),
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
              entry.id.contains('task-board') ? '让每个想法都有下一步' : '看清用量，安排预算',
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
              title: entry.id.contains('task-board') ? '快速收集' : '预算设置',
              detail: '在本机完成轻量交互与即时反馈',
            ),
            const _DetailInfoRow(
              icon: Icons.layers_outlined,
              title: '原生界面',
              detail: 'A2UI 由 Flutter 组件渲染',
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
            const _DetailInfoRow(
              icon: Icons.verified_user_outlined,
              title: '插件权限',
              detail: '当前演示不请求 Host 业务能力',
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
                OutlinedButton(onPressed: onPreview, child: Text('预览')),
                if (installed)
                  TextButton(
                    onPressed: onToggle,
                    child: Text(enabled ? '禁用' : '启用'),
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
