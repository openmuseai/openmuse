import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'sheet_axis_geometry.dart';
import 'visual_geometry.dart';

/// Paints engine-provided workbook visuals over the scrolling cell viewport.
/// The signed product controls commands; GridCraft owns chart data and layout.
final class WorkbookVisualLayer extends StatefulWidget {
  const WorkbookVisualLayer({
    super.key,
    required this.visuals,
    required this.imageBytes,
    required this.scrollX,
    required this.scrollY,
    required this.rows,
    required this.cols,
    required this.zoomFactor,
    required this.selectedId,
    required this.selectedKind,
    required this.editable,
    required this.onSelect,
    required this.onCommit,
    required this.onDelete,
    required this.onClearSelection,
  });

  final List<dynamic> visuals;
  final Map<int, Uint8List> imageBytes;
  final double scrollX;
  final double scrollY;
  final SheetAxisGeometry rows;
  final SheetAxisGeometry cols;
  final double zoomFactor;
  final int? selectedId;
  final String? selectedKind;
  final bool editable;
  final void Function(int id, String kind) onSelect;
  final void Function(
    int id,
    String kind,
    Offset delta,
    VisualResizeHandle? handle,
  )
  onCommit;
  final void Function(int id, String kind) onDelete;
  final VoidCallback onClearSelection;

  @override
  State<WorkbookVisualLayer> createState() => _WorkbookVisualLayerState();
}

final class _WorkbookVisualLayerState extends State<WorkbookVisualLayer> {
  static const _pointsToPixels = 4.0 / 3.0;
  final FocusNode _focusNode = FocusNode();
  int? _dragId;
  String? _dragKind;
  Offset _dragDelta = Offset.zero;
  Offset? _dragStartGlobal;
  Offset? _dragStartLocal;
  VisualResizeHandle? _resizeHandle;

  List<dynamic> get visuals => widget.visuals;
  Map<int, Uint8List> get imageBytes => widget.imageBytes;
  double get scrollX => widget.scrollX;
  double get scrollY => widget.scrollY;
  SheetAxisGeometry get rows => widget.rows;
  SheetAxisGeometry get cols => widget.cols;
  double get zoomFactor => widget.zoomFactor;
  int? get selectedId => widget.selectedId;

  @override
  void dispose() {
    _focusNode.dispose();
    super.dispose();
  }

