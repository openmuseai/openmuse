import 'dart:convert';

import 'generated_catalog.dart';
import 'message_format.dart';

final class LocaleOption {
  LocaleOption(Map<String, dynamic> data)
    : tag = data['tag'] as String,
      label = data['label'] as String,
      formatLocale = data['formatLocale'] as String,
      direction = data['direction'] as String,
      uiKey = data['uiKey'] as String,
      aliases = List<String>.unmodifiable(data['aliases'] as List);
  final String tag;
  final String label;
  final String formatLocale;
  final String direction;
  final String uiKey;
  final List<String> aliases;
}

final class OpenMuseCatalog {
  OpenMuseCatalog._(
    this.templateLocale,
    this.fallbackLocale,
    this.locales,
    this._messages,
    this._allowed, [
    this.revision = 0,
    this.sourceRevision = 1,
  ]);
  static final builtIn = OpenMuseCatalog.fromJson(
    jsonDecode(builtInCatalogJson),
  );

  factory OpenMuseCatalog.fromJson(Object? document) {
    if (document is! Map<String, dynamic>) {
      throw const FormatException('Invalid catalog');
    }
    final metadata = document['metadata'] as Map<String, dynamic>;
    if (metadata['schemaVersion'] != 1) {
      throw const FormatException('Unknown catalog schema');
    }
    final sourceRevision = metadata['sourceRevision'];
    if (sourceRevision is! int || sourceRevision < 1) {
      throw const FormatException('Invalid message contract revision');
    }
    final locales = [
      for (final item in metadata['locales'] as List)
        LocaleOption(Map<String, dynamic>.from(item as Map)),
    ];
    final tags = locales.map((v) => v.tag).toSet();
    final templateLocale = metadata['templateLocale'] as String;
    final fallbackLocale = metadata['fallbackLocale'] as String;
    if (!tags.contains(templateLocale) || !tags.contains(fallbackLocale)) {
      throw const FormatException('Invalid catalog defaults');
    }
    final messages = <String, Map<String, MessageTemplate>>{};
    final allowed = <String, Set<String>>{};
    for (final entry in (document['entries'] as Map<String, dynamic>).entries) {
      final definition = entry.value as Map<String, dynamic>;
      final parameters = Set<String>.from(definition['parameters'] as List);
      allowed[entry.key] = Set.unmodifiable(parameters);
      for (final translation
          in (definition['messages'] as Map<String, dynamic>).entries) {
        if (!tags.contains(translation.key) || translation.value is! String) {
          throw const FormatException('Invalid message locale or value');
        }
        final parsed = MessageTemplate(translation.value as String);
        if (!parameters.containsAll(parsed.parameters)) {
          throw const FormatException('Undeclared message parameter');
        }
        messages.putIfAbsent(translation.key, () => {})[entry.key] = parsed;
      }
      if (messages[templateLocale]?[entry.key] == null) {
        throw const FormatException('Missing template message');
      }
    }
    return OpenMuseCatalog._(
      templateLocale,
      fallbackLocale,
      List.unmodifiable(locales),
      Map.unmodifiable(
        messages.map(
          (k, v) => MapEntry(k, Map<String, MessageTemplate>.unmodifiable(v)),
        ),
      ),
      Map.unmodifiable(allowed),
      0,
      sourceRevision,
    );
  }

  final int revision;
  final int sourceRevision;
  final String templateLocale;
  final String fallbackLocale;
  final List<LocaleOption> locales;
  final Map<String, Map<String, MessageTemplate>> _messages;
  final Map<String, Set<String>> _allowed;

  bool hasTranslations(String tag) => _messages[tag]?.isNotEmpty ?? false;

  OpenMuseCatalog extendLocales(Iterable<Object?> directory) {
    final options = List<LocaleOption>.of(locales);
    for (final raw in directory) {
      final option = LocaleOption(Map<String, dynamic>.from(raw as Map));
      if (options.any((v) => v.tag == option.tag)) continue;
      if (option.formatLocale.split('-').first != option.tag.split('-').first ||
          option.label.contains(RegExp(r'[<>\x00-\x1f]'))) {
        throw const FormatException('Invalid locale presentation metadata');
      }
      options.add(option);
    }
    return OpenMuseCatalog._(
      templateLocale,
      fallbackLocale,
      List.unmodifiable(options),
      _messages,
      _allowed,
      revision,
      sourceRevision,
    );
  }

  OpenMuseCatalog withRevision(int value) => OpenMuseCatalog._(
    templateLocale,
    fallbackLocale,
    locales,
    _messages,
    _allowed,
    value,
    sourceRevision,
  );

