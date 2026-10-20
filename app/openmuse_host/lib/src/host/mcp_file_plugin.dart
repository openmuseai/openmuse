import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy_editor_native_plugin/appflowy_editor_native_plugin.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'mcp_market.dart';
import 'resource_agent_broker.dart';

/// Adapts any MCP tool with a JSON Schema base64 file input to a Host editor.
/// The MIME type is standard JSON Schema contentMediaType, supplied by MCP.
final class OpenMuseMcpFilePlugin implements OpenMusePlugin {
  OpenMuseMcpFilePlugin({
    required McpMarketEntry entry,
    required this.runtime,
    required this.toolName,
    required this.mediaType,
    required this.extension,
    this.nativeFlutterEditor = false,
    this.agentBroker,
  }) : descriptor = OpenMusePluginDescriptor(
         id: entry.name,
         name: entry.title,
         version: entry.version,
         runtime: OpenMusePluginRuntime.nativeProcess,
         activationEvents: ['onFileType:$extension'],
         permissions: {'filesystem.workspace.read'},
         editors: [
           OpenMuseEditorContribution(
             id: '${entry.name}.file',
             mediaTypes: {mediaType},
             extensions: {extension},
             priority: 50,
           ),
         ],
       );

  final McpBundleRuntime runtime;
  final String toolName;
  final String mediaType;
  final String extension;
  final bool nativeFlutterEditor;
  final ResourceAgentBroker? agentBroker;

