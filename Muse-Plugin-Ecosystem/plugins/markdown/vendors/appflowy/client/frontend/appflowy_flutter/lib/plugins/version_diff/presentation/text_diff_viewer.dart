import 'dart:math' as math;

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_engine.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_presentation.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_sync.dart';
import 'package:flutter/material.dart';

enum MuseDiffLayout { unified, split }

/// IntelliJ-style, in-Host text comparison workbench.
///
/// The public widget name is kept for plugin compatibility. Unlike the legacy
/// hunk-card viewer, this widget renders two continuous document surfaces and
/// keeps their viewport, change navigation, folding and connector layer in one
/// session.
final class MuseTextDiffViewer extends StatefulWidget {
  const MuseTextDiffViewer({
    super.key,
    required this.document,
    this.initialLayout = MuseDiffLayout.split,
  });

  final MuseTextComparisonDocument document;
  final MuseDiffLayout initialLayout;

  @override
  State<MuseTextDiffViewer> createState() => _MuseTextDiffViewerState();
}

final class _MuseTextDiffViewerState extends State<MuseTextDiffViewer> {
  static const _rowHeight = 25.0;

  final _presentationBuilder = const MuseTextPresentationBuilder();
  final _leftVertical = ScrollController();
  final _rightVertical = ScrollController();
  final _unifiedVertical = ScrollController();
  final _leftHorizontal = ScrollController();
  final _rightHorizontal = ScrollController();
  final Set<String> _expandedFoldIds = {};

