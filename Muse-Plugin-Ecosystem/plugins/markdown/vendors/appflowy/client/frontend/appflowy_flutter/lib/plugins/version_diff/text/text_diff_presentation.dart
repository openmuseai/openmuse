import 'dart:math' as math;

import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';

enum MuseTextCellKind { unchanged, deleted, inserted, filler, folded }

final class MuseTextPresentationCell {
  const MuseTextPresentationCell({
    required this.kind,
    this.lineNumber,
    this.text = '',
    this.inlineSpans = const [],
  });

  final MuseTextCellKind kind;
  final int? lineNumber;
  final String text;
  final List<MuseTextInlineSpan> inlineSpans;
}

final class MuseTextPresentationRow {
  const MuseTextPresentationRow({
    required this.id,
    this.left,
    this.right,
    this.changeId,
    this.changeKind,
    this.semanticLabel = '',
    this.foldId,
    this.hiddenLineCount = 0,
    this.hiddenLeftStart,
    this.hiddenLeftEnd,
    this.hiddenRightStart,
    this.hiddenRightEnd,
  });

  final String id;
  final MuseTextPresentationCell? left;
  final MuseTextPresentationCell? right;
  final String? changeId;
  final MuseTextChangeKind? changeKind;
  final String semanticLabel;
  final String? foldId;
  final int hiddenLineCount;
  final int? hiddenLeftStart;
  final int? hiddenLeftEnd;
  final int? hiddenRightStart;
  final int? hiddenRightEnd;

  bool get isFold => foldId != null;
  bool get isChange => changeId != null;
}

final class MuseTextPresentationRun {
  const MuseTextPresentationRun({
    required this.changeId,
    required this.kind,
    required this.semanticLabel,
    required this.startRow,
    required this.endRow,
    required this.leftLineCount,
    required this.rightLineCount,
  });

  final String changeId;
  final MuseTextChangeKind kind;
  final String semanticLabel;
  final int startRow;
  final int endRow;
  final int leftLineCount;
  final int rightLineCount;
}

final class MuseTextPresentation {
  const MuseTextPresentation({required this.rows, required this.runs});

  final List<MuseTextPresentationRow> rows;
  final List<MuseTextPresentationRun> runs;

  int get longestLeftLine => rows.fold(
        0,
        (value, row) => math.max(value, row.left?.text.length ?? 0),
      );

  int get longestRightLine => rows.fold(
        0,
        (value, row) => math.max(value, row.right?.text.length ?? 0),
      );
}

/// Projects the provider's line changes into two continuous, aligned document
/// surfaces. It is intentionally UI independent and therefore unit-testable.
final class MuseTextPresentationBuilder {
  const MuseTextPresentationBuilder({this.foldContextLines = 3});

  final int foldContextLines;

  MuseTextPresentation build(
    MuseTextDiffPayload payload, {
    bool collapseUnchanged = true,
    Set<String> expandedFoldIds = const {},
  }) {
    final leftLines = _splitLines(payload.baseText);
    final rightLines = _splitLines(payload.targetText);
    final changes = [
      for (final hunk in payload.hunks) ...hunk.changes,
    ]..sort((a, b) {
        final oldOrder = a.oldStart.compareTo(b.oldStart);
        return oldOrder != 0 ? oldOrder : a.newStart.compareTo(b.newStart);
      });

    final inlineByKey = <String, List<MuseTextInlineSpan>>{};
    for (final hunk in payload.hunks) {
      for (final row in hunk.rows) {
        final changeId = row.changeId;
        if (changeId == null || row.inlineSpans.isEmpty) continue;
        final line =
            row.kind == MuseTextRowKind.deletion ? row.oldLine : row.newLine;
        if (line != null) {
          inlineByKey['$changeId:${row.kind.name}:$line'] = row.inlineSpans;
        }
      }
    }

    final fullRows = <MuseTextPresentationRow>[];
    var oldCursor = 1;
    var newCursor = 1;

    void appendGap(int oldEndExclusive, int newEndExclusive) {
      final oldCount = math.max(0, oldEndExclusive - oldCursor);
      final newCount = math.max(0, newEndExclusive - newCursor);
      final count = math.max(oldCount, newCount);
      for (var offset = 0; offset < count; offset++) {
        final oldLine = oldCursor + offset;
        final newLine = newCursor + offset;
        fullRows.add(
          MuseTextPresentationRow(
            id: 'context:$oldLine:$newLine',
            left: offset < oldCount
                ? _cell(leftLines, oldLine, MuseTextCellKind.unchanged)
                : const MuseTextPresentationCell(
                    kind: MuseTextCellKind.filler,
                  ),
            right: offset < newCount
                ? _cell(rightLines, newLine, MuseTextCellKind.unchanged)
                : const MuseTextPresentationCell(
                    kind: MuseTextCellKind.filler,
                  ),
          ),
        );
      }
      oldCursor = oldEndExclusive;
      newCursor = newEndExclusive;
    }

    for (final change in changes) {
      if (change.oldStart < oldCursor || change.newStart < newCursor) continue;
      appendGap(change.oldStart, change.newStart);
      final count = math.max(change.oldCount, change.newCount);
      for (var offset = 0; offset < count; offset++) {
        final oldLine = change.oldStart + offset;
        final newLine = change.newStart + offset;
        final left = offset < change.oldCount
            ? _cell(
                leftLines,
                oldLine,
                MuseTextCellKind.deleted,
                inlineByKey[
                    '${change.id}:${MuseTextRowKind.deletion.name}:$oldLine'],
              )
            : const MuseTextPresentationCell(kind: MuseTextCellKind.filler);
        final right = offset < change.newCount
            ? _cell(
                rightLines,
                newLine,
                MuseTextCellKind.inserted,
                inlineByKey[
                    '${change.id}:${MuseTextRowKind.insertion.name}:$newLine'],
              )
            : const MuseTextPresentationCell(kind: MuseTextCellKind.filler);
        fullRows.add(
          MuseTextPresentationRow(
            id: '${change.id}:$offset',
            left: left,
            right: right,
            changeId: change.id,
            changeKind: change.kind,
            semanticLabel: change.semanticLabel,
          ),
        );
      }
      oldCursor = change.oldStart + change.oldCount;
      newCursor = change.newStart + change.newCount;
    }
    appendGap(leftLines.length + 1, rightLines.length + 1);

    final rows = collapseUnchanged
        ? _fold(fullRows, expandedFoldIds: expandedFoldIds)
        : fullRows;
    return MuseTextPresentation(
      rows: List.unmodifiable(rows),
      runs: List.unmodifiable(_runs(rows)),
    );
  }

