import 'package:flutter/services.dart';

import 'bundle.dart';
import 'release.dart';
import 'dart:convert';

Future<InstalledRelease> loadInstalledRelease({AssetBundle? assets}) async {
  const compiled = String.fromEnvironment('OPENMUSE_RELEASE_JSON');
  if (compiled.isNotEmpty) return InstalledRelease.parse(compiled);
  const installed = String.fromEnvironment('OPENMUSE_DEPLOYMENT_JSON');
  const legacyProfile = String.fromEnvironment('OPENMUSE_DEPLOYMENT_PROFILE');
  if (installed.isNotEmpty || legacyProfile.isNotEmpty) {
    final bundle = await loadDeploymentBundle(assets: assets);
    final region = bundle.snapshot.json['scope']['dataRegion'];
    if (region == 'cn' || region == 'us') {
      final market = region == 'cn' ? 'china' : 'overseas';
      final release =
          jsonDecode(
                await (assets ?? rootBundle).loadString(
                  'packages/openmuse_runtime_config/assets/releases/$market.json',
                ),
              )
              as Map<String, dynamic>;
      final source = installed.isNotEmpty
          ? installed
          : await (assets ?? rootBundle).loadString(
              'packages/openmuse_runtime_config/assets/deployments/$legacyProfile.json',
            );
      for (final entry in release['regions'] as List) {
        if (entry['id'] == region) entry['bundle'] = jsonDecode(source);
      }
      return InstalledRelease.parse(jsonEncode(release));
    }

    final source = installed.isNotEmpty
        ? installed
        : await (assets ?? rootBundle).loadString(
            'packages/openmuse_runtime_config/assets/deployments/$legacyProfile.json',
          );
    return InstalledRelease.parse(
      jsonEncode({
        'schemaVersion': 1,
        'id': 'custom-${bundle.snapshot.storageIdentity}',
        'market': 'custom',
        'defaultLocale': bundle.snapshot.defaultLocale,
        'selection': 'fixed',
        'fixedRegion': region,
        'regions': [
          {
            'id': region,
            'labels': {bundle.snapshot.defaultLocale: region},
            'available': true,
            'bundle': jsonDecode(source),
          },
        ],
      }),
    );
  }
  const market = String.fromEnvironment(
    'OPENMUSE_RELEASE_MARKET',
    defaultValue: 'overseas',
  );
  if (!{'china', 'overseas'}.contains(market)) {
    throw const FormatException('Invalid release market');
  }
  return InstalledRelease.parse(
    await (assets ?? rootBundle).loadString(
      'packages/openmuse_runtime_config/assets/releases/$market.json',
    ),
  );
}

/// Installed/compiled configuration only. Never pass HTTP data to this loader.
Future<DeploymentBundle> loadDeploymentBundle({AssetBundle? assets}) async {
  const installed = String.fromEnvironment('OPENMUSE_DEPLOYMENT_JSON');
  const profile = String.fromEnvironment(
    'OPENMUSE_DEPLOYMENT_PROFILE',
    defaultValue: 'default',
  );
  if (!RegExp(r'^[a-z0-9][a-z0-9_-]*$').hasMatch(profile)) {
    throw const FormatException('Invalid deployment profile');
  }
  final source = installed.isNotEmpty
      ? installed
      : await (assets ?? rootBundle).loadString(
          'packages/openmuse_runtime_config/assets/deployments/$profile.json',
        );
  return DeploymentBundle.parse(source);
}
