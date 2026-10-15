import 'dart:convert';

import 'package:flutter/material.dart';
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
    return FutureBuilder<Map<String, Object?>>(
      future: runtime.callWithFile(
        toolName: toolName,
        filePath: resource.uri.toFilePath(),
      ),
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Center(child: Text('MCP 工具执行失败：${snapshot.error}'));
        }
        if (!snapshot.hasData) {
          return const Center(child: CircularProgressIndicator());
        }
        final data = snapshot.data!;
        return ListView(
          padding: const EdgeInsets.all(20),
          children: [
            Text(
              descriptor.name,
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 12),
            SelectableText(const JsonEncoder.withIndent('  ').convert(data)),
          ],
        );
      },
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
  _ => null,
};
