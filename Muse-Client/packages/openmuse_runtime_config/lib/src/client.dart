import 'dart:async';
import 'dart:convert';
import 'dart:math';

import 'package:crypto/crypto.dart';
import 'package:http/http.dart' as http;

import 'bundle.dart';
import 'model.dart';
import 'signature.dart';

abstract interface class ConfigTransport {
  Future<List<int>> get(Uri uri, {required int maxBytes});
}

/// No account headers, cookies supplied by this SDK, or followed redirects.
/// A web adapter must additionally use a credential-free browser client.
final class HttpConfigTransport implements ConfigTransport {
  HttpConfigTransport({
    required this._client,
    this.timeout = const Duration(seconds: 2),
  });
  final http.Client _client;
  final Duration timeout;

  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    return await () async {
      final request = http.Request('GET', uri)..followRedirects = false;
      final response = await _client.send(request);
      if (response.statusCode != 200 ||
          (response.contentLength != null &&
              response.contentLength! > maxBytes)) {
        throw const FormatException('Config response rejected');
      }
      final bytes = <int>[];
      await for (final chunk in response.stream) {
        if (bytes.length + chunk.length > maxBytes) {
          throw const FormatException('Config response exceeds size limit');
        }
        bytes.addAll(chunk);
      }
      return bytes;
    }().timeout(timeout);
  }
}

/// Implementations atomically replace a whole record, including resource bytes.
/// Use a separate store per installed deployment identity.
abstract interface class ConfigStore {
  Future<String?> read();
  Future<void> commit(String record);
}

final class MemoryConfigStore implements ConfigStore {
  String? _record;
  @override
  Future<String?> read() async => _record;
  @override
  Future<void> commit(String record) async => _record = record;
}

final class RuntimeConfigClient {
  RuntimeConfigClient({
    required this.bundle,
    required this.transport,
    required this.store,
    required this.build,
    required Set<String> capabilities,
    this.validateResources,
    int? cohort,
    DateTime Function()? clock,
  }) : capabilities = Set.unmodifiable(capabilities),
       resourceOrigins = bundle.allowedResourceOrigins,
       _clock = clock ?? DateTime.now,
       _current = bundle.snapshot,
       _highestSequence = bundle.snapshot.sequence,
       _highestPayload = jsonEncode(bundle.snapshot.json),
       _networkExpiresAt = bundle.snapshot.expiresAt,
       _verifier = SignedConfigVerifier(publicKeys: bundle.publicKeys) {
    if (cohort != null && (cohort < 0 || cohort > 99)) {
      throw ArgumentError.value(cohort, 'cohort');
    }
    _cohort = cohort ?? Random.secure().nextInt(100);
    _validateIdentity(bundle.snapshot);
  }

  final DeploymentBundle bundle;
  final ConfigTransport transport;
  final ConfigStore store;
  final int build;
  final Set<String> capabilities;

  /// Validate resource meaning after digest checks and before atomic activation.
  final FutureOr<void> Function(RuntimeConfig, Map<String, List<int>>)?
  validateResources;
  late int _cohort;
  int get cohort => _cohort;

  /// Installed trust policy, never taken from the candidate snapshot.
  final Set<String> resourceOrigins;
  final DateTime Function() _clock;
  final SignedConfigVerifier _verifier;
  RuntimeConfig _current;
  int _highestSequence;
  String _highestPayload;
  DateTime? _networkExpiresAt;
  Object? restoreError;
  DateTime? _trustedTime;
  Map<String, List<int>> _resources = const {};
  Future<RuntimeConfig>? _refreshing;
  Future<void>? _restoring;
  Future<void>? _loadingLocale;
  String? _activeJws;
  final Set<String> _requestedLocales = {};
  bool _restored = false;
  bool _networkUnavailable = false;
  bool get networkUnavailable => _networkUnavailable;

  RuntimeConfig get current => _current;
  Map<String, List<int>> get resources => _resources;
  bool get networkConfigExpired =>
      _networkExpiresAt != null && !_networkExpiresAt!.isAfter(_now());
  RuntimeConfig get currentForNetwork {
    if (_networkUnavailable) {
      throw StateError('Authenticated configuration could not be restored');
    }
    if (networkConfigExpired) {
      throw StateError('Network configuration has expired');
    }
    return _current;
  }

  DateTime _now() {
    final now = _clock().toUtc();
    return _trustedTime != null && now.isBefore(_trustedTime!)
        ? _trustedTime!
        : now;
  }

  DateTime get effectiveTime => _now();

