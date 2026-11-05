import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';

enum MuseTextRowKind { context, deletion, insertion }

enum MuseTextChangeKind { insert, delete, replace }

final class MuseTextInlineSpan {
  const MuseTextInlineSpan({required this.text, required this.changed});

  final String text;
  final bool changed;
}

final class MuseTextDiffRow {
  const MuseTextDiffRow({
    required this.kind,
    required this.text,
    this.oldLine,
    this.newLine,
    this.changeId,
    this.inlineSpans = const [],
  });

  final MuseTextRowKind kind;
  final String text;
  final int? oldLine;
  final int? newLine;
  final String? changeId;
  final List<MuseTextInlineSpan> inlineSpans;
}

final class MuseTextChange {
  const MuseTextChange({
    required this.id,
    required this.kind,
    required this.oldStart,
    required this.oldCount,
    required this.newStart,
    required this.newCount,
    required this.semanticLabel,
  });

  final String id;
  final MuseTextChangeKind kind;
  final int oldStart;
  final int oldCount;
  final int newStart;
  final int newCount;
  final String semanticLabel;
}

final class MuseTextDiffHunk {
  const MuseTextDiffHunk({
    required this.id,
    required this.oldStart,
    required this.oldCount,
    required this.newStart,
    required this.newCount,
    required this.semanticLabel,
    required this.rows,
    required this.changes,
  });

  final String id;
  final int oldStart;
  final int oldCount;
  final int newStart;
  final int newCount;
  final String semanticLabel;
  final List<MuseTextDiffRow> rows;
  final List<MuseTextChange> changes;

  String get header =>
      '@@ -$oldStart,$oldCount +$newStart,$newCount @@ $semanticLabel';
}

final class MuseTextDiffPayload {
  const MuseTextDiffPayload({
    required this.language,
    required this.hunks,
    required this.additions,
    required this.deletions,
    required this.baseText,
    required this.targetText,
    this.engine = 'dart-myers',
    this.quality = MuseDiffQualityKind.exact,
    this.similarBoundaries = const [],
  });

  final String language;
  final List<MuseTextDiffHunk> hunks;
  final int additions;
  final int deletions;
  final String baseText;
  final String targetText;
  final String engine;
  final MuseDiffQualityKind quality;
  final List<MuseSimilarBoundary> similarBoundaries;

  int get changeCount =>
      hunks.fold(0, (total, hunk) => total + hunk.changes.length);
}

final class MuseSimilarBoundary {
  const MuseSimilarBoundary({required this.oldLine, required this.newLine});

  factory MuseSimilarBoundary.fromJson(Map<String, dynamic> json) =>
      MuseSimilarBoundary(
        oldLine: json['oldLine'] as int? ?? 0,
        newLine: json['newLine'] as int? ?? 0,
      );

  final int oldLine;
  final int newLine;
}
