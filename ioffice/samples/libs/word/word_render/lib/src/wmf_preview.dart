import 'dart:convert';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:flutter/painting.dart';

import 'wmf_charset.dart';

// Product Canvas adapter for the Java picparser record path:
// WMFParser.readMetaFileHeader/readRecord -> RecordMaps.getWmfRecordMap ->
// WMF_*.read/render -> MetaFilePainter.paint -> GdiPainter/GraphicsDrawer.
// Only records with a real implementation below are accepted. An unfamiliar
// record fails parsing instead of silently dropping a document element.
class WmfPreview {
  WmfPreview._(this.width, this.height, this.operations, this.recordIds);

  final double width;
  final double height;
  final List<_WmfOperation> operations;
  final Set<int> recordIds;

  int get operationCount => operations.length;

  static WmfPreview parse(Uint8List bytes) {
    final data = ByteData.sublistView(bytes);
    if (bytes.length < 18) throw const FormatException('WMF header truncated');
    final placeable = data.getUint32(0, Endian.little) == 0x9ac6cdd7;
    final header = placeable ? 22 : 0;
    if (bytes.length < header + 18) {
      throw const FormatException('WMF standard header truncated');
    }
    if (data.getUint16(header, Endian.little) != 1 ||
        data.getUint16(header + 2, Endian.little) != 9) {
      throw const FormatException('unsupported WMF header');
    }
    final handles = data.getUint16(header + 10, Endian.little);
    final objects = List<Object?>.filled(handles, null);
    final operations = <_WmfOperation>[];
    final ids = <int>{};
    var windowWidth = placeable
        ? (data.getInt16(12, Endian.little) - data.getInt16(6, Endian.little))
            .abs()
        : 0;
    var windowHeight = placeable
        ? (data.getInt16(14, Endian.little) - data.getInt16(8, Endian.little))
            .abs()
        : 0;
    var originX = 0;
    var originY = 0;
    var textAlign = 0;
    var textColor = const ui.Color(0xff000000);
    var pen = const _WmfPen(0xff000000, 1);
    _WmfFont? font;
    int? moveX;
    int? moveY;
    var offset = header + 18;
    var ended = false;
    while (offset + 6 <= bytes.length) {
      final words = data.getUint32(offset, Endian.little);
      final length = words * 2;
      if (length < 6 || offset + length > bytes.length) {
        throw const FormatException('invalid WMF record length');
      }
      final id = data.getUint16(offset + 4, Endian.little);
      ids.add(id);
      final body = offset + 6;
      final bodyLength = length - 6;
      void require(int count) {
        if (bodyLength < count) {
          throw FormatException(
              'WMF record 0x${id.toRadixString(16)} truncated');
        }
      }

      int u16(int at) => data.getUint16(body + at, Endian.little);
      int i16(int at) => data.getInt16(body + at, Endian.little);
      ui.Color rgb(int at) => ui.Color(0xff000000 |
          (bytes[body + at] << 16) |
          (bytes[body + at + 1] << 8) |
          bytes[body + at + 2]);
      void addObject(Object value) {
        final slot = objects.indexOf(null);
        if (slot < 0) throw const FormatException('WMF handle table full');
        objects[slot] = value;
      }

      switch (id) {
        case 0x0000: // Java WMF_EOF.render
          ended = true;
          break;
        case 0x0209: // Java WMF_SetTextColor.read/render
          require(4);
          textColor = rgb(0);
          break;
        case 0x0102: // Java WMF_SetBkMode.read/render
        case 0x0201: // Java WMF_SetBkColor.read/render
          // Background is not painted for this corpus: all records set
          // TRANSPARENT (1). Reject OPAQUE rather than hiding the difference.
          require(id == 0x0102 ? 2 : 4);
          if (id == 0x0102 && u16(0) != 1) {
            throw const FormatException(
                'opaque WMF text background unsupported');
          }
          break;
        case 0x012e: // Java WMF_SetTextAlign.read/render
          require(2);
          textAlign = u16(0);
          break;
        case 0x020b: // Java WMF_SetWindowOrg.read/render
          require(4);
          originY = i16(0);
          originX = i16(2);
          break;
        case 0x020c: // Java WMF_SetWindowExt.read/render
          require(4);
          windowHeight = i16(0).abs();
          windowWidth = i16(2).abs();
          break;
        case 0x02fb: // Java WMF_CreateFont_Indirect -> LogFont
          require(18);
          final familyBytes = bytes.sublist(body + 18, body + bodyLength);
          final zero = familyBytes.indexOf(0);
          final family = decodeGbk(Uint8List.fromList(
              zero < 0 ? familyBytes : familyBytes.sublist(0, zero)));
          addObject(_WmfFont(
            family: family,
            charset: bytes[body + 13],
            height: i16(0),
            width: i16(2),
            bold: i16(8) >= 700,
            italic: bytes[body + 10] != 0,
          ));
          break;
        case 0x02fa: // Java WMF_CreatePenIndirect -> WMF_Pen
          require(10);
          addObject(_WmfPen(rgb(6).toARGB32(), i16(2).abs().toDouble(),
              nullPen: (u16(0) & 255) == 5));
          break;
        case 0x012d: // Java WMF_SelectObject.render
          require(2);
          final index = u16(0);
          if (index < objects.length) {
            final value = objects[index];
            if (value is _WmfFont) font = value;
            if (value is _WmfPen) pen = value;
          }
          break;
        case 0x01f0: // Java WMF_DeleteObject.render
          require(2);
          final index = u16(0);
          if (index < objects.length) objects[index] = null;
          break;
        case 0x0a32: // Java WMF_ExtTextOut.read/render
          require(8);
          final y = i16(0);
          final x = i16(2);
          final count = u16(4);
          final options = u16(6);
          final textStart = body + 8 + ((options & 0x0006) != 0 ? 8 : 0);
          if (textStart + count > body + bodyLength || font == null) {
            throw const FormatException('WMF text/font record incomplete');
          }
          final raw = bytes.sublist(textStart, textStart + count);
          final text = _decodeWmfText(raw, font.family, font.charset);
          final widthsStart = textStart + count + (count & 1);
          final widths = <double>[];
          if (widthsStart + count * 2 <= body + bodyLength) {
            for (var i = 0; i < count; i++) {
              widths.add(data
                  .getUint16(widthsStart + i * 2, Endian.little)
                  .toDouble());
            }
          }
          operations.add(_WmfText(text, x - originX, y - originY, font,
              textColor, textAlign, widths));
          break;
        case 0x0214: // Java WMF_MoveTo.read/render
          require(4);
          moveY = i16(0) - originY;
          moveX = i16(2) - originX;
          break;
        case 0x0213: // Java WMF_LineTo.read/render
          require(4);
          final y = i16(0) - originY;
          final x = i16(2) - originX;
          if (moveX != null && moveY != null && !pen.nullPen) {
            operations.add(_WmfLine(moveX, moveY, x, y, pen));
          }
          moveX = x;
          moveY = y;
          break;
        case 0x0626: // Java WMFParser.readRecord(false): defaultRecord
          // These escape records carry non-rendering OLE preview metadata.
          break;
        default:
          throw FormatException(
              'WMF record 0x${id.toRadixString(16)} unsupported');
      }
      offset += length;
      if (ended) break;
    }
    if (!ended || windowWidth <= 1 || windowHeight <= 1 || operations.isEmpty) {
      throw const FormatException('WMF has no complete drawable preview');
    }
    return WmfPreview._(windowWidth.toDouble(), windowHeight.toDouble(),
        List.unmodifiable(operations), Set.unmodifiable(ids));
  }

