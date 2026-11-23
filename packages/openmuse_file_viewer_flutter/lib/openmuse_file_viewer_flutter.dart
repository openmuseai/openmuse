import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'src/syntax.dart';
import 'src/navigation.dart';
import 'src/review_view.dart';
import 'src/review_document.dart';
export 'src/syntax.dart'
    show
        viewerCodeLanguage,
        highlightViewerCode,
        ViewerSyntaxRange,
        viewerSyntaxColor;

final class OpenMuseFileViewerBody extends StatefulWidget {
  const OpenMuseFileViewerBody({
    super.key,
    required this.resourceName,
    required this.readBytes,
    this.mediaType,
    this.initialLine,
    this.onOpenLink,
    this.navigateCode,
    this.openCodeTarget,
  });

  final String resourceName;
  final String? mediaType;
  final int? initialLine;
  final Future<void> Function(String target)? onOpenLink;
  final CodeNavigationCallback? navigateCode;
  final Future<bool> Function(ResourceCodeTarget target)? openCodeTarget;
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
      if (widget.mediaType == 'application/vnd.openmuse.review-viewer+json' ||
          widget.resourceName.toLowerCase().endsWith('.omreview')) {
        try {
          final contents = utf8.decode(bytes);
          ViewerReviewDocument(contents);
          return OpenMuseReviewBody(contents: contents);
        } catch (_) {
          return const Center(child: Text('无法读取审阅快照，格式或大小不受支持。'));
        }
      }
      final extension = widget.resourceName.split('.').last.toLowerCase();
      if (const {'md', 'markdown', 'mdown'}.contains(extension)) {
        return OpenMuseMarkdownBody(
          contents: utf8.decode(bytes),
          onOpenLink: widget.onOpenLink,
        );
      }
      final language = viewerCodeLanguage(widget.resourceName);
      if (language != null) {
        return OpenMuseCodeBody(
          contents: utf8.decode(bytes),
          language: language,
          initialLine: widget.initialLine,
          navigateCode: widget.navigateCode,
          openCodeTarget: widget.openCodeTarget,
        );
      }
      if ((widget.mediaType?.startsWith('text/') ?? false) ||
          const {
            'txt',
            'log',
            'json',
            'svg',
            'dart',
            'py',
            'js',
            'ts',
            'tsx',
            'jsx',
            'go',
            'rs',
            'java',
            'kt',
            'swift',
            'c',
            'cpp',
            'h',
            'yaml',
            'yml',
            'toml',
            'xml',
            'sh',
            'sql',
          }.contains(extension) ||
          const {
            'application/json',
            'application/xml',
            'application/javascript',
            'application/x-yaml',
            'application/toml',
          }.contains(widget.mediaType)) {
        return OpenMuseTextBody(
          contents: utf8.decode(bytes),
          initialLine: widget.initialLine,
        );
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
  const OpenMuseMarkdownBody({
    super.key,
    required this.contents,
    this.onOpenLink,
  });
  final String contents;
  final Future<void> Function(String target)? onOpenLink;

  @override
  Widget build(BuildContext context) => Markdown(
    data: contents,
    selectable: true,
    padding: const EdgeInsets.fromLTRB(24, 20, 24, 32),
    imageBuilder: (_, _, alt) => Text(alt ?? '[图片]'),
    onTapLink: (_, href, _) async {
      if (href != null && onOpenLink != null) await onOpenLink!(href);
    },
  );
}

final class OpenMuseCodeBody extends StatefulWidget {
  const OpenMuseCodeBody({
    super.key,
    required this.contents,
    required this.language,
    this.initialLine,
    this.navigateCode,
    this.openCodeTarget,
  });
  final String contents, language;
  final int? initialLine;
  final CodeNavigationCallback? navigateCode;
  final Future<bool> Function(ResourceCodeTarget target)? openCodeTarget;
  @override
  State<OpenMuseCodeBody> createState() => _OpenMuseCodeBodyState();
}

final class _OpenMuseCodeBodyState extends State<OpenMuseCodeBody> {
  late Future<List<ViewerSyntaxRange>?> _highlight = highlightViewerCode(
    widget.contents,
    widget.language,
  );
  late final _scroll = ScrollController(
    initialScrollOffset:
        ((widget.initialLine ?? 1) - 1).clamp(
          0,
          widget.contents.split('\n').length - 1,
        ) *
        22.0,
  );
  @override
  void didUpdateWidget(covariant OpenMuseCodeBody oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.contents != oldWidget.contents ||
        widget.language != oldWidget.language) {
      _highlight = highlightViewerCode(widget.contents, widget.language);
    }
  }

