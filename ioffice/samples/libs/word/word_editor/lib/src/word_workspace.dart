import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:word_render/word_render.dart';

import 'editor_controller.dart';
import 'word_ruler.dart';

const _kWordBlue = Color(0xFF185ABD);
const _kRibbonBg = Color(0xFFF3F3F3);
const _kCanvasBg = Color(0xFFBFC3C7);
const _kTabInactive = Color(0xFF2B2B2B);

/// Word-inspired workspace: title + ribbon + paginated scroll + status.
class WordWorkspace extends StatefulWidget {
  const WordWorkspace({
    super.key,
    required this.controller,
    this.onOpenDocument,
    this.onReloadBundled,
    this.showChrome = true,
    this.readOnly = false,
    this.canvasColor = _kCanvasBg,
    this.pageTopInset = 24,
    this.pageBottomInset = 24,
    this.pageGap = 28,
  });

  final WordEditorController controller;
  final VoidCallback? onOpenDocument;
  final VoidCallback? onReloadBundled;

  /// Hide editing controls when a host shows the page beside a reference.
  final bool showChrome;
  final bool readOnly;
  final Color canvasColor;
  final double pageTopInset;
  final double pageBottomInset;
  final double pageGap;

  @override
  State<WordWorkspace> createState() => _WordWorkspaceState();
}

class _WordWorkspaceState extends State<WordWorkspace> {
  static const _tabs = ['File', 'Home', 'View'];
  String _tab = 'Home';
  final _scroll = ScrollController(keepScrollOffset: false);
  final _editFocus = FocusNode();
  Timer? _visiblePageTimer;
  RenderEnvelope? _lastEnvelope;
  int _lastPageNavigationSerial = 0;
  int _pageColumns = 1;

  WordEditorController get c => widget.controller;
  double get _effectivePageTopInset =>
      c.readingMode && widget.pageTopInset < 44 ? 44 : widget.pageTopInset;
  double get _rulerInset =>
      c.showRuler && !c.multiplePages && c.pageCount > 0 ? 26 * c.zoom : 0;

  @override
  void initState() {
    super.initState();
    c.addListener(_onCtl);
    _scroll.addListener(_onScroll);
  }

  @override
  void dispose() {
    c.removeListener(_onCtl);
    _scroll.removeListener(_onScroll);
    _visiblePageTimer?.cancel();
    _scroll.dispose();
    _editFocus.dispose();
    super.dispose();
  }

