import 'dart:convert';

import 'package:crypto/crypto.dart';

import 'models.dart';

final _id = RegExp(r'^[a-z0-9]+(?:[._-][a-z0-9]+)*$');
final _digest = RegExp(r'^sha256:[0-9a-f]{64}$');
final _revision = RegExp(r'^(0|[1-9][0-9]*)$');

class MuseFacetContractException extends FormatException {
  MuseFacetContractException(String message) : super(message);
}

void validateMuseFacetValue(MuseFacetSchemaKind kind, Object? input) {
  final value = _object(input);
  switch (kind) {
    case MuseFacetSchemaKind.pluginDescriptor:
      _validatePluginDescriptor(value);
    case MuseFacetSchemaKind.contextContribution:
      _validateContext(value);
    case MuseFacetSchemaKind.domainChange:
      _validateDomainChange(value);
    case MuseFacetSchemaKind.presentationIntent:
      _validateIntent(value);
    case MuseFacetSchemaKind.presentationIntentResult:
      _validateIntentResult(value);
  }
}

void _validatePluginDescriptor(Map<String, Object?> value) {
  _keys(value, const {
    'protocol',
    'pluginId',
    'version',
    'publisher',
    'facets',
    'metadata'
  });
  _equal(value, 'protocol', 'muse.plugin-descriptor/v1');
  _match(value, 'pluginId', _id);
  final version = _string(value, 'version');
  if (!RegExp(r'^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)')
      .hasMatch(version)) {
    throw MuseFacetContractException('version is not strict SemVer');
  }
  final facets = value['facets'];
  if (facets is! List || facets.isEmpty || facets.length > 3) {
    throw MuseFacetContractException('facets must contain 1..3 entries');
  }
  for (final raw in facets) {
    final facet = _object(raw);
    _keys(facet, const {
      'facetKind',
      'artifactRef',
      'runtime',
      'requiresKernel',
      'publishes',
      'consumes',
      'optionalRequires'
    });
    _oneOf(facet, 'facetKind', const {'ui', 'domain', 'agent'});
    _oneOf(facet, 'runtime', const {'flutter', 'rust-host', 'dsh-cordis'});
    _nonBlank(facet, 'artifactRef');
    _nonBlank(facet, 'requiresKernel');
    if (facet['publishes'] is! List || facet['consumes'] is! List) {
      throw MuseFacetContractException('Facet contracts must be arrays');
    }
  }
}

void _validateContext(Map<String, Object?> value) {
  _keys(value, const {
    'protocol',
    'pluginId',
    'pluginVersion',
    'facetInstanceRef',
    'surfaceInstanceRef',
    'surfaceKind',
    'scopeRef',
    'contextType',
    'contextSchemaDigest',
    'contextRevision',
    'epochRef',
    'lane',
    'capturedAt',
    'expiresAt',
    'payload'
  });
  _equal(value, 'protocol', 'muse.context-contribution/v1');
  _match(value, 'pluginId', _id);
  for (final key in const [
    'pluginVersion',
    'facetInstanceRef',
    'surfaceInstanceRef',
    'surfaceKind',
    'scopeRef',
    'contextType',
    'epochRef'
  ]) {
    _nonBlank(value, key);
  }
  _match(value, 'contextSchemaDigest', _digest);
  _match(value, 'contextRevision', _revision);
  _oneOf(value, 'lane', const {'control', 'state'});
  _time(value, 'capturedAt');
  _time(value, 'expiresAt');
  _required(value, 'payload');
}

void _validateDomainChange(Map<String, Object?> value) {
  _keys(value, const {
    'protocol',
    'pluginId',
    'providerInstanceRef',
    'scopeRef',
    'resourceRef',
    'eventType',
    'eventSchemaDigest',
    'domainRevision',
    'epochRef',
    'commandRef',
    'origin',
    'occurredAt',
    'payload'
  });
  _equal(value, 'protocol', 'muse.domain-change/v1');
  _match(value, 'pluginId', _id);
  for (final key in const [
    'providerInstanceRef',
    'scopeRef',
    'resourceRef',
    'eventType',
    'epochRef'
  ]) {
    _nonBlank(value, key);
  }
  _match(value, 'eventSchemaDigest', _digest);
  _match(value, 'domainRevision', _revision);
  _oneOf(value, 'origin',
      MuseMutationOrigin.values.map((value) => value.wireName).toSet());
  _time(value, 'occurredAt');
  _required(value, 'payload');
}

