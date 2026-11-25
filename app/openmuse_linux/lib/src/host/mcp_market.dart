import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:path/path.dart' as p;
import 'package:pub_semver/pub_semver.dart';

/// A standard MCP Registry server.json entry. No Client plugin manifest is read.
final class McpMarketEntry {
  const McpMarketEntry({
    required this.name,
    required this.title,
    required this.description,
    required this.version,
    required this.packageUri,
    required this.sha256,
  });

  factory McpMarketEntry.fromRegistry(Map<dynamic, dynamic> record) {
    final server = record['server'];
    if (server is! Map || server['packages'] is! List) {
      throw const FormatException('Invalid MCP Registry server record');
    }
    final packages = server['packages'] as List;
    final package = packages
        .whereType<Map>()
        .where(
          (item) =>
              item['registryType'] == 'mcpb' &&
              item['transport'] is Map &&
              (item['transport'] as Map)['type'] == 'stdio',
        )
        .firstOrNull;
    if (package == null ||
        server['name'] is! String ||
        server['version'] is! String ||
        package['identifier'] is! String ||
        package['fileSha256'] is! String) {
      throw const FormatException('Unsupported MCPB registry record');
    }
    final uri = Uri.parse(package['identifier'] as String);
    final digest = package['fileSha256'] as String;
    if (!RegExp(r'^[0-9a-f]{64}$').hasMatch(digest)) {
      throw const FormatException('Invalid MCPB SHA-256');
    }
    return McpMarketEntry(
      name: server['name'] as String,
      title: server['title'] is String
          ? server['title'] as String
          : server['name'] as String,
      description: server['description'] is String
          ? server['description'] as String
          : '',
      version: server['version'] as String,
      packageUri: uri,
      sha256: digest,
    );
  }

  final String name;
  final String title;
  final String description;
  final String version;
  final Uri packageUri;
  final String sha256;

  Map<String, Object?> toMarketCard() => {
    'id': name,
    'name': title,
    'ecosystem': 'mcp',
    'package': name,
    'version': version,
    'integrity': 'sha256:$sha256',
    'supportedDshVersions': <String>[],
    'description': description,
    'categories': <String>['工具'],
  };
}

/// Runs only the MCPB-defined stdio server and calls standard MCP tools.
final class McpBundleRuntime {
  McpBundleRuntime({
    required this.bundleDirectory,
    required this.nodeExecutable,
  });

  final Directory bundleDirectory;
  final String nodeExecutable;

  String _resolvedNode() {
    if (p.isAbsolute(nodeExecutable)) return nodeExecutable;
    final separator = Platform.isWindows ? ';' : ':';
    final pathKey = Platform.isWindows ? 'Path' : 'PATH';
    for (final directory in (Platform.environment[pathKey] ?? '').split(
      separator,
    )) {
      final candidate = p.join(directory, nodeExecutable);
      if (File(candidate).existsSync()) return candidate;
    }
    throw StateError('Node runtime unavailable for MCPB');
  }

  Future<T> _withClient<T>(
    Future<T> Function(McpClient client) operation,
  ) async {
    final session = await connect();
    try {
      return await operation(
        session.client,
      ).timeout(const Duration(seconds: 30));
    } finally {
      await session.close();
    }
  }

  /// One MCP connection per interactive View. Credentials are supplied only
  /// through MCPB user_config, never through the View or inherited environment.
  Future<McpBundleSession> connect({
    Map<String, Object?> configuration = const {},
  }) async {
    final manifest = jsonDecode(
      await File(p.join(bundleDirectory.path, 'manifest.json')).readAsString(),
    );
    if (manifest is! Map || manifest['server'] is! Map) {
      throw const FormatException('Invalid installed MCPB manifest');
    }
    final entry = (manifest['server'] as Map)['entry_point'];
    if (entry is! String ||
        entry.startsWith('/') ||
        entry.split('/').contains('..')) {
      throw const FormatException('Invalid MCPB entry point');
    }
    final executable = File(p.join(bundleDirectory.path, entry));
    if (!await executable.exists()) {
      throw const FileSystemException('MCPB server missing');
    }
    final config = (manifest['server'] as Map)['mcp_config'];
    final declared = config is Map ? config['env'] : null;
    final environment = <String, String>{'PATH': '/usr/bin:/bin'};
    if (declared is Map) {
      for (final item in declared.entries) {
        if (item.key is! String || item.value is! String) {
          throw const FormatException('Invalid MCPB environment');
        }
        environment[item.key as String] = (item.value as String)
            .replaceAllMapped(
              RegExp(r'\$\{(__dirname|user_config\.[a-zA-Z0-9_]+)\}'),
              (match) => match[1] == '__dirname'
                  ? bundleDirectory.path
                  : '${configuration[match[1]!.substring(12)] ?? ''}',
            );
      }
    }
    final client = McpClient(
      const Implementation(name: 'OpenMuse', version: '1.0.0'),
    );
    final transport = StdioClientTransport(
      StdioServerParameters(
        command: _resolvedNode(),
        args: [executable.path],
        includeParentEnvironment: false,
        environment: environment,
        workingDirectory: bundleDirectory.path,
        maxIncomingMessageBytes: 128 * 1024 * 1024,
      ),
    );
    try {
      await client.connect(transport).timeout(const Duration(seconds: 10));
      return McpBundleSession(client);
    } catch (_) {
      await client.close();
      rethrow;
    }
  }

