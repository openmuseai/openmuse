/// Immutable, signed spreadsheet chrome. It contains no workbook data.
final class SpreadsheetChromeDefinition {
  SpreadsheetChromeDefinition._({
    required this.tabs,
    required this.quickAccess,
    required this.sheetTabMenu,
    required this.menus,
    required this.colors,
    required this.metrics,
    required this.initialTab,
    required this.statusStatistics,
    required this.statusViews,
    required this.statusMessages,
    required this.zoomDialog,
    required this.goToDialog,
    required this.sheetTabColorPalette,
    required this.visualActions,
    required this.showRibbonGroupCaptions,
  });

  final List<SpreadsheetTab> tabs;
  final List<SpreadsheetControl> quickAccess;
  final String sheetTabMenu;
  final Map<String, List<SpreadsheetMenuNode>> menus;
  final Map<String, String> colors;
  final Map<String, double> metrics;
  final String initialTab;
  final List<SpreadsheetStatusStatistic> statusStatistics;
  final List<SpreadsheetStatusView> statusViews;
  final SpreadsheetStatusMessages statusMessages;
  final SpreadsheetZoomDialogDefinition zoomDialog;
  final SpreadsheetGoToDialogDefinition? goToDialog;
  final SpreadsheetColorPalette? sheetTabColorPalette;
  final SpreadsheetVisualActions? visualActions;
  final bool showRibbonGroupCaptions;

  factory SpreadsheetChromeDefinition.fromJson(
    Map<String, dynamic> json, {
    Map<String, dynamic> iconData = const {},
  }) {
    if (json['profile'] != 'openmuse-spreadsheet-chrome/1') {
      throw const FormatException('Unsupported spreadsheet chrome profile');
    }
    final tabs = (json['tabs'] as List)
        .map((x) => SpreadsheetTab.fromJson(_map(x)))
        .toList(growable: false);
    final rawMenus = _map(json['menus']);
    return SpreadsheetChromeDefinition._(
      tabs: tabs,
      sheetTabMenu: json['sheetTabMenu'] as String,
      quickAccess: (json['quickAccess'] as List)
          .map((x) => SpreadsheetControl.fromJson(_map(x)))
          .toList(growable: false),
      menus: rawMenus.map(
        (key, value) => MapEntry(
          key,
          (value as List)
              .map((x) => SpreadsheetMenuNode.fromJson(_map(x)))
              .toList(growable: false),
        ),
      ),
      colors: _map(
        json['colors'],
      ).map((key, value) => MapEntry(key, value as String)),
      metrics: _map(
        json['metrics'],
      ).map((key, value) => MapEntry(key, (value as num).toDouble())),
      initialTab: json['initialTab'] as String,
      statusStatistics: (json['statusStatistics'] as List? ?? const [])
          .map((x) => SpreadsheetStatusStatistic.fromJson(_map(x)))
          .toList(growable: false),
      statusViews: json['statusViews'] is List
          ? (json['statusViews'] as List)
                .map((x) => SpreadsheetStatusView.fromJson(_map(x)))
                .toList(growable: false)
          : const [
              SpreadsheetStatusView(
                'normal',
                '普通视图',
                'grid_view',
                'view.normal',
              ),
              SpreadsheetStatusView(
                'pageBreakPreview',
                '分页预览',
                'view_array',
                'view.pageBreakPreview',
              ),
            ],
      statusMessages: json['statusMessages'] is Map
          ? SpreadsheetStatusMessages.fromJson(_map(json['statusMessages']))
          : SpreadsheetStatusMessages.defaults,
      zoomDialog: json['zoomDialog'] is Map
          ? SpreadsheetZoomDialogDefinition.fromJson(_map(json['zoomDialog']))
          : SpreadsheetZoomDialogDefinition.defaults,
      goToDialog: json['goToDialog'] is Map
          ? SpreadsheetGoToDialogDefinition.fromJson(_map(json['goToDialog']))
          : null,
      sheetTabColorPalette: json['sheetTabColorPalette'] is Map
          ? SpreadsheetColorPalette.fromJson(_map(json['sheetTabColorPalette']))
          : null,
      visualActions: json['visualActions'] is Map
          ? SpreadsheetVisualActions.fromJson(_map(json['visualActions']))
          : null,
      showRibbonGroupCaptions: json['showRibbonGroupCaptions'] as bool? ?? true,
    );
  }

  double metric(String key, [double fallback = 0]) => metrics[key] ?? fallback;
}

/// Product copy for workbook state; the engine supplies the state itself.
final class SpreadsheetStatusMessages {
  const SpreadsheetStatusMessages({
    required this.ready,
    required this.modified,
    required this.limitedObjects,
    required this.limitedRows,
  });

  final String ready;
  final String modified;
  final String limitedObjects;
  final String limitedRows;

