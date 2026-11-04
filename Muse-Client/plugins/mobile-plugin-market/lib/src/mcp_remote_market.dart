import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:path_provider/path_provider.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';

/// Consumes only MCP Registry server.json and MCP Streamable HTTP.
final class RemoteMcpEntry {
  const RemoteMcpEntry(
    this.name,
    this.title,
    this.version,
    this.description,
    this.endpoint,
    this.packageUri,
    this.packageSha256,
    this.registryRecord,
  );
  final String name;
  final String title;
  final String version;
  final String description;
  final Uri? endpoint;
  final Uri? packageUri;
  final String? packageSha256;
  final Map<String, dynamic>? registryRecord;

  factory RemoteMcpEntry.fromRecord(Map record) {
    final server = record['server'];
    if (server is! Map ||
        server['name'] is! String ||
        server['version'] is! String) {
      throw const FormatException('Invalid MCP Registry record');
    }
    final remote = ((server['remotes'] as List?) ?? const [])
        .whereType<Map>()
        .where((item) => item['type'] == 'streamable-http')
        .firstOrNull;
    final endpoint = remote?['url'] is String
        ? Uri.tryParse(remote!['url'] as String)
        : null;
    if (endpoint != null &&
        endpoint.scheme != 'https' &&
        !(endpoint.scheme == 'http' &&
            {'localhost', '127.0.0.1'}.contains(endpoint.host))) {
      throw const FormatException('Remote MCP requires HTTPS');
    }
    final package = server['packages'] is List
        ? (server['packages'] as List)
              .whereType<Map>()
              .where((item) => item['registryType'] == 'mcpb')
              .firstOrNull
        : null;
    final packageUri = package?['identifier'] is String
        ? Uri.tryParse(package!['identifier'] as String)
        : null;
    final packageSha256 = package?['fileSha256'] is String
        ? package!['fileSha256'] as String
        : null;
    if (endpoint == null && (packageUri == null || packageSha256 == null)) {
      throw const FormatException('No installable MCP endpoint or package');
    }
    return RemoteMcpEntry(
      server['name'] as String,
      server['title'] is String
          ? server['title'] as String
          : server['name'] as String,
      server['version'] as String,
      server['description'] is String ? server['description'] as String : '',
      endpoint,
      packageUri,
      packageSha256,
      Map<String, dynamic>.from(record),
    );
  }
}

final class RemoteMcpMarket {
  RemoteMcpMarket(
    this.origin,
    this.receiptFile, {
    this.nativeMediaTypes = const {},
    Map<String, String>? allowedPackages,
  }) : allowedPackages = allowedPackages == null
           ? null
           : Map.unmodifiable(allowedPackages);
  final Uri origin;
  final File receiptFile;

  /// Immutable name@version -> digest pins supplied by the installed app.
  /// A pinned release accepts local HTML from these packages only.
  final Map<String, String>? allowedPackages;
  bool _permits(RemoteMcpEntry entry) =>
      allowedPackages == null ||
      (entry.endpoint == null &&
          entry.packageSha256 != null &&
          allowedPackages!['${entry.name}@${entry.version}'] ==
              entry.packageSha256!.toLowerCase());
  void _requirePermitted(RemoteMcpEntry entry) {
    if (!_permits(entry)) {
      throw const FormatException(
        'This plugin is not included in this release',
      );
    }
  }

  /// AOT renderers supplied by this client build, keyed by public Registry name.
  final Map<String, Set<String>> nativeMediaTypes;
  bool catalogFresh = false;
  Set<String> activeVersions = {};
  File get _catalogCache =>
      File('${receiptFile.parent.path}/mcp-registry-cache.json');
  File get _installedRecords =>
      File('${receiptFile.parent.path}/mcp-installed-records.json');
  File get _capabilityCache =>
      File('${receiptFile.parent.path}/mcp-resource-capabilities.json');

  Future<Map<String, List<String>>> _capabilities() async {
    if (!await _capabilityCache.exists()) return {};
    final value = jsonDecode(await _capabilityCache.readAsString());
    if (value is! Map) throw const FormatException('Invalid MCP capabilities');
    return value.map(
      (key, types) => MapEntry(
        key.toString(),
        (types as List).whereType<String>().toList(),
      ),
    );
  }

  /// A local snapshot of standard JSON Schema contentMediaType annotations.
  Future<Set<String>> resourceMediaTypes(RemoteMcpEntry entry) async =>
      (await _capabilities())['${entry.name}@${entry.version}']?.toSet() ??
      nativeMediaTypes[entry.name] ??
      {};

  Future<void> refreshInstalledCapabilities() async {
    final cached = await _capabilities();
    for (final entry in await installedEntries()) {
      if (cached.containsKey('${entry.name}@${entry.version}')) continue;
      try {
        await _cacheResourceCapabilities(entry);
      } on Object {
        // Installed offline apps remain available when their MCP endpoint is down.
      }
    }
  }

  Future<void> _cacheResourceCapabilities(RemoteMcpEntry entry) async {
    if (entry.endpoint == null) return;
    final mediaTypes = await withClient(entry, (client) async {
      final tools = await client.listTools();
      final types = <String>{};
      for (final tool in tools.tools) {
        final ui = tool.meta?['ui'];
        if (ui is Map &&
            ui['resourceUri'] is String &&
            (ui['resourceUri'] as String).startsWith('ui://') &&
            entry.packageSha256 != null) {
          final resource = await client.readResource(
            ReadResourceRequest(uri: ui['resourceUri'] as String),
          );
          for (final item
              in resource.contents.whereType<TextResourceContents>()) {
            if (item.mimeType == 'text/html;profile=mcp-app') {
              final directory = packageDirectory(entry);
              final reviewed = File('${directory.path}/index.html');
              if (!await reviewed.exists() ||
                  await reviewed.readAsString() != item.text) {
                throw const FormatException(
                  'MCP App UI differs from the reviewed package',
                );
              }
              await File(
                '${directory.path}/mcp-app.html',
              ).writeAsString(restrictMcpAppDocument(item.text), flush: true);
            }
          }
        }
        final properties = tool.inputSchema.toJson()['properties'];
        if (properties is! Map) continue;
        for (final input in properties.values.whereType<Map>()) {
          final type = input['contentMediaType'];
          if (input['contentEncoding'] == 'base64' && type is String) {
            types.add(type);
          }
        }
      }
      return types.toList()..sort();
    });
    final cached = await _capabilities();
    cached['${entry.name}@${entry.version}'] = mediaTypes;
    await _capabilityCache.writeAsString(jsonEncode(cached), flush: true);
  }

