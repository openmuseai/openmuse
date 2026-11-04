import 'package:flutter/material.dart';

import 'word_style_snapshot.dart';

/// Signed product resources describe presentation only. The Host owns command
/// availability and dispatch; the document remains in word-core.
class WordProductDefinition {
  const WordProductDefinition(
      {required this.ribbon, required this.chrome, this.icons = const {}});

  final WordRibbonDefinition ribbon;
  final WordChromeDefinition chrome;
  final Map<String, String> icons;

  factory WordProductDefinition.fromJson({
    required Map<String, dynamic> ribbon,
    required Map<String, dynamic> chrome,
    Map<String, dynamic>? menus,
    Map<String, dynamic>? icons,
  }) =>
      WordProductDefinition(
        ribbon: WordRibbonDefinition.fromJson(ribbon, menus: menus),
        chrome: WordChromeDefinition.fromJson(chrome),
        icons: icons == null
            ? const {}
            : Map<String, String>.from(_map(icons['icons'])),
      );
}

Map<String, dynamic> _map(Object? value) =>
    Map<String, dynamic>.from(value as Map);

class WordRibbonDefinition {
  const WordRibbonDefinition(
      {required this.initialTab, required this.tabs, required this.menus});

  final String initialTab;
  final List<WordRibbonTab> tabs;
  final Map<String, List<WordMenuEntry>> menus;

  factory WordRibbonDefinition.fromJson(Map<String, dynamic> json,
      {Map<String, dynamic>? menus}) {
    if (json['profile'] != 'openmuse-word-ribbon/2') {
      throw const FormatException('Unsupported Word ribbon profile');
    }
    final tabs = (json['tabs'] as List)
        .map((item) => WordRibbonTab.fromJson(_map(item)))
        .toList(growable: false);
    final initial = json['initialTab'] as String;
    if (!tabs.any((tab) => tab.id == initial)) {
      throw const FormatException('Unknown initial Word ribbon tab');
    }
    if (menus != null && menus['profile'] != 'openmuse-word-menus/1') {
      throw const FormatException('Unsupported Word menu profile');
    }
    final rawMenus = menus?['menus'] as Map? ?? const {};
    final parsedMenus = <String, List<WordMenuEntry>>{};
    for (final entry in rawMenus.entries) {
      parsedMenus[entry.key as String] = (entry.value as List)
          .map((item) => WordMenuEntry.fromJson(_map(item)))
          .toList(growable: false);
    }
    return WordRibbonDefinition(
        initialTab: initial, tabs: tabs, menus: parsedMenus);
  }
}

/// Menu nodes are signed product data. Only a leaf with a negotiated command
/// can mutate a document; headings and submenus never dispatch commands.
enum WordGridPresentation {
  generic,
  colorPalette,
  highlightPalette,
  tablePicker,
  styleGallery,
  cardGallery,
  wordArtGallery,
  symbolGallery,
  shapeGallery,
  positionPicker,
  bulletGallery,
  numberGallery,
  multilevelGallery,
  listLevels,
  dropCapGallery,
  equationGallery,
}

class WordMenuEntry {
  const WordMenuEntry(
      {required this.id,
      required this.label,
      required this.kind,
      this.command,
      this.shortcut,
      this.swatch,
      this.previewPath,
      this.previewText,
      this.previewParas = const [],
      this.previewWordArt,
      this.styleInfo,
      this.presentation = WordGridPresentation.generic,
      this.columns = 0,
      this.children = const []});

  final String id;
  final String label;
  final String kind;
  final String? command;
  final String? shortcut;
  final String? swatch;
  final String? previewPath;
  final String? previewText;
  final List<WordCoverPreviewParagraph> previewParas;
  final WordArtPreviewStyle? previewWordArt;
  final WordStyleInfo? styleInfo;
  final WordGridPresentation presentation;
  final int columns;
  final List<WordMenuEntry> children;

  factory WordMenuEntry.fromJson(Map<String, dynamic> json) => WordMenuEntry(
        id: json['id'] as String,
        label: json['label'] as String? ?? '',
        kind: json['kind'] as String? ?? 'action',
        command: json['command'] as String?,
        shortcut: json['shortcut'] as String?,
        swatch: json['swatch'] as String?,
        previewPath: json['previewPath'] as String?,
        previewText: json['previewText'] as String?,
        previewParas: (json['previewParas'] as List? ?? const [])
            .map((item) => WordCoverPreviewParagraph.fromJson(_map(item)))
            .toList(growable: false),
        previewWordArt: json['previewWordArt'] == null
            ? null
            : WordArtPreviewStyle.fromJson(_map(json['previewWordArt'])),
        styleInfo: json['styleInfo'] == null
            ? null
            : WordStyleInfo.fromJson(_map(json['styleInfo'])),
        presentation: WordGridPresentation.values
            .byName(json['presentation'] as String? ?? 'generic'),
        columns: json['columns'] as int? ?? 0,
        children: (json['children'] as List? ?? const [])
            .map((item) => WordMenuEntry.fromJson(_map(item)))
            .toList(growable: false),
      );
}

