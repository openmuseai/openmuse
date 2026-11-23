library;

import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;
import 'package:flutter/services.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

final class OpenMuseFileViewerPlugin
    implements
        OpenMusePlugin,
        OpenMuseEditorBannerContributor,
        OpenMuseResourceHandlerContributor {
  OpenMusePluginContext? _host;
  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.open-file-viewer',
    name: 'Open File Viewer',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: [
      'onStartup',
      'onFileType:markdown',
      'onFileType:text',
      'onFileType:image',
      'onFileType:pdf',
    ],
    permissions: {'filesystem.workspace.read'},
    editors: [
      OpenMuseEditorContribution(
        id: 'viewer.markdown',
        mediaTypes: {'text/markdown'},
        extensions: {
          'omreview','md', 'markdown', 'mdown'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.text',
        mediaTypes: {
          'application/vnd.openmuse.review-viewer+json',
          'text/*',
          'application/json',
          'application/xml',
          'application/javascript',
          'application/x-javascript',
          'application/x-yaml',
          'application/toml',
        },
        extensions: {
          'txt',
          'log',
          'json',
          'jsonl',
          'yaml',
          'yml',
          'toml',
          'xml',
          'csv',
          'tsv',
          'ini',
          'cfg',
          'conf',
          'properties',
          'env',
          'dart',
          'py',
          'pyi',
          'js',
          'jsx',
          'mjs',
          'cjs',
          'ts',
          'tsx',
          'html',
          'htm',
          'css',
          'scss',
          'sass',
          'less',
          'vue',
          'svelte',
          'java',
          'kt',
          'kts',
          'swift',
          'm',
          'mm',
          'h',
          'hpp',
          'c',
          'cc',
          'cpp',
          'cs',
          'rs',
          'go',
          'rb',
          'php',
          'sh',
          'bash',
          'zsh',
          'fish',
          'ps1',
          'bat',
          'sql',
          'graphql',
          'proto',
          'gradle',
          'cmake',
          'make',
          'dockerfile',
          'gitignore',
          'lock',
        },
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.image',
        mediaTypes: {'image/*'},
        extensions: {'png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.pdf',
        mediaTypes: {'application/pdf'},
        extensions: {'pdf'},
        priority: 20,
      ),
      OpenMuseEditorContribution(
        id: 'viewer.video',
        mediaTypes: {'video/mp4', 'video/webm', 'video/quicktime'},
        extensions: {'mp4', 'm4v', 'mov', 'webm'},
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
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'viewer.preview',
      title: 'Open File Viewer',
      mediaTypes: {
        ...descriptor.editors[0].mediaTypes,
        ...descriptor.editors[1].mediaTypes,
        'image/png',
        'image/jpeg',
        'image/gif',
        'image/webp',
      },
      extensions: {
        ...descriptor.editors[0].extensions,
        ...descriptor.editors[1].extensions,
        'png',
        'jpg',
        'jpeg',
        'gif',
        'webp',
      },
      // A view intent defaults to the native read-only preview. Explicit user
      // handler preferences still win, and edit intents are declined below.
      priority: 60,
    ),
  ];
  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async =>
      intent == ResourceOpenIntent.view &&
      resourceHandlers.single.accepts(record);
  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  ) => OpenMuseFileViewerBody(
    key: ValueKey((session.record.identity, session.record.version)),
    resourceName: session.record.name,
    mediaType: session.record.mediaType,
    initialLine: session.initialLine,
    navigateCode: session.navigateCode == null
        ? null
        : (operation, line, character) => session.navigateCode!(
            ResourceCodeRequest(
              operation,
              version: session.record.version,
              line: line,
              character: character,
            ),
          ),
    openCodeTarget: session.openCodeTarget,
    onOpenLink: (target) async {
      await session.openResourceLink?.call(target);
    },
    readBytes: () async => Uint8List.fromList(session.bytes),
  );

  bool canPreviewText(OpenMuseResource resource) =>
      descriptor.editors[0].accepts(resource) ||
      descriptor.editors[0].extensions.contains(resource.extension) ||
      descriptor.editors[1].accepts(resource) ||
      descriptor.editors[1].extensions.contains(resource.extension) ||
      {
        'makefile',
        'dockerfile',
        'license',
        'notice',
      }.contains(p.basename(resource.uri.path).toLowerCase());

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _host = context;
  }

  @override
  Future<void> deactivate() async {
    _host = null;
  }

  Future<void> _openLocalLink(
    BuildContext context,
    OpenMuseResource base,
    String target,
  ) async {
    try {
      if (target.startsWith('openmuse-resource://v1/')) {
        await ResourceLinkRouting.maybeOf(context)?.open(target);
        return;
      }
      final uri = Uri.parse(target);
      if ((uri.hasScheme && uri.scheme != 'file') ||
          uri.hasAuthority && uri.host.isNotEmpty && uri.host != 'localhost' ||
          _host == null) {
        throw const FormatException('Unsupported file link');
      }
      // This legacy editor receives a local Host-granted file handle. Its Host
      // command revalidates the workspace; remote resources use session links.
      await _host!.executeHostCommand('workspace.openResource', {
        'path': uri.scheme == 'file' ? uri.toFilePath() : uri.path,
        'cwd': p.dirname(base.uri.toFilePath()),
      });
    } on Object {
      if (context.mounted)
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('无法打开文件链接。')));
    }
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file')) {
      return const _ViewerMessage('Viewer 仅接受 Host 授权后的本地文件句柄。');
    }
    final path = resource.uri.toFilePath();
    if (descriptor.editors.first.accepts(resource) ||
        descriptor.editors.first.extensions.contains(resource.extension)) {
      return OpenMuseMarkdownPreview(
        key: ValueKey(path),
        path: path,
        onOpenLink: (target) => _openLocalLink(context, resource, target),
      );
    }
    if (canPreviewText(resource)) {
      return _LocalTextPreview(key: ValueKey(path), path: path, title: '文本或代码');
    }
    if (!descriptor.editors
        .skip(2)
        .take(3)
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
    final mediaType = resource.mediaType;
    if (const {
          'image/png',
          'image/jpeg',
          'image/gif',
          'image/webp',
        }.contains(mediaType) ||
        (mediaType == null &&
            const {
              '.png',
              '.jpg',
              '.jpeg',
              '.gif',
              '.webp',
            }.contains(extension))) {
      return _LocalImagePreview(key: ValueKey(path), path: path);
    }
    if (mediaType == 'image/svg+xml' ||
        (mediaType == null && extension == '.svg')) {
      return _LocalTextPreview(key: ValueKey(path), path: path, title: 'SVG');
    }
    if (mediaType == 'application/pdf' ||
        (mediaType == null && extension == '.pdf')) {
      return _LocalPdfPreview(key: ValueKey(path), path: path);
    }
    return const _ViewerMessage('此文件类型尚无可用预览器。可安装对应格式的插件。');
  }

  @override
  Widget? buildEditorBanner(
    BuildContext context,
    OpenMuseResource resource, {
    required bool Function(String editorId) canOpenWith,
    required void Function(String editorId) openWith,
  }) => OpenMuseEditorBanner(
    message: '文件预览 · 只读',
    actionLabel:
        resource.mediaType == 'text/markdown' && canOpenWith('helix.editor')
        ? 'Edit'
        : null,
    onAction: resource.mediaType == 'text/markdown'
        ? () => openWith('helix.editor')
        : null,
  );

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class OpenMuseMarkdownPreview extends StatefulWidget {
  const OpenMuseMarkdownPreview({
    super.key,
    required this.path,
    this.contents,
    this.onOpenLink,
  });
  final String path;
  final Future<void> Function(String target)? onOpenLink;
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
      return OpenMuseMarkdownBody(
        contents: snapshot.data!,
        onOpenLink: widget.onOpenLink,
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
      final language = viewerCodeLanguage(path);
      return language == null
          ? OpenMuseTextBody(contents: snapshot.data!)
          : OpenMuseCodeBody(contents: snapshot.data!, language: language);
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