  Future<List<RemoteMcpEntry>> _savedInstallations({
    bool includeUnavailable = false,
  }) async {
    if (!await _installedRecords.exists()) return [];
    final records = jsonDecode(await _installedRecords.readAsString());
    if (records is! List)
      throw const FormatException('Invalid installed MCP records');
    return records
        .whereType<Map>()
        .map(RemoteMcpEntry.fromRecord)
        .where((entry) => includeUnavailable || _permits(entry))
        .toList();
  }

  Directory packageDirectory(RemoteMcpEntry entry) {
    _requirePermitted(entry);
    final digest = entry.packageSha256;
    if (digest == null || !RegExp(r'^[a-fA-F0-9]{64}$').hasMatch(digest)) {
      throw const FormatException('No verified MCPB digest');
    }
    return Directory(
      '${receiptFile.parent.path}/mcp-web/${digest.toLowerCase()}',
    );
  }

  Future<String?> mcpAppDocument(RemoteMcpEntry entry) async {
    if (entry.packageSha256 == null) return null;
    final file = File('${packageDirectory(entry).path}/mcp-app.html');
    return await file.exists() ? file.readAsString() : null;
  }

  bool hasOfflineUi(RemoteMcpEntry entry) =>
      entry.packageSha256 != null &&
      File('${packageDirectory(entry).path}/index.html').existsSync();

  /// Public offline MCP discovery snapshot, contained in the reviewed MCPB.
  /// These are ordinary MCP ResourceContents and Tool records, not executable
  /// Node code or private Registry metadata.
  Future<List<Map<String, Object?>>> resourceExtensions(
    String mediaType,
  ) async {
    final result = <Map<String, Object?>>[];
    for (final entry in await installedEntries()) {
      final snapshot = File('${packageDirectory(entry).path}/mcp-offline.json');
      if (!await snapshot.exists() || await snapshot.length() > 8 * 1024 * 1024)
        continue;
      try {
        final data = jsonDecode(await snapshot.readAsString()) as Map;
        final contents = (data['resources'] as List).whereType<Map>();
        final tools = (data['tools'] as List).whereType<Map>();
        for (final catalog in contents.where(
          (r) =>
              r['mimeType'] ==
              'application/vnd.openmuse.resource-extensions+json',
        )) {
          final value = jsonDecode(catalog['text'] as String) as Map;
          if (value['profile'] != 'openmuse-resource-extensions/1') continue;
          final items = value['extensions'] as List;
          if (items.length > 4) throw const FormatException('Extension budget');
          for (final item in items.whereType<Map>()) {
            if (!(item['mediaTypes'] as List).contains(mediaType)) continue;
            final id = item['id'],
                width = item['width'],
                height = item['height'];
            if (id is! String ||
                !RegExp(r'^[a-zA-Z0-9._-]{1,80}$').hasMatch(id) ||
                width is! int ||
                width < 200 ||
                width > 480 ||
                height is! int ||
                height < 48 ||
                height > 180)
              throw const FormatException('Invalid extension');
            final tool = tools.singleWhere((t) => t['name'] == item['tool']);
            final uri = (tool['_meta'] as Map)['ui'] as Map;
            final ui = contents.singleWhere(
              (r) =>
                  r['uri'] == uri['resourceUri'] &&
                  r['mimeType'] == 'text/html;profile=mcp-app',
            );
            final html = ui['text'] as String;
            if (utf8.encode(html).length > 4 * 1024 * 1024)
              throw const FormatException('UI budget');
            result.add({
              'id': '${entry.name}:$id',
              'title': entry.title,
              'html': html,
              'digest': entry.packageSha256,
              'version': entry.version,
              'name': entry.name,
              'height': height,
              'width': width,
              'resourceTools': (item['resourceTools'] as List? ?? [])
                  .cast<String>(),
            });
          }
        }
      } on Object {
        /* Optional incompatible contributions must not break a file. */
      }
    }
    return result.take(4).toList();
  }

  void _checkRegistry() {
    if (origin.scheme != 'https' &&
        !(origin.scheme == 'http' &&
            {'localhost', '127.0.0.1'}.contains(origin.host))) {
      throw const FormatException('MCP Registry requires HTTPS');
    }
  }

  Future<List<RemoteMcpEntry>> list() async {
    _checkRegistry();
    final http = HttpClient()..connectionTimeout = const Duration(seconds: 10);
    try {
      final entries = <RemoteMcpEntry>[];
      final records = <Map>[];
      String? cursor;
      for (var page = 0; page < 20; page++) {
        final url = origin
            .resolve('/v0.1/servers')
            .replace(
              queryParameters: cursor == null ? null : {'cursor': cursor},
            );
        final response = await (await http.getUrl(
          url,
        )).close().timeout(const Duration(seconds: 15));
        if (response.statusCode != 200)
          throw HttpException('Registry HTTP ${response.statusCode}');
        final json = jsonDecode(await utf8.decoder.bind(response).join());
        if (json is! Map || json['servers'] is! List)
          throw const FormatException('Invalid MCP Registry response');
        for (final record in json['servers'] as List) {
          try {
            final document = record as Map;
            final entry = RemoteMcpEntry.fromRecord(document);
            if (!_permits(entry)) continue;
            entries.add(entry);
            records.add(document);
          } on FormatException {
            // Desktop-only MCPB entries are not installable on Mobile.
          }
        }
        cursor = json['metadata'] is Map
            ? (json['metadata'] as Map)['nextCursor'] as String?
            : null;
        if (cursor == null) break;
      }
      await _catalogCache.parent.create(recursive: true);
      await _catalogCache.writeAsString(jsonEncode(records), flush: true);
      final known = entries
          .map((entry) => '${entry.name}@${entry.version}')
          .toSet();
      catalogFresh = true;
      activeVersions = {...known};
      for (final installed in await _savedInstallations()) {
        if (known.add('${installed.name}@${installed.version}')) {
          entries.add(installed);
        }
      }
      return entries;
    } catch (_) {
      final receipts = await installed();
      final saved = await _savedInstallations();
      if (saved.isNotEmpty) {
        return saved
            .where(
              (entry) => receipts.contains('${entry.name}@${entry.version}'),
            )
            .toList();
      }
      if (!await _catalogCache.exists()) rethrow;
      final cache = jsonDecode(await _catalogCache.readAsString());
      if (cache is! List) rethrow;
      return cache
          .whereType<Map>()
          .map(RemoteMcpEntry.fromRecord)
          .where(_permits)
          .where((entry) => receipts.contains('${entry.name}@${entry.version}'))
          .toList();
    } finally {
      http.close(force: true);
    }
  }

  Future<Set<String>> _allReceipts() async {
    if (!await receiptFile.exists()) return {};
    final decoded = jsonDecode(await receiptFile.readAsString());
    return (decoded as List).whereType<String>().toSet();
  }