  MuseTextPresentationCell _cell(
    List<String> lines,
    int oneBasedLine,
    MuseTextCellKind kind, [
    List<MuseTextInlineSpan>? inlineSpans,
  ]) {
    final index = oneBasedLine - 1;
    return MuseTextPresentationCell(
      kind: kind,
      lineNumber: oneBasedLine,
      text: index >= 0 && index < lines.length ? lines[index] : '',
      inlineSpans: inlineSpans ?? const [],
    );
  }

  List<String> _splitLines(String value) {
    if (value.isEmpty) return const [];
    final lines = value.replaceAll('\r\n', '\n').split('\n');
    if (lines.isNotEmpty && lines.last.isEmpty) lines.removeLast();
    return lines;
  }

  List<MuseTextPresentationRow> _fold(
    List<MuseTextPresentationRow> source, {
    required Set<String> expandedFoldIds,
  }) {
    final result = <MuseTextPresentationRow>[];
    var cursor = 0;
    while (cursor < source.length) {
      if (source[cursor].isChange) {
        result.add(source[cursor++]);
        continue;
      }
      final start = cursor;
      while (cursor < source.length && !source[cursor].isChange) {
        cursor++;
      }
      final length = cursor - start;
      final foldThreshold = foldContextLines * 2 + 6;
      if (length <= foldThreshold) {
        result.addAll(source.getRange(start, cursor));
        continue;
      }
      final hiddenStart = start + foldContextLines;
      final hiddenEnd = cursor - foldContextLines;
      final foldId =
          'fold:${source[hiddenStart].id}:${source[hiddenEnd - 1].id}';
      if (expandedFoldIds.contains(foldId)) {
        result.addAll(source.getRange(start, cursor));
        continue;
      }
      result.addAll(source.getRange(start, hiddenStart));
      result.add(
        MuseTextPresentationRow(
          id: foldId,
          foldId: foldId,
          hiddenLineCount: hiddenEnd - hiddenStart,
          hiddenLeftStart:
              _firstLine(source, hiddenStart, hiddenEnd, left: true),
          hiddenLeftEnd: _lastLine(source, hiddenStart, hiddenEnd, left: true),
          hiddenRightStart:
              _firstLine(source, hiddenStart, hiddenEnd, left: false),
          hiddenRightEnd:
              _lastLine(source, hiddenStart, hiddenEnd, left: false),
          left: const MuseTextPresentationCell(kind: MuseTextCellKind.folded),
          right: const MuseTextPresentationCell(kind: MuseTextCellKind.folded),
        ),
      );
      result.addAll(source.getRange(hiddenEnd, cursor));
    }
    return result;
  }

  int? _firstLine(
    List<MuseTextPresentationRow> source,
    int start,
    int end, {
    required bool left,
  }) {
    for (var index = start; index < end; index++) {
      final number = left
          ? source[index].left?.lineNumber
          : source[index].right?.lineNumber;
      if (number != null) return number;
    }
    return null;
  }

  int? _lastLine(
    List<MuseTextPresentationRow> source,
    int start,
    int end, {
    required bool left,
  }) {
    for (var index = end - 1; index >= start; index--) {
      final number = left
          ? source[index].left?.lineNumber
          : source[index].right?.lineNumber;
      if (number != null) return number;
    }
    return null;
  }

  List<MuseTextPresentationRun> _runs(List<MuseTextPresentationRow> rows) {
    final result = <MuseTextPresentationRun>[];
    var index = 0;
    while (index < rows.length) {
      final changeId = rows[index].changeId;
      if (changeId == null) {
        index++;
        continue;
      }
      final start = index;
      var leftCount = 0;
      var rightCount = 0;
      while (index < rows.length && rows[index].changeId == changeId) {
        if (rows[index].left?.kind != MuseTextCellKind.filler) leftCount++;
        if (rows[index].right?.kind != MuseTextCellKind.filler) rightCount++;
        index++;
      }
      result.add(
        MuseTextPresentationRun(
          changeId: changeId,
          kind: rows[start].changeKind!,
          semanticLabel: rows[start].semanticLabel,
          startRow: start,
          endRow: index,
          leftLineCount: leftCount,
          rightLineCount: rightCount,
        ),
      );
    }
    return result;
  }
}