  void paint(
    ui.Canvas canvas,
    ui.Rect target, {
    double cropLeft = 0,
    double cropTop = 0,
    double cropRight = 0,
    double cropBottom = 0,
  }) {
    final visibleWidth = width * (1 - cropLeft - cropRight);
    final visibleHeight = height * (1 - cropTop - cropBottom);
    if (visibleWidth <= 0 || visibleHeight <= 0) return;
    final scaleX = target.width / visibleWidth;
    final scaleY = target.height / visibleHeight;
    canvas.save();
    canvas.clipRect(target);
    canvas.translate(target.left - width * cropLeft * scaleX,
        target.top - height * cropTop * scaleY);
    canvas.scale(scaleX, scaleY);
    for (final operation in operations) {
      operation.paint(canvas);
    }
    canvas.restore();
  }
}

String _decodeWmfText(Uint8List raw, String family, int charset) {
  if (family.toLowerCase() == 'symbol' || charset == 2) {
    // Java SymbolConverter chooses the native Symbol face using U+F000 PUA.
    // Bundled Flutter fonts use visible Unicode glyphs from the same encoding.
    return String.fromCharCodes(
        raw.map((byte) => symbolToUnicode[byte] ?? byte));
  }
  if (raw.length == 1 && raw[0] == 0x88) return 'ˆ'; // Java needRevise.
  if (charset == 134 || ((charset == 0 || charset == 1) && looksLikeGbk(raw))) {
    return decodeGbk(raw);
  }
  if (charset == 0 || charset == 1) return latin1.decode(raw);
  if (raw.every((byte) => byte < 0x80)) return ascii.decode(raw);
  throw FormatException('WMF charset $charset unsupported');
}