/// A signed, static gallery recipe. The live document and selection are not
/// represented here; editor commands remain capability-gated by the Host.
class WordArtPreviewStyle {
  const WordArtPreviewStyle({
    required this.fill,
    required this.bold,
    required this.italic,
    this.outlineColor,
    this.outlineWidthEmu,
  });

  final String fill;
  final bool bold;
  final bool italic;
  final String? outlineColor;
  final int? outlineWidthEmu;

  factory WordArtPreviewStyle.fromJson(Map<String, dynamic> json) {
    final outline = json['outline'] as Map?;
    return WordArtPreviewStyle(
      fill: json['fill'] as String,
      bold: json['bold'] as bool? ?? false,
      italic: json['italic'] as bool? ?? false,
      outlineColor: outline?['color'] as String?,
      outlineWidthEmu: outline?['widthEmu'] as int?,
    );
  }
}

/// Immutable product thumbnail data. It has no relationship to the live
/// document model or the editor selection.
class WordCoverPreviewParagraph {
  const WordCoverPreviewParagraph({
    this.text = '',
    this.bold = false,
    this.italic = false,
    this.color,
    this.sizeHalfPoints,
    this.align = 'left',
    this.spaceBefore = 0,
    this.spaceAfter = 0,
    this.shadingFill,
    this.borders = '',
    this.indentLeft = 0,
    this.indentRight = 0,
  });

  final String text;
  final bool bold;
  final bool italic;
  final String? color;
  final int? sizeHalfPoints;
  final String align;
  final int spaceBefore;
  final int spaceAfter;
  final String? shadingFill;
  final String borders;
  final int indentLeft;
  final int indentRight;

  factory WordCoverPreviewParagraph.fromJson(Map<String, dynamic> json) =>
      WordCoverPreviewParagraph(
        text: json['text'] as String? ?? '',
        bold: json['bold'] as bool? ?? false,
        italic: json['italic'] as bool? ?? false,
        color: json['color'] as String?,
        sizeHalfPoints: json['sizeHalfPoints'] as int?,
        align: json['align'] as String? ?? 'left',
        spaceBefore: json['spaceBefore'] as int? ?? 0,
        spaceAfter: json['spaceAfter'] as int? ?? 0,
        shadingFill: json['shadingFill'] as String?,
        borders: json['borders'] as String? ?? '',
        indentLeft: json['indentLeft'] as int? ?? 0,
        indentRight: json['indentRight'] as int? ?? 0,
      );
}

class WordRibbonTab {
  const WordRibbonTab(
      {required this.id,
      required this.label,
      required this.groups,
      required this.hideOnMac,
      this.menu,
      this.context});

  final String id;
  final String label;
  final bool hideOnMac;
  final String? menu;
  final String? context;
  final List<WordRibbonGroup> groups;

  factory WordRibbonTab.fromJson(Map<String, dynamic> json) => WordRibbonTab(
        id: json['id'] as String,
        label: json['label'] as String,
        hideOnMac: json['hideOnMac'] == true,
        menu: json['menu'] as String?,
        context: json['context'] as String?,
        groups: (json['groups'] as List)
            .map((item) => WordRibbonGroup.fromJson(_map(item)))
            .toList(growable: false),
      );
}

class WordRibbonGroup {
  const WordRibbonGroup(
      {required this.id,
      required this.label,
      required this.layout,
      required this.items,
      this.gallerySource});

  final String id;
  final String label;
  final String layout;
  final List<WordRibbonItem> items;
  final String? gallerySource;

  factory WordRibbonGroup.fromJson(Map<String, dynamic> json) =>
      WordRibbonGroup(
        id: json['id'] as String,
        label: json['label'] as String? ?? '',
        layout: json['layout'] as String,
        gallerySource: json['gallerySource'] as String?,
        items: (json['items'] as List)
            .map((item) => WordRibbonItem.fromJson(_map(item)))
            .toList(growable: false),
      );
}

class WordRibbonItem {
  const WordRibbonItem(
      {required this.id,
      required this.label,
      required this.kind,
      required this.icon,
      this.command,
      this.menu,
      this.row = 0,
      this.width = 0});

  final String id;
  final String label;
  final String kind;
  final String icon;
  final String? command;
  final String? menu;
  final int row;
  final double width;