  Future<List<Map<String, Object?>>> tools() => _withClient((client) async {
    final result = await client.listTools();
    return [
      for (final tool in result.tools)
        {
          ...tool.toJson(),
          'name': tool.name,
          'description': tool.description,
          'inputSchema': tool.inputSchema.toJson(),
          'uiResourceUri': (tool.meta?['ui'] is Map)
              ? (tool.meta!['ui'] as Map)['resourceUri']
              : null,
        },
    ];
  });

  Future<Map<String, Object?>> callTool(
    String name,
    Map<String, Object?> arguments,
  ) => _withClient(
    (client) async => (await client.callTool(
      CallToolRequest(name: name, arguments: arguments),
    )).toJson(),
  );

  /// Reads a standard MCP Apps UI resource. The View implements its own business
  /// logic; the Host supplies the selected file as standard tool arguments.
  Future<String?> uiDocument(String toolName) => _withClient((client) async {
    final tools = await client.listTools();
    final tool = tools.tools.where((item) => item.name == toolName).firstOrNull;
    final ui = tool?.meta?['ui'];
    if (ui is! Map ||
        ui['resourceUri'] is! String ||
        !(ui['resourceUri'] as String).startsWith('ui://')) {
      return null;
    }
    final resource = await client.readResource(
      ReadResourceRequest(uri: ui['resourceUri'] as String),
    );
    for (final content in resource.contents.whereType<TextResourceContents>()) {
      if (content.mimeType == 'text/html;profile=mcp-app') return content.text;
    }
    return null;
  });

  /// Uses MCP Apps tool metadata and the standard resource_link result.
  Future<Uri?> openUi() => _withClient((client) async {
    final listed = await client.listTools();
    for (final tool in listed.tools) {
      final ui = tool.meta?['ui'];
      if (ui is! Map || ui['resourceUri'] is! String) continue;
      final schema = tool.inputSchema.toJson();
      if (schema['required'] is List &&
          (schema['required'] as List).isNotEmpty) {
        continue;
      }
      final result = await client.callTool(
        CallToolRequest(name: tool.name, arguments: const {}),
      );
      if (result.isError) throw StateError('MCP App tool failed');
      for (final content in result.content.whereType<ResourceLink>()) {
        final uri = Uri.tryParse(content.uri);
        if (uri != null &&
            (uri.scheme == 'https' ||
                (uri.scheme == 'http' &&
                    {'localhost', '127.0.0.1'}.contains(uri.host))) &&
            content.mimeType == 'text/html') {
          return uri;
        }
      }
    }
    return null;
  });

  Future<Map<String, Object?>> callWithFile({
    required String toolName,
    required String filePath,
  }) async {
    final file = File(filePath);
    if (!await file.exists() || await file.length() > 6 * 1024 * 1024) {
      throw const FileSystemException(
        'Selected file is unavailable or exceeds 6 MiB',
      );
    }
    return _withClient((client) async {
      final listed = await client.listTools();
      final tool = listed.tools
          .where((item) => item.name == toolName)
          .firstOrNull;
      if (tool == null) throw const FormatException('MCP tool not advertised');
      final schema = tool.inputSchema.toJson();
      final properties = schema['properties'];
      if (properties is! Map ||
          properties['content'] is! Map ||
          (properties['content'] as Map)['contentEncoding'] != 'base64') {
        throw const FormatException('MCP tool does not accept a base64 file');
      }
      final result = await client.callTool(
        CallToolRequest(
          name: toolName,
          arguments: {
            'content': base64Encode(await file.readAsBytes()),
            if (properties['name'] is Map) 'name': p.basename(filePath),
          },
        ),
      );
      if (result.isError) throw StateError('MCP tool returned an error');
      for (final content in result.content.whereType<ResourceLink>()) {
        final uri = Uri.tryParse(content.uri);
        if (content.mimeType == 'text/html' &&
            uri != null &&
            (uri.scheme == 'https' ||
                (uri.scheme == 'http' &&
                    {'localhost', '127.0.0.1'}.contains(uri.host)))) {
          return {'uri': uri.toString()};
        }
      }
      final data = result.structuredContent;
      if (data == null) {
        throw const FormatException('MCP tool returned no structured content');
      }
      return Map<String, Object?>.from(data);
    });
  }
}

