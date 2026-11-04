import 'dart:async';
import 'dart:ui' as ui;

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'pdf_page_ranges.dart';
import 'pdf_document_status.dart';
import 'pdf_view_navigation.dart';
import 'pdf_shell_layout.dart';
import 'pdf_page_arrangement.dart';

/// Product configuration is data supplied by a verified PDF product package.
/// It never stores PDF document contents or implements an editing command.
final class PdfChromeDefinition {
  PdfChromeDefinition.fromJson(
    Map<String, dynamic> json, {
    Map<String, dynamic> iconData = const {},
    String? themeMode,
  }) : initialTab = _validatedInitialTab(json),
       sidebarContextMenu = json['sidebarContextMenu'] as String?,
       tabs = List<Map<String, dynamic>>.unmodifiable(
         (json['tabs'] as List).map(
           (value) => Map<String, dynamic>.from(value as Map),
         ),
       ),
       quickAccess = List<Map<String, dynamic>>.unmodifiable(
         (json['quickAccess'] as List).map(
           (value) => Map<String, dynamic>.from(value as Map),
         ),
       ),
       menus = Map<String, dynamic>.unmodifiable(
         json['menus'] as Map<String, dynamic>,
       ),
       presentations = Map<String, dynamic>.unmodifiable(
         json['presentations'] as Map<String, dynamic>? ?? {},
       ),
       colors = Map<String, dynamic>.unmodifiable(
         _selectedColors(json, themeMode),
       ),
       metrics = Map<String, dynamic>.unmodifiable(
         json['metrics'] as Map<String, dynamic>,
       ),
       menuPresentation = Map<String, dynamic>.unmodifiable(
         json['menuPresentation'] as Map<String, dynamic>? ?? {},
       ),
       sidebar = Map<String, dynamic>.unmodifiable(
         json['sidebar'] as Map<String, dynamic>? ?? {},
       ),
       pageViewport = PdfPageViewport.fromJson(
         json['pageViewport'] as Map<String, dynamic>?,
       ),
       pageArrangement = PdfPageArrangementPresentation.fromJson(
         json['pageArrangement'] as Map<String, dynamic>?,
       ),
       responsiveLayout = PdfShellLayoutPolicy.fromJson(
         json['responsiveLayout'] as Map<String, dynamic>?,
       ),
       aiDock = Map<String, dynamic>.unmodifiable(
         json['aiDock'] as Map<String, dynamic>? ?? {},
       ),
       statusBar = Map<String, dynamic>.unmodifiable(
         json['statusBar'] as Map<String, dynamic>? ?? {},
       ),
       ribbonState = Map<String, dynamic>.unmodifiable(
         json['ribbonState'] as Map<String, dynamic>? ?? {},
       ),
       palette = Map<String, dynamic>.unmodifiable(
         json['palette'] as Map<String, dynamic>? ?? {},
       ),
       dialogs = Map<String, dynamic>.unmodifiable(
         json['dialogs'] as Map<String, dynamic>? ?? {},
       ),
       searchBar = Map<String, dynamic>.unmodifiable(
         json['searchBar'] as Map<String, dynamic>? ?? {},
       ),
       keyboardShortcuts = Map<String, dynamic>.unmodifiable(
         json['keyboardShortcuts'] as Map<String, dynamic>? ?? {},
       ),
       keyboardNavigation = PdfKeyboardNavigation.fromJson(
         json['keyboardNavigation'] as Map<String, dynamic>?,
       ),
       iconRefs = Map<String, dynamic>.unmodifiable(
         iconData['refs'] as Map<String, dynamic>? ?? {},
       ),
       icons = Map<String, dynamic>.unmodifiable(
         iconData['icons'] as Map<String, dynamic>? ?? {},
       );

  final String initialTab;
  final String? sidebarContextMenu;
  final List<Map<String, dynamic>> tabs;
  final List<Map<String, dynamic>> quickAccess;
  final Map<String, dynamic> menus;
  final Map<String, dynamic> presentations;
  final Map<String, dynamic> colors;
  final Map<String, dynamic> metrics;
  final Map<String, dynamic> menuPresentation;
  final Map<String, dynamic> sidebar;
  final PdfPageViewport pageViewport;
  final PdfPageArrangementPresentation pageArrangement;
  final PdfShellLayoutPolicy responsiveLayout;
  final Map<String, dynamic> aiDock;
  final Map<String, dynamic> statusBar;
  final Map<String, dynamic> ribbonState;
  final Map<String, dynamic> palette;
  final Map<String, dynamic> dialogs;
  final Map<String, dynamic> searchBar;
  final Map<String, dynamic> keyboardShortcuts;
  final PdfKeyboardNavigation keyboardNavigation;
  final Map<String, dynamic> iconRefs;
  final Map<String, dynamic> icons;

  static String _validatedInitialTab(Map<String, dynamic> json) {
    if (json['profile'] != 'openmuse-pdf-chrome/1' ||
        json['initialTab'] is! String ||
        json['tabs'] is! List ||
        (json['tabs'] as List).isEmpty) {
      throw const FormatException('PDF chrome profile incompatible');
    }
    return json['initialTab'] as String;
  }

  static Map<String, dynamic> _selectedColors(
    Map<String, dynamic> json,
    String? themeMode,
  ) {
    final themes = json['themes'] as Map<String, dynamic>?;
    if (themes == null) return json['colors'] as Map<String, dynamic>;
    final selected = themeMode ?? themes['default'] as String;
    if (selected != 'light' && selected != 'dark') {
      throw FormatException('Unsupported PDF chrome theme: $selected');
    }
    return Map<String, dynamic>.from(themes[selected] as Map);
  }

  Color color(String key) => Color(
    int.parse((colors[key] as String).substring(1), radix: 16) | 0xff000000,
  );
  double metric(String key) => (metrics[key] as num).toDouble();

  String? iconSvg(String control) => icons[iconRefs[control]] as String?;
}

/// Signed presentation contract for the PDF page area, separate from page data.
final class PdfPageViewport {
  PdfPageViewport.fromJson(Map<String, dynamic>? json)
    : horizontalPadding =
          (json?['horizontalPadding'] as num?)?.toDouble() ?? 16,
      verticalPadding = (json?['verticalPadding'] as num?)?.toDouble() ?? 16,
      pageGap = (json?['pageGap'] as num?)?.toDouble() ?? 16,
      pageShadow = _shadow(json?['shadow'] as Map<String, dynamic>?);

  final double horizontalPadding;
  final double verticalPadding;
  final double pageGap;
  final BoxShadow pageShadow;

  static BoxShadow _shadow(Map<String, dynamic>? json) {
    if (json == null) {
      return const BoxShadow(
        color: Colors.black54,
        blurRadius: 4,
        spreadRadius: 2,
        offset: Offset(2, 2),
      );
    }
    final opaqueColor = Color(
      int.parse((json['color'] as String).substring(1), radix: 16) | 0xff000000,
    );
    return BoxShadow(
      color: opaqueColor.withValues(alpha: (json['opacity'] as num).toDouble()),
      blurRadius: (json['blurRadius'] as num).toDouble(),
      spreadRadius: (json['spreadRadius'] as num).toDouble(),
      offset: Offset(
        (json['offsetX'] as num).toDouble(),
        (json['offsetY'] as num).toDouble(),
      ),
    );
  }
}

typedef PdfChromeDispatch =
    void Function(String command, Map<String, dynamic> args);

/// Native Flutter replica of GenOffice PDF's product chrome. The document
/// widget and thumbnail raster callback both use the product PDFium session.
class PdfProductShell extends StatefulWidget {
  const PdfProductShell({
    super.key,
    required this.definition,
    required this.document,
    required this.pageNumber,
    required this.pageCount,
    required this.fileName,
    required this.zoom,
    required this.isCommandAvailable,
    required this.onCommand,
    required this.onGoToPage,
    this.onNavigateDocument,
    required this.onZoom,
    required this.renderThumbnail,
    this.pageArrangement = PdfPageArrangement.single,
    this.documentStatus = const PdfDocumentStatus(
      hasDocument: false,
      hasPendingChanges: false,
    ),
    this.searchCurrent = 0,
    this.searchTotal = 0,
    this.searchBusy = false,
    this.onSearchQuery,
    this.onSearchStep,
    this.onSearchClose,
    this.onViewportWidthChanged,
    this.assistantContent,
  });

  final PdfChromeDefinition definition;
  final Widget document;
  final PdfDocumentStatus documentStatus;
  final int pageNumber, pageCount;
  final String fileName;
  final double zoom;
  final bool Function(String command) isCommandAvailable;
  final PdfChromeDispatch onCommand;
  final ValueChanged<int> onGoToPage;
  final ValueChanged<PdfNavigationAction>? onNavigateDocument;
  final ValueChanged<double> onZoom;
  final ValueChanged<double>? onViewportWidthChanged;

  /// Optional AI plugin view; the PDF shell owns only the dock and its geometry.
  final Widget? assistantContent;
  final Future<ui.Image?> Function(int zeroBasedPage, int width)
  renderThumbnail;
  final PdfPageArrangement pageArrangement;
  final int searchCurrent, searchTotal;
  final bool searchBusy;
  final ValueChanged<String>? onSearchQuery;
  final ValueChanged<int>? onSearchStep;
  final VoidCallback? onSearchClose;

  @override
  State<PdfProductShell> createState() => _PdfProductShellState();
}