abstract class _WmfOperation {
  void paint(ui.Canvas canvas);
}

class _WmfFont {
  const _WmfFont(
      {required this.family,
      required this.charset,
      required this.height,
      required this.width,
      required this.bold,
      required this.italic});
  final String family;
  final int charset;
  final int height, width;
  final bool bold, italic;

  double get size {
    // Java LogFont stores the integer result of width * 96 / 72.
    if (width > 0) return (width * 96 ~/ 72).toDouble();
    final h = height.abs();
    // Java GDIFont.getFontSize keeps Chinese font height in WMF units below
    // 1024; Symbol also keeps its height below 700. Other faces convert from
    // device pixels with integer division.
    if ((family == 'Symbol' && h < 700) ||
        ((family.startsWith('宋体') ||
                family.startsWith('楷体') ||
                family.startsWith('微软雅黑')) &&
            h < 1024)) {
      return h.toDouble();
    }
    return (h * 72 ~/ 96).toDouble();
  }
}

class _WmfPen {
  const _WmfPen(this.color, this.width, {this.nullPen = false});
  final int color;
  final double width;
  final bool nullPen;
}

class _WmfLine extends _WmfOperation {
  _WmfLine(this.x1, this.y1, this.x2, this.y2, this.pen);
  final int x1, y1, x2, y2;
  final _WmfPen pen;

  @override
  void paint(ui.Canvas canvas) {
    canvas.drawLine(
        ui.Offset(x1.toDouble(), y1.toDouble()),
        ui.Offset(x2.toDouble(), y2.toDouble()),
        ui.Paint()
          ..color = ui.Color(pen.color)
          ..strokeWidth = pen.width <= 0 ? 1 : pen.width);
  }
}

class _WmfText extends _WmfOperation {
  _WmfText(this.text, this.x, this.y, this.font, this.color, this.align,
      this.widths);
  final String text;
  final int x, y, align;
  final _WmfFont font;
  final ui.Color color;
  final List<double> widths;

  // Java WMF_ExtTextOut.parserTextAndWidth converts byte advances to the
  // decoded character count before GraphicsDrawer.drawPosText uses them.
  List<double>? get _characterWidths {
    final count = text.length;
    if (widths.length == count) return widths;
    if (widths.length == count * 2) {
      return List.generate(count, (i) => widths[i * 2] + widths[i * 2 + 1]);
    }
    if (widths.length > count && widths.length < count * 2) {
      final nonzero = widths.where((width) => width > 0).take(count).toList();
      return [...nonzero, ...List<double>.filled(count - nonzero.length, 0)];
    }
    return null;
  }

  @override
  void paint(ui.Canvas canvas) {
    final style = TextStyle(
      fontFamily: font.family == 'Symbol' ? 'CJKFallback' : 'WcDefault',
      fontSize: font.size,
      fontWeight: font.bold ? FontWeight.w700 : FontWeight.w400,
      fontStyle: font.italic ? FontStyle.italic : FontStyle.normal,
      color: color,
    );
    final painter = TextPainter(
      text: TextSpan(text: text, style: style),
      textDirection: TextDirection.ltr,
      maxLines: 1,
    );
    painter.layout(minWidth: 0, maxWidth: double.infinity);
    final baseline =
        painter.computeDistanceToActualBaseline(TextBaseline.alphabetic);
    var drawX = x.toDouble();
    if ((align & 6) == 6) drawX -= painter.width / 2;
    if ((align & 6) == 2) drawX -= painter.width;
    // Java GraphicsDrawer.calculateAndConcatFont converts the WMF text
    // reference point to an Android Canvas baseline. Flutter's TextPainter
    // expects its top edge, so subtract its measured alphabetic baseline.
    final double baselineY;
    if ((align & 24) == 24) {
      baselineY = y.toDouble();
    } else if ((align & 8) == 8) {
      baselineY = y - (9 * font.size / 30).truncateToDouble();
    } else if ((align & 6) == 6) {
      baselineY = y + (27 * font.size / 30).truncateToDouble();
    } else {
      baselineY = y + font.size;
    }
    final drawY = baselineY - baseline;
    final advances = (align & 6) == 0 ? _characterWidths : null;
    if (advances == null) {
      painter.paint(canvas, ui.Offset(drawX, drawY));
    } else {
      var characterX = drawX;
      for (var i = 0; i < text.length; i++) {
        final character = TextPainter(
          text: TextSpan(text: text[i], style: style),
          textDirection: TextDirection.ltr,
          maxLines: 1,
        );
        character.layout(minWidth: 0, maxWidth: double.infinity);
        character.paint(canvas, ui.Offset(characterX, drawY));
        characterX += advances[i];
        character.dispose();
      }
    }
    painter.dispose();
  }
}
