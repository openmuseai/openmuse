import 'package:flutter/material.dart';

import 'word_product_definition.dart';
import 'word_style_card.dart';
import 'word_style_snapshot.dart';

/// Renders signed gallery data. It never reads or mutates a document; command
/// availability and dispatch are supplied by the Host at the menu boundary.
class WordMenuGrid extends StatefulWidget {
  const WordMenuGrid({
    super.key,
    required this.entry,
    required this.chrome,
    required this.touch,
    required this.isCommandAvailable,
    required this.onCommand,
    this.styleSnapshot,
  });

  final WordMenuEntry entry;
  final WordChromeDefinition chrome;
  final bool touch;
  final bool Function(String commandId) isCommandAvailable;
  final ValueChanged<String> onCommand;
  final WordStyleSnapshot? styleSnapshot;

  @override
  State<WordMenuGrid> createState() => _WordMenuGridState();
}

class _WordMenuGridState extends State<WordMenuGrid> {
  int? _hoveredTableCell;

  bool _available(WordMenuEntry cell) =>
      cell.command != null && widget.isCommandAvailable(cell.command!);

  void _run(WordMenuEntry cell) {
    if (_available(cell)) widget.onCommand(cell.command!);
  }

  @override
  Widget build(BuildContext context) {
    final entry = widget.entry;
    if (entry.presentation == WordGridPresentation.tablePicker) {
      return _tablePicker();
    }
    final touch = widget.touch;
    // Preserve the source gallery's four columns where there is room. On
    // narrow touch screens, reflow cards instead of hiding the last column
    // behind a horizontal gesture with no visible affordance.
    final columns =
        entry.presentation == WordGridPresentation.cardGallery && touch
            ? ((MediaQuery.sizeOf(context).width - 16 + 8) / 128)
                .floor()
                .clamp(1, entry.columns)
            : entry.presentation == WordGridPresentation.styleGallery && touch
                ? ((MediaQuery.sizeOf(context).width - 32 + 8) / 112)
                    .floor()
                    .clamp(1, entry.columns)
                : entry.columns;
    final (baseTileWidth, tileHeight, gap) = switch (entry.presentation) {
      WordGridPresentation.colorPalette ||
      WordGridPresentation.highlightPalette =>
        (touch ? 44.0 : 20.0, touch ? 44.0 : 20.0, touch ? 3.0 : 4.0),
      WordGridPresentation.styleGallery => (
          touch ? 108.0 : 74.0,
          touch ? 72.0 : 62.0,
          4.0
        ),
      WordGridPresentation.cardGallery => (
          touch ? 120.0 : 104.0,
          touch ? 155.0 : 148.0,
          8.0
        ),
      WordGridPresentation.wordArtGallery => (
          touch ? 44.0 : 44.0,
          touch ? 44.0 : 44.0,
          5.0
        ),
      WordGridPresentation.symbolGallery => (
          touch ? 44.0 : 28.0,
          touch ? 44.0 : 28.0,
          2.0
        ),
      WordGridPresentation.shapeGallery => (
          touch ? 44.0 : 28.0,
          touch ? 44.0 : 28.0,
          3.0
        ),
      WordGridPresentation.positionPicker => (
          touch ? 52.0 : 42.0,
          touch ? 52.0 : 42.0,
          4.0
        ),
      WordGridPresentation.bulletGallery ||
      WordGridPresentation.numberGallery =>
        (touch ? 76.0 : 74.0, touch ? 64.0 : 56.0, 4.0),
      WordGridPresentation.multilevelGallery => (
          touch ? 108.0 : 96.0,
          touch ? 76.0 : 66.0,
          4.0
        ),
      WordGridPresentation.listLevels => (
          touch ? 44.0 : 22.0,
          touch ? 44.0 : 22.0,
          3.0
        ),
      WordGridPresentation.dropCapGallery => (
          touch ? 72.0 : 62.0,
          touch ? 82.0 : 70.0,
          4.0
        ),
      WordGridPresentation.equationGallery => (
          touch ? 272.0 : 282.0,
          touch ? 64.0 : 58.0,
          2.0
        ),
      _ => (
          touch ? 44.0 : (columns <= 4 ? 74.0 : 28.0),
          touch ? 44.0 : (columns <= 4 ? 50.0 : 28.0),
          3.0
        ),
    };
    final tileWidth =
        entry.presentation == WordGridPresentation.styleGallery && !touch
            ? ((MediaQuery.sizeOf(context).width - 16).clamp(190.0, 478.0) -
                    16 -
                    (columns - 1) * gap) /
                columns
            : baseTileWidth;
    final width = columns * tileWidth + (columns - 1) * gap;
    return Padding(
        padding: const EdgeInsets.fromLTRB(8, 6, 8, 8),
        child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (entry.label.isNotEmpty) ...[
                Text(entry.label,
                    style: TextStyle(
                        fontSize: 11.5,
                        fontWeight: FontWeight.w600,
                        color: widget.chrome.muted)),
                const SizedBox(height: 6),
              ],
              SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: SizedBox(
                      width: width,
                      child: Wrap(spacing: gap, runSpacing: gap, children: [
                        for (final cell in entry.children)
                          _tile(cell, tileWidth, tileHeight),
                      ]))),
            ]));
  }

  Widget _tile(WordMenuEntry cell, double width, double height) {
    final c = widget.chrome;
    final kind = widget.entry.presentation;
    if (kind == WordGridPresentation.styleGallery && cell.styleInfo != null) {
      final style = cell.styleInfo!;
      final key =
          '${style.type == 'character' ? 'char' : 'para'}:${style.styleId}';
      return WordStyleCard(
        key: Key('word-menu-${cell.id}'),
        style: style,
        label: cell.label,
        chrome: c,
        snapshot: widget.styleSnapshot,
        active: widget.styleSnapshot?.activeStyleKey == key,
        width: width,
        height: height,
        onTap: _available(cell) ? () => _run(cell) : null,
      );
    }
    final color = cell.swatch == null
        ? null
        : WordChromeDefinition.colorFromHex(cell.swatch!);
    final isColor = kind == WordGridPresentation.colorPalette ||
        kind == WordGridPresentation.highlightPalette;
    final isCard = kind == WordGridPresentation.styleGallery ||
        kind == WordGridPresentation.cardGallery ||
        kind == WordGridPresentation.wordArtGallery;
    final isCover = kind == WordGridPresentation.cardGallery;
    return Tooltip(
        message: _available(cell) ? cell.label : '${cell.label}（暂不可用）',
        child: Semantics(
            button: true,
            enabled: _available(cell),
            label: cell.label,
            child: InkWell(
                key: Key('word-menu-${cell.id}'),
                onTap: _available(cell) ? () => _run(cell) : null,
                child: Container(
                    width: width,
                    height: height,
                    alignment: Alignment.center,
                    decoration: BoxDecoration(
                        color: kind == WordGridPresentation.wordArtGallery
                            ? c.chrome
                            : color ?? c.chrome,
                        border: Border.all(
                            color: isCover ? Colors.transparent : c.border),
                        borderRadius: isCard
                            ? BorderRadius.circular(isCover ? 6 : 4)
                            : null),
                    child: kind == WordGridPresentation.dropCapGallery
                        ? _dropCapPreview(cell)
                        : kind == WordGridPresentation.equationGallery
                            ? _equationPreview(cell)
                            : kind == WordGridPresentation.cardGallery
                                ? _coverPreview(cell)
                                : kind == WordGridPresentation.wordArtGallery
                                    ? _wordArtPreview(cell)
                                    : kind ==
                                            WordGridPresentation.positionPicker
                                        ? _positionPreview(cell)
                                        : kind == WordGridPresentation.bulletGallery ||
                                                kind ==
                                                    WordGridPresentation
                                                        .numberGallery ||
                                                kind ==
                                                    WordGridPresentation
                                                        .multilevelGallery ||
                                                kind ==
                                                    WordGridPresentation
                                                        .listLevels
                                            ? _listPreview(cell, kind)
                                            : isColor
                                                ? null
                                                : isCard
                                                    ? Column(
                                                        mainAxisAlignment:
                                                            MainAxisAlignment
                                                                .center,
                                                        children: [
                                                            Text('Aa文',
                                                                style: TextStyle(
                                                                    fontSize:
                                                                        15,
                                                                    fontWeight:
                                                                        FontWeight
                                                                            .w600,
                                                                    color: c
                                                                        .text)),
                                                            Text(cell.label,
                                                                maxLines: 1,
                                                                overflow:
                                                                    TextOverflow
                                                                        .ellipsis,
                                                                style: TextStyle(
                                                                    fontSize:
                                                                        10,
                                                                    color: c
                                                                        .muted)),
                                                          ])
                                                    : kind == WordGridPresentation.shapeGallery &&
                                                            cell.previewPath !=
                                                                null
                                                        ? CustomPaint(
                                                            size: const Size(
                                                                18, 18),
                                                            painter: _ShapePreviewPainter(
                                                                cell.previewPath!, c.text,
                                                                flat: cell.id
                                                                    .contains(
                                                                        'flowChart')))
                                                        : Text(cell.label,
                                                            maxLines: 2,
                                                            overflow: TextOverflow
                                                                .ellipsis,
                                                            textAlign:
                                                                TextAlign.center,
                                                            style: TextStyle(fontSize: kind == WordGridPresentation.symbolGallery || kind == WordGridPresentation.shapeGallery ? 19 : 10, color: c.text))))));
  }

  Widget _dropCapPreview(WordMenuEntry cell) => Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(cell.previewText ?? '—',
              style: TextStyle(
                  fontSize: 28,
                  height: 1,
                  fontWeight: FontWeight.bold,
                  color: widget.chrome.text)),
          const SizedBox(height: 6),
          Text(cell.label,
              style: TextStyle(fontSize: 11, color: widget.chrome.text)),
        ],
      );

  Widget _equationPreview(WordMenuEntry cell) => Padding(
        padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 4),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(cell.label,
                style: TextStyle(fontSize: 11, color: widget.chrome.muted)),
            const SizedBox(height: 3),
            Text(cell.previewText ?? '',
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                textAlign: TextAlign.center,
                style: TextStyle(
                    fontSize: 15,
                    fontFamily: 'Cambria Math',
                    color: widget.chrome.text)),
          ],
        ),
      );

  Widget _listPreview(WordMenuEntry cell, WordGridPresentation kind) {
    final c = widget.chrome;
    if (kind == WordGridPresentation.listLevels) {
      return Text(cell.label,
          style: TextStyle(fontSize: 11, color: c.text),
          textAlign: TextAlign.center);
    }
    if (kind == WordGridPresentation.bulletGallery) {
      return Text(cell.label,
          style: TextStyle(fontSize: 19, color: c.text),
          textAlign: TextAlign.center);
    }
    final lines = kind == WordGridPresentation.multilevelGallery
        ? cell.label.split(' → ')
        : cell.label.split(' ');
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 6),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          for (var i = 0; i < 3; i++)
            Padding(
              padding: EdgeInsets.only(
                  left: kind == WordGridPresentation.multilevelGallery
                      ? i * 5.0
                      : 0),
              child: Row(children: [
                Text(i < lines.length ? lines[i] : '',
                    maxLines: 1, style: TextStyle(fontSize: 9, color: c.text)),
                const SizedBox(width: 3),
                Expanded(child: Container(height: 1.5, color: c.border)),
              ]),
            ),
        ],
      ),
    );
  }

  Widget _positionPreview(WordMenuEntry cell) {
    final parts = cell.id.split('.');
    final row = switch (parts[1]) {
      'top' => 0,
      'center' => 1,
      _ => 2,
    };
    final column = int.parse(parts[2]) - 1;
    return CustomPaint(
      key: Key('word-position-preview-${cell.id}'),
      size: const Size(32, 32),
      painter: _PositionPreviewPainter(row, column, widget.chrome),
    );
  }

  Widget _coverPreview(WordMenuEntry cell) {
    final c = widget.chrome;
    return Column(mainAxisAlignment: MainAxisAlignment.center, children: [
      CustomPaint(
          key: Key('word-cover-preview-${cell.id}'),
          size: const Size(88, 116),
          painter: _CoverPreviewPainter(cell.previewParas, c.muted,
              DefaultTextStyle.of(context).style.fontFamily)),
      const SizedBox(height: 3),
      Text(cell.label,
          maxLines: 1,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(fontSize: 11, color: c.text)),
    ]);
  }

  Widget _wordArtPreview(WordMenuEntry cell) {
    final preview = cell.previewWordArt;
    if (preview == null) {
      return Text('艺',
          style: TextStyle(
              fontSize: 19,
              fontWeight: FontWeight.w600,
              color: cell.swatch == null
                  ? widget.chrome.text
                  : WordChromeDefinition.colorFromHex(cell.swatch!)));
    }
    final hostFont = DefaultTextStyle.of(context).style.fontFamily;
    final base = TextStyle(
        fontFamily: 'Georgia',
        fontFamilyFallback: [
          if (hostFont != null) hostFont,
          'WordArtVisual',
          'PingFang SC',
          'Noto Sans CJK SC'
        ],
        fontSize: 26,
        height: 1,
        fontWeight: preview.bold ? FontWeight.w800 : FontWeight.w400,
        fontStyle: preview.italic ? FontStyle.italic : FontStyle.normal);
    final fill = WordChromeDefinition.colorFromHex(preview.fill);
    final outlineColor = preview.outlineColor;
    final widthEmu = preview.outlineWidthEmu;
    final strokePx = widthEmu == null
        ? 0.0
        : ((widthEmu / 12700 * 4 / 3) * 100).round() / 100;
    return Stack(
        key: Key('word-wordart-preview-${cell.id}'),
        alignment: Alignment.center,
        children: [
          if (outlineColor != null && strokePx > 0)
            Text('艺',
                style: base.copyWith(
                    foreground: Paint()
                      ..style = PaintingStyle.stroke
                      ..strokeWidth = strokePx
                      ..strokeJoin = StrokeJoin.round
                      ..color =
                          WordChromeDefinition.colorFromHex(outlineColor))),
          Text('艺', style: base.copyWith(color: fill)),
        ]);
  }

  Widget _tablePicker() {
    final entry = widget.entry;
    final c = widget.chrome;
    final touch = widget.touch;
    final columns = entry.columns;
    final cellWidth = touch ? 44.0 : 18.0;
    final cellHeight = touch ? 44.0 : 16.0;
    final gap = touch ? 3.0 : 3.0;
    final width = columns * cellWidth + (columns - 1) * gap;
    final preview = _hoveredTableCell;
    final title = preview == null
        ? entry.label
        : '${preview % columns + 1} × ${preview ~/ columns + 1} 表格';
    return Padding(
        padding: const EdgeInsets.all(8),
        child: Column(mainAxisSize: MainAxisSize.min, children: [
          Text(title,
              key: const Key('word-table-picker-title'),
              style: TextStyle(fontSize: 12, color: c.muted)),
          const SizedBox(height: 6),
          MouseRegion(
              onExit: (_) => setState(() => _hoveredTableCell = null),
              child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: SizedBox(
                      width: width,
                      child: Wrap(spacing: gap, runSpacing: gap, children: [
                        for (var i = 0; i < entry.children.length; i++)
                          _tableCell(
                              entry.children[i], i, cellWidth, cellHeight),
                      ])))),
        ]));
  }

  Widget _tableCell(
      WordMenuEntry cell, int index, double width, double height) {
    final c = widget.chrome;
    final columns = widget.entry.columns;
    final preview = _hoveredTableCell;
    final hot = preview != null &&
        index ~/ columns <= preview ~/ columns &&
        index % columns <= preview % columns;
    return MouseRegion(
        onEnter: (_) => setState(() => _hoveredTableCell = index),
        child: Tooltip(
            message: _available(cell) ? cell.label : '${cell.label}（暂不可用）',
            child: Semantics(
                button: true,
                enabled: _available(cell),
                label: cell.label,
                child: InkWell(
                    key: Key('word-menu-${cell.id}'),
                    onTap: _available(cell) ? () => _run(cell) : null,
                    child: Container(
                        width: width,
                        height: height,
                        decoration: BoxDecoration(
                            color: hot
                                ? c.accent.withValues(alpha: 0.12)
                                : c.chrome,
                            border:
                                Border.all(color: hot ? c.accent : c.border),
                            borderRadius: BorderRadius.circular(2)))))));
  }
}

