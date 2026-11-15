import 'dart:collection';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:duskmoon_highlight/tree_sitter.dart';

const maxHighlightCodeUnits = 256 * 1024;

String? viewerCodeLanguage(String name) {
  final basename = name.replaceAll('\\', '/').split('/').last.toLowerCase();
  final extension = basename.split('.').last;
  return TreeSitterRuntime.resolveLanguage(switch (extension) {
    'mjs' || 'cjs' || 'jsx' => 'javascript',
    'h' || 'cc' || 'hpp' => 'cpp',
    'pyi' => 'python',
    'kts' => 'kotlin',
    'yml' => 'yaml',
    'htm' => 'html',
    'zsh' => 'bash',
    _ => extension,
  });
}

typedef ViewerSyntaxRange = ({int from, int to, String capture});

/// Official Tree-sitter C parsers and pinned queries, executed off the UI
/// isolate on native platforms. No source text is sent over the network.
Future<List<ViewerSyntaxRange>?> highlightViewerCode(
  String source,
  String language,
) async {
  if (source.length > maxHighlightCodeUnits) return null;
  try {
    return await compute(_parse, (source, language));
  } on Object {
    // Syntax highlighting must never prevent reading a resource.
    return null;
  }
}

Future<List<ViewerSyntaxRange>> _parse((String, String) input) async {
  final (source, language) = input;
  await TreeSitterRuntime.ensureInitialized(languages: {language});
  final session = TreeSitterSession(language: language);
  try {
    final captures = session.parse(source).captures;
    if (captures.length > 20000) return [];
    final events = <int, ({List<int> start, List<int> end})>{};
    for (var i = 0; i < captures.length; i++) {
      final c = captures[i];
      if (c.from < 0 || c.to > source.length || c.from >= c.to) continue;
      (events[c.from] ??= (start: [], end: [])).start.add(i);
      (events[c.to] ??= (start: [], end: [])).end.add(i);
    }
    final active = SplayTreeSet<int>((a, b) {
      final left = captures[a], right = captures[b];
      var cmp = right.priority.compareTo(left.priority);
      if (cmp == 0)
        cmp = (left.to - left.from).compareTo(right.to - right.from);
      if (cmp == 0) cmp = right.patternIndex.compareTo(left.patternIndex);
      return cmp == 0 ? a.compareTo(b) : cmp;
    });
    final boundaries = events.keys.toList()..sort();
    final output = <ViewerSyntaxRange>[];
    for (var i = 0; i < boundaries.length - 1; i++) {
      final from = boundaries[i], to = boundaries[i + 1];
      final event = events[from]!;
      active.removeAll(event.end);
      active.addAll(event.start);
      if (active.isNotEmpty && from < to) {
        output.add((from: from, to: to, capture: captures[active.first].name));
      }
    }
    return output;
  } finally {
    session.dispose();
  }
}

Color? viewerSyntaxColor(String capture, Brightness brightness) {
  final dark = brightness == Brightness.dark;
  final group = capture.split('.').first;
  return switch (group) {
    'comment' => dark ? const Color(0xff8ab88a) : const Color(0xff52734b),
    'string' ||
    'character' => dark ? const Color(0xffe5b47b) : const Color(0xff9b4d10),
    'keyword' ||
    'storageclass' ||
    'conditional' ||
    'repeat' => dark ? const Color(0xffc6a0f6) : const Color(0xff7b2cbf),
    'function' ||
    'method' => dark ? const Color(0xff8cc5ff) : const Color(0xff125ca5),
    'type' ||
    'constructor' ||
    'tag' => dark ? const Color(0xff76d7c4) : const Color(0xff087c78),
    'number' ||
    'float' ||
    'boolean' ||
    'constant' => dark ? const Color(0xfff1a7be) : const Color(0xffac315a),
    'attribute' ||
    'property' => dark ? const Color(0xffe4cf8a) : const Color(0xff785b10),
    _ => null,
  };
}

List<List<TextSpan>> viewerSyntaxLines(
  String source,
  List<ViewerSyntaxRange> ranges,
  Brightness brightness,
) {
  final lines = <List<TextSpan>>[[]];
  void append(String text, Color? color) {
    final parts = text.split('\n');
    for (var i = 0; i < parts.length; i++) {
      if (i > 0) lines.add([]);
      if (parts[i].isNotEmpty) {
        lines.last.add(
          TextSpan(
            text: parts[i],
            style: color == null ? null : TextStyle(color: color),
          ),
        );
      }
    }
  }

  var position = 0;
  for (final range in ranges) {
    if (range.from < position || range.to > source.length) continue;
    append(source.substring(position, range.from), null);
    append(
      source.substring(range.from, range.to),
      viewerSyntaxColor(range.capture, brightness),
    );
    position = range.to;
  }
  append(source.substring(position), null);
  return lines;
}