  KeyEventResult _onKeyEvent(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent ||
        selectedId == null ||
        widget.selectedKind == null) {
      return KeyEventResult.ignored;
    }
    if (widget.editable &&
        (event.logicalKey == LogicalKeyboardKey.delete ||
            event.logicalKey == LogicalKeyboardKey.backspace)) {
      widget.onDelete(selectedId!, widget.selectedKind!);
      return KeyEventResult.handled;
    }
    if (event.logicalKey == LogicalKeyboardKey.escape) {
      widget.onClearSelection();
      return KeyEventResult.handled;
    }
    return KeyEventResult.ignored;
  }

  @override
  Widget build(BuildContext context) => Focus(
    focusNode: _focusNode,
    onKeyEvent: _onKeyEvent,
    child: LayoutBuilder(
      builder: (context, size) => Stack(
        clipBehavior: Clip.hardEdge,
        children: [
          for (final raw in visuals)
            if (raw is Map && {'chart', 'shape', 'image'}.contains(raw['kind']))
              _visual(
                raw,
                size.maxWidth,
                size.maxHeight,
                Theme.of(context).textTheme.bodyMedium?.fontFamily,
              ),
        ],
      ),
    ),
  );

  Widget _visual(
    Map<dynamic, dynamic> visual,
    double viewportWidth,
    double viewportHeight,
    String? fontFamily,
  ) {
    final anchor = visual['anchor'];
    if (anchor is! Map) return const SizedBox.shrink();
    final kind = visual['kind'] as String;
    final original = VisualGeometry.frame(
      visual,
      rows: rows,
      cols: cols,
      zoomFactor: zoomFactor,
    );
    final frame = kind == 'shape' && visual['shapeKind'] == 'Line'
        ? VisualFrame(
            original.left,
            original.top,
            original.width,
            math.max(4.0, original.height),
          )
        : original;
    final width = frame.width;
    final height = frame.height;
    final left = frame.left - scrollX;
    final top = frame.top - scrollY;
    if (!width.isFinite ||
        !height.isFinite ||
        !left.isFinite ||
        !top.isFinite ||
        width <= 0 ||
        height <= 0 ||
        left >= viewportWidth ||
        top >= viewportHeight ||
        left + width <= 0 ||
        top + height <= 0) {
      return const SizedBox.shrink();
    }
    final id = (visual['id'] as num).toInt();
    final dragging = _dragId == id && _dragKind == kind;
    final offset = dragging ? _dragDelta : Offset.zero;
    final preview = dragging && _resizeHandle != null
        ? VisualGeometry.resizedFrame(
            frame,
            _resizeHandle!,
            deltaX: offset.dx,
            deltaY: offset.dy,
            zoomFactor: zoomFactor,
          )
        : frame;
    final displayWidth = preview.width;
    final displayHeight = preview.height;
    final selected = selectedId == id && widget.selectedKind == kind;
    return Positioned(
      left:
          preview.left -
          scrollX +
          (dragging && _resizeHandle == null ? offset.dx : 0),
      top:
          preview.top -
          scrollY +
          (dragging && _resizeHandle == null ? offset.dy : 0),
      width: displayWidth,
      height: displayHeight,
      child: GestureDetector(
        key: ValueKey('sheet-visual-$kind-$id'),
        onTap: () {
          _focusNode.requestFocus();
          widget.onSelect(id, kind);
        },
        onPanDown: widget.editable
            ? (details) {
                _dragStartGlobal = details.globalPosition;
                _dragStartLocal = details.localPosition;
              }
            : null,
        onPanStart: widget.editable
            ? (details) {
                _dragStartGlobal ??= details.globalPosition;
                _dragStartLocal ??= details.localPosition;
                _focusNode.requestFocus();
                widget.onSelect(id, kind);
                setState(() {
                  _dragId = id;
                  _dragKind = kind;
                  _dragDelta = Offset.zero;
                  _resizeHandle = selected
                      ? _hitHandle(_dragStartLocal!, width, height)
                      : null;
                });
              }
            : null,
        onPanUpdate: widget.editable
            ? (details) => setState(
                () => _dragDelta = details.globalPosition - _dragStartGlobal!,
              )
            : null,
        onPanEnd: widget.editable ? (_) => _finishDrag() : null,
        onPanCancel: widget.editable ? _cancelDrag : null,
        child: DecoratedBox(
          decoration: BoxDecoration(
            color: kind == 'chart' ? Colors.white : Colors.transparent,
            border: Border.all(
              color: selected
                  ? const Color(0xff156082)
                  : kind == 'chart'
                  ? const Color(0xffc9c9c9)
                  : Colors.transparent,
              width: selected ? 2 : 1,
            ),
            boxShadow: kind == 'chart'
                ? const [
                    BoxShadow(
                      color: Color(0x33000000),
                      blurRadius: 5,
                      offset: Offset(1, 2),
                    ),
                  ]
                : null,
          ),
          child: Stack(
            fit: StackFit.expand,
            children: [
              switch (kind) {
                'chart' =>
                  visual['tooComplex'] == true
                      ? Center(child: Text(visual['title'] as String? ?? '图表'))
                      : CustomPaint(
                          painter: _ChartPainter(
                            visual['primitives'] as List? ?? const [],
                            fontFamily,
                            zoomFactor,
                          ),
                          size: Size(displayWidth, displayHeight),
                        ),
                'image' => _image(visual, id),
                _ => _shape(visual, displayWidth, displayHeight, fontFamily),
              },
              if (selected && widget.editable)
                for (final handle in VisualResizeHandle.values)
                  IgnorePointer(
                    child: Align(
                      alignment: _alignment(handle),
                      child: Container(
                        width: 9,
                        height: 9,
                        decoration: const BoxDecoration(
                          color: Color(0xffffffff),
                          shape: BoxShape.circle,
                          border: Border.fromBorderSide(
                            BorderSide(color: Color(0xff156082), width: 1.5),
                          ),
                        ),
                      ),
                    ),
                  ),
            ],
          ),
        ),
      ),
    );
  }

  Alignment _alignment(VisualResizeHandle handle) => switch (handle) {
    VisualResizeHandle.nw => Alignment.topLeft,
    VisualResizeHandle.n => Alignment.topCenter,
    VisualResizeHandle.ne => Alignment.topRight,
    VisualResizeHandle.e => Alignment.centerRight,
    VisualResizeHandle.se => Alignment.bottomRight,
    VisualResizeHandle.s => Alignment.bottomCenter,
    VisualResizeHandle.sw => Alignment.bottomLeft,
    VisualResizeHandle.w => Alignment.centerLeft,
  };

  VisualResizeHandle? _hitHandle(Offset point, double width, double height) {
    VisualResizeHandle? nearest;
    var distance = 14.0;
    for (final handle in VisualResizeHandle.values) {
      final x = handle.west
          ? 4.5
          : handle.east
          ? width - 4.5
          : width / 2;
      final y = handle.north
          ? 4.5
          : handle.south
          ? height - 4.5
          : height / 2;
      final next = (point - Offset(x, y)).distance;
      if (next <= distance) {
        distance = next;
        nearest = handle;
      }
    }
    return nearest;
  }

  void _finishDrag() {
    final id = _dragId;
    final kind = _dragKind;
    final delta = _dragDelta;
    final handle = _resizeHandle;
    _cancelDrag();
    if (id != null && kind != null && delta.distance > 2) {
      widget.onCommit(id, kind, delta, handle);
    }
  }

  void _cancelDrag() {
    setState(() {
      _dragId = null;
      _dragKind = null;
      _dragDelta = Offset.zero;
      _dragStartGlobal = null;
      _dragStartLocal = null;
      _resizeHandle = null;
    });
  }

  Widget _image(Map<dynamic, dynamic> visual, int id) {
    final data = imageBytes[id];
    if (data == null) {
      return ColoredBox(
        color: const Color(0xfff4f4f4),
        child: Center(child: Text(visual['alt'] as String? ?? '图片不可预览')),
      );
    }
    return Image.memory(
      data,
      fit: BoxFit.fill,
      gaplessPlayback: true,
      filterQuality: FilterQuality.medium,
      errorBuilder: (context, error, stack) =>
          const Center(child: Text('图片不可预览')),
    );
  }

  Widget _shape(
    Map<dynamic, dynamic> visual,
    double width,
    double height,
    String? fontFamily,
  ) {
    const supported = {
      'Rectangle',
      'RoundedRectangle',
      'Ellipse',
      'Triangle',
      'Line',
      'Arrow',
      'TextBox',
    };
    if (!supported.contains(visual['shapeKind'])) {
      return const ColoredBox(
        color: Color(0xffeeeeee),
        child: Center(child: Text('形状不可预览')),
      );
    }
    final format = visual['format'] is Map ? visual['format'] as Map : const {};
    final text = visual['text'] as String? ?? '';
    final align = switch (format['align']) {
      'Right' => TextAlign.right,
      'Center' => TextAlign.center,
      _ => TextAlign.left,
    };
    final alignment = switch (format['anchor']) {
      'Top' => Alignment.topCenter,
      'Bottom' => Alignment.bottomCenter,
      _ => Alignment.center,
    };
    return CustomPaint(
      painter: _ShapePainter(visual, zoomFactor),
      child: SizedBox(
        width: width,
        height: height,
        child: text.isEmpty
            ? null
            : Align(
                alignment: alignment,
                child: Padding(
                  padding: EdgeInsets.all(5 * zoomFactor),
                  child: Text(
                    text,
                    maxLines: 6,
                    overflow: TextOverflow.clip,
                    textAlign: align,
                    style: TextStyle(
                      fontFamily: format['font'] as String? ?? fontFamily,
                      color: _hex(format['textColor']) ?? Colors.black,
                      fontSize:
                          ((format['fontSize'] as num?)?.toDouble() ?? 11) *
                          _pointsToPixels *
                          zoomFactor,
                      fontWeight: format['bold'] == true
                          ? FontWeight.bold
                          : FontWeight.normal,
                      fontStyle: format['italic'] == true
                          ? FontStyle.italic
                          : FontStyle.normal,
                    ),
                  ),
                ),
              ),
      ),
    );
  }
}