/// Native reproduction of GenOffice CoverThumb's 88×116 thumbnail. This only
/// paints signed product preview data; the document page is never inspected.
class _CoverPreviewPainter extends CustomPainter {
  _CoverPreviewPainter(this.paragraphs, this.defaultInk, this.fontFamily)
      : today = DateTime.now();

  final List<WordCoverPreviewParagraph> paragraphs;
  final Color defaultInk;
  final String? fontFamily;
  final DateTime today;

  Color _hex(String value) => Color(int.parse(value, radix: 16) | 0xff000000);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();
    canvas.clipRect(Offset.zero & size);
    canvas.drawRect(Offset.zero & size, Paint()..color = Colors.white);
    canvas.drawRect(
        Rect.fromLTWH(0.25, 0.25, size.width - 0.5, size.height - 0.5),
        Paint()
          ..color = const Color(0xffb8b8b8)
          ..style = PaintingStyle.stroke
          ..strokeWidth = 0.5);
    var y = 4.0;
    for (final paragraph in paragraphs) {
      y += paragraph.spaceBefore / 130;
      final x = 5.0 + paragraph.indentLeft / 130;
      final width = (size.width -
              10 -
              (paragraph.indentLeft + paragraph.indentRight) / 130)
          .clamp(1.0, size.width - 10)
          .toDouble();
      final fontSize = paragraph.sizeHalfPoints == null
          ? 4.0
          : (paragraph.sizeHalfPoints! / 14).clamp(3.0, 20.0).toDouble();
      final height = fontSize * 1.3;
      final rect = Rect.fromLTWH(x, y, width, height);
      if (paragraph.shadingFill != null) {
        canvas.drawRect(rect, Paint()..color = _hex(paragraph.shadingFill!));
      }
      final border = Paint()
        ..color = const Color(0xff666666)
        ..strokeWidth = 0.5;
      if (paragraph.borders.contains('t')) {
        canvas.drawLine(rect.topLeft, rect.topRight, border);
      }
      if (paragraph.borders.contains('b')) {
        canvas.drawLine(rect.bottomLeft, rect.bottomRight, border);
      }
      if (paragraph.borders.contains('l')) {
        canvas.drawLine(rect.topLeft, rect.bottomLeft, border);
      }
      if (paragraph.borders.contains('r')) {
        canvas.drawLine(rect.topRight, rect.bottomRight, border);
      }
      final text = paragraph.text
          .replaceAll('{year}', '${today.year}')
          .replaceAll('{date}', '${today.year}年${today.month}月${today.day}日');
      if (text.isNotEmpty) {
        final align = switch (paragraph.align) {
          'center' => TextAlign.center,
          'right' => TextAlign.right,
          _ => TextAlign.left,
        };
        final painter = TextPainter(
          text: TextSpan(
              text: text,
              style: TextStyle(
                  fontFamily: fontFamily,
                  fontSize: fontSize,
                  height: 1.3,
                  fontWeight: paragraph.bold ? FontWeight.w700 : null,
                  fontStyle: paragraph.italic ? FontStyle.italic : null,
                  color: paragraph.color == null
                      ? defaultInk
                      : _hex(paragraph.color!))),
          textDirection: TextDirection.ltr,
          textAlign: align,
          maxLines: 1,
        )..layout(maxWidth: width);
        canvas.save();
        canvas.clipRect(rect);
        painter.paint(canvas, Offset(x, y));
        canvas.restore();
        painter.dispose();
      }
      y += height + paragraph.spaceAfter / 130;
      if (y > size.height + 100) break;
    }
    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant _CoverPreviewPainter oldDelegate) =>
      paragraphs != oldDelegate.paragraphs ||
      defaultInk != oldDelegate.defaultInk ||
      fontFamily != oldDelegate.fontFamily ||
      today.day != oldDelegate.today.day ||
      today.month != oldDelegate.today.month ||
      today.year != oldDelegate.today.year;
}

