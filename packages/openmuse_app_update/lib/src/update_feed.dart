import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:pub_semver/pub_semver.dart';

/// A public, immutable release record served from the website download root.
@immutable
final class OpenMuseRelease {
  const OpenMuseRelease({
    required this.channel,
    required this.target,
    required this.version,
    required this.build,
    required this.publishedAt,
    required this.downloadUri,
    required this.sha256,
    required this.sizeBytes,
    required this.notes,
    required this.installKind,
    this.mandatory = false,
  });

  final String channel;
  final String target;
  final Version version;
  final int build;
  final DateTime publishedAt;
  final Uri downloadUri;
  final String sha256;
  final int sizeBytes;
  final Map<String, String> notes;
  final String installKind;
  final bool mandatory;

  String notesFor(String languageTag) {
    final catalog = OpenMuseCatalog.builtIn;
    final option = catalog.option(languageTag);
    for (final tag in [
      languageTag,
      option.tag,
      ...option.aliases,
      catalog.templateLocale,
    ]) {
      if (notes[tag] case final value?) return value;
    }
    return notes.isEmpty ? '' : notes.values.first;
  }

  bool isNewerThan(Version installedVersion, int installedBuild) =>
      version > installedVersion ||
      (version == installedVersion && build > installedBuild);

  factory OpenMuseRelease.parse(
    Object? document, {
    required String channel,
    required String target,
    required Uri feedUri,
  }) {
    if (document is! Map<String, dynamic> ||
        document['schema'] != 'openmuse.app-update/v1' ||
        document['channel'] != channel ||
        document['target'] != target) {
      throw const FormatException('更新清单身份或格式不匹配');
    }
    final version = Version.parse(_text(document['version'], 'version'));
    final build = document['build'];
    final size = document['sizeBytes'];
    final hash = (document['sha256'] as String? ?? '').toLowerCase();
    final notesValue = document['notes'];
    final uri = Uri.parse(_text(document['downloadUrl'], 'downloadUrl'));
    final installKind = _text(document['installKind'], 'installKind');
    final expectedKind = switch (target) {
      'macos-universal' => 'macos-zip',
      'windows-x64' => 'windows-installer',
      'android-arm64' => 'android-store',
      'ios-arm64' => 'ios-store',
      _ => throw const FormatException('不支持的更新目标'),
    };
    final isStore = expectedKind.endsWith('-store');
    // The feed path ends with downloads/updates/v1/channel/target.json.
    // Preserve any deployment gateway prefix when admitting download URLs.
    final prefixLength = feedUri.pathSegments.length - 5;
    final downloadPrefix = [
      ...feedUri.pathSegments.take(prefixLength < 0 ? 0 : prefixLength),
      'downloads',
    ];
    final matchesDownloadPrefix =
        uri.pathSegments.length > downloadPrefix.length &&
        Iterable<int>.generate(
          downloadPrefix.length,
        ).every((index) => uri.pathSegments[index] == downloadPrefix[index]);
    final validDownload = isStore
        ? uri.scheme == 'https' &&
              (expectedKind == 'ios-store'
                  ? {
                      'apps.apple.com',
                      'testflight.apple.com',
                    }.contains(uri.host)
                  : uri.host == 'play.google.com')
        : (uri.scheme == 'https' ||
                  (feedUri.scheme == 'http' && uri.scheme == 'http')) &&
              uri.host == feedUri.host &&
              uri.port == feedUri.port &&
              matchesDownloadPrefix;
    if (build is! int ||
        build < 1 ||
        size is! int ||
        size < (isStore ? 0 : 1) ||
        (!isStore && !RegExp(r'^[0-9a-f]{64}$').hasMatch(hash)) ||
        installKind != expectedKind ||
        notesValue is! Map ||
        notesValue.isEmpty ||
        !validDownload) {
      throw const FormatException('更新清单包含无效的版本制品或下载地址');
    }
    final notes = <String, String>{};
    for (final entry in notesValue.entries) {
      if (entry.key is! String ||
          entry.value is! String ||
          (entry.value as String).trim().isEmpty ||
          (entry.value as String).length > 4000) {
        throw const FormatException('更新说明无效');
      }
      notes[entry.key as String] = entry.value as String;
    }
    return OpenMuseRelease(
      channel: channel,
      target: target,
      version: version,
      build: build,
      publishedAt: DateTime.parse(
        _text(document['publishedAt'], 'publishedAt'),
      ),
      downloadUri: uri,
      sha256: hash,
      sizeBytes: size,
      notes: notes,
      installKind: installKind,
      mandatory: document['mandatory'] == true,
    );
  }

