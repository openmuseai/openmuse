import 'dart:convert';
import 'package:flutter/material.dart';
import '../openmuse_file_viewer_flutter.dart';
import 'syntax.dart';
import 'review_document.dart';

/// Shared Open File Viewer workbench: snapshot diff and full-source browsing.
final class OpenMuseReviewBody extends StatefulWidget {
  const OpenMuseReviewBody({super.key, required this.contents});
  final String contents;
  @override
  State<OpenMuseReviewBody> createState() => _OpenMuseReviewBodyState();
}

final class _OpenMuseReviewBodyState extends State<OpenMuseReviewBody> {
  late final document = ViewerReviewDocument(widget.contents);
  final collapsed = <String>{}, expanded = <String>{};
  final scroll = ScrollController();
  int index = 0, initialLine = 1, changeIndex = 0;
  String mode = 'diff';
  bool split = false, treeVisible = true;
  List<List<TextSpan>>? oldColors, newColors;
  Map<String, dynamic> get file => document.files[index];
  @override
  void initState() {
    super.initState();
    if (document.files.isNotEmpty) _highlight();
  }

  @override
  void dispose() {
    scroll.dispose();
    super.dispose();
  }

  Future<void> _highlight() async {
    final selected = index, f = file;
    final lang = viewerCodeLanguage(f['path']) ?? 'text';
    final before = f['before'] as String?, after = f['after'] as String?;
    final tokens = await Future.wait([
      before == null
          ? Future.value(<ViewerSyntaxRange>[])
          : highlightViewerCode(before, lang),
      after == null
          ? Future.value(<ViewerSyntaxRange>[])
          : highlightViewerCode(after, lang),
    ]);
    if (!mounted || selected != index) return;
    setState(() {
      final brightness = Theme.of(context).brightness;
      oldColors = before == null
          ? null
          : viewerSyntaxLines(before, tokens[0] ?? [], brightness);
      newColors = after == null
          ? null
          : viewerSyntaxLines(after, tokens[1] ?? [], brightness);
    });
  }

  void _select(int selected) {
    setState(() {
      index = selected;
      mode = 'diff';
      changeIndex = 0;
      expanded.clear();
      oldColors = null;
      newColors = null;
    });
    if (scroll.hasClients) scroll.jumpTo(0);
    _highlight();
  }