final class McpBundleSession {
  McpBundleSession(this.client);
  final McpClient client;
  bool _closed = false;
  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    await client.close();
  }
}

final class McpRegistryMarket {
  McpRegistryMarket({
    required this.origin,
    required this.installRoot,
    this.nodeExecutable = 'node',
    HttpClient? httpClient,
  }) : _http = httpClient ?? HttpClient();

  final Uri origin;
  final Directory installRoot;
  final String nodeExecutable;
  final HttpClient _http;

  bool _sameOrigin(Uri other) =>
      other.scheme == origin.scheme &&
      other.host == origin.host &&
      other.port == origin.port;

  void _checkUri(Uri uri) {
    if (!(uri.scheme == 'https' ||
        (uri.scheme == 'http' &&
            _sameOrigin(uri) &&
            (uri.host == 'localhost' || uri.host == '127.0.0.1')))) {
      throw FormatException('MCP registry requires HTTPS or loopback: $uri');
    }
  }

  Future<List<McpMarketEntry>> list() async {
    final entries = <McpMarketEntry>[];
    final seen = <String>{};
    String? cursor;
    for (var page = 0; page < 20; page++) {
      final uri = origin
          .resolve('/v0.1/servers')
          .replace(queryParameters: cursor == null ? null : {'cursor': cursor});
      _checkUri(uri);
      final data = jsonDecode(utf8.decode(await _get(uri, 512 * 1024)));
      if (data is! Map || data['servers'] is! List) {
        throw const FormatException('Invalid MCP Registry response');
      }
      for (final record in data['servers'] as List) {
        if (record is! Map) continue;
        try {
          entries.add(McpMarketEntry.fromRegistry(record));
        } on FormatException {
          // A server without an MCPB package may target remote/mobile clients.
        }
      }
      final metadata = data['metadata'];
      final next = metadata is Map ? metadata['nextCursor'] : null;
      if (next is! String || next.isEmpty) return entries;
      if (!seen.add(next)) {
        throw const FormatException('MCP Registry cursor loop');
      }
      cursor = next;
    }
    throw const FormatException('MCP Registry pagination limit exceeded');
  }

  Directory directoryFor(McpMarketEntry entry) => Directory(
    p.join(
      installRoot.path,
      entry.name.replaceAll(RegExp(r'[^a-zA-Z0-9._-]'), '_'),
    ),
  );

  Future<List<McpMarketEntry>> installedEntries() async {
    if (!await installRoot.exists()) return [];
    final entries = <McpMarketEntry>[];
    await for (final child in installRoot.list()) {
      if (child is! Directory) continue;
      final receipt = File(p.join(child.path, 'receipt.json'));
      if (!await receipt.exists()) continue;
      try {
        final record = jsonDecode(await receipt.readAsString());
        if (record is! Map) continue;
        entries.add(
          McpMarketEntry(
            name: record['name'] as String,
            title: record['title'] as String,
            description: record['description'] as String,
            version: record['version'] as String,
            packageUri: Uri.parse(record['packageUri'] as String),
            sha256: record['sha256'] as String,
          ),
        );
      } catch (_) {
        // A corrupt receipt cannot activate downloaded code.
      }
    }
    return entries;
  }

  Future<Map<String, Object?>> plan(McpMarketEntry entry) async {
    _checkUri(entry.packageUri);
    final receipt = File(p.join(directoryFor(entry).path, 'receipt.json'));
    String? installed;
    if (await receipt.exists()) {
      final decoded = jsonDecode(await receipt.readAsString());
      if (decoded is Map && decoded['sha256'] == entry.sha256) {
        installed = decoded['version'] as String?;
      }
    }
    return {
      ...entry.toMarketCard(),
      'installedVersion': installed,
      'enabled': installed != null,
      'action': installed == entry.version ? 'already-installed' : 'install',
    };
  }

