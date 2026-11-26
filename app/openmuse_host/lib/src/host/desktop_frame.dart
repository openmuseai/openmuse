import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'desktop_icons.dart';
import 'desktop_navigation.dart';

/// The Desktop chrome is a row, including at narrow widths: navigation always
/// pushes the content. Surface identity belongs to the retained canvas child.
final class DesktopFrame extends StatelessWidget {
  const DesktopFrame({
    super.key,
    required this.navigation,
    required this.items,
    required this.explorer,
    required this.content,
    required this.onMarket,
    required this.onSettings,
    required this.onNotifications,
    required this.onAccount,
    required this.title,
    required this.onDrag,
    this.nativeMac = false,
    this.captionButtons,
    this.onSearch,
    this.hasUnreadNotifications = false,
    this.narrowWorkspace = false,
    required this.onNarrowWorkspace,
  });
  final DesktopNavigationController navigation;
  final List<OpenMuseNavigationItem> items;
  final Widget explorer, content;
  final VoidCallback onMarket, onSettings, onNotifications, onAccount, onDrag;
  final String title;
  final bool nativeMac, narrowWorkspace;
  final Widget? captionButtons;
  final VoidCallback? onSearch;
  final bool hasUnreadNotifications;
  final ValueChanged<bool> onNarrowWorkspace;
  bool _zh(BuildContext c) => Localizations.localeOf(c).languageCode == 'zh';
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: navigation,
    builder: (context, _) => LayoutBuilder(
      builder: (context, box) {
        final dark = Theme.of(context).brightness == Brightness.dark;
        final accent = Color(dark ? 0xff8fbea5 : 0xff326451);
        final surface = Color(dark ? 0xff1b211e : 0xffffffff);
        final side = Color(dark ? 0xff202723 : 0xfff7f8f7);
        final rail = Color(dark ? 0xff252d28 : 0xffeef1ef);
        final ink = Color(dark ? 0xffe0e8e2 : 0xff252d29);
        final theme = Theme.of(context).copyWith(
          scaffoldBackgroundColor: surface,
          dividerColor: Color(dark ? 0xff333d36 : 0xffe2e7e3),
          colorScheme: Theme.of(context).colorScheme.copyWith(
            primary: accent,
            surface: surface,
            onSurface: ink,
          ),
          textTheme: Theme.of(context).textTheme.apply(
            fontFamily: '.AppleSystemUIFont',
            bodyColor: ink,
            displayColor: ink,
          ),
          iconTheme: IconThemeData(color: ink, size: 18),
          visualDensity: VisualDensity.compact,
        );
        final selected = items
            .where((i) => i.id == navigation.selected)
            .firstOrNull;
        final visible = navigation.selected == 'explorer' || selected != null;
        final width = box.maxWidth >= 1120 ? 256.0 : 232.0;
        return Theme(
          data: theme,
          child: CallbackShortcuts(
            bindings: {
              const SingleActivator(LogicalKeyboardKey.keyP, meta: true):
                  ?onSearch,
              const SingleActivator(LogicalKeyboardKey.keyP, control: true):
                  ?onSearch,
              const SingleActivator(LogicalKeyboardKey.keyB, meta: true):
                  navigation.toggle,
              const SingleActivator(LogicalKeyboardKey.keyB, control: true):
                  navigation.toggle,
              const SingleActivator(
                LogicalKeyboardKey.keyE,
                meta: true,
                shift: true,
              ): () {
                if (navigation.selected != 'explorer') {
                  navigation.select('explorer');
                }
              },
              const SingleActivator(
                LogicalKeyboardKey.keyE,
                control: true,
                shift: true,
              ): () {
                if (navigation.selected != 'explorer') {
                  navigation.select('explorer');
                }
              },
            },
            child: Focus(
              autofocus: true,
              child: ColoredBox(
                color: surface,
                child: Column(
                  children: [
                    SizedBox(
                      height: 36,
                      child: Listener(
                        behavior: HitTestBehavior.opaque,
                        onPointerDown: (e) {
                          if ((e.buttons & kPrimaryMouseButton) != 0 &&
                              (captionButtons == null ||
                                  e.localPosition.dx < box.maxWidth - 150)) {
                            onDrag();
                          }
                        },
                        child: Padding(
                          padding: EdgeInsets.only(
                            left: nativeMac ? 78 : 16,
                            right: 16,
                          ),
                          child: Row(
                            children: [
                              Expanded(
                                child: Text(
                                  title,
                                  textAlign: TextAlign.center,
                                  style: TextStyle(
                                    fontSize: 11,
                                    color: ink.withValues(alpha: .65),
                                  ),
                                ),
                              ),
                              if (captionButtons != null)
                                captionButtons!
                              else
                                Text(
                                  'Desktop',
                                  style: TextStyle(
                                    fontSize: 10,
                                    color: ink.withValues(alpha: .55),
                                  ),
                                ),
                            ],
                          ),
                        ),
                      ),
                    ),
                    if (box.maxWidth < 900)
                      SizedBox(
                        height: 34,
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            TextButton(
                              key: const ValueKey('desktop-segment-agent'),
                              onPressed: () => onNarrowWorkspace(false),
                              child: Text(
                                'Agent',
                                style: TextStyle(
                                  fontWeight: !narrowWorkspace
                                      ? FontWeight.w600
                                      : FontWeight.normal,
                                ),
                              ),
                            ),
                            TextButton(
                              key: const ValueKey('desktop-segment-workspace'),
                              onPressed: () => onNarrowWorkspace(true),
                              child: Text(
                                _zh(context) ? '工作区' : 'Workspace',
                                style: TextStyle(
                                  fontWeight: narrowWorkspace
                                      ? FontWeight.w600
                                      : FontWeight.normal,
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    Expanded(
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          SizedBox(
                            width: 56,
                            child: ColoredBox(
                              color: rail,
                              child: Column(
                                children: [
                                  const SizedBox(height: 8),
                                  _button(
                                    context,
                                    'explorer',
                                    'Explorer',
                                    'files',
                                    navigation.selected == 'explorer',
                                    () => navigation.select('explorer'),
                                    accent,
                                  ),
                                  for (final item in items)
                                    _button(
                                      context,
                                      item.id,
                                      item.title,
                                      item.icon,
                                      item.id == navigation.selected,
                                      () => navigation.select(item.id),
                                      accent,
                                    ),
                                  const Spacer(),
                                  _button(
                                    context,
                                    'market',
                                    _zh(context)
                                        ? '插件市场'
                                        : 'Plugin Marketplace',
                                    'market',
                                    false,
                                    onMarket,
                                    accent,
                                  ),
                                  _button(
                                    context,
                                    'notifications',
                                    _zh(context) ? '通知' : 'Notifications',
                                    'bell',
                                    false,
                                    onNotifications,
                                    accent,
                                  ),
                                  _button(
                                    context,
                                    'account',
                                    _zh(context) ? '账号' : 'Account',
                                    'user',
                                    false,
                                    onAccount,
                                    accent,
                                  ),
                                  _button(
                                    context,
                                    'settings',
                                    _zh(context) ? '设置' : 'Settings',
                                    'gear',
                                    false,
                                    onSettings,
                                    accent,
                                  ),
                                  const SizedBox(height: 8),
                                ],
                              ),
                            ),
                          ),
                          Offstage(
                            offstage: !visible,
                            child: TickerMode(
                              enabled: visible,
                              child: SizedBox(
                                key: const ValueKey('desktop-sidebar'),
                                width: width,
                                child: DecoratedBox(
                                  decoration: BoxDecoration(
                                    border: Border(
                                      right: BorderSide(
                                        color: theme.dividerColor,
                                      ),
                                    ),
                                  ),
                                  child: Material(
                                    color: side,
                                    child: Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.stretch,
                                      children: [
                                        SizedBox(
                                          height: 38,
                                          child: Padding(
                                            padding: const EdgeInsets.symmetric(
                                              horizontal: 14,
                                            ),
                                            child: Row(
                                              children: [
                                                Expanded(
                                                  child: Text(
                                                    navigation.selected ==
                                                            'explorer'
                                                        ? 'Explorer'
                                                        : selected?.title ??
                                                              'Explorer',
                                                    style: const TextStyle(
                                                      fontSize: 12,
                                                      fontWeight:
                                                          FontWeight.w600,
                                                    ),
                                                  ),
                                                ),
                                                if (selected?.onOpenWorkspace !=
                                                    null)
                                                  IconButton(
                                                    tooltip: _zh(context)
                                                        ? '打开工作区'
                                                        : 'Open workspace',
                                                    icon: const DesktopIcon(
                                                      'arrowout',
                                                      size: 16,
                                                    ),
                                                    onPressed: selected!
                                                        .onOpenWorkspace,
                                                  ),
                                              ],
                                            ),
                                          ),
                                        ),
                                        Expanded(
                                          child: _DesktopSidebarDeck(
                                            selected: navigation.selected,
                                            explorer: explorer,
                                            items: items,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ),
                              ),
                            ),
                          ),
                          Expanded(child: content),
                        ],
                      ),
                    ),
                    Container(
                      height: 23,
                      decoration: BoxDecoration(
                        color: side,
                        border: Border(
                          top: BorderSide(color: theme.dividerColor),
                        ),
                      ),
                      padding: const EdgeInsets.symmetric(horizontal: 12),
                      child: Row(
                        children: [
                          const DesktopIcon('branch', size: 12),
                          const SizedBox(width: 6),
                          Expanded(
                            child: Text(
                              title,
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                              style: const TextStyle(fontSize: 10),
                            ),
                          ),
                          Text(
                            _zh(context) ? '本地工作区' : 'Local workspace',
                            style: const TextStyle(fontSize: 10),
                          ),
                          const SizedBox(width: 20),
                          const Text('UTF-8', style: TextStyle(fontSize: 10)),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        );
      },
    ),
  );
  Widget _button(
    BuildContext context,
    String id,
    String label,
    String icon,
    bool selected,
    VoidCallback tap,
    Color accent,
  ) {
    final coarse =
        MediaQuery.of(context).navigationMode == NavigationMode.directional;
    return Semantics(
      selected: selected,
      button: true,
      label: label,
      child: Tooltip(
        message: label,
        child: SizedBox(
          height: coarse ? 44 : 38,
          width: 56,
          child: Stack(
            children: [
              if (id == 'notifications' && hasUnreadNotifications)
                Positioned(
                  key: const ValueKey('notification-unread-dot'),
                  right: 9,
                  top: 7,
                  child: Container(
                    width: 5,
                    height: 5,
                    decoration: BoxDecoration(
                      color: accent,
                      shape: BoxShape.circle,
                    ),
                  ),
                ),
              if (selected)
                Positioned(
                  left: 0,
                  top: 7,
                  bottom: 7,
                  child: Container(width: 2, color: accent),
                ),
              Positioned.fill(
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 7,
                    vertical: 2,
                  ),
                  child: Material(
                    color: selected
                        ? accent.withValues(alpha: .1)
                        : Colors.transparent,
                    borderRadius: BorderRadius.circular(7),
                    child: InkWell(
                      key: ValueKey('activity-$id'),
                      borderRadius: BorderRadius.circular(7),
                      onTap: tap,
                      child: Center(
                        child: DesktopIcon(
                          icon,
                          color: selected
                              ? accent
                              : Theme.of(
                                  context,
                                ).colorScheme.onSurface.withValues(alpha: .65),
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
    );
  }
}

/// Retain only visited contributors. Hiding/switching never reboots a plugin UI;
/// uninstalling removes its keyed subtree and releases its resources.
final class _DesktopSidebarDeck extends StatefulWidget {
  const _DesktopSidebarDeck({
    required this.selected,
    required this.explorer,
    required this.items,
  });
  final String? selected;
  final Widget explorer;
  final List<OpenMuseNavigationItem> items;
  @override
  State<_DesktopSidebarDeck> createState() => _DesktopSidebarDeckState();
}

final class _DesktopSidebarDeckState extends State<_DesktopSidebarDeck> {
  final Set<String> _visited = {};
  @override
  Widget build(BuildContext context) {
    final available = {'explorer', ...widget.items.map((item) => item.id)};
    _visited.removeWhere((id) => !available.contains(id));
    if (widget.selected != null && available.contains(widget.selected))
      _visited.add(widget.selected!);
    final ids = _visited.toList();
    return Stack(
      children: [
        for (final id in ids)
          Positioned.fill(
            key: ValueKey(id),
            child: Offstage(
              offstage: id != widget.selected,
              child: TickerMode(
                enabled: id == widget.selected,
                child: id == 'explorer'
                    ? widget.explorer
                    : Builder(
                        builder: widget.items
                            .firstWhere((item) => item.id == id)
                            .sidebar,
                      ),
              ),
            ),
          ),
      ],
    );
  }
}
