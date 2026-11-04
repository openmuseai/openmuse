import 'package:flutter/material.dart';

import 'command_executor.dart';
import 'render_command.dart';

class DocumentPainter extends CustomPainter {
  DocumentPainter({
    required this.envelope,
    required this.zoom,
  });

  final RenderEnvelope envelope;
  final double zoom;

  static const _pageGap = 24.0;
  static const _bg = Color(0xFFE8E8E8);
  static const _page = Color(0xFFFFFFFF);
  static const _shadow = Color(0x40000000);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(Offset.zero & size, Paint()..color = _bg);
    final p = envelope.page;
    final pageW = twipToPx(p.pageWidthTwip, zoom);
    final pageH = twipToPx(p.pageHeightTwip, zoom);
    for (var i = 0; i < p.pageCount; i++) {
      final pageX = ((size.width - pageW) / 2).clamp(8.0, size.width);
      final pageY = _pageGap + i * (pageH + _pageGap);
      final rect = Rect.fromLTWH(pageX, pageY, pageW, pageH);
      canvas.drawShadow(Path()..addRect(rect), _shadow, 4, false);
      canvas.drawRect(rect, Paint()..color = _page);
      // Commands are page-absolute twip. One scale, then draw in twip.
      canvas.save();
      canvas.translate(pageX, pageY);
      canvas.scale(zoom * (1.3333536 / 20.0));
      executeCommands(
        canvas,
        envelope.commands,
        page: i,
      );
      canvas.restore();
    }
  }

  @override
  bool shouldRepaint(DocumentPainter old) =>
      old.zoom != zoom ||
      old.envelope.pipeline != envelope.pipeline ||
      old.envelope.commands.length != envelope.commands.length ||
      old.envelope.page.pageCount != envelope.page.pageCount;
}