class _PdfProductShellState extends State<PdfProductShell> {
  late String _tab = widget.definition.initialTab;
  bool _ribbonCollapsed = false;
  String? _sidebar = 'thumbs';
  double? _sidebarWidth;
  bool _sidebarUserChanged = false;
  late bool _aiOpen =
      widget.definition.aiDock['initiallyOpen'] as bool? ?? false;
  double? _aiPreferredWidth;
  bool _aiOpenUserChanged = false;
  bool _aiWidthUserChanged = false;
  double _aiDragStartX = 0;
  double _aiDragStartWidth = 0;
  double _sidebarDragStartX = 0;
  double _sidebarDragStartWidth = 0;
  late int _thumbnailRasterWidth;
  final TextEditingController _pageInput = TextEditingController();
  final FocusNode _pageFocus = FocusNode();
  final ScrollController _ribbonScroll = ScrollController();
  final TextEditingController _searchInput = TextEditingController();
  final FocusNode _searchFocus = FocusNode();
  final FocusNode _shellFocus = FocusNode(debugLabel: 'PDF product shell');
  final Map<int, FocusNode> _thumbnailFocusNodes = {};
  bool _searchOpen = false;
  String _highlightColor = '#ffde59';
  String _drawColor = '#db382e';

  @override
  void initState() {
    super.initState();
    _pageInput.text = '${widget.pageNumber}';
    _thumbnailRasterWidth = _rasterWidth(
      widget.definition.metric('sidebarWidth'),
    );
    unawaited(_restoreSidebarWidth());
    unawaited(_restoreAiDock());
  }

  int _rasterWidth(double width) =>
      (width -
              ((widget.definition.sidebar['rasterInset'] as num?)?.toDouble() ??
                  24))
          .round()
          .clamp(1, 4096);

  double _clampSidebarWidth(double width, double viewportWidth) {
    final spec = widget.definition.sidebar;
    final minimum = (spec['minWidth'] as num?)?.toDouble() ?? 120;
    final maximum = (spec['maxWidth'] as num?)?.toDouble() ?? 320;
    final fraction = (spec['maxViewportFraction'] as num?)?.toDouble() ?? 0.4;
    return width.clamp(
      minimum,
      maximum.clamp(minimum, (viewportWidth * fraction).roundToDouble()),
    );
  }

  Future<void> _restoreSidebarWidth() async {
    final key = widget.definition.sidebar['preferenceKey'] as String?;
    if (key == null) return;
    try {
      final saved = (await SharedPreferences.getInstance()).getDouble(key);
      if (!mounted ||
          _sidebarUserChanged ||
          saved == null ||
          !saved.isFinite ||
          saved <= 0) {
        return;
      }
      final restored = _clampSidebarWidth(
        saved,
        MediaQuery.sizeOf(context).width,
      );
      setState(() {
        _sidebarWidth = restored;
        _thumbnailRasterWidth = _rasterWidth(restored);
      });
    } catch (_) {
      // A host without a preference adapter still has the signed default width.
    }
  }

  Future<void> _finishSidebarResize(double viewportWidth) async {
    final width = _clampSidebarWidth(
      _sidebarWidth ?? widget.definition.metric('sidebarWidth'),
      viewportWidth,
    );
    setState(() {
      _sidebarWidth = width;
      _thumbnailRasterWidth = _rasterWidth(width);
    });
    final key = widget.definition.sidebar['preferenceKey'] as String?;
    if (key == null) return;
    try {
      await (await SharedPreferences.getInstance()).setDouble(
        key,
        width.roundToDouble(),
      );
    } catch (_) {
      // Persisting a visual preference must not prevent document editing.
    }
  }

  double _clampAiWidth(double preferred, double viewportWidth) {
    final spec = widget.definition.aiDock;
    final minimum = (spec['minWidth'] as num?)?.toDouble() ?? 280;
    final maximum = (spec['maxWidth'] as num?)?.toDouble() ?? 720;
    final fraction = (spec['maxViewportFraction'] as num?)?.toDouble() ?? 0.6;
    return preferred.clamp(
      minimum,
      maximum.clamp(minimum, (viewportWidth * fraction).roundToDouble()),
    );
  }

  Future<void> _restoreAiDock() async {
    final spec = widget.definition.aiDock;
    try {
      final preferences = await SharedPreferences.getInstance();
      if (!mounted) return;
      final savedWidth = preferences.getDouble(
        spec['widthPreferenceKey'] as String,
      );
      final savedOpen = preferences.getBool(
        spec['openPreferenceKey'] as String,
      );
      setState(() {
        if (!_aiWidthUserChanged &&
            savedWidth != null &&
            savedWidth.isFinite &&
            savedWidth > 0) {
          // Keep the preference independent from a temporarily narrow viewport.
          _aiPreferredWidth = savedWidth.clamp(
            (spec['minWidth'] as num).toDouble(),
            (spec['maxWidth'] as num).toDouble(),
          );
        }
        if (!_aiOpenUserChanged && savedOpen != null) _aiOpen = savedOpen;
      });
    } catch (_) {
      // Missing host preference adapter leaves the signed initial state intact.
    }
  }

  void _setAiOpen(bool open) {
    _aiOpenUserChanged = true;
    setState(() => _aiOpen = open);
    final key = widget.definition.aiDock['openPreferenceKey'] as String?;
    if (key == null) return;
    unawaited(() async {
      try {
        await (await SharedPreferences.getInstance()).setBool(key, open);
      } catch (_) {}
    }());
  }

  void _finishAiResize() {
    final key = widget.definition.aiDock['widthPreferenceKey'] as String?;
    final width = _aiPreferredWidth;
    if (key == null || width == null) return;
    unawaited(() async {
      try {
        await (await SharedPreferences.getInstance()).setDouble(
          key,
          width.roundToDouble(),
        );
      } catch (_) {}
    }());
  }

  void _toggleAiDock() {
    final width = MediaQuery.sizeOf(context).width;
    final columns = _resolveColumns(width);
    if (columns.mobile ||
        width - widget.definition.responsiveLayout.minDocumentWidth <
            (widget.definition.aiDock['minWidth'] as num).toDouble()) {
      unawaited(
        showModalBottomSheet<void>(
          context: context,
          isScrollControlled: true,
          backgroundColor: surface,
          builder: (context) => SizedBox(
            height: MediaQuery.sizeOf(context).height * 0.72,
            child: _aiPanel(mobile: true),
          ),
        ),
      );
    } else {
      _setAiOpen(!columns.aiPanel);
    }
  }

  PdfShellColumns _resolveColumns(double width) {
    final aiSpec = widget.definition.aiDock;
    if (width < widget.definition.responsiveLayout.mobileBelow) {
      return widget.definition.responsiveLayout.resolve(
        width: width,
        aiPreferredWidth: (aiSpec['defaultWidth'] as num).toDouble(),
        aiMinimumWidth: (aiSpec['minWidth'] as num).toDouble(),
        aiRailWidth: widget.definition.metric('aiRailWidth'),
        sidebarWidth: 0,
        aiRequested: _aiOpen,
        aiExplicit: _aiOpenUserChanged,
        sidebarRequested: false,
      );
    }
    final sidebarSpec = widget.definition.sidebar;
    final handleWidth = (sidebarSpec['resizerWidth'] as num?)?.toDouble() ?? 6;
    final overlap = (sidebarSpec['resizerOverlap'] as num?)?.toDouble() ?? 3;
    final sidebarWidth = _clampSidebarWidth(
      _sidebarWidth ?? widget.definition.metric('sidebarWidth'),
      width,
    );
    return widget.definition.responsiveLayout.resolve(
      width: width,
      aiPreferredWidth: _clampAiWidth(
        _aiPreferredWidth ??
            (aiSpec['defaultWidth'] as num?)?.toDouble() ??
            360,
        width,
      ),
      aiMinimumWidth: (aiSpec['minWidth'] as num?)?.toDouble() ?? 280,
      aiRailWidth: widget.definition.metric('aiRailWidth'),
      sidebarWidth: sidebarWidth + handleWidth - overlap,
      aiRequested: _aiOpen,
      aiExplicit: _aiOpenUserChanged,
      sidebarRequested: _sidebar != null && widget.pageCount > 0,
    );
  }