  Future<Set<String>> installed() async => (await _allReceipts())
      .where(
        (key) => allowedPackages == null || allowedPackages!.containsKey(key),
      )
      .toSet();

  /// Resource dispatch uses the locally installed catalog. A registry outage
  /// must not turn a file click into a failed marketplace lookup.
  Future<List<RemoteMcpEntry>> installedEntries() async {
    final receipts = await installed();
    if (receipts.isEmpty) return [];
    return (await _savedInstallations())
        .where((entry) => receipts.contains('${entry.name}@${entry.version}'))
        .toList();
  }

  Future<T> withClient<T>(
    RemoteMcpEntry entry,
    Future<T> Function(McpClient) operation,
  ) async {
    _requirePermitted(entry);
    if (entry.endpoint == null) {
      throw const FormatException('This plugin has no remote MCP endpoint');
    }
    final client = McpClient(
      const Implementation(name: 'OpenMuse Mobile', version: '1.0.0'),
      options: const McpClientOptions(protocol: McpProtocol.require2026),
    );
    final transport = StreamableHttpClientTransport(entry.endpoint!);
    try {
      await client.connect(transport).timeout(const Duration(seconds: 15));
      return await operation(client).timeout(const Duration(seconds: 60));
    } finally {
      await client.close();
    }
  }

  Future<void> install(
    RemoteMcpEntry entry, {
    List<int>? bundledArtifact,
  }) async {
    _requirePermitted(entry);
    final offline = await _installOfflineUi(
      entry,
      bundledArtifact: bundledArtifact,
    );
    if (entry.endpoint != null) {
      try {
        await _cacheResourceCapabilities(entry);
      } on FormatException {
        rethrow;
      } on Object {
        if (!offline) rethrow;
      }
    }
    if (!offline)
      await withClient(entry, (client) async {
        final tools = await client.listTools();
        if (tools.tools.isEmpty)
          throw const FormatException('MCP server advertises no tools');
        if (client.getServerCapabilities()?.resources != null) {
          final resources = await client.listResources();
          for (final resource in resources.resources.where(
            (item) => item.mimeType == 'application/a2ui+json',
          )) {
            final content = await client.readResource(
              ReadResourceRequest(uri: resource.uri),
            );
            for (final item
                in content.contents.whereType<TextResourceContents>()) {
              if (jsonDecode(item.text) is! List)
                throw const FormatException('Invalid A2UI message array');
            }
          }
        }
      });
    final receipts = await _allReceipts();
    receipts.removeWhere((version) => version.startsWith('${entry.name}@'));
    receipts.add('${entry.name}@${entry.version}');
    await receiptFile.parent.create(recursive: true);
    await receiptFile.writeAsString(
      jsonEncode(receipts.toList()..sort()),
      flush: true,
    );
    if (entry.registryRecord != null) {
      final records = await _savedInstallations(includeUnavailable: true);
      final next = records.where((old) => old.name != entry.name).toList()
        ..add(entry);
      await _installedRecords.writeAsString(
        jsonEncode(next.map((item) => item.registryRecord).toList()),
        flush: true,
      );
    }
  }

  Future<void> _cacheOfflineDiscovery(
    RemoteMcpEntry entry,
    Directory directory,
  ) async {
    final snapshot = File('${directory.path}/mcp-offline.json');
    if (!await snapshot.exists()) return;
    if (await snapshot.length() > 64 * 1024 * 1024)
      throw const FormatException('MCP discovery budget');
    final data = jsonDecode(await snapshot.readAsString()) as Map;
    final tools = (data['tools'] as List).whereType<Map>();
    final contents = (data['resources'] as List).whereType<Map>();
    final types = <String>{};
    String? app;
    for (final tool in tools) {
      final properties = (tool['inputSchema'] as Map?)?['properties'];
      final inputTypes = properties is Map
          ? properties.values
                .whereType<Map>()
                .where((p) => p['contentEncoding'] == 'base64')
                .map((p) => p['contentMediaType'])
                .whereType<String>()
                .toSet()
          : <String>{};
      final ui = (tool['_meta'] as Map?)?['ui'];
      if (inputTypes.isEmpty ||
          ui is! Map ||
          ui['resourceUri'] is! String ||
          !(ui['resourceUri'] as String).startsWith('ui://'))
        continue;
      final resource = contents.singleWhere(
        (r) =>
            r['uri'] == ui['resourceUri'] &&
            r['mimeType'] == 'text/html;profile=mcp-app',
      );
      final text = resource['text'] as String;
      // Discovery cannot replace the reviewed primary document with arbitrary HTML.
      if (text != await File('${directory.path}/index.html').readAsString())
        throw const FormatException('Offline MCP App differs from reviewed UI');
      if (app != null && app != text)
        throw const FormatException('Ambiguous offline UI');
      app = text;
      types.addAll(inputTypes);
    }
    if (app != null)
      await File(
        '${directory.path}/mcp-app.html',
      ).writeAsString(restrictMcpAppDocument(app), flush: true);
    final cached = await _capabilities();
    cached['${entry.name}@${entry.version}'] = types.toList()..sort();
    await _capabilityCache.writeAsString(jsonEncode(cached), flush: true);
  }

  Future<bool> _installOfflineUi(
    RemoteMcpEntry entry, {
    List<int>? bundledArtifact,
  }) async {
    final packageUri = entry.packageUri;
    final expected = entry.packageSha256?.toLowerCase();
    if (packageUri == null || expected == null) return false;
    if (!RegExp(r'^[a-f0-9]{64}$').hasMatch(expected)) {
      throw const FormatException('Invalid MCPB SHA-256');
    }
    if (packageUri.scheme != 'https' &&
        !(packageUri.scheme == 'http' &&
            {'localhost', '127.0.0.1'}.contains(packageUri.host))) {
      throw const FormatException('MCPB download requires HTTPS');
    }
    final http = HttpClient()..connectionTimeout = const Duration(seconds: 15);
    try {
      final bytes = <int>[];
      if (bundledArtifact != null) {
        if (bundledArtifact.length > 80 * 1024 * 1024)
          throw const FormatException('MCPB exceeds 80 MiB');
        bytes.addAll(bundledArtifact);
      } else {
        final response = await (await http.getUrl(packageUri)).close();
        if (response.statusCode != 200)
          throw HttpException('MCPB download HTTP ${response.statusCode}');
        await for (final chunk in response) {
          bytes.addAll(chunk);
          if (bytes.length > 80 * 1024 * 1024)
            throw const FormatException('MCPB exceeds 80 MiB');
        }
      }
      if (sha256.convert(bytes).toString() != expected) {
        throw const FormatException('MCPB SHA-256 mismatch');
      }
      final archive = ZipDecoder().decodeBytes(bytes, verify: true);
      final files = archive.files
          .where((file) => file.isFile && file.name.startsWith('web/'))
          .toList();
      if (!files.any((file) => file.name == 'web/index.html')) {
        return nativeMediaTypes.containsKey(entry.name);
      }
      final target = packageDirectory(entry);
      final staging = Directory('${target.path}.installing');
      if (await staging.exists()) await staging.delete(recursive: true);
      await staging.create(recursive: true);
      try {
        var total = 0;
        for (final file in files) {
          final relative = file.name.substring(4);
          final segments = relative.split('/');
          if (segments.any(
                (part) =>
                    part.isEmpty ||
                    part == '.' ||
                    part == '..' ||
                    part.contains('\\'),
              ) ||
              relative.startsWith('/')) {
            throw const FormatException('Unsafe MCPB asset path');
          }
          total += file.size;
          if (total > 160 * 1024 * 1024 || files.length > 1024) {
            throw const FormatException('MCPB web assets exceed budget');
          }
          final output = File('${staging.path}/$relative');
          await output.parent.create(recursive: true);
          final content = file.readBytes();
          if (content == null)
            throw const FormatException('Invalid MCPB asset');
          await output.writeAsBytes(content, flush: true);
        }
        await _cacheOfflineDiscovery(entry, staging);
        if (await target.exists()) await target.delete(recursive: true);
        await staging.rename(target.path);
      } catch (_) {
        if (await staging.exists()) await staging.delete(recursive: true);
        rethrow;
      }
      return true;
    } finally {
      http.close(force: true);
    }
  }