  static String _text(Object? value, String field) {
    if (value is! String || value.trim().isEmpty) {
      throw FormatException('缺少更新字段：$field');
    }
    return value;
  }
}

/// Static, same-origin feed; no account token or user content is transmitted.
final class OpenMuseUpdateFeed {
  const OpenMuseUpdateFeed({
    required this.origin,
    required this.channel,
    required this.target,
    this.allowInsecureLoopback = false,
    this.fetchOverride,
  });

  final Uri origin;
  final String channel;
  final String target;
  final bool allowInsecureLoopback;
  final Future<String?> Function(Uri uri)? fetchOverride;

  Uri get feedUri {
    if (!RegExp(r'^[a-z0-9-]{1,32}$').hasMatch(channel) ||
        !RegExp(r'^[a-z0-9-]{1,40}$').hasMatch(target)) {
      throw const FormatException('无效更新通道或平台');
    }
    if (origin.scheme != 'https' &&
        !(allowInsecureLoopback &&
            origin.scheme == 'http' &&
            (origin.host == '127.0.0.1' || origin.host == 'localhost'))) {
      throw const FormatException('更新服务必须使用 HTTPS');
    }
    return origin.replace(
      pathSegments: [
        ...origin.pathSegments.where((segment) => segment.isNotEmpty),
        'downloads',
        'updates',
        'v1',
        channel,
        '$target.json',
      ],
      query: '',
      fragment: '',
    );
  }

  Future<OpenMuseRelease?> fetch() async {
    final uri = feedUri;
    final override = fetchOverride;
    final body = override == null ? await _fetchHttp(uri) : await override(uri);
    if (body == null) return null;
    if (body.length > 65536) throw const FormatException('更新清单过大');
    return OpenMuseRelease.parse(
      jsonDecode(body),
      channel: channel,
      target: target,
      feedUri: uri,
    );
  }

  Future<String?> _fetchHttp(Uri uri) async {
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 4);
    try {
      final request = await client
          .getUrl(uri)
          .timeout(const Duration(seconds: 4));
      request.followRedirects = false;
      request.headers.set(HttpHeaders.cacheControlHeader, 'no-cache');
      final response = await request.close().timeout(
        const Duration(seconds: 4),
      );
      if (response.statusCode == HttpStatus.notFound) {
        await response.drain<void>();
        return null;
      }
      if (response.statusCode != HttpStatus.ok) {
        await response.drain<void>();
        throw HttpException('更新服务返回 ${response.statusCode}', uri: uri);
      }
      final bytes = <int>[];
      await for (final chunk in response.timeout(const Duration(seconds: 4))) {
        bytes.addAll(chunk);
        if (bytes.length > 65536) throw const FormatException('更新清单过大');
      }
      return utf8.decode(bytes);
    } finally {
      client.close(force: true);
    }
  }
}

final class OpenMuseUpdateController extends ChangeNotifier {
  OpenMuseUpdateController({
    required this.feed,
    required this.installedVersion,
    required this.installedBuild,
  });

  final OpenMuseUpdateFeed feed;
  final Version installedVersion;
  final int installedBuild;
  OpenMuseRelease? _available;
  OpenMuseRelease? get available => _available;
  Object? lastError;
  Timer? _poller;
  Future<void>? _inFlight;

  Future<void> check() =>
      _inFlight ??= _check().whenComplete(() => _inFlight = null);

  Future<void> _check() async {
    try {
      final release = await feed.fetch();
      final next =
          release != null &&
              release.isNewerThan(installedVersion, installedBuild)
          ? release
          : null;
      lastError = null;
      if (_available?.version != next?.version ||
          _available?.build != next?.build ||
          _available?.sha256 != next?.sha256) {
        _available = next;
        notifyListeners();
      }
    } catch (error) {
      // A failed update check must not block offline startup or erase a
      // previously verified update notice.
      lastError = error;
    }
  }

  void startPolling([Duration interval = const Duration(minutes: 30)]) {
    _poller?.cancel();
    _poller = Timer.periodic(interval, (_) => unawaited(check()));
  }

  @override
  void dispose() {
    _poller?.cancel();
    super.dispose();
  }
}