  @override
  void didUpdateWidget(covariant PdfProductShell oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!_pageFocus.hasFocus &&
        (oldWidget.pageNumber != widget.pageNumber ||
            _pageInput.text.isEmpty)) {
      _pageInput.text = '${widget.pageNumber}';
    }
    if (!widget.definition.tabs.any((tab) => tab['id'] == _tab)) {
      _tab = widget.definition.initialTab;
    }
  }

  @override
  void dispose() {
    _pageInput.dispose();
    _pageFocus.dispose();
    _ribbonScroll.dispose();
    _searchInput.dispose();
    _searchFocus.dispose();
    _shellFocus.dispose();
    for (final node in _thumbnailFocusNodes.values) {
      node.dispose();
    }
    super.dispose();
  }

  Color get chrome => widget.definition.color('chrome');
  Color get surface => widget.definition.color('surface');
  Color get muted => widget.definition.color('muted');
  Color get ink => widget.definition.color('text');
  Color get border => widget.definition.color('border');
  Color get accent => widget.definition.color('accent');

  Map<String, dynamic> _menuSpec(String kind) =>
      widget.definition.menuPresentation[kind] as Map<String, dynamic>;

  MenuStyle _menuStyle(String kind) {
    final spec = _menuSpec(kind);
    return MenuStyle(
      visualDensity: const VisualDensity(horizontal: 0, vertical: 0),
      backgroundColor: WidgetStatePropertyAll(surface),
      surfaceTintColor: const WidgetStatePropertyAll(Colors.transparent),
      shadowColor: WidgetStatePropertyAll(
        Colors.black.withValues(
          alpha: (spec['shadowOpacity'] as num).toDouble(),
        ),
      ),
      elevation: WidgetStatePropertyAll((spec['elevation'] as num).toDouble()),
      minimumSize: WidgetStatePropertyAll(
        Size((spec['minWidth'] as num).toDouble(), 0),
      ),
      side: WidgetStatePropertyAll(
        BorderSide(color: widget.definition.color('borderStrong')),
      ),
      shape: WidgetStatePropertyAll(
        RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(
            (spec['radius'] as num).toDouble(),
          ),
        ),
      ),
      padding: WidgetStatePropertyAll(
        EdgeInsets.all((spec['padding'] as num).toDouble()),
      ),
    );
  }

  ButtonStyle _menuButtonStyle(String kind) => ButtonStyle(
    visualDensity: const VisualDensity(horizontal: 0, vertical: 0),
    tapTargetSize: MaterialTapTargetSize.shrinkWrap,
    foregroundColor: WidgetStateProperty.resolveWith(
      (states) => states.contains(WidgetState.disabled)
          ? muted.withValues(alpha: 0.4)
          : ink,
    ),
    backgroundColor: WidgetStateProperty.resolveWith(
      (states) => states.contains(WidgetState.hovered)
          ? widget.definition.color('hover')
          : Colors.transparent,
    ),
    minimumSize: WidgetStatePropertyAll(
      Size(0, (_menuSpec(kind)['itemHeight'] as num).toDouble()),
    ),
    maximumSize: WidgetStatePropertyAll(
      Size(double.infinity, (_menuSpec(kind)['itemHeight'] as num).toDouble()),
    ),
    padding: WidgetStatePropertyAll(
      EdgeInsets.symmetric(
        horizontal: (_menuSpec(kind)['itemHorizontalPadding'] as num)
            .toDouble(),
      ),
    ),
    shape: WidgetStatePropertyAll(
      RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
    ),
  );

  Widget _menuLabel(String value, String kind) => Text(
    value,
    style: TextStyle(
      fontSize: (_menuSpec(kind)['fontSize'] as num).toDouble(),
      fontFamily: Theme.of(context).textTheme.bodyMedium?.fontFamily,
    ),
  );

  Widget _glyph(
    String control,
    String fallback,
    double size,
    Color color, {
    String? iconAsset,
  }) {
    final svg = iconAsset == null
        ? widget.definition.iconSvg(control)
        : widget.definition.icons[iconAsset] as String?;
    if (svg == null) return Icon(_icon(fallback), size: size, color: color);
    return SvgPicture.string(
      svg,
      width: size,
      height: size,
      colorFilter: ColorFilter.mode(color, BlendMode.srcIn),
    );
  }

  bool _enabled(String? command) =>
      command != null && widget.isCommandAvailable(command);

  void _dispatch(String? command, [Map<String, dynamic> args = const {}]) {
    if (command == 'pdf.search.open' && _enabled(command)) {
      setState(() => _searchOpen = true);
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _searchFocus.requestFocus();
      });
      return;
    }
    if (command == 'pdf.view.thumbs' && _enabled(command)) {
      setState(() => _sidebar = _sidebar == 'thumbs' ? null : 'thumbs');
      return;
    }
    if (command == 'pdf.page.extract' &&
        !args.containsKey('pageNumber') &&
        _enabled(command)) {
      unawaited(_showExtractDialog());
      return;
    }
    if (command == 'pdf.page.split' && _enabled(command)) {
      unawaited(_showSplitDialog());
      return;
    }
    if (command != null && _enabled(command)) widget.onCommand(command, args);
  }

  Future<void> _showExtractDialog() async {
    final pages = await _showPageInputDialog<List<int>>(
      resourceId: 'extractRange',
      dialogKey: 'pdf-extract-dialog',
      inputKey: 'pdf-extract-range-input',
      initialValue: '${widget.pageNumber}',
      parse: (value, count) => parsePdfPageRanges(value, count),
    );
    if (pages != null && mounted && _enabled('pdf.page.extract')) {
      widget.onCommand('pdf.page.extract', {'pageRange': pages});
    }
  }

  Future<void> _showSplitDialog() async {
    final pagesPerFile = await _showPageInputDialog<int>(
      resourceId: 'splitPdf',
      dialogKey: 'pdf-split-dialog',
      inputKey: 'pdf-split-count-input',
      initialValue: '1',
      parse: (value, count) {
        final number = int.tryParse(value.trim());
        return number != null && number >= 1 && number < count ? number : null;
      },
    );
    if (pagesPerFile != null && mounted && _enabled('pdf.page.split')) {
      widget.onCommand('pdf.page.split', {'pagesPerFile': pagesPerFile});
    }
  }

  Future<T?> _showPageInputDialog<T>({
    required String resourceId,
    required String dialogKey,
    required String inputKey,
    required String initialValue,
    required T? Function(String value, int pageCount) parse,
  }) async {
    final spec = Map<String, dynamic>.from(
      widget.definition.dialogs[resourceId] as Map? ?? {},
    );
    var entered = initialValue;
    var invalid = false;
    return showDialog<T>(
      context: context,
      barrierColor: Colors.black54,
      builder: (dialogContext) => StatefulBuilder(
        builder: (dialogContext, redraw) => Dialog(
          key: Key(dialogKey),
          alignment: Alignment.topCenter,
          insetPadding: const EdgeInsets.only(top: 96, left: 16, right: 16),
          backgroundColor: surface,
          shape: RoundedRectangleBorder(
            side: BorderSide(color: border),
            borderRadius: BorderRadius.circular(16),
          ),
          child: ConstrainedBox(
            constraints: const BoxConstraints(minWidth: 300, maxWidth: 360),
            child: Padding(
              padding: const EdgeInsets.all(14),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    spec['title'] as String? ?? '',
                    style: TextStyle(
                      color: ink,
                      fontSize: 13,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 10),
                  SizedBox(
                    height: 28,
                    child: TextFormField(
                      key: Key(inputKey),
                      initialValue: entered,
                      autofocus: true,
                      keyboardType: resourceId == 'splitPdf'
                          ? TextInputType.number
                          : TextInputType.text,
                      style: TextStyle(color: ink, fontSize: 13),
                      decoration: InputDecoration(
                        hintText: (spec['hint'] as String? ?? '').replaceAll(
                          '{total}',
                          '${widget.pageCount}',
                        ),
                        hintStyle: TextStyle(color: muted, fontSize: 12),
                        contentPadding: const EdgeInsets.symmetric(
                          horizontal: 8,
                        ),
                        isDense: true,
                        enabledBorder: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(4),
                          borderSide: BorderSide(
                            color: invalid ? const Color(0xffd83535) : border,
                          ),
                        ),
                        focusedBorder: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(4),
                          borderSide: BorderSide(
                            color: invalid ? const Color(0xffd83535) : accent,
                          ),
                        ),
                      ),
                      onChanged: (value) => redraw(() {
                        entered = value;
                        invalid = false;
                      }),
                      onFieldSubmitted: (_) {
                        final parsed = parse(entered, widget.pageCount);
                        if (parsed == null) {
                          redraw(() => invalid = true);
                        } else {
                          Navigator.pop(dialogContext, parsed);
                        }
                      },
                    ),
                  ),
                  const SizedBox(height: 10),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      _dialogButton(
                        spec['cancel'] as String? ?? '取消',
                        () => Navigator.pop(dialogContext),
                      ),
                      const SizedBox(width: 8),
                      _dialogButton(spec['confirm'] as String? ?? '确定', () {
                        final parsed = parse(entered, widget.pageCount);
                        if (parsed == null) {
                          redraw(() => invalid = true);
                        } else {
                          Navigator.pop(dialogContext, parsed);
                        }
                      }, primary: true),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _dialogButton(
    String label,
    VoidCallback onPressed, {
    bool primary = false,
  }) => SizedBox(
    height: 32,
    child: OutlinedButton(
      onPressed: onPressed,
      style: OutlinedButton.styleFrom(
        foregroundColor: primary ? Colors.white : ink,
        backgroundColor: primary ? const Color(0xff232425) : chrome,
        side: BorderSide(color: border),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        padding: const EdgeInsets.symmetric(horizontal: 14),
        textStyle: TextStyle(
          fontSize: 13,
          fontWeight: FontWeight.w500,
          fontFamily: Theme.of(context).textTheme.bodyMedium?.fontFamily,
        ),
      ),
      child: Text(label),
    ),
  );

  IconData _icon(String name) => switch (name) {
    'save' => Icons.save_outlined,
    'undo' => Icons.undo,
    'redo' => Icons.redo,
    'auto_awesome' => Icons.auto_awesome,
    'summarize' => Icons.summarize_outlined,
    'format_list_bulleted' => Icons.format_list_bulleted,
    'border_color' => Icons.border_color_outlined,
    'format_underlined' => Icons.format_underlined,
    'strikethrough_s' => Icons.strikethrough_s,
    'edit_note' => Icons.edit_note,
    'text_fields' => Icons.text_fields,
    'sync' => Icons.sync,
    'find_in_page' => Icons.find_in_page_outlined,
    'zoom_out' => Icons.zoom_out,
    'zoom_in' => Icons.zoom_in,
    'fit_screen' => Icons.fit_screen,
    'crop_free' => Icons.crop_free,
    'search' => Icons.search,
    'print' => Icons.print_outlined,
    'image' || 'image_outlined' => Icons.image_outlined,
    'info_outline' => Icons.info_outline,
    'gesture' => Icons.gesture,
    'crop_square' => Icons.crop_square,
    'circle_outlined' => Icons.circle_outlined,
    'arrow_forward' => Icons.arrow_forward,
    'sticky_note_2_outlined' => Icons.sticky_note_2_outlined,
    'block' => Icons.block,
    'draw' => Icons.draw_outlined,
    'verified_user_outlined' => Icons.verified_user_outlined,
    'palette_outlined' => Icons.palette_outlined,
    'add_photo_alternate_outlined' => Icons.add_photo_alternate_outlined,
    'branding_watermark_outlined' => Icons.branding_watermark_outlined,
    'rotate_left' => Icons.rotate_left,
    'rotate_right' => Icons.rotate_right,
    'rotate_90_degrees_ccw_outlined' => Icons.rotate_90_degrees_ccw_outlined,
    'delete_outline' => Icons.delete_outline,
    'file_copy_outlined' => Icons.file_copy_outlined,
    'note_add_outlined' => Icons.note_add_outlined,
    'post_add' => Icons.post_add,
    'swap_horiz' => Icons.swap_horiz,
    'crop' => Icons.crop,
    'aspect_ratio' => Icons.aspect_ratio,
    'sort' => Icons.sort,
    'call_split' => Icons.call_split,
    'merge_type' => Icons.merge_type,
    'view_week_outlined' => Icons.view_week_outlined,
    'splitscreen_outlined' => Icons.splitscreen_outlined,
    'view_sidebar_outlined' => Icons.view_sidebar_outlined,
    'list_alt_outlined' => Icons.list_alt_outlined,
    'article_outlined' => Icons.article_outlined,
    'menu_book_outlined' => Icons.menu_book_outlined,
    'dark_mode_outlined' => Icons.dark_mode_outlined,
    'check_box_outlined' => Icons.check_box_outlined,
    'close' => Icons.close,
    'arrow_back' => Icons.arrow_back,
    'task_alt' => Icons.task_alt,
    'event_outlined' => Icons.event_outlined,
    'radio_button_checked' => Icons.radio_button_checked,
    'arrow_drop_down_circle_outlined' => Icons.arrow_drop_down_circle_outlined,
    'push_pin_outlined' => Icons.push_pin_outlined,
    'visibility_outlined' => Icons.visibility_outlined,
    'auto_fix_high_outlined' => Icons.auto_fix_high_outlined,
    'align_horizontal_left' => Icons.align_horizontal_left,
    _ => Icons.extension_outlined,
  };

  /// The signed product binds familiar keys to commands; the shell only
  /// interprets those bindings and leaves document operations to the session.
  KeyEventResult _onKeyEvent(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent || widget.pageCount == 0) {
      return KeyEventResult.ignored;
    }
    final keyboard = HardwareKeyboard.instance;
    final primary = keyboard.isMetaPressed || keyboard.isControlPressed;
    final shift = keyboard.isShiftPressed;
    final focusContext = FocusManager.instance.primaryFocus?.context;
    final editingText =
        _pageFocus.hasFocus ||
        _searchFocus.hasFocus ||
        focusContext?.findAncestorStateOfType<EditableTextState>() != null;
    if (primary) {
      final key = switch (event.logicalKey) {
        LogicalKeyboardKey.keyS => 's',
        LogicalKeyboardKey.keyZ => 'z',
        LogicalKeyboardKey.keyF => 'f',
        LogicalKeyboardKey.equal => '=',
        LogicalKeyboardKey.minus => '-',
        LogicalKeyboardKey.digit0 => '0',
        _ => null,
      };
      if (key == null || (editingText && key == 'z')) {
        return KeyEventResult.ignored;
      }
      final binding = 'primary+${shift && key == 'z' ? 'shift+' : ''}$key';
      final command = widget.definition.keyboardShortcuts[binding] as String?;
      if (!_enabled(command)) return KeyEventResult.ignored;
      _dispatch(command);
      return KeyEventResult.handled;
    }
    if (shift || keyboard.isAltPressed || editingText) {
      return KeyEventResult.ignored;
    }
    final key = switch (event.logicalKey) {
      LogicalKeyboardKey.pageDown => 'PageDown',
      LogicalKeyboardKey.space => 'Space',
      LogicalKeyboardKey.pageUp => 'PageUp',
      LogicalKeyboardKey.home => 'Home',
      LogicalKeyboardKey.end => 'End',
      LogicalKeyboardKey.arrowDown => 'ArrowDown',
      LogicalKeyboardKey.arrowUp => 'ArrowUp',
      LogicalKeyboardKey.arrowRight => 'ArrowRight',
      LogicalKeyboardKey.arrowLeft => 'ArrowLeft',
      _ => null,
    };
    if (key == null) return KeyEventResult.ignored;
    final inThumbnails = _thumbnailFocusNodes.values.any(
      (node) => node.hasFocus,
    );
    final action = widget.definition.keyboardNavigation.actionFor(
      key,
      inThumbnails: inThumbnails,
    );
    if (action == null) return KeyEventResult.ignored;
    if (action == PdfNavigationAction.pageNext ||
        action == PdfNavigationAction.pagePrevious) {
      final delta = action == PdfNavigationAction.pageNext ? 1 : -1;
      final target = widget.pageArrangement.stepPage(
        widget.pageNumber,
        delta,
        widget.pageCount,
      );
      if (target != widget.pageNumber) {
        widget.onGoToPage(target);
        if (inThumbnails) {
          final targetNode = _thumbnailFocusNodes[target - 1];
          if (targetNode?.context != null) targetNode!.requestFocus();
        }
      }
    } else {
      if (widget.onNavigateDocument == null) return KeyEventResult.ignored;
      widget.onNavigateDocument!(action);
    }
    return KeyEventResult.handled;
  }

  @override
  Widget build(BuildContext context) => Focus(
    key: const Key('pdf-shell-keyboard'),
    focusNode: _shellFocus,
    autofocus: true,
    onKeyEvent: _onKeyEvent,
    child: LayoutBuilder(
      builder: (context, constraints) {
        final width = constraints.maxWidth;
        final columns = _resolveColumns(width);
        final mobile = columns.mobile;
        final compact = columns.compact;
        final tabs = widget.definition.tabs;
        final active = tabs.firstWhere(
          (tab) => tab['id'] == _tab,
          orElse: () => tabs.first,
        );
        final showSidebar = columns.sidebar;
        final sidebarWidth = _clampSidebarWidth(
          _sidebarWidth ?? widget.definition.metric('sidebarWidth'),
          width,
        );
        return ColoredBox(
          color: chrome,
          child: Column(
            children: [
              _tabRow(tabs, mobile),
              if (!_ribbonCollapsed) _ribbon(active, compact, mobile),
              Expanded(
                child: Row(
                  children: [
                    if (!mobile)
                      _aiDock(width, columns.aiPanel, columns.aiWidth),
                    Expanded(
                      child: Column(
                        children: [
                          Expanded(
                            child: Row(
                              children: [
                                if (showSidebar)
                                  _sidebarWithResizer(sidebarWidth, width),
                                Expanded(
                                  child: ColoredBox(
                                    color: widget.definition.color('canvas'),
                                    child: LayoutBuilder(
                                      builder: (context, viewport) {
                                        widget.onViewportWidthChanged?.call(
                                          viewport.maxWidth,
                                        );
                                        return Stack(
                                          children: [
                                            Positioned.fill(
                                              child: widget.document,
                                            ),
                                            if (_searchOpen)
                                              Positioned(
                                                top: 8,
                                                right: mobile ? 8 : 20,
                                                child: _searchBar(
                                                  viewport.maxWidth,
                                                  mobile,
                                                ),
                                              ),
                                          ],
                                        );
                                      },
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          ),
                          _statusBar(mobile),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    ),
  );

  Widget _tabRow(List<Map<String, dynamic>> tabs, bool mobile) => Container(
    height: mobile ? 46 : widget.definition.metric('tabHeight'),
    padding: const EdgeInsets.only(left: 10),
    color: chrome,
    child: Row(
      children: [
        for (final item in widget.definition.quickAccess)
          SizedBox(
            width: mobile ? 42 : 30,
            child: IconButton(
              tooltip: item['label'] as String,
              padding: EdgeInsets.zero,
              iconSize: mobile ? 20 : 18,
              color: _quickAccessEnabled(item) ? ink : muted,
              onPressed: _quickAccessEnabled(item)
                  ? () => _dispatch(item['command'] as String?)
                  : null,
              icon: _glyph(
                'quick.${item['id']}',
                item['icon'] as String,
                mobile ? 20 : 18,
                _quickAccessEnabled(item) ? ink : muted,
              ),
            ),
          ),
        Container(
          width: 1,
          height: 17,
          color: border,
          margin: const EdgeInsets.symmetric(horizontal: 8),
        ),
        Expanded(
          child: ListView(
            scrollDirection: Axis.horizontal,
            children: [for (final tab in tabs) _tabButton(tab, mobile)],
          ),
        ),
        if (_documentStateLabel() case final label?)
          Padding(
            padding: const EdgeInsets.only(left: 4, right: 10),
            child: Tooltip(
              message: widget.documentStatus.copySaveError ?? label,
              child: Text(
                label,
                key: const Key('pdf-ribbon-document-state'),
                maxLines: 1,
                style: TextStyle(
                  color: _documentStateColor(),
                  fontSize:
                      (widget.definition.ribbonState['fontSize'] as num?)
                          ?.toDouble() ??
                      12,
                ),
              ),
            ),
          ),
      ],
    ),
  );

  bool _quickAccessEnabled(Map<String, dynamic> item) =>
      _enabled(item['command'] as String?) &&
      !(item['id'] == 'save' &&
          widget.documentStatus.copySavePhase == PdfCopySavePhase.saving);

  String? _documentStateLabel() {
    final status = widget.documentStatus;
    final labels = widget.definition.ribbonState;
    if (!status.hasDocument) return null;
    if (status.isReadOnly) return labels['readOnly'] as String?;
    return switch (status.copySavePhase) {
      PdfCopySavePhase.saving => labels['savingCopy'] as String?,
      PdfCopySavePhase.saved => labels['copySaved'] as String?,
      PdfCopySavePhase.failed => labels['copyFailed'] as String?,
      PdfCopySavePhase.idle =>
        status.hasPendingChanges ? labels['unsaved'] as String? : null,
    };
  }

  Color _documentStateColor() => switch (widget.documentStatus.copySavePhase) {
    PdfCopySavePhase.saved => widget.definition.color('saveOk'),
    PdfCopySavePhase.failed => widget.definition.color('saveError'),
    _ => muted,
  };

  Widget _tabButton(Map<String, dynamic> tab, bool mobile) {
    final label = tab['label'] as String;
    final selected = _tab == tab['id'];
    final active = selected && !_ribbonCollapsed;
    final style = TextStyle(
      color: active ? accent : ink,
      fontSize: mobile ? 14 : 13,
      fontWeight: FontWeight.w600,
    );
    final measure = TextPainter(
      text: TextSpan(text: label, style: style),
      textDirection: Directionality.of(context),
    )..layout();
    final underlineWidth = measure.width;
    measure.dispose();
    return InkWell(
      key: Key('pdf-tab-${tab['id']}'),
      onTap: () => setState(() {
        if (_ribbonCollapsed) {
          _tab = tab['id'] as String;
          _ribbonCollapsed = false;
        } else if (selected) {
          _ribbonCollapsed = true;
        } else {
          _tab = tab['id'] as String;
        }
      }),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 11),
        child: Column(
          children: [
            Expanded(
              child: Center(child: Text(label, style: style)),
            ),
            Container(
              width: underlineWidth,
              height: 3,
              decoration: BoxDecoration(
                color: active ? accent : Colors.transparent,
                borderRadius: BorderRadius.circular(1),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _ribbon(Map<String, dynamic> active, bool compact, bool mobile) =>
      Container(
        height: mobile
            ? 72
            : compact
            ? widget.definition.metric('compactRibbonHeight')
            : widget.definition.metric('ribbonHeight'),
        decoration: BoxDecoration(
          color: chrome,
          border: Border(bottom: BorderSide(color: border)),
        ),
        child: Scrollbar(
          controller: _ribbonScroll,
          thumbVisibility: !mobile,
          interactive: !mobile,
          thickness: 5,
          child: Listener(
            onPointerSignal: (event) {
              if (event is! PointerScrollEvent || !_ribbonScroll.hasClients) {
                return;
              }
              final position = _ribbonScroll.position;
              final delta = event.scrollDelta.dy != 0
                  ? event.scrollDelta.dy
                  : event.scrollDelta.dx;
              _ribbonScroll.jumpTo(
                (_ribbonScroll.offset + delta).clamp(
                  position.minScrollExtent,
                  position.maxScrollExtent,
                ),
              );
            },
            child: ListView(
              key: ValueKey('pdf-ribbon-$_tab'),
              controller: _ribbonScroll,
              scrollDirection: Axis.horizontal,
              padding: const EdgeInsets.fromLTRB(8, 5, 8, 6),
              children: [
                for (final group in active['groups'] as List)
                  Row(
                    children: [
                      if (group['layout'] == 'pageZoom')
                        _pageZoomGroup(
                          group['items'] as List,
                          mobile,
                          compact && !mobile,
                        )
                      else
                        for (final raw in group['items'] as List)
                          _ribbonItem(
                            (raw as List).cast<dynamic>(),
                            mobile,
                            compact && !mobile,
                          ),
                      Container(
                        width: 1,
                        height: compact && !mobile ? 26 : 46,
                        color: border,
                        margin: const EdgeInsets.symmetric(horizontal: 7),
                      ),
                    ],
                  ),
              ],
            ),
          ),
        ),
      );

  void _commitPageInput() {
    final page = int.tryParse(_pageInput.text.trim());
    if (page != null && page >= 1 && page <= widget.pageCount) {
      widget.onGoToPage(page);
    } else {
      _pageInput.text = '${widget.pageNumber}';
    }
    _pageFocus.unfocus();
  }

  Widget _pageZoomGroup(List<dynamic> items, bool mobile, bool compact) {
    final fitItems = items.where(
      (raw) => (raw as List).first == 'fitWidth' || raw.first == 'fitPage',
    );
    final pageRow = Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        SizedBox(
          width: 40,
          height: 24,
          child: TextField(
            key: const Key('pdf-page-input'),
            controller: _pageInput,
            focusNode: _pageFocus,
            enabled: widget.pageCount > 0,
            textAlign: TextAlign.center,
            keyboardType: TextInputType.number,
            style: TextStyle(fontSize: 12, color: ink),
            decoration: InputDecoration(
              isDense: true,
              contentPadding: EdgeInsets.zero,
              border: OutlineInputBorder(borderSide: BorderSide(color: border)),
            ),
            onSubmitted: (_) => _commitPageInput(),
            onTapOutside: (_) => _commitPageInput(),
          ),
        ),
        const SizedBox(width: 6),
        Text(
          '共 ${widget.pageCount} 页',
          style: TextStyle(fontSize: 11, color: muted),
        ),
      ],
    );
    final zoomRow = Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _zoomIcon('缩小', Icons.remove, 'pdf.view.zoomOut'),
        SizedBox(
          width: 42,
          child: Text(
            '${(widget.zoom * 100).round()}%',
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 11, color: ink),
          ),
        ),
        _zoomIcon('放大', Icons.add, 'pdf.view.zoomIn'),
      ],
    );
    return Row(
      children: [
        SizedBox(
          width: compact ? 214 : 126,
          child: compact
              ? Row(children: [pageRow, const SizedBox(width: 6), zoomRow])
              : Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [pageRow, const SizedBox(height: 4), zoomRow],
                ),
        ),
        for (final raw in fitItems)
          _ribbonItem((raw as List).cast<dynamic>(), mobile, compact),
      ],
    );
  }

  Widget _zoomIcon(String label, IconData icon, String command) => Tooltip(
    message: label,
    child: InkWell(
      onTap: _enabled(command) ? () => _dispatch(command) : null,
      child: SizedBox(
        width: 25,
        height: 22,
        child: Icon(icon, size: 16, color: _enabled(command) ? ink : muted),
      ),
    ),
  );

  Color _hexColor(String value) =>
      Color(int.parse(value.substring(1), radix: 16) | 0xff000000);

  void _choosePaletteColor(String purpose, String value) {
    setState(() {
      if (purpose == 'highlight') {
        _highlightColor = value;
      } else {
        _drawColor = value;
      }
    });
  }

  Future<void> _morePaletteColor(String purpose) async {
    var entered = purpose == 'highlight' ? _highlightColor : _drawColor;
    final selected = await showDialog<String>(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, redraw) {
          final valid = RegExp(r'^#[0-9a-fA-F]{6}$').hasMatch(entered);
          return AlertDialog(
            title: const Text('更多颜色'),
            content: TextFormField(
              key: Key('pdf-palette-$purpose-custom-input'),
              initialValue: entered,
              autofocus: true,
              maxLength: 7,
              decoration: const InputDecoration(labelText: '十六进制颜色（#RRGGBB）'),
              onChanged: (value) => redraw(() => entered = value),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('取消'),
              ),
              TextButton(
                onPressed: valid
                    ? () => Navigator.pop(context, entered.toLowerCase())
                    : null,
                child: const Text('确定'),
              ),
            ],
          );
        },
      ),
    );
    if (selected != null && mounted) _choosePaletteColor(purpose, selected);
  }

  Widget _palettePanel(String purpose, MenuController controller) {
    final palette = widget.definition.palette;
    final theme = (palette['theme'] as List).cast<String>();
    final shades = (palette['shades'] as List)
        .map((row) => (row as List).cast<String>())
        .toList();
    final standard = (palette['standard'] as List).cast<String>();
    final labels = Map<String, dynamic>.from(palette['labels'] as Map);
    final current = purpose == 'highlight' ? _highlightColor : _drawColor;
    var index = 0;
    Widget row(List<String> values) => Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (var column = 0; column < values.length; column++) ...[
          if (column > 0) const SizedBox(width: 4),
          _paletteSwatch(
            purpose,
            index++,
            values[column],
            values[column] == current,
            () {
              controller.close();
              _choosePaletteColor(purpose, values[column]);
            },
          ),
        ],
      ],
    );
    Widget section(String title) => Padding(
      padding: const EdgeInsets.only(top: 10, bottom: 6),
      child: Align(
        alignment: Alignment.centerLeft,
        child: Text(title, style: TextStyle(color: ink, fontSize: 12.5)),
      ),
    );
    return SizedBox(
      key: Key('pdf-palette-$purpose'),
      width: 252,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(8, 2, 8, 4),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            section(labels['theme'] as String),
            row(theme),
            const SizedBox(height: 8),
            for (final shade in shades) row(shade),
            section(labels['standard'] as String),
            row(standard),
            const Divider(height: 12),
            InkWell(
              key: Key('pdf-palette-$purpose-more'),
              onTap: () {
                controller.close();
                _morePaletteColor(purpose);
              },
              child: SizedBox(
                height: 30,
                child: Row(
                  children: [
                    Icon(Icons.palette_outlined, size: 16, color: ink),
                    const SizedBox(width: 8),
                    Text(
                      labels['more'] as String,
                      style: TextStyle(fontSize: 12.5, color: ink),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _paletteSwatch(
    String purpose,
    int index,
    String value,
    bool selected,
    VoidCallback onTap,
  ) => Tooltip(
    message: value.toUpperCase(),
    child: InkWell(
      key: Key('pdf-palette-$purpose-$index'),
      onTap: onTap,
      child: Container(
        width: 20,
        height: 20,
        decoration: BoxDecoration(
          color: _hexColor(value),
          border: Border.all(
            color: selected ? accent : muted,
            width: selected ? 2 : 0.5,
          ),
        ),
      ),
    ),
  );

  Widget _ribbonItem(List<dynamic> item, bool mobile, bool compact) {
    final presentation = widget.definition.presentations[item[0]] as String?;
    final isPageArrangementToggle = presentation == 'pageArrangementToggle';
    final arrangementButton = isPageArrangementToggle
        ? widget.definition.pageArrangement.buttonFor(widget.pageArrangement)
        : null;
    final label = arrangementButton?.label ?? item[1] as String;
    final command = item[3] as String?;
    final menuId = item.length > 4 ? item[4] as String : null;
    final isPanelToggle = presentation == 'panelToggle';
    final isPalette = presentation == 'colorPalette';
    final selectionRequired = presentation == 'selectionRequired';
    final menuAvailable = (menuId != null || isPalette) && !selectionRequired;
    final enabled = isPanelToggle || _enabled(command);
    final iconSize = compact
        ? 20.0
        : item[0] == 'summary' || item[0] == 'keypoints'
        ? 24.0
        : 28.0;
    if (presentation == 'splitColor') {
      final paletteController = MenuController();
      return Tooltip(
        message: enabled ? label : '$label（当前内核尚未支持）',
        child: Row(
          children: [
            InkWell(
              key: Key('pdf-control-$_tab-highlight'),
              onTap: enabled
                  ? () => _dispatch(command, {'color': _highlightColor})
                  : null,
              child: SizedBox(
                width: compact
                    ? 31
                    : mobile
                    ? 51
                    : 48,
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    _glyph(
                      '$_tab.${item[0]}',
                      item[2] as String,
                      iconSize,
                      enabled ? ink : muted,
                    ),
                    Container(
                      width: 20,
                      height: 3,
                      color: _hexColor(_highlightColor),
                    ),
                    if (!compact) ...[
                      const SizedBox(height: 3),
                      Text(
                        label,
                        style: TextStyle(
                          fontSize: 12,
                          color: enabled ? ink : muted,
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ),
            MenuAnchor(
              controller: paletteController,
              crossAxisUnconstrained: false,
              style: _menuStyle('palette'),
              alignmentOffset: Offset(
                0,
                (_menuSpec('palette')['offsetY'] as num).toDouble(),
              ),
              menuChildren: [_palettePanel('highlight', paletteController)],
              builder: (context, controller, _) => InkWell(
                key: Key('pdf-highlight-colors-$_tab'),
                onTap: () =>
                    controller.isOpen ? controller.close() : controller.open(),
                child: SizedBox(
                  width: 16,
                  height: compact ? 30 : 48,
                  child: Icon(Icons.arrow_drop_down, size: 16, color: ink),
                ),
              ),
            ),
          ],
        ),
      );
    }
    Widget button = InkWell(
      key: Key('pdf-control-$_tab-${item[0]}'),
      onTap: menuId != null || isPalette
          ? null
          : enabled
          ? () => isPanelToggle ? _toggleAiDock() : _dispatch(command)
          : null,
      borderRadius: BorderRadius.circular(4),
      child: SizedBox(
        width: compact
            ? 38
            : mobile
            ? 63
            : label.length > 5
            ? 69
            : 58,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            _glyph(
              '$_tab.${item[0]}',
              item[2] as String,
              iconSize,
              enabled || menuAvailable ? ink : muted,
              iconAsset: arrangementButton?.icon,
            ),
            if (isPalette)
              Container(width: 20, height: 3, color: _hexColor(_drawColor)),
            if (!compact) ...[
              const SizedBox(height: 4),
              Text(
                label,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(
                  fontSize: 12,
                  color: enabled || menuAvailable ? ink : muted,
                ),
              ),
            ],
          ],
        ),
      ),
    );
    if ((isPanelToggle && _aiOpen) ||
        (isPageArrangementToggle &&
            widget.pageArrangement == PdfPageArrangement.spread)) {
      button = DecoratedBox(
        decoration: BoxDecoration(
          color: border,
          borderRadius: BorderRadius.circular(4),
        ),
        child: button,
      );
    }
    if (menuId != null || isPalette) {
      final trigger = button;
      final paletteController = MenuController();
      button = MenuAnchor(
        controller: isPalette ? paletteController : null,
        crossAxisUnconstrained: false,
        style: _menuStyle(isPalette ? 'palette' : 'ribbon'),
        alignmentOffset: Offset(
          0,
          (_menuSpec(isPalette ? 'palette' : 'ribbon')['offsetY'] as num)
              .toDouble(),
        ),
        menuChildren: isPalette
            ? [_palettePanel('draw', paletteController)]
            : _menuChildren(widget.definition.menus[menuId] as List),
        builder: (context, controller, _) => InkWell(
          onTap: menuAvailable
              ? () => controller.isOpen ? controller.close() : controller.open()
              : null,
          child: trigger,
        ),
      );
    }
    return Tooltip(
      message: selectionRequired
          ? '$label（请先选择至少两个字段）'
          : enabled || menuAvailable
          ? label
          : '$label（当前内核尚未支持）',
      child: button,
    );
  }

  List<Widget> _menuChildren(
    List<dynamic> nodes, [
    Map<String, dynamic> contextArgs = const {},
    String kind = 'ribbon',
  ]) => [
    for (final raw in nodes)
      if ((raw as Map)['children'] case final List children)
        SubmenuButton(
          menuStyle: _menuStyle(kind),
          style: _menuButtonStyle(kind),
          menuChildren: _menuChildren(children, contextArgs, kind),
          child: _menuLabel(raw['label'] as String, kind),
        )
      else
        MenuItemButton(
          style: _menuButtonStyle(kind),
          onPressed:
              raw['uiAction'] == 'highlightColor' && raw['color'] is String
              ? () => setState(() => _highlightColor = raw['color'] as String)
              : _enabled(raw['command'] as String?)
              ? () => _dispatch(raw['command'] as String, {
                  ...contextArgs,
                  ...Map<String, dynamic>.from(raw['args'] as Map? ?? {}),
                })
              : null,
          child: _menuLabel(raw['label'] as String, kind),
        ),
  ];

  Widget _aiDock(double viewportWidth, bool panelOpen, double width) {
    final spec = widget.definition.aiDock;
    return AnimatedContainer(
      key: const Key('pdf-ai-dock'),
      duration: Duration(
        milliseconds: (spec['transitionMs'] as num?)?.toInt() ?? 180,
      ),
      curve: Curves.easeInOut,
      width: panelOpen ? width : widget.definition.metric('aiRailWidth'),
      clipBehavior: Clip.hardEdge,
      decoration: BoxDecoration(
        color: surface,
        border: Border(right: BorderSide(color: border)),
      ),
      child: Stack(
        children: [
          Offstage(
            offstage: !panelOpen,
            child: OverflowBox(
              alignment: Alignment.centerLeft,
              minWidth: width,
              maxWidth: width,
              child: _aiPanel(viewportWidth: viewportWidth, panelWidth: width),
            ),
          ),
          Offstage(offstage: panelOpen, child: _aiRail()),
        ],
      ),
    );
  }

  Widget _aiRail() => Tooltip(
    message: widget.definition.aiDock['openLabel'] as String? ?? '打开 AI 助手',
    child: InkWell(
      key: const Key('pdf-ai-rail'),
      onTap: _toggleAiDock,
      child: Padding(
        padding: const EdgeInsets.only(top: 12),
        child: Align(
          alignment: Alignment.topCenter,
          child: Icon(Icons.auto_awesome, size: 23, color: muted),
        ),
      ),
    ),
  );

  Widget _aiPanel({
    double? viewportWidth,
    double? panelWidth,
    bool mobile = false,
  }) {
    final spec = widget.definition.aiDock;
    final size = mobile ? MediaQuery.sizeOf(context).width : viewportWidth!;
    final width = mobile
        ? size
        : panelWidth ??
              _clampAiWidth(
                _aiPreferredWidth ??
                    (spec['defaultWidth'] as num?)?.toDouble() ??
                    360,
                size,
              );
    return Stack(
      children: [
        SizedBox(
          key: const Key('pdf-ai-panel'),
          width: width,
          child: Column(
            children: [
              Container(
                height: 47,
                padding: const EdgeInsets.only(left: 14, right: 10),
                decoration: BoxDecoration(
                  border: Border(bottom: BorderSide(color: border)),
                ),
                child: Row(
                  children: [
                    Icon(Icons.auto_awesome, size: 22, color: ink),
                    const SizedBox(width: 7),
                    Expanded(
                      child: Text(
                        spec['title'] as String? ?? 'Muse AI',
                        style: TextStyle(
                          color: ink,
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                    IconButton(
                      key: const Key('pdf-ai-collapse'),
                      tooltip: spec['collapseLabel'] as String? ?? '收起 AI 助手',
                      onPressed: mobile
                          ? () => Navigator.of(context).pop()
                          : () => _setAiOpen(false),
                      icon: const Icon(
                        Icons.keyboard_double_arrow_left,
                        size: 18,
                      ),
                      color: muted,
                      constraints: const BoxConstraints.tightFor(
                        width: 26,
                        height: 26,
                      ),
                      padding: EdgeInsets.zero,
                    ),
                  ],
                ),
              ),
              Expanded(
                child: widget.assistantContent ?? _aiUnavailableBody(spec),
              ),
              if (widget.assistantContent == null) _aiDisabledComposer(spec),
            ],
          ),
        ),
        if (!mobile)
          Positioned(
            top: 0,
            bottom: 0,
            right: 0,
            width: (spec['resizerWidth'] as num?)?.toDouble() ?? 6,
            child: GestureDetector(
              key: const Key('pdf-ai-resizer'),
              behavior: HitTestBehavior.opaque,
              onHorizontalDragDown: (details) {
                _aiDragStartX = details.globalPosition.dx;
                _aiDragStartWidth = width;
              },
              onHorizontalDragStart: (_) {
                _aiWidthUserChanged = true;
                _aiOpenUserChanged = true;
              },
              onHorizontalDragUpdate: (details) => setState(() {
                final preferred = _clampAiWidth(
                  _aiDragStartWidth + details.globalPosition.dx - _aiDragStartX,
                  size,
                );
                _aiPreferredWidth = preferred.clamp(
                  (spec['minWidth'] as num).toDouble(),
                  size - widget.definition.responsiveLayout.minDocumentWidth,
                );
              }),
              onHorizontalDragEnd: (_) => _finishAiResize(),
              onHorizontalDragCancel: _finishAiResize,
              child: const MouseRegion(
                cursor: SystemMouseCursors.resizeLeftRight,
              ),
            ),
          ),
      ],
    );
  }

  Widget _aiUnavailableBody(Map<String, dynamic> spec) => SingleChildScrollView(
    padding: const EdgeInsets.fromLTRB(26, 68, 26, 16),
    child: Column(
      children: [
        Text(
          spec['emptyTitle'] as String? ?? '',
          textAlign: TextAlign.center,
          style: TextStyle(
            color: ink,
            fontSize: 17,
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 8),
        Text(
          spec['emptyBody'] as String? ?? '',
          textAlign: TextAlign.center,
          style: TextStyle(color: muted, fontSize: 13, height: 1.5),
        ),
        const SizedBox(height: 16),
        for (final name in ['summaryLabel', 'keyPointsLabel']) ...[
          OutlinedButton(
            onPressed: null,
            style: OutlinedButton.styleFrom(
              disabledForegroundColor: muted,
              side: BorderSide(color: border),
              shape: const StadiumBorder(),
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              textStyle: const TextStyle(fontSize: 13),
            ),
            child: Text(
              spec[name] as String? ?? '',
              style: TextStyle(
                fontFamily: Theme.of(context).textTheme.bodyMedium?.fontFamily,
                color: muted,
                fontSize: 13,
              ),
            ),
          ),
          const SizedBox(height: 8),
        ],
        const SizedBox(height: 12),
        Text(
          spec['unavailable'] as String? ?? '',
          key: const Key('pdf-ai-unavailable'),
          style: TextStyle(color: muted, fontSize: 12),
        ),
      ],
    ),
  );

  Widget _aiDisabledComposer(Map<String, dynamic> spec) => Padding(
    padding: const EdgeInsets.fromLTRB(12, 8, 12, 12),
    child: Container(
      height: 100,
      padding: const EdgeInsets.fromLTRB(14, 12, 8, 8),
      decoration: BoxDecoration(
        color: surface,
        border: Border.all(color: border),
        borderRadius: BorderRadius.circular(16),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            spec['promptPlaceholder'] as String? ?? '',
            style: TextStyle(color: muted, fontSize: 16),
          ),
          const Spacer(),
          Align(
            alignment: Alignment.bottomRight,
            child: Icon(Icons.arrow_upward, color: muted, size: 20),
          ),
        ],
      ),
    ),
  );

  Widget _sidebarPane(double width) => Container(
    key: const Key('pdf-sidebar-pane'),
    width: width,
    decoration: BoxDecoration(
      color: surface,
      border: Border(right: BorderSide(color: border)),
    ),
    child: _sidebar == 'thumbs'
        ? _thumbnailList()
        : Center(
            child: Text('暂无大纲', style: TextStyle(color: muted)),
          ),
  );

  Widget _sidebarWithResizer(double sidebarWidth, double viewportWidth) {
    final spec = widget.definition.sidebar;
    final handleWidth = (spec['resizerWidth'] as num?)?.toDouble() ?? 6;
    final overlap = ((spec['resizerOverlap'] as num?)?.toDouble() ?? 3).clamp(
      0.0,
      handleWidth,
    );
    return SizedBox(
      width: sidebarWidth + handleWidth - overlap,
      child: Stack(
        fit: StackFit.expand,
        children: [
          Positioned(
            left: 0,
            top: 0,
            bottom: 0,
            width: sidebarWidth,
            child: _sidebarPane(sidebarWidth),
          ),
          Positioned(
            left: sidebarWidth - overlap,
            top: 0,
            bottom: 0,
            width: handleWidth,
            child: GestureDetector(
              key: const Key('pdf-sidebar-resizer'),
              behavior: HitTestBehavior.opaque,
              onHorizontalDragDown: (details) {
                _sidebarDragStartX = details.globalPosition.dx;
                _sidebarDragStartWidth = sidebarWidth;
              },
              onHorizontalDragStart: (_) {
                _sidebarUserChanged = true;
                _sidebarWidth = sidebarWidth;
              },
              onHorizontalDragUpdate: (details) => setState(
                () => _sidebarWidth = _clampSidebarWidth(
                  _sidebarDragStartWidth +
                      details.globalPosition.dx -
                      _sidebarDragStartX,
                  viewportWidth,
                ),
              ),
              onHorizontalDragEnd: (_) =>
                  unawaited(_finishSidebarResize(viewportWidth)),
              child: const MouseRegion(
                cursor: SystemMouseCursors.resizeLeftRight,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _thumbnailList({bool mobile = false}) => ListView.builder(
    key: const Key('pdf-thumbnails'),
    padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 10),
    itemCount: widget.pageCount,
    itemBuilder: (context, index) => _thumbnailTile(index, mobile),
  );

  Widget _thumbnailTile(int index, bool mobile) {
    final tile = Focus(
      key: Key('pdf-thumb-focus-${index + 1}'),
      focusNode: _thumbnailFocusNodes.putIfAbsent(
        index,
        () => FocusNode(debugLabel: 'PDF thumbnail ${index + 1}'),
      ),
      onKeyEvent: _onKeyEvent,
      child: InkWell(
        canRequestFocus: false,
        onTap: () {
          _thumbnailFocusNodes[index]?.requestFocus();
          widget.onGoToPage(index + 1);
        },
        child: Column(
          children: [
            SizedBox(
              width: mobile ? 160 : double.infinity,
              child: _PdfThumbnail(
                key: ValueKey(
                  'thumb-${widget.fileName}-$index-${mobile ? 160 : _thumbnailRasterWidth}',
                ),
                load: () => widget.renderThumbnail(
                  index,
                  mobile ? 160 : _thumbnailRasterWidth,
                ),
                selected: widget.pageNumber == index + 1,
                accent: accent,
                border: muted,
              ),
            ),
            const SizedBox(height: 3),
            Text(
              '${index + 1}',
              style: TextStyle(
                fontSize: 11,
                color: widget.pageNumber == index + 1 ? accent : muted,
              ),
            ),
            const SizedBox(height: 10),
          ],
        ),
      ),
    );
    final menuId = widget.definition.sidebarContextMenu;
    if (menuId == null || widget.definition.menus[menuId] is! List) return tile;
    return MenuAnchor(
      crossAxisUnconstrained: false,
      style: _menuStyle('thumbnail'),
      menuChildren: _menuChildren(widget.definition.menus[menuId] as List, {
        'pageNumber': index + 1,
      }, 'thumbnail'),
      builder: (context, controller, _) => GestureDetector(
        key: Key('pdf-thumb-menu-${index + 1}'),
        onSecondaryTapDown: (_) => controller.open(),
        onLongPress: controller.open,
        child: tile,
      ),
    );
  }

  void _closeSearch() {
    setState(() => _searchOpen = false);
    _searchInput.clear();
    _searchFocus.unfocus();
    widget.onSearchClose?.call();
  }

  Widget _searchBar(double viewportWidth, bool mobile) {
    final spec = widget.definition.searchBar;
    final query = _searchInput.text.trim();
    final available = viewportWidth - (mobile ? 16 : 40) - 110;
    final inputWidth = ((spec['inputWidth'] as num?)?.toDouble() ?? 180).clamp(
      90.0,
      available.clamp(90.0, 180.0),
    );
    final count = query.isEmpty
        ? ''
        : widget.searchBusy && widget.searchTotal == 0
        ? (spec['searching'] as String? ?? '搜索中')
        : widget.searchTotal == 0
        ? (spec['noResults'] as String? ?? '无结果')
        : '${widget.searchCurrent} / ${widget.searchTotal}';
    return Container(
      key: const Key('pdf-search-bar'),
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 5),
      decoration: BoxDecoration(
        color: surface,
        border: Border.all(color: border),
        borderRadius: BorderRadius.circular(8),
        boxShadow: const [BoxShadow(color: Colors.black38, blurRadius: 12)],
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: inputWidth,
            height: 26,
            child: TextField(
              key: const Key('pdf-search-input'),
              controller: _searchInput,
              focusNode: _searchFocus,
              autofocus: true,
              style: TextStyle(fontSize: 13, color: ink),
              decoration: InputDecoration(
                hintText: spec['placeholder'] as String? ?? '搜索',
                hintStyle: TextStyle(color: muted),
                isDense: true,
                contentPadding: const EdgeInsets.symmetric(
                  horizontal: 8,
                  vertical: 4,
                ),
                filled: true,
                fillColor: surface,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(4),
                ),
              ),
              onChanged: (value) {
                setState(() {});
                widget.onSearchQuery?.call(value.trim());
              },
              onSubmitted: (_) => widget.onSearchStep?.call(1),
            ),
          ),
          const SizedBox(width: 4),
          SizedBox(
            width: 52,
            child: Text(
              count,
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 12, color: muted),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
          ),
          _searchButton(
            'pdf-search-prev',
            spec['previous'] as String? ?? '上一个',
            '‹',
            widget.searchTotal > 0 ? () => widget.onSearchStep?.call(-1) : null,
          ),
          _searchButton(
            'pdf-search-next',
            spec['next'] as String? ?? '下一个',
            '›',
            widget.searchTotal > 0 ? () => widget.onSearchStep?.call(1) : null,
          ),
          _searchButton(
            'pdf-search-close',
            spec['close'] as String? ?? '关闭',
            '×',
            _closeSearch,
          ),
        ],
      ),
    );
  }

  Widget _searchButton(
    String key,
    String tooltip,
    String glyph,
    VoidCallback? action,
  ) => Tooltip(
    message: tooltip,
    child: Semantics(
      label: tooltip,
      button: true,
      enabled: action != null,
      child: InkWell(
        key: Key(key),
        onTap: action,
        child: SizedBox(
          width: 26,
          height: 26,
          child: Center(
            child: Text(
              glyph,
              style: TextStyle(
                fontSize: 21,
                color: action == null ? muted.withValues(alpha: 0.4) : muted,
              ),
            ),
          ),
        ),
      ),
    ),
  );

  double get _minimumZoom =>
      (widget.definition.statusBar['minZoom'] as num?)?.toDouble() ?? 0.1;
  double get _maximumZoom =>
      (widget.definition.statusBar['maxZoom'] as num?)?.toDouble() ?? 8;
  double get _sliderStep =>
      (widget.definition.statusBar['sliderStep'] as num?)?.toDouble() ?? 0.05;

  void _stepZoom(int direction) {
    final steps =
        (widget.definition.statusBar['zoomSteps'] as List?)
            ?.map((value) => (value as num).toDouble())
            .toList() ??
        const <double>[
          0.1,
          0.25,
          0.5,
          0.67,
          0.8,
          0.9,
          1,
          1.1,
          1.25,
          1.5,
          1.75,
          2,
          2.5,
          3,
          4,
          5,
          6,
          8,
        ];
    final target = direction > 0
        ? steps.firstWhere(
            (step) => step > widget.zoom + 0.001,
            orElse: () => _maximumZoom,
          )
        : steps.reversed.firstWhere(
            (step) => step < widget.zoom - 0.001,
            orElse: () => _minimumZoom,
          );
    widget.onZoom(target);
  }

  Widget _statusZoomButton({
    required String keyName,
    required String tooltip,
    required IconData icon,
    required bool mobile,
    required VoidCallback? onTap,
  }) => Tooltip(
    message: tooltip,
    child: Semantics(
      label: tooltip,
      button: true,
      enabled: onTap != null,
      child: InkWell(
        key: Key(keyName),
        onTap: onTap,
        borderRadius: BorderRadius.circular(3),
        hoverColor: widget.definition.color('hover'),
        child: SizedBox(
          width: mobile ? 48 : 20,
          height: mobile ? 36 : 20,
          child: Icon(
            icon,
            size: 16,
            color: onTap == null ? muted.withValues(alpha: 0.4) : muted,
          ),
        ),
      ),
    ),
  );

  Widget _statusBar(bool mobile) => LayoutBuilder(
    builder: (context, viewport) {
      final showSlider =
          !mobile &&
          viewport.maxWidth >=
              ((widget.definition.statusBar['minSliderColumnWidth'] as num?)
                      ?.toDouble() ??
                  380);
      return Container(
        key: const Key('pdf-status-bar'),
        height: mobile ? 36 : widget.definition.metric('statusHeight'),
        padding: const EdgeInsets.symmetric(horizontal: 11),
        decoration: BoxDecoration(
          color: widget.definition.color('status'),
          border: Border(
            top: BorderSide(color: widget.definition.color('statusBorder')),
          ),
        ),
        child: Row(
          children: [
            if (mobile)
              IconButton(
                tooltip:
                    widget.definition.statusBar['thumbnails'] as String? ??
                    '缩略图',
                icon: const Icon(Icons.view_sidebar_outlined),
                iconSize: 18,
                color: muted,
                padding: EdgeInsets.zero,
                onPressed: widget.pageCount > 0
                    ? () => showModalBottomSheet<void>(
                        context: context,
                        builder: (_) => SizedBox(
                          height: 480,
                          child: _thumbnailList(mobile: true),
                        ),
                      )
                    : null,
              ),
            Text(
              (widget.definition.statusBar['pageLabel'] as String? ??
                      '第 {current} 页，共 {total} 页')
                  .replaceAll('{current}', '${widget.pageNumber}')
                  .replaceAll('{total}', '${widget.pageCount}'),
              style: TextStyle(fontSize: 11.5, color: muted),
            ),
            const Spacer(),
            _statusZoomButton(
              keyName: 'pdf-status-zoom-out',
              tooltip:
                  widget.definition.statusBar['zoomOut'] as String? ?? '缩小',
              icon: Icons.remove,
              mobile: mobile,
              onTap: _enabled('pdf.view.zoomOut') ? () => _stepZoom(-1) : null,
            ),
            if (showSlider) const SizedBox(width: 6),
            if (showSlider)
              SizedBox(
                width:
                    (widget.definition.statusBar['sliderWidth'] as num?)
                        ?.toDouble() ??
                    110,
                child: SliderTheme(
                  data: SliderThemeData(
                    trackHeight: 2,
                    thumbShape: const RoundSliderThumbShape(
                      enabledThumbRadius: 6,
                    ),
                    activeTrackColor: muted,
                    inactiveTrackColor: border,
                    thumbColor: muted,
                  ),
                  child: Slider(
                    key: const Key('pdf-status-zoom-slider'),
                    value: widget.zoom.clamp(_minimumZoom, _maximumZoom),
                    min: _minimumZoom,
                    max: _maximumZoom,
                    onChanged: widget.pageCount > 0
                        ? (value) => widget.onZoom(
                            ((value / _sliderStep).round() * _sliderStep).clamp(
                              _minimumZoom,
                              _maximumZoom,
                            ),
                          )
                        : null,
                  ),
                ),
              ),
            if (showSlider) const SizedBox(width: 6),
            _statusZoomButton(
              keyName: 'pdf-status-zoom-in',
              tooltip: widget.definition.statusBar['zoomIn'] as String? ?? '放大',
              icon: Icons.add,
              mobile: mobile,
              onTap: _enabled('pdf.view.zoomIn') ? () => _stepZoom(1) : null,
            ),
            const SizedBox(width: 6),
            SizedBox(
              width: 38,
              child: Text(
                '${(widget.zoom * 100).round()}%',
                textAlign: TextAlign.right,
                style: TextStyle(fontSize: 11.5, color: muted),
              ),
            ),
          ],
        ),
      );
    },
  );
}

class _PdfThumbnail extends StatefulWidget {
  const _PdfThumbnail({
    super.key,
    required this.load,
    required this.selected,
    required this.accent,
    required this.border,
  });
  final Future<ui.Image?> Function() load;
  final bool selected;
  final Color accent, border;
  @override
  State<_PdfThumbnail> createState() => _PdfThumbnailState();
}

class _PdfThumbnailState extends State<_PdfThumbnail> {
  ui.Image? _image;
  @override
  void initState() {
    super.initState();
    unawaited(_load());
  }

  Future<void> _load() async {
    try {
      final image = await widget.load();
      if (!mounted) {
        image?.dispose();
        return;
      }
      setState(() => _image = image);
    } catch (error, stackTrace) {
      debugPrint('PDF thumbnail render failed: $error\n$stackTrace');
    }
  }

  @override
  void dispose() {
    _image?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AspectRatio(
    aspectRatio: _image == null ? 0.77 : _image!.width / _image!.height,
    child: Container(
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border.all(
          color: widget.selected ? widget.accent : widget.border,
          width: 2,
        ),
        borderRadius: BorderRadius.circular(4),
      ),
      clipBehavior: Clip.antiAlias,
      child: _image == null
          ? const SizedBox.shrink()
          : RawImage(image: _image, fit: BoxFit.fill),
    ),
  );
}
