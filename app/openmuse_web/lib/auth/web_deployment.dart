import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

/// Check installation and actual host before creating auth or document adapters.
/// Portal query parameters never change this binding.
void validateWebDeployment(
  InstalledRelease release,
  Uri page, {
  required String region,
  required String origin,
  required String basePath,
}) {
  if (release.fixedRegion != region ||
      release.regions.length != 1 ||
      !release.regions.single.available ||
      release.regions.single.id != region) {
    throw const FormatException('Web requires one fixed installed data region');
  }
  final expected = Uri.parse(origin);
  if (expected.scheme != 'https' ||
      expected.userInfo.isNotEmpty ||
      expected.hasQuery ||
      expected.hasFragment ||
      expected.path.isNotEmpty && expected.path != '/' ||
      !basePath.startsWith('/') ||
      !basePath.endsWith('/') ||
      basePath.contains('..') ||
      basePath.contains('//')) {
    throw const FormatException('Invalid Web deployment boundary');
  }
  final loopback = {
    'localhost',
    '127.0.0.1',
    '::1',
    '[::1]',
  }.contains(page.host);
  if ((!loopback || page.scheme != 'http') && page.origin != expected.origin) {
    throw const FormatException('Web installation served from another region');
  }
  if (!page.path.startsWith(basePath)) {
    throw const FormatException('Web installation served outside its mount');
  }
}

Future<InstalledRelease> loadWebInstalledRelease() async {
  final release = await loadInstalledRelease();
  const region = String.fromEnvironment('OPENMUSE_WEB_DATA_REGION');
  const origin = String.fromEnvironment('OPENMUSE_WEB_ORIGIN');
  const basePath = String.fromEnvironment('OPENMUSE_WEB_BASE_PATH');
  const production = bool.fromEnvironment('dart.vm.product');
  if (region.isNotEmpty || production) {
    validateWebDeployment(
      release,
      Uri.base,
      region: region,
      origin: origin,
      basePath: basePath,
    );
  }
  return release;
}
