import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:word_render/word_render.dart';

import 'word_style_snapshot.dart';
import 'word_statistics_snapshot.dart';

/// Draft character style not yet applied by the kernel (no run mutate on FFI).
class WordDraftStyle {
  const WordDraftStyle({
    this.bold = false,
    this.italic = false,
  });

  final bool bold;
  final bool italic;

  WordDraftStyle copyWith({bool? bold, bool? italic}) => WordDraftStyle(
        bold: bold ?? this.bold,
        italic: italic ?? this.italic,
      );
}

class WordDragSelection {
  const WordDragSelection({
    required this.pageIndex,
    required this.ax,
    required this.ay,
    required this.bx,
    required this.by,
  });

  final int pageIndex;
  final int ax;
  final int ay;
  final int bx;
  final int by;

  RectTwip get bounds {
    return RectTwip(
      left: ax < bx ? ax : bx,
      top: ay < by ? ay : by,
      right: ax < bx ? bx : ax,
      bottom: ay < by ? by : ay,
    );
  }
}

class RectTwip {
  const RectTwip({
    required this.left,
    required this.top,
    required this.right,
    required this.bottom,
  });

  final int left;
  final int top;
  final int right;
  final int bottom;
}

/// Viewport fitting is presentation state; the paginated document is unchanged.
enum WordZoomFitMode { pageWidth, wholePage, multiplePages }

ViewParams _copyView(
  ViewParams view, {
  double? fontSizePt,
  int? jc,
  bool? underline,
}) {
  return ViewParams(
    pageWidthTwip: view.pageWidthTwip,
    pageHeightTwip: view.pageHeightTwip,
    marginLeftTwip: view.marginLeftTwip,
    marginRightTwip: view.marginRightTwip,
    marginTopTwip: view.marginTopTwip,
    marginBottomTwip: view.marginBottomTwip,
    contentWidthTwip: view.contentWidthTwip,
    contentHeightTwip: view.contentHeightTwip,
    fontSizePt: fontSizePt ?? view.fontSizePt,
    jc: jc ?? view.jc,
    requireInjectedFonts: view.requireInjectedFonts,
    underline: underline ?? view.underline,
  );
}

/// Host-facing editor session: open/layout + view chrome state.
class WordEditorController extends ChangeNotifier {
  WordEditorController({WordCoreEngine? engine})
      : engine = engine ?? WordCoreEngine();

  final WordCoreEngine engine;

  RenderEnvelope? envelope;
  Uint8List? _docx;
  String documentName = 'Document';
  String status = 'idle';
  double zoom = 1.0;
  bool _manualZoom = false;
  WordZoomFitMode? zoomFitMode;

  /// View arrangement survives manual zoom, as in GenOffice's Multiple Pages.
  bool multiplePages = false;
  int visiblePage = 1;

  /// Presentation state only; the document model and layout stay in word-core.
  bool readingMode = false;
  bool showGridlines = false;
  bool darkPage = false;
  bool showRuler = false;
  int pageNavigationSerial = 0;
  int requestedPage = 1;
  ViewParams view = kViewParamsA4;
  WordCaret? caret;
  int? caretCp;
  int? selStart;
  int? selEnd;
  WordDragSelection? selection;
  WordDraftStyle draft = const WordDraftStyle();
  WordStyleSnapshot? styleSnapshot;
  WordStatisticsSnapshot? statisticsSnapshot;
  int? wordCount;
  int selectedWordCount = 0;
  Timer? _wordCountTimer;
  bool _alive = true;

  @override
  void dispose() {
    _alive = false;
    _wordCountTimer?.cancel();
    super.dispose();
  }

  void _notify() {
    if (!_alive) return;
    notifyListeners();
  }

  bool get canRelayout => engine.ffiReady && _docx != null;

  void report(String message) {
    status = message;
    _notify();
  }

  void setShowGridlines(bool enabled) {
    if (showGridlines == enabled) return;
    showGridlines = enabled;
    _notify();
  }

  void setDarkPage(bool enabled) {
    if (darkPage == enabled) return;
    darkPage = enabled;
    _notify();
  }

  void setShowRuler(bool enabled) {
    if (showRuler == enabled) return;
    showRuler = enabled;
    _notify();
  }