  void _validateIdentity(RuntimeConfig config) {
    if (config.identity != bundle.snapshot.identity ||
        config.authSessionStorageKey != bundle.snapshot.authSessionStorageKey ||
        config.webAuthSessionStorageKey !=
            bundle.snapshot.webAuthSessionStorageKey ||
        config.minBuild > build ||
        !capabilities.containsAll(config.requiredCapabilities) ||
        (!bundle.snapshot.enabledLocales.toSet().containsAll(
              config.enabledLocales,
            ) &&
            !(capabilities.contains('messages.dynamic.v1') &&
                config.requiredCapabilities.contains('messages.dynamic.v1')))) {
      throw const FormatException(
        'Incompatible config identity, build, or capability',
      );
    }
    if (config.requiredCapabilities.contains('messages.dynamic.v1') &&
        !config.enabledLocales.toSet().containsAll(
          bundle.snapshot.enabledLocales,
        )) {
      throw const FormatException(
        'Dynamic languages must retain installed fallback languages',
      );
    }
    for (final entry in config.services.entries) {
      if (!(bundle.allowedServiceOrigins[entry.key]?.contains(
            entry.value.origin,
          ) ??
          false)) {
        throw const FormatException(
          'Service origin is outside installed trust policy',
        );
      }
    }
  }

  void _validateLifetime(RuntimeConfig config, {required bool allowExpired}) {
    final issued = config.issuedAt;
    final expires = config.expiresAt;
    if (issued == null ||
        expires == null ||
        issued.isAfter(_now().add(const Duration(minutes: 5))) ||
        (!allowExpired && !expires.isAfter(_now()))) {
      throw const FormatException('Invalid or expired signed config lifetime');
    }
  }

  /// Restores even an expired authentic snapshot for offline UI resources.
  /// Network users must read currentForNetwork, which fails closed on expiry.
  Future<void> restore() => _restoring ??= _restore();

  Future<void> _restore() async {
    try {
      final record = await store.read();
      if (record == null) return;
      if (record.length > 8 * 1024 * 1024) {
        throw const FormatException('Cache is too large');
      }
      final data = configObject(jsonDecode(record), 'config cache');
      final cachedCohort = data['cohort'];
      if (cachedCohort is int && cachedCohort >= 0 && cachedCohort < 100) {
        _cohort = cachedCohort;
      }
      final candidate = await _verifier.verify(
        configText(data['jws'], 'cached JWS'),
      );
      _validateIdentity(candidate);
      _validateLifetime(candidate, allowExpired: true);
      if (candidate.sequence < _highestSequence ||
          (candidate.sequence == _highestSequence &&
              jsonEncode(candidate.json) != _highestPayload)) {
        throw const FormatException('Cached config rollback');
      }
      _highestSequence = candidate.sequence;
      _highestPayload = jsonEncode(candidate.json);
      _networkExpiresAt = candidate.expiresAt;
      _trustedTime = candidate.issuedAt;
      _networkUnavailable = true;
      final resources = await _stageResources(
        candidate,
        configObject(data['resources'], 'cached resources'),
      );
      await validateResources?.call(candidate, resources);
      _highestSequence = candidate.sequence;
      _current = candidate;
      _resources = resources;
      _activeJws = data['jws'] as String;
      _rememberLocales(candidate, resources);
      _networkUnavailable = false;
      // Only the signed issue time is a trustworthy lower bound on time.
      _trustedTime = candidate.issuedAt;
    } catch (error) {
      restoreError = error;
    } finally {
      _restored = true;
    }
  }

  /// Refreshes coalesce. Restore is always attempted before accepting a remote
  /// version, so a fresh process cannot silently bypass the cached high-water mark.
  Future<RuntimeConfig> refresh() {
    if (_refreshing case final pending?) return pending;
    final future = _refresh();
    _refreshing = future;
    return future.whenComplete(() => _refreshing = null);
  }

  Future<RuntimeConfig> _refresh() async {
    if (_loadingLocale case final loading?) {
      try {
        await loading;
      } catch (_) {}
    }
    if (!_restored) await restore();
    if (bundle.discoveryUris.isEmpty) return currentForNetwork;
    Object? failure;
    for (final endpoint in bundle.discoveryUris) {
      try {
        // A finite cohort path is cacheable and carries no user identifier.
        final distribution =
            bundle.cohortRouting && endpoint.path.endsWith('/active.jws')
            ? endpoint.replace(
                path: endpoint.path.replaceFirst(
                  RegExp(r'/active\.jws$'),
                  '/active/$_cohort.jws',
                ),
              )
            : endpoint;
        final bytes = await transport.get(distribution, maxBytes: 1024 * 1024);
        final jws = utf8.decode(bytes);
        final candidate = await _verifier.verify(jws);
        _validateIdentity(candidate);
        _validateLifetime(candidate, allowExpired: false);
        if (candidate.sequence < _highestSequence ||
            (candidate.sequence == _highestSequence &&
                jsonEncode(candidate.json) != _highestPayload)) {
          throw const FormatException('Config replay or conflicting sequence');
        }
        final staged = await _stageResources(candidate, null);
        await validateResources?.call(candidate, staged);
        await store.commit(
          jsonEncode({
            'jws': jws,
            'cohort': _cohort,
            'resources': staged.map(
              (key, value) => MapEntry(key, base64Encode(value)),
            ),
          }),
        );
        // Commit is complete before exposing either metadata or resources.
        _highestSequence = candidate.sequence;
        _highestPayload = jsonEncode(candidate.json);
        _networkExpiresAt = candidate.expiresAt;
        _trustedTime = candidate.issuedAt;
        _current = candidate;
        _resources = staged;
        _activeJws = jws;
        _rememberLocales(candidate, staged);
        _networkUnavailable = false;
        return candidate;
      } catch (error) {
        failure = error;
      }
    }
    throw failure ?? StateError('No runtime config endpoint available');
  }