  void _onCtl() {
    if (!identical(_lastEnvelope, c.envelope)) {
      _lastEnvelope = c.envelope;
      _visiblePageTimer?.cancel();
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted && _scroll.hasClients) _scroll.jumpTo(0);
      });
    }
    if (_lastPageNavigationSerial != c.pageNavigationSerial) {
      _lastPageNavigationSerial = c.pageNavigationSerial;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!mounted || !_scroll.hasClients || c.envelope == null) return;
        final env = c.envelope!;
        final pageHeight = twipToPx(env.page.pageHeightTwip, c.zoom);
        final target = _effectivePageTopInset +
            _rulerInset +
            ((c.requestedPage - 1) ~/ _pageColumns) *
                (pageHeight + widget.pageGap);
        _scroll.jumpTo(target.clamp(0.0, _scroll.position.maxScrollExtent));
      });
    }
    if (mounted) setState(() {});
  }

  void _onScroll() {
    final env = c.envelope;
    if (env == null || !_scroll.hasClients) return;
    final pageH = twipToPx(env.page.pageHeightTwip, c.zoom) + widget.pageGap;
    final idx = (((_scroll.offset - _rulerInset).clamp(0.0, double.infinity) +
                pageH * 0.35) /
            pageH)
        .floor();
    final rowFirstPage = idx * _pageColumns + 1;
    // Keep the active page when two visible pages share a row. Scrolling to
    // another row selects that row's first page.
    final next = (c.visiblePage - 1) ~/ _pageColumns == idx
        ? c.visiblePage
        : rowFirstPage;
    _visiblePageTimer?.cancel();
    if (next == c.visiblePage) return;
    _visiblePageTimer = Timer(const Duration(milliseconds: 120), () {
      if (mounted) c.setVisiblePage(next);
    });
  }

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: Colors.white,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (widget.showChrome) ...[
            _TitleBar(
              name: c.documentName,
              onOpen: widget.onOpenDocument,
              onReload: widget.onReloadBundled,
            ),
            _TabStrip(
              tabs: _tabs,
              selected: _tab,
              onSelect: (t) {
                if (t == 'File') {
                  widget.onOpenDocument?.call();
                  return;
                }
                setState(() => _tab = t);
              },
            ),
            if (_tab == 'Home') _HomeRibbon(controller: c),
            if (_tab == 'View') _ViewRibbon(controller: c),
            const Divider(height: 1, color: Color(0xFFD0D0D0)),
          ],
          Expanded(
            child: widget.readOnly
                ? _DocumentCanvas(
                    controller: c,
                    scroll: _scroll,
                    readOnly: true,
                    canvasColor: widget.canvasColor,
                    pageTopInset: _effectivePageTopInset,
                    pageBottomInset: widget.pageBottomInset,
                    pageGap: widget.pageGap,
                    onColumnsChanged: (n) => _pageColumns = n)
                : Focus(
                    focusNode: _editFocus,
                    autofocus: true,
                    onKeyEvent: (node, event) {
                      final k = event.logicalKey;
                      final ch = event.character;
                      final typing = k == LogicalKeyboardKey.backspace ||
                          k == LogicalKeyboardKey.delete ||
                          k == LogicalKeyboardKey.enter ||
                          k == LogicalKeyboardKey.numpadEnter ||
                          (ch != null &&
                              ch.isNotEmpty &&
                              ch.codeUnitAt(0) >= 0x20);
                      if (!typing) return KeyEventResult.ignored;
                      c.handleKeyEvent(event);
                      return KeyEventResult.handled;
                    },
                    child: GestureDetector(
                      onTap: () => _editFocus.requestFocus(),
                      child: _DocumentCanvas(
                          controller: c,
                          scroll: _scroll,
                          canvasColor: widget.canvasColor,
                          pageTopInset: _effectivePageTopInset,
                          pageBottomInset: widget.pageBottomInset,
                          pageGap: widget.pageGap,
                          onColumnsChanged: (n) => _pageColumns = n),
                    ),
                  ),
          ),
          if (widget.showChrome) _StatusBar(controller: c),
        ],
      ),
    );
  }
}

class _TitleBar extends StatelessWidget {
  const _TitleBar({
    required this.name,
    this.onOpen,
    this.onReload,
  });

  final String name;
  final VoidCallback? onOpen;
  final VoidCallback? onReload;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 40,
      color: _kWordBlue,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      child: Row(
        children: [
          Container(
            width: 22,
            height: 22,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: Colors.white,
              borderRadius: BorderRadius.circular(3),
            ),
            child: const Text(
              'W',
              style: TextStyle(
                color: _kWordBlue,
                fontWeight: FontWeight.w800,
                fontSize: 14,
              ),
            ),
          ),
          const SizedBox(width: 10),
          Expanded(
            child: Text(
              name,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 13,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
          IconButton(
            tooltip: 'Reload sample',
            visualDensity: VisualDensity.compact,
            color: Colors.white,
            onPressed: onReload,
            icon: const Icon(Icons.refresh, size: 18),
          ),
          IconButton(
            tooltip: 'Open',
            visualDensity: VisualDensity.compact,
            color: Colors.white,
            onPressed: onOpen,
            icon: const Icon(Icons.folder_open, size: 18),
          ),
        ],
      ),
    );
  }
}

class _TabStrip extends StatelessWidget {
  const _TabStrip({
    required this.tabs,
    required this.selected,
    required this.onSelect,
  });