class _PositionPreviewPainter extends CustomPainter {
  const _PositionPreviewPainter(this.row, this.column, this.chrome);

  final int row;
  final int column;
  final WordChromeDefinition chrome;

  @override
  void paint(Canvas canvas, Size size) {
    final ink = Paint()
      ..color = chrome.muted.withValues(alpha: 0.65)
      ..strokeWidth = 1.2;
    for (var line = 0; line < 5; line++) {
      final y = 5.0 + line * 5.5;
      canvas.drawLine(Offset(3, y), Offset(size.width - 3, y), ink);
    }
    final x = 3.0 + column * 8.5;
    final y = 3.0 + row * 8.5;
    final block = Rect.fromLTWH(x, y, 10, 10);
    canvas.drawRect(block, Paint()..color = chrome.chrome);
    canvas.drawRect(block, Paint()..color = chrome.accent);
  }

  @override
  bool shouldRepaint(covariant _PositionPreviewPainter oldDelegate) =>
      row != oldDelegate.row ||
      column != oldDelegate.column ||
      chrome != oldDelegate.chrome;
}

class _ShapePreviewPainter extends CustomPainter {
  _ShapePreviewPainter(this.data, this.color, {required this.flat});

  final String data;
  final Color color;
  final bool flat;
  static final Map<String, Path> _paths = {};

