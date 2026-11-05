import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';

/// Compact change-block result shared by the Rust FFI runtime and the Dart
/// Myers fallback. Neither engine returns document text.
final class MuseTextDiffEngineResult {
  const MuseTextDiffEngineResult({
    required this.blocks,
    required this.additions,
    required this.deletions,
    required this.engine,
    this.quality = MuseDiffQualityKind.exact,
    this.similarBoundaries = const [],
  });

  final List<MuseChangeBlock> blocks;
  final int additions;
  final int deletions;
  final String engine;
  final MuseDiffQualityKind quality;
  final List<MuseSimilarBoundary> similarBoundaries;
}

final class MuseChangeBlock {
  const MuseChangeBlock({
    required this.oldStart,
    required this.oldCount,
    required this.newStart,
    required this.newCount,
    this.kind = MuseTextChangeKind.replace,
  });

  factory MuseChangeBlock.fromJson(Map<String, dynamic> json) {
    final oldCount = json['oldCount'] as int? ?? 0;
    final newCount = json['newCount'] as int? ?? 0;
    return MuseChangeBlock(
      oldStart: json['oldStart'] as int? ?? 0,
      oldCount: oldCount,
      newStart: json['newStart'] as int? ?? 0,
      newCount: newCount,
      kind: _kindFrom(json['kind'] as String?, oldCount, newCount),
    );
  }

  /// Zero-based line index in the before snapshot.
  final int oldStart;
  final int oldCount;

  /// Zero-based line index in the after snapshot.
  final int newStart;
  final int newCount;
  final MuseTextChangeKind kind;

  static MuseTextChangeKind _kindFrom(
    String? value,
    int oldCount,
    int newCount,
  ) {
    return switch (value) {
      'insert' => MuseTextChangeKind.insert,
      'delete' => MuseTextChangeKind.delete,
      'replace' => MuseTextChangeKind.replace,
      _ when oldCount == 0 => MuseTextChangeKind.insert,
      _ when newCount == 0 => MuseTextChangeKind.delete,
      _ => MuseTextChangeKind.replace,
    };
  }
}

final class MuseDartTextDiffEngine {
  const MuseDartTextDiffEngine();

  static const engineId = 'dart-myers';

  MuseTextDiffEngineResult compare(String before, String after) {
    final oldLines = splitLines(before);
    final newLines = splitLines(after);
    final edits = _myers(oldLines, newLines);
    final blocks = _changeBlocks(edits);
    return MuseTextDiffEngineResult(
      blocks: blocks,
      additions: edits.where((edit) => edit.kind == _EditKind.insert).length,
      deletions: edits.where((edit) => edit.kind == _EditKind.delete).length,
      engine: engineId,
      similarBoundaries: _similarBoundaries(
        blocks,
        oldLines.length,
        newLines.length,
      ),
    );
  }

  static List<String> splitLines(String value) {
    if (value.isEmpty) return const [];
    final lines = value.replaceAll('\r\n', '\n').split('\n');
    if (lines.isNotEmpty && lines.last.isEmpty) lines.removeLast();
    return lines;
  }

  static String copyLines(String value, int startLine, int endLine) {
    final lines = splitLines(value);
    final start = (startLine - 1).clamp(0, lines.length);
    final end = endLine.clamp(0, lines.length);
    if (start >= end) return '';
    return lines.sublist(start, end).join('\n');
  }

  static List<MuseTextSearchHit> search(
    String value,
    String query, {
    bool caseSensitive = false,
    int limit = 256,
  }) {
    if (query.isEmpty || limit <= 0) return const [];
    final needle = caseSensitive ? query : query.toLowerCase();
    final lines = splitLines(value);
    final hits = <MuseTextSearchHit>[];
    for (var line = 0; line < lines.length; line++) {
      final haystack = caseSensitive ? lines[line] : lines[line].toLowerCase();
      var start = 0;
      while (true) {
        final column = haystack.indexOf(needle, start);
        if (column < 0) break;
        hits.add(
          MuseTextSearchHit(
            line: line + 1,
            column: column,
            length: query.length,
          ),
        );
        if (hits.length >= limit) return hits;
        start = column + needle.length;
      }
    }
    return hits;
  }

  List<MuseChangeBlock> _changeBlocks(List<_Edit> edits) {
    final result = <MuseChangeBlock>[];
    var oldCursor = 0;
    var newCursor = 0;
    var index = 0;
    while (index < edits.length) {
      if (edits[index].kind == _EditKind.equal) {
        oldCursor++;
        newCursor++;
        index++;
        continue;
      }
      final oldStart = oldCursor;
      final newStart = newCursor;
      var oldCount = 0;
      var newCount = 0;
      while (index < edits.length && edits[index].kind != _EditKind.equal) {
        switch (edits[index].kind) {
          case _EditKind.delete:
            oldCount++;
            oldCursor++;
          case _EditKind.insert:
            newCount++;
            newCursor++;
          case _EditKind.equal:
            break;
        }
        index++;
      }
      result.add(
        MuseChangeBlock(
          oldStart: oldStart,
          oldCount: oldCount,
          newStart: newStart,
          newCount: newCount,
          kind: MuseChangeBlock._kindFrom(null, oldCount, newCount),
        ),
      );
    }
    return result;
  }