  @override
  final OpenMusePluginDescriptor descriptor;

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') || resource.extension != extension) {
      return const Center(child: Text('请选择本地文件'));
    }
    if (nativeFlutterEditor) {
      return AppFlowyMarkdownEditor(
        key: ValueKey('appflowy-${resource.uri}'),
        filePath: resource.uri.toFilePath(),
      );
    }
    return _McpFileEditor(
      key: ValueKey('mcp-file-${resource.uri}'),
      runtime: runtime,
      toolName: toolName,
      resource: resource,
      mediaType: mediaType,
      agentBroker: agentBroker,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

String? extensionForMediaType(String mediaType) => switch (mediaType) {
  'application/vnd.openxmlformats-officedocument.presentationml.presentation' =>
    'pptx',
  'application/pdf' => 'pdf',
  'text/plain' => 'txt',
  'text/markdown' => 'md',
  _ => null,
};

/// Derives a Host editor contribution from a verified MCPB server's standard
/// tool input schema. The bundle supplies no Host-specific manifest fields.
Future<OpenMuseMcpFilePlugin?> discoverMcpFilePlugin({
  required McpMarketEntry entry,
  required McpBundleRuntime runtime,
  ResourceAgentBroker? agentBroker,
}) async {
  for (final tool in await runtime.tools()) {
    final schema = tool['inputSchema'];
    final properties = schema is Map ? schema['properties'] : null;
    final content = properties is Map ? properties['content'] : null;
    if (content is! Map || content['contentEncoding'] != 'base64') continue;
    final mediaType = content['contentMediaType'];
    if (mediaType is! String) continue;
    final extension = extensionForMediaType(mediaType);
    if (extension == null || tool['name'] is! String) continue;
    return OpenMuseMcpFilePlugin(
      entry: entry,
      runtime: runtime,
      toolName: tool['name'] as String,
      mediaType: mediaType,
      extension: extension,
      agentBroker: agentBroker,
      nativeFlutterEditor:
          entry.name == 'io.openmuse/appflowy-editor' &&
          mediaType == 'text/markdown',
    );
  }
  return null;
}

final class _McpFileEditor extends StatefulWidget {
  const _McpFileEditor({
    super.key,
    required this.runtime,
    required this.toolName,
    required this.resource,
    required this.mediaType,
    this.agentBroker,
  });
  final McpBundleRuntime runtime;
  final String toolName;
  final OpenMuseResource resource;
  final String mediaType;
  final ResourceAgentBroker? agentBroker;
  @override
  State<_McpFileEditor> createState() => _McpFileEditorState();
}

final class _McpFileEditorState extends State<_McpFileEditor> {
  MethodChannel? _channel;
  ResourceAppSession? _session;
  int _sequence = 0;
  final Map<String, Completer<Map<String, Object?>>> _pending = {};

  Future<Map<String, Object?>> _request(
    String method,
    Map<String, Object?> params,
  ) async {
    final channel = _channel;
    if (channel == null || !mounted) throw StateError('Resource view closed');
    final id = 'host:${++_sequence}';
    final result = Completer<Map<String, Object?>>();
    _pending[id] = result;
    try {
      await channel.invokeMethod<void>('message', {
        'jsonrpc': '2.0',
        'id': id,
        'method': method,
        'params': params,
      });
      return await result.future.timeout(const Duration(seconds: 45));
    } finally {
      _pending.remove(id);
    }
  }

  void _created(int id) {
    final channel = MethodChannel('com.openmuse.mcp/app-view/$id');
    _channel = channel;
    final session = ResourceAppSession(
      uri: widget.resource.uri.toString(),
      title: widget.resource.displayName,
      request: _request,
    );
    _session = session;
    channel.setMethodCallHandler((call) async {
      if (call.method != 'message' || call.arguments is! Map) return;
      final message = Map<String, Object?>.from(call.arguments as Map);
      final pending = _pending[message['id']];
      if (message['method'] == null && pending != null) {
        if (message['error'] != null) {
          pending.completeError(
            StateError('Resource tool rejected: ${message['error']}'),
          );
        } else {
          pending.complete(
            Map<String, Object?>.from(message['result'] as Map? ?? {}),
          );
        }
        return;
      }
      final params = Map<String, Object?>.from(message['params'] as Map? ?? {});
      try {
        switch (message['method']) {
          case 'ui/notifications/initialized':
            widget.agentBroker?.register(session);
            return;
          case 'ui/update-model-context':
            if (jsonEncode(params).length > 128 * 1024) {
              throw const FormatException('Context exceeds budget');
            }
            session.context = params;
          case 'ui/message':
            if (widget.agentBroker?.activeUri != session.uri) {
              throw StateError('Inactive resource');
            }
            final sender = widget.agentBroker?.sendAgentMessage;
            if (sender == null) throw StateError('No Agent plugin available');
            final content = (params['content'] as List)
                .map((e) => Map<String, Object?>.from(e as Map))
                .toList();
            if (jsonEncode(content).length > 128 * 1024) {
              throw const FormatException('Message exceeds budget');
            }
            await sender([
              {
                'type': 'text',
                'text':
                    'Active resource: ${session.uri}. Discover its live tools using list_open_resources and list_resource_tools; edit and save through call_resource_tool.',
              },
              ...content,
            ]);
          default:
            throw UnsupportedError('Method unavailable');
        }
        await channel.invokeMethod<void>('message', {
          'jsonrpc': '2.0',
          'id': message['id'],
          'result': {},
        });
      } catch (_) {
        await channel.invokeMethod<void>('message', {
          'jsonrpc': '2.0',
          'id': message['id'],
          'error': {'code': -32602, 'message': 'Agent request rejected'},
        });
      }
    });
  }

  @override
  void dispose() {
    if (_session case final session?) widget.agentBroker?.unregister(session);
    _channel?.setMethodCallHandler(null);
    for (final pending in _pending.values) {
      pending.completeError(StateError('Resource view closed'));
    }
    _pending.clear();
    super.dispose();
  }

  late final Future<Map<String, Object?>> _view = _load();
  Future<Map<String, Object?>> _load() async {
    final html = await widget.runtime.uiDocument(widget.toolName);
    if (html == null) {
      return widget.runtime.callWithFile(
        toolName: widget.toolName,
        filePath: widget.resource.uri.toFilePath(),
      );
    }
    final file = File(widget.resource.uri.toFilePath());
    if (await file.length() > 64 * 1024 * 1024) {
      throw const FormatException('File exceeds 64 MiB');
    }
    return {
      'html': buildMcpAppHostDocument(
        appHtml: html,
        arguments: {
          'content': base64Encode(await file.readAsBytes()),
          'name': widget.resource.displayName,
        },
        platform: 'desktop',
        agentAvailable: widget.agentBroker != null,
      ),
    };
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<Map<String, Object?>>(
    future: _view,
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return Center(child: SelectableText('插件打开失败：${snapshot.error}'));
      }
      if (!snapshot.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      final data = snapshot.data!;
      if (data['html'] is String || data['uri'] is String) {
        return AppKitView(
          viewType: 'com.openmuse.mcp/app-view',
          creationParams: {
            if (data['html'] is String)
              'html': data['html']
            else
              'url': data['uri'],
            'filePath': widget.resource.uri.toFilePath(),
          },
          creationParamsCodec: const StandardMessageCodec(),
          onPlatformViewCreated: _created,
        );
      }
      return SelectableText(const JsonEncoder.withIndent('  ').convert(data));
    },
  );
}
