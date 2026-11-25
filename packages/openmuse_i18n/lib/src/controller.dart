import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';

import 'catalog.dart';
import 'generated_strings.dart';

abstract interface class LocalePreferences {
  Future<String?> read();
  Future<void> write(String value);
}

final class CallbackLocalePreferences implements LocalePreferences {
  CallbackLocalePreferences({
    required this.readValue,
    required this.writeValue,
  });
  final Future<String?> Function() readValue;
  final Future<void> Function(String) writeValue;
  @override
  Future<String?> read() => readValue();
  @override
  Future<void> write(String value) => writeValue(value);
}

final class MemoryLocalePreferences implements LocalePreferences {
  String? _value;
  @override
  Future<String?> read() async => _value;
  @override
  Future<void> write(String value) async => _value = value;
}

Locale localeFromTag(String tag) {
  final parts = tag.split('-');
  String? script;
  String? country;
  for (final part in parts.skip(1)) {
    if (part.length == 4) {
      script = part;
    } else {
      country = part;
    }
  }
  return Locale.fromSubtags(
    languageCode: parts.first,
    scriptCode: script,
    countryCode: country,
  );
}

class OpenMuseLocaleController extends ChangeNotifier {
  OpenMuseLocaleController({
    OpenMuseCatalog? catalog,
    String? defaultTag,
    String? initialTag,
    Iterable<String>? enabledLocales,
    Iterable<String> systemLocales = const [],
    Map<String, String> aliases = const {},
    this.preferences,
  }) : _catalog = catalog ?? OpenMuseCatalog.builtIn,
       _defaultTag =
           defaultTag ?? (catalog ?? OpenMuseCatalog.builtIn).fallbackLocale,
       _enabled = List.unmodifiable(
         enabledLocales ??
             (catalog ?? OpenMuseCatalog.builtIn).locales.map((v) => v.tag),
       ),
       _aliases = Map.unmodifiable(aliases),
       _system = List.of(systemLocales),
       _selected = initialTag {
    if (_enabled.isEmpty ||
        !_enabled.contains(_defaultTag) ||
        !_catalog.locales.map((v) => v.tag).toSet().containsAll(_enabled) ||
        _aliases.values.any((tag) => !_enabled.contains(tag))) {
      throw const FormatException('Invalid installed language configuration');
    }
    _effective = _resolve();
    strings = OpenMuseStrings(() => _catalog, () => tag);
  }
  OpenMuseCatalog _catalog;
  String _defaultTag;
  List<String> _enabled;
  Map<String, String> _aliases;
  List<String> _system;
  String? _selected;
  late String _effective;
  final LocalePreferences? preferences;
  late final OpenMuseStrings strings;
  Future<void> _saving = Future.value();
  int _revision = 0;
  bool _disposed = false;
  Object? preferenceError;
  Object? languageError;
  String? loadingLanguage;
  Future<void> Function(String)? ensureLocale;
  int _languageIntent = 0;
  String? _failedLanguage;
  bool _failedExplicit = false;
  Future<void>? _pendingPreparation;

  OpenMuseCatalog get catalog => _catalog;
  String get tag => _effective;
  bool get followsSystem => _selected == null;
  Locale get locale => localeFromTag(tag);
  List<LocaleOption> get options => List.unmodifiable(
    _catalog.locales.where((v) => _enabled.contains(v.tag)),
  );
  List<Locale> get supportedLocales =>
      options.map((v) => localeFromTag(v.tag)).toList(growable: false);
  String _resolve() => _catalog.resolve(
    [?_selected, ..._system].map((tag) => _aliases[tag] ?? tag),
    enabled: _enabled.where(_catalog.hasTranslations),
    fallback: _defaultTag,
  );

  Future<void> restore() async {
    final before = _revision;
    try {
      final encoded = await preferences?.read();
      if (encoded == null || _revision != before || _disposed) return;
      final data = jsonDecode(encoded);
      if (data is! Map ||
          data['version'] != 1 ||
          !{'selected', 'system'}.contains(data['mode'])) {
        throw const FormatException('Invalid language preference');
      }
      if (data['mode'] == 'selected') {
        final tag = data['locale'];
        if (tag is! String ||
            !RegExp(
              r'^[a-z]{2,3}(?:-[A-Z][a-z]{3})?(?:-[A-Z]{2}|-[0-9]{3})?$',
            ).hasMatch(tag)) {
          throw const FormatException('Unavailable preferred language');
        }
        _selected = tag;
      } else {
        _selected = null;
      }
      _effective = _resolve();
      notifyListeners();
    } catch (error) {
      preferenceError = error;
    }
  }

  void select(Locale value) => selectTag(value.toLanguageTag());
  void selectTag(String value) {
    if (ensureLocale != null &&
        !_catalog.hasTranslations(
          _catalog.resolve([_aliases[value] ?? value], enabled: _enabled),
        )) {
      unawaited(requestTag(value));
      return;
    }
    _languageIntent++;
    languageError = null;
    loadingLanguage = null;
    if (!options.any(
      (v) =>
          v.tag == value ||
          v.aliases.contains(value) ||
          v.tag == _aliases[value],
    )) {
      throw ArgumentError.value(value, 'locale', 'Language is not enabled');
    }
    _selected = _catalog.resolve(
      [_aliases[value] ?? value],
      enabled: _enabled,
      fallback: _defaultTag,
    );
    _change();
  }

  void followSystem() {
    _languageIntent++;
    loadingLanguage = null;
    _selected = null;
    _change();
    unawaited(ensurePreferredLocale());
  }