  List<MuseSimilarBoundary> _similarBoundaries(
    List<MuseChangeBlock> blocks,
    int beforeLen,
    int afterLen,
  ) {
    final result = <MuseSimilarBoundary>[
      const MuseSimilarBoundary(oldLine: 0, newLine: 0),
    ];
    for (final block in blocks) {
      result.add(
        MuseSimilarBoundary(oldLine: block.oldStart, newLine: block.newStart),
      );
      result.add(
        MuseSimilarBoundary(
          oldLine: block.oldStart + block.oldCount,
          newLine: block.newStart + block.newCount,
        ),
      );
    }
    result.add(MuseSimilarBoundary(oldLine: beforeLen, newLine: afterLen));
    final unique = <MuseSimilarBoundary>[];
    for (final boundary in result) {
      if (unique.isEmpty ||
          unique.last.oldLine != boundary.oldLine ||
          unique.last.newLine != boundary.newLine) {
        unique.add(boundary);
      }
    }
    return unique;
  }

  List<_Edit> _myers(List<String> oldLines, List<String> newLines) {
    if (oldLines.isEmpty) {
      return [
        for (var index = 0; index < newLines.length; index++)
          _Edit.insert(newLines[index], index),
      ];
    }
    if (newLines.isEmpty) {
      return [
        for (var index = 0; index < oldLines.length; index++)
          _Edit.delete(oldLines[index], index),
      ];
    }
    final max = oldLines.length + newLines.length;
    var frontier = <int, int>{1: 0};
    final trace = <Map<int, int>>[];
    for (var distance = 0; distance <= max; distance++) {
      trace.add(Map<int, int>.from(frontier));
      final next = Map<int, int>.from(frontier);
      for (var diagonal = -distance; diagonal <= distance; diagonal += 2) {
        final down = diagonal == -distance ||
            (diagonal != distance &&
                (frontier[diagonal - 1] ?? -1) <
                    (frontier[diagonal + 1] ?? -1));
        var x = down
            ? (frontier[diagonal + 1] ?? 0)
            : (frontier[diagonal - 1] ?? 0) + 1;
        var y = x - diagonal;
        while (x < oldLines.length &&
            y < newLines.length &&
            oldLines[x] == newLines[y]) {
          x++;
          y++;
        }
        next[diagonal] = x;
        if (x >= oldLines.length && y >= newLines.length) {
          trace.add(next);
          return _backtrack(trace, oldLines, newLines);
        }
      }
      frontier = next;
    }
    throw StateError('Unable to calculate text diff');
  }

  List<_Edit> _backtrack(
    List<Map<int, int>> trace,
    List<String> oldLines,
    List<String> newLines,
  ) {
    var x = oldLines.length;
    var y = newLines.length;
    final result = <_Edit>[];
    for (var distance = trace.length - 2; distance >= 0; distance--) {
      final frontier = trace[distance];
      final diagonal = x - y;
      final down = diagonal == -distance ||
          (diagonal != distance &&
              (frontier[diagonal - 1] ?? -1) < (frontier[diagonal + 1] ?? -1));
      final previousDiagonal = down ? diagonal + 1 : diagonal - 1;
      final previousX = frontier[previousDiagonal] ?? 0;
      final previousY = previousX - previousDiagonal;
      while (x > previousX && y > previousY) {
        x--;
        y--;
        result.add(_Edit.equal(oldLines[x], x, y));
      }
      if (distance == 0) break;
      if (down) {
        y--;
        result.add(_Edit.insert(newLines[y], y));
      } else {
        x--;
        result.add(_Edit.delete(oldLines[x], x));
      }
    }
    return result.reversed.toList(growable: false);
  }
}

final class MuseTextSearchHit {
  const MuseTextSearchHit({
    required this.line,
    required this.column,
    required this.length,
  });

  final int line;
  final int column;
  final int length;
}

enum _EditKind { equal, delete, insert }

final class _Edit {
  const _Edit._(this.kind, this.text, this.oldIndex, this.newIndex);

  factory _Edit.equal(String text, int oldIndex, int newIndex) =>
      _Edit._(_EditKind.equal, text, oldIndex, newIndex);

  factory _Edit.delete(String text, int oldIndex) =>
      _Edit._(_EditKind.delete, text, oldIndex, null);

  factory _Edit.insert(String text, int newIndex) =>
      _Edit._(_EditKind.insert, text, null, newIndex);

  final _EditKind kind;
  final String text;
  final int? oldIndex;
  final int? newIndex;
}