  int get pageCount {
    final n = envelope?.page.pageCount ?? 0;
    return n <= 0 ? 0 : n;
  }

  Future<void> init() async {
    status = 'init FFI…';
    _notify();
    try {
      await engine.initFfi();
      status = engine.ffiReady ? 'Ready' : engine.source;
    } catch (e) {
      status = 'FFI init error: $e';
    }
    _notify();
  }

  Future<void> openBytes(Uint8List bytes, {required String name}) async {
    _wordCountTimer?.cancel();
    documentName = name;
    _docx = bytes;
    envelope = null;
    visiblePage = 1;
    readingMode = false;
    zoom = 1.0;
    _manualZoom = false;
    zoomFitMode = null;
    multiplePages = false;
    caret = null;
    caretCp = null;
    selStart = null;
    selEnd = null;
    selection = null;
    styleSnapshot = null;
    statisticsSnapshot = null;
    wordCount = null;
    selectedWordCount = 0;
    engine.session = null;
    if (!engine.ffiReady) {
      status = 'FFI not ready: ${engine.source}';
      _notify();
      return;
    }
    status = 'openSession…';
    _notify();
    try {
      envelope = await engine.openSession(bytes, view: view);
      final styleJson = engine.sessionStyleSnapshotJson();
      final styles = styleJson == null
          ? null
          : WordStyleSnapshot.fromEngineJson(styleJson);
      // A DOCX may omit styles.xml. Keep the signed product's fallback gallery
      // in that case; an empty metadata projection is not an empty UI gallery.
      styleSnapshot = styles?.styles.isEmpty == true ? null : styles;
      _refreshWordStatistics();
      visiblePage = 1;
      status =
          '${envelope!.commands.length} cmds · session · ${engine.paintFontSource}';
    } catch (e) {
      envelope = null;
      status = 'error: $e';
    }
    _notify();
  }

  Future<void> showGolden(RenderEnvelope env, {required String name}) async {
    _wordCountTimer?.cancel();
    documentName = name;
    envelope = env;
    _docx = null;
    engine.session = null;
    caret = null;
    caretCp = null;
    selStart = null;
    selEnd = null;
    selection = null;
    styleSnapshot = null;
    statisticsSnapshot = null;
    wordCount = null;
    selectedWordCount = 0;
    visiblePage = 1;
    zoom = 1.0;
    _manualZoom = false;
    zoomFitMode = null;
    multiplePages = false;
    status = '${env.commands.length} cmds · golden JSON';
    _notify();
  }

  /// Replaces presentation metadata after a document/selection update. The
  /// engine remains responsible for computing it from the active document.
  void setStyleSnapshot(WordStyleSnapshot? snapshot) {
    styleSnapshot = snapshot;
    _notify();
  }

  /// Read-only presentation state. Test previews may provide a known value.
  void setWordStatistics(int? total, {int selected = 0}) {
    wordCount = total;
    selectedWordCount = selected;
    _notify();
  }

  void setStatisticsSnapshot(WordStatisticsSnapshot? snapshot) {
    statisticsSnapshot = snapshot;
    wordCount = snapshot?.words;
    _notify();
  }

  void _refreshWordStatistics() {
    if (engine.session == null) return;
    try {
      final json = engine.sessionTextStatisticsJson();
      if (json != null) {
        statisticsSnapshot = WordStatisticsSnapshot.fromEngineJson(
          json,
          envelope: envelope,
        );
        wordCount = statisticsSnapshot!.words;
      }
      final start = selStart;
      final end = selEnd;
      selectedWordCount = start != null && end != null && start != end
          ? engine.sessionWordCount(start, end) ?? 0
          : 0;
    } catch (_) {
      // Word statistics are supplementary UI; keep the document interactive.
    }
    _notify();
  }

  void _scheduleWordStatistics() {
    _wordCountTimer?.cancel();
    _wordCountTimer = Timer(const Duration(milliseconds: 400), () {
      if (_alive) _refreshWordStatistics();
    });
  }

  void setZoom(double value) {
    _manualZoom = true;
    zoomFitMode = null;
    zoom = value.clamp(0.1, 5.0);
    _notify();
  }