  void updateSystemLocales(Iterable<String> values) {
    _system = List.of(values);
    if (followsSystem) {
      _languageIntent++;
      loadingLanguage = null;
      _effective = _resolve();
      notifyListeners();
      unawaited(ensurePreferredLocale());
    }
  }

  void replaceCatalog(OpenMuseCatalog value) {
    if (!value.locales.map((v) => v.tag).toSet().containsAll(_enabled)) {
      throw const FormatException('Catalog removed an enabled language');
    }
    _catalog = value;
    _effective = _resolve();
    notifyListeners();
  }

  void applyConfiguration({
    required OpenMuseCatalog catalog,
    required String defaultTag,
    required Iterable<String> enabledLocales,
    Map<String, String> aliases = const {},
  }) {
    final enabled = List<String>.unmodifiable(enabledLocales);
    if (enabled.isEmpty ||
        !enabled.contains(defaultTag) ||
        !catalog.locales.map((v) => v.tag).toSet().containsAll(enabled)) {
      throw const FormatException('Unavailable runtime language configuration');
    }
    _aliases = Map.unmodifiable(aliases);
    _catalog = catalog;
    _defaultTag = defaultTag;
    _enabled = enabled;
    _effective = _resolve();
    notifyListeners();
    unawaited(ensurePreferredLocale());
  }

  Future<void> requestTag(String value) async {
    if (!options.any(
      (v) =>
          v.tag == value ||
          v.aliases.contains(value) ||
          v.tag == _aliases[value],
    )) {
      throw ArgumentError.value(value, 'locale', 'Language is not enabled');
    }
    final tag = _catalog.resolve([_aliases[value] ?? value], enabled: _enabled);
    await _prepare(tag, explicit: true);
  }

  Future<void> ensurePreferredLocale() async {
    if (_disposed || ensureLocale == null) return;
    final desired = _catalog.resolve(
      [?_selected, ..._system].map((v) => _aliases[v] ?? v),
      enabled: _enabled,
      fallback: _defaultTag,
    );
    if (loadingLanguage == desired) {
      await _pendingPreparation;
      return;
    }
    if (!_catalog.hasTranslations(desired)) {
      await _prepare(desired, explicit: false);
    }
  }

  Future<void> _prepare(String tag, {required bool explicit}) {
    final operation = _runPreparation(tag, explicit: explicit);
    _pendingPreparation = operation;
    return operation.whenComplete(() {
      if (identical(_pendingPreparation, operation)) _pendingPreparation = null;
    });
  }

  Future<void> _runPreparation(String tag, {required bool explicit}) async {
    final intent = ++_languageIntent;
    try {
      if (!_catalog.hasTranslations(tag)) {
        loadingLanguage = tag;
        languageError = null;
        notifyListeners();
        if (ensureLocale == null) {
          throw StateError('Language download is unavailable');
        }
        await ensureLocale!(tag);
      }
      if (_disposed || intent != _languageIntent) return;
      if (!_catalog.hasTranslations(tag)) {
        throw StateError('Language package is empty');
      }
      if (explicit) _selected = tag;
      languageError = null;
      _change();
    } catch (error) {
      if (!_disposed && intent == _languageIntent) {
        languageError = error;
        _failedLanguage = tag;
        _failedExplicit = explicit;
        notifyListeners();
      }
    } finally {
      if (!_disposed && intent == _languageIntent) {
        loadingLanguage = null;
        notifyListeners();
      }
    }
  }

  Future<void> retryLanguage() async {
    if (_failedLanguage case final tag?) {
      await _prepare(tag, explicit: _failedExplicit);
    }
  }

  void _change() {
    _revision++;
    _effective = _resolve();
    notifyListeners();
    final encoded = jsonEncode({
      'version': 1,
      'mode': followsSystem ? 'system' : 'selected',
      if (!followsSystem) 'locale': _selected,
    });
    _saving = _saving.then((_) async {
      try {
        await preferences?.write(encoded);
      } catch (error) {
        preferenceError = error;
      }
    });
  }

  Future<void> get saved => _saving;
  @override
  void dispose() {
    _disposed = true;
    _revision++;
    super.dispose();
  }
}

class OpenMuseLocaleScope extends InheritedNotifier<OpenMuseLocaleController> {
  const OpenMuseLocaleScope({
    super.key,
    required OpenMuseLocaleController controller,
    required super.child,
  }) : super(notifier: controller);
  static OpenMuseLocaleController? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<OpenMuseLocaleScope>()
      ?.notifier;
}

OpenMuseStrings stringsOf(BuildContext context) =>
    OpenMuseLocaleScope.maybeOf(context)?.strings ??
    OpenMuseStrings.forLocale(
      Localizations.maybeLocaleOf(context)?.toLanguageTag() ??
          OpenMuseCatalog.builtIn.fallbackLocale,
    );

final class OpenMuseStringsDelegate
    extends LocalizationsDelegate<OpenMuseStrings> {
  const OpenMuseStringsDelegate(this.catalog);
  final OpenMuseCatalog catalog;
  @override
  bool isSupported(Locale locale) => catalog.locales.any(
    (v) =>
        v.tag == locale.toLanguageTag() ||
        v.aliases.contains(locale.toLanguageTag()),
  );
  @override
  Future<OpenMuseStrings> load(Locale locale) => SynchronousFuture(
    OpenMuseStrings.forLocale(locale.toLanguageTag(), catalog: catalog),
  );
  @override
  bool shouldReload(OpenMuseStringsDelegate old) => old.catalog != catalog;
}