  static const defaults = SpreadsheetStatusMessages(
    ready: 'Ready',
    modified: '已修改',
    limitedObjects: '对象预览受限',
    limitedRows: '行尺寸预览受限',
  );

  factory SpreadsheetStatusMessages.fromJson(Map<String, dynamic> json) =>
      SpreadsheetStatusMessages(
        ready: json['ready'] as String,
        modified: json['modified'] as String,
        limitedObjects: json['limitedObjects'] as String,
        limitedRows: json['limitedRows'] as String,
      );
}

/// Product-owned labels and command for the formula bar's Go To control.
final class SpreadsheetGoToDialogDefinition {
  const SpreadsheetGoToDialogDefinition({
    required this.title,
    required this.reference,
    required this.placeholder,
    required this.hint,
    required this.invalidReference,
    required this.cancel,
    required this.confirm,
    required this.command,
  });

  final String title;
  final String reference;
  final String placeholder;
  final String hint;
  final String invalidReference;
  final String cancel;
  final String confirm;
  final String command;

  factory SpreadsheetGoToDialogDefinition.fromJson(Map<String, dynamic> json) =>
      SpreadsheetGoToDialogDefinition(
        title: json['title'] as String,
        reference: json['reference'] as String,
        placeholder: json['placeholder'] as String,
        hint: json['hint'] as String,
        invalidReference: json['invalidReference'] as String,
        cancel: json['cancel'] as String,
        confirm: json['confirm'] as String,
        command: json['command'] as String,
      );
}

/// Product-declared engine commands for built-in object gestures.
final class SpreadsheetVisualActions {
  const SpreadsheetVisualActions(this.move, this.resize, this.delete);
  final String move;
  final String resize;
  final String delete;

  factory SpreadsheetVisualActions.fromJson(Map<String, dynamic> json) =>
      SpreadsheetVisualActions(
        json['move'] as String,
        json['resize'] as String,
        json['delete'] as String,
      );
}

/// Product-owned swatches; the Host only arranges and dispatches them.
final class SpreadsheetColorPalette {
  const SpreadsheetColorPalette({
    required this.noColor,
    required this.themeTitle,
    required this.standardTitle,
    required this.moreColors,
    required this.themeBase,
    required this.themeShades,
    required this.standard,
  });

  final String noColor;
  final String themeTitle;
  final String standardTitle;
  final String moreColors;
  final List<String> themeBase;
  final List<List<String>> themeShades;
  final List<String> standard;

  factory SpreadsheetColorPalette.fromJson(Map<String, dynamic> json) =>
      SpreadsheetColorPalette(
        noColor: json['noColor'] as String,
        themeTitle: json['themeTitle'] as String,
        standardTitle: json['standardTitle'] as String,
        moreColors: json['moreColors'] as String,
        themeBase: (json['themeBase'] as List).cast<String>(),
        themeShades: (json['themeShades'] as List)
            .map((row) => (row as List).cast<String>())
            .toList(growable: false),
        standard: (json['standard'] as List).cast<String>(),
      );
}

/// Product-defined status aggregate. The engine supplies values; the product
/// chooses labels, order and initial visibility.
final class SpreadsheetStatusStatistic {
  const SpreadsheetStatusStatistic(this.id, this.label, this.defaultEnabled);
  final String id;
  final String label;
  final bool defaultEnabled;

  factory SpreadsheetStatusStatistic.fromJson(Map<String, dynamic> json) =>
      SpreadsheetStatusStatistic(
        json['id'] as String,
        json['label'] as String,
        json['defaultEnabled'] as bool,
      );
}

/// Product-owned status-bar view option; the surface renders the selected
/// state and dispatches its command without knowing workbook view names.
final class SpreadsheetStatusView {
  const SpreadsheetStatusView(this.value, this.label, this.icon, this.command);
  final String value;
  final String label;
  final String icon;
  final String command;

  factory SpreadsheetStatusView.fromJson(Map<String, dynamic> json) =>
      SpreadsheetStatusView(
        json['value'] as String,
        json['label'] as String,
        json['icon'] as String,
        json['command'] as String,
      );
}

/// Product-owned zoom choices and command bindings; the dialog owns only
/// temporary user input and returns a validated choice to the surface.
final class SpreadsheetZoomDialogDefinition {
  const SpreadsheetZoomDialogDefinition({
    required this.title,
    required this.magnification,
    required this.fitSelection,
    required this.custom,
    required this.cancel,
    required this.confirm,
    required this.rangeError,
    required this.presets,
    required this.min,
    required this.max,
    required this.zoomCommand,
    required this.fitSelectionCommand,
  });

