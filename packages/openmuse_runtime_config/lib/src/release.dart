import 'dart:convert';
import 'bundle.dart';
import 'client.dart';
import 'model.dart';

/// Trusted installation input. Region trust roots cannot be introduced by a
/// remote release, IP geolocation, language selection, or an account response.
final class InstalledRelease {
  InstalledRelease._(
    this.id,
    this.market,
    this.defaultLocale,
    this.fixedRegion,
    this.regions,
  );
  factory InstalledRelease.parse(String source) {
    if (utf8.encode(source).length > 2 * 1024 * 1024) {
      throw const FormatException('Release too large');
    }
    final data = configObject(jsonDecode(source), 'installed release');
    configKeys(data, {
      'schemaVersion',
      'id',
      'market',
      'defaultLocale',
      'selection',
      'fixedRegion',
      'regions',
    }, 'installed release');
    if (!RegExp(
      r'^[a-z][a-z0-9-]{0,95}$',
    ).hasMatch(configText(data['id'], 'release ID'))) {
      throw const FormatException('Invalid installation identity');
    }
    if (data['schemaVersion'] != 1 ||
        !{'china', 'overseas', 'custom'}.contains(data['market']) ||
        !{'fixed', 'required'}.contains(data['selection'])) {
      throw const FormatException('Invalid release policy');
    }
    final regions = <InstalledRegion>[];
    final raw = data['regions'];
    if (raw is! List || raw.isEmpty || raw.length > 32) {
      throw const FormatException('Invalid region directory');
    }
    final ids = <String>{}, identities = <String>{}, sessionKeys = <String>{};
    final privateOrigins = <String>{};
    for (final item in raw) {
      final region = configObject(item, 'region');
      configKeys(region, {'id', 'labels', 'available', 'bundle'}, 'region');
      final id = configText(region['id'], 'region ID');
      if (!RegExp(r'^[a-z][a-z0-9-]{0,31}$').hasMatch(id) ||
          !ids.add(id) ||
          region['available'] is! bool) {
        throw const FormatException('Invalid region ID or availability');
      }
      final labels = configObject(region['labels'], 'region labels');
      if (labels.isEmpty ||
          labels.values.any(
            (v) =>
                v is! String ||
                v.isEmpty ||
                v.length > 100 ||
                v.contains(RegExp(r'[<>\x00-\x1f]')),
          )) {
        throw const FormatException('Invalid region labels');
      }
      DeploymentBundle? bundle;
      if (region['available'] == true) {
        bundle = DeploymentBundle.parse(jsonEncode(region['bundle']));
        final keys = {
          bundle.snapshot.authSessionStorageKey,
          bundle.snapshot.webAuthSessionStorageKey,
        };
        final origins = {
          for (final entry in bundle.snapshot.services.entries)
            if ({
              ServiceId.auth,
              ServiceId.cloud,
              ServiceId.relay,
            }.contains(entry.key))
              entry.value.origin,
        };
        if (bundle.snapshot.json['scope']['dataRegion'] != id ||
            !identities.add(bundle.snapshot.identity) ||
            keys.intersection(sessionKeys).isNotEmpty ||
            origins.intersection(privateOrigins).isNotEmpty) {
          throw const FormatException(
            'Region identity or session namespace overlaps',
          );
        }
        sessionKeys.addAll(keys);
        privateOrigins.addAll(origins);
        if ((data['market'] == 'china' && id != 'cn') ||
            (data['market'] == 'overseas' && id == 'cn')) {
          throw const FormatException('Region outside installation market');
        }
      } else if (region.containsKey('bundle')) {
        throw const FormatException(
          'Unavailable region cannot contain endpoints',
        );
      }
      regions.add(
        InstalledRegion(
          id,
          Map.unmodifiable(labels.cast<String, String>()),
          bundle,
        ),
      );
    }
    final fixed = data['selection'] == 'fixed'
        ? configText(data['fixedRegion'], 'fixed region')
        : null;
    if (fixed != null && !regions.any((r) => r.id == fixed && r.available)) {
      throw const FormatException('Unavailable fixed region');
    }
    if (data['market'] == 'china' && (fixed != 'cn' || regions.length != 1)) {
      throw const FormatException('Domestic release must be fixed to China');
    }
    return InstalledRelease._(
      configText(data['id'], 'release ID'),
      data['market'] as String,
      configText(data['defaultLocale'], 'default locale'),
      fixed,
      List.unmodifiable(regions),
    );
  }
  final String id, market, defaultLocale;
  final String? fixedRegion;
  final List<InstalledRegion> regions;
}

final class InstalledRegion {
  const InstalledRegion(this.id, this.labels, this.bundle);
  final String id;
  final Map<String, String> labels;
  final DeploymentBundle? bundle;
  bool get available => bundle != null;
  String label(String locale) => labels[locale] ?? labels.values.first;
}

/// Selection is durable before credentials or business adapters are created.
/// A removed/changed saved binding requires a fresh choice, never US fallback.
final class RegionSelection {
  RegionSelection(this.release, this.store);
  final InstalledRelease release;
  final ConfigStore store;
  Future<InstalledRegion?> restore() async {
    if (release.fixedRegion case final fixed?) {
      return release.regions.firstWhere((r) => r.id == fixed);
    }
    try {
      final raw = await store.read();
      if (raw == null || raw.length > 4096) return null;
      final saved = jsonDecode(raw);
      if (saved is! Map ||
          saved['version'] != 1 ||
          saved['release'] != release.id ||
          saved['market'] != release.market) {
        return null;
      }
      for (final region in release.regions) {
        if (region.available &&
            region.id == saved['region'] &&
            region.bundle!.snapshot.identity == saved['binding']) {
          return region;
        }
      }
    } catch (_) {
      /* Corrupt preferences must not guess a residency boundary. */
    }
    return null;
  }

  static const legalRevision = '2026-11-03';

  String _legalBinding(InstalledRegion region) => jsonEncode({
    'revision': legalRevision,
    'binding': region.bundle!.snapshot.identity,
    'terms': region.bundle!.snapshot
        .linkUri(LinkId.terms, locale: region.id == 'cn' ? 'zh-Hans' : 'en')
        .toString(),
    'privacy': region.bundle!.snapshot
        .linkUri(LinkId.privacy, locale: region.id == 'cn' ? 'zh-Hans' : 'en')
        .toString(),
  });

  Future<bool> hasAcceptedLegal(InstalledRegion region) async {
    try {
      final saved = jsonDecode(await store.read() ?? '{}');
      return saved is Map && saved['legalConsent'] == _legalBinding(region);
    } catch (_) {
      return false;
    }
  }

  Future<void> acceptLegal(InstalledRegion region) async {
    await choose(region.id);
    final saved = jsonDecode((await store.read())!) as Map<String, dynamic>;
    saved['legalConsent'] = _legalBinding(region);
    saved['legalAcceptedAt'] = DateTime.now().toUtc().toIso8601String();
    await store.commit(jsonEncode(saved));
  }

  Future<InstalledRegion> choose(String id) async {
    final region = release.regions
        .where((r) => r.id == id && r.available)
        .firstOrNull;
    if (region == null ||
        (release.fixedRegion != null && release.fixedRegion != id)) {
      throw StateError('Region is not available in this installation');
    }
    await store.commit(
      jsonEncode({
        'version': 1,
        'release': release.id,
        'market': release.market,
        'region': id,
        'binding': region.bundle!.snapshot.identity,
      }),
    );
    return region;
  }
}