  Future<Map<String, dynamic>> inspectFile(RemoteMcpEntry entry, File file) =>
      withClient(entry, (client) async {
        if (await file.length() > 16 * 1024 * 1024) {
          throw const FormatException('File exceeds 16 MiB remote tool limit');
        }
        final tools = await client.listTools();
        for (final tool in tools.tools) {
          final properties = tool.inputSchema.toJson()['properties'];
          if (properties is! Map ||
              properties['content'] is! Map ||
              (properties['content'] as Map)['contentEncoding'] != 'base64')
            continue;
          final result = await client.callTool(
            CallToolRequest(
              name: tool.name,
              arguments: {'content': base64Encode(await file.readAsBytes())},
            ),
          );
          if (result.isError || result.structuredContent == null)
            throw StateError('MCP tool failed');
          return Map<String, dynamic>.from(result.structuredContent!);
        }
        throw const FormatException('No MCP tool accepts a base64 file');
      });

  /// Discovers a file editor from standard JSON Schema contentMediaType.
  Future<bool> acceptsContent(RemoteMcpEntry entry, String mediaType) =>
      withClient(entry, (client) async {
        final tools = await client.listTools();
        return tools.tools.any((tool) {
          final properties = tool.inputSchema.toJson()['properties'];
          final content = properties is Map ? properties['content'] : null;
          return content is Map &&
              content['contentEncoding'] == 'base64' &&
              content['contentMediaType'] == mediaType;
        });
      });

  /// Sends content only after the Host has obtained user consent. The result
  /// is the standard MCP resource_link URL, treated as opaque by the Host.
  Future<Uri> openContent(
    RemoteMcpEntry entry, {
    required String mediaType,
    required String content,
    required String name,
  }) => withClient(entry, (client) async {
    final bytes = utf8.encode(content);
    if (bytes.length > 32 * 1024) {
      throw const FormatException('文件超过编辑器的 32 KiB 限额');
    }
    final tools = await client.listTools();
    for (final tool in tools.tools) {
      final schema = tool.inputSchema.toJson();
      final properties = schema['properties'];
      final input = properties is Map ? properties['content'] : null;
      if (input is! Map ||
          input['contentEncoding'] != 'base64' ||
          input['contentMediaType'] != mediaType)
        continue;
      final arguments = <String, Object?>{'content': base64Encode(bytes)};
      if (properties['name'] is Map) arguments['name'] = name;
      final result = await client.callTool(
        CallToolRequest(name: tool.name, arguments: arguments),
      );
      if (result.isError) throw StateError('MCP 编辑器打开失败');
      for (final link in result.content.whereType<ResourceLink>()) {
        final uri = Uri.tryParse(link.uri);
        if (link.mimeType == 'text/html' &&
            uri != null &&
            (uri.scheme == 'https' ||
                (uri.scheme == 'http' &&
                    {'localhost', '127.0.0.1'}.contains(uri.host)))) {
          return uri;
        }
      }
      throw const FormatException('MCP 编辑器未返回 HTML resource_link');
    }
    throw const FormatException('没有支持该文件类型的已安装 MCP 插件');
  });

  /// Opens any zero-argument MCP Apps tool that returns a standard HTML
  /// resource_link. A null result means this is a file-oriented MCP server.
  Future<Uri?> openUi(RemoteMcpEntry entry) => withClient(entry, (
    client,
  ) async {
    final tools = await client.listTools();
    for (final tool in tools.tools) {
      final ui = tool.meta?['ui'];
      if (ui is! Map || ui['resourceUri'] is! String) continue;
      final schema = tool.inputSchema.toJson();
      if (schema['required'] is List && (schema['required'] as List).isNotEmpty)
        continue;
      final result = await client.callTool(
        CallToolRequest(name: tool.name, arguments: const {}),
      );
      if (result.isError) throw StateError('MCP App tool failed');
      for (final content in result.content.whereType<ResourceLink>()) {
        if (content.mimeType != 'text/html') continue;
        final uri = Uri.tryParse(content.uri);
        if (uri != null &&
            (uri.scheme == 'https' ||
                (uri.scheme == 'http' &&
                    {'localhost', '127.0.0.1'}.contains(uri.host)))) {
          return uri;
        }
      }
    }
    return null;
  });
}

final class RemoteMcpMarketPage extends StatefulWidget {
  const RemoteMcpMarketPage({
    super.key,
    this.origin,
    this.providers,
    this.receiptFile,
    this.pickFile,
    this.nativeMediaTypes = const {},
    this.allowedPackages,
    this.openNativeFile,
    this.embedded = false,
    this.query = '',
    this.installedOnly = false,
    this.visibleCategory = '精选',
  });
  final bool embedded;
  final String query;
  final bool installedOnly;
  final String visibleCategory;
  final Uri? origin;
  final McpRegistryProviders? providers;
  final File? receiptFile;
  final Future<File?> Function()? pickFile;
  final Map<String, Set<String>> nativeMediaTypes;
  final Map<String, String>? allowedPackages;
  final Future<void> Function(BuildContext, RemoteMcpEntry, File)?
  openNativeFile;
  @override
  State<RemoteMcpMarketPage> createState() => _RemoteMcpMarketPageState();
}

