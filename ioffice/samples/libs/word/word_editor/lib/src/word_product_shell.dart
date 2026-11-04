import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'editor_controller.dart';
import 'word_product_definition.dart';
import 'word_menu_grid.dart';
import 'word_style_card.dart';
import 'word_style_snapshot.dart';
import 'word_statistics_dialog.dart';
import 'word_zoom_dialog.dart';
import 'word_go_to_panel.dart';

/// Native Flutter product chrome inspired by GenOffice Docs. The page child
/// is the existing word_editor canvas, never a DOM/WebView editor.
enum _WordShellDensity { desktop, tablet, foldable, mobile }

class WordProductShell extends StatefulWidget {
  const WordProductShell(
      {super.key,
      required this.definition,
      required this.controller,
      required this.document,
      required this.isCommandAvailable,
      required this.onCommand,
      this.visibleContexts = const {}});

  final WordProductDefinition definition;
  final WordEditorController controller;
  final Widget document;
  final bool Function(String commandId) isCommandAvailable;
  final ValueChanged<String> onCommand;
  final Set<String> visibleContexts;

  @override
  State<WordProductShell> createState() => _WordProductShellState();
}

class _WordProductShellState extends State<WordProductShell> {
  late String _tab = widget.definition.ribbon.initialTab;
  final ScrollController _ribbonScroll = ScrollController();
  bool _showGoTo = false;

  @override
  void dispose() {
    _ribbonScroll.dispose();
    super.dispose();
  }

  WordChromeDefinition get c => widget.definition.chrome;

  Widget _glyph(String name, {required double size, required Color color}) {
    final signedSvg = widget.definition.icons[name];
    if (signedSvg == null) return Icon(_icon(name), size: size, color: color);
    final painted = size >= 20
        ? 1.5
        : size >= 13
            ? 1.25
            : 1.1;
    final stroke = (painted * 16 / size).toStringAsFixed(4);
    return SvgPicture.string(
      signedSvg.replaceFirst('__STROKE_WIDTH__', stroke),
      width: size,
      height: size,
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
    );
  }

  bool _enabled(WordRibbonItem item) =>
      item.command != null &&
      widget.isCommandAvailable(item.command!) &&
      (!(item.command == 'word.view.printLayout' ||
              item.command == 'word.view.readMode' ||
              item.command == 'word.view.zoomDialog' ||
              item.command == 'word.view.zoom100' ||
              item.command == 'word.view.fitWidth' ||
              item.command == 'word.view.fitPage' ||
              item.command == 'word.view.fitMultiplePages' ||
              item.command == 'word.view.gridlines' ||
              item.command == 'word.view.ruler' ||
              item.command == 'word.view.darkMode') ||
          widget.controller.pageCount > 0) &&
      (item.command != 'word.tools.wordCount' ||
          widget.controller.statisticsSnapshot != null);

  void _setReadingMode(bool enabled) {
    final command = enabled ? 'word.view.readMode' : 'word.view.printLayout';
    if (widget.controller.pageCount == 0 || !widget.isCommandAvailable(command))
      return;
    if (_showGoTo) setState(() => _showGoTo = false);
    widget.controller.setReadingMode(enabled);
  }

  void _openStatistics() {
    final snapshot = widget.controller.statisticsSnapshot;
    if (snapshot == null ||
        !widget.isCommandAvailable('word.tools.wordCount')) {
      return;
    }
    showDialog<void>(
      context: context,
      builder: (context) =>
          WordStatisticsDialog(chrome: c, statistics: snapshot),
    );
  }

  void _openZoomDialog() {
    if (widget.controller.pageCount <= 0) return;
    showDialog<void>(
      context: context,
      builder: (context) => WordZoomDialog(
        chrome: c,
        initialZoom: widget.controller.zoom,
        initialFitMode: widget.controller.zoomFitMode,
        onApply: widget.controller.setZoom,
        onFit: widget.controller.setZoomFitMode,
      ),
    );
  }

  void _openGoTo(_WordShellDensity density) {
    if (widget.controller.pageCount <= 0 ||
        !widget.isCommandAvailable('word.navigation.goTo')) return;
    if (density == _WordShellDensity.mobile) {
      showModalBottomSheet<void>(
        context: context,
        isScrollControlled: true,
        useSafeArea: true,
        builder: (sheetContext) => Padding(
          padding: EdgeInsets.fromLTRB(
              12, 12, 12, MediaQuery.viewInsetsOf(sheetContext).bottom + 12),
          child: AnimatedBuilder(
            animation: widget.controller,
            builder: (context, _) => WordGoToPanel(
              definition: c.goTo,
              currentPage: widget.controller.visiblePage,
              pageCount: widget.controller.pageCount,
              onNavigate: widget.controller.navigateToPage,
              onClose: () => Navigator.pop(sheetContext),
            ),
          ),
        ),
      );
    } else {
      setState(() => _showGoTo = true);
    }
  }