  final List<String> tabs;
  final String selected;
  final ValueChanged<String> onSelect;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 32,
      color: Colors.white,
      padding: const EdgeInsets.only(left: 8),
      child: Row(
        children: [
          for (final t in tabs)
            InkWell(
              onTap: () => onSelect(t),
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 14),
                alignment: Alignment.center,
                decoration: BoxDecoration(
                  border: Border(
                    bottom: BorderSide(
                      color: t == selected ? _kWordBlue : Colors.transparent,
                      width: 3,
                    ),
                  ),
                ),
                child: Text(
                  t,
                  style: TextStyle(
                    fontSize: 13,
                    color: t == selected ? _kWordBlue : _kTabInactive,
                    fontWeight:
                        t == selected ? FontWeight.w600 : FontWeight.w400,
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }
}

class _HomeRibbon extends StatelessWidget {
  const _HomeRibbon({required this.controller});
  final WordEditorController controller;

  @override
  Widget build(BuildContext context) {
    final c = controller;
    return Container(
      height: 88,
      color: _kRibbonBg,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _RibbonGroup(
              label: 'Clipboard',
              child: _RibbonIcon(
                icon: Icons.content_paste,
                label: 'Paste',
                onTap: () {},
              ),
            ),
            _vdiv(),
            _RibbonGroup(
              label: 'Edit',
              child: Text(
                c.engine.session != null
                    ? 'Type after tap (session)'
                    : 'Golden: view only',
                style: const TextStyle(fontSize: 11, color: Color(0xFF555555)),
              ),
            ),
            _vdiv(),
            _RibbonGroup(
              label: 'Font',
              child: Row(
                children: [
                  _SizeMenu(controller: c),
                  const SizedBox(width: 6),
                  _Toggle(
                    label: 'B',
                    tooltip: 'Bold (draft — kernel CHPX later)',
                    selected: c.draft.bold,
                    weight: FontWeight.w800,
                    onTap: c.toggleBold,
                  ),
                  _Toggle(
                    label: 'I',
                    tooltip: 'Italic (draft)',
                    selected: c.draft.italic,
                    italic: true,
                    onTap: c.toggleItalic,
                  ),
                  _Toggle(
                    label: 'U',
                    tooltip: 'Underline (relayout)',
                    selected: c.view.underline,
                    underline: true,
                    onTap: () => c.setUnderline(!c.view.underline),
                  ),
                ],
              ),
            ),
            _vdiv(),
            _RibbonGroup(
              label: 'Paragraph',
              child: Row(
                children: [
                  _AlignBtn(
                    icon: Icons.format_align_left,
                    selected: c.view.jc == 0,
                    onTap: () => c.setAlign(0),
                  ),
                  _AlignBtn(
                    icon: Icons.format_align_center,
                    selected: c.view.jc == 1,
                    onTap: () => c.setAlign(1),
                  ),
                  _AlignBtn(
                    icon: Icons.format_align_right,
                    selected: c.view.jc == 2,
                    onTap: () => c.setAlign(2),
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

class _ViewRibbon extends StatelessWidget {
  const _ViewRibbon({required this.controller});
  final WordEditorController controller;

  @override
  Widget build(BuildContext context) {
    final c = controller;
    return Container(
      height: 88,
      color: _kRibbonBg,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: _RibbonGroup(
        label: 'Zoom',
        child: Row(
          children: [
            SizedBox(
              width: 220,
              child: Slider(
                value: c.zoom,
                min: 0.15,
                max: 2.0,
                activeColor: _kWordBlue,
                onChanged: c.setZoom,
              ),
            ),
            Text('${(c.zoom * 100).round()}%'),
          ],
        ),
      ),
    );
  }
}

Widget _vdiv() => const Padding(
      padding: EdgeInsets.symmetric(horizontal: 8),
      child: VerticalDivider(width: 1, color: Color(0xFFD8D8D8)),
    );

class _RibbonGroup extends StatelessWidget {
  const _RibbonGroup({required this.label, required this.child});
  final String label;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Expanded(child: Align(alignment: Alignment.centerLeft, child: child)),
        Text(
          label,
          style: const TextStyle(fontSize: 10, color: Color(0xFF666666)),
        ),
      ],
    );
  }
}

class _RibbonIcon extends StatelessWidget {
  const _RibbonIcon({
    required this.icon,
    required this.label,
    required this.onTap,
  });
  final IconData icon;
  final String label;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 8),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 22, color: const Color(0xFF444444)),
            const SizedBox(height: 2),
            Text(label, style: const TextStyle(fontSize: 11)),
          ],
        ),
      ),
    );
  }
}

