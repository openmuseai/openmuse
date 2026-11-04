import 'dart:convert';
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'dart:ui' show Color;

import 'wmf_preview.dart';

/// word-core Stage3 tagged union (`t`: text | rect | line). Coordinates in **twip**.
abstract class WcRenderCommand {
  int get page;

  static List<WcRenderCommand> fromDocumentJson(Map<String, dynamic> document) {
    final list = document['commands'];
    if (list is! List) return [];
    final out = <WcRenderCommand>[];
    for (final item in list) {
      if (item is! Map) continue;
      final map = Map<String, dynamic>.from(item);
      final t = map['t']?.toString();
      if (t == 'text') {
        out.add(WcDrawText.fromJson(map));
      } else if (t == 'rect') {
        out.add(WcDrawRect.fromJson(map));
      } else if (t == 'line') {
        out.add(WcDrawLine.fromJson(map));
      } else if (t == 'image') {
        out.add(WcDrawImage.fromJson(map));
      }
    }
    return out;
  }
}

/// Image bytes travel with the FFI frame; [mediaPath] is diagnostic only.
class WcDrawImage extends WcRenderCommand {
  WcDrawImage({
    required this.page,
    required this.x,
    required this.y,
    required this.w,
    required this.h,
    required this.shapeId,
    required this.mediaId,
    required this.mediaPath,
    required this.bytes,
    required this.rotation,
    required this.cropLeft,
    required this.cropTop,
    required this.cropRight,
    required this.cropBottom,
  });

  @override
  final int page;
  final int x, y, w, h, shapeId, mediaId;
  final String mediaPath;
  final Uint8List bytes;
  final double rotation, cropLeft, cropTop, cropRight, cropBottom;
  ui.Image? decoded;
  WmfPreview? wmfPreview;
  String? decodeError;

  bool get isRenderable => decoded != null || wmfPreview != null;

  Future<void> decode() async {
    if (isRenderable || bytes.isEmpty) return;
    if (mediaPath.toLowerCase().endsWith('.wmf')) {
      try {
        wmfPreview = WmfPreview.parse(bytes);
        decodeError = null;
      } catch (error) {
        decodeError = 'WMF shape $shapeId: $error';
      }
      return;
    }
    final codec = await ui.instantiateImageCodec(bytes);
    try {
      final frame = await codec.getNextFrame();
      decoded = frame.image;
    } finally {
      codec.dispose();
    }
  }

  factory WcDrawImage.fromJson(Map<String, dynamic> m) => WcDrawImage(
        page: _i(m['page']),
        x: _i(m['x']),
        y: _i(m['y']),
        w: _i(m['w']),
        h: _i(m['h']),
        shapeId: _i(m['shape_id']),
        mediaId: _i(m['media_id']),
        mediaPath: _s(m['media_path']),
        bytes: Uint8List(0),
        rotation: (m['rotation'] as num?)?.toDouble() ?? 0,
        cropLeft: (m['crop_left'] as num?)?.toDouble() ?? 0,
        cropTop: (m['crop_top'] as num?)?.toDouble() ?? 0,
        cropRight: (m['crop_right'] as num?)?.toDouble() ?? 0,
        cropBottom: (m['crop_bottom'] as num?)?.toDouble() ?? 0,
      );
}

int _i(dynamic v) {
  if (v is int) return v;
  if (v is double) return v.round();
  return int.tryParse('$v') ?? 0;
}

String _s(dynamic v) => v?.toString() ?? '';

class WcDrawText extends WcRenderCommand {
  WcDrawText({
    required this.page,
    required this.x,
    required this.y,
    required this.text,
    required this.fontId,
    required this.sizeTwip,
    required this.color,
    required this.style,
    this.xpos = const [],
  });

  @override
  final int page;
  final int x;
  final int y;
  final String text;
  final String fontId;
  final int sizeTwip;
  final int color;
  final int style;

  /// Page-absolute twip per visible glyph. Empty = whole-run fallback.
  final List<int> xpos;

  bool get bold => (style & 1) != 0;
  bool get italic => (style & 2) != 0;

  Color get flutterColor {
    final c = color == 0 ? 0xFF000000 : color;
    if ((c >> 24) == 0) {
      return Color(0xFF000000 | (c & 0xFFFFFF));
    }
    return Color(c);
  }

  factory WcDrawText.fromJson(Map<String, dynamic> m) {
    final rawXpos = m['xpos'];
    final xpos = <int>[];
    if (rawXpos is List) {
      for (final v in rawXpos) {
        xpos.add(_i(v));
      }
    }
    return WcDrawText(
      page: _i(m['page']),
      x: _i(m['x']),
      y: _i(m['y']),
      text: _s(m['text']),
      fontId: _s(m['font_id']),
      sizeTwip: _i(m['size']),
      color: _i(m['color']),
      style: _i(m['style']),
      xpos: xpos,
    );
  }
}

