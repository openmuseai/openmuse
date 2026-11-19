import 'dart:convert';

/// Test/operator tooling only. Understand the existing accounts pseudo-object
/// without treating intervening comments or later tokens as a password.
({String email, String password}) parseLocalAccount(String text) {
  final marker = RegExp(
    r'^\s*accounts\s*:\s*\[',
    multiLine: true,
  ).firstMatch(text);
  final start = marker == null
      ? text.indexOf('[')
      : marker.start + marker.group(0)!.lastIndexOf('[');
  if (start < 0) throw const FormatException('Local account format is invalid');
  var depth = 0, inString = false, escaped = false, line = false, block = false;
  var quoted = '';
  final clean = StringBuffer(), values = <String>[];
  for (var i = start; i < text.length; i++) {
    final c = text[i], next = i + 1 < text.length ? text[i + 1] : '';
    if (line) {
      if (c == '\n' || c == '\r') {
        line = false;
        clean.writeln();
      }
      continue;
    }
    if (block) {
      if (c == '*' && next == '/') {
        block = false;
        i++;
        clean.write(' ');
      }
      continue;
    }
    if (inString) {
      clean.write(c);
      quoted += c;
      if (escaped) {
        escaped = false;
      } else if (c == '\\') {
        escaped = true;
      } else if (c == '"') {
        inString = false;
        try {
          values.add(jsonDecode(quoted) as String);
        } on Object {
          throw const FormatException('Local account format is invalid');
        }
      }
      continue;
    }
    if (c == '/' && next == '/') {
      line = true;
      i++;
      continue;
    }
    if (c == '/' && next == '*') {
      block = true;
      i++;
      continue;
    }
    clean.write(c);
    if (c == '"') {
      inString = true;
      quoted = '"';
      continue;
    }
    if (c == '[') depth++;
    if (c == ']' && --depth == 0) {
      Object? pair = values;
      try {
        final parsed = jsonDecode(clean.toString());
        if (parsed is List && parsed.length == 1 && parsed[0] is Map) {
          pair = [parsed[0]['email'], parsed[0]['password']];
        } else {
          pair = parsed;
        }
      } on FormatException {
        /* Legacy two-value pseudo-object. */
      }
      if (pair is! List ||
          pair.length != 2 ||
          pair[0] is! String ||
          pair[1] is! String ||
          !RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]+$').hasMatch(pair[0] as String) ||
          (pair[1] as String).isEmpty) {
        throw const FormatException('Local account format is invalid');
      }
      return (email: pair[0] as String, password: pair[1] as String);
    }
  }
  throw const FormatException('Local account format is invalid');
}