Color? _hex(Object? value) {
  if (value is! String || !RegExp(r'^#[0-9A-Fa-f]{6}$').hasMatch(value))
    return null;
  return Color(int.parse(value.substring(1), radix: 16) | 0xff000000);
}

final class _ShapePainter extends CustomPainter {
  const _ShapePainter(this.visual, this.zoomFactor);
  final Map<dynamic, dynamic> visual;
  final double zoomFactor;

  @override
  void paint(Canvas canvas, Size size) {
    final kind = visual['shapeKind'];
    final fill = _hex(visual['fillColor']);
    final format = visual['format'] is Map ? visual['format'] as Map : const {};
    final line = format['noLine'] == true ? null : _hex(visual['lineColor']);
    final rect = Offset.zero & size;
    final path = Path();
    switch (kind) {
      case 'Ellipse':
        path.addOval(rect);
      case 'RoundedRectangle':
        path.addRRect(
          RRect.fromRectAndRadius(
            rect,
            Radius.circular(math.min(size.width, size.height) * .12),
          ),
        );
      case 'Triangle':
        path.moveTo(size.width / 2, 0);
        path.lineTo(size.width, size.height);
        path.lineTo(0, size.height);
        path.close();
      case 'Line':
        path.moveTo(0, size.height / 2);
        path.lineTo(size.width, size.height / 2);
      case 'Arrow':
        path.moveTo(0, size.height * .3);
        path.lineTo(size.width * .65, size.height * .3);
        path.lineTo(size.width * .65, 0);
        path.lineTo(size.width, size.height / 2);
        path.lineTo(size.width * .65, size.height);
        path.lineTo(size.width * .65, size.height * .7);
        path.lineTo(0, size.height * .7);
        path.close();
      default:
        path.addRect(rect);
    }
    if (fill != null && kind != 'Line')
      canvas.drawPath(path, Paint()..color = fill);
    if (line != null)
      canvas.drawPath(
        path,
        Paint()
          ..color = line
          ..style = PaintingStyle.stroke
          ..strokeWidth = 1.3 * zoomFactor
          ..isAntiAlias = true,
      );
  }