  Future<Map<String, Object?>> install(
    McpMarketEntry entry, {
    List<int>? bundleBytes,
  }) async {
    final planned = await plan(entry);
    if (planned['action'] == 'already-installed') return planned;
    final bytes = bundleBytes ?? await _get(entry.packageUri, 80 * 1024 * 1024);
    if (bytes.length > 80 * 1024 * 1024) {
      throw const FormatException('MCPB exceeds budget');
    }
    if (sha256.convert(bytes).toString() != entry.sha256) {
      throw const FormatException('MCPB digest mismatch');
    }
    final archive = ZipDecoder().decodeBytes(bytes);
    if (archive.length > 1024) {
      throw const FormatException('MCPB has too many files');
    }
    final files = <String, Uint8List>{};
    var expandedSize = 0;
    for (final item in archive) {
      final name = item.name.replaceAll('\\', '/');
      if (name.startsWith('/') ||
          name.split('/').contains('..') ||
          p.isAbsolute(name)) {
        throw const FormatException('Unsafe MCPB archive path');
      }
      if (!item.isFile) continue;
      final content = item.content;
      expandedSize += content.length;
      if (expandedSize > 160 * 1024 * 1024) {
        throw const FormatException('MCPB expanded size limit exceeded');
      }
      files[name] = Uint8List.fromList(content);
    }
    final manifestBytes = files['manifest.json'];
    if (manifestBytes == null) {
      throw const FormatException('MCPB manifest missing');
    }
    final manifest = jsonDecode(utf8.decode(manifestBytes));
    if (manifest is! Map ||
        !{'0.3', '0.4'}.contains(manifest['manifest_version']) ||
        manifest['version'] != entry.version ||
        manifest['server'] is! Map ||
        (manifest['server'] as Map)['type'] != 'node') {
      throw const FormatException('Unsupported MCPB manifest');
    }
    final server = manifest['server'] as Map;
    final compatibility = manifest['compatibility'];
    if (compatibility is Map) {
      final platforms = compatibility['platforms'];
      final platform = Platform.isMacOS
          ? 'darwin'
          : Platform.isWindows
          ? 'win32'
          : 'linux';
      if (platforms is List && !platforms.contains(platform)) {
        throw const FormatException('MCPB does not support this platform');
      }
      final runtimes = compatibility['runtimes'];
      final nodeRange = runtimes is Map ? runtimes['node'] : null;
      if (nodeRange is String) {
        final result = await Process.run(nodeExecutable, ['--version']);
        if (result.exitCode != 0 ||
            !VersionConstraint.parse(nodeRange).allows(
              Version.parse('${result.stdout}'.trim().replaceFirst('v', '')),
            )) {
          throw const FormatException('MCPB requires a different Node version');
        }
      }
    }
    final entryPoint = server['entry_point'];
    if (entryPoint is! String || !files.containsKey(entryPoint)) {
      throw const FormatException('MCPB server entry point missing');
    }
    final target = directoryFor(entry);
    final staging = Directory('${target.path}.staging');
    final backup = Directory('${target.path}.backup');
    if (await staging.exists()) await staging.delete(recursive: true);
    if (await backup.exists()) await backup.delete(recursive: true);
    await staging.create(recursive: true);
    try {
      for (final file in files.entries) {
        final destination = File(p.join(staging.path, file.key));
        await destination.parent.create(recursive: true);
        await destination.writeAsBytes(file.value, flush: true);
      }
      await File(p.join(staging.path, 'receipt.json')).writeAsString(
        jsonEncode({
          'name': entry.name,
          'title': entry.title,
          'description': entry.description,
          'version': entry.version,
          'packageUri': entry.packageUri.toString(),
          'sha256': entry.sha256,
          'fileDigests': {
            for (final file in files.entries)
              file.key: sha256.convert(file.value).toString(),
          },
        }),
        flush: true,
      );
      await McpBundleRuntime(
        bundleDirectory: staging,
        nodeExecutable: nodeExecutable,
      ).tools();
      if (await target.exists()) await target.rename(backup.path);
      try {
        await staging.rename(target.path);
      } catch (_) {
        if (await backup.exists()) await backup.rename(target.path);
        rethrow;
      }
      if (await backup.exists()) await backup.delete(recursive: true);
    } catch (_) {
      if (await staging.exists()) await staging.delete(recursive: true);
      rethrow;
    }
    return {
      ...planned,
      'installedVersion': entry.version,
      'enabled': true,
      'action': 'installed',
    };
  }

  Future<List<int>> _get(Uri uri, int maxBytes) async {
    _checkUri(uri);
    final request = await _http.getUrl(uri);
    final response = await request.close();
    for (final redirect in response.redirects) {
      _checkUri(redirect.location);
    }
    if (response.statusCode != 200) {
      throw HttpException('MCP Registry HTTP ${response.statusCode}', uri: uri);
    }
    final bytes = <int>[];
    await for (final chunk in response) {
      bytes.addAll(chunk);
      if (bytes.length > maxBytes) {
        throw const FormatException('MCP Registry response too large');
      }
    }
    return bytes;
  }
}