  @override
  void paint(Canvas canvas, Size size) {
    final path = _paths.putIfAbsent(data, () => _parsePath(data));
    canvas.save();
    canvas.scale(size.width / 18, size.height / 18);
    if (flat) canvas.translate(0, (18 - 11.16) / 2);
    canvas.drawPath(
        path,
        Paint()
          ..color = color
          ..style = PaintingStyle.stroke
          ..strokeWidth = 1.2
          ..strokeJoin = StrokeJoin.round);
    canvas.restore();
  }

  static Path _parsePath(String data) {
    final tokens = RegExp(r'[MLCQZ]|-?(?:\d+\.?\d*|\.\d+)')
        .allMatches(data)
        .map((match) => match.group(0)!)
        .toList(growable: false);
    final path = Path();
    String? command;
    var cursor = 0;
    while (cursor < tokens.length) {
      final token = tokens[cursor];
      if ('MLCQZ'.contains(token) && token.length == 1) {
        command = token;
        cursor++;
        if (command == 'Z') {
          path.close();
          command = null;
          continue;
        }
      }
      final needed = switch (command) {
        'M' || 'L' => 2,
        'Q' => 4,
        'C' => 6,
        _ => 0,
      };
      if (needed == 0 || cursor + needed > tokens.length) break;
      final values = tokens
          .skip(cursor)
          .take(needed)
          .map(double.tryParse)
          .toList(growable: false);
      if (values.any((value) => value == null)) break;
      switch (command) {
        case 'M':
          path.moveTo(values[0]!, values[1]!);
          command = 'L';
        case 'L':
          path.lineTo(values[0]!, values[1]!);
        case 'Q':
          path.quadraticBezierTo(
              values[0]!, values[1]!, values[2]!, values[3]!);
        case 'C':
          path.cubicTo(values[0]!, values[1]!, values[2]!, values[3]!,
              values[4]!, values[5]!);
      }
      cursor += needed;
    }
    return path;
  }

  @override
  bool shouldRepaint(covariant _ShapePreviewPainter oldDelegate) =>
      data != oldDelegate.data ||
      color != oldDelegate.color ||
      flat != oldDelegate.flat;
}