void _validateIntent(Map<String, Object?> value) {
  _keys(value, const {
    'protocol',
    'pluginId',
    'targetSurfaceInstanceRef',
    'scopeRef',
    'intentType',
    'intentSchemaDigest',
    'intentRef',
    'requestedAt',
    'expiresAt',
    'payload'
  });
  _equal(value, 'protocol', 'muse.presentation-intent/v1');
  _match(value, 'pluginId', _id);
  for (final key in const ['scopeRef', 'intentType', 'intentRef']) {
    _nonBlank(value, key);
  }
  _match(value, 'intentSchemaDigest', _digest);
  _time(value, 'requestedAt');
  _time(value, 'expiresAt');
  _required(value, 'payload');
}

void _validateIntentResult(Map<String, Object?> value) {
  _keys(value, const {
    'protocol',
    'intentRef',
    'status',
    'appliedSurfaceInstanceRef',
    'observedDomainRevision',
    'reasonCode',
    'completedAt'
  });
  _equal(value, 'protocol', 'muse.presentation-intent-result/v1');
  _nonBlank(value, 'intentRef');
  _oneOf(value, 'status', const {
    'applied',
    'rejected',
    'stale',
    'not-found',
    'not-supported',
    'surface-closed',
    'timed-out'
  });
  if (value['observedDomainRevision'] != null)
    _match(value, 'observedDomainRevision', _revision);
  _time(value, 'completedAt');
}

String museCanonicalJson(Object? value) {
  if (value == null || value is bool || value is num || value is String)
    return jsonEncode(value);
  if (value is List) return '[${value.map(museCanonicalJson).join(',')}]';
  if (value is Map) {
    final entries = value.entries
        .map((entry) => MapEntry(entry.key.toString(), entry.value))
        .toList()
      ..sort((left, right) => left.key.compareTo(right.key));
    return '{${entries.map((entry) => '${jsonEncode(entry.key)}:${museCanonicalJson(entry.value)}').join(',')}}';
  }
  throw MuseFacetContractException('value is not JSON');
}

String museSchemaDigest(Object? schema) {
  final bytes = <int>[
    ...utf8.encode('muse-schema-v1\u0000'),
    ...utf8.encode(museCanonicalJson(schema)),
  ];
  return 'sha256:${sha256.convert(bytes)}';
}

Map<String, Object?> _object(Object? value) {
  if (value is! Map)
    throw MuseFacetContractException('value must be an object');
  return value.map((key, value) => MapEntry(key.toString(), value));
}

void _keys(Map<String, Object?> value, Set<String> allowed) {
  final unknown = value.keys.where((key) => !allowed.contains(key)).toList();
  if (unknown.isNotEmpty)
    throw MuseFacetContractException('unknown fields: $unknown');
}

void _required(Map<String, Object?> value, String key) {
  if (!value.containsKey(key))
    throw MuseFacetContractException('$key is required');
}

String _string(Map<String, Object?> value, String key) {
  _required(value, key);
  final item = value[key];
  if (item is! String)
    throw MuseFacetContractException('$key must be a string');
  return item;
}

void _nonBlank(Map<String, Object?> value, String key) {
  final item = _string(value, key);
  if (item.isEmpty || item.trim() != item)
    throw MuseFacetContractException('$key must be non-blank');
}

void _match(Map<String, Object?> value, String key, RegExp pattern) {
  if (!pattern.hasMatch(_string(value, key)))
    throw MuseFacetContractException('$key has invalid format');
}

void _equal(Map<String, Object?> value, String key, Object expected) {
  if (value[key] != expected)
    throw MuseFacetContractException('$key must equal $expected');
}

void _oneOf(Map<String, Object?> value, String key, Set<String> options) {
  if (!options.contains(_string(value, key)))
    throw MuseFacetContractException('$key is unsupported');
}

void _time(Map<String, Object?> value, String key) {
  _required(value, key);
  final item = value[key];
  if (item is! int || item < 0 || item > 9007199254740991) {
    throw MuseFacetContractException(
        '$key must be a safe non-negative integer');
  }
}