  @override
  bool shouldRepaint(covariant _ShapePainter oldDelegate) =>
      oldDelegate.visual != visual || oldDelegate.zoomFactor != zoomFactor;
}

final class _ChartPainter extends CustomPainter {
  const _ChartPainter(this.primitives, this.fontFamily, this.zoomFactor);
  final List<dynamic> primitives;
  final String? fontFamily;
  final double zoomFactor;

  Color _color(Object? value) {
    if (value is! List || value.length < 4) return Colors.transparent;
    return Color.fromARGB(
      (value[3] as num).toInt(),
      (value[0] as num).toInt(),
      (value[1] as num).toInt(),
      (value[2] as num).toInt(),
    );
  }

  Paint _paint(Object? color, {double width = 1, bool stroke = false}) =>
      Paint()
        ..color = _color(color)
        ..style = stroke ? PaintingStyle.stroke : PaintingStyle.fill
        ..strokeWidth = width
        ..isAntiAlias = true;

  void _stroke(Canvas canvas, Path path, Object? value) {
    if (value is List && value.length >= 2) {
      canvas.drawPath(
        path,
        _paint(value[0], width: (value[1] as num).toDouble(), stroke: true),
      );
    }
  }

  double _n(Map<dynamic, dynamic> body, String key) =>
      (body[key] as num?)?.toDouble() ?? 0;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();
    canvas.scale(4 / 3 * zoomFactor);
    for (final raw in primitives) {
      if (raw is! Map || raw.isEmpty) continue;
      final kind = raw.keys.first;
      final body = raw[kind];
      if (body is! Map) continue;
      switch (kind) {
        case 'Rect':
          final rect = RRect.fromRectAndRadius(
            Rect.fromLTWH(
              _n(body, 'x'),
              _n(body, 'y'),
              _n(body, 'w'),
              _n(body, 'h'),
            ),
            Radius.circular(_n(body, 'radius')),
          );
          if (body['fill'] != null)
            canvas.drawRRect(rect, _paint(body['fill']));
          final stroke = body['stroke'];
          if (stroke is List && stroke.length >= 2) {
            canvas.drawRRect(
              rect,
              _paint(
                stroke[0],
                width: (stroke[1] as num).toDouble(),
                stroke: true,
              ),
            );
          }
        case 'Line':
          final pts = body['pts'];
          if (pts is! List || pts.length < 2) break;
          final path = Path();
          for (var i = 0; i < pts.length; i++) {
            final p = pts[i] as List;
            final x = (p[0] as num).toDouble();
            final y = (p[1] as num).toDouble();
            if (i == 0) {
              path.moveTo(x, y);
            } else {
              path.lineTo(x, y);
            }
          }
          canvas.drawPath(
            path,
            _paint(body['color'], width: _n(body, 'width'), stroke: true),
          );
        case 'Polygon':
          final pts = body['pts'];
          if (pts is! List || pts.isEmpty) break;
          final path = Path();
          for (var i = 0; i < pts.length; i++) {
            final p = pts[i] as List;
            if (i == 0) {
              path.moveTo((p[0] as num).toDouble(), (p[1] as num).toDouble());
            } else {
              path.lineTo((p[0] as num).toDouble(), (p[1] as num).toDouble());
            }
          }
          path.close();
          canvas.drawPath(path, _paint(body['fill']));
          _stroke(canvas, path, body['stroke']);
        case 'Wedge':
          final cx = _n(body, 'cx');
          final cy = _n(body, 'cy');
          final outer = _n(body, 'r_outer');
          final inner = _n(body, 'r_inner');
          final start = _n(body, 'a0') - math.pi / 2;
          final sweep = _n(body, 'a1') - _n(body, 'a0');
          final path = Path();
          if (inner > 0) {
            path.moveTo(
              cx + inner * math.cos(start),
              cy + inner * math.sin(start),
            );
          } else {
            path.moveTo(cx, cy);
          }
          path.arcTo(
            Rect.fromCircle(center: Offset(cx, cy), radius: outer),
            start,
            sweep,
            false,
          );
          if (inner > 0) {
            path.arcTo(
              Rect.fromCircle(center: Offset(cx, cy), radius: inner),
              start + sweep,
              -sweep,
              false,
            );
          }
          path.close();
          canvas.drawPath(path, _paint(body['fill']));
          _stroke(canvas, path, body['stroke']);
        case 'Circle':
          final center = Offset(_n(body, 'cx'), _n(body, 'cy'));
          final radius = _n(body, 'r');
          if (body['fill'] != null)
            canvas.drawCircle(center, radius, _paint(body['fill']));
          final stroke = body['stroke'];
          if (stroke is List && stroke.length >= 2) {
            canvas.drawCircle(
              center,
              radius,
              _paint(
                stroke[0],
                width: (stroke[1] as num).toDouble(),
                stroke: true,
              ),
            );
          }
        case 'Text':
          final painter = TextPainter(
            text: TextSpan(
              text: body['text'] as String? ?? '',
              style: TextStyle(
                fontFamily: fontFamily,
                color: _color(body['color']),
                fontSize: _n(body, 'size'),
                fontWeight: body['bold'] == true
                    ? FontWeight.bold
                    : FontWeight.normal,
              ),
            ),
            textDirection: TextDirection.ltr,
            maxLines: 1,
          )..layout();
          final x = _n(body, 'x');
          final y = _n(body, 'y');
          final dx = switch (body['align']) {
            'Center' => -painter.width / 2,
            'Right' => -painter.width,
            _ => 0.0,
          };
          final dy = switch (body['valign']) {
            'Middle' => -painter.height / 2,
            'Bottom' => -painter.height,
            _ => 0.0,
          };
          canvas.save();
          canvas.translate(x, y);
          canvas.rotate(_n(body, 'rotation'));
          painter.paint(canvas, Offset(dx, dy));
          canvas.restore();
      }
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant _ChartPainter oldDelegate) =>
      oldDelegate.primitives != primitives ||
      oldDelegate.fontFamily != fontFamily ||
      oldDelegate.zoomFactor != zoomFactor;
}