  String resolve(
    Iterable<String> desired, {
    Iterable<String>? enabled,
    String? fallback,
  }) {
    final accepted = (enabled ?? locales.map((v) => v.tag)).toSet();
    if (accepted.isEmpty) throw const FormatException('No usable languages');
    for (final tag in desired) {
      for (final option in locales) {
        if (accepted.contains(option.tag) &&
            (option.tag == tag || option.aliases.contains(tag))) {
          return option.tag;
        }
      }
      final parts = tag.split('-');
      if (parts.length > 1 &&
          RegExp(r'^(?:[A-Z]{2}|[0-9]{3})$').hasMatch(parts.last)) {
        final parent = parts.take(parts.length - 1).join('-');
        if (accepted.contains(parent) && locales.any((v) => v.tag == parent)) {
          return parent;
        }
      }
    }
    final defaultTag = fallback ?? fallbackLocale;
    return accepted.contains(defaultTag)
        ? defaultTag
        : locales.firstWhere((v) => accepted.contains(v.tag)).tag;
  }

  LocaleOption option(String tag) =>
      locales.firstWhere((v) => v.tag == resolve([tag]));

  String format(
    String locale,
    String id, [
    Map<String, Object?> arguments = const {},
  ]) {
    final selected = resolve([locale]);
    final message = _messages[selected]?[id] ?? _messages[templateLocale]?[id];
    if (message == null) throw FormatException('Unknown message ID: $id');
    return message.format(arguments, locale: option(selected).formatLocale);
  }

  /// Remote language data is accepted only after the runtime configuration SDK
  /// has authenticated its resource digest. Parse the complete update before
  /// returning a new immutable catalog, keeping the old catalog on any error.
  Map<String, Object?> namespaceData(String locale, String namespace) {
    final tag = resolve([locale]);
    final messages = {...?_messages[templateLocale], ...?_messages[tag]};
    return {
      'locale': tag,
      'direction': option(tag).direction,
      'revision': revision,
      'messages': {
        for (final entry in messages.entries)
          if (entry.key.startsWith('$namespace.'))
            entry.key: entry.value.toData(),
      },
    };
  }

  OpenMuseCatalog overlay(Iterable<List<int>> resources) {
    final updated = _messages.map(
      (k, v) => MapEntry(k, Map<String, MessageTemplate>.of(v)),
    );
    for (final bytes in resources) {
      if (bytes.length > 512 * 1024) {
        throw const FormatException('Language package too large');
      }
      final data = jsonDecode(utf8.decode(bytes));
      if (data is! Map<String, dynamic> ||
          data['schemaVersion'] != 1 ||
          data['sourceRevision'] != sourceRevision ||
          data.keys.any(
            (key) => !{
              'schemaVersion',
              'sourceRevision',
              'locale',
              'namespace',
              'messages',
            }.contains(key),
          ) ||
          data['messages'] is! Map<String, dynamic>) {
        throw const FormatException('Invalid language package');
      }
      final locale = data['locale'];
      final namespace = data['namespace'];
      if (locale is! String ||
          !locales.any((v) => v.tag == locale) ||
          namespace is! String ||
          !RegExp(r'^[a-z][a-z0-9_.]*$').hasMatch(namespace)) {
        throw const FormatException('Unsupported language namespace');
      }
      final translations = data['messages'] as Map<String, dynamic>;
      if (translations.length > 4000) {
        throw const FormatException('Too many translations');
      }
      for (final entry in translations.entries) {
        if (!entry.key.startsWith('$namespace.') ||
            !_allowed.containsKey(entry.key) ||
            entry.value is! String) {
          throw const FormatException('Unknown or cross-namespace message');
        }
        final message = MessageTemplate(entry.value as String);
        final expected =
            _messages[locale]?[entry.key] ??
            _messages[templateLocale]![entry.key]!;
        if (message.parameters.length != expected.parameters.length ||
            !message.parameters.containsAll(expected.parameters)) {
          throw const FormatException('Translation parameter mismatch');
        }
        updated.putIfAbsent(locale, () => {})[entry.key] = message;
      }
    }
    return OpenMuseCatalog._(
      templateLocale,
      fallbackLocale,
      locales,
      Map.unmodifiable(
        updated.map(
          (k, v) => MapEntry(k, Map<String, MessageTemplate>.unmodifiable(v)),
        ),
      ),
      _allowed,
      revision + 1,
      sourceRevision,
    );
  }
}
