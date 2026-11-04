import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'navigation.dart';
import 'settings.dart';

enum ServiceId { auth, cloud, pluginRegistry, updates, relay }

enum LinkId {
  terms('legal.terms'),
  privacy('legal.privacy'),
  accountDeletion('legal.accountDeletion'),
  filing('regulatory.filing');

  const LinkId(this.key);
  final String key;
}

Map<String, dynamic> configObject(Object? value, String field) {
  if (value is! Map<String, dynamic>) {
    throw FormatException('$field must be an object');
  }
  return value;
}

void configKeys(Map<String, dynamic> value, Set<String> allowed, String field) {
  if (value.keys.any((key) => !allowed.contains(key))) {
    throw FormatException('$field contains an unsupported field');
  }
}

String configText(Object? value, String field) {
  if (value is! String || value.trim().isEmpty || value != value.trim()) {
    throw FormatException('$field must be non-empty text');
  }
  return value;
}

List<String> configTexts(Object? value, String field) {
  if (value is! List || value.isEmpty) {
    throw FormatException('$field must be a non-empty list');
  }
  final result = value.map((v) => configText(v, field)).toList();
  if (result.toSet().length != result.length) {
    throw FormatException('$field contains duplicates');
  }
  return List.unmodifiable(result);
}

Uri configHttpsUri(Object? value, String field) {
  final uri = Uri.parse(configText(value, field));
  if (uri.scheme != 'https' ||
      uri.host.isEmpty ||
      uri.authority.contains('@') ||
      uri.hasQuery ||
      uri.hasFragment ||
      uri.pathSegments.any((s) => s == '.' || s == '..')) {
    throw FormatException('$field must be an absolute HTTPS URL');
  }
  return uri;
}

bool _localeTag(String tag) => RegExp(
  r'^[a-z]{2,3}(?:-[A-Z][a-z]{3})?(?:-[A-Z]{2}|-[0-9]{3})?$',
).hasMatch(tag);

/// Validated, immutable data. Parsing bundled data does not authenticate it;
/// remote callers must use SignedConfigVerifier through RuntimeConfigClient.
final class RuntimeConfig {
  RuntimeConfig._({
    required this.json,
    required this.identity,
    required this.releaseId,
    required this.sequence,
    required this.services,
    required this.defaultLocale,
    required this.enabledLocales,
    required this.localeAliases,
    required this.requiredCapabilities,
    required this.minBuild,
    required this.issuedAt,
    required this.expiresAt,
  });