  void _run(WordRibbonItem item) {
    if (!_enabled(item)) return;
    if (item.command == 'word.tools.wordCount') {
      _openStatistics();
    } else if (item.command == 'word.view.readMode') {
      _setReadingMode(true);
    } else if (item.command == 'word.view.printLayout') {
      _setReadingMode(false);
    } else if (item.command == 'word.view.zoomDialog') {
      _openZoomDialog();
    } else if (item.command == 'word.view.zoom100') {
      widget.controller.setZoom(1.0);
    } else if (item.command == 'word.view.fitWidth') {
      widget.controller.setZoomFitMode(WordZoomFitMode.pageWidth);
    } else if (item.command == 'word.view.fitPage') {
      widget.controller.setZoomFitMode(WordZoomFitMode.wholePage);
    } else if (item.command == 'word.view.fitMultiplePages') {
      widget.controller.setZoomFitMode(WordZoomFitMode.multiplePages);
    } else if (item.command == 'word.view.gridlines') {
      widget.controller.setShowGridlines(!widget.controller.showGridlines);
    } else if (item.command == 'word.view.ruler') {
      widget.controller.setShowRuler(!widget.controller.showRuler);
    } else if (item.command == 'word.view.darkMode') {
      widget.controller.setDarkPage(!widget.controller.darkPage);
    } else {
      widget.onCommand(item.command!);
    }
  }

  bool _viewActive(WordRibbonItem item) => switch (item.command) {
        'word.view.printLayout' => !widget.controller.readingMode,
        'word.view.readMode' => widget.controller.readingMode,
        'word.view.zoom100' => widget.controller.zoomFitMode == null &&
            (widget.controller.zoom - 1).abs() < 0.001,
        'word.view.fitWidth' =>
          widget.controller.zoomFitMode == WordZoomFitMode.pageWidth,
        'word.view.fitPage' =>
          widget.controller.zoomFitMode == WordZoomFitMode.wholePage,
        'word.view.fitMultiplePages' => widget.controller.multiplePages,
        'word.view.gridlines' => widget.controller.showGridlines,
        'word.view.ruler' => widget.controller.showRuler,
        'word.view.darkMode' => widget.controller.darkPage,
        _ => false,
      };

  _WordShellDensity _density(BuildContext context, double width) {
    final platform = Theme.of(context).platform;
    final touch =
        platform == TargetPlatform.android || platform == TargetPlatform.iOS;
    if (!touch && width >= c.desktopMinWidth) {
      return _WordShellDensity.desktop;
    }
    if (width >= c.tabletMinWidth) return _WordShellDensity.tablet;
    if (width >= c.foldableMinWidth) return _WordShellDensity.foldable;
    return _WordShellDensity.mobile;
  }

  double _ribbonHeight(_WordShellDensity density) => switch (density) {
        _WordShellDensity.desktop => c.ribbonHeight,
        _WordShellDensity.tablet => c.tabletRibbonHeight,
        _WordShellDensity.foldable => c.foldableRibbonHeight,
        _WordShellDensity.mobile => c.compactRibbonHeight,
      };

