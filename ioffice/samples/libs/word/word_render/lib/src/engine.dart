import 'dart:ui' as ui;

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'command_executor.dart';
import 'ffi_loader_stub.dart' if (dart.library.io) 'ffi_loader_io.dart';
import 'font_source_stub.dart' if (dart.library.io) 'font_source_io.dart';
import 'render_command.dart';
import 'rust/api.dart' as rust;
import 'rust/api/session.dart' as sess;
import 'rust/frb_generated.dart';

/// Package asset for the latin fallback face (same bytes as FontHost injection).
const kWordRenderBundledFontAsset = 'packages/word_render/assets/MvpLatin.ttf';

/// Default A4 ViewParams in **twip** (constructed in Dart — no extra FFI, no px).
const rust.ViewParams kViewParamsA4 = rust.ViewParams(
  pageWidthTwip: 11906,
  pageHeightTwip: 16838,
  marginLeftTwip: 1800,
  marginRightTwip: 1800,
  marginTopTwip: 1440,
  marginBottomTwip: 1440,
  fontSizePt: 12.0,
  jc: 0,
  requireInjectedFonts: false,
  underline: false,
);

/// Heap `DocumentSession` plus one-shot `layoutDocument`. Zoom must not increment [layoutCalls].
class WordCoreEngine {
  int layoutCalls = 0;
  bool ffiReady = false;
  bool paintFontReady = false;
  String source = '';
  Uint8List? mvpFontBytes;
  final List<rust.FontBlob> injectedFaces = [];

  /// Heap session (wr-18); null when using one-shot [layoutDocument] or golden JSON.
  sess.WordSession? session;

  /// `host-injected` (Arial [+ CJKFallback]) or `bundled-MvpLatin`.
  String paintFontSource = '';
  bool _cjkReady = false;
  static bool _rustLibInited = false;

  Future<void> initFfi() async {
    if (kIsWeb) {
      source = 'web: wasm deferred; using golden JSON';
      return;
    }
    if (ffiReady) return;
    final path = await resolveFfiLibrary();
    if (path == null) {
      source = 'FFI dylib missing (cargo build -p word_core_ffi)';
      return;
    }
    if (!_rustLibInited) {
      await RustLib.init(externalLibrary: ExternalLibrary.open(path));
      _rustLibInited = true;
    }
    if (isAndroidHost) {
      const channel = MethodChannel('docx_mvp_render/platform');
      final cacheDir = await channel.invokeMethod<String>('cacheDir');
      if (cacheDir == null || cacheDir.isEmpty) {
        throw LayoutException('Android cache directory unavailable');
      }
      rust.configureTempDirectory(path: cacheDir);
    }
    ffiReady = true;
    source = 'FFI $path';
  }

  /// Same TTF bytes as the kernel FontHost table.
  /// [includeCjk] loads a fallback CJK face only when the document needs it.
  Future<void> ensurePaintFont({bool includeCjk = false}) async {
    if (paintFontReady &&
        mvpFontBytes != null &&
        injectedFaces.isNotEmpty &&
        (!includeCjk || _cjkReady)) {
      return;
    }
    injectedFaces.clear();
    final host = tryLoadHostFonts(includeCjk: includeCjk);
    if (host.isNotEmpty) {
      for (final f in host) {
        injectedFaces.add(rust.FontBlob(id: f.id, bytes: f.bytes));
        final family = f.id == 'default' ? kWcDefaultFontFamily : f.id;
        await ui.loadFontFromList(f.bytes, fontFamily: family);
      }
      mvpFontBytes = host.first.bytes;
      paintFontSource = 'host-injected ${host.map((f) => f.id).join("+")}';
      _cjkReady = host.any((f) => f.id == 'CJKFallback');
    } else {
      final data = await rootBundle.load(kWordRenderBundledFontAsset);
      mvpFontBytes = data.buffer.asUint8List();
      injectedFaces.add(rust.FontBlob(id: 'default', bytes: mvpFontBytes!));
      await ui.loadFontFromList(mvpFontBytes!,
          fontFamily: kWcDefaultFontFamily);
      paintFontSource = 'bundled-MvpLatin';
      _cjkReady = false;
    }
    paintFontReady = true;
  }

  /// Product I/O. Throws [LayoutException] on rust Err (do not return empty commands).
  Future<RenderEnvelope> layoutDocument(
    Uint8List docx, {
    List<rust.FontBlob>? fonts,
    rust.ViewParams view = kViewParamsA4,
  }) async {
    layoutCalls++;
    if (!ffiReady) {
      throw LayoutException('FFI not initialized: $source');
    }
    await ensurePaintFont();
    final injected = fonts ?? List<rust.FontBlob>.from(injectedFaces);
    try {
      final frame = rust.layoutDocument(
        docx: docx,
        fonts: injected,
        view: view,
      );
      final envelope = envelopeFromFrame(frame);
      await envelope.decodeImages();
      return envelope;
    } catch (e) {
      final msg = '$e';
      final missingGlyph = msg.contains('glyph') ||
          msg.contains('Measure') ||
          msg.contains('U+');
      if (fonts == null && !_cjkReady && missingGlyph) {
        await ensurePaintFont(includeCjk: true);
        if (_cjkReady) {
          final frame = rust.layoutDocument(
            docx: docx,
            fonts: List<rust.FontBlob>.from(injectedFaces),
            view: view,
          );
          final envelope = envelopeFromFrame(frame);
          await envelope.decodeImages();
          return envelope;
        }
      }
      throw LayoutException(msg);
    }
  }