  factory WordRibbonItem.fromJson(Map<String, dynamic> json) => WordRibbonItem(
        id: json['id'] as String,
        label: json['label'] as String,
        kind: json['kind'] as String,
        icon: json['icon'] as String,
        command: json['command'] as String?,
        menu: json['menu'] as String?,
        row: json['row'] as int? ?? 0,
        width: (json['width'] as num?)?.toDouble() ?? 0,
      );
}

class WordStatisticsRowDefinition {
  const WordStatisticsRowDefinition(this.id, this.label);

  final String id;
  final String label;
}

/// Product copy for the low-frequency page navigation panel. The editor owns
/// the current page and the workspace owns scrolling.
class WordGoToDefinition {
  const WordGoToDefinition({
    this.find = '查找',
    this.replace = '替换',
    this.title = '定位',
    this.page = '页',
    this.numberHint = '数字，或 +N / -N',
    this.relativeHint = '输入 + 或 - 表示相对当前位置移动',
    this.notFound = '未找到',
    this.previous = '上一个',
    this.next = '下一个',
    this.close = '关闭',
  });

  final String find;
  final String replace;
  final String title;
  final String page;
  final String numberHint;
  final String relativeHint;
  final String notFound;
  final String previous;
  final String next;
  final String close;

  factory WordGoToDefinition.fromJson(Map<String, dynamic> json) =>
      WordGoToDefinition(
        find: json['find'] as String? ?? '查找',
        replace: json['replace'] as String? ?? '替换',
        title: json['title'] as String? ?? '定位',
        page: json['page'] as String? ?? '页',
        numberHint: json['numberHint'] as String? ?? '数字，或 +N / -N',
        relativeHint: json['relativeHint'] as String? ?? '输入 + 或 - 表示相对当前位置移动',
        notFound: json['notFound'] as String? ?? '未找到',
        previous: json['previous'] as String? ?? '上一个',
        next: json['next'] as String? ?? '下一个',
        close: json['close'] as String? ?? '关闭',
      );
}

class WordChromeDefinition {
  const WordChromeDefinition(
      {required this.tabRowHeight,
      required this.ribbonHeight,
      required this.compactRibbonHeight,
      required this.statusHeight,
      required this.desktopMinWidth,
      required this.tabletMinWidth,
      required this.foldableMinWidth,
      required this.tabletRibbonHeight,
      required this.foldableRibbonHeight,
      required this.mobileTabRowHeight,
      required this.mobileStatusHeight,
      required this.pageTopInset,
      required this.pageBottomInset,
      required this.pageGap,
      required this.canvas,
      required this.chrome,
      required this.status,
      required this.border,
      required this.accent,
      required this.text,
      required this.muted,
      required this.quickAccess,
      required this.viewModes,
      required this.pageLabel,
      required this.wordCountLabel,
      required this.selectedWordCountLabel,
      required this.statisticsTitle,
      required this.statisticsCloseLabel,
      required this.statisticsRows,
      this.goTo = const WordGoToDefinition(),
      this.readExitLabel = '退出阅读视图 (Esc)',
      required this.revisionLabel,
      required this.readyLabel,
      this.zoomMinPercent = 10,
      this.zoomMaxPercent = 500,
      this.zoomStepPercent = 10,
      this.zoomPresets = const [200, 100, 75]});

  final double tabRowHeight;
  final double ribbonHeight;
  final double compactRibbonHeight;
  final double statusHeight;
  final double desktopMinWidth;
  final double tabletMinWidth;
  final double foldableMinWidth;
  final double tabletRibbonHeight;
  final double foldableRibbonHeight;
  final double mobileTabRowHeight;
  final double mobileStatusHeight;
  final double pageTopInset;
  final double pageBottomInset;
  final double pageGap;
  final Color canvas;
  final Color chrome;
  final Color status;
  final Color border;
  final Color accent;
  final Color text;
  final Color muted;
  final List<WordRibbonItem> quickAccess;
  final List<WordRibbonItem> viewModes;
  final String pageLabel;
  final String wordCountLabel;
  final String selectedWordCountLabel;
  final String statisticsTitle;
  final String statisticsCloseLabel;
  final List<WordStatisticsRowDefinition> statisticsRows;
  final WordGoToDefinition goTo;
  final String readExitLabel;
  final String revisionLabel;
  final String readyLabel;

  /// View-only zoom policy supplied by the signed product package.
  final int zoomMinPercent;
  final int zoomMaxPercent;
  final int zoomStepPercent;
  final List<int> zoomPresets;

  static Color colorFromHex(String value) =>
      Color(int.parse(value.substring(1), radix: 16) | 0xff000000);