class WcDrawRect extends WcRenderCommand {
  WcDrawRect({
    required this.page,
    required this.x,
    required this.y,
    required this.w,
    required this.h,
    required this.color,
    required this.kind,
  });

  @override
  final int page;
  final int x;
  final int y;
  final int w;
  final int h;
  final int color;
  final String kind;

  factory WcDrawRect.fromJson(Map<String, dynamic> m) => WcDrawRect(
        page: _i(m['page']),
        x: _i(m['x']),
        y: _i(m['y']),
        w: _i(m['w']),
        h: _i(m['h']),
        color: _i(m['color']),
        kind: _s(m['kind']),
      );
}

class WcDrawLine extends WcRenderCommand {
  WcDrawLine({
    required this.page,
    required this.x1,
    required this.y1,
    required this.x2,
    required this.y2,
    required this.kind,
    required this.color,
    required this.width,
  });

  @override
  final int page;
  final int x1;
  final int y1;
  final int x2;
  final int y2;
  final String kind;
  final int color;
  final int width;

  factory WcDrawLine.fromJson(Map<String, dynamic> m) => WcDrawLine(
        page: _i(m['page']),
        x1: _i(m['x1']),
        y1: _i(m['y1']),
        x2: _i(m['x2']),
        y2: _i(m['y2']),
        kind: _s(m['kind']),
        color: _i(m['color']),
        width: _i(m['width']),
      );
}

class PageMetrics {
  PageMetrics({
    required this.pageCount,
    required this.pageWidthTwip,
    required this.pageHeightTwip,
    required this.marginLeftTwip,
    required this.marginRightTwip,
    required this.marginTopTwip,
    required this.marginBottomTwip,
    required this.paperSize,
  });

  final int pageCount;
  final int pageWidthTwip;
  final int pageHeightTwip;
  final int marginLeftTwip;
  final int marginRightTwip;
  final int marginTopTwip;
  final int marginBottomTwip;
  final String paperSize;

  factory PageMetrics.fromJson(Map<String, dynamic> m) => PageMetrics(
        pageCount: _i(m['page_count']),
        pageWidthTwip: _i(m['page_width_twip']),
        pageHeightTwip: _i(m['page_height_twip']),
        marginLeftTwip: _i(m['margin_left_twip']),
        marginRightTwip: _i(m['margin_right_twip']),
        marginTopTwip: _i(m['margin_top_twip']),
        marginBottomTwip: _i(m['margin_bottom_twip']),
        paperSize: _s(m['paper_size']).isEmpty ? 'A4' : _s(m['paper_size']),
      );

  factory PageMetrics.defaultA4() => PageMetrics(
        pageCount: 1,
        pageWidthTwip: 11906,
        pageHeightTwip: 16838,
        marginLeftTwip: 1800,
        marginRightTwip: 1800,
        marginTopTwip: 1440,
        marginBottomTwip: 1440,
        paperSize: 'A4',
      );
}

class RenderEnvelope {
  RenderEnvelope({
    required this.ok,
    required this.pipeline,
    required this.page,
    required this.commands,
    this.rawJson = '',
  });

  final bool ok;
  final String pipeline;
  final PageMetrics page;
  final List<WcRenderCommand> commands;
  final String rawJson;

  List<WcDrawImage> get unsupportedImages => commands
      .whereType<WcDrawImage>()
      .where((image) => image.decodeError != null)
      .toList(growable: false);

  Future<void> decodeImages() async {
    for (final command in commands.whereType<WcDrawImage>()) {
      await command.decode();
    }
  }

  factory RenderEnvelope.fromJsonString(String json) {
    final root = jsonDecode(json) as Map<String, dynamic>;
    final pageMap = root['page'];
    final doc = root['document'];
    return RenderEnvelope(
      ok: root['ok'] == true,
      pipeline: root['pipeline']?.toString() ?? '',
      page: pageMap is Map
          ? PageMetrics.fromJson(Map<String, dynamic>.from(pageMap))
          : PageMetrics.defaultA4(),
      commands: doc is Map
          ? WcRenderCommand.fromDocumentJson(Map<String, dynamic>.from(doc))
          : const [],
      rawJson: json,
    );
  }
}

/// Java MetricsUtil 96dpi: `px = twip * zoom * (1.3333536 / 20)`.
double twipToPx(num twip, [double zoom = 1.0]) =>
    twip.toDouble() * zoom * (1.3333536 / 20.0);

double pxToTwip(num px, [double zoom = 1.0]) {
  final z = zoom == 0 ? 1.0 : zoom;
  return px.toDouble() / (z * (1.3333536 / 20.0));
}
