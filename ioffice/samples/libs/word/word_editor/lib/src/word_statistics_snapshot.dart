import 'dart:convert';

import 'package:word_render/word_render.dart';

/// Read-only projection of word-core statistics plus the current page layout.
/// Product resources provide labels; this object never owns document text.
class WordStatisticsSnapshot {
  const WordStatisticsSnapshot({
    required this.pages,
    required this.words,
    required this.asianChars,
    required this.nonAsianWords,
    required this.charsNoSpace,
    required this.charsWithSpace,
    required this.paragraphs,
    required this.lines,
  });

  final int pages;
  final int words;
  final int asianChars;
  final int nonAsianWords;
  final int charsNoSpace;
  final int charsWithSpace;
  final int paragraphs;
  final int lines;

  int value(String id) => switch (id) {
        'pages' => pages,
        'words' => words,
        'asianChars' => asianChars,
        'nonAsianWords' => nonAsianWords,
        'charsNoSpace' => charsNoSpace,
        'charsWithSpace' => charsWithSpace,
        'paragraphs' => paragraphs,
        'lines' => lines,
        _ => throw FormatException('Unknown Word statistic: $id'),
      };

  factory WordStatisticsSnapshot.fromEngineJson(
    String json, {
    required RenderEnvelope? envelope,
  }) {
    final value = jsonDecode(json);
    if (value is! Map<String, dynamic> || value['schemaVersion'] != 1) {
      throw const FormatException('Unsupported Word statistics snapshot');
    }
    int field(String key) {
      final result = value[key];
      if (result is! int || result < 0) {
        throw FormatException('Invalid Word statistic: $key');
      }
      return result;
    }

    return WordStatisticsSnapshot(
      pages: envelope?.page.pageCount ?? 0,
      words: field('words'),
      asianChars: field('asianChars'),
      nonAsianWords: field('nonAsianWords'),
      charsNoSpace: field('charsNoSpace'),
      charsWithSpace: field('charsWithSpace'),
      paragraphs: field('paragraphs'),
      lines: _renderedLineCount(envelope),
    );
  }
}

/// Count visible baselines, merging text runs on the same rendered line.
int _renderedLineCount(RenderEnvelope? envelope) {
  if (envelope == null) return 0;
  final baselines = <int, List<int>>{};
  for (final command in envelope.commands) {
    if (command is! WcDrawText || command.text.trim().isEmpty) continue;
    final pageLines = baselines.putIfAbsent(command.page, () => []);
    if (pageLines.any((y) => (y - command.y).abs() <= 120)) continue;
    pageLines.add(command.y);
  }
  return baselines.values.fold(0, (total, page) => total + page.length);
}