  /// Open a heap session and layout (edit path). Zoom still does not call this.
  Future<RenderEnvelope> openSession(
    Uint8List docx, {
    List<rust.FontBlob>? fonts,
    rust.ViewParams view = kViewParamsA4,
  }) async {
    layoutCalls++;
    if (!ffiReady) {
      throw LayoutException('FFI not initialized: $source');
    }
    await ensurePaintFont();
    final injected = fonts ?? List<rust.FontBlob>.from(injectedFaces);
    try {
      session =
          await sess.WordSession.open(docx: docx, fonts: injected, view: view);
      final envelope = envelopeFromFrame(await session!.layout());
      await envelope.decodeImages();
      return envelope;
    } catch (e) {
      final msg = '$e';
      final missingGlyph = msg.contains('glyph') ||
          msg.contains('Measure') ||
          msg.contains('U+');
      if (fonts == null && !_cjkReady && missingGlyph) {
        await ensurePaintFont(includeCjk: true);
        if (_cjkReady) {
          session = await sess.WordSession.open(
            docx: docx,
            fonts: List<rust.FontBlob>.from(injectedFaces),
            view: view,
          );
          final envelope = envelopeFromFrame(await session!.layout());
          await envelope.decodeImages();
          return envelope;
        }
      }
      session = null;
      throw LayoutException(msg);
    }
  }

  Future<RenderEnvelope> sessionSetView(rust.ViewParams view) async {
    final s = session;
    if (s == null) {
      throw LayoutException('no session');
    }
    layoutCalls++;
    return envelopeFromFrame(s.setView(view: view));
  }

  sess.CaretHit? sessionCaretAt({
    required int page,
    required int x,
    required int y,
  }) {
    return session?.caretAt(page: page, x: x, y: y);
  }

  RenderEnvelope sessionSetSelection(int start, int end) {
    final s = session;
    if (s == null) {
      throw LayoutException('no session');
    }
    return envelopeFromFrame(s.setSelection(start: start, end: end));
  }

  RenderEnvelope sessionClearSelection() {
    final s = session;
    if (s == null) {
      throw LayoutException('no session');
    }
    return envelopeFromFrame(s.clearSelection());
  }

  RenderEnvelope sessionInsert(int cp, String text) {
    final s = session;
    if (s == null) {
      throw LayoutException('no session');
    }
    layoutCalls++;
    return envelopeFromFrame(s.insertText(cp: cp, text: text));
  }

  RenderEnvelope sessionRemove(int start, int end) {
    final s = session;
    if (s == null) {
      throw LayoutException('no session');
    }
    layoutCalls++;
    return envelopeFromFrame(s.removeRange(start: start, end: end));
  }

  int? sessionMainLength() => session?.mainLength();

  /// Read-only word statistics from the live native document model.
  int? sessionWordCount(int start, int end) =>
      session?.wordCount(start: start, end: end);

  String? sessionTextStatisticsJson() => session?.textStatisticsJson();

  /// Serialized read-only metadata from the same native document session.
  /// The editor decodes it into its presentation model; the render package
  /// does not depend on editor widgets or plugin data.
  String? sessionStyleSnapshotJson() => session?.styleSnapshotJson();
}

class LayoutException implements Exception {
  LayoutException(this.message);
  final String message;
  @override
  String toString() => 'LayoutException: $message';
}

RenderEnvelope envelopeFromFrame(rust.LayoutFrame frame, {String dump = ''}) {
  final commands = <WcRenderCommand>[];
  for (final cmd in frame.commands) {
    commands.add(switch (cmd) {
      rust.RenderCommandWire_Text(
        :final page,
        :final x,
        :final y,
        :final text,
        :final fontId,
        :final size,
        :final color,
        :final style,
        :final xpos,
      ) =>
        WcDrawText(
          page: page,
          x: x,
          y: y,
          text: text,
          fontId: fontId,
          sizeTwip: size,
          color: color,
          style: style,
          xpos: xpos,
        ),
      rust.RenderCommandWire_Rect(
        :final page,
        :final x,
        :final y,
        :final w,
        :final h,
        :final color,
        :final kind,
      ) =>
        WcDrawRect(
          page: page,
          x: x,
          y: y,
          w: w,
          h: h,
          color: color,
          kind: kind,
        ),
      rust.RenderCommandWire_Line(
        :final page,
        :final x1,
        :final y1,
        :final x2,
        :final y2,
        :final kind,
        :final color,
        :final width,
      ) =>
        WcDrawLine(
          page: page,
          x1: x1,
          y1: y1,
          x2: x2,
          y2: y2,
          kind: kind,
          color: color,
          width: width,
        ),
      rust.RenderCommandWire_Image(
        :final page,
        :final x,
        :final y,
        :final w,
        :final h,
        :final shapeId,
        :final mediaId,
        :final mediaPath,
        :final bytes,
        :final rotation,
        :final cropLeft,
        :final cropTop,
        :final cropRight,
        :final cropBottom,
      ) =>
        WcDrawImage(
          page: page,
          x: x,
          y: y,
          w: w,
          h: h,
          shapeId: shapeId,
          mediaId: mediaId,
          mediaPath: mediaPath,
          bytes: bytes,
          rotation: rotation,
          cropLeft: cropLeft,
          cropTop: cropTop,
          cropRight: cropRight,
          cropBottom: cropBottom,
        ),
    });
  }
  return RenderEnvelope(
    ok: true,
    pipeline: frame.pipeline,
    page: PageMetrics(
      pageCount: frame.page.pageCount,
      pageWidthTwip: frame.page.pageWidthTwip,
      pageHeightTwip: frame.page.pageHeightTwip,
      marginLeftTwip: frame.page.marginLeftTwip,
      marginRightTwip: frame.page.marginRightTwip,
      marginTopTwip: frame.page.marginTopTwip,
      marginBottomTwip: frame.page.marginBottomTwip,
      paperSize: frame.page.paperSize,
    ),
    commands: commands,
    rawJson: dump,
  );
}