  factory RuntimeConfig.fromJson(Object? document) {
    final data = configObject(document, 'snapshot');
    configKeys(data, {
      'schemaVersion',
      'releaseId',
      'sequence',
      'productId',
      'environment',
      'distributionId',
      'scope',
      'compatibility',
      'deployment',
      'localization',
      'origins',
      'links',
      'authentication',
      'web',
      'updates',
      'support',
      'resources',
      'issuedAt',
      'expiresAt',
      'navigation',
      'regulatory',
      'settings',
    }, 'snapshot');
    ClientSettings.fromJson(data['settings']);
    if (data.containsKey('regulatory')) {
      final regulatory = configObject(data['regulatory'], 'regulatory');
      configKeys(regulatory, {
        'appFilingNumber',
        'websiteFilingNumber',
      }, 'regulatory');
      for (final number in regulatory.values) {
        if (number is! String ||
            number.length > 100 ||
            number.contains(RegExp(r'[<>\x00-\x1f]'))) {
          throw const FormatException('Invalid filing number');
        }
      }
    }
    if (data.containsKey('navigation')) {
      RuntimeNavigation.fromJson(data['navigation']);
    }
    if (data['schemaVersion'] is! int || data['schemaVersion'] != 1) {
      throw const FormatException('Unsupported config schema');
    }
    final deployment = configObject(data['deployment'], 'deployment');
    final scope = configObject(data['scope'], 'scope');
    configKeys(scope, {'dataRegion', 'platform', 'variantId'}, 'scope');
    configKeys(deployment, {'id', 'authBindingId', 'services'}, 'deployment');
    final identity = <String>[
      configText(data['productId'], 'productId'),
      configText(data['environment'], 'environment'),
      configText(data['distributionId'], 'distributionId'),
      configText(deployment['id'], 'deployment.id'),
      configText(deployment['authBindingId'], 'deployment.authBindingId'),
      configText(scope['dataRegion'], 'scope.dataRegion'),
      configText(scope['platform'], 'scope.platform'),
      configText(scope['variantId'], 'scope.variantId'),
    ];
    final sequence = data['sequence'];
    if (sequence is! int || sequence < 0 || sequence > 9007199254740991) {
      throw const FormatException('Invalid config sequence');
    }
    final rawServices = configObject(deployment['services'], 'services');
    if (rawServices.keys.any(
      (key) => !ServiceId.values.any((id) => id.name == key),
    )) {
      throw const FormatException('Unknown service ID');
    }
    final services = <ServiceId, Uri>{};
    for (final id in ServiceId.values) {
      final entry = rawServices[id.name];
      if (entry == null && id == ServiceId.relay) continue;
      final service = configObject(entry, 'services.${id.name}');
      configKeys(service, {'baseUrl', 'credentialScope'}, 'service');
      if (service.containsKey('credentialScope')) {
        configText(service['credentialScope'], 'credentialScope');
      }
      services[id] = configHttpsUri(
        service['baseUrl'],
        'services.${id.name}.baseUrl',
      );
      final base = services[id]!;
      if ((id == ServiceId.cloud || id == ServiceId.relay) &&
          base.path.isNotEmpty &&
          base.path != '/') {
        throw const FormatException(
          'Cloud and relay require origin-only deployment endpoints',
        );
      }
      if (id == ServiceId.auth &&
          base.pathSegments
              .where((v) => v.isNotEmpty)
              .any((v) => !RegExp(r'^[A-Za-z0-9._~-]+$').hasMatch(v))) {
        throw const FormatException('Unsupported authentication gateway path');
      }
    }
    final localization = configObject(data['localization'], 'localization');
    configKeys(localization, {
      'defaultLocale',
      'enabledLocales',
      'aliases',
      'bundleRefs',
      'localeCatalog',
    }, 'localization');
    final locales = configTexts(
      localization['enabledLocales'],
      'enabledLocales',
    );
    final localeCatalog = localization['localeCatalog'] ?? <Object?>[];
    if (localeCatalog is! List || localeCatalog.length > 64) {
      throw const FormatException('Invalid downloadable locale directory');
    }
    final directoryTags = <String>{};
    for (final value in localeCatalog) {
      final option = configObject(value, 'locale option');
      configKeys(option, {
        'tag',
        'label',
        'direction',
        'formatLocale',
        'aliases',
        'uiKey',
      }, 'locale option');
      final tag = configText(option['tag'], 'locale tag');
      if (!_localeTag(tag) ||
          !directoryTags.add(tag) ||
          !locales.contains(tag) ||
          !{'ltr', 'rtl'}.contains(option['direction']) ||
          !_localeTag(configText(option['formatLocale'], 'format locale')) ||
          configText(option['label'], 'locale label').length > 100 ||
          configText(option['uiKey'], 'locale UI key').length > 64) {
        throw const FormatException('Invalid downloadable locale option');
      }
      final optionAliases = option['aliases'];
      if (optionAliases is! List ||
          optionAliases.any((v) => v is! String || !_localeTag(v)) ||
          optionAliases.toSet().length != optionAliases.length) {
        throw const FormatException('Invalid locale aliases');
      }
    }
    if (!locales.every(_localeTag)) {
      throw const FormatException('Invalid canonical locale tag');
    }
    final defaultLocale = configText(
      localization['defaultLocale'],
      'defaultLocale',
    );
    if (!locales.contains(defaultLocale)) {
      throw const FormatException('Default locale must be enabled');
    }
    final aliases = <String, String>{};
    final aliasData = configObject(
      localization['aliases'] ?? <String, dynamic>{},
      'aliases',
    );
    for (final entry in aliasData.entries) {
      final target = configText(entry.value, 'alias target');
      if (!_localeTag(entry.key) ||
          !locales.contains(target) ||
          (locales.contains(entry.key) && entry.key != target)) {
        throw const FormatException('Invalid locale alias');
      }
      aliases[entry.key] = target;
    }
    final origins = configObject(data['origins'], 'origins');
    for (final entry in origins.entries) {
      final uri = configHttpsUri(entry.value, 'origins.${entry.key}');
      if (uri.path.isNotEmpty && uri.path != '/') {
        throw const FormatException('Link origins cannot contain a base path');
      }
    }
    final links = configObject(data['links'], 'links');
    if (LinkId.values.any((id) => !links.containsKey(id.key))) {
      throw const FormatException('Missing required link');
    }
    for (final entry in links.entries) {
      final link = configObject(entry.value, entry.key);
      configKeys(link, {'originRef', 'pathByLocale', 'fallbackLocale'}, 'link');
      if (!origins.containsKey(link['originRef'])) {
        throw const FormatException('Unknown link origin');
      }
      final paths = configObject(link['pathByLocale'], 'pathByLocale');
      final fallback = configText(link['fallbackLocale'], 'fallbackLocale');
      if (!locales.contains(fallback) || !paths.containsKey(fallback)) {
        throw const FormatException('Missing link fallback');
      }
      for (final entry in paths.entries) {
        if (!locales.contains(entry.key)) {
          throw const FormatException('Link locale must be enabled');
        }
        _linkPath(entry.value);
      }
    }
    final web = configObject(data['web'], 'web');
    configKeys(web, {'sessionStorageKey', 'servicePaths'}, 'web');
    final webStorageKey = configText(
      web['sessionStorageKey'],
      'web sessionStorageKey',
    );
    if (!RegExp(r'^[A-Za-z0-9_.-]{1,180}$').hasMatch(webStorageKey)) {
      throw const FormatException('Invalid web session storage key');
    }
    final webPaths = configObject(web['servicePaths'], 'web servicePaths');
    for (final entry in webPaths.entries) {
      if (!ServiceId.values.any((id) => id.name == entry.key)) {
        throw const FormatException('Unknown web service ID');
      }
      final path = _linkPath(entry.value);
      if (entry.key == 'relay' && path != '/') {
        throw const FormatException(
          'Paired relay gateway requires the origin root',
        );
      }
      if (Uri.parse(path).hasQuery || Uri.parse(path).hasFragment) {
        throw const FormatException(
          'Web service path cannot contain query or fragment',
        );
      }
    }
    final auth = configObject(data['authentication'], 'authentication');
    configKeys(auth, {'sessionStorageKey'}, 'authentication');
    final storageKey = configText(
      auth['sessionStorageKey'],
      'sessionStorageKey',
    );
    if (!RegExp(r'^[A-Za-z0-9_.-]{1,180}$').hasMatch(storageKey)) {
      throw const FormatException('Invalid authentication storage key');
    }
    final updates = configObject(data['updates'], 'updates');
    configKeys(updates, {'channel'}, 'updates');
    if (!RegExp(
      r'^[a-z0-9][a-z0-9-]{0,31}$',
    ).hasMatch(configText(updates['channel'], 'update channel'))) {
      throw const FormatException('Invalid update channel');
    }
    final support = configObject(data['support'], 'support');
    configKeys(support, {'email'}, 'support');
    final email = configText(support['email'], 'support email');
    if (!RegExp(
      r'^[A-Za-z0-9.!%+_-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$',
    ).hasMatch(email)) {
      throw const FormatException('Invalid support email');
    }
    final compatibility = configObject(data['compatibility'], 'compatibility');
    configKeys(compatibility, {
      'minBuild',
      'requiredCapabilities',
    }, 'compatibility');
    final capabilities = configTexts(
      compatibility['requiredCapabilities'],
      'requiredCapabilities',
    );
    if (data.containsKey('settings') && !capabilities.contains('settings.v1')) {
      throw const FormatException('Missing runtime settings capability');
    }
    final minBuild = compatibility['minBuild'];
    if (minBuild is! int || minBuild < 0) {
      throw const FormatException('Invalid minBuild');
    }
    final resources = data['resources'] ?? <Object?>[];
    if (resources is! List || resources.length > 32) {
      throw const FormatException('Invalid resource manifest');
    }
    final resourceIds = <String>{};
    var totalBytes = 0;
    for (final value in resources) {
      final resource = configObject(value, 'resource');
      configKeys(resource, {
        'id',
        'url',
        'sha256',
        'maxBytes',
        'kind',
        'locale',
        'namespace',
        'delivery',
      }, 'resource');
      if (!resourceIds.add(configText(resource['id'], 'resource ID'))) {
        throw const FormatException('Duplicate resource ID');
      }
      if (resource.containsKey('delivery') &&
          (resource['delivery'] != 'onDemand' ||
              !{'messages', 'pluginMessages'}.contains(resource['kind']))) {
        throw const FormatException('Unsupported optional resource');
      }
      configHttpsUri(resource['url'], 'resource URL');
      final maxBytes = resource['maxBytes'];
      if (maxBytes is! int ||
          maxBytes < 1 ||
          maxBytes > 512 * 1024 ||
          !RegExp(
            r'^[0-9a-f]{64}$',
          ).hasMatch(configText(resource['sha256'], 'resource digest'))) {
        throw const FormatException('Invalid resource size or digest');
      }
      for (final field in ['kind', 'locale', 'namespace']) {
        if (resource.containsKey(field)) configText(resource[field], field);
      }
      if (resource.containsKey('locale') &&
          !_localeTag(resource['locale'] as String)) {
        throw const FormatException('Invalid resource locale');
      }
      totalBytes += maxBytes;
    }
    if (totalBytes > 4 * 1024 * 1024) {
      throw const FormatException('Resource budget exceeded');
    }
    final refs = localization['bundleRefs'] ?? <Object?>[];
    if (refs is! List ||
        refs.any((id) => id is! String || !resourceIds.contains(id)) ||
        refs.toSet().length != refs.length) {
      throw const FormatException('Invalid required resource references');
    }
    DateTime? timestamp(String key) {
      final value = data[key];
      if (!data.containsKey(key)) return null;
      final text = configText(value, key);
      if (!RegExp(
        r'^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?Z$',
      ).hasMatch(text)) {
        throw FormatException('$key must use UTC ISO time');
      }
      final result = DateTime.parse(text).toUtc();
      if (result.toIso8601String().substring(0, 19) != text.substring(0, 19)) {
        throw FormatException('$key contains an invalid calendar date');
      }
      return result;
    }

    final issued = timestamp('issuedAt');
    final expires = timestamp('expiresAt');
    if ((issued == null) != (expires == null) ||
        (issued != null && !expires!.isAfter(issued))) {
      throw const FormatException('Invalid config lifetime');
    }
    // Round-trip before freezing so callers cannot mutate nested source data.
    final frozen =
        _freeze(jsonDecode(jsonEncode(data))) as Map<String, dynamic>;
    return RuntimeConfig._(
      json: frozen,
      identity: jsonEncode(identity),
      releaseId: configText(data['releaseId'], 'releaseId'),
      sequence: sequence,
      services: Map.unmodifiable(services),
      defaultLocale: defaultLocale,
      enabledLocales: locales,
      localeAliases: Map.unmodifiable(aliases),
      requiredCapabilities: capabilities,
      minBuild: minBuild,
      issuedAt: issued,
      expiresAt: expires,
    );
  }