final class _RemoteMcpMarketPageState extends State<RemoteMcpMarketPage> {
  RemoteMcpMarket? market;
  List<RemoteMcpEntry>? entries;
  Set<String> installed = {};
  String? error;
  String? busy;
  String? output;
  Timer? _refreshTimer;
  Uri? get _activeOrigin => widget.providers?.selected ?? widget.origin;

  @override
  void initState() {
    super.initState();
    _load();
    _refreshTimer = Timer.periodic(const Duration(seconds: 20), (_) => _load());
  }

  @override
  void dispose() {
    _refreshTimer?.cancel();
    super.dispose();
  }

  Future<void> _load() async {
    try {
      final origin = _activeOrigin;
      if (origin == null) {
        if (mounted)
          setState(() {
            entries = [];
            market = null;
            error = null;
          });
        return;
      }
      final directory = widget.receiptFile == null
          ? await getApplicationSupportDirectory()
          : null;
      final source = RemoteMcpMarket(
        origin,
        widget.receiptFile ??
            File('${directory!.path}/mcp-remote-installations.json'),
        nativeMediaTypes: widget.nativeMediaTypes,
        allowedPackages: widget.allowedPackages,
      );
      final catalog = await source.list();
      final receipts = await source.installed();
      if (mounted)
        setState(() {
          market = source;
          entries = catalog;
          installed = receipts;
          error = null;
        });
    } catch (cause) {
      if (mounted) setState(() => error = '$cause');
    }
  }

  Future<void> _configureProvider() async {
    final providers = widget.providers;
    if (providers == null) return;
    final input = TextEditingController(
      text: providers.selected?.toString() ?? '',
    );
    final choice = await showDialog<String>(
      context: context,
      builder: (dialog) => AlertDialog(
        title: Text(stringsOf(context).marketProviders()),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final uri in providers.origins)
              ListTile(
                title: Text(uri.toString()),
                trailing: uri == providers.selected
                    ? const Icon(Icons.check)
                    : null,
                onTap: () => Navigator.pop(dialog, uri.toString()),
              ),
            TextField(
              controller: input,
              decoration: InputDecoration(
                labelText: stringsOf(context).marketRegistryAddress(),
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialog),
            child: Text(stringsOf(context).marketCancel()),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(dialog, input.text),
            child: Text(stringsOf(context).marketUseAddress()),
          ),
        ],
      ),
    );
    input.dispose();
    if (choice == null || choice.trim().isEmpty) return;
    try {
      await providers.select(choice);
      if (mounted)
        setState(() {
          entries = null;
          error = null;
        });
      await _load();
    } catch (cause) {
      if (mounted) setState(() => error = '$cause');
    }
  }

  Future<void> _install(RemoteMcpEntry entry) async {
    setState(() {
      busy = entry.name;
      error = null;
    });
    try {
      await market!.install(entry);
      if (mounted)
        setState(() => installed.add('${entry.name}@${entry.version}'));
    } catch (cause) {
      if (mounted) setState(() => error = '$cause');
    } finally {
      if (mounted) setState(() => busy = null);
    }
  }

  Future<void> _use(RemoteMcpEntry entry) async {
    setState(() {
      busy = entry.name;
      error = null;
    });
    try {
      if (widget.nativeMediaTypes.containsKey(entry.name) &&
          widget.openNativeFile != null) {
        final selected = widget.pickFile == null
            ? await FilePicker.platform
                  .pickFiles(
                    type: FileType.custom,
                    allowedExtensions: const ['pptx'],
                  )
                  .then(
                    (value) => value?.files.single.path == null
                        ? null
                        : File(value!.files.single.path!),
                  )
            : await widget.pickFile!();
        if (selected != null && mounted) {
          await widget.openNativeFile!(context, entry, selected);
        }
        return;
      }
      if ((Platform.isAndroid || Platform.isIOS) &&
          market!.hasOfflineUi(entry)) {
        final discovery = File(
          '${market!.packageDirectory(entry).path}/mcp-offline.json',
        );
        if (await discovery.exists()) {
          final snapshot = jsonDecode(await discovery.readAsString()) as Map;
          if ((snapshot['resources'] as List? ?? []).whereType<Map>().any(
            (r) =>
                r['mimeType'] ==
                'application/vnd.openmuse.resource-extensions+json',
          )) {
            if (mounted) {
              await RemoteMcpResourceHostScope.of(context)?.refresh();
              if (mounted)
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(stringsOf(context).marketExtensionsReady()),
                  ),
                );
            }
            return;
          }
        }
        final html = await market!.mcpAppDocument(entry);
        if (Platform.isIOS && html == null) {
          throw const FormatException(
            'Reviewed offline MCP App is unavailable',
          );
        }
        File? selected;
        if (html != null) {
          selected = widget.pickFile == null
              ? await FilePicker.platform.pickFiles().then(
                  (r) => r?.files.single.path == null
                      ? null
                      : File(r!.files.single.path!),
                )
              : await widget.pickFile!();
          if (selected == null) return;
        }
        final bytes = await selected?.readAsBytes();
        final selectedMediaType = (await market!.resourceMediaTypes(
          entry,
        )).firstOrNull;
        if (mounted) {
          await Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (_) => RemoteMcpOfflineAppPage(
                title: entry.title,
                digest: entry.packageSha256!.toLowerCase(),
                appHtml: html,
                fileBytes: bytes,
                fileName: selected?.uri.pathSegments.last,
                mediaType: selected == null ? null : selectedMediaType,
                filePath: selected?.path,
              ),
            ),
          );
        }
        return;
      }
      final ui = await market!.openUi(entry);
      if (ui != null) {
        if (mounted) {
          await Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (_) => RemoteMcpAppPage(title: entry.title, uri: ui),
            ),
          );
        }
        return;
      }
    } catch (cause) {
      if (mounted) setState(() => error = '$cause');
      return;
    } finally {
      if (mounted) setState(() => busy = null);
    }
    final File? file;
    if (widget.pickFile case final picker?) {
      file = await picker();
    } else {
      final selection = await FilePicker.platform.pickFiles(
        type: FileType.any,
        withData: false,
      );
      final path = selection?.files.single.path;
      file = path == null ? null : File(path);
    }
    final selectedFile = file;
    if (selectedFile == null) return;
    if (!mounted) return;
    final approved = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: Text(stringsOf(context).marketSendFileTitle()),
        content: Text(
          stringsOf(context).marketSendFileDescription(
            name: selectedFile.uri.pathSegments.last,
            service:
                entry.endpoint?.host ??
                stringsOf(context).marketPluginService(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, false),
            child: Text(stringsOf(context).marketCancel()),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(dialogContext, true),
            child: Text(stringsOf(context).marketSend()),
          ),
        ],
      ),
    );
    if (approved != true || !mounted) return;
    setState(() {
      busy = entry.name;
      error = null;
    });
    try {
      final result = await market!.inspectFile(entry, selectedFile);
      if (mounted)
        setState(
          () => output = const JsonEncoder.withIndent('  ').convert(result),
        );
    } catch (cause) {
      if (mounted) setState(() => error = '$cause');
    } finally {
      if (mounted) setState(() => busy = null);
    }
  }

  List<RemoteMcpEntry> get visibleEntries {
    // Registry records do not declare product categories. Do not infer a
    // category from the transport/runtime used by a plugin.
    if (!{'精选', '全部', '已启用'}.contains(widget.visibleCategory)) return [];
    final query = widget.query.trim().toLowerCase();
    // Discovery shows the current published version. An installed, delisted
    // version remains accessible in My plugins, with its source state visible.
    return (entries ?? <RemoteMcpEntry>[]).where((entry) {
      final local = installed.contains('${entry.name}@${entry.version}');
      if (widget.installedOnly && !local) return false;
      if (!widget.installedOnly && sourceDelisted(entry)) return false;
      return query.isEmpty ||
          '${entry.title} ${entry.description}'.toLowerCase().contains(query);
    }).toList();
  }

  List<Widget> _catalog(BuildContext context) => [
    if (!widget.embedded)
      Row(
        children: [
          Expanded(
            child: TextButton.icon(
              onPressed:
                  widget.providers == null || widget.allowedPackages != null
                  ? null
                  : _configureProvider,
              icon: const Icon(Icons.storefront_outlined, size: 18),
              label: Text(stringsOf(context).marketProvider()),
            ),
          ),
          IconButton(
            tooltip: stringsOf(context).marketRefresh(),
            onPressed: _load,
            icon: const Icon(Icons.refresh),
          ),
        ],
      ),
    if (_activeOrigin == null)
      Padding(
        padding: const EdgeInsets.all(16),
        child: Text(stringsOf(context).marketSelectProvider()),
      ),
    if (entries == null && error == null) const LinearProgressIndicator(),
    if (error != null)
      Padding(
        padding: const EdgeInsets.all(16),
        child: Text(
          stringsOf(context).marketUnavailable(),
          style: TextStyle(color: Theme.of(context).colorScheme.error),
        ),
      ),
    for (final entry in visibleEntries)
      Card(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Padding(
                padding: EdgeInsets.only(right: 14),
                child: Icon(Icons.extension_outlined, size: 36),
              ),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      entry.title,
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                    const SizedBox(height: 6),
                    Text(
                      entry.description,
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 6),
                    Text(
                      '${entry.version}${sourceDelisted(entry) ? ' · ${stringsOf(context).marketDelistedInstalled()}' : ''}',
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                      ),
                    ),
                    if (installed.contains('${entry.name}@${entry.version}') &&
                        market?.hasOfflineUi(entry) == true)
                      Text(
                        stringsOf(context).marketLocalFiles(),
                        style: const TextStyle(fontSize: 12),
                      ),
                  ],
                ),
              ),
              if (busy == entry.name)
                const SizedBox(
                  width: 24,
                  height: 24,
                  child: CircularProgressIndicator(),
                )
              else
                TextButton(
                  onPressed:
                      installed.contains('${entry.name}@${entry.version}')
                      ? () => _use(entry)
                      : () => _install(entry),
                  child: Text(
                    installed.contains('${entry.name}@${entry.version}')
                        ? stringsOf(context).marketOpen()
                        : stringsOf(context).marketInstall(),
                  ),
                ),
            ],
          ),
        ),
      ),
    if (output != null)
      Padding(
        padding: const EdgeInsets.all(16),
        child: SelectableText(output!),
      ),
  ];

  @override
  Widget build(BuildContext context) => widget.embedded
      ? Column(mainAxisSize: MainAxisSize.min, children: _catalog(context))
      : Scaffold(
          appBar: AppBar(title: Text(stringsOf(context).marketTitle())),
          body: ListView(children: _catalog(context)),
        );

  bool sourceDelisted(RemoteMcpEntry entry) =>
      market?.catalogFresh == true &&
      !(market?.activeVersions.contains('${entry.name}@${entry.version}') ??
          true);
}