class _Toggle extends StatelessWidget {
  const _Toggle({
    required this.label,
    required this.selected,
    required this.onTap,
    this.tooltip,
    this.weight,
    this.italic = false,
    this.underline = false,
  });

  final String label;
  final bool selected;
  final VoidCallback onTap;
  final String? tooltip;
  final FontWeight? weight;
  final bool italic;
  final bool underline;

  @override
  Widget build(BuildContext context) {
    final btn = InkWell(
      onTap: onTap,
      child: Container(
        width: 28,
        height: 28,
        margin: const EdgeInsets.only(right: 2),
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: selected ? const Color(0xFFCDE6F7) : Colors.transparent,
          borderRadius: BorderRadius.circular(3),
          border: Border.all(
            color: selected ? const Color(0xFF90CAF9) : Colors.transparent,
          ),
        ),
        child: Text(
          label,
          style: TextStyle(
            fontSize: 14,
            fontWeight: weight ?? FontWeight.w600,
            fontStyle: italic ? FontStyle.italic : FontStyle.normal,
            decoration:
                underline ? TextDecoration.underline : TextDecoration.none,
          ),
        ),
      ),
    );
    return tooltip == null ? btn : Tooltip(message: tooltip!, child: btn);
  }
}

class _AlignBtn extends StatelessWidget {
  const _AlignBtn({
    required this.icon,
    required this.selected,
    required this.onTap,
  });
  final IconData icon;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: Container(
        width: 28,
        height: 28,
        margin: const EdgeInsets.only(right: 2),
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: selected ? const Color(0xFFCDE6F7) : Colors.transparent,
          borderRadius: BorderRadius.circular(3),
        ),
        child: Icon(icon, size: 18, color: const Color(0xFF333333)),
      ),
    );
  }
}

class _SizeMenu extends StatelessWidget {
  const _SizeMenu({required this.controller});
  final WordEditorController controller;

  static const sizes = [10.0, 11.0, 12.0, 14.0, 16.0, 18.0, 24.0];

  @override
  Widget build(BuildContext context) {
    final v = controller.view.fontSizePt;
    return Container(
      height: 28,
      padding: const EdgeInsets.symmetric(horizontal: 6),
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border.all(color: const Color(0xFFC8C8C8)),
        borderRadius: BorderRadius.circular(3),
      ),
      child: DropdownButtonHideUnderline(
        child: DropdownButton<double>(
          value: sizes.contains(v) ? v : 12.0,
          isDense: true,
          items: [
            for (final s in sizes)
              DropdownMenuItem(value: s, child: Text('${s.round()}')),
          ],
          onChanged: (s) {
            if (s != null) controller.setFontSizePt(s);
          },
        ),
      ),
    );
  }
}

class _DocumentCanvas extends StatelessWidget {
  const _DocumentCanvas({
    required this.controller,
    required this.scroll,
    required this.canvasColor,
    required this.pageTopInset,
    required this.pageBottomInset,
    required this.pageGap,
    required this.onColumnsChanged,
    this.readOnly = false,
  });

  final WordEditorController controller;
  final ScrollController scroll;
  final Color canvasColor;
  final double pageTopInset;
  final double pageBottomInset;
  final double pageGap;
  final ValueChanged<int> onColumnsChanged;
  final bool readOnly;

