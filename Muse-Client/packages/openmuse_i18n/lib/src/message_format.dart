import 'package:intl/intl.dart';

/// A bounded ICU MessageFormat subset: arguments, number, select, cardinal
/// plural, exact plural arms, nesting, #, and ICU apostrophe escaping.
final class MessageTemplate {
  MessageTemplate(String source) {
    if (source.length > 16000) throw const FormatException('Message too large');
    final parser = _Parser(source);
    _nodes = parser.parse();
    if (parser.index != source.length) {
      throw const FormatException('Unexpected closing brace');
    }
    parameters = Set.unmodifiable(parser.parameters);
  }

  late final List<_Node> _nodes;
  late final Set<String> parameters;

  List<Object?> toData() => _encode(_nodes);

  String format(Map<String, Object?> values, {required String locale}) {
    if (!values.keys.toSet().containsAll(parameters)) {
      throw const FormatException('Missing message argument');
    }
    return _render(_nodes, values, locale, null);
  }
}

sealed class _Node {}

final class _Text extends _Node {
  _Text(this.value);
  final String value;
}

final class _Arg extends _Node {
  _Arg(this.name, this.kind);
  final String name;
  final String kind;
}

final class _Choice extends _Node {
  _Choice(this.name, this.plural, this.arms);
  final String name;
  final bool plural;
  final Map<String, List<_Node>> arms;
}

final class _Hash extends _Node {}

String _render(
  List<_Node> nodes,
  Map<String, Object?> values,
  String locale,
  num? count,
) {
  final result = StringBuffer();
  for (final node in nodes) {
    switch (node) {
      case _Text():
        result.write(node.value);
      case _Hash():
        result.write(
          count == null
              ? '#'
              : NumberFormat.decimalPattern(locale).format(count),
        );
      case _Arg():
        final value = values[node.name];
        if (node.kind == 'number') {
          if (value is! num || !value.isFinite) {
            throw const FormatException('Expected finite number');
          }
          result.write(NumberFormat.decimalPattern(locale).format(value));
        } else {
          result.write(value);
        }
      case _Choice():
        final value = values[node.name];
        String key;
        num? pluralCount = count;
        if (node.plural) {
          if (value is! num || !value.isFinite) {
            throw const FormatException('Plural argument must be finite');
          }
          pluralCount = value;
          final exact =
              '=${value == value.truncateToDouble() ? value.toInt() : value}';
          key = node.arms.containsKey(exact)
              ? exact
              : Intl.pluralLogic(
                  value,
                  locale: locale,
                  zero: 'zero',
                  one: 'one',
                  two: 'two',
                  few: 'few',
                  many: 'many',
                  other: 'other',
                );
        } else {
          if (value == null) {
            throw const FormatException('Null select argument');
          }
          key = value.toString();
        }
        result.write(
          _render(
            node.arms[key] ?? node.arms['other']!,
            values,
            locale,
            pluralCount,
          ),
        );
    }
  }
  return result.toString();
}

final class _Parser {
  _Parser(this.source);
  final String source;
  final Set<String> parameters = {};
  int index = 0;
  int _nodeCount = 0;

  List<_Node> parse({int depth = 0, bool plural = false}) {
    if (depth > 8) {
      throw const FormatException('Message nesting limit exceeded');
    }
    final nodes = <_Node>[];
    var text = StringBuffer();
    void flush() {
      if (text.isNotEmpty) {
        nodes.add(_Text(text.toString()));
        text = StringBuffer();
      }
    }

    while (index < source.length) {
      final char = source[index];
      if (++_nodeCount > 20000) {
        throw const FormatException('Message complexity limit exceeded');
      }
      if (char == '}') break;
      if (char == "'") {
        if (index + 1 < source.length && source[index + 1] == "'") {
          text.write("'");
          index += 2;
          continue;
        }
        if (index + 1 < source.length && "{}#".contains(source[index + 1])) {
          index++;
          var closed = false;
          while (index < source.length) {
            if (source[index] == "'") {
              if (index + 1 < source.length && source[index + 1] == "'") {
                text.write("'");
                index += 2;
                continue;
              }
              index++;
              closed = true;
              break;
            }
            text.write(source[index++]);
          }
          if (!closed) throw const FormatException('Unterminated ICU quote');
          continue;
        }
      }
      if (char == '#' && plural) {
        flush();
        nodes.add(_Hash());
        index++;
        continue;
      }
      if (char != '{') {
        text.write(char);
        index++;
        continue;
      }
      flush();
      index++;
      final name = _word();
      if (!RegExp(r'^[A-Za-z][A-Za-z0-9_]*$').hasMatch(name)) {
        throw const FormatException('Invalid parameter name');
      }
      parameters.add(name);
      _space();
      if (_consume('}')) {
        nodes.add(_Arg(name, 'text'));
        continue;
      }
      _expect(',');
      final kind = _word();
      _space();
      if (kind == 'number') {
        _expect('}');
        nodes.add(_Arg(name, kind));
        continue;
      }
      if (kind != 'plural' && kind != 'select') {
        throw const FormatException('Unsupported ICU format');
      }
      _expect(',');
      final arms = <String, List<_Node>>{};
      while (true) {
        _space();
        if (_consume('}')) break;
        final key = _word();
        if (key.isEmpty ||
            arms.containsKey(key) ||
            (kind == 'plural' &&
                !RegExp(
                  r'^(?:zero|one|two|few|many|other|=-?[0-9]+(?:\.[0-9]+)?)$',
                ).hasMatch(key))) {
          throw const FormatException('Invalid choice arm');
        }
        _space();
        _expect('{');
        arms[key] = parse(depth: depth + 1, plural: plural || kind == 'plural');
        _expect('}');
      }
      if (!arms.containsKey('other')) {
        throw const FormatException('Choice requires other');
      }
      nodes.add(_Choice(name, kind == 'plural', arms));
    }
    flush();
    return nodes;
  }

  void _space() {
    while (index < source.length && source[index].trim().isEmpty) {
      index++;
    }
  }

  String _word() {
    _space();
    final start = index;
    while (index < source.length &&
        !RegExp(r'[\s,{}]').hasMatch(source[index])) {
      index++;
    }
    return source.substring(start, index);
  }

  bool _consume(String token) {
    _space();
    if (index < source.length && source[index] == token) {
      index++;
      return true;
    }
    return false;
  }

  void _expect(String token) {
    if (!_consume(token)) throw FormatException('Expected $token in message');
  }
}

List<Object?> _encode(List<_Node> nodes) => [
  for (final node in nodes)
    switch (node) {
      _Text() => ['text', node.value],
      _Arg() => ['arg', node.name, node.kind],
      _Hash() => ['hash'],
      _Choice() => [
        node.plural ? 'plural' : 'select',
        node.name,
        node.arms.map((k, v) => MapEntry(k, _encode(v))),
      ],
    },
];