  Widget _tree({bool drawer = false}) => ListView(
    children: [
      Padding(
        padding: const EdgeInsets.all(10),
        child: Text(
          '变更文件 · ${document.files.length}',
          style: const TextStyle(fontWeight: FontWeight.w600),
        ),
      ),
      for (final row in reviewTree(document.files, collapsed))
        SizedBox(
          height:
              drawer ||
                  {
                    TargetPlatform.android,
                    TargetPlatform.iOS,
                  }.contains(Theme.of(context).platform)
              ? 44
              : 32,
          child: InkWell(
            key: ValueKey('review-tree-${row.path}'),
            onTap: () {
              if (row.directory) {
                setState(
                  () => collapsed.contains(row.path)
                      ? collapsed.remove(row.path)
                      : collapsed.add(row.path),
                );
              } else {
                _select(
                  document.files.indexWhere((f) => f['path'] == row.path),
                );
                if (drawer) Navigator.pop(context);
              }
            },
            child: ColoredBox(
              color: !row.directory && file['path'] == row.path
                  ? Theme.of(context).colorScheme.primaryContainer
                  : Colors.transparent,
              child: Padding(
                padding: EdgeInsets.only(
                  left: 8 + row.depth.clamp(0, 8) * 12,
                  right: 8,
                ),
                child: Row(
                  children: [
                    Icon(
                      row.directory
                          ? (collapsed.contains(row.path)
                                ? Icons.chevron_right
                                : Icons.expand_more)
                          : Icons.description_outlined,
                      size: 15,
                    ),
                    const SizedBox(width: 5),
                    Expanded(
                      child: Text(
                        row.label,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: const TextStyle(fontSize: 13),
                      ),
                    ),
                    if (!row.directory)
                      Text(
                        '${document.files.firstWhere((f) => f['path'] == row.path)['status']}'
                            .substring(0, 1)
                            .toUpperCase(),
                        style: TextStyle(
                          fontSize: 11,
                          color: Theme.of(context).colorScheme.primary,
                        ),
                      ),
                  ],
                ),
              ),
            ),
          ),
        ),
    ],
  );
  @override
  Widget build(BuildContext context) {
    if (document.files.isEmpty) return const Center(child: Text('没有变更文件'));
    return LayoutBuilder(
      builder: (context, bounds) {
        final wide = bounds.maxWidth >= 768;
        return Row(
          children: [
            if (wide && treeVisible) ...[
              SizedBox(width: 256, child: _tree()),
              const VerticalDivider(width: 1),
            ],
            Expanded(
              child: Column(
                children: [
                  Material(
                    color: Theme.of(context).colorScheme.surfaceContainerLow,
                    child: SizedBox(
                      height: 44,
                      child: Row(
                        children: [
                          IconButton(
                            key: const ValueKey('review-sidebar-toggle'),
                            tooltip: wide && treeVisible ? '收起文件树' : '展开文件树',
                            icon: const Icon(
                              Icons.view_sidebar_outlined,
                              size: 18,
                            ),
                            onPressed: () {
                              if (wide) {
                                setState(() => treeVisible = !treeVisible);
                              } else {
                                showModalBottomSheet<void>(
                                  context: context,
                                  isScrollControlled: true,
                                  builder: (_) => SizedBox(
                                    height: bounds.maxHeight * .8,
                                    child: _tree(drawer: true),
                                  ),
                                );
                              }
                            },
                          ),
                          Expanded(
                            child: Text(
                              file['path'],
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                              style: const TextStyle(
                                fontSize: 13,
                                fontWeight: FontWeight.w500,
                              ),
                            ),
                          ),
                          Text(
                            '+${file['additions']} −${file['deletions']}',
                            style: const TextStyle(fontSize: 12),
                          ),
                          PopupMenuButton<String>(
                            key: const ValueKey('review-file-menu'),
                            tooltip: '当前文件操作',
                            icon: const Icon(Icons.more_horiz, size: 20),
                            onSelected: (value) {
                              if (value == 'previous') {
                                _jumpChange(-1);
                                return;
                              }
                              if (value == 'next') {
                                _jumpChange(1);
                                return;
                              }
                              if (value == 'jump') {
                                _jump();
                                return;
                              }
                              if (value == 'search') {
                                _search();
                                return;
                              }
                              setState(() {
                                if (value == 'unified' || value == 'split') {
                                  mode = 'diff';
                                  split = value == 'split';
                                } else {
                                  mode = value;
                                  initialLine = 1;
                                }
                              });
                            },
                            itemBuilder: (_) => [
                              for (final item in [
                                ('unified', '统一对比'),
                                ('split', '并排对比'),
                                ('before', '原文件'),
                                ('after', '新文件'),
                                ('previous', '上一处变更'),
                                ('next', '下一处变更'),
                                ('jump', '跳转行'),
                                ('search', '查找代码'),
                              ])
                                PopupMenuItem(
                                  value: item.$1,
                                  child: Text(item.$2),
                                ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                  if (document.incomplete)
                    const Padding(
                      padding: EdgeInsets.all(6),
                      child: Text(
                        '来源不完整，仅展示已取得的内容；不能作为完整审阅通过。',
                        style: TextStyle(fontSize: 12, color: Colors.orange),
                      ),
                    ),
                  Expanded(
                    child: file['binary'] == true
                        ? _binary()
                        : mode == 'diff'
                        ? _diff()
                        : _full(),
                  ),
                ],
              ),
            ),
          ],
        );
      },
    );
  }

  Widget _full() {
    final text = file[mode] as String?;
    if (text == null) return const Center(child: Text('此历史来源没有完整文件；请返回变更视图。'));
    return OpenMuseCodeBody(
      key: ValueKey('$index:$mode:$initialLine'),
      contents: text,
      language: viewerCodeLanguage(file['path']) ?? 'text',
      initialLine: initialLine,
    );
  }

  List<Map<String, dynamic>> _rows() {
    final rows = <Map<String, dynamic>>[];
    var oldEnd = 1, newEnd = 1;
    for (final h in file['hunks'] as List) {
      _gap(rows, oldEnd, h['oldStart'] as int, newEnd, h['newStart'] as int);
      rows.addAll(
        (h['lines'] as List).map((l) => Map<String, dynamic>.from(l)),
      );
      oldEnd = (h['oldStart'] as int) + (h['oldLines'] as int);
      newEnd = (h['newStart'] as int) + (h['newLines'] as int);
    }
    if (file['before'] is String && file['after'] is String)
      _gap(
        rows,
        oldEnd,
        (file['before'] as String).split('\n').length + 1,
        newEnd,
        (file['after'] as String).split('\n').length + 1,
      );
    return rows;
  }

  void _gap(List<Map<String, dynamic>> rows, int a, int endA, int b, int endB) {
    final count = endA - a;
    if (a < 1 || b < 1) return;
    if (count <= 0 || count != endB - b) return;
    final before = (file['before'] as String?)?.split('\n'),
        after = (file['after'] as String?)?.split('\n');
    final key = '$a:$b';
    if (before == null ||
        after == null ||
        endA - 1 > before.length ||
        endB - 1 > after.length ||
        List.generate(
          count,
          (i) => before[a - 1 + i] == after[b - 1 + i],
        ).contains(false))
      return;
    rows.add({
      'kind': 'fold',
      'text': '${expanded.contains(key) ? '折叠' : '展开'} $count 行未变更内容',
      'key': key,
    });
    if (expanded.contains(key))
      for (var i = 0; i < count; i++)
        rows.add({
          'kind': 'context',
          'text': before[a - 1 + i],
          'oldLine': a + i,
          'newLine': b + i,
        });
  }

  Widget _diff() {
    final raw = _rows(), rows = <Map<String, dynamic>>[];
    for (var i = 0; i < raw.length; i++) {
      if (split && raw[i]['kind'] == 'delete') {
        final left = <Map<String, dynamic>>[], right = <Map<String, dynamic>>[];
        while (i < raw.length && raw[i]['kind'] == 'delete') {
          left.add(raw[i++]);
        }
        while (i < raw.length && raw[i]['kind'] == 'add') {
          right.add(raw[i++]);
        }
        for (
          var n = 0;
          n < (left.length > right.length ? left.length : right.length);
          n++
        )
          rows.add({
            'pair': true,
            'left': n < left.length ? left[n] : null,
            'right': n < right.length ? right[n] : null,
          });
        i--;
      } else {
        rows.add(raw[i]);
      }
    }
    if (rows.isEmpty) return const Center(child: Text('没有文本变更'));
    return ListView.builder(
      key: const ValueKey('review-diff-lines'),
      controller: scroll,
      itemCount: rows.length,
      itemBuilder: (context, i) {
        final row = rows[i];
        if (row['kind'] == 'fold')
          return SizedBox(
            height: 36,
            child: TextButton(
              key: ValueKey('review-fold-${row['key']}'),
              onPressed: () => setState(
                () => expanded.contains(row['key'])
                    ? expanded.remove(row['key'])
                    : expanded.add(row['key']),
              ),
              child: Text(row['text'], style: const TextStyle(fontSize: 12)),
            ),
          );
        if (row['pair'] == true)
          return IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Expanded(child: _line(row['left'], false)),
                const VerticalDivider(width: 1),
                Expanded(child: _line(row['right'], true)),
              ],
            ),
          );
        if (split)
          return IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Expanded(
                  child: _line(row['kind'] == 'add' ? null : row, false),
                ),
                const VerticalDivider(width: 1),
                Expanded(
                  child: _line(row['kind'] == 'delete' ? null : row, true),
                ),
              ],
            ),
          );
        return _line(row, row['kind'] != 'delete');
      },
    );
  }

  Widget _line(Map<String, dynamic>? row, bool right) {
    if (row == null) return const SizedBox(height: 24);
    final dark = Theme.of(context).brightness == Brightness.dark,
        added = row['kind'] == 'add',
        removed = row['kind'] == 'delete';
    final line = (right ? row['newLine'] : row['oldLine']) as int?;
    final palette = right ? newColors : oldColors;
    final spans =
        line != null && palette != null && line > 0 && line <= palette.length
        ? palette[line - 1]
        : <TextSpan>[TextSpan(text: row['text'])];
    return ColoredBox(
      color: added
          ? (dark ? const Color(0xff193a2c) : const Color(0xffe8f5e9))
          : removed
          ? (dark ? const Color(0xff42262b) : const Color(0xffffebee))
          : Theme.of(context).colorScheme.surface,
      child: InkWell(
        onTap: line == null
            ? null
            : () => setState(() {
                initialLine = line;
                mode = right ? 'after' : 'before';
              }),
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 3),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                width: 42,
                child: Text(
                  '${line ?? ''}',
                  textAlign: TextAlign.right,
                  style: TextStyle(
                    fontSize: 11,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
              ),
              SizedBox(
                width: 20,
                child: Text(
                  added
                      ? '+'
                      : removed
                      ? '−'
                      : ' ',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    color: added
                        ? Colors.green
                        : removed
                        ? Colors.red
                        : null,
                  ),
                ),
              ),
              Expanded(
                child: Text.rich(
                  TextSpan(children: spans),
                  style: const TextStyle(
                    fontFamily: 'monospace',
                    fontSize: 13,
                    height: 1.5,
                  ),
                ),
              ),
              const SizedBox(width: 6),
            ],
          ),
        ),
      ),
    );
  }

  void _jumpChange(int direction) {
    final hunks = file['hunks'] as List;
    if (hunks.isEmpty) return;
    changeIndex = (changeIndex + direction) % hunks.length;
    final h = hunks[changeIndex];
    setState(() {
      mode = file['after'] is String ? 'after' : 'before';
      initialLine = (h[mode == 'after' ? 'newStart' : 'oldStart'] as int).clamp(
        1,
        1000000,
      );
    });
  }

  Future<String?> _input(String title) async {
    final controller = TextEditingController();
    final result = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(title),
        content: TextField(
          controller: controller,
          autofocus: true,
          onSubmitted: (v) => Navigator.pop(context, v),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('取消'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, controller.text),
            child: const Text('前往'),
          ),
        ],
      ),
    );
    // Dialog text fields are disposed after their route transition completes.
    Future<void>.delayed(const Duration(seconds: 1), controller.dispose);
    return result;
  }

  Future<void> _jump() async {
    final input = await _input('跳转到行');
    final n = int.tryParse(input ?? '');
    if (!mounted || n == null || n < 1) return;
    final side = mode == 'before' ? 'before' : 'after',
        text = file[mode == 'before' ? 'before' : 'after'] as String?;
    if (text == null || n > text.split('\n').length) return;
    setState(() {
      mode = side;
      initialLine = n;
    });
  }

  Future<void> _search() async {
    final query = await _input('查找代码 · 当前快照全文');
    if (!mounted || query == null || query.isEmpty) return;
    final matches = <({int file, int line, String text})>[];
    for (var f = 0; f < document.files.length; f++) {
      final text = document.files[f]['after'] as String?;
      if (text == null) continue;
      final lines = text.split('\n');
      for (var l = 0; l < lines.length && matches.length < 100; l++)
        if (lines[l].contains(query))
          matches.add((file: f, line: l + 1, text: lines[l]));
    }
    await showModalBottomSheet<void>(
      context: context,
      isScrollControlled: true,
      builder: (context) => SizedBox(
        height: 420,
        child: ListView(
          children: [
            const ListTile(
              title: Text('快照中的文本匹配'),
              subtitle: Text('固定版本的精确文本查找；不是当前工作区的 LS 语义结果。'),
            ),
            if (matches.isEmpty) const ListTile(title: Text('没有匹配内容')),
            for (final m in matches)
              ListTile(
                dense: true,
                title: Text(
                  '${document.files[m.file]['path']}:${m.line}',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                subtitle: Text(
                  m.text,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                onTap: () {
                  Navigator.pop(context);
                  _select(m.file);
                  setState(() {
                    mode = 'after';
                    initialLine = m.line;
                  });
                },
              ),
          ],
        ),
      ),
    );
  }

  Widget _binary() => ListView(
    children: [
      for (final side in ['before', 'after'])
        if (file['${side}Info'] is Map)
          Padding(
            padding: const EdgeInsets.all(12),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(side == 'before' ? '原文件' : '新文件'),
                Text(
                  '${file['${side}Info']['byteLength']} 字节',
                  style: const TextStyle(fontSize: 12),
                ),
                if (file['${side}Info']['base64'] is String &&
                    RegExp(
                      r'\.(png|jpe?g|webp)$',
                      caseSensitive: false,
                    ).hasMatch(file['path']))
                  Image.memory(
                    base64Decode(file['${side}Info']['base64']),
                    errorBuilder: (_, _, _) => const Text('图片不可用'),
                  ),
              ],
            ),
          ),
    ],
  );
}