  final Map<String, dynamic> json;
  RuntimeNavigation? get navigation => json['navigation'] == null
      ? null
      : RuntimeNavigation.fromJson(json['navigation']);
  String? get appFilingNumber {
    final number = (json['regulatory'] as Map?)?['appFilingNumber'] as String?;
    return number == null || number.trim().isEmpty ? null : number;
  }

  /// Collision-free encoding of the complete publication stream identity.
  final String identity;
  ClientSettings get settings => ClientSettings.fromJson(json['settings']);
  bool isEnabled(AppFeature feature) => settings.enabled(feature);
  String get storageIdentity =>
      sha256.convert(utf8.encode(identity)).toString();
  final String releaseId;
  final int sequence;
  final Map<ServiceId, Uri> services;
  final String defaultLocale;
  final List<String> enabledLocales;
  final Map<String, String> localeAliases;
  final List<String> requiredCapabilities;
  final int minBuild;
  final DateTime? issuedAt;
  final DateTime? expiresAt;

  String get authSessionStorageKey =>
      (json['authentication'] as Map)['sessionStorageKey'] as String;
  String get updateChannel => (json['updates'] as Map)['channel'] as String;

  Uri serviceUri(ServiceId id) =>
      services[id] ??
      (throw StateError('Service ${id.name} is not configured'));