  void setZoomFitMode(WordZoomFitMode mode) {
    if (pageCount <= 0) return;
    _manualZoom = false;
    multiplePages = mode == WordZoomFitMode.multiplePages;
    zoomFitMode = mode;
    _notify();
  }

  /// Fit pages to a narrow viewport until the user chooses an explicit zoom.
  void setAutoFitZoom(double value,
      {double? wholePageZoom, double? multiplePagesZoom}) {
    if (_manualZoom) return;
    final next = (switch (zoomFitMode) {
      WordZoomFitMode.wholePage => wholePageZoom ?? value,
      WordZoomFitMode.multiplePages => multiplePagesZoom ?? value,
      _ => value,
    })
        .clamp(0.1, 5.0);
    if ((zoom - next).abs() < 0.001) return;
    zoom = next;
    _notify();
  }

  void setVisiblePage(int oneBased) {
    final n = pageCount;
    if (n <= 0) return;
    final next = oneBased.clamp(1, n);
    if (next == visiblePage) return;
    visiblePage = next;
    _notify();
  }

  /// Native view chrome state shared by the Ribbon, status bar and canvas.
  /// It does not alter the word-core session or its paginated output.
  void setReadingMode(bool enabled) {
    if (readingMode == enabled) return;
    readingMode = enabled;
    _notify();
  }

  /// Low-frequency view request. The workspace owns scrolling; this never
  /// changes the document model or layout.
  void navigateToPage(int oneBased) {
    final n = pageCount;
    if (n <= 0) return;
    requestedPage = oneBased.clamp(1, n);
    visiblePage = requestedPage;
    pageNavigationSerial++;
    _notify();
  }

  Future<void> setAlign(int jc) async {
    view = _copyView(view, jc: jc);
    await _relayout();
  }

  Future<void> setFontSizePt(double pt) async {
    view = _copyView(view, fontSizePt: pt);
    await _relayout();
  }

  Future<void> setUnderline(bool on) async {
    view = _copyView(view, underline: on);
    await _relayout();
  }

  void toggleBold() {
    draft = draft.copyWith(bold: !draft.bold);
    _notify();
  }

  void toggleItalic() {
    draft = draft.copyWith(italic: !draft.italic);
    _notify();
  }

  void tapPage(int pageIndex, Offset localPx) {
    final env = envelope;
    if (env == null) return;
    final hit = hitTestPage(
      pageIndex: pageIndex,
      localPx: localPx,
      zoom: zoom,
      page: env.page,
    );
    if (hit == null) return;
    if (engine.session != null) {
      final c = engine.sessionCaretAt(
        page: pageIndex,
        x: hit.xTwip,
        y: hit.yTwip,
      );
      if (c == null) return;
      caretCp = c.cp;
      caret = WordCaret(
        pageIndex: c.page,
        xTwip: c.x,
        yTwip: c.y,
        heightTwip: c.height,
      );
      selStart = null;
      selEnd = null;
      selection = null;
      selectedWordCount = 0;
      try {
        envelope = engine.sessionClearSelection();
      } catch (_) {}
      visiblePage = pageIndex + 1;
      _notify();
      return;
    }
    caret = caretFromTap(env, hit);
    selection = null;
    visiblePage = pageIndex + 1;
    _notify();
  }

  void dragPage(int pageIndex, Offset a, Offset b) {
    final env = envelope;
    if (env == null) return;
    final ha = hitTestPage(
      pageIndex: pageIndex,
      localPx: a,
      zoom: zoom,
      page: env.page,
    );
    final hb = hitTestPage(
      pageIndex: pageIndex,
      localPx: b,
      zoom: zoom,
      page: env.page,
    );
    if (ha == null || hb == null) return;
    if (engine.session != null) {
      final ca =
          engine.sessionCaretAt(page: pageIndex, x: ha.xTwip, y: ha.yTwip);
      final cb =
          engine.sessionCaretAt(page: pageIndex, x: hb.xTwip, y: hb.yTwip);
      if (ca == null || cb == null) return;
      final lo = math.min(ca.cp, cb.cp);
      final hi = math.max(ca.cp, cb.cp);
      selStart = lo;
      selEnd = hi;
      caretCp = cb.cp;
      caret = WordCaret(
        pageIndex: cb.page,
        xTwip: cb.x,
        yTwip: cb.y,
        heightTwip: cb.height,
      );
      selection = null;
      try {
        envelope = engine.sessionSetSelection(lo, hi);
      } catch (e) {
        status = 'error: $e';
      }
      _scheduleWordStatistics();
      visiblePage = pageIndex + 1;
      _notify();
      return;
    }
    selection = WordDragSelection(
      pageIndex: pageIndex,
      ax: ha.xTwip,
      ay: ha.yTwip,
      bx: hb.xTwip,
      by: hb.yTwip,
    );
    caret = caretFromTap(env, hb);
    visiblePage = pageIndex + 1;
    _notify();
  }