  @override
  Widget build(BuildContext context) {
    final active = widget.definition.ribbon.tabs.firstWhere(
        (tab) =>
            tab.id == _tab &&
            (tab.context == null ||
                widget.visibleContexts.contains(tab.context)),
        orElse: () => widget.definition.ribbon.tabs.firstWhere(
            (tab) => tab.id == widget.definition.ribbon.initialTab));
    return Focus(
        autofocus: true,
        onKeyEvent: (node, event) {
          if (event is KeyDownEvent &&
              event.logicalKey == LogicalKeyboardKey.escape &&
              widget.controller.readingMode) {
            _setReadingMode(false);
            return KeyEventResult.handled;
          }
          return KeyEventResult.ignored;
        },
        child: LayoutBuilder(builder: (context, constraints) {
          final density = _density(context, constraints.maxWidth);
          return AnimatedBuilder(
            animation: widget.controller,
            builder: (context, _) => Stack(children: [
              ColoredBox(
                color: c.chrome,
                child: Column(children: [
                  if (!widget.controller.readingMode) ...[
                    _tabs(context, active.id, density),
                    _ribbon(active, density),
                  ],
                  Expanded(child: widget.document),
                  _statusBar(density),
                ]),
              ),
              if (_showGoTo && !widget.controller.readingMode)
                Positioned(
                  top: (density == _WordShellDensity.mobile
                          ? c.mobileTabRowHeight
                          : c.tabRowHeight) +
                      _ribbonHeight(density) +
                      8,
                  right: 20,
                  child: WordGoToPanel(
                    definition: c.goTo,
                    currentPage: widget.controller.visiblePage,
                    pageCount: widget.controller.pageCount,
                    onNavigate: widget.controller.navigateToPage,
                    onClose: () => setState(() => _showGoTo = false),
                  ),
                ),
              if (widget.controller.readingMode)
                Positioned(
                  top: 12,
                  right: density == _WordShellDensity.mobile ? 12 : 24,
                  child: Container(
                    decoration: BoxDecoration(
                      color: c.chrome,
                      border: Border.all(color: c.border),
                      borderRadius: BorderRadius.circular(15),
                      boxShadow: const [
                        BoxShadow(
                          color: Color(0x26000000),
                          blurRadius: 6,
                          offset: Offset(0, 2),
                        ),
                      ],
                    ),
                    child: Material(
                      color: Colors.transparent,
                      child: InkWell(
                        key: const Key('word-read-exit'),
                        borderRadius: BorderRadius.circular(15),
                        onTap: () => _setReadingMode(false),
                        child: SizedBox(
                          height: 30,
                          child: Padding(
                            padding: const EdgeInsets.symmetric(horizontal: 14),
                            child: Center(
                              child: Text(c.readExitLabel,
                                  style:
                                      TextStyle(fontSize: 12, color: c.text)),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
            ]),
          );
        }));
  }

  Widget _tabs(
      BuildContext context, String activeTabId, _WordShellDensity density) {
    final mac = Theme.of(context).platform == TargetPlatform.macOS;
    final mobile = density == _WordShellDensity.mobile;
    return Container(
      key: const Key('word-tab-row'),
      height: mobile ? c.mobileTabRowHeight : c.tabRowHeight,
      decoration: BoxDecoration(
          color: c.chrome, border: Border(bottom: BorderSide(color: c.border))),
      child: Row(children: [
        const SizedBox(width: 8),
        for (final action in c.quickAccess)
          action.kind == 'toggle'
              ? _quickToggle(action, mobile: mobile)
              : _smallButton(action, size: mobile ? 44 : 27),
        Container(
            width: 1,
            height: 16,
            color: c.border,
            margin: const EdgeInsets.symmetric(horizontal: 7)),
        Expanded(
            child: ListView(scrollDirection: Axis.horizontal, children: [
          for (final tab in widget.definition.ribbon.tabs)
            if (!(mac && tab.hideOnMac) &&
                (tab.context == null ||
                    widget.visibleContexts.contains(tab.context)))
              _tabButton(tab, activeTabId, density),
        ])),
      ]),
    );
  }

  Widget _tabButton(
      WordRibbonTab tab, String activeTabId, _WordShellDensity density) {
    final selected = tab.id == activeTabId;
    final file = tab.id == 'file';
    final horizontalInset = density == _WordShellDensity.mobile ? 10.0 : 12.0;
    final button = InkWell(
      key: tab.menu == null ? Key('word-tab-${tab.id}') : null,
      onTap: tab.menu == null
          ? () => setState(() {
                _tab = tab.id;
              })
          : null,
      child: SizedBox(
          height: density == _WordShellDensity.mobile
              ? c.mobileTabRowHeight
              : c.tabRowHeight,
          child: file
              ? Container(
                  margin:
                      const EdgeInsets.symmetric(horizontal: 2, vertical: 4),
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  alignment: Alignment.center,
                  decoration: BoxDecoration(
                      color: c.accent, borderRadius: BorderRadius.circular(4)),
                  child: Text(tab.label,
                      style: const TextStyle(
                          color: Colors.white,
                          fontSize: 13,
                          fontWeight: FontWeight.w600)))
              : Stack(alignment: Alignment.center, children: [
                  Padding(
                      padding:
                          EdgeInsets.symmetric(horizontal: horizontalInset),
                      child: Text(tab.label,
                          style: TextStyle(
                              color: selected ? c.accent : c.text,
                              fontSize: 13,
                              fontWeight: FontWeight.w600))),
                  if (selected)
                    Positioned(
                        left: horizontalInset,
                        right: horizontalInset,
                        bottom: 0,
                        child: Container(
                            height: 2.5,
                            decoration: BoxDecoration(
                                color: c.accent,
                                borderRadius: BorderRadius.circular(1)))),
                ])),
    );
    if (tab.menu == null) return button;
    return _menuTrigger(tab.menu!, button,
        title: tab.label, density: density, key: Key('word-tab-${tab.id}'));
  }

  List<WordMenuEntry> _menuEntries(String menuId) {
    final entries =
        widget.definition.ribbon.menus[menuId] ?? const <WordMenuEntry>[];
    if (menuId != 'home.styles' || widget.controller.styleSnapshot == null) {
      return entries;
    }
    final snapshot = widget.controller.styleSnapshot!;
    return [
      for (final entry in entries)
        if (entry.presentation == WordGridPresentation.styleGallery)
          WordMenuEntry(
            id: entry.id,
            label: entry.label,
            kind: entry.kind,
            presentation: entry.presentation,
            columns: entry.columns,
            children: [
              for (final style in snapshot.quickStyles)
                WordMenuEntry(
                  id: 'document.style.${style.type}.${style.styleId}',
                  label: style.name,
                  kind: 'action',
                  styleInfo: style,
                ),
            ],
          )
        else
          entry,
    ];
  }

  List<Widget> _menuChildren(String menuId) => [
        const SizedBox(width: 190, height: 0),
        for (final entry in _menuEntries(menuId)) _menuEntry(entry),
      ];

  MenuStyle _menuStyle({String? menuId}) {
    final viewport = MediaQuery.sizeOf(context);
    return MenuStyle(
        backgroundColor: WidgetStatePropertyAll(c.chrome),
        surfaceTintColor: const WidgetStatePropertyAll(Colors.transparent),
        side: WidgetStatePropertyAll(BorderSide(color: c.border)),
        shape: WidgetStatePropertyAll(RoundedRectangleBorder(
            side: BorderSide(color: c.border),
            borderRadius: const BorderRadius.all(Radius.circular(3)))),
        elevation: const WidgetStatePropertyAll(1),
        minimumSize: const WidgetStatePropertyAll(Size(190, 0)),
        maximumSize: WidgetStatePropertyAll(Size(
            (viewport.width - 16)
                .clamp(
                    120.0,
                    switch (menuId) {
                      'home.styles' => 478.0,
                      'insert.cover' => 460.0,
                      _ => 320.0,
                    })
                .toDouble(),
            (viewport.height - 72)
                .clamp(160.0, menuId == 'insert.cover' ? 420.0 : 640.0)
                .toDouble())));
  }

  Widget _menuEntry(WordMenuEntry entry) {
    switch (entry.kind) {
      case 'separator':
        return const Divider(height: 7, thickness: 1);
      case 'heading':
        return Padding(
            padding: const EdgeInsets.fromLTRB(12, 7, 12, 3),
            child: Text(entry.label,
                style: TextStyle(
                    fontSize: 11,
                    fontWeight: FontWeight.w600,
                    color: c.muted)));
      case 'submenu':
        return SubmenuButton(
            key: Key('word-menu-${entry.id}'),
            menuStyle: _menuStyle(),
            style: const ButtonStyle(
                minimumSize: WidgetStatePropertyAll(Size(0, 34))),
            menuChildren: [
              const SizedBox(width: 190, height: 0),
              for (final child in entry.children) _menuEntry(child)
            ],
            child: Text(entry.label));
      case 'grid':
        return WordMenuGrid(
            entry: entry,
            chrome: c,
            touch: false,
            styleSnapshot: widget.controller.styleSnapshot,
            isCommandAvailable: widget.isCommandAvailable,
            onCommand: widget.onCommand);
      default:
        final enabled =
            entry.command != null && widget.isCommandAvailable(entry.command!);
        return MenuItemButton(
            key: Key('word-menu-${entry.id}'),
            style: const ButtonStyle(
                minimumSize: WidgetStatePropertyAll(Size(0, 34))),
            onPressed: enabled ? () => widget.onCommand(entry.command!) : null,
            trailingIcon: entry.shortcut == null
                ? null
                : Text(entry.shortcut!,
                    style: TextStyle(fontSize: 11, color: c.muted)),
            child: Text(entry.label));
    }
  }

  Widget _menuTrigger(String menuId, Widget child,
      {required String title, required _WordShellDensity density, Key? key}) {
    final touch = Theme.of(context).platform == TargetPlatform.android ||
        Theme.of(context).platform == TargetPlatform.iOS;
    if (touch ||
        density == _WordShellDensity.mobile ||
        density == _WordShellDensity.foldable) {
      return InkWell(
          key: key,
          onTap: () => showModalBottomSheet<void>(
                context: context,
                useSafeArea: true,
                isScrollControlled: true,
                backgroundColor: c.chrome,
                builder: (_) => SizedBox(
                    height: (MediaQuery.sizeOf(context).height * 0.82)
                        .clamp(280.0, menuId == 'home.styles' ? 340.0 : 640.0),
                    child: _WordTouchMenu(
                        title: title,
                        entries: _menuEntries(menuId),
                        chrome: c,
                        styleSnapshot: widget.controller.styleSnapshot,
                        isCommandAvailable: widget.isCommandAvailable,
                        onCommand: widget.onCommand)),
              ),
          child: child);
    }
    return MenuAnchor(
      menuChildren: _menuChildren(menuId),
      style: _menuStyle(menuId: menuId),
      builder: (context, controller, _) => InkWell(
          key: key,
          onTap: () =>
              controller.isOpen ? controller.close() : controller.open(),
          child: child),
    );
  }

  Widget _ribbon(WordRibbonTab tab, _WordShellDensity density) => Container(
        key: const Key('word-ribbon'),
        height: _ribbonHeight(density),
        decoration: BoxDecoration(
            color: c.chrome,
            border: Border(bottom: BorderSide(color: c.border))),
        child: Scrollbar(
            controller: _ribbonScroll,
            thumbVisibility: density == _WordShellDensity.desktop,
            trackVisibility: density == _WordShellDensity.desktop,
            scrollbarOrientation: ScrollbarOrientation.bottom,
            thickness: 5,
            radius: const Radius.circular(2.5),
            child: ListView(
                controller: _ribbonScroll,
                scrollDirection: Axis.horizontal,
                padding: EdgeInsets.fromLTRB(
                    density == _WordShellDensity.desktop ? 12 : 5,
                    density == _WordShellDensity.mobile ? 0 : 6,
                    5,
                    density == _WordShellDensity.mobile ? 0 : 6),
                children: [
                  for (var i = 0; i < tab.groups.length; i++) ...[
                    if (i > 0)
                      Container(
                          width: 1,
                          margin: const EdgeInsets.symmetric(
                              horizontal: 6, vertical: 3),
                          color: c.border),
                    _group(tab.groups[i], density),
                  ],
                ])),
      );

  Widget _group(WordRibbonGroup group, _WordShellDensity density) {
    final items = group.items;
    if (group.gallerySource == 'document.quickStyles') {
      return _styleGalleryGroup(group, density);
    }
    if (density != _WordShellDensity.desktop) {
      return Row(mainAxisSize: MainAxisSize.min, children: [
        for (final item in items) _control(item, density: density),
      ]);
    }
    if (Theme.of(context).platform == TargetPlatform.macOS) {
      return Center(child: _desktopGroupItems(group));
    }
    return Column(children: [
      Expanded(child: Center(child: _desktopGroupItems(group))),
      SizedBox(
          height: 15,
          child: Text(group.label,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(fontSize: 10, color: c.muted))),
    ]);
  }

  Widget _styleGalleryGroup(WordRibbonGroup group, _WordShellDensity density) {
    if (density != _WordShellDensity.desktop) {
      return _menuTrigger(
        'home.styles',
        SizedBox(
          width: density == _WordShellDensity.tablet ? 58 : 44,
          height: _ribbonHeight(density),
          child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
            _glyph('title', size: 19, color: c.text),
            if (density == _WordShellDensity.tablet)
              Text(group.label, style: TextStyle(fontSize: 9, color: c.muted)),
          ]),
        ),
        title: group.label,
        density: density,
        key: const Key('word-control-styles-gallery'),
      );
    }
    final gallery = _menuEntries('home.styles').where(
        (entry) => entry.presentation == WordGridPresentation.styleGallery);
    final cards =
        gallery.isEmpty ? const <WordMenuEntry>[] : gallery.first.children;
    final snapshot = widget.controller.styleSnapshot;
    final content = Row(mainAxisSize: MainAxisSize.min, children: [
      for (final cell in cards.take(2)) ...[
        WordStyleCard(
          key: Key('word-inline-style-${cell.styleInfo?.styleId ?? cell.id}'),
          style: cell.styleInfo!,
          label: cell.label,
          chrome: c,
          snapshot: snapshot,
          active: snapshot?.activeStyleKey ==
              '${cell.styleInfo!.type == 'character' ? 'char' : 'para'}:${cell.styleInfo!.styleId}',
        ),
        const SizedBox(width: 4),
      ],
      if (cards.isNotEmpty)
        _menuTrigger(
          'home.styles',
          SizedBox(
            width: 17,
            height: 62,
            child: DecoratedBox(
              decoration: BoxDecoration(
                border: Border.all(color: c.border),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Icon(Icons.arrow_drop_down, size: 15, color: c.muted),
            ),
          ),
          title: group.label,
          density: density,
          key: const Key('word-control-styles-more'),
        ),
      if (group.items.isNotEmpty) ...[
        const SizedBox(width: 4),
        _control(group.items.first),
      ],
    ]);
    if (Theme.of(context).platform == TargetPlatform.macOS) {
      return Center(child: content);
    }
    return Column(children: [
      Expanded(child: Center(child: content)),
      SizedBox(
          height: 15,
          child: Text(group.label,
              style: TextStyle(fontSize: 10, color: c.muted))),
    ]);
  }

  Widget _desktopGroupItems(WordRibbonGroup group) {
    final items = group.items;
    switch (group.layout) {
      case 'twoRow':
        return Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          Row(mainAxisSize: MainAxisSize.min, children: [
            for (final item in items.where((item) => item.row == 0))
              _control(item)
          ]),
          Row(mainAxisSize: MainAxisSize.min, children: [
            for (final item in items.where((item) => item.row == 1))
              _control(item)
          ]),
        ]);
      case 'clipboard':
        return Row(mainAxisSize: MainAxisSize.min, children: [
          if (items.isNotEmpty) _control(items.first),
          Column(mainAxisAlignment: MainAxisAlignment.center, children: [
            for (final item in items.skip(1)) _control(item, tiny: true)
          ]),
        ]);
      default:
        return Row(
            mainAxisSize: MainAxisSize.min,
            children: [for (final item in items) _control(item)]);
    }
  }

  Widget _control(WordRibbonItem item,
      {bool tiny = false,
      _WordShellDensity density = _WordShellDensity.desktop}) {
    final enabled = _enabled(item);
    final hasMenu = item.menu != null;
    final active = enabled && _viewActive(item);
    final icon = _glyph(item.icon,
        size: item.kind == 'large' ? 24 : 18,
        color: enabled ? c.text : c.muted.withValues(alpha: 0.7));
    Widget iconPlate(Widget glyph) => active
        ? Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
            decoration: BoxDecoration(
                color: const Color(0xFFE8E8E8),
                borderRadius: BorderRadius.circular(4)),
            child: glyph,
          )
        : glyph;
    Widget content;
    if (density != _WordShellDensity.desktop) {
      final tablet = density == _WordShellDensity.tablet;
      content = SizedBox(
          width: tablet ? 58 : 44,
          height: _ribbonHeight(density) -
              (density == _WordShellDensity.mobile ? 0 : 6),
          child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
            iconPlate(_glyph(item.icon,
                size: tablet ? 20 : 19,
                color: enabled ? c.text : c.muted.withValues(alpha: 0.7))),
            if (tablet)
              Text(item.label,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(fontSize: 9, color: c.muted)),
          ]));
    } else {
      switch (item.kind) {
        case 'combo':
          content = Container(
              width: item.width,
              height: 27,
              margin: const EdgeInsets.all(2),
              padding: const EdgeInsets.symmetric(horizontal: 5),
              decoration: BoxDecoration(
                  border: Border.all(color: c.border),
                  borderRadius: BorderRadius.circular(3)),
              child: Row(children: [
                Expanded(
                    child: Text(item.label,
                        style: TextStyle(color: c.muted, fontSize: 11))),
                Icon(Icons.arrow_drop_down, size: 15, color: c.muted)
              ]));
        case 'style':
          content = Container(
              width: 67,
              height: 57,
              margin: const EdgeInsets.symmetric(horizontal: 2),
              decoration: BoxDecoration(
                  border: Border.all(color: c.border),
                  borderRadius: BorderRadius.circular(3)),
              child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text('Aa文',
                        style: TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                            color: enabled
                                ? c.text
                                : c.muted.withValues(alpha: 0.7))),
                    Text(item.label,
                        style: TextStyle(fontSize: 10, color: c.muted)),
                  ]));
        case 'large':
          content = SizedBox(
              width: item.label.length > 5 ? 68 : 60,
              height: 58,
              child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    iconPlate(icon),
                    const SizedBox(height: 3),
                    Text(item.label,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                            color: enabled ? c.text : c.muted, fontSize: 10.5)),
                  ]));
        default:
          content = SizedBox(
              width: tiny ? 28 : 33,
              height: tiny ? 19 : 28,
              child: Center(child: icon));
      }
    }
    if (hasMenu && item.kind == 'large') {
      content = Stack(children: [
        content,
        Positioned(
            right: 2,
            top: density == _WordShellDensity.desktop ? 20 : 2,
            child: const Icon(Icons.arrow_drop_down, size: 12)),
      ]);
    }
    final button = Semantics(
        button: true,
        enabled: enabled || hasMenu,
        selected: active,
        label: item.label,
        child: InkWell(
            key: hasMenu ? null : Key('word-control-${item.id}'),
            onTap: hasMenu ? null : (enabled ? () => _run(item) : null),
            borderRadius: BorderRadius.circular(3),
            child: content));
    return Tooltip(
      message: enabled || hasMenu ? item.label : '${item.label}（暂不可用）',
      child: Semantics(
          container: true,
          child: hasMenu
              ? _menuTrigger(item.menu!, button,
                  title: item.label,
                  density: density,
                  key: Key('word-control-${item.id}'))
              : button),
    );
  }

  Widget _smallButton(WordRibbonItem item, {double size = 20}) {
    final enabled = _enabled(item);
    return Tooltip(
        message: enabled ? item.label : '${item.label}（暂不可用）',
        child: IconButton(
          key: Key('word-quick-${item.id}'),
          tooltip: null,
          onPressed: enabled ? () => _run(item) : null,
          icon: _glyph(item.icon,
              size: 16,
              color: enabled ? c.muted : c.muted.withValues(alpha: 0.45)),
          style: IconButton.styleFrom(
              fixedSize: Size(size, size),
              padding: EdgeInsets.zero,
              foregroundColor: c.muted,
              disabledForegroundColor: c.muted.withValues(alpha: 0.45)),
        ));
  }

  Widget _quickToggle(WordRibbonItem item, {bool mobile = false}) => Tooltip(
        message: '${item.label}（暂不可用）',
        child: Semantics(
            button: true,
            enabled: false,
            label: item.label,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 5),
              child: Row(mainAxisSize: MainAxisSize.min, children: [
                Container(
                    width: 23,
                    height: 13,
                    decoration: BoxDecoration(
                        color: c.border,
                        borderRadius: BorderRadius.circular(8)),
                    child: Align(
                        alignment: Alignment.centerLeft,
                        child: Container(
                            width: 10,
                            height: 10,
                            margin: const EdgeInsets.only(left: 2),
                            decoration: const BoxDecoration(
                                color: Colors.white, shape: BoxShape.circle)))),
                if (!mobile) ...[
                  const SizedBox(width: 4),
                  Text(item.label,
                      style: TextStyle(color: c.muted, fontSize: 10.5)),
                ],
              ]),
            )),
      );

  Widget _statusBar(_WordShellDensity density) {
    final ctl = widget.controller;
    final pages = ctl.pageCount;
    final page = pages == 0
        ? c.readyLabel
        : c.pageLabel
            .replaceAll('{current}', '${ctl.visiblePage}')
            .replaceAll('{total}', '$pages');
    final count = ctl.wordCount;
    final totalWords =
        c.wordCountLabel.replaceAll('{count}', '${count ?? '—'}');
    final selectedWords = c.selectedWordCountLabel
        .replaceAll('{selected}', '${ctl.selectedWordCount}')
        .replaceAll('{total}', '${count ?? '—'}');
    return LayoutBuilder(builder: (context, constraints) {
      final roomy = constraints.maxWidth >= 850;
      final medium = constraints.maxWidth >= 520;
      final showWordCount =
          roomy || medium || (constraints.maxWidth >= 375 && count != null);
      return Container(
        key: const Key('word-status'),
        height: density == _WordShellDensity.mobile
            ? c.mobileStatusHeight
            : c.statusHeight,
        padding: const EdgeInsets.symmetric(horizontal: 12),
        decoration: BoxDecoration(
            color: c.status, border: Border(top: BorderSide(color: c.border))),
        child: Row(children: [
          Tooltip(
            message: c.goTo.title,
            child: InkWell(
              key: const Key('word-status-page'),
              onTap: () => _openGoTo(density),
              child: SizedBox(
                height: density == _WordShellDensity.mobile ? 44 : 20,
                child: Center(
                  child: Text(page,
                      style: TextStyle(fontSize: 11.5, color: c.muted)),
                ),
              ),
            ),
          ),
          if (showWordCount) ...[
            SizedBox(width: roomy ? 14 : 8),
            Tooltip(
                message: c.statisticsTitle,
                child: InkWell(
                    key: const Key('word-status-word-count'),
                    onTap: _openStatistics,
                    child: Text(
                        ctl.selectedWordCount > 0 && roomy
                            ? selectedWords
                            : totalWords,
                        style: TextStyle(
                            fontSize: roomy ? 11.5 : 10.5, color: c.muted)))),
          ],
          if (roomy) ...[
            const SizedBox(width: 14),
            Text(c.revisionLabel,
                style: TextStyle(fontSize: 11.5, color: c.muted)),
          ],
          if (roomy &&
              (pages == 0 || ctl.status.startsWith('error:')) &&
              ctl.status.isNotEmpty) ...[
            const SizedBox(width: 8),
            Flexible(
                child: Text('— ${ctl.status}',
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(fontSize: 11, color: c.muted))),
          ],
          const Spacer(),
          if (roomy)
            for (final mode in c.viewModes)
              Tooltip(
                message: _enabled(mode) ? mode.label : '${mode.label}（暂不可用）',
                child: InkWell(
                  key: Key('word-status-view-${mode.id}'),
                  onTap: _enabled(mode)
                      ? () => mode.id == 'read' && ctl.readingMode
                          ? _setReadingMode(false)
                          : _run(mode)
                      : null,
                  borderRadius: BorderRadius.circular(3),
                  child: Container(
                    width: 20,
                    height: 20,
                    margin: const EdgeInsets.only(right: 2),
                    decoration: BoxDecoration(
                      color: (mode.id == 'print' && !ctl.readingMode) ||
                              (mode.id == 'read' && ctl.readingMode)
                          ? c.accent.withValues(alpha: 0.1)
                          : null,
                      borderRadius: BorderRadius.circular(3),
                    ),
                    child: Center(
                      child: _glyph(mode.icon,
                          size: 13,
                          color: (mode.id == 'print' && !ctl.readingMode) ||
                                  (mode.id == 'read' && ctl.readingMode)
                              ? c.accent
                              : _enabled(mode)
                                  ? c.muted
                                  : c.muted.withValues(alpha: 0.5)),
                    ),
                  ),
                ),
              ),
          const SizedBox(width: 8),
          _zoomButton(Icons.remove,
              () => ctl.setZoom(ctl.zoom - c.zoomStepPercent / 100),
              size: density == _WordShellDensity.mobile ? 44 : 20),
          if (medium)
            SizedBox(
                width: 110,
                child: SliderTheme(
                    data: SliderTheme.of(context).copyWith(
                      trackHeight: 3,
                      thumbShape:
                          const RoundSliderThumbShape(enabledThumbRadius: 7),
                      overlayShape: SliderComponentShape.noOverlay,
                      thumbColor: c.muted,
                    ),
                    child: Slider(
                        value: ctl.zoom.clamp(
                            c.zoomMinPercent / 100, c.zoomMaxPercent / 100),
                        min: c.zoomMinPercent / 100,
                        max: c.zoomMaxPercent / 100,
                        activeColor: c.muted,
                        inactiveColor: c.border,
                        onChanged: ctl.setZoom))),
          _zoomButton(
              Icons.add, () => ctl.setZoom(ctl.zoom + c.zoomStepPercent / 100),
              size: density == _WordShellDensity.mobile ? 44 : 20),
          Tooltip(
            message: '缩放',
            child: InkWell(
              key: const Key('word-status-zoom-value'),
              onTap: pages == 0 ? null : _openZoomDialog,
              borderRadius: BorderRadius.circular(3),
              child: SizedBox(
                width: 42,
                height: density == _WordShellDensity.mobile ? 44 : 20,
                child: Align(
                  alignment: Alignment.centerRight,
                  child: Text('${(ctl.zoom * 100).round()}%',
                      style: TextStyle(fontSize: 11.5, color: c.muted)),
                ),
              ),
            ),
          ),
        ]),
      );
    });
  }

  Widget _zoomButton(IconData icon, VoidCallback onTap, {double size = 20}) =>
      SizedBox(
          width: size,
          height: size,
          child: InkWell(
              onTap: onTap, child: Icon(icon, size: 14, color: c.muted)));
}

