import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';

final class OpenMuseFileViewerBody extends StatefulWidget {
  const OpenMuseFileViewerBody({
    super.key,
    required this.resourceName,
    required this.readBytes,
  });

  final String resourceName;
  final Future<Uint8List> Function() readBytes;

  @override
  State<OpenMuseFileViewerBody> createState() => _OpenMuseFileViewerBodyState();
}

final class _OpenMuseFileViewerBodyState extends State<OpenMuseFileViewerBody> {
  late Future<Uint8List> _bytes = widget.readBytes();

  @override
  void didUpdateWidget(covariant OpenMuseFileViewerBody oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.resourceName != widget.resourceName) {
      _bytes = widget.readBytes();
    }
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<Uint8List>(
    future: _bytes,
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return const Center(child: Text('无法读取文件预览。'));
      }
      if (!snapshot.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      final bytes = snapshot.data!;
      final extension = widget.resourceName.split('.').last.toLowerCase();
      if (const {'md', 'markdown', 'mdown'}.contains(extension)) {
        return OpenMuseMarkdownBody(contents: utf8.decode(bytes));
      }
      if (const {'txt', 'log', 'json', 'svg'}.contains(extension)) {
        return OpenMuseTextBody(contents: utf8.decode(bytes));
      }
      if (const {'png', 'jpg', 'jpeg', 'gif', 'webp'}.contains(extension)) {
        return ColoredBox(
          color: const Color(0xfff1f2f5),
          child: InteractiveViewer(
            minScale: 0.2,
            maxScale: 8,
            child: Center(
              child: Image.memory(
                bytes,
                errorBuilder: (_, _, _) =>
                    const Center(child: Text('无法读取图片预览。')),
              ),
            ),
          ),
        );
      }
      return const Center(child: Text('此文件类型尚无可用预览器。可安装对应格式的插件。'));
    },
  );
}

final class OpenMuseMarkdownBody extends StatelessWidget {
  const OpenMuseMarkdownBody({super.key, required this.contents});
  final String contents;

  @override
  Widget build(BuildContext context) => Markdown(
    data: contents,
    selectable: true,
    padding: const EdgeInsets.fromLTRB(24, 20, 24, 32),
    imageBuilder: (_, _, alt) => Text(alt ?? '[图片]'),
    onTapLink: (_, _, _) {},
  );
}

final class OpenMuseTextBody extends StatelessWidget {
  const OpenMuseTextBody({super.key, required this.contents});
  final String contents;

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xfff1f2f5),
    child: SingleChildScrollView(
      padding: const EdgeInsets.all(24),
      child: SelectableText(contents),
    ),
  );
}
