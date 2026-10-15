import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as path;
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// A market entry points to a complete, immutable JSON artifact.
@immutable
final class MobileMarketEntry {
  const MobileMarketEntry({
    required this.id,
    required this.name,
    required this.version,
    required this.description,
    required this.author,
    required this.platforms,
    required this.bundle,
    required this.sha256,
    this.source,
    this.categories = const [],
    this.media = const [],
  });

  final String id;
  final String name;
  final String version;
  final String description;
  final String author;
  final List<String> platforms;
  final String bundle;
  final String sha256;
  final String? source;
  final List<String> categories;
  final List<MobileMarketMedia> media;

  factory MobileMarketEntry.fromJson(Map<String, Object?> json) =>
      MobileMarketEntry(
        id: json['id'] as String,
        name: json['name'] as String,
        version: json['version'] as String,
        description: json['description'] as String,
        author: json['author'] as String,
        platforms: (json['platforms'] as List).cast<String>(),
        bundle: json['bundle'] as String,
        sha256: json['sha256'] as String,
        source: json['source'] as String?,
        categories: (json['categories'] as List? ?? const []).cast<String>(),
        media: [
          for (final item in json['media'] as List? ?? const [])
            MobileMarketMedia.fromJson(Map<String, Object?>.from(item as Map)),
        ],
      );
}

@immutable
final class MobileMarketMedia {
  const MobileMarketMedia({required this.type, required this.uri, this.poster});
  final String type;
  final String uri;
  final String? poster;

  factory MobileMarketMedia.fromJson(Map<String, Object?> json) {
    final type = json['type'];
    final uri = json['uri'];
    if (!const {'image', 'gif', 'video'}.contains(type) ||
        uri is! String ||
        uri.isEmpty ||
        (json['poster'] != null && json['poster'] is! String)) {
      throw const FormatException('invalid market media');
    }
    return MobileMarketMedia(
      type: type as String,
      uri: uri,
      poster: json['poster'] as String?,
    );
  }
}

@immutable
final class MobilePluginBundle {
  const MobilePluginBundle({
    required this.id,
    required this.version,
    required this.js,
    required this.manifest,
  });
  final String id;
  final String version;
  final String js;
  final OpenMusePluginManifestV2 manifest;

  factory MobilePluginBundle.fromJson(Map<String, Object?> json) {
    if (json['format'] != 'openmuse-mobile-js/v1' ||
        json['uiProtocol'] != 'a2ui/v0.9' ||
        json['engine'] != 'quickjs' ||
        json['id'] is! String ||
        json['version'] is! String ||
        json['js'] is! String ||
        json['manifest'] is! Map) {
      throw const FormatException('unsupported Mobile plugin bundle');
    }
    final manifest = OpenMusePluginManifestV2.fromJson(json['manifest']);
    if (manifest.id != json['id'] ||
        manifest.version != json['version'] ||
        manifest.uiRuntime.kind != OpenMuseUiRuntimeKind.a2ui ||
        manifest.executionConnector.kind !=
            OpenMuseExecutionConnectorKind.quickjs ||
        manifest.artifacts.isEmpty ||
        manifest.artifacts.any(
          (artifact) =>
              artifact.kind != OpenMuseArtifactKind.jsBundle ||
              artifact.digest.value !=
                  sha256.convert(utf8.encode(json['js'] as String)).toString(),
        )) {
      throw const FormatException(
        'Mobile plugin manifest does not match JS artifact',
      );
    }
    return MobilePluginBundle(
      id: json['id'] as String,
      version: json['version'] as String,
      js: json['js'] as String,
      manifest: manifest,
    );
  }
}

/// Installation is a Host operation. Artifact bytes are checked before use.
final class MobilePluginStore extends ChangeNotifier {
  MobilePluginStore(this.directory);
  final Directory directory;
  final Map<String, MobilePluginBundle> _installed = {};
  final Map<String, String> _digests = {};
  final Set<String> _disabled = {};

  bool isInstalled(String id) => _installed.containsKey(id);
  bool isEnabled(String id) => isInstalled(id) && !_disabled.contains(id);
  MobilePluginBundle? installed(String id) => _installed[id];
  Iterable<MobilePluginBundle> get installedBundles => _installed.values;