/// Touch navigation for the same signed menu tree used by desktop MenuAnchor.
/// A level occupies the sheet width, so nested menus remain reachable on phones.
class _WordTouchMenu extends StatefulWidget {
  const _WordTouchMenu({
    required this.title,
    required this.entries,
    required this.chrome,
    this.styleSnapshot,
    required this.isCommandAvailable,
    required this.onCommand,
  });

  final String title;
  final List<WordMenuEntry> entries;
  final WordChromeDefinition chrome;
  final WordStyleSnapshot? styleSnapshot;
  final bool Function(String commandId) isCommandAvailable;
  final ValueChanged<String> onCommand;

  @override
  State<_WordTouchMenu> createState() => _WordTouchMenuState();
}

class _WordTouchMenuState extends State<_WordTouchMenu> {
  final List<WordMenuEntry> _path = [];

  List<WordMenuEntry> get _entries =>
      _path.isEmpty ? widget.entries : _path.last.children;

  bool _available(WordMenuEntry entry) =>
      entry.command != null && widget.isCommandAvailable(entry.command!);

  void _dispatch(WordMenuEntry entry) {
    if (!_available(entry)) return;
    widget.onCommand(entry.command!);
    Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) {
    final c = widget.chrome;
    final title = _path.isEmpty ? widget.title : _path.last.label;
    return Column(children: [
      SizedBox(
          height: 52,
          child: Row(children: [
            IconButton(
                key: const Key('word-menu-back'),
                tooltip: _path.isEmpty ? '关闭' : '返回上一级',
                onPressed: () {
                  if (_path.isEmpty) {
                    Navigator.of(context).pop();
                  } else {
                    setState(() => _path.removeLast());
                  }
                },
                icon: Icon(_path.isEmpty ? Icons.close : Icons.arrow_back)),
            Expanded(
                child: Text(title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                        fontSize: 16,
                        color: c.text,
                        fontWeight: FontWeight.w600))),
            Text('${_path.length + 1}',
                style: TextStyle(fontSize: 11, color: c.muted)),
            const SizedBox(width: 16),
          ])),
      Divider(height: 1, color: c.border),
      Expanded(
          child: ListView.builder(
              key: Key('word-menu-level-${_path.length}'),
              itemCount: _entries.length,
              itemBuilder: (context, index) => _entry(_entries[index]))),
    ]);
  }

  Widget _entry(WordMenuEntry entry) {
    final c = widget.chrome;
    switch (entry.kind) {
      case 'separator':
        return Divider(height: 12, color: c.border);
      case 'heading':
        return Padding(
            padding: const EdgeInsets.fromLTRB(16, 13, 16, 5),
            child: Text(entry.label,
                style: TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    color: c.muted)));
      case 'submenu':
        return ListTile(
            key: Key('word-menu-${entry.id}'),
            dense: true,
            visualDensity: VisualDensity.compact,
            title: Text(entry.label),
            trailing: const Icon(Icons.chevron_right, size: 20),
            onTap: () => setState(() => _path.add(entry)));
      case 'grid':
        return WordMenuGrid(
            entry: entry,
            chrome: c,
            touch: true,
            styleSnapshot: widget.styleSnapshot,
            isCommandAvailable: widget.isCommandAvailable,
            onCommand: (command) {
              widget.onCommand(command);
              Navigator.of(context).pop();
            });
      default:
        return ListTile(
            key: Key('word-menu-${entry.id}'),
            dense: true,
            visualDensity: VisualDensity.compact,
            enabled: _available(entry),
            title: Text(entry.label),
            trailing: entry.shortcut == null
                ? null
                : Text(entry.shortcut!,
                    style: TextStyle(fontSize: 11, color: c.muted)),
            onTap: _available(entry) ? () => _dispatch(entry) : null);
    }
  }
}