final class RemoteMcpResourceHostScope extends InheritedWidget {
  const RemoteMcpResourceHostScope({
    super.key,
    required this.open,
    required this.close,
    required this.refresh,
    required super.child,
  });
  final Future<void> Function(RemoteMcpViewConnection, String, String, String)
  open;
  final void Function(RemoteMcpViewConnection) close;
  final Future<void> Function() refresh;
  static RemoteMcpResourceHostScope? of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<RemoteMcpResourceHostScope>();
  @override
  bool updateShouldNotify(RemoteMcpResourceHostScope oldWidget) => false;
}

/// A private native channel transports standard MCP Apps messages. It exposes
/// no Android bridge to the sandboxed plugin iframe.
final class RemoteMcpViewConnection {
  RemoteMcpViewConnection(int id)
    : channel = MethodChannel('io.openmuse/mcp_view/$id'),
      _send = null {
    channel!.setMethodCallHandler((call) async {
      if (call.method != 'message' || closed) return;
      await handleMessage(
        Map<String, Object?>.from(jsonDecode(call.arguments as String) as Map),
      );
    });
  }

  RemoteMcpViewConnection.webView(
    Future<void> Function(Map<String, Object?>) send,
  ) : channel = null,
      _send = send;

  final MethodChannel? channel;
  final Future<void> Function(Map<String, Object?>)? _send;
  final ready = Completer<void>();
  final _pending = <Object, Completer<Map<String, Object?>>>{};
  int _next = 0;
  bool closed = false;
  Future<Map<String, Object?>> Function(String, Map<String, Object?>)?
  onRequest;

  Future<void> handleMessage(Map<String, Object?> message) async {
    if (closed) return;
    final requestId = message['id'];
    if (message['method'] == null && requestId != null) {
      final pending = _pending.remove(requestId);
      if (message['error'] != null) {
        pending?.completeError(StateError('RESOURCE_TOOL_FAILED'));
      } else {
        pending?.complete(
          Map<String, Object?>.from(message['result'] as Map? ?? {}),
        );
      }
      return;
    }
    if (message['method'] == 'ui/notifications/initialized') {
      if (!ready.isCompleted) ready.complete();
    }
    if (requestId == null) return;
    try {
      final result = await onRequest?.call(
        message['method'] as String,
        Map<String, Object?>.from(message['params'] as Map? ?? {}),
      );
      if (result == null) throw StateError('METHOD_UNAVAILABLE');
      await receive({'jsonrpc': '2.0', 'id': requestId, 'result': result});
    } catch (error) {
      await receive({
        'jsonrpc': '2.0',
        'id': requestId,
        'error': {
          'code': -32602,
          'message': error is StateError ? error.message : 'Request rejected',
        },
      });
    }
  }

