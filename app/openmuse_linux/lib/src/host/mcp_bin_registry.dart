import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;
import 'mcp_market.dart';

/// npm's public package.json bin convention inside an audited MCPB. The Host
/// owns only generic verified command launch, not a plugin's CLI arguments.
Future<Map<String, ({String pluginId, String entry})>> discoverMcpBins(
  McpRegistryMarket market,
) async {
  final bins = <String, ({String pluginId, String entry})>{},
      conflicts = <String>{};
  for (final entry in await market.installedEntries()) {
    final directory = market.directoryFor(entry),
        metadata = File(p.join(directory.path, 'package.json'));
    if (!await metadata.exists()) continue;
    Object? value;
    try {
      value = jsonDecode(await metadata.readAsString());
    } on FormatException {
      continue;
    } on FileSystemException {
      continue;
    }
    if (value is! Map) continue;
    if (value['bin'] is! Map) continue;
    for (final bin in (value['bin'] as Map).entries) {
      if (bin.key is! String ||
          bin.value is! String ||
          !RegExp(r'^[a-z][a-z0-9-]{1,40}$').hasMatch(bin.key) ||
          bin.key == 'openmuse') {
        continue;
      }
      if (conflicts.contains(bin.key)) continue;
      final relative = (bin.value as String).replaceAll('\\', '/');
      if (p.isAbsolute(relative) || relative.split('/').contains('..')) {
        continue;
      }
      if (bins.containsKey(bin.key)) {
        bins.remove(bin.key);
        conflicts.add(bin.key as String);
        continue;
      }
      bins[bin.key as String] = (pluginId: entry.name, entry: relative);
    }
  }
  return bins;
}

Future<File> verifiedMcpBin(
  McpRegistryMarket market,
  String pluginId,
  String name,
) async {
  final bin = (await discoverMcpBins(market))[name];
  if (bin == null || bin.pluginId != pluginId) {
    throw StateError('MCPB command unavailable');
  }
  final entry = (await market.installedEntries()).singleWhere(
    (e) => e.name == pluginId,
  );
  final root = market.directoryFor(entry),
      receipt =
          jsonDecode(
                await File(p.join(root.path, 'receipt.json')).readAsString(),
              )
              as Map;
  final digests = receipt['fileDigests'];
  if (digests is! Map) {
    throw StateError('Reinstall this package to enable verified commands');
  }
  if (!digests.containsKey('package.json') || !digests.containsKey(bin.entry)) {
    throw StateError('Command metadata and entry must be verified');
  }
  final canonicalRoot = await root.resolveSymbolicLinks();
  for (final item in digests.entries) {
    final relative = item.key as String;
    if (p.isAbsolute(relative) || relative.split('/').contains('..')) {
      throw const FormatException('Invalid receipt path');
    }
    final file = File(p.join(root.path, relative));
    if (await FileSystemEntity.type(file.path, followLinks: false) !=
            FileSystemEntityType.file ||
        await file.resolveSymbolicLinks() != p.join(canonicalRoot, relative) ||
        sha256.convert(await file.readAsBytes()).toString() != item.value) {
      throw StateError('Installed command files changed');
    }
  }
  return File(p.join(root.path, bin.entry));
}

Future<String> refreshMcpBinShims(
  McpRegistryMarket market,
  Directory directory,
  String launcher,
) async {
  await directory.create(recursive: true);
  for (final file in await directory.list().toList()) {
    if (file is File) await file.delete();
  }
  String quote(String s) => "'${s.replaceAll("'", "'\\''")}'";
  for (final item in (await discoverMcpBins(market)).entries) {
    final file = File(p.join(directory.path, item.key));
    await file.writeAsString(
      '#!/bin/sh\nexec ${quote(launcher)} --data-dir ${quote(market.installRoot.parent.path)} mcp-bin ${quote(item.value.pluginId)} ${quote(item.key)} -- "\$@"\n',
    );
    await Process.run('/bin/chmod', ['700', file.path]);
  }
  return directory.path;
}
