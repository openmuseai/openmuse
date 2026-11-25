import 'dart:convert';

import 'model.dart';
import 'signature.dart';

/// This input is installed/compiled data, never an HTTP response or ordinary
/// process environment override. Remote data must pass SignedConfigVerifier.
final class DeploymentBundle {
  DeploymentBundle._(
    this.snapshot,
    this.discoveryUris,
    this.publicKeys,
    this.allowedServiceOrigins,
    this.allowedResourceOrigins,
    this.cohortRouting,
  );

  factory DeploymentBundle.parse(String source) {
    if (utf8.encode(source).length > 512 * 1024) {
      throw const FormatException('Deployment bundle is too large');
    }
    final data = configObject(jsonDecode(source), 'deployment bundle');
    configKeys(data, {'bootstrap', 'snapshot'}, 'deployment bundle');
    final snapshot = RuntimeConfig.fromJson(data['snapshot']);
    final bootstrap = configObject(data['bootstrap'], 'bootstrap');
    configKeys(bootstrap, {
      'cohortRouting',
      'discoveryUrls',
      'publicKeys',
      'allowedServiceOrigins',
      'allowedResourceOrigins',
    }, 'bootstrap');
    final cohortRouting = bootstrap['cohortRouting'] ?? false;
    if (cohortRouting is! bool) {
      throw const FormatException('Invalid cohort routing');
    }
    final discovery = bootstrap['discoveryUrls'];
    if (discovery is! List) {
      throw const FormatException('Invalid discoveryUrls');
    }
    final keys = configObject(bootstrap['publicKeys'], 'publicKeys');
    final parsedKeys = <String, List<int>>{};
    for (final entry in keys.entries) {
      configText(entry.key, 'key ID');
      final bytes = decodeConfigBase64(configText(entry.value, 'public key'));
      if (bytes.length != 32) {
        throw const FormatException('Invalid Ed25519 public key');
      }
      parsedKeys[entry.key] = List.unmodifiable(bytes);
    }
    if (discovery.isNotEmpty && parsedKeys.isEmpty) {
      throw const FormatException(
        'Remote config requires an installed public key',
      );
    }
    final allowed = <ServiceId, Set<String>>{};
    final rawAllowed = configObject(
      bootstrap['allowedServiceOrigins'],
      'allowedServiceOrigins',
    );
    configKeys(
      rawAllowed,
      ServiceId.values.map((id) => id.name).toSet(),
      'allowedServiceOrigins',
    );
    for (final id in ServiceId.values) {
      if (!snapshot.services.containsKey(id) &&
          !rawAllowed.containsKey(id.name)) {
        continue;
      }
      final values = configTexts(
        rawAllowed[id.name],
        'allowedServiceOrigins.${id.name}',
      );
      final origins = values.map((value) {
        final uri = configHttpsUri(value, 'allowed service origin');
        if (uri.path.isNotEmpty && uri.path != '/') {
          throw const FormatException('Expected an origin');
        }
        return uri.origin;
      }).toSet();
      if (snapshot.services[id] case final uri?) {
        if (!origins.contains(uri.origin)) {
          throw const FormatException(
            'Bundled service origin is not authorized',
          );
        }
      }
      allowed[id] = Set.unmodifiable(origins);
    }
    final resourceOrigins = bootstrap['allowedResourceOrigins'];
    if (resourceOrigins is! List) {
      throw const FormatException('allowedResourceOrigins must be a list');
    }
    return DeploymentBundle._(
      snapshot,
      List.unmodifiable(
        discovery.map((v) => configHttpsUri(v, 'discovery URL')),
      ),
      Map.unmodifiable(parsedKeys),
      Map.unmodifiable(allowed),
      Set.unmodifiable(
        resourceOrigins.map((value) {
          final uri = configHttpsUri(value, 'resource origin');
          if (uri.path.isNotEmpty && uri.path != '/') {
            throw const FormatException('Expected a resource origin');
          }
          return uri.origin;
        }),
      ),
      cohortRouting,
    );
  }

  final bool cohortRouting;
  final RuntimeConfig snapshot;
  final List<Uri> discoveryUris;
  final Map<String, List<int>> publicKeys;
  final Map<ServiceId, Set<String>> allowedServiceOrigins;
  final Set<String> allowedResourceOrigins;
}
