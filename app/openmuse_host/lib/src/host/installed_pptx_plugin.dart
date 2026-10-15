import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Host-owned adapter for the trusted local PPTX development artifact.
/// The installed worker supplies the parser; the Host supplies the editor UI.
final class OpenMuseInstalledPptxPlugin implements OpenMusePlugin {
  OpenMuseInstalledPptxPlugin({
    required this.workerPath,
    required this.nodeExecutable,
  });

  final String workerPath;
  final String nodeExecutable;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.pptx',
    name: 'PPTX · GenOffice Core',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.nativeProcess,
    activationEvents: ['onFileType:pptx'],
    permissions: {'filesystem.workspace.read'},
    editors: [
      OpenMuseEditorContribution(
        id: 'pptx.viewer',
        mediaTypes: {
          'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          'application/pptx',
        },
        extensions: {'pptx'},
        priority: 50,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') || resource.extension != 'pptx') {
      return const Center(child: Text('PPTX 仅接受 Host 授权的本地文件'));
    }
    return _PptxInspection(
      key: ValueKey(resource.uri),
      filePath: resource.uri.toFilePath(),
      workerPath: workerPath,
      nodeExecutable: nodeExecutable,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _PptxInspection extends StatefulWidget {
  const _PptxInspection({
    super.key,
    required this.filePath,
    required this.workerPath,
    required this.nodeExecutable,
  });

  final String filePath;
  final String workerPath;
  final String nodeExecutable;

  @override
  State<_PptxInspection> createState() => _PptxInspectionState();
}

final class _PptxInspectionState extends State<_PptxInspection> {
  late final Future<Map<String, Object?>> _inspection = inspectInstalledPptx(
    workerPath: widget.workerPath,
    nodeExecutable: widget.nodeExecutable,
    filePath: widget.filePath,
  );

  @override
  Widget build(BuildContext context) => FutureBuilder<Map<String, Object?>>(
    future: _inspection,
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return Center(child: Text('PPTX 打开失败：${snapshot.error}'));
      }
      final inspection = snapshot.data;
      if (inspection == null) {
        return const Center(child: CircularProgressIndicator());
      }
      final pages = inspection['pages'] as List;
      return ListView(
        padding: const EdgeInsets.all(20),
        children: [
          const Chip(label: Text('GenOffice PPTX · 结构预览')),
          Text('共 ${inspection['slides']} 页'),
          const SizedBox(height: 12),
          for (final page in pages)
            if (page is Map)
              ListTile(
                title: Text('第 ${(page['index'] as int) + 1} 页'),
                subtitle: Text('${page['elements']} 个元素'),
              ),
        ],
      );
    },
  );
}

Future<Map<String, Object?>> inspectInstalledPptx({
  required String workerPath,
  required String nodeExecutable,
  required String filePath,
}) async {
  final file = File(filePath);
  final worker = File(workerPath);
  if (!await file.exists() ||
      !await worker.exists() ||
      await file.length() > 64 * 1024 * 1024) {
    throw const FileSystemException('PPTX 文件或插件 worker 不可用');
  }
  final process = await Process.start(
    nodeExecutable,
    [workerPath, 'inspect', filePath],
    runInShell: false,
    includeParentEnvironment: false,
  );
  late final List<String> output;
  try {
    output = await Future.wait([
      _readLimited(process.stdout, 1024 * 1024),
      _readLimited(process.stderr, 4096),
      process.exitCode.then((code) => '$code'),
    ]).timeout(const Duration(seconds: 30));
  } catch (_) {
    process.kill(ProcessSignal.sigkill);
    rethrow;
  }
  if (output[2] != '0') {
    throw StateError('PPTX worker 失败：${output[1]}');
  }
  final decoded = jsonDecode(output[0]);
  if (decoded is! Map ||
      decoded['schema'] != 'openmuse.pptx.inspect/v1' ||
      decoded['slides'] is! int ||
      decoded['pages'] is! List) {
    throw const FormatException('PPTX worker 返回了无效响应');
  }
  return Map<String, Object?>.from(decoded);
}

Future<String> _readLimited(Stream<List<int>> stream, int maxBytes) async {
  final bytes = <int>[];
  await for (final chunk in stream) {
    if (bytes.length + chunk.length > maxBytes) {
      throw const FormatException('PPTX worker 输出超限');
    }
    bytes.addAll(chunk);
  }
  return utf8.decode(bytes);
}