  late MuseDiffLayout _layout;
  late MuseTextPresentation _presentation;
  bool _syncScroll = true;
  bool _alignChanges = true;
  bool _collapseUnchanged = true;
  bool _showChanges = true;
  bool _showAudit = false;
  bool _softWrap = false;
  bool _showSearch = false;
  bool _duringSync = false;
  int _currentChange = 0;
  int _searchIndex = 0;
  List<int> _searchHits = const [];
  List<double> _leftPrefix = const [0];
  List<double> _rightPrefix = const [0];
  List<double?> _leftLines = const [];
  List<double?> _rightLines = const [];
  final _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _layout = widget.initialLayout;
    _rebuildPresentation();
    final initialChangeId = widget.document.initialChangeId;
    if (initialChangeId != null) {
      final index = _presentation.runs.indexWhere(
        (run) => run.changeId == initialChangeId,
      );
      if (index >= 0) _currentChange = index;
    }
    _leftVertical.addListener(_syncFromLeft);
    _rightVertical.addListener(_syncFromRight);
    _searchController.addListener(_onSearchChanged);
    _revealCurrentChangeAfterLayout();
  }

  @override
  void didUpdateWidget(covariant MuseTextDiffViewer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.document.diff.comparison.id !=
        widget.document.diff.comparison.id) {
      _expandedFoldIds.clear();
      _currentChange = 0;
      _rebuildPresentation();
      final initialChangeId = widget.document.initialChangeId;
      if (initialChangeId != null) {
        final index = _presentation.runs.indexWhere(
          (run) => run.changeId == initialChangeId,
        );
        if (index >= 0) _currentChange = index;
      }
      _revealCurrentChangeAfterLayout();
    }
  }

  @override
  void dispose() {
    _leftVertical
      ..removeListener(_syncFromLeft)
      ..dispose();
    _rightVertical
      ..removeListener(_syncFromRight)
      ..dispose();
    _unifiedVertical.dispose();
    _leftHorizontal.dispose();
    _rightHorizontal.dispose();
    _searchController
      ..removeListener(_onSearchChanged)
      ..dispose();
    super.dispose();
  }

  void _rebuildPresentation() {
    _presentation = _presentationBuilder.build(
      widget.document.diff.payload,
      collapseUnchanged: _collapseUnchanged,
      expandedFoldIds: _expandedFoldIds,
    );
    final query = _searchController.text.trim();
    if (query.isNotEmpty) {
      final covering = _foldsCoveringSearch(query);
      if (covering.isNotEmpty) {
        _expandedFoldIds.addAll(covering);
        _presentation = _presentationBuilder.build(
          widget.document.diff.payload,
          collapseUnchanged: _collapseUnchanged,
          expandedFoldIds: _expandedFoldIds,
        );
      }
    }
    _leftLines = [
      for (final row in _presentation.rows)
        row.left?.lineNumber == null ? null : row.left!.lineNumber! - 1.0,
    ];
    _rightLines = [
      for (final row in _presentation.rows)
        row.right?.lineNumber == null ? null : row.right!.lineNumber! - 1.0,
    ];
    _recomputeSearch();
  }

  Set<String> _foldsCoveringSearch(String query) {
    final payload = widget.document.diff.payload;
    final leftHits = MuseDartTextDiffEngine.search(payload.baseText, query);
    final rightHits = MuseDartTextDiffEngine.search(payload.targetText, query);
    bool covers(int? start, int? end, List<MuseTextSearchHit> hits) {
      if (start == null || end == null) return false;
      return hits.any((hit) => hit.line >= start && hit.line <= end);
    }

    return {
      for (final row in _presentation.rows)
        if (row.isFold &&
            (covers(row.hiddenLeftStart, row.hiddenLeftEnd, leftHits) ||
                covers(row.hiddenRightStart, row.hiddenRightEnd, rightHits)))
          row.foldId!,
    };
  }

  void _onSearchChanged() {
    setState(_rebuildPresentation);
  }

  void _recomputeSearch() {
    final query = _searchController.text.trim();
    if (query.isEmpty) {
      _searchHits = const [];
      _searchIndex = 0;
      return;
    }
    final needle = query.toLowerCase();
    final hits = <int>[];
    for (var index = 0; index < _presentation.rows.length; index++) {
      final row = _presentation.rows[index];
      final left = row.left?.text.toLowerCase() ?? '';
      final right = row.right?.text.toLowerCase() ?? '';
      if (left.contains(needle) || right.contains(needle)) {
        hits.add(index);
      }
    }
    _searchHits = hits;
    if (_searchHits.isEmpty) {
      _searchIndex = 0;
    } else {
      _searchIndex = _searchIndex.clamp(0, _searchHits.length - 1);
    }
  }

  void _syncFromLeft() => _sync(
        source: _leftVertical,
        target: _rightVertical,
        sourcePrefix: _leftPrefix,
        targetPrefix: _rightPrefix,
        sourceLines: _leftLines,
        targetLines: _rightLines,
        fromLeft: true,
      );

  void _syncFromRight() => _sync(
        source: _rightVertical,
        target: _leftVertical,
        sourcePrefix: _rightPrefix,
        targetPrefix: _leftPrefix,
        sourceLines: _rightLines,
        targetLines: _leftLines,
        fromLeft: false,
      );

  void _sync({
    required ScrollController source,
    required ScrollController target,
    required List<double> sourcePrefix,
    required List<double> targetPrefix,
    required List<double?> sourceLines,
    required List<double?> targetLines,
    required bool fromLeft,
  }) {
    if (!_syncScroll ||
        _duringSync ||
        !source.hasClients ||
        !target.hasClients) {
      return;
    }
    final mapped = MuseTextDiffSync.mapOffset(
      sourceOffset: source.offset,
      viewportHeight: source.position.viewportDimension,
      sourcePrefix: sourcePrefix,
      targetPrefix: targetPrefix,
      sourceLines: sourceLines,
      targetLines: targetLines,
      boundaries: widget.document.diff.payload.similarBoundaries,
      fromLeft: fromLeft,
    ).clamp(target.position.minScrollExtent, target.position.maxScrollExtent);
    if ((target.offset - mapped).abs() < 0.5) return;
    _duringSync = true;
    target.jumpTo(mapped);
    _duringSync = false;
  }

  @override
  Widget build(BuildContext context) {
    final payload = widget.document.diff.payload;
    final colorScheme = Theme.of(context).colorScheme;
    return ColoredBox(
      color: colorScheme.surface,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _buildToolbar(context),
          Divider(height: 1, color: colorScheme.outlineVariant),
          if (payload.hunks.isEmpty)
            const Expanded(child: _NoChangesView())
          else
            Expanded(
              child: LayoutBuilder(
                builder: (context, constraints) => _buildWorkbench(
                  context,
                  constraints.maxWidth,
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildWorkbench(BuildContext context, double width) {
    final showChanges = _showChanges && width >= 920;
    final showAudit = _showAudit && width >= 800;
    return Row(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        if (showChanges) ...[
          SizedBox(
            width: math.min(236, width * 0.22),
            child: _ChangesTree(
              runs: _presentation.runs,
              selectedIndex: _currentChange,
              onSelected: _selectChange,
            ),
          ),
          VerticalDivider(width: 1, color: Theme.of(context).dividerColor),
        ],
        Expanded(
          child: _layout == MuseDiffLayout.split
              ? _buildSplit(context)
              : _UnifiedSurface(
                  presentation: _presentation,
                  controller: _unifiedVertical,
                  rowHeight: _rowHeight,
                  searchQuery: _searchController.text.trim(),
                  onExpandFold: _expandFold,
                ),
        ),
        if (showAudit) ...[
          VerticalDivider(width: 1, color: Theme.of(context).dividerColor),
          SizedBox(
            width: math.min(260, width * 0.24),
            child: _AuditInspector(document: widget.document),
          ),
        ],
      ],
    );
  }

  Widget _buildSplit(BuildContext context) => Column(
        children: [
          _SplitFileHeader(document: widget.document),
          Divider(height: 1, color: Theme.of(context).dividerColor),
          Expanded(
            child: LayoutBuilder(
              builder: (context, constraints) {
                final paneWidth =
                    math.max(80.0, (constraints.maxWidth - 46) / 2);
                final leftExtents = [
                  for (var index = 0;
                      index < _presentation.rows.length;
                      index++)
                    _rowExtent(index, paneWidth, left: true),
                ];
                final rightExtents = [
                  for (var index = 0;
                      index < _presentation.rows.length;
                      index++)
                    _rowExtent(index, paneWidth, left: false),
                ];
                _leftPrefix = MuseTextDiffSync.prefix(leftExtents);
                _rightPrefix = MuseTextDiffSync.prefix(rightExtents);
                return Row(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Expanded(
                      child: _TextSurface(
                        key: const ValueKey('diff-before-surface'),
                        side: _TextSurfaceSide.left,
                        presentation: _presentation,
                        verticalController: _leftVertical,
                        horizontalController: _leftHorizontal,
                        rowHeight: _rowHeight,
                        extents: leftExtents,
                        alignChanges: _alignChanges,
                        softWrap: _softWrap,
                        searchQuery: _searchController.text.trim(),
                        searchHits: _searchHits,
                        onExpandFold: _expandFold,
                        onOverviewTap: _onOverviewTap,
                      ),
                    ),
                    SizedBox(
                      width: 46,
                      child: AnimatedBuilder(
                        animation: Listenable.merge([
                          _leftVertical,
                          _rightVertical,
                        ]),
                        builder: (context, _) => CustomPaint(
                          key: const ValueKey('diff-connector-layer'),
                          painter: _DiffConnectorPainter(
                            runs: _presentation.runs,
                            leftOffsets: _leftPrefix,
                            rightOffsets: _rightPrefix,
                            leftScroll: _leftVertical.hasClients
                                ? _leftVertical.offset
                                : 0,
                            rightScroll: _rightVertical.hasClients
                                ? _rightVertical.offset
                                : 0,
                            selectedChangeId: _selectedChangeId,
                            colorScheme: Theme.of(context).colorScheme,
                          ),
                        ),
                      ),
                    ),
                    Expanded(
                      child: _TextSurface(
                        key: const ValueKey('diff-after-surface'),
                        side: _TextSurfaceSide.right,
                        presentation: _presentation,
                        verticalController: _rightVertical,
                        horizontalController: _rightHorizontal,
                        rowHeight: _rowHeight,
                        extents: rightExtents,
                        alignChanges: _alignChanges,
                        softWrap: _softWrap,
                        searchQuery: _searchController.text.trim(),
                        searchHits: _searchHits,
                        onExpandFold: _expandFold,
                        onOverviewTap: _onOverviewTap,
                      ),
                    ),
                  ],
                );
              },
            ),
          ),
        ],
      );

  double _rowExtent(int index, double paneWidth, {required bool left}) {
    final row = _presentation.rows[index];
    if (row.isFold) return _rowHeight;
    final cell = left ? row.left : row.right;
    final isFiller = cell == null || cell.kind == MuseTextCellKind.filler;
    if (!_alignChanges && isFiller) return 2.0;

    int visualLines(String text) {
      if (!_softWrap) return 1;
      final cols = math.max(1, ((paneWidth - 74) / 7.8).floor());
      if (text.isEmpty) return 1;
      return math.max(1, (text.length / cols).ceil());
    }

    if (_alignChanges) {
      return math.max(
            visualLines(row.left?.text ?? ''),
            visualLines(row.right?.text ?? ''),
          ) *
          _rowHeight;
    }
    return visualLines(cell?.text ?? '') * _rowHeight;
  }

  String? get _selectedChangeId => _presentation.runs.isEmpty
      ? null
      : _presentation
          .runs[_currentChange.clamp(0, _presentation.runs.length - 1)]
          .changeId;

  Widget _buildToolbar(BuildContext context) {
    final payload = widget.document.diff.payload;
    const iconStyle = ButtonStyle(
      visualDensity: VisualDensity.compact,
      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      padding: WidgetStatePropertyAll(EdgeInsets.all(6)),
      minimumSize: WidgetStatePropertyAll(Size(30, 30)),
    );
    return SizedBox(
      height: 36,
      child: Material(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        child: SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          padding: const EdgeInsets.symmetric(horizontal: 8),
          child: Row(
            children: [
              IconButton(
                tooltip: '显示变化树',
                isSelected: _showChanges,
                style: iconStyle,
                onPressed: () => setState(() => _showChanges = !_showChanges),
                icon: const Icon(Icons.account_tree_outlined, size: 16),
              ),
              _MetricChip(
                label: '${payload.changeCount} 处变更',
                color: Theme.of(context).colorScheme.primary,
              ),
              const SizedBox(width: 6),
              _MetricChip(label: '+${payload.additions}', color: Colors.green),
              const SizedBox(width: 6),
              _MetricChip(label: '-${payload.deletions}', color: Colors.red),
              const SizedBox(width: 8),
              SegmentedButton<MuseDiffLayout>(
                style: const ButtonStyle(
                  visualDensity: VisualDensity.compact,
                  tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                ),
                segments: const [
                  ButtonSegment(
                    value: MuseDiffLayout.unified,
                    label: Text('统一'),
                    icon: Icon(Icons.view_stream_outlined, size: 14),
                  ),
                  ButtonSegment(
                    value: MuseDiffLayout.split,
                    label: Text('并排'),
                    icon: Icon(Icons.vertical_split_outlined, size: 14),
                  ),
                ],
                selected: {_layout},
                showSelectedIcon: false,
                onSelectionChanged: (selection) =>
                    setState(() => _layout = selection.single),
              ),
              IconButton(
                tooltip: '上一处变更',
                style: iconStyle,
                onPressed: _presentation.runs.isEmpty ? null : () => _jump(-1),
                icon: const Icon(Icons.keyboard_arrow_up, size: 18),
              ),
              IconButton(
                tooltip: '下一处变更',
                style: iconStyle,
                onPressed: _presentation.runs.isEmpty ? null : () => _jump(1),
                icon: const Icon(Icons.keyboard_arrow_down, size: 18),
              ),
              Text(
                _presentation.runs.isEmpty
                    ? '0 / 0'
                    : '${_currentChange + 1} / ${_presentation.runs.length}',
                style: Theme.of(context).textTheme.labelSmall,
              ),
              _ToolbarToggle(
                tooltip: '同步滚动',
                selected: _syncScroll,
                icon: Icons.sync_alt,
                onPressed: () => setState(() => _syncScroll = !_syncScroll),
              ),
              _ToolbarToggle(
                tooltip: '对齐变化',
                selected: _alignChanges,
                icon: Icons.align_vertical_center,
                onPressed: () => setState(() => _alignChanges = !_alignChanges),
              ),
              _ToolbarToggle(
                tooltip: '折叠未变化区域',
                selected: _collapseUnchanged,
                icon: Icons.unfold_less,
                onPressed: _toggleCollapse,
              ),
              _ToolbarToggle(
                tooltip: '软换行',
                selected: _softWrap,
                icon: Icons.wrap_text,
                onPressed: () => setState(() => _softWrap = !_softWrap),
              ),
              _ToolbarToggle(
                tooltip: '在两侧文档中查找',
                selected: _showSearch,
                icon: Icons.search,
                onPressed: () => setState(() => _showSearch = !_showSearch),
              ),
              if (_showSearch) ...[
                SizedBox(
                  width: 180,
                  height: 28,
                  child: TextField(
                    key: const ValueKey('diff-search-field'),
                    controller: _searchController,
                    autofocus: true,
                    style: const TextStyle(fontSize: 12),
                    decoration: const InputDecoration(
                      isDense: true,
                      hintText: '查找…',
                      border: OutlineInputBorder(),
                      contentPadding: EdgeInsets.symmetric(
                        horizontal: 8,
                        vertical: 6,
                      ),
                    ),
                  ),
                ),
                IconButton(
                  tooltip: '上一处匹配',
                  style: iconStyle,
                  onPressed:
                      _searchHits.isEmpty ? null : () => _jumpSearch(-1),
                  icon: const Icon(Icons.keyboard_arrow_up, size: 18),
                ),
                IconButton(
                  tooltip: '下一处匹配',
                  style: iconStyle,
                  onPressed: _searchHits.isEmpty ? null : () => _jumpSearch(1),
                  icon: const Icon(Icons.keyboard_arrow_down, size: 18),
                ),
                Text(
                  _searchHits.isEmpty
                      ? '0 / 0'
                      : '${_searchIndex + 1} / ${_searchHits.length}',
                  style: Theme.of(context).textTheme.labelSmall,
                ),
              ],
              _ToolbarToggle(
                tooltip: '审计信息',
                selected: _showAudit,
                icon: Icons.fact_check_outlined,
                onPressed: () => setState(() => _showAudit = !_showAudit),
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _toggleCollapse() {
    setState(() {
      _collapseUnchanged = !_collapseUnchanged;
      _expandedFoldIds.clear();
      _rebuildPresentation();
      _currentChange = _currentChange.clamp(
        0,
        math.max(0, _presentation.runs.length - 1),
      );
    });
  }

  void _expandFold(String id) {
    setState(() {
      _expandedFoldIds.add(id);
      _rebuildPresentation();
    });
  }

  void _jump(int delta) {
    if (_presentation.runs.isEmpty) return;
    var next = (_currentChange + delta) % _presentation.runs.length;
    if (next < 0) next += _presentation.runs.length;
    _selectChange(next);
  }

  void _jumpSearch(int delta) {
    if (_searchHits.isEmpty) return;
    var next = (_searchIndex + delta) % _searchHits.length;
    if (next < 0) next += _searchHits.length;
    setState(() => _searchIndex = next);
    _scrollToRow(_searchHits[next]);
  }

  void _selectChange(int index) {
    if (index < 0 || index >= _presentation.runs.length) return;
    setState(() => _currentChange = index);
    _scrollToRow(_presentation.runs[index].startRow);
  }

  void _onOverviewTap(int row) {
    final runIndex = _presentation.runs.indexWhere(
      (run) => row >= run.startRow && row < run.endRow,
    );
    if (runIndex >= 0) {
      _selectChange(runIndex);
      return;
    }
    _scrollToRow(row);
  }

  void _revealCurrentChangeAfterLayout() {
    if (_presentation.runs.isEmpty) return;
    final run = _presentation
        .runs[_currentChange.clamp(0, _presentation.runs.length - 1)];
    _scrollToRow(run.startRow);
  }

  void _scrollToRow(int row) {
    final offset = _offsetForRow(row);
    if (_layout == MuseDiffLayout.unified) {
      _animateTo(_unifiedVertical, offset);
    } else {
      _animateTo(_leftVertical, offset);
    }
  }

  double _offsetForRow(int row) {
    if (row >= 0 && row < _leftPrefix.length) {
      return math.max(0.0, _leftPrefix[row] - _rowHeight * 2);
    }
    return math.max(0.0, row * _rowHeight - _rowHeight * 2);
  }

  void _animateTo(ScrollController controller, double offset) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!controller.hasClients) return;
      controller.animateTo(
        offset.clamp(0, controller.position.maxScrollExtent).toDouble(),
        duration: const Duration(milliseconds: 180),
        curve: Curves.easeOutCubic,
      );
    });
  }
}

enum _TextSurfaceSide { left, right }

final class _TextSurface extends StatelessWidget {
  const _TextSurface({
    super.key,
    required this.side,
    required this.presentation,
    required this.verticalController,
    required this.horizontalController,
    required this.rowHeight,
    required this.extents,
    required this.alignChanges,
    required this.softWrap,
    required this.searchQuery,
    required this.searchHits,
    required this.onExpandFold,
    required this.onOverviewTap,
  });

  final _TextSurfaceSide side;
  final MuseTextPresentation presentation;
  final ScrollController verticalController;
  final ScrollController horizontalController;
  final double rowHeight;
  final List<double> extents;
  final bool alignChanges;
  final bool softWrap;
  final String searchQuery;
  final List<int> searchHits;
  final ValueChanged<String> onExpandFold;
  final ValueChanged<int> onOverviewTap;

  @override
  Widget build(BuildContext context) => LayoutBuilder(
        builder: (context, constraints) {
          final longest = side == _TextSurfaceSide.left
              ? presentation.longestLeftLine
              : presentation.longestRightLine;
          final contentWidth = softWrap
              ? constraints.maxWidth
              : math.max(
                  constraints.maxWidth,
                  74.0 + math.min(longest, 2400) * 7.8,
                );
          final variableExtents = softWrap || !alignChanges;
          final list = ListView.builder(
            controller: verticalController,
            itemCount: presentation.rows.length,
            itemExtent: variableExtents ? null : rowHeight,
            itemExtentBuilder: variableExtents
                ? (index, _) =>
                    index < extents.length ? extents[index] : rowHeight
                : null,
            itemBuilder: (context, index) {
              final row = presentation.rows[index];
              return _TextSurfaceRow(
                row: row,
                side: side,
                alignChanges: alignChanges,
                softWrap: softWrap,
                searchQuery: searchQuery,
                onExpandFold: onExpandFold,
              );
            },
          );
          return SelectionArea(
            key: ValueKey('diff-${side.name}-selection'),
            child: Stack(
              children: [
                Scrollbar(
                  controller: verticalController,
                  notificationPredicate: (notification) =>
                      notification.metrics.axis == Axis.vertical,
                  child: softWrap
                      ? list
                      : Scrollbar(
                          controller: horizontalController,
                          notificationPredicate: (notification) =>
                              notification.metrics.axis == Axis.horizontal,
                          child: SingleChildScrollView(
                            controller: horizontalController,
                            scrollDirection: Axis.horizontal,
                            child: SizedBox(
                              width: contentWidth,
                              height: constraints.maxHeight,
                              child: list,
                            ),
                          ),
                        ),
                ),
                Positioned(
                  top: 0,
                  right: 0,
                  bottom: 0,
                  width: 7,
                  child: GestureDetector(
                    key: ValueKey('diff-${side.name}-overview'),
                    behavior: HitTestBehavior.opaque,
                    onTapDown: (details) {
                      final height = math.max(1.0, constraints.maxHeight);
                      final last = math.max(0, presentation.rows.length - 1);
                      final row = (details.localPosition.dy /
                              height *
                              presentation.rows.length)
                          .floor()
                          .clamp(0, last)
                          .toInt();
                      onOverviewTap(row);
                    },
                    child: AnimatedBuilder(
                      animation: verticalController,
                      builder: (context, _) => CustomPaint(
                        painter: _OverviewPainter(
                          runs: presentation.runs,
                          searchHits: searchHits,
                          totalRows: presentation.rows.length,
                          colorScheme: Theme.of(context).colorScheme,
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      );
}

final class _TextSurfaceRow extends StatelessWidget {
  const _TextSurfaceRow({
    required this.row,
    required this.side,
    required this.alignChanges,
    required this.softWrap,
    required this.searchQuery,
    required this.onExpandFold,
  });

  final MuseTextPresentationRow row;
  final _TextSurfaceSide side;
  final bool alignChanges;
  final bool softWrap;
  final String searchQuery;
  final ValueChanged<String> onExpandFold;

  @override
  Widget build(BuildContext context) {
    if (row.isFold) {
      return _FoldRow(
        hiddenLineCount: row.hiddenLineCount,
        onPressed: () => onExpandFold(row.foldId!),
      );
    }
    final cell = side == _TextSurfaceSide.left ? row.left : row.right;
    final scheme = Theme.of(context).colorScheme;
    final kind = cell?.kind ?? MuseTextCellKind.filler;
    final background = switch (kind) {
      MuseTextCellKind.deleted => Colors.red.withValues(alpha: 0.14),
      MuseTextCellKind.inserted => Colors.green.withValues(alpha: 0.14),
      MuseTextCellKind.filler => alignChanges
          ? scheme.surfaceContainerHighest.withValues(alpha: 0.42)
          : scheme.surface,
      _ => scheme.surface,
    };
    final marker = switch (kind) {
      MuseTextCellKind.deleted => Colors.red.shade400,
      MuseTextCellKind.inserted => Colors.green.shade500,
      _ => Colors.transparent,
    };
    return ColoredBox(
      color: background,
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(width: 3, color: marker),
          Container(
            width: 51,
            alignment: Alignment.centerRight,
            padding: const EdgeInsets.only(right: 8),
            color: scheme.surfaceContainerHighest.withValues(alpha: 0.5),
            child: SelectionContainer.disabled(
              child: Text(
                cell?.lineNumber?.toString() ?? '',
                style: TextStyle(
                  color: scheme.onSurfaceVariant,
                  fontFamily: 'monospace',
                  fontSize: 11,
                ),
              ),
            ),
          ),
          SizedBox(
            width: 20,
            child: Center(
              child: Text(
                kind == MuseTextCellKind.deleted
                    ? '−'
                    : kind == MuseTextCellKind.inserted
                        ? '+'
                        : '',
                style: TextStyle(
                  color: marker,
                  fontWeight: FontWeight.w700,
                  fontSize: 12,
                ),
              ),
            ),
          ),
          Expanded(
            child: Align(
              alignment: Alignment.centerLeft,
              child: kind == MuseTextCellKind.filler
                  ? const SelectionContainer.disabled(
                      child: SizedBox.shrink(
                        key: ValueKey('diff-filler-cell'),
                      ),
                    )
                  : Text.rich(
                      TextSpan(
                        style: TextStyle(
                          color: scheme.onSurface,
                          fontFamily: 'monospace',
                          fontSize: 12.5,
                          height: 1.2,
                        ),
                        children: _inlineSpans(cell!, kind),
                      ),
                      maxLines: softWrap ? null : 1,
                      softWrap: softWrap,
                      overflow:
                          softWrap ? TextOverflow.visible : TextOverflow.clip,
                    ),
            ),
          ),
        ],
      ),
    );
  }

  List<InlineSpan> _inlineSpans(
    MuseTextPresentationCell cell,
    MuseTextCellKind kind,
  ) {
    if (cell.inlineSpans.isEmpty && searchQuery.isEmpty) {
      return [TextSpan(text: cell.text)];
    }
    if (searchQuery.isNotEmpty) {
      return _searchSpans(cell.text);
    }
    final changedColor = kind == MuseTextCellKind.deleted
        ? Colors.red.withValues(alpha: 0.34)
        : Colors.green.withValues(alpha: 0.34);
    return [
      for (final span in cell.inlineSpans)
        TextSpan(
          text: span.text,
          style: span.changed
              ? TextStyle(
                  backgroundColor: changedColor,
                  fontWeight: FontWeight.w600,
                )
              : null,
        ),
    ];
  }

  List<InlineSpan> _searchSpans(String text) {
    final needle = searchQuery.toLowerCase();
    if (needle.isEmpty || text.isEmpty) return [TextSpan(text: text)];
    final haystack = text.toLowerCase();
    final spans = <InlineSpan>[];
    var start = 0;
    while (start < text.length) {
      final index = haystack.indexOf(needle, start);
      if (index < 0) {
        spans.add(TextSpan(text: text.substring(start)));
        break;
      }
      if (index > start) {
        spans.add(TextSpan(text: text.substring(start, index)));
      }
      spans.add(
        TextSpan(
          text: text.substring(index, index + searchQuery.length),
          style: TextStyle(
            backgroundColor: Colors.amber.withValues(alpha: 0.55),
            fontWeight: FontWeight.w700,
          ),
        ),
      );
      start = index + searchQuery.length;
    }
    return spans;
  }
}

final class _FoldRow extends StatelessWidget {
  const _FoldRow({
    required this.hiddenLineCount,
    required this.onPressed,
  });

  final int hiddenLineCount;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return Material(
      color: scheme.surfaceContainerLow,
      child: InkWell(
        onTap: onPressed,
        child: Row(
          children: [
            Expanded(child: Divider(color: scheme.outlineVariant)),
            const SizedBox(width: 8),
            const Icon(Icons.unfold_more, size: 14),
            const SizedBox(width: 4),
            Text(
              '展开 $hiddenLineCount 行未变化内容',
              style: Theme.of(context).textTheme.labelSmall,
            ),
            const SizedBox(width: 8),
            Expanded(child: Divider(color: scheme.outlineVariant)),
          ],
        ),
      ),
    );
  }
}

final class _UnifiedSurface extends StatelessWidget {
  const _UnifiedSurface({
    required this.presentation,
    required this.controller,
    required this.rowHeight,
    required this.searchQuery,
    required this.onExpandFold,
  });

  final MuseTextPresentation presentation;
  final ScrollController controller;
  final double rowHeight;
  final String searchQuery;
  final ValueChanged<String> onExpandFold;

  @override
  Widget build(BuildContext context) => ListView.builder(
        key: const ValueKey('diff-unified-surface'),
        controller: controller,
        itemCount: presentation.rows.length,
        itemBuilder: (context, index) {
          final row = presentation.rows[index];
          if (row.isFold) {
            return SizedBox(
              height: rowHeight,
              child: _FoldRow(
                hiddenLineCount: row.hiddenLineCount,
                onPressed: () => onExpandFold(row.foldId!),
              ),
            );
          }
          final cells = <(_TextSurfaceSide, MuseTextPresentationCell)>[];
          final left = row.left;
          if (left != null && left.kind != MuseTextCellKind.filler) {
            cells.add((_TextSurfaceSide.left, left));
          }
          final right = row.right;
          if (right != null &&
              right.kind != MuseTextCellKind.filler &&
              (row.isChange || row.left == null)) {
            cells.add((_TextSurfaceSide.right, right));
          }
          return Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final (side, _) in cells)
                SizedBox(
                  height: rowHeight,
                  child: _TextSurfaceRow(
                    row: row,
                    side: side,
                    alignChanges: false,
                    softWrap: false,
                    searchQuery: searchQuery,
                    onExpandFold: onExpandFold,
                  ),
                ),
            ],
          );
        },
      );
}

final class _ChangesTree extends StatelessWidget {
  const _ChangesTree({
    required this.runs,
    required this.selectedIndex,
    required this.onSelected,
  });

  final List<MuseTextPresentationRun> runs;
  final int selectedIndex;
  final ValueChanged<int> onSelected;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return ColoredBox(
      color: scheme.surfaceContainerLow,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
            child: Text(
              '变更 ${runs.length}',
              style: Theme.of(context).textTheme.titleSmall,
            ),
          ),
          Expanded(
            child: ListView.builder(
              key: const ValueKey('diff-changes-tree'),
              itemCount: runs.length,
              itemBuilder: (context, index) {
                final run = runs[index];
                final color = _changeColor(run.kind);
                return ListTile(
                  dense: true,
                  selected: index == selectedIndex,
                  leading: Icon(_changeIcon(run.kind), size: 16, color: color),
                  title: Text(
                    run.semanticLabel.isEmpty
                        ? _changeLabel(run.kind)
                        : run.semanticLabel,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  subtitle: Text(
                    '${_changeLabel(run.kind)} · L${run.startRow + 1}',
                    maxLines: 1,
                  ),
                  onTap: () => onSelected(index),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

final class _AuditInspector extends StatelessWidget {
  const _AuditInspector({required this.document});

  final MuseTextComparisonDocument document;

  @override
  Widget build(BuildContext context) {
    final comparison = document.diff.comparison;
    return ColoredBox(
      color: Theme.of(context).colorScheme.surfaceContainerLow,
      child: ListView(
        key: const ValueKey('diff-audit-inspector'),
        padding: const EdgeInsets.all(14),
        children: [
          Text('审计', style: Theme.of(context).textTheme.titleSmall),
          const SizedBox(height: 14),
          _InspectorField(label: '比较发起者', value: comparison.actor.displayName),
          _InspectorField(
            label: '基线创建者',
            value: document.base.actor.displayName,
          ),
          _InspectorField(
            label: '目标创建者',
            value: document.target.actor.displayName,
          ),
          _InspectorField(
            label: '基线版本',
            value: _short(document.base.contentDigest),
          ),
          _InspectorField(
            label: '目标版本',
            value: _short(document.target.contentDigest),
          ),
          _InspectorField(label: 'Comparison', value: _short(comparison.id)),
          _InspectorField(
            label: '时间',
            value: comparison.createdAt.toLocal().toString(),
          ),
          _InspectorField(label: 'Provider', value: comparison.rendererType),
          if (document.semanticChangeSet case final changeSet?) ...[
            _InspectorField(label: 'Change schema', value: changeSet.schema),
            _InspectorField(
              label: 'Diff quality',
              value: changeSet.quality.kind.name,
            ),
          ],
        ],
      ),
    );
  }

  static String _short(String value) =>
      value.length <= 16 ? value : value.substring(0, 16);
}

final class _InspectorField extends StatelessWidget {
  const _InspectorField({required this.label, required this.value});

  final String label;
  final String value;

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.only(bottom: 12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(label, style: Theme.of(context).textTheme.labelSmall),
            const SizedBox(height: 3),
            SelectableText(
              value,
              style: const TextStyle(fontFamily: 'monospace', fontSize: 12),
            ),
          ],
        ),
      );
}

final class _DiffConnectorPainter extends CustomPainter {
  const _DiffConnectorPainter({
    required this.runs,
    required this.leftOffsets,
    required this.rightOffsets,
    required this.leftScroll,
    required this.rightScroll,
    required this.selectedChangeId,
    required this.colorScheme,
  });

  final List<MuseTextPresentationRun> runs;
  final List<double> leftOffsets;
  final List<double> rightOffsets;
  final double leftScroll;
  final double rightScroll;
  final String? selectedChangeId;
  final ColorScheme colorScheme;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(
      Offset.zero & size,
      Paint()..color = colorScheme.surfaceContainerLow,
    );
    for (final run in runs) {
      final leftTop = _edge(leftOffsets, run.startRow) - leftScroll;
      final leftBottom = _edge(leftOffsets, run.endRow) - leftScroll;
      final rightTop = _edge(rightOffsets, run.startRow) - rightScroll;
      final rightBottom = _edge(rightOffsets, run.endRow) - rightScroll;
      if (math.max(leftBottom, rightBottom) < 0 ||
          math.min(leftTop, rightTop) > size.height) {
        continue;
      }
      final color = _changeColor(run.kind);
      final selected = run.changeId == selectedChangeId;
      final path = Path()
        ..moveTo(0, leftTop)
        ..cubicTo(
          size.width * .30,
          leftTop,
          size.width * .70,
          rightTop,
          size.width,
          rightTop,
        )
        ..lineTo(size.width, rightBottom)
        ..cubicTo(
          size.width * .70,
          rightBottom,
          size.width * .30,
          leftBottom,
          0,
          leftBottom,
        )
        ..close();
      canvas.drawPath(
        path,
        Paint()
          ..color = color.withValues(alpha: selected ? 0.32 : 0.18)
          ..style = PaintingStyle.fill,
      );
      canvas.drawPath(
        path,
        Paint()
          ..color = color.withValues(alpha: selected ? 0.9 : 0.45)
          ..strokeWidth = selected ? 1.5 : 1
          ..style = PaintingStyle.stroke,
      );
    }
  }

  double _edge(List<double> offsets, int row) {
    if (offsets.isEmpty) return 0;
    if (row < 0) return offsets.first;
    if (row >= offsets.length) return offsets.last;
    return offsets[row];
  }

  @override
  bool shouldRepaint(covariant _DiffConnectorPainter oldDelegate) =>
      oldDelegate.leftScroll != leftScroll ||
      oldDelegate.rightScroll != rightScroll ||
      oldDelegate.selectedChangeId != selectedChangeId ||
      oldDelegate.runs != runs ||
      oldDelegate.leftOffsets != leftOffsets ||
      oldDelegate.rightOffsets != rightOffsets ||
      oldDelegate.colorScheme != colorScheme;
}

final class _OverviewPainter extends CustomPainter {
  const _OverviewPainter({
    required this.runs,
    required this.searchHits,
    required this.totalRows,
    required this.colorScheme,
  });

  final List<MuseTextPresentationRun> runs;
  final List<int> searchHits;
  final int totalRows;
  final ColorScheme colorScheme;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(
      Offset.zero & size,
      Paint()..color = colorScheme.surfaceContainerHighest,
    );
    final denominator = math.max(1, totalRows);
    for (final run in runs) {
      final top = run.startRow / denominator * size.height;
      final height = math.max(
        2.0,
        (run.endRow - run.startRow) / denominator * size.height,
      );
      canvas.drawRect(
        Rect.fromLTWH(0, top, size.width, height),
        Paint()..color = _changeColor(run.kind).withValues(alpha: 0.9),
      );
    }
    for (final row in searchHits) {
      final top = row / denominator * size.height;
      canvas.drawRect(
        Rect.fromLTWH(
            0, top, size.width, math.max(2.0, size.height / denominator)),
        Paint()..color = Colors.amber.withValues(alpha: 0.85),
      );
    }
  }

  @override
  bool shouldRepaint(covariant _OverviewPainter oldDelegate) =>
      oldDelegate.runs != runs ||
      oldDelegate.searchHits != searchHits ||
      oldDelegate.totalRows != totalRows ||
      oldDelegate.colorScheme != colorScheme;
}

final class _SplitFileHeader extends StatelessWidget {
  const _SplitFileHeader({required this.document});

  final MuseTextComparisonDocument document;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    Widget side({
      required IconData icon,
      required String title,
      required String digest,
      required String actor,
    }) =>
        Expanded(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            child: Row(
              children: [
                Icon(icon, size: 15, color: scheme.onSurfaceVariant),
                const SizedBox(width: 6),
                Text(
                  title,
                  style: const TextStyle(fontWeight: FontWeight.w600),
                ),
                const SizedBox(width: 7),
                Text(
                  digest.length <= 8 ? digest : digest.substring(0, 8),
                  style: TextStyle(
                    color: scheme.onSurfaceVariant,
                    fontFamily: 'monospace',
                    fontSize: 10.5,
                  ),
                ),
                const Spacer(),
                Flexible(
                  child: Text(
                    actor,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.labelSmall,
                  ),
                ),
              ],
            ),
          ),
        );
    return ColoredBox(
      color: scheme.surfaceContainerLow,
      child: Row(
        children: [
          side(
            icon: Icons.history,
            title: '基线版本',
            digest: document.base.contentDigest,
            actor: document.base.actor.displayName,
          ),
          const SizedBox(width: 46),
          side(
            icon: Icons.edit_outlined,
            title: '当前版本',
            digest: document.target.contentDigest,
            actor: document.target.actor.displayName,
          ),
        ],
      ),
    );
  }
}

final class _MetricChip extends StatelessWidget {
  const _MetricChip({required this.label, required this.color});

  final String label;
  final Color color;

  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 3),
        decoration: BoxDecoration(
          color: color.withValues(alpha: 0.12),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Text(
          label,
          style: TextStyle(
            color: color,
            fontSize: 11.5,
            fontWeight: FontWeight.w600,
          ),
        ),
      );
}

final class _ToolbarToggle extends StatelessWidget {
  const _ToolbarToggle({
    required this.tooltip,
    required this.selected,
    required this.icon,
    required this.onPressed,
  });

  final String tooltip;
  final bool selected;
  final IconData icon;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) => IconButton(
        tooltip: tooltip,
        isSelected: selected,
        style: const ButtonStyle(
          visualDensity: VisualDensity.compact,
          tapTargetSize: MaterialTapTargetSize.shrinkWrap,
          padding: WidgetStatePropertyAll(EdgeInsets.all(6)),
          minimumSize: WidgetStatePropertyAll(Size(30, 30)),
        ),
        selectedIcon: Icon(icon, size: 16),
        icon: Icon(icon, size: 16),
        onPressed: onPressed,
      );
}

final class _NoChangesView extends StatelessWidget {
  const _NoChangesView();

  @override
  Widget build(BuildContext context) => const Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.check_circle_outline, size: 36),
            SizedBox(height: 12),
            Text('没有语义变化'),
            SizedBox(height: 4),
            Text('两个版本的文本内容一致'),
          ],
        ),
      );
}

Color _changeColor(MuseTextChangeKind kind) => switch (kind) {
      MuseTextChangeKind.insert => Colors.green.shade600,
      MuseTextChangeKind.delete => Colors.red.shade500,
      MuseTextChangeKind.replace => Colors.blue.shade500,
    };

IconData _changeIcon(MuseTextChangeKind kind) => switch (kind) {
      MuseTextChangeKind.insert => Icons.add_circle_outline,
      MuseTextChangeKind.delete => Icons.remove_circle_outline,
      MuseTextChangeKind.replace => Icons.change_circle_outlined,
    };

String _changeLabel(MuseTextChangeKind kind) => switch (kind) {
      MuseTextChangeKind.insert => '新增',
      MuseTextChangeKind.delete => '删除',
      MuseTextChangeKind.replace => '修改',
    };
