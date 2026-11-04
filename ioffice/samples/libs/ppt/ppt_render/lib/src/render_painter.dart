import 'dart:developer' as developer;
import 'dart:ui' as ui;

import 'package:flutter/material.dart';

/// Consumes ppt-core `RenderCommand` JSON (`#[serde(tag = "type")]`).
class RenderPainter extends CustomPainter {
  RenderPainter(this.commands);

  final List<Map<String, dynamic>> commands;
  final List<String> ops = <String>[];
  final List<String> skipped = <String>[];

  factory RenderPainter.fromRoot(dynamic root) {
    return RenderPainter(parseCommands(root));
  }

  static List<Map<String, dynamic>> parseCommands(dynamic root) {
    if (root is List) {
      return root
          .whereType<Map>()
          .map((e) => Map<String, dynamic>.from(e))
          .toList();
    }
    if (root is Map && root['commands'] is List) {
      return (root['commands'] as List)
          .whereType<Map>()
          .map((e) => Map<String, dynamic>.from(e))
          .toList();
    }
    return const [];
  }

  static Color? argb(dynamic v) {
    if (v == null) return null;
    final n = v is int ? v : int.tryParse('$v');
    if (n == null) return null;
    return Color(n);
  }

  @override
  void paint(Canvas canvas, Size size) {
    ops.clear();
    skipped.clear();
    for (final cmd in commands) {
      final type = cmd['type']?.toString() ?? '';
      switch (type) {
        case 'DrawRect':
          _drawRect(canvas, cmd);
          ops.add('DrawRect');
          break;
        case 'DrawPath':
          _drawPath(canvas, cmd);
          ops.add('DrawPath');
          break;
        case 'DrawText':
          _drawText(canvas, cmd);
          ops.add('DrawText:${cmd['text'] ?? ''}');
          break;
        case 'DrawImage':
          _drawImage(canvas, cmd);
          ops.add('DrawImage');
          break;
        default:
          skipped.add(type);
          developer.log('skip unknown RenderCommand type=$type', name: 'ppt_render');
          debugPrint('skip unknown RenderCommand type=$type');
          break;
      }
    }
  }

  void _paintPair(Canvas canvas, void Function(Paint p) draw, Color? fill, Color? stroke) {
    if (fill != null) {
      draw(Paint()
        ..style = PaintingStyle.fill
        ..color = fill);
    }
    if (stroke != null) {
      draw(Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = stroke);
    }
    if (fill == null && stroke == null) {
      skipped.add('emptyStyle');
      return;
    }
  }

  void _drawRect(Canvas canvas, Map<String, dynamic> cmd) {
    final x = (cmd['x'] as num?)?.toDouble() ?? 0;
    final y = (cmd['y'] as num?)?.toDouble() ?? 0;
    final w = (cmd['w'] as num?)?.toDouble() ?? 0;
    final h = (cmd['h'] as num?)?.toDouble() ?? 0;
    final rect = Rect.fromLTWH(x, y, w, h);
    _paintPair(canvas, (p) => canvas.drawRect(rect, p), argb(cmd['fill']), argb(cmd['stroke']));
  }

  void _drawPath(Canvas canvas, Map<String, dynamic> cmd) {
    final raw = cmd['path'];
    if (raw is! List || raw.isEmpty) return;
    final path = Path();
    var first = true;
    for (final pt in raw) {
      if (pt is! List || pt.length < 2) continue;
      final x = (pt[0] as num).toDouble();
      final y = (pt[1] as num).toDouble();
      if (first) {
        path.moveTo(x, y);
        first = false;
      } else {
        path.lineTo(x, y);
      }
    }
    if (raw.length >= 3) {
      path.close();
    }
    _paintPair(canvas, (p) => canvas.drawPath(path, p), argb(cmd['fill']), argb(cmd['stroke']));
  }

  void _drawText(Canvas canvas, Map<String, dynamic> cmd) {
    final text = cmd['text']?.toString() ?? '';
    final x = (cmd['x'] as num?)?.toDouble() ?? 0;
    final y = (cmd['y'] as num?)?.toDouble() ?? 0;
    final size = (cmd['size'] as num?)?.toDouble() ?? 14;
    final font = cmd['font'] as String?;
    final bold = cmd['bold'] == true;
    final italic = cmd['italic'] == true;
    final color = argb(cmd['color']) ?? const Color(0xFF000000);
    final tp = TextPainter(
      text: TextSpan(
        text: text,
        style: TextStyle(
          fontFamily: font,
          fontSize: size,
          color: color,
          fontWeight: bold ? FontWeight.bold : FontWeight.normal,
          fontStyle: italic ? FontStyle.italic : FontStyle.normal,
        ),
      ),
      textDirection: ui.TextDirection.ltr,
    )..layout();
    tp.paint(canvas, Offset(x, y));
  }

  void _drawImage(Canvas canvas, Map<String, dynamic> cmd) {
    final x = (cmd['x'] as num?)?.toDouble() ?? 0;
    final y = (cmd['y'] as num?)?.toDouble() ?? 0;
    final w = (cmd['w'] as num?)?.toDouble() ?? 0;
    final h = (cmd['h'] as num?)?.toDouble() ?? 0;
    final rect = Rect.fromLTWH(x, y, w, h);
    final b64 = cmd['bytes_b64']?.toString();
    if (b64 == null || b64.isEmpty) {
      canvas.drawRect(
        rect,
        Paint()
          ..style = PaintingStyle.stroke
          ..color = const Color(0xFF888888),
      );
      return;
    }
    canvas.drawRect(
      rect,
      Paint()
        ..style = PaintingStyle.fill
        ..color = const Color(0xFFCCCCCC),
    );
  }

  @override
  bool shouldRepaint(covariant RenderPainter oldDelegate) =>
      oldDelegate.commands != commands;
}