  final String title;
  final String magnification;
  final String fitSelection;
  final String custom;
  final String cancel;
  final String confirm;
  final String rangeError;
  final List<int> presets;
  final int min;
  final int max;
  final String zoomCommand;
  final String fitSelectionCommand;

  static const defaults = SpreadsheetZoomDialogDefinition(
    title: '缩放',
    magnification: '缩放比例',
    fitSelection: '恰好容纳选定区域',
    custom: '自定义',
    cancel: '取消',
    confirm: '确定',
    rangeError: '请输入 10 到 400 之间的数字。',
    presets: [200, 100, 75, 50, 25],
    min: 10,
    max: 400,
    zoomCommand: 'view.zoom',
    fitSelectionCommand: 'view.zoomToSelection',
  );

  factory SpreadsheetZoomDialogDefinition.fromJson(Map<String, dynamic> json) =>
      SpreadsheetZoomDialogDefinition(
        title: json['title'] as String,
        magnification: json['magnification'] as String,
        fitSelection: json['fitSelection'] as String,
        custom: json['custom'] as String,
        cancel: json['cancel'] as String,
        confirm: json['confirm'] as String,
        rangeError: json['rangeError'] as String,
        presets: (json['presets'] as List)
            .map((value) => (value as num).toInt())
            .toList(growable: false),
        min: (json['min'] as num).toInt(),
        max: (json['max'] as num).toInt(),
        zoomCommand: json['zoomCommand'] as String,
        fitSelectionCommand: json['fitSelectionCommand'] as String,
      );
}

final class SpreadsheetTab {
  const SpreadsheetTab(this.id, this.label, this.groups, this.when);
  final String id;
  final String label;
  final List<SpreadsheetGroup> groups;
  final String? when;

  factory SpreadsheetTab.fromJson(Map<String, dynamic> json) => SpreadsheetTab(
    json['id'] as String,
    json['label'] as String,
    (json['groups'] as List)
        .map((x) => SpreadsheetGroup.fromJson(_map(x)))
        .toList(growable: false),
    json['when'] as String?,
  );
}

final class SpreadsheetGroup {
  const SpreadsheetGroup(this.id, this.label, this.items, this.layout);
  final String id;
  final String label;
  final List<SpreadsheetControl> items;
  final List<SpreadsheetGroupLayout> layout;

  factory SpreadsheetGroup.fromJson(Map<String, dynamic> json) =>
      SpreadsheetGroup(
        json['id'] as String,
        json['label'] as String,
        (json['items'] as List)
            .map((x) => SpreadsheetControl.fromJson(_map(x)))
            .toList(growable: false),
        (json['layout'] as List? ?? const [])
            .map((x) => SpreadsheetGroupLayout.fromJson(_map(x)))
            .toList(growable: false),
      );
}

/// Ordered product-owned composition of controls within one ribbon group.
final class SpreadsheetGroupLayout {
  const SpreadsheetGroupLayout(this.type, this.items, this.rows);
  final String type;
  final List<String> items;
  final List<List<String>> rows;

  factory SpreadsheetGroupLayout.fromJson(Map<String, dynamic> json) =>
      SpreadsheetGroupLayout(
        json['type'] as String,
        (json['items'] as List? ?? const []).cast<String>(),
        (json['rows'] as List? ?? const [])
            .map((row) => (row as List).cast<String>())
            .toList(growable: false),
      );
}

final class SpreadsheetControl {
  const SpreadsheetControl(
    this.id,
    this.label,
    this.icon,
    this.command,
    this.menu,
    this.params,
  );
  final String id;
  final String label;
  final String icon;
  final String? command;
  final String? menu;
  final Map<String, dynamic> params;

  factory SpreadsheetControl.fromJson(Map<String, dynamic> json) =>
      SpreadsheetControl(
        json['id'] as String,
        json['label'] as String,
        json['icon'] as String,
        json['command'] as String?,
        json['menu'] as String?,
        json['params'] is Map ? _map(json['params']) : const {},
      );
}

final class SpreadsheetMenuNode {
  const SpreadsheetMenuNode(
    this.label,
    this.command,
    this.params,
    this.children,
    this.presentation,
  );
  final String label;
  final String? command;
  final Map<String, dynamic> params;
  final List<SpreadsheetMenuNode> children;
  final String? presentation;

  factory SpreadsheetMenuNode.fromJson(Map<String, dynamic> json) =>
      SpreadsheetMenuNode(
        json['label'] as String,
        json['command'] as String?,
        json['params'] is Map ? _map(json['params']) : const {},
        json['children'] is List
            ? (json['children'] as List)
                  .map((x) => SpreadsheetMenuNode.fromJson(_map(x)))
                  .toList(growable: false)
            : const [],
        json['presentation'] as String?,
      );
}

Map<String, dynamic> _map(Object? value) =>
    Map<String, dynamic>.from(value as Map);