  @override
  void dispose() {
    _scroll.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<List<ViewerSyntaxRange>?>(
    future: _highlight,
    builder: (context, snapshot) {
      final theme = Theme.of(context);
      final lines = viewerSyntaxLines(
        widget.contents,
        snapshot.data ?? [],
        theme.brightness,
      );
      final rawLines = widget.contents.split('\n');
      final scale = MediaQuery.textScalerOf(context).scale(14) / 14;
      final width =
          rawLines.fold<int>(0, (n, s) => s.length > n ? s.length : n) *
              14.0 *
              scale +
          76;
      return Column(
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                if (widget.navigateCode != null)
                  TextButton.icon(
                    onPressed: () => _navigate(0, 0, '代码导航'),
                    icon: const Icon(Icons.code, size: 18),
                    label: const Text('代码导航'),
                  ),
                Text(
                  snapshot.data != null && snapshot.data!.isNotEmpty
                      ? 'Tree-sitter · ${widget.language}'
                      : widget.language,
                  key: ValueKey(
                    snapshot.data != null && snapshot.data!.isNotEmpty
                        ? 'file-viewer-highlight-ready'
                        : 'file-viewer-highlight-pending',
                  ),
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: theme.colorScheme.onSurfaceVariant,
                  ),
                ),
              ],
            ),
          ),
          Expanded(
            child: LayoutBuilder(
              builder: (context, bounds) => SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                child: SizedBox(
                  width: width < bounds.maxWidth ? bounds.maxWidth : width,
                  height: bounds.maxHeight,
                  child: SelectionArea(
                    child: ListView.builder(
                      key: const ValueKey('file-viewer-code-lines'),
                      controller: _scroll,
                      itemExtent: 22 * scale,
                      itemCount: lines.length,
                      itemBuilder: (context, i) => ColoredBox(
                        color: i + 1 == widget.initialLine
                            ? theme.colorScheme.primaryContainer
                            : theme.colorScheme.surface,
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            SizedBox(
                              width: 52,
                              child: Text(
                                '${i + 1}',
                                textAlign: TextAlign.right,
                                style: TextStyle(
                                  color: theme.colorScheme.onSurfaceVariant,
                                  fontSize: 12,
                                ),
                              ),
                            ),
                            const SizedBox(width: 12),
                            Expanded(
                              child: _CodeTapText(
                                spans: lines[i],
                                onTap: widget.navigateCode == null
                                    ? null
                                    : (column) {
                                        final raw = rawLines[i];
                                        if (column >= raw.length ||
                                            raw[column].trim().isEmpty)
                                          return;
                                        var a = column, b = column;
                                        bool part(int index) =>
                                            index >= 0 &&
                                            index < raw.length &&
                                            RegExp(
                                              r'[a-zA-Z0-9_$\u0080-\uffff]',
                                            ).hasMatch(raw[index]);
                                        while (part(a - 1)) {
                                          a--;
                                        }
                                        while (part(b)) {
                                          b++;
                                        }
                                        if (b > a)
                                          _navigate(
                                            i,
                                            column,
                                            raw.substring(a, b),
                                          );
                                      },
                                key: ValueKey('file-viewer-code-line-${i + 1}'),
                                style: TextStyle(
                                  fontFamily: 'monospace',
                                  fontSize: 14,
                                  height: 22 / 14,
                                  color: theme.colorScheme.onSurface,
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      );
    },
  );

  Future<void> _navigate(int line, int character, String symbol) async {
    final callback = widget.navigateCode;
    if (callback == null) return;
    final target = await showModalBottomSheet<ResourceCodeTarget>(
      context: context,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      builder: (_) => CodeNavigationSheet(
        callback: callback,
        line: line,
        character: character,
        symbol: symbol,
      ),
    );
    if (target != null && mounted) await widget.openCodeTarget?.call(target);
  }
}

final class _CodeTapText extends StatefulWidget {
  const _CodeTapText({
    super.key,
    required this.spans,
    required this.style,
    this.onTap,
  });
  final List<TextSpan> spans;
  final TextStyle style;
  final void Function(int)? onTap;
  @override
  State<_CodeTapText> createState() => _CodeTapTextState();
}

final class _CodeTapTextState extends State<_CodeTapText> {
  final _text = GlobalKey();
  Offset? _down;
  Duration? _downTime;
  @override
  Widget build(BuildContext context) => Listener(
    onPointerDown: (event) {
      _down = event.position;
      _downTime = event.timeStamp;
    },
    onPointerMove: (event) {
      if (_down != null && (event.position - _down!).distance > 8) _down = null;
    },
    onPointerCancel: (_) => _down = null,
    onPointerUp: widget.onTap == null
        ? null
        : (event) {
            if (_down == null ||
                _downTime == null ||
                event.timeStamp - _downTime! >
                    const Duration(milliseconds: 350))
              return;
            _down = null;
            final root = _text.currentContext?.findRenderObject();
            RenderParagraph? paragraph;
            void find(RenderObject node) {
              if (node is RenderParagraph) {
                paragraph ??= node;
              } else {
                node.visitChildren(find);
              }
            }

            if (root != null) find(root);
            if (paragraph != null)
              widget.onTap!(
                paragraph!
                    .getPositionForOffset(
                      paragraph!.globalToLocal(event.position),
                    )
                    .offset,
              );
          },
    child: Text.rich(
      TextSpan(children: widget.spans),
      key: _text,
      maxLines: 1,
      softWrap: false,
      style: widget.style,
    ),
  );
}

final class OpenMuseTextBody extends StatefulWidget {
  const OpenMuseTextBody({super.key, required this.contents, this.initialLine});
  final String contents;
  final int? initialLine;
  @override
  State<OpenMuseTextBody> createState() => _OpenMuseTextBodyState();
}

final class _OpenMuseTextBodyState extends State<OpenMuseTextBody> {
  late final lines = widget.contents.split('\n');
  late final scroll = ScrollController(
    initialScrollOffset:
        (((widget.initialLine ?? 1) - 1).clamp(0, lines.length - 1) * 22)
            .toDouble(),
  );
  @override
  void dispose() {
    scroll.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.initialLine == null)
      return ColoredBox(
        color: const Color(0xfff1f2f5),
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: SelectableText(
            widget.contents,
            style: const TextStyle(
              fontFamily: 'monospace',
              color: Color(0xff202124),
            ),
          ),
        ),
      );
    return LayoutBuilder(
      builder: (context, constraints) => SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: SizedBox(
          width:
              lines.fold<int>(0, (n, s) => s.length > n ? s.length : n) * 9.0 +
                      72 <
                  constraints.maxWidth
              ? constraints.maxWidth
              : lines.fold<int>(0, (n, s) => s.length > n ? s.length : n) *
                        9.0 +
                    72,
          height: constraints.maxHeight,
          child: ListView.builder(
            controller: scroll,
            itemExtent: 22,
            itemCount: lines.length,
            itemBuilder: (context, i) => ColoredBox(
              color: i + 1 == widget.initialLine
                  ? Theme.of(context).colorScheme.primaryContainer
                  : Theme.of(context).colorScheme.surface,
              child: Row(
                children: [
                  SizedBox(
                    width: 52,
                    child: Text(
                      '${i + 1}',
                      textAlign: TextAlign.right,
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: SelectableText(
                      lines[i],
                      maxLines: 1,
                      style: const TextStyle(
                        fontFamily: 'monospace',
                        fontSize: 14,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