  Future<Map<String, List<int>>> _stageResources(
    RuntimeConfig config,
    Map<String, dynamic>? cached,
  ) async {
    final raw = config.json['resources'] ?? <Object?>[];
    if (raw is! List || raw.length > 32) {
      throw const FormatException('Invalid resource manifest');
    }
    final staged = <String, List<int>>{};
    var total = 0;
    for (final item in raw) {
      final resource = configObject(item, 'resource');
      final id = configText(resource['id'], 'resource ID');
      final hash = configText(resource['sha256'], 'resource digest');
      final maxBytes = resource['maxBytes'];
      final uri = configHttpsUri(resource['url'], 'resource URL');
      if (staged.containsKey(id) ||
          !RegExp(r'^[0-9a-f]{64}$').hasMatch(hash) ||
          maxBytes is! int ||
          maxBytes < 1 ||
          maxBytes > 512 * 1024 ||
          !resourceOrigins.contains(uri.origin)) {
        throw const FormatException('Invalid or unauthorized config resource');
      }
      total += maxBytes;
      if (total > 4 * 1024 * 1024) {
        throw const FormatException('Resource budget exceeded');
      }
      final optional = resource['delivery'] == 'onDemand';
      if (optional && cached != null && !cached.containsKey(id)) continue;
      if (optional &&
          cached == null &&
          !_requestedLocales.contains(resource['locale'])) {
        continue;
      }
      final bytes = cached != null
          ? base64Decode(configText(cached[id], 'cached resource'))
          : await transport.get(uri, maxBytes: maxBytes);
      if (bytes.length > maxBytes || sha256.convert(bytes).toString() != hash) {
        throw const FormatException('Config resource integrity check failed');
      }
      staged[id] = List.unmodifiable(bytes);
    }
    final localeData = configObject(
      config.json['localization'],
      'localization',
    );
    final refs = localeData['bundleRefs'] ?? <Object?>[];
    if (refs is! List ||
        refs.any((ref) => ref is! String || !staged.containsKey(ref))) {
      throw const FormatException('Required language resource missing');
    }
    return Map.unmodifiable(staged);
  }

  void _rememberLocales(
    RuntimeConfig config,
    Map<String, List<int>> resources,
  ) {
    for (final raw in config.json['resources'] as List? ?? const []) {
      if (raw['delivery'] == 'onDemand' && resources.containsKey(raw['id'])) {
        _requestedLocales.add(raw['locale'] as String);
      }
    }
  }

  /// Only URLs/digests in the active signed snapshot may be fetched. Serialize
  /// with refresh so an older pack cannot replace a newer snapshot/cache.
  Future<void> loadLocaleResources(String locale) {
    if (_refreshing case final pending?) {
      return pending.then((_) => loadLocaleResources(locale));
    }
    if (_loadingLocale case final pending?) {
      return pending.then(
        (_) => loadLocaleResources(locale),
        onError: (_) => loadLocaleResources(locale),
      );
    }
    final operation = _loadLocaleResources(locale);
    _loadingLocale = operation;
    return operation.whenComplete(() => _loadingLocale = null);
  }

  Future<void> _loadLocaleResources(String locale) async {
    final snapshot = currentForNetwork;
    if (!snapshot.enabledLocales.contains(locale) || _activeJws == null) {
      throw StateError('No verified language manifest');
    }
    final staged = Map<String, List<int>>.of(_resources);
    var found = false;
    for (final item in snapshot.json['resources'] as List? ?? const []) {
      final r = item as Map;
      if (!{'messages', 'pluginMessages'}.contains(r['kind']) ||
          r['locale'] != locale)
        continue;
      found = true;
      final id = r['id'] as String;
      if (staged.containsKey(id)) continue;
      final uri = configHttpsUri(r['url'], 'language resource');
      if (!resourceOrigins.contains(uri.origin)) {
        throw const FormatException('Untrusted language origin');
      }
      final bytes = await transport.get(uri, maxBytes: r['maxBytes'] as int);
      if (bytes.length > (r['maxBytes'] as int) ||
          sha256.convert(bytes).toString() != r['sha256']) {
        throw const FormatException('Language resource integrity check failed');
      }
      staged[id] = List.unmodifiable(bytes);
    }
    if (!found) throw StateError('Language resources are not published');
    await validateResources?.call(snapshot, staged);
    await store.commit(
      jsonEncode({
        'jws': _activeJws,
        'cohort': _cohort,
        'resources': staged.map(
          (key, value) => MapEntry(key, base64Encode(value)),
        ),
      }),
    );
    _resources = Map.unmodifiable(staged);
    _requestedLocales.add(locale);
  }
}
