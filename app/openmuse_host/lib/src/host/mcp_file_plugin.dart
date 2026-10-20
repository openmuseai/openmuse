import 'dart:convert';
import 'dart:io';

import 'package:appflowy_editor_native_plugin/appflowy_editor_native_plugin.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'mcp_market.dart';

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
  });
  final McpBundleRuntime runtime;
  final String toolName;
  final OpenMuseResource resource;
  final String mediaType;
  @override
  State<_McpFileEditor> createState() => _McpFileEditorState();
}

final class _McpFileEditorState extends State<_McpFileEditor> {
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
        );
      }
      return SelectableText(const JsonEncoder.withIndent('  ').convert(data));
    },
  );
}
