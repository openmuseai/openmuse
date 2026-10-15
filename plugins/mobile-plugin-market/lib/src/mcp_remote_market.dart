import 'dart:convert';
import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:url_launcher/url_launcher.dart';

/// Consumes only MCP Registry server.json and MCP Streamable HTTP.
final class RemoteMcpEntry {
  const RemoteMcpEntry(
    this.name,
    this.title,
    this.version,
    this.description,
    this.endpoint,
  );
  final String name;
  final String title;
  final String version;
  final String description;
  final Uri endpoint;

  factory RemoteMcpEntry.fromRecord(Map record) {
    final server = record['server'];
    if (server is! Map ||
        server['name'] is! String ||
        server['version'] is! String ||
        server['remotes'] is! List) {
      throw const FormatException('Invalid MCP Registry record');
    }
    final remote = (server['remotes'] as List)
        .whereType<Map>()
        .where((item) => item['type'] == 'streamable-http')
        .firstOrNull;
    if (remote == null || remote['url'] is! String)
      throw const FormatException('No MCP Streamable HTTP endpoint');
    final endpoint = Uri.parse(remote['url'] as String);
    if (endpoint.scheme != 'https' &&
        !(endpoint.scheme == 'http' &&
            {'localhost', '127.0.0.1'}.contains(endpoint.host))) {
      throw const FormatException('Remote MCP requires HTTPS');
    }
    return RemoteMcpEntry(
      server['name'] as String,
      server['title'] is String
          ? server['title'] as String
          : server['name'] as String,
      server['version'] as String,
      server['description'] is String ? server['description'] as String : '',
      endpoint,
    );
  }
}

final class RemoteMcpMarket {
  RemoteMcpMarket(this.origin, this.receiptFile);
  final Uri origin;
  final File receiptFile;

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
            entries.add(RemoteMcpEntry.fromRecord(record as Map));
          } on FormatException {
            // Desktop-only MCPB entries are not installable on Mobile.
          }
        }
        cursor = json['metadata'] is Map
            ? (json['metadata'] as Map)['nextCursor'] as String?
            : null;
        if (cursor == null) break;
      }
      return entries;
    } finally {
      http.close(force: true);
    }
  }

  Future<Set<String>> installed() async {
    if (!await receiptFile.exists()) return {};
    final decoded = jsonDecode(await receiptFile.readAsString());
    return (decoded as List).whereType<String>().toSet();
  }

  Future<T> withClient<T>(
    RemoteMcpEntry entry,
    Future<T> Function(McpClient) operation,
  ) async {
    final client = McpClient(
      const Implementation(name: 'OpenMuse Mobile', version: '1.0.0'),
      options: const McpClientOptions(protocol: McpProtocol.require2026),
    );
    final transport = StreamableHttpClientTransport(entry.endpoint);
    try {
      await client.connect(transport).timeout(const Duration(seconds: 15));
      return await operation(client).timeout(const Duration(seconds: 60));
    } finally {
      await client.close();
    }
  }

  Future<void> install(RemoteMcpEntry entry) async {
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
    final receipts = await installed();
    receipts.add('${entry.name}@${entry.version}');
    await receiptFile.parent.create(recursive: true);
    await receiptFile.writeAsString(
      jsonEncode(receipts.toList()..sort()),
      flush: true,
    );
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
    required this.origin,
    this.receiptFile,
    this.pickFile,
  });
  final Uri origin;
  final File? receiptFile;
  final Future<File?> Function()? pickFile;
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

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final directory = widget.receiptFile == null
          ? await getApplicationSupportDirectory()
          : null;
      final source = RemoteMcpMarket(
        widget.origin,
        widget.receiptFile ??
            File('${directory!.path}/mcp-remote-installations.json'),
      );
      final catalog = await source.list();
      final receipts = await source.installed();
      if (mounted)
        setState(() {
          market = source;
          entries = catalog;
          installed = receipts;
        });
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
      final ui = await market!.openUi(entry);
      if (ui != null) {
        if (Platform.isAndroid) {
          if (!await launchUrl(ui, mode: LaunchMode.externalApplication)) {
            throw StateError('无法打开插件页面');
          }
          return;
        }
        if (mounted) {
          await Navigator.of(context).push(
            MaterialPageRoute<void>(
              builder: (_) => _RemoteMcpAppPage(title: entry.title, uri: ui),
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
        title: const Text('发送文件给插件？'),
        content: Text(
          '将 ${selectedFile.uri.pathSegments.last} 发送至 ${entry.endpoint.host} 处理。',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, false),
            child: const Text('取消'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(dialogContext, true),
            child: const Text('发送'),
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

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: const Text('在线 MCP 插件')),
    body: entries == null && error == null
        ? const Center(child: CircularProgressIndicator())
        : ListView(
            children: [
              if (error != null)
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: Text(
                    error!,
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ),
              for (final entry in entries ?? <RemoteMcpEntry>[])
                Card(
                  child: ListTile(
                    title: Text(entry.title),
                    subtitle: Text('${entry.description}\n${entry.version}'),
                    isThreeLine: true,
                    trailing: busy == entry.name
                        ? const CircularProgressIndicator()
                        : TextButton(
                            onPressed:
                                installed.contains(
                                  '${entry.name}@${entry.version}',
                                )
                                ? () => _use(entry)
                                : () => _install(entry),
                            child: Text(
                              installed.contains(
                                    '${entry.name}@${entry.version}',
                                  )
                                  ? '打开 / 使用'
                                  : '安装',
                            ),
                          ),
                  ),
                ),
              if (output != null)
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: SelectableText(output!),
                ),
            ],
          ),
  );
}

final class _RemoteMcpAppPage extends StatefulWidget {
  const _RemoteMcpAppPage({required this.title, required this.uri});
  final String title;
  final Uri uri;

  @override
  State<_RemoteMcpAppPage> createState() => _RemoteMcpAppPageState();
}

final class _RemoteMcpAppPageState extends State<_RemoteMcpAppPage> {
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
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.title)),
    body: WebViewWidget(controller: controller),
  );
}
