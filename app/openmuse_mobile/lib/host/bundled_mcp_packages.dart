import 'package:flutter/services.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

/// Installed application composition, independent of any remote catalog.
const bundledMcpPackagePins = {
  'io.openmuse/genoffice-pptx@0.9.8':
      'd4d5fc69e39d94f110c584de87c4e3aa968d45d315f2e79c9daeea6c163d9432',
};
const bundledSlidesAsset = 'assets/bundled/slides-0.9.8.mcpb';
final bundledSlidesEntry = RemoteMcpEntry.fromRecord({
  'server': {
    'name': 'io.openmuse/genoffice-pptx',
    'title': 'Slides',
    'description': 'Presentations',
    'version': '0.9.8',
    'packages': [
      {
        'registryType': 'mcpb',
        'identifier': 'asset:/$bundledSlidesAsset',
        'fileSha256': bundledMcpPackagePins.values.single,
      },
    ],
  },
});

/// Uses the existing digest verification, atomic extraction and MCP discovery.
/// No server is contacted or executed. A missing local cache can be repaired
/// from the installed app; unrelated and legacy installation files are retained.
Future<void> installBundledMcpPackages(
  RemoteMcpMarket market, {
  AssetBundle? assets,
}) async {
  final entry = bundledSlidesEntry;
  final policy = market.allowedPackages;
  if (policy != null &&
      policy['${entry.name}@${entry.version}'] != entry.packageSha256) {
    return; // An explicit internal candidate override owns this identity.
  }
  final current = await market.installedEntries();
  if (current.any(
        (item) =>
            item.name == entry.name &&
            item.version == entry.version &&
            item.packageSha256 == entry.packageSha256,
      ) &&
      market.hasOfflineUi(entry) &&
      await market.mcpAppDocument(entry) != null) {
    return;
  }
  final data = await (assets ?? rootBundle).load(bundledSlidesAsset);
  await market.install(
    entry,
    bundledArtifact: data.buffer.asUint8List(
      data.offsetInBytes,
      data.lengthInBytes,
    ),
  );
}