  Future<void> restore() async {
    await directory.create(recursive: true);
    final state = File(path.join(directory.path, 'state.json'));
    if (!await state.exists()) return;
    final decoded = jsonDecode(await state.readAsString()) as Map;
    _disabled.addAll((decoded['disabled'] as List? ?? []).cast<String>());
    final records = Map<String, Object?>.from(
      decoded['installed'] as Map? ?? {},
    );
    for (final record in records.entries) {
      final recordValue = record.value;
      if (!_safeId(record.key) || recordValue is! Map) continue;
      final version = recordValue['version'];
      final digest = recordValue['sha256'];
      if (version is! String ||
          digest is! String ||
          !RegExp(r'^[0-9a-f]{64}$').hasMatch(digest))
        continue;
      final file = File(path.join(directory.path, record.key, '$version.json'));
      if (!await file.exists()) continue;
      try {
        final bytes = await file.readAsBytes();
        if (sha256.convert(bytes).toString() != digest) continue;
        final bundle = MobilePluginBundle.fromJson(
          Map<String, Object?>.from(jsonDecode(utf8.decode(bytes)) as Map),
        );
        if (bundle.id == record.key && bundle.version == version) {
          _installed[record.key] = bundle;
          _digests[record.key] = digest;
        }
      } catch (_) {
        // A damaged artifact is unavailable rather than executable.
      }
    }
    notifyListeners();
  }

  Future<void> install(MobileMarketEntry entry, List<int> bytes) async {
    if (!_safeId(entry.id) ||
        !RegExp(r'^\d+\.\d+\.\d+$').hasMatch(entry.version)) {
      throw const FormatException('invalid plugin identity');
    }
    if (bytes.length > 1024 * 1024 ||
        sha256.convert(bytes).toString() != entry.sha256) {
      throw const FormatException('plugin artifact digest mismatch');
    }
    final bundle = MobilePluginBundle.fromJson(
      Map<String, Object?>.from(jsonDecode(utf8.decode(bytes)) as Map),
    );
    if (bundle.id != entry.id || bundle.version != entry.version) {
      throw const FormatException('plugin bundle identity mismatch');
    }
    final destination = File(
      path.join(directory.path, entry.id, '${entry.version}.json'),
    );
    await destination.parent.create(recursive: true);
    final temporary = File('${destination.path}.tmp');
    await temporary.writeAsBytes(bytes, flush: true);
    await temporary.rename(destination.path);
    _installed[entry.id] = bundle;
    _digests[entry.id] = entry.sha256;
    _disabled.remove(entry.id);
    await _save();
    notifyListeners();
  }

  Future<void> setEnabled(String id, bool enabled) async {
    if (!isInstalled(id)) throw StateError('plugin is not installed');
    if (enabled) {
      _disabled.remove(id);
    } else {
      _disabled.add(id);
    }
    await _save();
    notifyListeners();
  }

  Future<void> _save() async {
    final state = File(path.join(directory.path, 'state.json'));
    final temporary = File('${state.path}.tmp');
    await temporary.writeAsString(
      jsonEncode({
        'installed': {
          for (final e in _installed.entries)
            e.key: {'version': e.value.version, 'sha256': _digests[e.key]},
        },
        'disabled': _disabled.toList()..sort(),
      }),
      flush: true,
    );
    await temporary.rename(state.path);
  }

  static bool _safeId(String value) =>
      RegExp(r'^[a-z0-9]+(?:[.-][a-z0-9]+)+$').hasMatch(value);
}

/// Bundled launch catalog. A remote catalog must first pass signature checks;
/// it must never be passed directly to [MobilePluginStore.install].
Future<List<MobileMarketEntry>> loadBundledMobileMarket() async {
  final raw = await rootBundle.loadString(
    'packages/openmuse_mobile_plugin_runtime/assets/market.json',
  );
  final entries = jsonDecode(raw) as List;
  return entries
      .map(
        (e) => MobileMarketEntry.fromJson(Map<String, Object?>.from(e as Map)),
      )
      .toList(growable: false);
}

Future<List<int>> loadBundledMobileArtifact(MobileMarketEntry entry) async {
  if (!RegExp(r'^[a-z0-9-]+\.json$').hasMatch(entry.bundle)) {
    throw const FormatException('invalid embedded artifact path');
  }
  final data = await rootBundle.load(
    'packages/openmuse_mobile_plugin_runtime/assets/${entry.bundle}',
  );
  return data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes);
}
