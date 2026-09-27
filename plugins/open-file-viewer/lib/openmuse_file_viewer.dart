library;

import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;
import 'package:flutter/services.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

final class OpenMuseFileViewerPlugin implements OpenMusePlugin {
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.open-file-viewer',
    name: 'Open File Viewer',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: [
      'onFileType:markdown',
      'onFileType:image',
      'onFileType:pdf',
    ],
    permissions: {'filesystem.workspace.read'},
    editors: [
      OpenMuseEditorContribution(
        id: 'viewer.markdown',
        extensions: {'md', 'markdown', 'mdown'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.image',
        extensions: {'png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.pdf',
        extensions: {'pdf'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.fallback',
        extensions: {},
        priority: 0,
        catchAll: true,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file')) {
      return const _ViewerMessage('Viewer 仅接受 Host 授权后的本地文件句柄。');
    }
    final path = resource.uri.toFilePath();
    if (descriptor.editors.first.accepts(resource)) {
      return OpenMuseMarkdownPreview(key: ValueKey(path), path: path);
    }
    if (!descriptor.editors
        .skip(1)
        .take(2)
        .any((editor) => editor.accepts(resource))) {
      return const _ViewerMessage('此文件类型尚无可用预览器。可安装对应格式的插件。');
    }
    if (Platform.isMacOS) {
      return AppKitView(
        key: ValueKey(path),
        viewType: 'com.openmuse.viewer',
        creationParams: <String, Object?>{'path': path},
        creationParamsCodec: const StandardMessageCodec(),
      );
    }
    final extension = p.extension(path).toLowerCase();
    if (const {'.png', '.jpg', '.jpeg', '.gif', '.webp'}.contains(extension)) {
      return _LocalImagePreview(key: ValueKey(path), path: path);
    }
    if (extension == '.svg') {
      return _LocalTextPreview(key: ValueKey(path), path: path, title: 'SVG');
    }
    if (extension == '.pdf') {
      return _LocalPdfPreview(key: ValueKey(path), path: path);
    }
    return const _ViewerMessage('此文件类型尚无可用预览器。可安装对应格式的插件。');
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class OpenMuseMarkdownPreview extends StatefulWidget {
  const OpenMuseMarkdownPreview({super.key, required this.path, this.contents});
  final String path;
  @visibleForTesting
  final Future<String>? contents;

  @override
  State<OpenMuseMarkdownPreview> createState() => _MarkdownPreviewState();
}

final class _MarkdownPreviewState extends State<OpenMuseMarkdownPreview> {
  late Future<String> _contents =
      widget.contents ?? loadOpenMuseMarkdownFile(widget.path);

  @override
  Widget build(BuildContext context) => FutureBuilder<String>(
    future: _contents,
    builder: (context, snapshot) {
      if (snapshot.hasError)
        return _ViewerMessage('无法读取 Markdown：${snapshot.error}');
      if (!snapshot.hasData)
        return const Center(child: CircularProgressIndicator());
      return Markdown(
        data: snapshot.data!,
        selectable: true,
        padding: const EdgeInsets.fromLTRB(24, 20, 24, 32),
        // A local document must not trigger remote image fetches or silently
        // open links outside the Host's resource router.
        imageBuilder: (_, _, alt) => Text(alt ?? '[图片]'),
        onTapLink: (_, _, _) {},
      );
    },
  );
}

Future<String> loadOpenMuseMarkdownFile(String path) async {
  final file = File(path);
  if (await file.length() > 16 * 1024 * 1024) {
    throw const FileSystemException('Markdown 文件超过 16 MB 预览上限');
  }
  return file.readAsString();
}

final class _LocalImagePreview extends StatelessWidget {
  const _LocalImagePreview({super.key, required this.path});
  final String path;

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xfff1f2f5),
    child: InteractiveViewer(
      minScale: 0.2,
      maxScale: 8,
      child: Center(
        child: Image.file(
          File(path),
          errorBuilder: (_, error, _) => _ViewerMessage('无法读取图片：$error'),
        ),
      ),
    ),
  );
}

final class _LocalTextPreview extends StatelessWidget {
  const _LocalTextPreview({super.key, required this.path, required this.title});
  final String path;
  final String title;

  @override
  Widget build(BuildContext context) => FutureBuilder<String>(
    future: loadOpenMuseMarkdownFile(path),
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return _ViewerMessage('无法读取 $title：${snapshot.error}');
      }
      if (!snapshot.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      return ColoredBox(
        color: const Color(0xfff1f2f5),
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: SelectableText(snapshot.data!),
        ),
      );
    },
  );
}

final class _LocalPdfPreview extends StatelessWidget {
  const _LocalPdfPreview({super.key, required this.path});
  final String path;

  @override
  Widget build(BuildContext context) => FutureBuilder<Uint8List>(
    future: _readBoundedFile(path),
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return _ViewerMessage('无法读取 PDF：${snapshot.error}');
      }
      if (!snapshot.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      final bytes = snapshot.data!;
      final pages = _pdfPageCount(bytes);
      return ColoredBox(
        color: const Color(0xfff1f2f5),
        child: Center(
          child: Text(
            '${p.basename(path)}\n${bytes.length} 字节'
            '${pages == null ? '' : '\n$pages 页'}',
            textAlign: TextAlign.center,
          ),
        ),
      );
    },
  );
}

Future<Uint8List> _readBoundedFile(String path) async {
  final file = File(path);
  if (await file.length() > 32 * 1024 * 1024) {
    throw const FileSystemException('PDF 超过 32 MB 预览上限');
  }
  return file.readAsBytes();
}

int? _pdfPageCount(Uint8List bytes) {
  final text = latin1.decode(bytes, allowInvalid: true);
  final matches = RegExp(r'/Type\s*/Page(?!s)').allMatches(text);
  if (matches.isEmpty) return null;
  return matches.length;
}

final class _ViewerMessage extends StatelessWidget {
  const _ViewerMessage(this.message);
  final String message;

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xfff1f2f5),
    child: Center(child: Text(message)),
  );
}