  @override
  Widget build(BuildContext context) {
    final env = controller.envelope;
    final surfaceColor =
        controller.darkPage ? const Color(0xFF3D3D3D) : canvasColor;
    if (env == null) {
      return ColoredBox(
        color: surfaceColor,
        child: Center(
          child: Text(
            controller.status,
            style: const TextStyle(color: Color(0xFF444444)),
          ),
        ),
      );
    }
    final n = env.page.pageCount <= 0 ? 1 : env.page.pageCount;
    final pageW = twipToPx(env.page.pageWidthTwip, controller.zoom);
    final pageH = twipToPx(env.page.pageHeightTwip, controller.zoom);
    return ColoredBox(
      color: surfaceColor,
      child: LayoutBuilder(builder: (context, constraints) {
        final basePageW = twipToPx(env.page.pageWidthTwip, 1.0);
        final basePageH = twipToPx(env.page.pageHeightTwip, 1.0);
        final widthFit = ((constraints.maxWidth - 48) / basePageW)
            .clamp(0.1, 5.0)
            .toDouble();
        final heightFit =
            ((constraints.maxHeight - pageTopInset - 24) / basePageH)
                .clamp(0.1, 5.0)
                .toDouble();
        // GenOffice lays out as many pages across as the viewport fits. Keep
        // a single readable column on phone-sized viewports.
        final canTile = controller.multiplePages && constraints.maxWidth >= 600;
        final twoPageFit =
            ((constraints.maxWidth - 48 - pageGap) / (2 * basePageW))
                .clamp(0.1, 5.0)
                .toDouble();
        final multipleFit = canTile
            ? math.min(heightFit, twoPageFit)
            : math.min(widthFit, heightFit);
        final columns = canTile
            ? math.max(
                1,
                ((constraints.maxWidth - 48 + pageGap) / (pageW + pageGap))
                    .floor())
            : 1;
        onColumnsChanged(columns);
        final fitZoom =
            controller.zoomFitMode == null ? math.min(widthFit, 1.0) : widthFit;
        WidgetsBinding.instance.addPostFrameCallback((_) {
          controller.setAutoFitZoom(fitZoom,
              wholePageZoom: math.min(widthFit, heightFit),
              multiplePagesZoom: multipleFit);
        });
        return SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: SizedBox(
                width: columns * pageW + (columns - 1) * pageGap + 48 >
                        constraints.maxWidth
                    ? columns * pageW + (columns - 1) * pageGap + 48
                    : constraints.maxWidth,
                height: constraints.maxHeight,
                child: ListView.builder(
                  controller: scroll,
                  padding: EdgeInsets.fromLTRB(
                      24, pageTopInset, 24, pageBottomInset),
                  itemCount: (n + columns - 1) ~/ columns +
                      (controller.showRuler && !controller.multiplePages
                          ? 1
                          : 0),
                  itemBuilder: (context, index) {
                    final showRuler =
                        controller.showRuler && !controller.multiplePages;
                    if (showRuler && index == 0) {
                      return Center(
                        child: WordRuler(page: env.page, zoom: controller.zoom),
                      );
                    }
                    final row = showRuler ? index - 1 : index;
                    return Padding(
                      padding: EdgeInsets.only(bottom: pageGap),
                      child: Center(
                        child: Row(mainAxisSize: MainAxisSize.min, children: [
                          for (var col = 0;
                              col < columns && row * columns + col < n;
                              col++) ...[
                            if (col > 0) SizedBox(width: pageGap),
                            _PaperPage(
                              envelope: env,
                              pageIndex: row * columns + col,
                              zoom: controller.zoom,
                              width: pageW,
                              height: pageH,
                              showGridlines: controller.showGridlines,
                              darkPage: controller.darkPage,
                              caret: controller.caret,
                              selection: controller.selection,
                              readOnly: readOnly,
                              onTapDown: (local) => controller.tapPage(
                                  row * columns + col, local),
                              onDrag: (a, b) => controller.dragPage(
                                  row * columns + col, a, b),
                            ),
                          ],
                        ]),
                      ),
                    );
                  },
                )));
      }),
    );
  }
}

class _PaperPage extends StatefulWidget {
  const _PaperPage({
    required this.envelope,
    required this.pageIndex,
    required this.zoom,
    required this.width,
    required this.height,
    required this.showGridlines,
    required this.darkPage,
    required this.caret,
    required this.selection,
    required this.onTapDown,
    required this.onDrag,
    required this.readOnly,
  });

  final RenderEnvelope envelope;
  final int pageIndex;
  final double zoom;
  final double width;
  final double height;
  final bool showGridlines;
  final bool darkPage;
  final WordCaret? caret;
  final WordDragSelection? selection;
  final ValueChanged<Offset> onTapDown;
  final void Function(Offset a, Offset b) onDrag;
  final bool readOnly;

  @override
  State<_PaperPage> createState() => _PaperPageState();
}

