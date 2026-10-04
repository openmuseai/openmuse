library;

final class SpeechDraftSnapshot {
  const SpeechDraftSnapshot({
    required this.text,
    required this.selectionStart,
    required this.selectionEnd,
  });

  final String text;
  final int selectionStart;
  final int selectionEnd;
}

/// Owns exactly one temporary speech segment inside an existing text draft.
/// New partials replace the segment instead of appending duplicate hypotheses.
final class SpeechDraftBuffer {
  SpeechDraftBuffer.begin({
    required String text,
    required int selectionStart,
    required int selectionEnd,
  }) : _base = SpeechDraftSnapshot(
         text: text,
         selectionStart: selectionStart.clamp(0, text.length),
         selectionEnd: selectionEnd.clamp(0, text.length),
       ) {
    if (_base.selectionStart > _base.selectionEnd) {
      throw ArgumentError('selectionStart must not exceed selectionEnd');
    }
  }

  final SpeechDraftSnapshot _base;
  SpeechDraftSnapshot get original => _base;

  SpeechDraftSnapshot applyPartial(String text) => _replace(text.trim());

  SpeechDraftSnapshot applyFinal(String text) => _replace(text.trim());

  SpeechDraftSnapshot cancel() => _base;

  SpeechDraftSnapshot _replace(String value) {
    final before = _base.text.substring(0, _base.selectionStart);
    final after = _base.text.substring(_base.selectionEnd);
    final separator =
        before.isNotEmpty &&
            value.isNotEmpty &&
            !_endsWithWhitespace(before) &&
            !_startsWithPunctuation(value) &&
            _joinsLatinWords(before, value)
        ? ' '
        : '';
    final inserted = '$separator$value';
    final cursor = before.length + inserted.length;
    return SpeechDraftSnapshot(
      text: '$before$inserted$after',
      selectionStart: cursor,
      selectionEnd: cursor,
    );
  }

  static bool _endsWithWhitespace(String value) =>
      RegExp(r'\s$').hasMatch(value);

  static bool _startsWithPunctuation(String value) =>
      RegExp(r'^[，。！？、,.!?]').hasMatch(value);

  static bool _joinsLatinWords(String before, String value) =>
      RegExp(r'[A-Za-z0-9]$').hasMatch(before) &&
      RegExp(r'^[A-Za-z0-9]').hasMatch(value);
}