  String get supportEmail => (json['support'] as Map)['email'] as String;
  Uri get supportUri => Uri(scheme: 'mailto', path: supportEmail);

  String get webAuthSessionStorageKey =>
      (json['web'] as Map)['sessionStorageKey'] as String;

  Uri webServiceUri(ServiceId id, Uri page) {
    final path = ((json['web'] as Map)['servicePaths'] as Map)[id.name];
    if (path == null) return serviceUri(id);
    final loopback = {
      'localhost',
      '127.0.0.1',
      '::1',
      '[::1]',
    }.contains(page.host);
    if (page.scheme != 'https' && !(loopback && page.scheme == 'http')) {
      throw const FormatException('Web services require HTTPS or loopback');
    }
    return Uri.parse(page.origin).resolve(path as String);
  }

  Uri servicePath(ServiceId id, List<String> segments) {
    if (segments.any(
      (s) =>
          s.isEmpty ||
          s == '.' ||
          s == '..' ||
          s.contains('/') ||
          s.contains('\\'),
    )) {
      throw const FormatException('Invalid service path segment');
    }
    final base = serviceUri(id);
    return base.replace(
      pathSegments: [
        ...base.pathSegments.where((s) => s.isNotEmpty),
        ...segments,
      ],
    );
  }

  String selectLocale(Iterable<String> preferred) {
    for (final tag in preferred) {
      if (enabledLocales.contains(tag)) return tag;
      final alias = localeAliases[tag];
      if (alias != null) return alias;
      // Only drop a region, never a script. Script-sensitive aliases are data.
      final parts = tag.split('-');
      if (parts.length > 1 &&
          RegExp(r'^(?:[A-Z]{2}|[0-9]{3})$').hasMatch(parts.last)) {
        final parent = parts.take(parts.length - 1).join('-');
        if (enabledLocales.contains(parent)) return parent;
      }
    }
    return defaultLocale;
  }