  Future<void> handleKeyEvent(KeyEvent event) async {
    if (event is! KeyDownEvent && event is! KeyRepeatEvent) return;
    if (engine.session == null) return;
    final key = event.logicalKey;
    try {
      if (key == LogicalKeyboardKey.backspace) {
        await _deleteBackward();
      } else if (key == LogicalKeyboardKey.delete) {
        await _deleteForward();
      } else if (key == LogicalKeyboardKey.enter ||
          key == LogicalKeyboardKey.numpadEnter) {
        await _insertAtCaret('\r');
      } else {
        final ch = event.character;
        if (ch == null || ch.isEmpty) return;
        if (ch == '\n' || ch == '\r') {
          await _insertAtCaret('\r');
        } else if (ch.codeUnitAt(0) >= 0x20) {
          await _insertAtCaret(ch);
        }
      }
      _scheduleWordStatistics();
    } catch (e) {
      status = 'error: $e';
      _notify();
    }
  }

  Future<void> _deleteSelIfAny() async {
    final a = selStart;
    final b = selEnd;
    if (a != null && b != null && a != b) {
      envelope = engine.sessionRemove(a, b);
      caretCp = a;
      selStart = null;
      selEnd = null;
      _syncCaretGeom();
    }
  }

  Future<void> _insertAtCaret(String text) async {
    await _deleteSelIfAny();
    final cp = caretCp ?? engine.sessionMainLength() ?? 0;
    envelope = engine.sessionInsert(cp, text);
    caretCp = cp + text.runes.length;
    _syncCaretGeom();
    status =
        '${envelope!.commands.length} cmds · session · layoutCalls=${engine.layoutCalls}';
    _notify();
  }

  Future<void> _deleteBackward() async {
    if (selStart != null && selEnd != null && selStart != selEnd) {
      await _deleteSelIfAny();
      status = 'session delete';
      _notify();
      return;
    }
    final cp = caretCp;
    if (cp == null || cp <= 0) return;
    envelope = engine.sessionRemove(cp - 1, cp);
    caretCp = cp - 1;
    _syncCaretGeom();
    _notify();
  }

  Future<void> _deleteForward() async {
    if (selStart != null && selEnd != null && selStart != selEnd) {
      await _deleteSelIfAny();
      _notify();
      return;
    }
    final cp = caretCp;
    if (cp == null) return;
    final len = engine.sessionMainLength() ?? 0;
    if (cp >= len) return;
    envelope = engine.sessionRemove(cp, cp + 1);
    _syncCaretGeom();
    _notify();
  }

  void _syncCaretGeom() {
    final env = envelope;
    final cp = caretCp;
    if (env == null || cp == null) return;
    final g = caretFromVisibleCp(env, cp);
    if (g != null) caret = g;
  }

  Future<void> _relayout() async {
    final bytes = _docx;
    if (!engine.ffiReady) {
      _notify();
      return;
    }
    status = 'layout…';
    _notify();
    try {
      if (engine.session != null) {
        envelope = await engine.sessionSetView(view);
      } else if (bytes != null) {
        envelope = await engine.layoutDocument(bytes, view: view);
      } else {
        _notify();
        return;
      }
      status =
          '${envelope!.commands.length} cmds · ${envelope!.pipeline} · layoutCalls=${engine.layoutCalls}';
      _refreshWordStatistics();
    } catch (e) {
      status = 'error: $e';
    }
    _notify();
  }
}