  factory WordChromeDefinition.fromJson(Map<String, dynamic> json) {
    if (json['profile'] != 'openmuse-word-chrome/1') {
      throw const FormatException('Unsupported Word chrome profile');
    }
    final metrics = _map(json['metrics']);
    final responsive = _map(json['responsive'] ?? const <String, dynamic>{});
    final colors = _map(json['colors']);
    final status = _map(json['status']);
    final zoom = _map(status['zoom'] ?? const <String, dynamic>{});
    final statistics = _map(status['statistics'] ?? const <String, dynamic>{});
    final goTo = _map(status['goTo'] ?? const <String, dynamic>{});
    const fallbackRows = [
      WordStatisticsRowDefinition('pages', '页数'),
      WordStatisticsRowDefinition('words', '字数'),
      WordStatisticsRowDefinition('asianChars', '亚洲字符（中日韩）'),
      WordStatisticsRowDefinition('nonAsianWords', '非亚洲语言单词'),
      WordStatisticsRowDefinition('charsNoSpace', '字符数(不计空格)'),
      WordStatisticsRowDefinition('charsWithSpace', '字符数(计空格)'),
      WordStatisticsRowDefinition('paragraphs', '段落数'),
      WordStatisticsRowDefinition('lines', '行数'),
    ];
    WordRibbonItem action(Object? value) {
      final item = _map(value);
      return WordRibbonItem(
          id: item['id'] as String,
          label: item['label'] as String,
          kind: item['kind'] as String? ?? 'icon',
          icon: item['icon'] as String,
          command: item['command'] as String?);
    }

    return WordChromeDefinition(
      tabRowHeight: (metrics['tabRowHeight'] as num).toDouble(),
      ribbonHeight: (metrics['ribbonHeight'] as num).toDouble(),
      compactRibbonHeight: (metrics['compactRibbonHeight'] as num).toDouble(),
      statusHeight: (metrics['statusHeight'] as num).toDouble(),
      desktopMinWidth:
          (responsive['desktopMinWidth'] as num?)?.toDouble() ?? 1100,
      tabletMinWidth: (responsive['tabletMinWidth'] as num?)?.toDouble() ?? 700,
      foldableMinWidth:
          (responsive['foldableMinWidth'] as num?)?.toDouble() ?? 480,
      tabletRibbonHeight:
          (responsive['tabletRibbonHeight'] as num?)?.toDouble() ?? 64,
      foldableRibbonHeight:
          (responsive['foldableRibbonHeight'] as num?)?.toDouble() ?? 52,
      mobileTabRowHeight:
          (responsive['mobileTabRowHeight'] as num?)?.toDouble() ?? 44,
      mobileStatusHeight:
          (responsive['mobileStatusHeight'] as num?)?.toDouble() ?? 32,
      pageTopInset: (metrics['pageTopInset'] as num).toDouble(),
      pageBottomInset: (metrics['pageBottomInset'] as num).toDouble(),
      pageGap: (metrics['pageGap'] as num).toDouble(),
      canvas: colorFromHex(colors['canvas'] as String),
      chrome: colorFromHex(colors['chrome'] as String),
      status: colorFromHex(colors['status'] as String),
      border: colorFromHex(colors['border'] as String),
      accent: colorFromHex(colors['accent'] as String),
      text: colorFromHex(colors['text'] as String),
      muted: colorFromHex(colors['muted'] as String),
      quickAccess:
          (json['quickAccess'] as List).map(action).toList(growable: false),
      viewModes:
          (status['viewModes'] as List).map(action).toList(growable: false),
      pageLabel: status['page'] as String,
      wordCountLabel: status['wordCount'] as String,
      selectedWordCountLabel: status['selectedWordCount'] as String? ??
          '已选 {selected} / {total} 个字',
      statisticsTitle: statistics['title'] as String? ?? '字数统计',
      statisticsCloseLabel: statistics['close'] as String? ?? '关闭',
      statisticsRows: statistics['rows'] is List
          ? (statistics['rows'] as List).map((value) {
              final row = _map(value);
              return WordStatisticsRowDefinition(
                  row['id'] as String, row['label'] as String);
            }).toList(growable: false)
          : fallbackRows,
      goTo: WordGoToDefinition.fromJson(goTo),
      readExitLabel: status['readExit'] as String? ?? '退出阅读视图 (Esc)',
      revisionLabel: status['revision'] as String,
      readyLabel: status['ready'] as String,
      zoomMinPercent: zoom['minPercent'] as int? ?? 10,
      zoomMaxPercent: zoom['maxPercent'] as int? ?? 500,
      zoomStepPercent: zoom['stepPercent'] as int? ?? 10,
      zoomPresets: (zoom['presets'] as List? ?? const [200, 100, 75])
          .cast<int>()
          .toList(growable: false),
    );
  }
}
