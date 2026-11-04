import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as path;
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'mobile_widget_catalog.dart';

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
    this.runtime,
    this.permissions,
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

  /// Signed catalog hint for presenting an artifact before installation.
  final String? runtime;
  final List<String>? permissions;

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
        runtime: switch (json['runtime']) {
          null => null,
          'webview' => 'webview',
          'quickjs' => 'quickjs',
          _ => throw const FormatException('invalid Mobile market runtime'),
        },
        permissions: json.containsKey('permissions')
            ? (json['permissions'] as List).cast<String>()
            : null,
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
    this.html,
    this.card,
    this.preview = const [],
  });
  final String id;
  final String version;
  final String js;
  final OpenMusePluginManifestV2 manifest;
  final String? html;
  final OpenMuseWorkbenchCard? card;
  final List<Map<String, Object?>> preview;
  bool get isWebView => html != null;

  factory MobilePluginBundle.fromJson(Map<String, Object?> json) {
    if (json['format'] == 'openmuse-mobile-web/v1') {
      return _webFromJson(json);
    }
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

  static MobilePluginBundle _webFromJson(Map<String, Object?> json) {
    if (json.keys.toSet().difference(const {
          'format',
          'uiProtocol',
          'id',
          'version',
          'html',
          'card',
          'preview',
          'manifest',
        }).isNotEmpty ||
        json['uiProtocol'] != 'a2ui/v0.9' ||
        json['id'] is! String ||
        json['version'] is! String ||
        json['html'] is! String ||
        json['card'] is! Map ||
        json['preview'] is! List ||
        json['manifest'] is! Map) {
      throw const FormatException('unsupported Mobile WebView bundle');
    }
    final html = json['html'] as String;
    if (html.length > 512 * 1024 ||
        !RegExp(r'<head>', caseSensitive: false).hasMatch(html) ||
        !RegExp(r'</html\s*>', caseSensitive: false).hasMatch(html)) {
      throw const FormatException('invalid Mobile WebView document');
    }
    final manifest = OpenMusePluginManifestV2.fromJson(json['manifest']);
    final digest = sha256.convert(utf8.encode(html)).toString();
    if (manifest.id != json['id'] ||
        manifest.version != json['version'] ||
        manifest.uiRuntime.kind != OpenMuseUiRuntimeKind.webView ||
        manifest.uiRuntime.entrypoint != 'index.html' ||
        manifest.executionConnector.kind !=
            OpenMuseExecutionConnectorKind.none ||
        manifest.requestedPermissions.isNotEmpty ||
        manifest.artifacts.isEmpty ||
        manifest.artifacts.any(
          (artifact) =>
              artifact.kind != OpenMuseArtifactKind.webBundle ||
              artifact.digest.value != digest,
        )) {
      throw const FormatException('Mobile WebView manifest mismatch');
    }
    final rawCard = Map<String, Object?>.from(json['card'] as Map);
    if (rawCard.keys.toSet().difference(const {
          'title',
          'summary',
          'columns',
          'height',
        }).isNotEmpty ||
        rawCard['title'] is! String ||
        (rawCard['title'] as String).trim().isEmpty ||
        rawCard['summary'] is! String ||
        !const [1, 2].contains(rawCard['columns']) ||
        !const ['small', 'medium', 'large'].contains(rawCard['height'])) {
      throw const FormatException('invalid Mobile workbench card');
    }
    final messages = (json['preview'] as List)
        .map((value) => Map<String, Object?>.from(value as Map))
        .toList(growable: false);
    final surface = MobileWidgetCatalog.surface();
    try {
      MobileWidgetCatalog.apply(surface, messages);
    } finally {
      surface.dispose();
    }
    return MobilePluginBundle(
      id: json['id'] as String,
      version: json['version'] as String,
      js: '',
      html: html,
      card: OpenMuseWorkbenchCard(
        title: rawCard['title'] as String,
        summary: rawCard['summary'] as String,
        columns: rawCard['columns'] as int,
        height: OpenMuseWidgetHeight.values.byName(rawCard['height'] as String),
      ),
      preview: messages,
      manifest: manifest,
    );
  }
}

/// Installation is a Host operation. Artifact bytes are checked before use.
final class MobilePluginStore extends ChangeNotifier {
  MobilePluginStore(
    this.directory, {
    this.allowInstall,
    this.allowDownloadedPlugins = true,
  });
  final bool Function()? allowInstall;

  /// A release can supply all executable plugins in its application binary.
  /// Old downloaded artifacts remain on disk but are not restored or executed.
  final bool allowDownloadedPlugins;
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
    if (!allowDownloadedPlugins) return;
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
          !RegExp(r'^\d+\.\d+\.\d+$').hasMatch(version) ||
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
    if (!allowDownloadedPlugins) {
      throw StateError('This release uses bundled plugins');
    }
    if (allowInstall?.call() == false)
      throw StateError('Plugin installation is currently disabled');
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
    if (entry.runtime != null &&
        entry.runtime != (bundle.isWebView ? 'webview' : 'quickjs')) {
      throw const FormatException('plugin runtime differs from catalog');
    }
    if (entry.permissions != null &&
        entry.permissions!.toSet().length != entry.permissions!.length) {
      throw const FormatException('duplicate plugin permissions');
    }
    if (entry.permissions != null &&
        !setEquals(
          entry.permissions!.toSet(),
          bundle.manifest.requestedPermissions,
        )) {
      throw const FormatException('plugin permissions differ from catalog');
    }
    final localOs = Platform.isAndroid
        ? OpenMuseTargetOs.android
        : Platform.isIOS
        ? OpenMuseTargetOs.ios
        : null;
    if (localOs != null &&
        (!entry.platforms.contains(localOs.name) ||
            !bundle.manifest.compatibility.any(
              (decision) =>
                  decision.status == OpenMuseTargetStatus.supported &&
                  decision.target.os == localOs,
            ))) {
      throw const FormatException('plugin does not support this Mobile OS');
    }
    if (bundle.isWebView &&
        !bundle.manifest.compatibility.any(
          (decision) =>
              decision.status == OpenMuseTargetStatus.supported &&
              ((entry.platforms.contains('android') &&
                      decision.target.os == OpenMuseTargetOs.android) ||
                  (entry.platforms.contains('ios') &&
                      decision.target.os == OpenMuseTargetOs.ios)),
        )) {
      throw const FormatException('Mobile WebView target mismatch');
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

  Future<void> uninstall(String id) async {
    final previous = _installed.remove(id);
    if (previous == null) return;
    final previousDigest = _digests.remove(id);
    final wasDisabled = _disabled.remove(id);
    try {
      await _save();
    } on Object {
      _installed[id] = previous;
      if (previousDigest != null) _digests[id] = previousDigest;
      if (wasDisabled) _disabled.add(id);
      rethrow;
    }
    notifyListeners();
    final folder = Directory(path.join(directory.path, id));
    if (await folder.exists()) await folder.delete(recursive: true);
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
Future<List<MobileMarketEntry>> loadBundledMobileMarket({
  String catalogAsset =
      'packages/openmuse_mobile_plugin_runtime/assets/market.json',
}) async {
  final raw = await rootBundle.loadString(catalogAsset);
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