  Future<void> receive(Map<String, Object?> message) async {
    if (closed) return;
    if (_send case final send?) {
      await send(message);
    } else {
      await channel!.invokeMethod<void>('receive', jsonEncode(message));
    }
  }

  Future<Map<String, Object?>> request(
    String method,
    Map<String, Object?> params,
  ) async {
    if (closed) throw StateError('RESOURCE_SESSION_CLOSED');
    await ready.future.timeout(const Duration(seconds: 60));
    if (closed) throw StateError('RESOURCE_SESSION_CLOSED');
    final id = 'mobile-${++_next}';
    final pending = Completer<Map<String, Object?>>();
    _pending[id] = pending;
    try {
      await receive({
        'jsonrpc': '2.0',
        'id': id,
        'method': method,
        'params': params,
      });
      return await pending.future.timeout(const Duration(seconds: 30));
    } finally {
      _pending.remove(id);
    }
  }

  void dispose() {
    closed = true;
    channel?.setMethodCallHandler(null);
    for (final pending in _pending.values) {
      pending.completeError(StateError('RESOURCE_SESSION_CLOSED'));
    }
    _pending.clear();
  }
}

final class RemoteMcpOfflineView extends StatefulWidget {
  const RemoteMcpOfflineView({
    super.key,
    required this.digest,
    this.hostDocument,
    this.fileName,
    this.mediaType,
    this.fileBytes,
    this.filePath,
    this.onCreated,
    this.onClosed,
  });
  final String digest;
  final String? hostDocument, fileName, mediaType, filePath;
  final Uint8List? fileBytes;
  final ValueChanged<RemoteMcpViewConnection>? onCreated, onClosed;
  @override
  State<RemoteMcpOfflineView> createState() => _RemoteMcpOfflineViewState();
}

final class _RemoteMcpOfflineViewState extends State<RemoteMcpOfflineView> {
  RemoteMcpViewConnection? _connection;
  Map<String, Object?> _presentation = const {};
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _presentation = mcpAppPresentation(context);
    unawaited(_sendPresentation());
  }

  Future<void> _sendPresentation() async {
    final connection = _connection;
    if (connection == null ||
        !connection.ready.isCompleted ||
        connection.closed)
      return;
    try {
      await connection.receive({'_hostContext': _presentation});
    } catch (_) {}
  }

  @override
  void dispose() {
    if (_connection case final connection?) {
      widget.onClosed?.call(connection);
      connection.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Platform.isAndroid
      ? AndroidView(
          viewType: 'io.openmuse/offline_web_plugin',
          onPlatformViewCreated: (id) {
            final connection = RemoteMcpViewConnection(id);
            _connection = connection;
            unawaited(connection.ready.future.then((_) => _sendPresentation()));
            widget.onCreated?.call(connection);
          },
          creationParams: {
            'digest': widget.digest,
            if (widget.hostDocument != null) 'html': widget.hostDocument,
            if (widget.filePath != null) 'filePath': widget.filePath,
            if (widget.fileName != null) 'fileName': widget.fileName,
            if (widget.mediaType != null) 'mediaType': widget.mediaType,
            if (widget.fileBytes != null) 'fileBytes': widget.fileBytes,
          },
          creationParamsCodec: const StandardMessageCodec(),
        )
      : Center(child: Text(stringsOf(context).marketOfflineUnsupported()));
}

final class RemoteMcpOfflineAppPage extends StatefulWidget {
  const RemoteMcpOfflineAppPage({
    super.key,
    required this.title,
    required this.digest,
    this.fileName,
    this.mediaType,
    this.fileBytes,
    this.appHtml,
    this.filePath,
    this.onCreated,
    this.onClosed,
  });
  final String title, digest;
  final String? fileName, mediaType, appHtml, filePath;
  final Uint8List? fileBytes;
  final ValueChanged<RemoteMcpViewConnection>? onCreated, onClosed;
  @override
  State<RemoteMcpOfflineAppPage> createState() =>
      _RemoteMcpOfflineAppPageState();
}

final class _RemoteMcpOfflineAppPageState
    extends State<RemoteMcpOfflineAppPage> {
  RemoteMcpResourceHostScope? _host;
  RemoteMcpViewConnection? _connection;
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _host = RemoteMcpResourceHostScope.of(context);
  }

  void _created(RemoteMcpViewConnection connection) {
    _connection = connection;
    widget.onCreated?.call(connection);
    unawaited(
      _host?.open(
        connection,
        widget.title,
        widget.mediaType ?? 'application/octet-stream',
        widget.digest,
      ),
    );
  }

  void _closed(RemoteMcpViewConnection connection) {
    if (identical(_connection, connection)) _connection = null;
    _host?.close(connection);
    widget.onClosed?.call(connection);
  }

  @override
  Widget build(BuildContext context) {
    final digest = widget.digest;
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        actions: [
          if (_host != null)
            IconButton(
              tooltip: stringsOf(context).marketExtensions(),
              icon: const Icon(Icons.extension_outlined),
              onPressed: () => unawaited(_host?.refresh()),
            ),
        ],
      ),
      body: Platform.isIOS && widget.appHtml != null
          ? _IosOfflineMcpAppView(
              title: widget.title,
              appHtml: widget.appHtml!,
              fileName: widget.fileName,
              fileBytes: widget.fileBytes,
              filePath: widget.filePath,
              resourceToolsAvailable: _host != null || widget.onCreated != null,
              onCreated: _created,
              onClosed: _closed,
            )
          : RemoteMcpOfflineView(
              digest: digest,
              hostDocument: widget.appHtml == null
                  ? null
                  : buildMcpAppHostDocument(
                      appUrl:
                          'https://p${digest.substring(0, 32)}.p${digest.substring(32)}.plugins.openmuse.io/editor/mcp-app.html',
                      argumentsUrl:
                          'https://p${digest.substring(0, 32)}.p${digest.substring(32)}.plugins.openmuse.io/host/arguments.json',
                      arguments: const {},
                      platform: 'mobile',
                      presentation: mcpAppPresentation(context),
                      theme: Theme.of(context).brightness.name,
                      resourceToolsAvailable:
                          _host != null || widget.onCreated != null,
                      nativeSend:
                          'window.McpHost.postMessage(JSON.stringify(message))',
                    ),
              fileName: widget.fileName,
              mediaType: widget.mediaType,
              fileBytes: widget.fileBytes,
              filePath: widget.filePath,
              onCreated: _created,
              onClosed: _closed,
            ),
    );
  }
}