  Uri linkUri(LinkId id, {String? locale}) {
    final link = configObject(
      configObject(json['links'], 'links')[id.key],
      id.key,
    );
    final paths = configObject(link['pathByLocale'], 'pathByLocale');
    final overseasLegal =
        id.key.startsWith('legal.') &&
        (json['scope'] as Map)['dataRegion'] != 'cn' &&
        paths.containsKey('en');
    final selected = overseasLegal ? 'en' : selectLocale([?locale]);
    final path = paths[selected] ?? paths[link['fallbackLocale']];
    return Uri.parse(
      configObject(json['origins'], 'origins')[link['originRef']] as String,
    ).resolve(_linkPath(path));
  }
}

String _linkPath(Object? value) {
  final text = configText(value, 'link path');
  final rawPath = text.split(RegExp(r'[?#]')).first;
  if (rawPath
      .split('/')
      .map(Uri.decodeComponent)
      .any(
        (s) => s == '.' || s == '..' || s.contains('/') || s.contains('\\'),
      )) {
    throw const FormatException('Unsafe encoded link path');
  }
  final uri = Uri.parse(text);
  if (!text.startsWith('/') ||
      text.startsWith('//') ||
      uri.hasAuthority ||
      uri.hasScheme ||
      text.contains('\\') ||
      RegExp(r'[\x00-\x20\x7f]').hasMatch(text) ||
      uri.pathSegments.any(
        (s) => s == '.' || s == '..' || s.contains('/') || s.contains('\\'),
      )) {
    throw const FormatException('Unsafe link path');
  }
  return text;
}

Object? _freeze(Object? value) => switch (value) {
  Map<String, dynamic>() => Map<String, dynamic>.unmodifiable(
    value.map((k, v) => MapEntry(k, _freeze(v))),
  ),
  List() => List<Object?>.unmodifiable(value.map(_freeze)),
  _ => value,
};