IconData _icon(String name) => switch (name) {
      'save' => Icons.save_outlined,
      'undo' => Icons.undo,
      'redo' => Icons.redo,
      'toggle_off' => Icons.toggle_off,
      'folder_open' => Icons.folder_open,
      'auto_awesome' => Icons.auto_awesome,
      'article' => Icons.article_outlined,
      'edit_spark' => Icons.auto_fix_high,
      'account_tree' => Icons.account_tree_outlined,
      'sync' => Icons.sync,
      'content_paste' => Icons.content_paste,
      'content_cut' => Icons.content_cut,
      'content_copy' => Icons.content_copy,
      'format_paint' => Icons.format_paint,
      'font_download' => Icons.font_download,
      'format_size' => Icons.format_size,
      'text_increase' => Icons.text_increase,
      'text_decrease' => Icons.text_decrease,
      'text_fields' => Icons.text_fields,
      'format_clear' => Icons.format_clear,
      'format_bold' => Icons.format_bold,
      'format_italic' => Icons.format_italic,
      'format_underline' => Icons.format_underlined,
      'strikethrough_s' => Icons.strikethrough_s,
      'subscript' => Icons.subscript,
      'superscript' => Icons.superscript,
      'border_color' => Icons.border_color,
      'format_color_text' => Icons.format_color_text,
      'format_list_bulleted' => Icons.format_list_bulleted,
      'format_list_numbered' => Icons.format_list_numbered,
      'list' => Icons.list,
      'format_indent_decrease' => Icons.format_indent_decrease,
      'format_indent_increase' => Icons.format_indent_increase,
      'notes' => Icons.notes,
      'sort_by_alpha' => Icons.sort_by_alpha,
      'format_align_left' => Icons.format_align_left,
      'format_align_center' => Icons.format_align_center,
      'format_align_right' => Icons.format_align_right,
      'format_align_justify' => Icons.format_align_justify,
      'format_line_spacing' => Icons.format_line_spacing,
      'format_color_fill' => Icons.format_color_fill,
      'border_all' => Icons.border_all,
      'title' => Icons.title,
      'insert_page_break' => Icons.insert_page_break,
      'table_chart' => Icons.table_chart,
      'image' => Icons.image_outlined,
      'category' => Icons.category_outlined,
      'bar_chart' => Icons.bar_chart,
      'link' => Icons.link,
      'draw' => Icons.draw,
      'auto_fix_normal' => Icons.auto_fix_normal,
      'palette' => Icons.palette_outlined,
      'color_lens' => Icons.color_lens_outlined,
      'water_drop' => Icons.water_drop_outlined,
      'crop_free' => Icons.crop_free,
      'screen_rotation' => Icons.screen_rotation,
      'description' => Icons.description_outlined,
      'print_layout' => Icons.description_outlined,
      'web_layout' => Icons.language,
      'outline_view' => Icons.list,
      'read_mode' => Icons.menu_book_outlined,
      'view_column' => Icons.view_column,
      'toc' => Icons.toc,
      'note_add' => Icons.note_add_outlined,
      'format_quote' => Icons.format_quote,
      'spellcheck' => Icons.spellcheck,
      'add_comment' => Icons.add_comment_outlined,
      'history' => Icons.history,
      'language' => Icons.language,
      'menu_book' => Icons.menu_book_outlined,
      'zoom_out' => Icons.zoom_out,
      'zoom_in' => Icons.zoom_in,
      'numbers' => Icons.numbers,
      'vertical_align_top' => Icons.vertical_align_top,
      'vertical_align_bottom' => Icons.vertical_align_bottom,
      'functions' => Icons.functions,
      'open_with' => Icons.open_with,
      'wrap_text' => Icons.wrap_text,
      'translate' => Icons.translate,
      'delete_outline' => Icons.delete_outline,
      'visibility' => Icons.visibility_outlined,
      'check' => Icons.check,
      'close' => Icons.close,
      'select_all' => Icons.select_all,
      'fit_screen' => Icons.fit_screen,
      'text_rotation_none' => Icons.text_rotation_none,
      'rotate_right' => Icons.rotate_right,
      'settings' => Icons.settings_outlined,
      'access_time' => Icons.access_time,
      _ => Icons.help_outline,
    };
