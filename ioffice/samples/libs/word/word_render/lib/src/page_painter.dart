import 'package:flutter/material.dart';

import 'command_executor.dart';
import 'render_command.dart';

/// Paints a single page into a canvas sized to that page in device pixels.
class PagePainter extends CustomPainter {
  PagePainter({
    required this.envelope,
    required this.pageIndex,
    required this.zoom,
    this.showGridlines = false,
    this.darkPage = false,
  });

  final RenderEnvelope envelope;
  final int pageIndex;
  final double zoom;
  final bool showGridlines;
  final bool darkPage;

  static const _page = Color(0xFFFFFFFF);
  static const _grid = Color(0x212B7CD3);
  static const _darkPage = Color(0xFF1E1E1E);
  static const _darkGrid = Color(0x21468EDB);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(
        Offset.zero & size, Paint()..color = darkPage ? _darkPage : _page);
    if (showGridlines) _paintGridlines(canvas, size);
    canvas.save();
    canvas.scale(zoom * (1.3333536 / 20.0));
    executeCommands(
      canvas,
      envelope.commands,
      page: pageIndex,
      darkPage: darkPage,
    );
    canvas.restore();
  }

  // GenOffice's .show-grid .doc-page uses a 24px alignment grid clipped to
  // the content box. Page margins come from the sole word-core layout model.
  void _paintGridlines(Canvas canvas, Size size) {
    final page = envelope.page;
    final area = Rect.fromLTRB(
      twipToPx(page.marginLeftTwip, zoom),
      twipToPx(page.marginTopTwip, zoom),
      size.width - twipToPx(page.marginRightTwip, zoom),
      size.height - twipToPx(page.marginBottomTwip, zoom),
    );
    if (area.isEmpty) return;
    final spacing = 24 * zoom;
    if (spacing <= 0) return;
    final paint = Paint()
      ..color = darkPage ? _darkGrid : _grid
      ..strokeWidth = zoom;
    canvas.save();
    canvas.clipRect(area);
    for (var x = area.left; x <= area.right; x += spacing) {
      canvas.drawLine(Offset(x, area.top), Offset(x, area.bottom), paint);
    }
    for (var y = area.top; y <= area.bottom; y += spacing) {
      canvas.drawLine(Offset(area.left, y), Offset(area.right, y), paint);
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(PagePainter old) =>
      old.zoom != zoom ||
      old.showGridlines != showGridlines ||
      old.darkPage != darkPage ||
      old.pageIndex != pageIndex ||
      !identical(old.envelope, envelope) ||
      old.envelope.pipeline != envelope.pipeline ||
      old.envelope.commands.length != envelope.commands.length;
}
