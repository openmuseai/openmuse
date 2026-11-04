import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';

import 'gridcraft_engine.dart';
import 'ss_core_engine.dart';
import 'go_to_dialog.dart';
import 'go_to_reference.dart';
import 'product_definition.dart';
import 'sheet_axis_geometry.dart';
import 'visual_geometry.dart';
import 'workbook_visual_layer.dart';
import 'zoom_dialog.dart';

/// Product chrome and the grid are two views of one Host-owned workbook session.
/// The signed package controls navigation; the engine owns every cell mutation.
final class SpreadsheetProductShell extends StatefulWidget {
  const SpreadsheetProductShell({
    super.key,
    required this.chrome,
    required this.engine,
    this.selection = const SpreadsheetEngineSelection.gridcraft(),
    this.initialBytes,
    this.initialName = 'Book1.xlsx',
    this.onSave,
  });

  final SpreadsheetChromeDefinition chrome;
  final SpreadsheetEngine engine;
  final SpreadsheetEngineSelection selection;
  final Uint8List? initialBytes;
  final String initialName;
  final Future<void> Function(Uint8List bytes, String suggestedName)? onSave;

  @override
  State<SpreadsheetProductShell> createState() =>
      _SpreadsheetProductShellState();
}

final class _SpreadsheetProductShellState
    extends State<SpreadsheetProductShell> {
  static const _sheetRows = 1048576;
  static const _sheetCols = 16384;
  double get _zoomFactor => _zoom / 100;
  double get _headerWidth => widget.chrome.metric('rowHeaderWidth', 46);
  double get _columnHeaderHeight =>
      widget.chrome.metric('columnHeaderHeight', 20);
  int get _windowRows => _gridY.hasClients
      ? (_rows.visibleCount(
                  _rows.indexAt(_gridY.offset),
                  _gridY.position.viewportDimension,
                ) +
                8)
            .clamp(120, 512)
      : 120;
  int get _windowCols => _gridX.hasClients
      ? (_cols.visibleCount(
                  _cols.indexAt(_gridX.offset),
                  _gridX.position.viewportDimension,
                ) +
                8)
            .clamp(40, 160)
      : 40;
  static const _directCommands = {
    'edit.undo',
    'edit.redo',
    'edit.cut',
    'edit.copy',
    'edit.paste',
    'home.bold',
    'home.italic',
    'home.underline',
    'home.strikethrough',
    'home.alignLeft',
    'home.alignCenter',
    'home.alignRight',
    'home.alignTop',
    'home.alignMiddle',
    'home.alignBottom',
    'home.wrapText',
    'home.percent',
    'home.comma',
    'data.sortAscending',
    'data.sortDescending',
    'data.filter',
    'formulas.autoSum',
    'insert.chart',
    'insert.shape',
    'insert.textBox',
    'view.zoom100',
    'view.normal',
    'view.pageBreakPreview',
  };

  late String _tab = widget.chrome.initialTab;
  Map<String, dynamic>? _snapshot;
  int? _selectedVisualId;
  String? _selectedVisualKind;
  final Map<String, Uint8List> _imageBytes = {};
  Map<String, Map<String, dynamic>> _cells = {};
  Set<String> _enabled = {};
  String? _error;
  late String _status = widget.chrome.statusMessages.ready;
  bool _aiOpen = false;
  bool _autoSave = false;
  bool _saving = false;
  bool _resaveRequested = false;
  Timer? _autoSaveTimer;
  String? _editingCell;
  final _cellEditor = TextEditingController();
  final _cellEditorFocus = FocusNode();
  late final Set<String> _enabledStats;
  bool _syncing = false;
  int _zoom = 100;
  SheetAxisGeometry _rows = SheetAxisGeometry(
    count: _sheetRows,
    defaultExtent: 20,
  );
  SheetAxisGeometry _cols = SheetAxisGeometry(
    count: _sheetCols,
    defaultExtent: 64,
  );
  int _windowRow = 0;
  int _windowCol = 0;
  int _visibleCol = 0;
  final _formula = TextEditingController();
  final _name = TextEditingController(text: 'A1');
  final _ribbonScroll = ScrollController();
  final _gridX = ScrollController();
  final _gridY = ScrollController();
  final _columnsX = ScrollController();
  final _rowsY = ScrollController();

  @override
  void initState() {
    super.initState();
    _enabledStats = {
      for (final item in widget.chrome.statusStatistics)
        if (item.defaultEnabled) item.id,
    };
    _gridX.addListener(() => _onViewportScroll(_gridX, _columnsX));
    _columnsX.addListener(() => _onViewportScroll(_columnsX, _gridX));
    _gridY.addListener(() => _onViewportScroll(_gridY, _rowsY));
    _rowsY.addListener(() => _onViewportScroll(_rowsY, _gridY));
    WidgetsBinding.instance.addPostFrameCallback((_) => _open());
  }

  void _mirror(ScrollController source, ScrollController target) {
    if (_syncing || !source.hasClients || !target.hasClients) return;
    _syncing = true;
    target.jumpTo(source.offset.clamp(0, target.position.maxScrollExtent));
    _syncing = false;
  }

  void _onViewportScroll(ScrollController source, ScrollController target) {
    if (_syncing) return;
    _mirror(source, target);
    if (_snapshot == null || !_gridX.hasClients || !_gridY.hasClients) return;
    final firstRow = _rows.indexAt(_gridY.offset);
    final firstCol = _cols.indexAt(_gridX.offset);
    final visibleRows = _rows.visibleCount(
      firstRow,
      _gridY.position.viewportDimension,
    );
    final visibleCols = _cols.visibleCount(
      firstCol,
      _gridX.position.viewportDimension,
    );
    var fetch = false;
    if (firstRow < _windowRow ||
        firstRow + visibleRows + 2 >= _windowRow + _windowRows) {
      _windowRow = firstRow;
      fetch = true;
    }
    if (firstCol < _windowCol ||
        firstCol + visibleCols + 2 >= _windowCol + _windowCols) {
      _windowCol = firstCol;
      fetch = true;
    }
    if (fetch) {
      _refresh();
    } else if (firstCol != _visibleCol) {
      setState(() => _visibleCol = firstCol);
    }
  }

  void _open() {
    if (widget.selection.kind == SpreadsheetEngineKind.ssCore &&
        widget.engine is! SsCoreEngine) {
      if (mounted) {
        setState(() {
          _error =
              'ss-core engine is not injected; refusing to read through another engine';
        });
      }
      return;
    }
    try {
      _imageBytes.clear();
      if (widget.initialBytes == null) {
        widget.engine.newWorkbook();
      } else {
        widget.engine.open(widget.initialName, widget.initialBytes!);
      }
      _refresh();
    } on Object catch (error) {
      if (mounted) setState(() => _error = '$error');
    }
  }

  void _refresh() {
    final previousZoom = _zoom;
    final snap = widget.engine.snapshot(
      row: _windowRow,
      col: _windowCol,
      rows: _windowRows,
      cols: _windowCols,
    );
    final commands = widget.engine.commands();
    final nextZoom = (snap['zoom'] as num?)?.toInt() ?? 100;
    final geometry = snap['sheetGeometry'] as Map?;
    final nextRows = SheetAxisGeometry.fromPairs(
      count: _sheetRows,
      defaultExtent: (geometry?['defaultRowHeight'] as num?)?.toDouble() ?? 20,
      zoomFactor: nextZoom / 100,
      pairs: geometry?['rowSizes'],
    );
    final nextCols = SheetAxisGeometry.fromPairs(
      count: _sheetCols,
      defaultExtent: (geometry?['defaultColWidth'] as num?)?.toDouble() ?? 64,
      zoomFactor: nextZoom / 100,
      pairs: geometry?['colSizes'],
    );
    final geometryChanged =
        !_rows.sameLayout(nextRows) || !_cols.sameLayout(nextCols);
    final sheetIndex = (snap['activeSheet'] as num?)?.toInt() ?? 0;
    final visuals = snap['visuals'] as List? ?? const [];
    var unavailableImages = 0;
    for (final visual in visuals) {
      if (visual is! Map || visual['kind'] != 'image') continue;
      if (visual['mediaAvailable'] != true) {
        unavailableImages++;
        continue;
      }
      final id = (visual['id'] as num).toInt();
      final key = '$sheetIndex:$id';
      if (_imageBytes.containsKey(key)) continue;
      try {
        final bytes = widget.engine.imageBytes(id);
        if (bytes != null) {
          _imageBytes[key] = bytes;
        } else {
          unavailableImages++;
        }
      } on Object {
        unavailableImages++;
      }
    }
    if (!mounted) return;
    setState(() {
      _snapshot = snap;
      _rows = nextRows;
      _cols = nextCols;
      if (_gridX.hasClients) {
        _visibleCol = _cols.indexAt(_gridX.offset);
      }
      _cells = {
        for (final raw in (snap['cells'] as List))
          '${(raw as Map)['row']},${raw['col']}': Map<String, dynamic>.from(
            raw,
          ),
      };
      _enabled = {
        for (final raw in commands)
          if (raw['enabled'] == true) raw['id'] as String,
      };
      _name.text = snap['selection'] as String? ?? 'A1';
      _formula.text = snap['selectionInput'] as String? ?? '';
      _zoom = nextZoom;
      final omitted = (snap['visualsOmitted'] as num?)?.toInt() ?? 0;
      final copy = widget.chrome.statusMessages;
      final baseStatus = snap['dirty'] == true ? copy.modified : copy.ready;
      final notices = <String>[
        if (omitted > 0 || unavailableImages > 0)
          '${copy.limitedObjects}：${omitted + unavailableImages}',
        if (geometry?['rowsComplete'] == false) copy.limitedRows,
      ];
      _status = notices.isEmpty
          ? baseStatus
          : '$baseStatus · ${notices.join(' · ')}';
      if (_selectedVisualId != null &&
          !visuals.any(
            (v) =>
                v is Map &&
                v['id'] == _selectedVisualId &&
                v['kind'] == _selectedVisualKind,
          )) {
        _selectedVisualId = null;
        _selectedVisualKind = null;
      }
      final contexts = {
        ...(snap['contexts'] as List? ?? const []).cast<String>(),
        if (_selectedVisualKind == 'chart') 'chart',
        if (_selectedVisualKind == 'shape') 'shape',
        if (_selectedVisualKind == 'image') 'picture',
      };
      if (!widget.chrome.tabs.any(
        (tab) =>
            tab.id == _tab && (tab.when == null || contexts.contains(tab.when)),
      )) {
        _tab = widget.chrome.initialTab;
      }
    });
    if (previousZoom != _zoom || geometryChanged) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!mounted) return;
        if (_gridY.hasClients) {
          _windowRow = _rows.indexAt(_gridY.offset);
        }
        if (_gridX.hasClients) {
          _windowCol = _cols.indexAt(_gridX.offset);
        }
        _refresh();
      });
    }
  }

  bool _canRun(String? command) =>
      command != null &&
      _enabled.contains(command) &&
      _directCommands.contains(command);

  void _run(String command, [Map<String, dynamic> params = const {}]) {
    try {
      widget.engine.execute(command, params);
      _refresh();
      if (_autoSave && _snapshot?['dirty'] == true) _scheduleAutoSave();
    } on Object catch (error) {
      if (mounted) setState(() => _status = '$error');
    }
  }

  void _commitVisual(
    int id,
    String kind,
    Offset delta,
    VisualResizeHandle? handle,
  ) {
    final actions = widget.chrome.visualActions;
    final command = handle == null ? actions?.move : actions?.resize;
    if (command == null || !_enabled.contains(command)) return;
    final visuals = _snapshot?['visuals'] as List? ?? const [];
    for (final raw in visuals) {
      if (raw is! Map || raw['id'] != id || raw['kind'] != kind) continue;
      final params = handle != null
          ? VisualGeometry.resize(
              raw,
              handle: handle,
              deltaX: delta.dx,
              deltaY: delta.dy,
              rows: _rows,
              cols: _cols,
              zoomFactor: _zoomFactor,
            )
          : VisualGeometry.move(
              raw,
              deltaX: delta.dx,
              deltaY: delta.dy,
              rows: _rows,
              cols: _cols,
              zoomFactor: _zoomFactor,
            );
      _run(command, params);
      return;
    }
  }

  void _deleteVisual(int id, String kind) {
    final command = widget.chrome.visualActions?.delete;
    if (command != null && _enabled.contains(command)) {
      _run(command, {'kind': kind, 'id': id});
    }
  }

  void _clearVisualSelection() => setState(() {
    _selectedVisualId = null;
    _selectedVisualKind = null;
    if ({'chartDesign', 'shapeFormat', 'pictureFormat'}.contains(_tab)) {
      _tab = 'home';
    }
  });

  Future<void> _save() async {
    if (widget.onSave == null || _snapshot == null) return;
    if (_saving) {
      _resaveRequested = true;
      return;
    }
    _saving = true;
    try {
      final data = widget.engine.export('xlsx');
      await widget.onSave!(
        data,
        _snapshot!['title'] as String? ?? widget.initialName,
      );
      if (mounted) setState(() => _status = '已保存');
    } on Object catch (error) {
      if (mounted) setState(() => _status = '保存失败：$error');
    } finally {
      _saving = false;
      if (_resaveRequested && _autoSave && mounted) {
        _resaveRequested = false;
        _scheduleAutoSave();
      }
    }
  }

  void _scheduleAutoSave() {
    _autoSaveTimer?.cancel();
    _autoSaveTimer = Timer(const Duration(milliseconds: 850), () {
      if (mounted && _autoSave) _save();
    });
  }

  void _beginEdit(String cell, String initial) {
    if (_editingCell != null && _editingCell != cell) _finishEdit();
    _cellEditor.text = initial;
    _cellEditor.selection = TextSelection.collapsed(offset: initial.length);
    setState(() => _editingCell = cell);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted && _editingCell == cell) _cellEditorFocus.requestFocus();
    });
  }

  void _finishEdit({bool commit = true}) {
    final cell = _editingCell;
    if (cell == null) return;
    final input = _cellEditor.text;
    setState(() => _editingCell = null);
    if (commit) _run('cell.set', {'cell': cell, 'input': input});
  }

  void _select(String cell) {
    final reference = GoToReference.parse(cell);
    setState(() {
      _selectedVisualId = null;
      _selectedVisualKind = null;
    });
    _run(widget.chrome.goToDialog?.command ?? 'selection.set', {
      'range': reference?.range ?? cell,
    });
    if (reference != null) _revealReference(reference);
  }

  void _revealReference(GoToReference reference) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      if (_gridX.hasClients) {
        final leading = _cols.leading(reference.col);
        final trailing = _cols.leading(reference.col + 1);
        final offset = _gridX.offset;
        final viewport = _gridX.position.viewportDimension;
        if (leading < offset || trailing > offset + viewport) {
          // GenOffice brings a distant column in at the visible right edge.
          final target = leading < offset ? leading : trailing - viewport;
          _gridX.jumpTo(target.clamp(0, _gridX.position.maxScrollExtent));
        }
      }
      if (_gridY.hasClients) {
        final leading = _rows.leading(reference.row);
        final trailing = _rows.leading(reference.row + 1);
        final offset = _gridY.offset;
        final viewport = _gridY.position.viewportDimension;
        if (leading < offset || trailing > offset + viewport) {
          // Leave one preceding row visible, as in GenOffice's Go To view.
          final target = _rows.leading(
            reference.row == 0 ? 0 : reference.row - 1,
          );
          _gridY.jumpTo(target.clamp(0, _gridY.position.maxScrollExtent));
        }
      }
    });
  }

  Future<void> _openGoTo() async {
    final definition = widget.chrome.goToDialog;
    if (definition == null || !_enabled.contains(definition.command)) return;
    final blur = widget.chrome.metric('dialogBackdropBlur', 4);
    final opacity = widget.chrome
        .metric('dialogBackdropOpacity', .5)
        .clamp(0.0, 1.0);
    final reference = await showGeneralDialog<GoToReference>(
      context: context,
      barrierDismissible: true,
      barrierLabel: MaterialLocalizations.of(context).modalBarrierDismissLabel,
      barrierColor: Colors.transparent,
      transitionDuration: Duration.zero,
      pageBuilder: (dialogContext, _, __) => Stack(
        children: [
          Positioned.fill(
            child: BackdropFilter(
              key: const ValueKey('sheet-go-to-backdrop'),
              filter: ui.ImageFilter.blur(sigmaX: blur, sigmaY: blur),
              child: GestureDetector(
                onTap: () => Navigator.of(dialogContext).pop(),
                child: ColoredBox(
                  color: _color(
                    'dialogBackdrop',
                    Colors.black,
                  ).withValues(alpha: opacity),
                ),
              ),
            ),
          ),
          Center(
            child: SpreadsheetGoToDialog(
              definition: definition,
              surface: _color('surface', const Color(0xff242424)),
              ink: _color('text', Colors.white),
              border: _color('border', const Color(0xff414141)),
              focus: _color('dialogFocus', const Color(0xff8ab4f8)),
              primary: _color('dialogPrimary', const Color(0xffe4e4e4)),
              primaryText: _color('dialogPrimaryText', const Color(0xff1e1e1e)),
            ),
          ),
        ],
      ),
    );
    if (reference != null && mounted) _select(reference.range);
  }

  Color _color(String key, Color fallback) {
    final raw = widget.chrome.colors[key];
    if (raw == null) return fallback;
    return Color(int.parse(raw.substring(1), radix: 16) | 0xff000000);
  }

  @override
  Widget build(BuildContext context) {
    final surface = _color('surface', const Color(0xff242424));
    final ink = _color('text', Colors.white);
    final accent = _color('accent', const Color(0xff34a853));
    return LayoutBuilder(
      builder: (context, constraints) {
        final compact = constraints.maxWidth < 1100;
        final phone = constraints.maxWidth < 600;
        return ColoredBox(
          color: surface,
          child: DefaultTextStyle(
            style: TextStyle(
              color: ink,
              fontSize: 12,
              fontFamily: Theme.of(context).textTheme.bodyMedium?.fontFamily,
            ),
            child: Column(
              children: [
                _tabs(ink, accent, phone),
                _ribbon(ink, accent, compact, phone),
                Expanded(
                  child: Row(
                    children: [
                      _aiRail(ink, accent),
                      if (_aiOpen && !phone)
                        SizedBox(width: 230, child: _aiPanel(ink)),
                      Expanded(
                        child: Column(
                          children: [
                            _formulaBar(ink, phone),
                            Expanded(
                              child: _error != null
                                  ? Center(child: Text('工作簿打开失败：$_error'))
                                  : _snapshot == null
                                  ? const Center(
                                      child: CircularProgressIndicator(),
                                    )
                                  : _grid(ink, accent),
                            ),
                            _sheetTabs(ink, accent),
                            _statusBar(ink, accent, phone),
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
      },
    );
  }

  Widget _tabs(Color ink, Color accent, bool phone) {
    final contexts = {
      ...(_snapshot?['contexts'] as List? ?? const []).cast<String>(),
      if (_selectedVisualKind == 'chart') 'chart',
      if (_selectedVisualKind == 'shape') 'shape',
      if (_selectedVisualKind == 'image') 'picture',
    };
    final tabs = widget.chrome.tabs
        .where((tab) => tab.when == null || contexts.contains(tab.when))
        .toList();
    return SizedBox(
      height: widget.chrome.metric('tabHeight'),
      child: Row(
        children: [
          if (!phone) ...[
            for (final control in widget.chrome.quickAccess)
              if (control.id == 'autoSave')
                _autoSaveToggle(control, ink, accent)
              else
                _quick(
                  _icon(control.icon),
                  control.id == 'saveAs'
                      ? '${control.label}（等待 Host 文件选择接口）'
                      : control.label,
                  switch (control.command) {
                    'file.saveBytes' when widget.onSave != null => _save,
                    'edit.undo' when _enabled.contains('edit.undo') =>
                      () => _run('edit.undo'),
                    'edit.redo' when _enabled.contains('edit.redo') =>
                      () => _run('edit.redo'),
                    _ => null,
                  },
                  ink,
                ),
            Container(
              height: 18,
              width: 1,
              color: _color('border', Colors.grey),
            ),
          ],
          Expanded(
            child: ListView.builder(
              scrollDirection: Axis.horizontal,
              itemCount: tabs.length,
              itemBuilder: (context, index) {
                final tab = tabs[index];
                final active = tab.id == _tab;
                return InkWell(
                  key: ValueKey('sheet-tab-${tab.id}'),
                  onTap: () => setState(() => _tab = tab.id),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 10),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        Text(
                          tab.label,
                          style: TextStyle(
                            color: active ? accent : ink,
                            fontSize: 13,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const SizedBox(height: 5),
                        Container(
                          width: active ? tab.label.length * 13.0 : 1,
                          height: 2.5,
                          color: active ? accent : Colors.transparent,
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
          if (!phone)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: Tooltip(
                message: _snapshot?['title'] as String? ?? widget.initialName,
                child: Text(
                  _status,
                  key: const ValueKey('sheet-workbook-status-top'),
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    color: ink.withValues(alpha: .55),
                    fontSize: 10,
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _quick(
    IconData icon,
    String tooltip,
    VoidCallback? action,
    Color ink,
  ) => IconButton(
    tooltip: tooltip,
    icon: Icon(icon, size: 17, color: ink),
    onPressed: action,
    padding: EdgeInsets.zero,
    constraints: const BoxConstraints.tightFor(width: 31, height: 30),
  );

  Widget _autoSaveToggle(SpreadsheetControl control, Color ink, Color accent) =>
      Tooltip(
        message: widget.onSave == null
            ? '${control.label}（当前容器未提供保存接口）'
            : control.label,
        child: InkWell(
          key: const ValueKey('sheet-auto-save'),
          onTap: widget.onSave == null
              ? null
              : () {
                  setState(() => _autoSave = !_autoSave);
                  if (_autoSave && _snapshot?['dirty'] == true) {
                    _scheduleAutoSave();
                  } else if (!_autoSave) {
                    _autoSaveTimer?.cancel();
                  }
                },
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 5),
            child: Row(
              children: [
                Container(
                  width: 26,
                  height: 14,
                  padding: const EdgeInsets.all(2),
                  decoration: BoxDecoration(
                    color: _autoSave ? accent : ink.withValues(alpha: .32),
                    borderRadius: BorderRadius.circular(7),
                  ),
                  alignment: _autoSave
                      ? Alignment.centerRight
                      : Alignment.centerLeft,
                  child: const CircleAvatar(
                    radius: 5,
                    backgroundColor: Colors.white,
                  ),
                ),
                const SizedBox(width: 5),
                Text(
                  control.label,
                  style: TextStyle(
                    color: ink.withValues(alpha: .6),
                    fontSize: 11,
                  ),
                ),
              ],
            ),
          ),
        ),
      );

  Widget _ribbon(Color ink, Color accent, bool compact, bool phone) {
    final tab = widget.chrome.tabs.firstWhere(
      (tab) => tab.id == _tab,
      orElse: () => widget.chrome.tabs.first,
    );
    final height = compact
        ? widget.chrome.metric('compactRibbonHeight')
        : widget.chrome.metric('ribbonHeight');
    final showCaptions =
        widget.chrome.showRibbonGroupCaptions && !compact && !phone;
    return Container(
      height: phone ? 54 : height,
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(color: _color('border', Colors.grey)),
        ),
      ),
      child: Scrollbar(
        controller: _ribbonScroll,
        thumbVisibility: !phone,
        thickness: 4,
        child: ListView(
          key: ValueKey('sheet-ribbon-$_tab'),
          controller: _ribbonScroll,
          scrollDirection: Axis.horizontal,
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
          children: tab.groups
              .map(
                (group) => Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 6,
                    vertical: 2,
                  ),
                  decoration: BoxDecoration(
                    border: Border(
                      right: BorderSide(color: _color('border', Colors.grey)),
                    ),
                  ),
                  child: Column(
                    children: [
                      Expanded(
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: _groupControls(
                            group,
                            ink,
                            accent,
                            compact || phone,
                          ),
                        ),
                      ),
                      if (showCaptions)
                        Text(
                          group.label,
                          maxLines: 1,
                          style: TextStyle(
                            color: ink.withValues(alpha: .65),
                            fontSize: 10,
                            height: 1.1,
                          ),
                        ),
                    ],
                  ),
                ),
              )
              .toList(),
        ),
      ),
    );
  }

  List<Widget> _groupControls(
    SpreadsheetGroup group,
    Color ink,
    Color accent,
    bool compact,
  ) {
    if (compact || group.layout.isEmpty) {
      return [
        for (final item in group.items)
          _control(item, ink, accent, compact ? 'compact' : 'large'),
      ];
    }
    final byId = {for (final item in group.items) item.id: item};
    return [
      for (final block in group.layout)
        if (block.type == 'grid')
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 3),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                for (final row in block.rows)
                  SizedBox(
                    height: 20,
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        for (final id in row)
                          _control(byId[id]!, ink, accent, 'grid'),
                      ],
                    ),
                  ),
              ],
            ),
          )
        else if (block.type == 'rows')
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 3),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                for (final row in block.rows)
                  SizedBox(
                    height: 31,
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        for (final id in row)
                          _control(
                            byId[id]!,
                            ink,
                            accent,
                            id == 'fontName' ||
                                    id == 'fontSize' ||
                                    id == 'numberFormat'
                                ? 'field'
                                : 'icon',
                          ),
                      ],
                    ),
                  ),
              ],
            ),
          )
        else if (block.type == 'stack')
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              for (final id in block.items)
                _control(byId[id]!, ink, accent, 'stack'),
            ],
          )
        else
          for (final id in block.items)
            _control(byId[id]!, ink, accent, 'large'),
    ];
  }

  Widget _control(
    SpreadsheetControl control,
    Color ink,
    Color accent,
    String presentation,
  ) {
    final menu = control.menu == null
        ? null
        : widget.chrome.menus[control.menu];
    final enabled = _canRun(control.command);
    final large = presentation == 'large';
    final field = presentation == 'field';
    final stack = presentation == 'stack';
    final compact = presentation == 'compact';
    final grid = presentation == 'grid';
    final usable = enabled || menu != null;
    final button = Tooltip(
      message: enabled || menu != null
          ? control.label
          : '${control.label}（当前内核尚未接入）',
      child: InkWell(
        onTap: enabled ? () => _run(control.command!, control.params) : null,
        borderRadius: BorderRadius.circular(4),
        child: Container(
          width: field
              ? (control.id == 'fontName'
                    ? widget.chrome.metric('ribbonFontNameWidth', 150)
                    : control.id == 'fontSize'
                    ? widget.chrome.metric('ribbonFontSizeWidth', 57)
                    : widget.chrome.metric('ribbonNumberFormatWidth', 112))
              : large
              ? (control.label.length > 5
                    ? widget.chrome.metric('ribbonLargeWideWidth', 72)
                    : widget.chrome.metric('ribbonLargeWidth', 58))
              : stack
              ? 94
              : grid
              ? 32
              : compact
              ? 41
              : 32,
          height: large
              ? 60
              : stack
              ? 20
              : grid
              ? 20
              : compact
              ? 38
              : 29,
          padding: field
              ? const EdgeInsets.symmetric(horizontal: 6)
              : EdgeInsets.zero,
          decoration: field
              ? BoxDecoration(
                  border: Border.all(color: _color('border', Colors.grey)),
                  borderRadius: BorderRadius.circular(4),
                )
              : null,
          child: large
              ? Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      _icon(control.icon),
                      size: 27,
                      color: usable ? ink : ink.withValues(alpha: .38),
                    ),
                    const SizedBox(height: 5),
                    Text(
                      control.label,
                      maxLines: 1,
                      style: TextStyle(
                        fontSize: 11,
                        color: usable ? ink : ink.withValues(alpha: .38),
                      ),
                    ),
                  ],
                )
              : Row(
                  mainAxisAlignment: stack || field
                      ? MainAxisAlignment.start
                      : MainAxisAlignment.center,
                  children: [
                    if (!field)
                      Icon(
                        _icon(control.icon),
                        size: stack ? 14 : 18,
                        color: usable ? ink : ink.withValues(alpha: .38),
                      ),
                    if (stack || field) ...[
                      if (stack) const SizedBox(width: 5),
                      Expanded(
                        child: Text(
                          field
                              ? control.id == 'fontName'
                                    ? 'Aptos'
                                    : control.id == 'fontSize'
                                    ? '11'
                                    : '常规'
                              : control.label,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            fontSize: 11,
                            color: usable ? ink : ink.withValues(alpha: .38),
                          ),
                        ),
                      ),
                    ],
                  ],
                ),
        ),
      ),
    );
    if (menu == null) return button;
    return MenuAnchor(
      style: _popupStyle(),
      menuChildren: menu.map(_menuNode).toList(),
      builder: (context, controller, child) => InkWell(
        onTap: controller.isOpen ? controller.close : controller.open,
        child: Stack(
          children: [
            button,
            Positioned(
              right: 0,
              bottom: large ? 13 : 0,
              child: InkWell(
                key: ValueKey('sheet-control-menu-${control.id}'),
                onTap: controller.isOpen ? controller.close : controller.open,
                child: SizedBox(
                  width: 16,
                  height: 17,
                  child: Icon(
                    Icons.arrow_drop_down,
                    size: 12,
                    color: ink.withValues(alpha: .7),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _menuNode(SpreadsheetMenuNode node) {
    if (node.children.isNotEmpty) {
      return SubmenuButton(
        menuStyle: _popupStyle(),
        style: _menuItemStyle(),
        menuChildren: node.children.map(_menuNode).toList(),
        child: Text(node.label),
      );
    }
    return MenuItemButton(
      style: _menuItemStyle(),
      onPressed:
          widget.chrome.goToDialog != null &&
              node.command == widget.chrome.goToDialog!.command &&
              node.params.isEmpty
          ? (_enabled.contains(node.command) ? _openGoTo : null)
          : _canRun(node.command)
          ? () => _run(node.command!, node.params)
          : null,
      child: Text(node.label),
    );
  }

  MenuStyle _popupStyle() => MenuStyle(
    backgroundColor: WidgetStatePropertyAll(
      _color('chrome', const Color(0xff303030)),
    ),
    surfaceTintColor: const WidgetStatePropertyAll(Colors.transparent),
    side: WidgetStatePropertyAll(
      BorderSide(color: _color('border', const Color(0xff494949))),
    ),
    shape: WidgetStatePropertyAll(
      RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
    ),
    elevation: const WidgetStatePropertyAll(7),
  );

  ButtonStyle _menuItemStyle() => ButtonStyle(
    foregroundColor: WidgetStateProperty.resolveWith(
      (states) => states.contains(WidgetState.disabled)
          ? _color('muted', const Color(0xff999999))
          : _color('text', Colors.white),
    ),
  );

  Widget _aiRail(Color ink, Color accent) => Container(
    width: widget.chrome.metric('aiRailWidth'),
    color: _color('chrome', const Color(0xff202020)),
    alignment: Alignment.topCenter,
    child: IconButton(
      tooltip: 'Muse AI',
      icon: Icon(Icons.auto_awesome, size: 20, color: accent),
      padding: const EdgeInsets.only(top: 8),
      onPressed: () => setState(() => _aiOpen = !_aiOpen),
    ),
  );

  Widget _aiPanel(Color ink) => Container(
    decoration: BoxDecoration(
      border: Border(right: BorderSide(color: _color('border', Colors.grey))),
    ),
    padding: const EdgeInsets.all(14),
    child: Text('Muse AI\n\n此扩展当前未安装。', style: TextStyle(color: ink)),
  );

  Widget _formulaBar(Color ink, bool phone) => Container(
    height: widget.chrome.metric('formulaHeight'),
    decoration: BoxDecoration(
      border: Border(bottom: BorderSide(color: _color('border', Colors.grey))),
    ),
    child: Row(
      children: [
        SizedBox(
          width: phone ? 70 : 95,
          child: TextField(
            controller: _name,
            style: TextStyle(color: ink, fontSize: 13),
            decoration: const InputDecoration(
              isDense: true,
              border: InputBorder.none,
              contentPadding: EdgeInsets.symmetric(horizontal: 9, vertical: 9),
            ),
            onSubmitted: _select,
          ),
        ),
        if (widget.chrome.goToDialog case final definition?)
          IconButton(
            key: const ValueKey('sheet-go-to-open'),
            tooltip: definition.title,
            onPressed: _enabled.contains(definition.command) ? _openGoTo : null,
            icon: Icon(Icons.arrow_drop_down, size: 15, color: ink),
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints.tightFor(width: 20, height: 28),
          )
        else
          Icon(Icons.arrow_drop_down, size: 15, color: ink),
        Container(width: 1, color: _color('border', Colors.grey)),
        if (!phone) ...[
          _quick(Icons.close, '取消编辑', () => _refresh(), ink),
          _quick(
            Icons.check,
            '确认输入',
            () =>
                _run('cell.set', {'cell': _name.text, 'input': _formula.text}),
            ink,
          ),
        ],
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 9),
          child: Text(
            'ƒx',
            style: TextStyle(
              color: ink,
              fontSize: 17,
              fontStyle: FontStyle.italic,
            ),
          ),
        ),
        Expanded(
          child: TextField(
            controller: _formula,
            style: TextStyle(color: ink, fontSize: 13),
            decoration: const InputDecoration(
              isDense: true,
              border: InputBorder.none,
              contentPadding: EdgeInsets.symmetric(horizontal: 8, vertical: 9),
            ),
            onSubmitted: (input) =>
                _run('cell.set', {'cell': _name.text, 'input': input}),
          ),
        ),
      ],
    ),
  );

  Widget _grid(Color ink, Color accent) {
    final heading = _color('chrome', const Color(0xff202020));
    final line = _color(
      'border',
      const Color(0xff414141),
    ).withValues(alpha: .47);
    return Column(
      children: [
        SizedBox(
          height: _columnHeaderHeight,
          child: Row(
            children: [
              Container(width: _headerWidth, color: heading),
              Expanded(
                child: ListView.builder(
                  controller: _columnsX,
                  scrollDirection: Axis.horizontal,
                  itemExtentBuilder: (index, _) => _cols.extentAt(index),
                  itemCount: _sheetCols,
                  itemBuilder: (context, c) => Container(
                    height: _columnHeaderHeight,
                    alignment: Alignment.center,
                    decoration: BoxDecoration(
                      color: heading,
                      border: Border(
                        right: BorderSide(color: line),
                        bottom: BorderSide(color: line),
                      ),
                    ),
                    child: Text(
                      _letters(c),
                      style: TextStyle(color: ink, fontSize: 12),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            children: [
              SizedBox(
                width: _headerWidth,
                child: ListView.builder(
                  controller: _rowsY,
                  itemExtentBuilder: (index, _) => _rows.extentAt(index),
                  itemCount: _sheetRows,
                  itemBuilder: (context, r) => Container(
                    width: _headerWidth,
                    alignment: Alignment.centerRight,
                    padding: const EdgeInsets.only(right: 7),
                    decoration: BoxDecoration(
                      color: heading,
                      border: Border(
                        right: BorderSide(color: line),
                        bottom: BorderSide(color: line),
                      ),
                    ),
                    child: Text(
                      '${r + 1}',
                      style: TextStyle(color: ink, fontSize: 12),
                    ),
                  ),
                ),
              ),
              Expanded(
                child: LayoutBuilder(
                  builder: (context, constraints) => Stack(
                    children: [
                      SingleChildScrollView(
                        controller: _gridX,
                        scrollDirection: Axis.horizontal,
                        child: SizedBox(
                          width: _cols.totalExtent,
                          child: ListView.builder(
                            key: const ValueKey('sheet-grid-rows'),
                            controller: _gridY,
                            itemExtentBuilder: (index, _) =>
                                _rows.extentAt(index),
                            itemCount: _sheetRows,
                            itemBuilder: (context, r) => SizedBox(
                              width: _cols.totalExtent,
                              height: _rows.extentAt(r),
                              child: Stack(
                                children: [
                                  for (
                                    var c = _visibleCol;
                                    c < _sheetCols &&
                                        c <
                                            _cols.indexAt(
                                                  (_gridX.hasClients
                                                          ? _gridX.offset
                                                          : 0) +
                                                      constraints.maxWidth,
                                                ) +
                                                4;
                                    c++
                                  )
                                    Positioned(
                                      left: _cols.leading(c),
                                      child: _cell(r, c, ink, accent, line),
                                    ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ),
                      Positioned.fill(
                        child: AnimatedBuilder(
                          animation: Listenable.merge([_gridX, _gridY]),
                          builder: (context, _) => WorkbookVisualLayer(
                            visuals: _snapshot?['visuals'] as List? ?? const [],
                            imageBytes: {
                              for (final entry in _imageBytes.entries)
                                if (entry.key.startsWith(
                                  '${_snapshot?['activeSheet']}:',
                                ))
                                  int.parse(entry.key.split(':').last):
                                      entry.value,
                            },
                            scrollX: _gridX.hasClients ? _gridX.offset : 0,
                            scrollY: _gridY.hasClients ? _gridY.offset : 0,
                            rows: _rows,
                            cols: _cols,
                            zoomFactor: _zoomFactor,
                            selectedId: _selectedVisualId,
                            selectedKind: _selectedVisualKind,
                            editable:
                                widget.chrome.visualActions != null &&
                                _enabled.contains(
                                  widget.chrome.visualActions!.move,
                                ) &&
                                _enabled.contains(
                                  widget.chrome.visualActions!.delete,
                                ),
                            onCommit: _commitVisual,
                            onDelete: _deleteVisual,
                            onClearSelection: _clearVisualSelection,
                            onSelect: (id, kind) => setState(() {
                              _selectedVisualId = id;
                              _selectedVisualKind = kind;
                              _tab = switch (kind) {
                                'shape' => 'shapeFormat',
                                'image' => 'pictureFormat',
                                _ => 'chartDesign',
                              };
                            }),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _cell(int row, int col, Color ink, Color accent, Color line) {
    final data = _cells['$row,$col'];
    final cell = '${_letters(col)}${row + 1}';
    final active = _snapshot?['selection'] == cell;
    final selected = _isSelected(row, col);
    Color? parseColor(Object? raw) => raw is String && raw.startsWith('#')
        ? Color(int.parse(raw.substring(1), radix: 16) | 0xff000000)
        : null;
    final baseColor =
        parseColor(data?['fillColor']) ??
        _color('grid', const Color(0xff232323));
    BorderSide edge(
      int neighborRow,
      int neighborCol, {
      bool trailing = false,
    }) => selected && !_isSelected(neighborRow, neighborCol)
        ? BorderSide(color: accent, width: 1.5)
        : trailing
        ? BorderSide(color: line)
        : BorderSide.none;
    return GestureDetector(
      key: ValueKey('sheet-cell-$cell'),
      onTap: () {
        if (_editingCell != null && _editingCell != cell) _finishEdit();
        _select(cell);
      },
      onDoubleTap: () => _beginEdit(cell, data?['input'] as String? ?? ''),
      child: Container(
        width: _cols.extentAt(col),
        height: _rows.extentAt(row),
        alignment: Alignment.centerLeft,
        padding: EdgeInsets.symmetric(horizontal: 5 * _zoomFactor),
        decoration: BoxDecoration(
          color: selected && !active
              ? Color.alphaBlend(accent.withValues(alpha: .14), baseColor)
              : baseColor,
          border: active
              ? Border.all(color: accent, width: 2)
              : Border(
                  left: edge(row, col - 1),
                  top: edge(row - 1, col),
                  right: edge(row, col + 1, trailing: true),
                  bottom: edge(row + 1, col, trailing: true),
                ),
        ),
        child: _editingCell == cell
            ? TextField(
                key: ValueKey('sheet-cell-editor-$cell'),
                controller: _cellEditor,
                focusNode: _cellEditorFocus,
                style: TextStyle(color: ink, fontSize: 12 * _zoomFactor),
                textInputAction: TextInputAction.done,
                decoration: const InputDecoration(
                  isDense: true,
                  border: InputBorder.none,
                  contentPadding: EdgeInsets.zero,
                ),
                onSubmitted: (_) => _finishEdit(),
                onTapOutside: (_) => _finishEdit(),
              )
            : Text(
                data?['text'] as String? ?? '',
                maxLines: 1,
                overflow: TextOverflow.clip,
                style: TextStyle(
                  color: parseColor(data?['fontColor']) ?? ink,
                  fontSize: 12 * _zoomFactor,
                  fontWeight: data?['bold'] == true
                      ? FontWeight.bold
                      : FontWeight.normal,
                  fontStyle: data?['italic'] == true
                      ? FontStyle.italic
                      : FontStyle.normal,
                ),
              ),
      ),
    );
  }

  bool _isSelected(int row, int col) {
    if (row < 0 || col < 0) return false;
    for (final raw in (_snapshot?['selectionRanges'] as List? ?? const [])) {
      final range = raw as Map;
      if (row >= (range['startRow'] as num) &&
          row <= (range['endRow'] as num) &&
          col >= (range['startCol'] as num) &&
          col <= (range['endCol'] as num)) {
        return true;
      }
    }
    return false;
  }

  Widget _sheetTabs(Color ink, Color accent) => Container(
    height: widget.chrome.metric('sheetTabsHeight'),
    decoration: BoxDecoration(
      border: Border(top: BorderSide(color: _color('border', Colors.grey))),
    ),
    child: Row(
      children: [
        _quick(
          Icons.add,
          '新建工作表',
          _enabled.contains('home.insertSheet')
              ? () => _run('home.insertSheet')
              : null,
          ink,
        ),
        const SizedBox(width: 5),
        Expanded(
          child: ListView(
            scrollDirection: Axis.horizontal,
            children: [
              for (
                var i = 0;
                i < (_snapshot?['sheets'] as List? ?? const []).length;
                i++
              )
                if (_sheetVisible(i)) _sheetTab(i, ink, accent),
            ],
          ),
        ),
        _quick(Icons.grid_on, '工作表导航', null, ink),
      ],
    ),
  );

  bool _sheetVisible(int index) {
    final states = _snapshot?['sheetStates'] as List?;
    return states == null ||
        index >= states.length ||
        (states[index] as Map)['visibility'] == 'Visible';
  }

  Color? _sheetTabColor(int index) {
    final states = _snapshot?['sheetStates'] as List?;
    if (states == null || index >= states.length) return null;
    final hex = (states[index] as Map)['tabColor'];
    if (hex is! String || !RegExp(r'^#[0-9a-fA-F]{6}$').hasMatch(hex)) {
      return null;
    }
    return Color(int.parse(hex.substring(1), radix: 16) | 0xff000000);
  }

  Widget _sheetTab(int index, Color ink, Color accent) {
    final controller = MenuController();
    return MenuAnchor(
      controller: controller,
      style: _popupStyle(),
      menuChildren: widget.chrome.menus[widget.chrome.sheetTabMenu]!
          .map((node) => _sheetMenuNode(node, index, controller))
          .toList(),
      builder: (context, _, child) => GestureDetector(
        onSecondaryTapDown: (_) {
          _run('sheet.activate', {'sheet': index});
          controller.open();
        },
        onLongPressStart: (_) {
          _run('sheet.activate', {'sheet': index});
          controller.open();
        },
        onDoubleTap: () => _renameSheet(index),
        child: InkWell(
          key: ValueKey('sheet-tab-item-$index'),
          onTap: () => _run('sheet.activate', {'sheet': index}),
          child: Container(
            alignment: Alignment.center,
            padding: const EdgeInsets.symmetric(horizontal: 14),
            margin: const EdgeInsets.symmetric(vertical: 4, horizontal: 2),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(4),
              color: index == _snapshot?['activeSheet']
                  ? accent.withValues(alpha: .16)
                  : Colors.transparent,
              border: _sheetTabColor(index) == null
                  ? null
                  : Border(
                      bottom: BorderSide(
                        color: _sheetTabColor(index)!,
                        width: 3,
                      ),
                    ),
            ),
            child: Text(
              '${(_snapshot!['sheets'] as List)[index]}',
              style: TextStyle(
                color: index == _snapshot?['activeSheet'] ? accent : ink,
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _sheetMenuNode(
    SpreadsheetMenuNode node,
    int index,
    MenuController controller,
  ) {
    if (node.presentation == 'colorPalette' &&
        widget.chrome.sheetTabColorPalette != null) {
      return SubmenuButton(
        menuStyle: _popupStyle(),
        style: _menuItemStyle(),
        menuChildren: [_sheetColorPalette(index, controller)],
        child: Text(node.label),
      );
    }
    if (node.children.isNotEmpty) {
      return SubmenuButton(
        menuStyle: _popupStyle(),
        style: _menuItemStyle(),
        menuChildren: node.children
            .map((child) => _sheetMenuNode(child, index, controller))
            .toList(),
        child: Text(node.label),
      );
    }
    final command = node.command;
    final states = _snapshot?['sheetStates'] as List? ?? const [];
    final visible = states
        .where((s) => (s as Map)['visibility'] == 'Visible')
        .length;
    final hidden = states
        .where((s) => (s as Map)['visibility'] != 'Visible')
        .length;
    final supported =
        command != null &&
        _enabled.contains(command) &&
        {
          'home.insertSheet',
          'home.deleteSheet',
          'sheet.rename',
          'sheet.move',
          'sheet.tabColor',
          'sheet.hide',
          'sheet.unhide',
        }.contains(command) &&
        (command != 'sheet.hide' && command != 'home.deleteSheet' ||
            visible > 1) &&
        (command != 'sheet.unhide' || hidden > 0);
    return MenuItemButton(
      style: _menuItemStyle(),
      onPressed: supported ? () => _sheetMenuCommand(node, index) : null,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (command == 'sheet.tabColor' &&
              node.params['color'] is String &&
              RegExp(
                r'^#[0-9a-fA-F]{6}$',
              ).hasMatch(node.params['color'] as String)) ...[
            Container(
              width: 13,
              height: 13,
              decoration: BoxDecoration(
                color: Color(
                  int.parse(
                        (node.params['color'] as String).substring(1),
                        radix: 16,
                      ) |
                      0xff000000,
                ),
                border: Border.all(color: Colors.white38),
              ),
            ),
            const SizedBox(width: 8),
          ],
          Text(node.label),
        ],
      ),
    );
  }

  Widget _sheetColorPalette(int index, MenuController controller) {
    final palette = widget.chrome.sheetTabColorPalette!;
    final ink = _color('text', Colors.white);
    final border = _color('border', const Color(0xff414141));
    final selected =
        _snapshot?['sheetStates'] is List &&
            index < (_snapshot!['sheetStates'] as List).length
        ? (_snapshot!['sheetStates'] as List)[index]['tabColor'] as String?
        : null;
    Widget swatch(String hex, String row, int column) {
      final color = '#$hex';
      return Tooltip(
        message: color,
        child: InkWell(
          key: ValueKey('sheet-color-$row-$column'),
          onTap: () {
            controller.close();
            _run('sheet.tabColor', {'sheet': index, 'color': color});
          },
          child: Semantics(
            button: true,
            selected: selected?.toUpperCase() == color,
            label: '标签颜色 $color',
            child: Container(
              width: 20,
              height: 20,
              decoration: BoxDecoration(
                color: Color(int.parse(hex, radix: 16) | 0xff000000),
                border: Border.all(
                  color: selected?.toUpperCase() == color
                      ? _color('accent', const Color(0xff34a853))
                      : ink.withValues(alpha: .25),
                  width: selected?.toUpperCase() == color ? 2 : 1,
                ),
              ),
            ),
          ),
        ),
      );
    }

    Widget row(List<String> colors, String id) => Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (var i = 0; i < colors.length; i++) ...[
          if (i > 0) const SizedBox(width: 4),
          swatch(colors[i], id, i),
        ],
      ],
    );

    return Container(
      key: const ValueKey('sheet-color-palette'),
      width: 262,
      padding: const EdgeInsets.fromLTRB(12, 10, 12, 8),
      decoration: BoxDecoration(
        color: _color('surface', const Color(0xff242424)),
        border: Border.all(color: border),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: double.infinity,
            height: 28,
            child: OutlinedButton(
              key: const ValueKey('sheet-color-none'),
              onPressed: () {
                controller.close();
                _run('sheet.tabColor', {'sheet': index, 'color': 'none'});
              },
              child: Text(palette.noColor),
            ),
          ),
          const SizedBox(height: 12),
          Text(
            palette.themeTitle,
            style: TextStyle(color: ink, fontSize: 12.5),
          ),
          const SizedBox(height: 6),
          row(palette.themeBase, 'theme'),
          const SizedBox(height: 8),
          for (var i = 0; i < palette.themeShades.length; i++)
            row(palette.themeShades[i], 'shade-$i'),
          const SizedBox(height: 12),
          Text(
            palette.standardTitle,
            style: TextStyle(color: ink, fontSize: 12.5),
          ),
          const SizedBox(height: 6),
          row(palette.standard, 'standard'),
          const SizedBox(height: 12),
          Divider(height: 1, color: border),
          TextButton.icon(
            key: const ValueKey('sheet-color-more'),
            onPressed: () {
              controller.close();
              _chooseSheetColor(index);
            },
            icon: const Icon(Icons.palette_outlined, size: 16),
            label: Text(palette.moreColors),
          ),
        ],
      ),
    );
  }

  void _sheetMenuCommand(SpreadsheetMenuNode node, int index) {
    final command = node.command!;
    switch (command) {
      case 'home.insertSheet':
        _run(command, {'before': index});
        break;
      case 'home.deleteSheet':
        _deleteSheet(index);
        break;
      case 'sheet.rename':
        _renameSheet(index);
        break;
      case 'sheet.move':
        _moveSheet(index);
        break;
      case 'sheet.tabColor':
        final color = node.params['color'] as String;
        if (color == 'custom') {
          _chooseSheetColor(index);
        } else {
          _run(command, {'sheet': index, 'color': color});
        }
        break;
      case 'sheet.hide':
        _run(command, {'sheet': index});
        break;
      case 'sheet.unhide':
        _unhideSheet();
        break;
    }
  }

  Widget _sheetDialog(Widget child) {
    final base = Theme.of(context);
    final surface = _color('surface', const Color(0xff242424));
    final border = _color('border', const Color(0xff414141));
    final ink = _color('text', Colors.white);
    final accent = _color('accent', const Color(0xff34a853));
    return Theme(
      data: base.copyWith(
        colorScheme: ColorScheme.dark(
          primary: accent,
          surface: surface,
          onSurface: ink,
        ),
        textTheme: base.textTheme.apply(bodyColor: ink, displayColor: ink),
        primaryTextTheme: base.primaryTextTheme.apply(
          bodyColor: ink,
          displayColor: ink,
        ),
        dialogTheme: DialogThemeData(
          backgroundColor: surface,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
            side: BorderSide(color: border),
          ),
        ),
      ),
      child: child,
    );
  }

  Future<void> _moveSheet(int index) async {
    final names = (_snapshot?['sheets'] as List? ?? const []).cast<String>();
    if (names.isEmpty) return;
    var before = index + 1 < names.length ? index + 1 : -1;
    var copy = false;
    final choice = await showDialog<({int before, bool copy})>(
      context: context,
      builder: (context) => _sheetDialog(
        StatefulBuilder(
          builder: (context, setDialogState) => AlertDialog(
            title: const Text('移动或复制工作表'),
            content: SizedBox(
              width: 300,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('将工作表移动到以下工作表之前：'),
                  const SizedBox(height: 10),
                  Container(
                    key: const ValueKey('sheet-move-target'),
                    height: (names.length + 1).clamp(4, 10) * 28.0 + 8,
                    decoration: BoxDecoration(
                      border: Border.all(color: _color('border', Colors.grey)),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: ListView(
                      children: [
                        for (var i = 0; i < names.length; i++)
                          _moveTargetRow(
                            i,
                            names[i],
                            before == i,
                            () => setDialogState(() => before = i),
                          ),
                        _moveTargetRow(
                          -1,
                          '移到最后',
                          before == -1,
                          () => setDialogState(() => before = -1),
                        ),
                      ],
                    ),
                  ),
                  CheckboxListTile(
                    key: const ValueKey('sheet-move-copy-check'),
                    contentPadding: EdgeInsets.zero,
                    title: const Text('建立副本'),
                    value: copy,
                    onChanged: (value) =>
                        setDialogState(() => copy = value ?? false),
                  ),
                ],
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('取消'),
              ),
              FilledButton(
                onPressed: () =>
                    Navigator.pop(context, (before: before, copy: copy)),
                child: const Text('确定'),
              ),
            ],
          ),
        ),
      ),
    );
    if (choice == null) return;
    final to = choice.before == -1
        ? (choice.copy ? names.length : names.length - 1)
        : choice.copy
        ? choice.before
        : choice.before > index
        ? choice.before - 1
        : choice.before;
    _run('sheet.move', {'sheet': index, 'to': to, 'copy': choice.copy});
  }

  Widget _moveTargetRow(
    int value,
    String label,
    bool selected,
    VoidCallback onTap,
  ) => InkWell(
    key: ValueKey('sheet-move-target-$value'),
    onTap: onTap,
    child: Container(
      height: 28,
      alignment: Alignment.centerLeft,
      padding: const EdgeInsets.symmetric(horizontal: 8),
      color: selected
          ? _color('accent', const Color(0xff34a853)).withValues(alpha: .2)
          : Colors.transparent,
      child: Text(label, style: const TextStyle(fontSize: 12)),
    ),
  );

  Future<void> _chooseSheetColor(int index) async {
    var hex = '#FF0000';
    final color = await showDialog<String>(
      context: context,
      builder: (context) => _sheetDialog(
        StatefulBuilder(
          builder: (context, setDialogState) => AlertDialog(
            title: const Text('更多工作表标签颜色'),
            content: TextFormField(
              initialValue: hex,
              autofocus: true,
              decoration: const InputDecoration(labelText: '十六进制颜色，例如 #FF0000'),
              onChanged: (value) => setDialogState(() => hex = value),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('取消'),
              ),
              FilledButton(
                onPressed: RegExp(r'^#[0-9a-fA-F]{6}$').hasMatch(hex)
                    ? () => Navigator.pop(context, hex)
                    : null,
                child: const Text('确定'),
              ),
            ],
          ),
        ),
      ),
    );
    if (color != null) _run('sheet.tabColor', {'sheet': index, 'color': color});
  }

  Future<void> _renameSheet(int index) async {
    var editedName = '${(_snapshot!['sheets'] as List)[index]}';
    final name = await showDialog<String>(
      context: context,
      builder: (context) => _sheetDialog(
        AlertDialog(
          title: const Text('重命名工作表'),
          content: TextFormField(
            initialValue: editedName,
            autofocus: true,
            onChanged: (value) => editedName = value,
            onFieldSubmitted: (value) => Navigator.pop(context, value),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('取消'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(context, editedName),
              child: const Text('确定'),
            ),
          ],
        ),
      ),
    );
    if (name != null && name.trim().isNotEmpty)
      _run('sheet.rename', {'sheet': index, 'name': name.trim()});
  }

  Future<void> _deleteSheet(int index) async {
    final name = '${(_snapshot!['sheets'] as List)[index]}';
    final approved = await showDialog<bool>(
      context: context,
      builder: (context) => _sheetDialog(
        AlertDialog(
          title: const Text('删除工作表'),
          content: Text('确定删除“$name”？'),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: const Text('取消'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(context, true),
              child: const Text('删除'),
            ),
          ],
        ),
      ),
    );
    if (approved == true) _run('home.deleteSheet', {'sheet': index});
  }

  Future<void> _unhideSheet() async {
    final states = _snapshot?['sheetStates'] as List? ?? const [];
    final hidden = [
      for (var i = 0; i < states.length; i++)
        if ((states[i] as Map)['visibility'] != 'Visible') i,
    ];
    final index = await showDialog<int>(
      context: context,
      builder: (context) => _sheetDialog(
        SimpleDialog(
          title: const Text('取消隐藏工作表'),
          children: [
            for (final i in hidden)
              SimpleDialogOption(
                onPressed: () => Navigator.pop(context, i),
                child: Text('${(states[i] as Map)['name']}'),
              ),
          ],
        ),
      ),
    );
    if (index != null) _run('sheet.unhide', {'sheet': index});
  }

  Widget _statusBar(Color ink, Color accent, bool phone) => GestureDetector(
    onSecondaryTapDown: (details) =>
        _openStatisticsMenu(details.globalPosition),
    onLongPressStart: (details) => _openStatisticsMenu(details.globalPosition),
    child: Container(
      height: phone ? 32 : widget.chrome.metric('statusHeight'),
      decoration: BoxDecoration(
        color: _color('statusBackground', const Color(0xff202020)),
        border: Border(
          top: BorderSide(
            color: _color('statusBorder', const Color(0xff414141)),
          ),
        ),
      ),
      padding: EdgeInsets.symmetric(
        horizontal: widget.chrome.metric('statusHorizontalPadding', 8),
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              _status,
              key: const ValueKey('sheet-workbook-status-bottom'),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                color: ink.withValues(alpha: .6),
                fontSize: 11.5,
              ),
            ),
          ),
          if (!phone && _statsLabel().isNotEmpty) ...[
            Text(
              _statsLabel(),
              style: TextStyle(
                color: ink.withValues(alpha: .65),
                fontSize: 11.5,
              ),
            ),
            const SizedBox(width: 18),
          ],
          if (!phone) ...[
            for (var i = 0; i < widget.chrome.statusViews.length; i++) ...[
              _statusViewButton(widget.chrome.statusViews[i], ink, accent),
              if (i < widget.chrome.statusViews.length - 1)
                SizedBox(width: widget.chrome.metric('statusViewGap', 2)),
            ],
            const SizedBox(width: 6),
          ],
          SizedBox(
            width: 20,
            height: 20,
            child: InkWell(
              onTap: () => _setZoom(_zoom - 10),
              child: Icon(Icons.remove, size: 14, color: ink),
            ),
          ),
          SizedBox(
            width: phone
                ? 55
                : widget.chrome.metric('statusZoomSliderWidth', 110),
            child: SliderTheme(
              data: SliderTheme.of(context).copyWith(
                trackHeight: 3,
                activeTrackColor: _color('statusZoom', const Color(0xff777777)),
                inactiveTrackColor: _color(
                  'statusZoom',
                  const Color(0xff777777),
                ).withValues(alpha: .65),
                thumbColor: _color('statusZoom', const Color(0xff777777)),
                thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 6),
                overlayShape: SliderComponentShape.noOverlay,
              ),
              child: Slider(
                value: _zoom
                    .clamp(
                      widget.chrome.zoomDialog.min,
                      widget.chrome.zoomDialog.max,
                    )
                    .toDouble(),
                min: widget.chrome.zoomDialog.min.toDouble(),
                max: widget.chrome.zoomDialog.max.toDouble(),
                onChanged: (value) => _setZoom(value.round()),
              ),
            ),
          ),
          SizedBox(
            width: 20,
            height: 20,
            child: InkWell(
              onTap: () => _setZoom(_zoom + 10),
              child: Icon(Icons.add, size: 14, color: ink),
            ),
          ),
          SizedBox(
            width: 38,
            child: Tooltip(
              message: widget.chrome.zoomDialog.title,
              child: InkWell(
                key: const ValueKey('sheet-zoom-value'),
                onTap: _openZoomDialog,
                borderRadius: BorderRadius.circular(3),
                child: Center(
                  child: Text(
                    '$_zoom%',
                    textAlign: TextAlign.right,
                    style: TextStyle(
                      color: ink.withValues(alpha: .75),
                      fontSize: 11.5,
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    ),
  );

  Widget _statusViewButton(
    SpreadsheetStatusView view,
    Color ink,
    Color accent,
  ) {
    final selected = _snapshot?['view'] == view.value;
    return Tooltip(
      message: view.label,
      child: InkWell(
        onTap: _enabled.contains(view.command) && !selected
            ? () => _run(view.command)
            : null,
        borderRadius: BorderRadius.circular(3),
        child: Container(
          width: widget.chrome.metric('statusViewButtonWidth', 22),
          height: 20,
          alignment: Alignment.center,
          decoration: BoxDecoration(
            color: selected
                ? _color(
                    'statusSelectedBackground',
                    accent.withValues(alpha: .16),
                  )
                : Colors.transparent,
            borderRadius: BorderRadius.circular(3),
          ),
          child: Icon(
            _icon(view.icon),
            size: 14,
            color: selected ? accent : ink.withValues(alpha: .6),
          ),
        ),
      ),
    );
  }

  Future<void> _openStatisticsMenu(Offset position) async {
    final size = MediaQuery.sizeOf(context);
    final ink = _color('text', Colors.white);
    await showGeneralDialog<void>(
      context: context,
      barrierDismissible: true,
      barrierLabel: '关闭统计菜单',
      barrierColor: Colors.transparent,
      transitionDuration: Duration.zero,
      pageBuilder: (dialogContext, _, __) => Material(
        type: MaterialType.transparency,
        child: Stack(
          children: [
            Positioned(
              left: position.dx.clamp(8.0, size.width - 176.0),
              bottom: widget.chrome.metric('statusHeight') + 4,
              child: StatefulBuilder(
                builder: (context, refreshMenu) => Material(
                  color: _color('chrome', const Color(0xff202020)),
                  elevation: 8,
                  borderRadius: BorderRadius.circular(4),
                  child: SizedBox(
                    width: 168,
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        for (final item in widget.chrome.statusStatistics)
                          InkWell(
                            onTap: () {
                              setState(() {
                                if (!_enabledStats.add(item.id)) {
                                  _enabledStats.remove(item.id);
                                }
                              });
                              refreshMenu(() {});
                            },
                            child: SizedBox(
                              height: 32,
                              child: Row(
                                children: [
                                  const SizedBox(width: 8),
                                  Icon(
                                    _enabledStats.contains(item.id)
                                        ? Icons.check_box
                                        : Icons.check_box_outline_blank,
                                    size: 17,
                                    color: ink,
                                  ),
                                  const SizedBox(width: 8),
                                  Text(
                                    item.label,
                                    style: TextStyle(color: ink),
                                  ),
                                ],
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _statsLabel() {
    final stats = _snapshot?['selectionStats'];
    if (stats is! Map || (stats['count'] as num? ?? 0) < 2) return '';
    String format(num value) {
      final number = value.toDouble();
      return number == number.truncateToDouble()
          ? number.toInt().toString()
          : number.toStringAsFixed(2);
    }

    final numeric = (stats['numeric'] as num? ?? 0).toInt();
    final values = <String, num?>{
      'AVERAGE': numeric > 0 ? stats['average'] as num? : null,
      'COUNTA': stats['count'] as num?,
      'COUNT': stats['numeric'] as num?,
      'MIN': numeric > 0 ? stats['min'] as num? : null,
      'MAX': numeric > 0 ? stats['max'] as num? : null,
      'SUM': numeric > 0 ? stats['sum'] as num? : null,
    };
    return [
      for (final item in widget.chrome.statusStatistics)
        if (_enabledStats.contains(item.id) && values[item.id] != null)
          '${item.label}: ${format(values[item.id]!)}',
    ].join('   ');
  }

  void _setZoom(int value) {
    final zoom = widget.chrome.zoomDialog;
    _run(zoom.zoomCommand, {'percent': value.clamp(zoom.min, zoom.max)});
  }

  Future<void> _openZoomDialog() async {
    final zoom = widget.chrome.zoomDialog;
    final choice = await showDialog<SpreadsheetZoomChoice>(
      context: context,
      builder: (_) => SpreadsheetZoomDialog(
        definition: zoom,
        currentPercent: _zoom,
        surface: _color('surface', const Color(0xff242424)),
        ink: _color('text', const Color(0xffe8e8e8)),
        border: _color('border', const Color(0xff414141)),
        accent: _color('accent', const Color(0xff34a853)),
        primary: _color('dialogPrimary', const Color(0xffe4e4e4)),
        primaryText: _color('dialogPrimaryText', const Color(0xff1e1e1e)),
      ),
    );
    if (!mounted || choice == null) return;
    if (choice.fitSelection) {
      if (!_enabled.contains(zoom.fitSelectionCommand)) return;
      final viewWidth = _gridX.hasClients
          ? _gridX.position.viewportDimension
          : 1200.0;
      final viewHeight = _gridY.hasClients
          ? _gridY.position.viewportDimension
          : 700.0;
      // The engine and surface now share workbook pixel geometry.
      _run(zoom.fitSelectionCommand, {
        'viewWidth': viewWidth,
        'viewHeight': viewHeight,
      });
    } else if (choice.percent != null && _enabled.contains(zoom.zoomCommand)) {
      _setZoom(choice.percent!);
    }
  }

  String _letters(int col) {
    var n = col + 1;
    var out = '';
    while (n > 0) {
      n--;
      out = String.fromCharCode(65 + n % 26) + out;
      n ~/= 26;
    }
    return out;
  }

  IconData _icon(String name) => switch (name) {
    'save' => Icons.save_outlined,
    'save_as' => Icons.save_as_outlined,
    'undo' => Icons.undo,
    'redo' => Icons.redo,
    'content_paste' || 'content_paste_go' => Icons.content_paste,
    'content_copy' => Icons.content_copy,
    'content_cut' => Icons.content_cut,
    'format_bold' => Icons.format_bold,
    'format_italic' => Icons.format_italic,
    'format_underlined' => Icons.format_underlined,
    'strikethrough_s' => Icons.strikethrough_s,
    'format_align_left' => Icons.format_align_left,
    'format_align_center' => Icons.format_align_center,
    'format_align_right' => Icons.format_align_right,
    'format_color_fill' => Icons.format_color_fill,
    'format_color_text' => Icons.format_color_text,
    'border_all' => Icons.border_all,
    'functions' => Icons.functions,
    'table_chart' => Icons.table_chart,
    'bar_chart' => Icons.bar_chart,
    'pie_chart' => Icons.pie_chart,
    'search' => Icons.search,
    'auto_awesome' => Icons.auto_awesome,
    'filter_alt' => Icons.filter_alt,
    'sort' => Icons.sort,
    'zoom_in' => Icons.zoom_in,
    'grid_on' => Icons.grid_on,
    'comment' || 'add_comment' => Icons.comment_outlined,
    'palette' => Icons.palette_outlined,
    'image' => Icons.image_outlined,
    'link' => Icons.link,
    'lock' => Icons.lock_outline,
    'ac_unit' => Icons.ac_unit,
    'account_tree' => Icons.account_tree,
    'area_chart' => Icons.area_chart_outlined,
    'add_box' => Icons.add_box_outlined,
    'add_chart' => Icons.add_chart,
    'align_horizontal_left' => Icons.align_horizontal_left,
    'arrow_downward' => Icons.arrow_downward,
    'arrow_drop_down_circle' => Icons.arrow_drop_down_circle_outlined,
    'arrow_upward' => Icons.arrow_upward,
    'aspect_ratio' => Icons.aspect_ratio,
    'attach_money' => Icons.attach_money,
    'auto_graph' => Icons.auto_graph,
    'backspace' => Icons.backspace_outlined,
    'bolt' => Icons.bolt_outlined,
    'border_color' => Icons.border_color,
    'calculate' => Icons.calculate_outlined,
    'call_merge' => Icons.call_merge,
    'category' => Icons.category_outlined,
    'check_box' => Icons.check_box_outlined,
    'color_lens' => Icons.color_lens_outlined,
    'crop' => Icons.crop,
    'crop_square' => Icons.crop_square,
    'dashboard' => Icons.dashboard_outlined,
    'data_object' => Icons.data_object,
    'delete' => Icons.delete_outline,
    'description' => Icons.description_outlined,
    'donut_large' => Icons.donut_large,
    'drive_file_rename_outline' => Icons.drive_file_rename_outline,
    'edit_attributes' => Icons.edit_attributes,
    'edit_note' => Icons.edit_note,
    'enhanced_encryption' => Icons.enhanced_encryption,
    'error_outline' => Icons.error_outline,
    'exposure_neg_1' => Icons.exposure_neg_1,
    'exposure_plus_1' => Icons.exposure_plus_1,
    'fact_check' => Icons.fact_check_outlined,
    'file_download' => Icons.file_download_outlined,
    'filter_alt_off' => Icons.filter_alt_off_outlined,
    'filter_list' => Icons.filter_list,
    'filter_1' => Icons.filter_1,
    'filter_none' => Icons.filter_none,
    'fit_screen' => Icons.fit_screen,
    'flip_to_back' => Icons.flip_to_back,
    'flip_to_front' => Icons.flip_to_front,
    'folder' => Icons.folder_outlined,
    'folder_off' => Icons.folder_off_outlined,
    'font_download' => Icons.font_download_outlined,
    'format_list_numbered' => Icons.format_list_numbered,
    'format_textdirection_r_to_l' => Icons.format_textdirection_r_to_l,
    'format_paint' => Icons.format_paint,
    'format_size' => Icons.format_size,
    'grid_view' => Icons.grid_view,
    'highlight_alt' => Icons.highlight_alt_outlined,
    'history' => Icons.history,
    'height' => Icons.height,
    'indeterminate_check_box' => Icons.indeterminate_check_box_outlined,
    'insert_chart' => Icons.insert_chart_outlined,
    'insights' => Icons.insights,
    'interests' => Icons.interests,
    'join_full' => Icons.join_full,
    'list_alt' => Icons.list_alt,
    'multiline_chart' => Icons.multiline_chart,
    'more_horiz' => Icons.more_horiz,
    'note_add' => Icons.note_add_outlined,
    'numbers' => Icons.numbers,
    'open_in_new' => Icons.open_in_new,
    'percent' => Icons.percent,
    'pivot_table_chart' => Icons.pivot_table_chart,
    'print' => Icons.print_outlined,
    'query_stats' => Icons.query_stats,
    'refresh' => Icons.refresh,
    'radar' => Icons.radar,
    'rule' => Icons.rule,
    'screen_rotation' => Icons.screen_rotation,
    'scatter_plot' => Icons.scatter_plot_outlined,
    'select_all' => Icons.select_all,
    'show_chart' => Icons.show_chart,
    'space_dashboard' => Icons.space_dashboard_outlined,
    'spellcheck' => Icons.spellcheck,
    'splitscreen' => Icons.splitscreen,
    'sticky_note_2' => Icons.sticky_note_2_outlined,
    'stacked_bar_chart' => Icons.stacked_bar_chart,
    'subdirectory_arrow_left' => Icons.subdirectory_arrow_left,
    'subdirectory_arrow_right' => Icons.subdirectory_arrow_right,
    'subscript' => Icons.subscript,
    'superscript' => Icons.superscript,
    'swap_horiz' => Icons.swap_horiz,
    'text_decrease' => Icons.text_decrease,
    'text_fields' => Icons.text_fields,
    'text_increase' => Icons.text_increase,
    'text_rotation_angleup' => Icons.text_rotation_angleup,
    'today' => Icons.today,
    'timeline' => Icons.timeline,
    'translate' => Icons.translate,
    'verified_user' => Icons.verified_user_outlined,
    'vertical_align_bottom' => Icons.vertical_align_bottom,
    'vertical_align_center' => Icons.vertical_align_center,
    'vertical_align_top' => Icons.vertical_align_top,
    'vertical_split' => Icons.vertical_split,
    'visibility' => Icons.visibility_outlined,
    'view_array' => Icons.view_array_outlined,
    'view_column' => Icons.view_column_outlined,
    'view_headline' => Icons.view_headline,
    'view_stream' => Icons.view_stream,
    'width' => Icons.width_normal,
    'wrap_text' => Icons.wrap_text,
    _ => Icons.apps,
  };

  @override
  void dispose() {
    _autoSaveTimer?.cancel();
    _cellEditor.dispose();
    _cellEditorFocus.dispose();
    _formula.dispose();
    _name.dispose();
    _ribbonScroll.dispose();
    _gridX.dispose();
    _gridY.dispose();
    _columnsX.dispose();
    _rowsY.dispose();
    super.dispose();
  }
}