/// iOS renders only the reviewed, CSP-restricted web document extracted from
/// the verified MCPB. The sandboxed iframe cannot reach the native bridge;
/// the outer host handles standard MCP Apps messages and user-selected files.
final class _IosOfflineMcpAppView extends StatefulWidget {
  const _IosOfflineMcpAppView({
    required this.title,
    required this.appHtml,
    this.fileName,
    this.fileBytes,
    this.filePath,
    required this.resourceToolsAvailable,
    this.onCreated,
    this.onClosed,
  });

  final String title, appHtml;
  final String? fileName, filePath;
  final Uint8List? fileBytes;
  final bool resourceToolsAvailable;
  final ValueChanged<RemoteMcpViewConnection>? onCreated, onClosed;

  @override
  State<_IosOfflineMcpAppView> createState() => _IosOfflineMcpAppViewState();
}

final class _IosOfflineMcpAppViewState extends State<_IosOfflineMcpAppView> {
  late final WebViewController _controller;
  late final RemoteMcpViewConnection _connection;
  bool _loaded = false, _documentStarted = false;
  Map<String, Object?> _presentation = const {};
  String? _error;

  @override
  void initState() {
    super.initState();
    _controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..addJavaScriptChannel(
        'McpHost',
        onMessageReceived: (message) => unawaited(_receive(message.message)),
      )
      ..setNavigationDelegate(
        NavigationDelegate(
          onPageFinished: (_) {
            if (mounted) {
              setState(() => _loaded = true);
              unawaited(_sendPresentation());
            }
          },
          onWebResourceError: (error) {
            if (error.isForMainFrame == true && mounted) {
              setState(() => _error = error.description);
            }
          },
          onNavigationRequest: (request) {
            if (!request.isMainFrame) return NavigationDecision.navigate;
            final scheme = Uri.tryParse(request.url)?.scheme;
            return scheme == 'about' || scheme == 'data'
                ? NavigationDecision.navigate
                : NavigationDecision.prevent;
          },
        ),
      );
    _connection = RemoteMcpViewConnection.webView(
      (message) => _controller.runJavaScript(
        'window.__mcpHostReceive(${jsonEncode(message)})',
      ),
    );
    widget.onCreated?.call(_connection);
  }

  @override
  void dispose() {
    widget.onClosed?.call(_connection);
    _connection.dispose();
    super.dispose();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _presentation = mcpAppPresentation(context);
    if (_loaded) {
      unawaited(_sendPresentation());
      return;
    }
    if (_documentStarted || _error != null) return;
    _documentStarted = true;
    final presentation = _presentation;
    final content = widget.fileBytes;
    final document = buildMcpAppHostDocument(
      appHtml: widget.appHtml,
      arguments: {
        'name': widget.fileName ?? 'Document',
        'content': base64Encode(content ?? Uint8List(0)),
      },
      platform: 'mobile',
      resourceToolsAvailable: widget.resourceToolsAvailable,
      presentation: presentation,
      theme: Theme.of(context).brightness.name,
      nativeSend: 'window.McpHost.postMessage(JSON.stringify(message))',
    );
    unawaited(
      _controller.loadHtmlString(document).catchError((Object error) {
        if (mounted) setState(() => _error = '$error');
      }),
    );
  }

  Future<void> _sendPresentation() async {
    try {
      await _controller.runJavaScript(
        'window.__mcpHostReceive(${jsonEncode({'_hostContext': _presentation})})',
      );
    } on Object {
      // Keep the mounted document and retry with the next presentation update.
    }
  }

  Future<void> _receive(String raw) async {
    if (raw.length > 96 * 1024 * 1024) return;
    final Map<String, dynamic> request;
    try {
      request = Map<String, dynamic>.from(jsonDecode(raw) as Map);
    } on Object {
      return;
    }
    if (request['method'] != 'ui/download-file') {
      await _connection.handleMessage(request);
      return;
    }
    final response = <String, Object?>{'jsonrpc': '2.0', 'id': request['id']};
    try {
      final contents = (request['params'] as Map)['contents'] as List;
      if (contents.length != 1) throw const FormatException('Invalid export');
      final resource = (contents.single as Map)['resource'] as Map;
      final bytes = resource['blob'] is String
          ? base64Decode(resource['blob'] as String)
          : utf8.encode(resource['text'] as String);
      if (bytes.length > 64 * 1024 * 1024) {
        throw const FormatException('Export exceeds 64 MiB');
      }
      final name = Uri.parse(resource['uri'] as String).pathSegments.last;
      if (name.isEmpty || name.length > 200 || name.contains('\\')) {
        throw const FormatException('Invalid export name');
      }
      final destination = widget.filePath == null
          ? File('${(await getApplicationDocumentsDirectory()).path}/$name')
          : File(widget.filePath!);
      await destination.parent.create(recursive: true);
      final staging = File('${destination.path}.saving');
      await staging.writeAsBytes(bytes, flush: true);
      await staging.rename(destination.path);
      response['result'] = <String, Object?>{};
    } on Object {
      response['error'] = {
        'code': -32603,
        'message': 'Unable to save the authorized resource',
      };
    }
    await _controller.runJavaScript(
      'window.__mcpHostReceive(${jsonEncode(response)})',
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) {
      return Center(
        child: Text(stringsOf(context).marketOpenFailed(title: widget.title)),
      );
    }
    return Stack(
      children: [
        WebViewWidget(controller: _controller),
        if (!_loaded) const Center(child: CircularProgressIndicator()),
      ],
    );
  }
}

final class RemoteMcpAppPage extends StatefulWidget {
  const RemoteMcpAppPage({super.key, required this.title, required this.uri});
  final String title;
  final Uri uri;

  @override
  State<RemoteMcpAppPage> createState() => _RemoteMcpAppPageState();
}

final class _RemoteMcpAppPageState extends State<RemoteMcpAppPage> {
  late final WebViewController controller;

  @override
  void initState() {
    super.initState();
    controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setNavigationDelegate(
        NavigationDelegate(
          onNavigationRequest: (request) {
            final next = Uri.tryParse(request.url);
            return next != null &&
                    next.scheme == widget.uri.scheme &&
                    next.host == widget.uri.host &&
                    next.port == widget.uri.port
                ? NavigationDecision.navigate
                : NavigationDecision.prevent;
          },
        ),
      )
      ..loadRequest(widget.uri);
  }

  @override
  Widget build(BuildContext context) {
    final webView = Platform.isAndroid
        ? WebViewWidget.fromPlatformCreationParams(
            params: AndroidWebViewWidgetCreationParams(
              controller: controller.platform,
              displayWithHybridComposition: true,
            ),
          )
        : WebViewWidget(controller: controller);
    return Scaffold(
      appBar: AppBar(title: Text(widget.title)),
      body: webView,
    );
  }
}