class _PaperPageState extends State<_PaperPage> {
  Offset? _dragStart;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: widget.darkPage ? const Color(0xFF1E1E1E) : Colors.white,
        boxShadow: [
          BoxShadow(
              color: Color(0x47000000), offset: Offset(0, 1), blurRadius: 3),
          BoxShadow(
              color: Color(0x1A000000), offset: Offset(0, 4), blurRadius: 16),
        ],
      ),
      child: Material(
          key: Key('word-paper-page-${widget.pageIndex + 1}'),
          color: widget.darkPage ? const Color(0xFF1E1E1E) : Colors.white,
          child: SizedBox(
            width: widget.width,
            height: widget.height,
            child: widget.readOnly
                ? RepaintBoundary(
                    child: CustomPaint(
                      size: Size(widget.width, widget.height),
                      painter: PagePainter(
                        envelope: widget.envelope,
                        pageIndex: widget.pageIndex,
                        zoom: widget.zoom,
                        showGridlines: widget.showGridlines,
                        darkPage: widget.darkPage,
                      ),
                    ),
                  )
                : GestureDetector(
                    onTapDown: (d) => widget.onTapDown(d.localPosition),
                    onPanStart: (d) {
                      _dragStart = d.localPosition;
                      widget.onTapDown(d.localPosition);
                    },
                    onPanUpdate: (d) {
                      final a = _dragStart;
                      if (a != null) widget.onDrag(a, d.localPosition);
                    },
                    onPanEnd: (_) => _dragStart = null,
                    child: Stack(
                      children: [
                        CustomPaint(
                          size: Size(widget.width, widget.height),
                          painter: PagePainter(
                            envelope: widget.envelope,
                            pageIndex: widget.pageIndex,
                            zoom: widget.zoom,
                            showGridlines: widget.showGridlines,
                            darkPage: widget.darkPage,
                          ),
                        ),
                        CustomPaint(
                          size: Size(widget.width, widget.height),
                          painter: _OverlayPainter(
                            zoom: widget.zoom,
                            pageIndex: widget.pageIndex,
                            caret: widget.caret,
                            selection: widget.selection,
                          ),
                        ),
                      ],
                    ),
                  ),
          )),
    );
  }
}

class _OverlayPainter extends CustomPainter {
  _OverlayPainter({
    required this.zoom,
    required this.pageIndex,
    required this.caret,
    required this.selection,
  });

  final double zoom;
  final int pageIndex;
  final WordCaret? caret;
  final WordDragSelection? selection;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();
    canvas.scale(zoom * (1.3333536 / 20.0));
    final sel = selection;
    if (sel != null && sel.pageIndex == pageIndex) {
      final b = sel.bounds;
      canvas.drawRect(
        Rect.fromLTRB(
          b.left.toDouble(),
          b.top.toDouble(),
          b.right.toDouble(),
          b.bottom.toDouble(),
        ),
        Paint()..color = const Color(0x552B579A),
      );
    }
    final cr = caret;
    if (cr != null && cr.pageIndex == pageIndex) {
      final h = cr.heightTwip.toDouble();
      canvas.drawRect(
        Rect.fromLTWH(cr.xTwip.toDouble(), cr.yTwip - h, 15, h),
        Paint()..color = const Color(0xFF185ABD),
      );
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant _OverlayPainter old) =>
      old.zoom != zoom ||
      old.caret != caret ||
      old.selection != selection ||
      old.pageIndex != pageIndex;
}

class _StatusBar extends StatelessWidget {
  const _StatusBar({required this.controller});
  final WordEditorController controller;

  @override
  Widget build(BuildContext context) {
    final c = controller;
    final pages = c.pageCount;
    return Container(
      height: 24,
      color: _kWordBlue,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      child: Row(
        children: [
          Text(
            pages == 0 ? 'Page —' : 'Page ${c.visiblePage} of $pages',
            style: const TextStyle(color: Colors.white, fontSize: 11),
          ),
          const SizedBox(width: 16),
          Text(
            '${(c.zoom * 100).round()}%',
            style: const TextStyle(color: Colors.white, fontSize: 11),
          ),
          const Spacer(),
          Flexible(
            child: Text(
              c.status,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              textAlign: TextAlign.right,
              style: const TextStyle(color: Colors.white70, fontSize: 11),
            ),
          ),
        ],
      ),
    );
  }
}
