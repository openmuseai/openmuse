import 'package:pub_semver/pub_semver.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:video_player/video_player.dart';

bool marketHasUpdate(MobilePluginStore store, MobileMarketEntry entry) {
  final installed = store.installed(entry.id);
  if (installed == null) return false;
  try {
    return Version.parse(entry.version) > Version.parse(installed.version);
  } on FormatException {
    return false;
  }
}

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
    this.showSections = true,
    this.showAppBar = false,
    this.onDiscover,
    this.onManageBuiltIn,
    this.searchController,
    this.onRetry,
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
  final bool showSections, showAppBar;
  final VoidCallback? onDiscover;
  final ValueChanged<OpenMusePlugin>? onManageBuiltIn;
  final TextEditingController? searchController;
  final VoidCallback? onRetry;
  final String? error;
  final String? busy;

  @override
  Widget build(BuildContext context) {
    final filtered = entries.where((entry) {
      final categoryMatch = mine
          ? category == '全部' ||
                (category == '已启用' && store.isEnabled(entry.id)) ||
                (category == '已停用' && !store.isEnabled(entry.id)) ||
                (category == '更新' && marketHasUpdate(store, entry))
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
      appBar: showAppBar
          ? AppBar(
              title: Text(
                mine
                    ? stringsOf(context).marketMyPlugins()
                    : stringsOf(context).marketCategoryDiscover(),
              ),
            )
          : null,
      body: CustomScrollView(
        key: PageStorageKey('mobile-market-${mine ? 'installed' : 'discover'}'),
        slivers: [
          if (showSections)
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(20, 16, 20, 8),
                child: LayoutBuilder(
                  builder: (context, constraints) {
                    final sections = [
                      (false, stringsOf(context).marketCategoryDiscover()),
                      (true, stringsOf(context).marketMyPlugins()),
                    ];
                    final scaler = MediaQuery.textScalerOf(context);
                    var naturalWidth = 0.0;
                    for (final section in sections) {
                      final painter = TextPainter(
                        text: TextSpan(
                          text: section.$2,
                          style: TextStyle(
                            fontSize: section.$1 == mine ? 30 : 24,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        textDirection: Directionality.of(context),
                        textScaler: scaler,
                      )..layout();
                      naturalWidth += painter.width + 20;
                      painter.dispose();
                    }
                    final scale = (constraints.maxWidth / naturalWidth).clamp(
                      .8,
                      1.0,
                    );
                    return SingleChildScrollView(
                      key: const ValueKey('mobile-market-sections'),
                      scrollDirection: Axis.horizontal,
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          for (final section in sections)
                            Padding(
                              padding: const EdgeInsets.only(right: 20),
                              child: Semantics(
                                label: section.$1
                                    ? stringsOf(context).marketMyPlugins()
                                    : stringsOf(
                                        context,
                                      ).marketDiscoverPlugins(),
                                excludeSemantics: true,
                                selected: mine == section.$1,
                                button: true,
                                child: InkWell(
                                  key: ValueKey(
                                    'mobile-market-section-${section.$1 ? '我的' : '发现'}',
                                  ),
                                  borderRadius: BorderRadius.circular(8),
                                  onTap: () => onSection(section.$1),
                                  child: Padding(
                                    padding: const EdgeInsets.symmetric(
                                      vertical: 10,
                                    ),
                                    child: AnimatedDefaultTextStyle(
                                      duration:
                                          MediaQuery.disableAnimationsOf(
                                            context,
                                          )
                                          ? Duration.zero
                                          : const Duration(milliseconds: 180),
                                      style: TextStyle(
                                        fontSize:
                                            (mine == section.$1 ? 30 : 24) *
                                            scale,
                                        height: 1.15,
                                        fontWeight: mine == section.$1
                                            ? FontWeight.w700
                                            : FontWeight.w500,
                                        letterSpacing: -.6,
                                        color: mine == section.$1
                                            ? Theme.of(
                                                context,
                                              ).colorScheme.onSurface
                                            : Theme.of(
                                                context,
                                              ).colorScheme.onSurfaceVariant,
                                      ),
                                      child: Text(
                                        section.$2,
                                        maxLines: 1,
                                        softWrap: false,
                                      ),
                                    ),
                                  ),
                                ),
                              ),
                            ),
                        ],
                      ),
                    );
                  },
                ),
              ),
            ),
          if (mine && !showSections)
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(20, 6, 20, 18),
                child: Text(
                  stringsOf(context).mobilePluginsOnThisDevice(),
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                    fontSize: 12,
                  ),
                ),
              ),
            ),
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 6, 20, 10),
              child: TextField(
                key: const ValueKey('mobile-market-search'),
                controller: searchController,
                onChanged: onSearch,
                decoration: InputDecoration(
                  prefixIcon: Icon(Icons.search_rounded),
                  hintText: mine
                      ? stringsOf(context).marketSearchInstalled()
                      : stringsOf(context).marketSearchPlugins(),
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
          if (mine)
            SliverToBoxAdapter(
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                padding: const EdgeInsets.symmetric(
                  horizontal: 20,
                  vertical: 6,
                ),
                child: Row(
                  children: [
                    for (final item
                        in (showSections
                            ? const ['全部', '已启用', '已停用', '更新']
                            : const ['全部', '更新', '已停用']))
                      Padding(
                        padding: const EdgeInsets.only(right: 8),
                        child: ChoiceChip(
                          key: ValueKey('market-category-$item'),
                          label: Text(marketCategoryLabel(context, item)),
                          showCheckmark: false,
                          selectedColor: Theme.of(
                            context,
                          ).colorScheme.secondaryContainer,
                          labelStyle: TextStyle(
                            color: category == item
                                ? Theme.of(context).colorScheme.primary
                                : Theme.of(
                                    context,
                                  ).colorScheme.onSurfaceVariant,
                          ),
                          selected: category == item,
                          onSelected: (_) => onCategory(item),
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
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          error!,
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.error,
                          ),
                        ),
                        if (onRetry != null)
                          TextButton.icon(
                            key: const ValueKey('market-retry'),
                            onPressed: onRetry,
                            icon: const Icon(Icons.refresh, size: 18),
                            label: Text(stringsOf(context).commonRetry()),
                          ),
                      ],
                    ),
                  ),
                if (!mine &&
                    featured != null &&
                    featuredArt != null &&
                    category == '精选' &&
                    query.isEmpty) ...[
                  MarketHeroArt(
                    asset: featuredArt.uri.substring('asset:'.length),
                    height: showSections ? 300 : 220,
                    onTap: () => onDetail(featured),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          stringsOf(context).marketFeaturedCategory(
                            category: marketCategoryLabel(
                              context,
                              featured.categories.firstOrNull ?? '插件',
                            ),
                          ),
                          style: TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.w700,
                            color: const Color(0xFF1765E9),
                          ),
                        ),
                        const SizedBox(height: 14),
                        Text(
                          stringsOf(context).marketHero(),
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
                          label: Text(
                            stringsOf(context).marketViewPlugin(
                              name: marketDisplayName(featured),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 24),
                ],
                if (!mine || showSections)
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          mine
                              ? stringsOf(context).marketMyPlugins()
                              : category == '精选'
                              ? stringsOf(context).marketRecommended()
                              : stringsOf(context).marketCategoryPlugins(
                                  category: marketCategoryLabel(
                                    context,
                                    category,
                                  ),
                                ),
                          style: TextStyle(
                            fontSize: 21,
                            fontWeight: FontWeight.w800,
                            color: Theme.of(context).colorScheme.onSurface,
                          ),
                        ),
                      ),
                      Text(
                        mcpSection == null
                            ? stringsOf(context).marketPluginCount(
                                count: filtered.length + visibleBuiltIns.length,
                              )
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
                          stringsOf(context).marketCategoryEmpty(),
                          style: TextStyle(
                            fontWeight: FontWeight.w700,
                            fontSize: 16,
                          ),
                        ),
                        SizedBox(height: 5),
                        Text(
                          stringsOf(context).marketCategoryEmptyDetail(),
                          style: TextStyle(
                            color: Theme.of(
                              context,
                            ).colorScheme.onSurfaceVariant,
                          ),
                        ),
                      ],
                    ),
                  ),
                if (mcpSection != null && !mine) mcpSection!,
                for (final entry in filtered)
                  _MarketEntryTile(
                    entry: entry,
                    store: store,
                    management: mine && !showSections,
                    busy: busy == entry.id,
                    onDetail: () => onDetail(entry),
                    onInstall: () => onInstall(entry),
                    onOpen: () => onOpen(entry),
                    onPreview: () => onPreview(entry),
                  ),
                if (visibleBuiltIns.isNotEmpty) ...[
                  const SizedBox(height: 14),
                  if (!mine || showSections)
                    Text(
                      stringsOf(context).marketBuiltInTools(),
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
                          title: Text(
                            builtinPluginDisplayName(
                              context,
                              plugin.descriptor.id,
                              plugin.descriptor.name,
                            ),
                          ),
                          subtitle: Text(
                            builtInState?.call(plugin) ==
                                    OpenMusePluginState.active
                                ? (!showSections
                                      ? stringsOf(context).mobileEnabledHere()
                                      : stringsOf(context).marketBuiltInReady())
                                : (!showSections
                                      ? stringsOf(context).mobileDisabledHere()
                                      : stringsOf(
                                          context,
                                        ).marketBuiltInDisabled()),
                          ),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: mine && onManageBuiltIn != null
                              ? () => onManageBuiltIn!(plugin)
                              : builtInState?.call(plugin) ==
                                    OpenMusePluginState.active
                              ? () => onOpenBuiltIn?.call(plugin)
                              : null,
                        ),
                      ),
                    ),
                ],
                if (mine && mcpSection != null) mcpSection!,
                if (mine && onDiscover != null)
                  Padding(
                    padding: const EdgeInsets.only(top: 20),
                    child: OutlinedButton.icon(
                      key: const ValueKey('market-discover-more'),
                      onPressed: onDiscover,
                      icon: const Icon(Icons.add),
                      label: Text(stringsOf(context).mobileFindMoreApps()),
                    ),
                  ),
                if (mine && onManageWorkbench != null) ...[
                  const SizedBox(height: 20),
                  Material(
                    color: Theme.of(context).colorScheme.surface,
                    borderRadius: BorderRadius.circular(18),
                    child: ListTile(
                      key: const ValueKey('market-manage-workbench'),
                      leading: const Icon(Icons.dashboard_customize_outlined),
                      title: Text(stringsOf(context).marketWorkbenchDisplay()),
                      subtitle: Text(stringsOf(context).marketManageCards()),
                      trailing: const Icon(Icons.chevron_right),
                      onTap: onManageWorkbench,
                    ),
                  ),
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
    this.management = false,
    required this.onDetail,
    required this.onInstall,
    required this.onOpen,
    required this.onPreview,
  });
  final MobileMarketEntry entry;
  final MobilePluginStore store;
  final bool busy, management;
  final VoidCallback onDetail;
  final VoidCallback onInstall;
  final VoidCallback onOpen;
  final VoidCallback onPreview;

  @override
  Widget build(BuildContext context) {
    final installed = store.isInstalled(entry.id);
    final enabled = store.isEnabled(entry.id);
    final supported = marketSupportsCurrentMobile(entry);
    final updateAvailable = marketHasUpdate(store, entry);
    if (management) {
      return Padding(
        padding: const EdgeInsets.only(bottom: 10),
        child: Material(
          color: Theme.of(context).colorScheme.surface,
          borderRadius: BorderRadius.circular(18),
          child: ListTile(
            key: ValueKey('market-installed-${entry.id}'),
            leading: MarketPluginIcon(entry.id, size: 32),
            title: Text(
              marketDisplayName(entry),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            subtitle: Text(
              '${updateAvailable
                  ? stringsOf(context).marketUpdate()
                  : enabled
                  ? stringsOf(context).mobileEnabledHere()
                  : stringsOf(context).mobileDisabledHere()} · ${store.installed(entry.id)!.version}',
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: onDetail,
          ),
        ),
      );
    }
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
                              ? stringsOf(context).marketUnsupported()
                              : busy
                              ? stringsOf(context).marketProcessing()
                              : updateAvailable
                              ? stringsOf(context).marketUpdate()
                              : stringsOf(context).marketInstall(),
                        ),
                      )
                    else
                      FilledButton(
                        key: ValueKey('open-${entry.id}'),
                        onPressed: enabled && supported ? onOpen : null,
                        child: Text(
                          !supported
                              ? stringsOf(context).marketUnsupported()
                              : enabled
                              ? stringsOf(context).marketOpen()
                              : stringsOf(context).marketDisabled(),
                        ),
                      ),
                    TextButton(
                      key: ValueKey('preview-${entry.id}'),
                      onPressed: supported && !busy && entry.bundle.isNotEmpty
                          ? onPreview
                          : null,
                      child: Text(stringsOf(context).marketPreview()),
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
    this.operation,
    this.installationFailure,
    this.onShowOverview,
  });
  final MobileMarketEntry entry;
  final MobilePluginStore store;
  final VoidCallback onInstall;
  final VoidCallback onOpen;
  final VoidCallback onPreview;
  final VoidCallback onToggle;
  final VoidCallback onRemove;
  final ValueListenable<String?>? operation, installationFailure;
  final VoidCallback? onShowOverview;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: Listenable.merge([store, ?operation, ?installationFailure]),
    builder: (context, _) {
      final pending = operation?.value == entry.id;
      final installed = store.isInstalled(entry.id);
      final enabled = store.isEnabled(entry.id);
      final supported = marketSupportsCurrentMobile(entry);
      final updateAvailable = marketHasUpdate(store, entry);
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
                  onPressed: !supported || pending
                      ? null
                      : (!installed || updateAvailable)
                      ? onInstall
                      : enabled
                      ? onOpen
                      : onToggle,
                  child: Text(
                    pending
                        ? stringsOf(context).marketProcessing()
                        : !supported
                        ? stringsOf(context).marketDeviceUnsupported()
                        : updateAvailable
                        ? stringsOf(context).marketUpdate()
                        : !installed
                        ? stringsOf(context).marketInstall()
                        : enabled
                        ? stringsOf(context).marketOpen()
                        : stringsOf(context).marketEnable(),
                  ),
                ),
              ],
            ),
          ),
        ),
        body: ListView(
          padding: const EdgeInsets.fromLTRB(20, 12, 20, 28),
          children: [
            if (installationFailure?.value case final failure?)
              Padding(
                padding: const EdgeInsets.only(bottom: 16),
                child: Text(
                  failure,
                  style: TextStyle(color: Theme.of(context).colorScheme.error),
                ),
              ),
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
              stringsOf(context).marketHighlights(),
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.w800),
            ),
            const SizedBox(height: 9),
            _DetailInfoRow(
              icon: Icons.bolt_outlined,
              title: stringsOf(context).marketWorkbenchPreview(),
              detail: stringsOf(context).marketA2uiState(),
            ),
            _DetailInfoRow(
              icon: Icons.layers_outlined,
              title: stringsOf(context).marketFullPage(),
              detail: entry.runtime == 'webview'
                  ? stringsOf(context).marketWebviewPage()
                  : stringsOf(context).marketA2uiPage(),
            ),
            const SizedBox(height: 22),
            Text(
              stringsOf(context).marketPermissionsCompatibility(),
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.w800),
            ),
            const SizedBox(height: 9),
            _DetailInfoRow(
              icon: Icons.devices_outlined,
              title: stringsOf(context).marketPlatforms(),
              detail: entry.platforms.join(' / '),
            ),
            _DetailInfoRow(
              icon: Icons.verified_user_outlined,
              title: stringsOf(context).marketPermissions(),
              detail: permissions == null
                  ? stringsOf(context).marketManifestPermissions()
                  : permissions.isEmpty
                  ? stringsOf(context).marketNoPermissions()
                  : permissions.join('、'),
            ),
            const SizedBox(height: 8),
            Text(
              stringsOf(context).marketVersionAuthor(
                version: entry.version,
                author: entry.author,
              ),
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
            ),
            if (entry.source != null)
              Text(
                stringsOf(context).marketReference(source: entry.source),
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                  fontSize: 11,
                ),
              ),
            if (installed && enabled && onShowOverview != null)
              TextButton.icon(
                key: const ValueKey('market-show-overview'),
                onPressed: () {
                  Navigator.of(context).pop();
                  onShowOverview!();
                },
                icon: const Icon(Icons.grid_view_outlined),
                label: Text(stringsOf(context).mobileYourOverview()),
              ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 8,
              children: [
                OutlinedButton(
                  onPressed: supported && !pending && entry.bundle.isNotEmpty
                      ? onPreview
                      : null,
                  child: Text(stringsOf(context).marketPreview()),
                ),
                if (installed)
                  TextButton(
                    onPressed: pending ? null : onToggle,
                    child: Text(
                      enabled
                          ? stringsOf(context).marketDisable()
                          : stringsOf(context).marketEnable(),
                    ),
                  ),
                if (installed)
                  TextButton(
                    key: ValueKey('remove-${entry.id}'),
                    onPressed: pending
                        ? null
                        : () async {
                            final confirmed = await showDialog<bool>(
                              context: context,
                              builder: (dialogContext) => AlertDialog(
                                title: Text(
                                  stringsOf(context).marketRemoveTitle(),
                                ),
                                content: Text(
                                  stringsOf(
                                    context,
                                  ).marketRemoveDescription(name: entry.name),
                                ),
                                actions: [
                                  TextButton(
                                    onPressed: () =>
                                        Navigator.pop(dialogContext, false),
                                    child: Text(
                                      stringsOf(context).marketCancel(),
                                    ),
                                  ),
                                  FilledButton(
                                    onPressed: () =>
                                        Navigator.pop(dialogContext, true),
                                    child: Text(
                                      stringsOf(context).marketRemove(),
                                    ),
                                  ),
                                ],
                              ),
                            );
                            if (confirmed == true) onRemove();
                          },
                    child: Text(stringsOf(context).marketRemove()),
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
      setState(() => _error = 'VIDEO_ADDRESS');
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
      if (mounted) setState(() => _error = 'VIDEO_PLAYBACK');
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
        errorBuilder: (_, _, _) =>
            Center(child: Text(stringsOf(context).marketImageUnavailable())),
      );
    }
    final uri = Uri.tryParse(location);
    if (uri?.scheme != 'https')
      return Center(child: Text(stringsOf(context).marketMediaUnavailable()));
    return Image.network(
      location,
      fit: BoxFit.cover,
      width: double.infinity,
      height: double.infinity,
      errorBuilder: (_, _, _) =>
          Center(child: Text(stringsOf(context).marketImageUnavailable())),
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
                tooltip: _video?.value.isPlaying == true
                    ? stringsOf(context).marketPauseVideo()
                    : stringsOf(context).marketPlayVideo(),
                onPressed: _play,
                icon: Icon(
                  _video?.value.isPlaying == true
                      ? Icons.pause
                      : Icons.play_arrow,
                ),
              ),
            ),
          if (widget.media.type == 'gif')
            Positioned(
              right: 10,
              bottom: 10,
              child: MarketPlatformBadge(stringsOf(context).marketGif()),
            ),
          if (_error != null)
            Align(
              alignment: Alignment.bottomCenter,
              child: Text(
                _error == 'VIDEO_ADDRESS'
                    ? stringsOf(context).marketMediaUnavailable()
                    : stringsOf(context).marketVideoUnavailable(),
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

String marketCategoryLabel(BuildContext context, String id) => switch (id) {
  '精选' => stringsOf(context).marketCategoryFeatured(),
  '办公' => stringsOf(context).marketCategoryOffice(),
  'UI' => stringsOf(context).marketCategoryUi(),
  '编程' => stringsOf(context).marketCategoryCoding(),
  '生活' => stringsOf(context).marketCategoryLife(),
  '全部' => stringsOf(context).marketCategoryAll(),
  '已启用' => stringsOf(context).marketCategoryEnabled(),
  '已停用' => stringsOf(context).marketCategoryInactive(),
  '更新' => stringsOf(context).marketCategoryUpdates(),
  '发现' => stringsOf(context).marketCategoryDiscover(),
  '我的' => stringsOf(context).marketCategoryMine(),
  '插件' => stringsOf(context).marketCategoryLabelPlugins(),
  _ => id,
};

/// A quiet, reduced-motion loading state with the same search/feature/list rhythm.
class MarketLoadingView extends StatelessWidget {
  const MarketLoadingView({super.key});
  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    Widget block(double height, {double? width}) => Container(
      height: height,
      width: width,
      decoration: BoxDecoration(
        color: colors.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(16),
      ),
    );
    return Semantics(
      label: stringsOf(context).htmlLoading(),
      child: ExcludeSemantics(
        child: ListView(
          padding: const EdgeInsets.all(20),
          children: [
            block(48),
            const SizedBox(height: 20),
            block(220),
            const SizedBox(height: 28),
            for (var i = 0; i < 3; i++)
              Padding(
                padding: const EdgeInsets.only(bottom: 20),
                child: Row(
                  children: [
                    block(48, width: 48),
                    const SizedBox(width: 14),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          block(14),
                          const SizedBox(height: 10),
                          block(10, width: 140),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
          ],
        ),
      ),
    );
  }
}
