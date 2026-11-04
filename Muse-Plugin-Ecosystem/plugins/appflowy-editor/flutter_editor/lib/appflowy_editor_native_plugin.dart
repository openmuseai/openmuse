library;

import 'dart:async';
import 'dart:io';

import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';

/// A Flutter surface that edits Markdown in place. The embedding application
/// grants access by passing a file path; no host protocol or WebView is used.
class AppFlowyMarkdownEditor extends StatefulWidget {
  const AppFlowyMarkdownEditor({
    super.key,
    this.filePath,
    this.initialMarkdown,
  });

  /// Null opens this plugin's own persistent draft in application support.
  final String? filePath;

  /// Optional initial content for a host that already holds the document.
  final String? initialMarkdown;

  @override
  State<AppFlowyMarkdownEditor> createState() => _AppFlowyMarkdownEditorState();
}

class _AppFlowyMarkdownEditorState extends State<AppFlowyMarkdownEditor> {
  EditorState? _editor;
  StreamSubscription<Object?>? _changes;
  Timer? _autosave;
  File? _file;
  String _loadedMarkdown = '';
  String _status = '正在打开…';
  bool _dirty = false;
  bool _saving = false;
  int _generation = 0;

  @override
  void initState() {
    super.initState();
    unawaited(_load());
  }

  @override
  void didUpdateWidget(covariant AppFlowyMarkdownEditor oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.filePath != widget.filePath) {
      _autosave?.cancel();
      unawaited(_load());
    }
  }

  Future<File> _resolveFile() async {
    if (widget.filePath case final path?) return File(path);
    final support = await getApplicationSupportDirectory();
    return File('${support.path}/appflowy-editor/draft.md');
  }

  Future<void> _load() async {
    final generation = ++_generation;
    try {
      final file = await _resolveFile();
      if (widget.initialMarkdown == null &&
          widget.filePath != null &&
          !await file.exists()) {
        throw const FileSystemException('文档不存在');
      }
      final source =
          widget.initialMarkdown ??
          (await file.exists() ? await file.readAsString() : '');
      if (!mounted || generation != _generation) return;
      final editor = source.isEmpty
          ? EditorState.blank(withInitialText: true)
          : EditorState(document: markdownToDocument(source));
      await _changes?.cancel();
      _editor?.dispose();
      _changes = editor.transactionStream.listen((_) {
        if (!mounted) return;
        setState(() {
          _dirty = true;
          _status = '未保存';
        });
        _autosave?.cancel();
        _autosave = Timer(const Duration(milliseconds: 700), () {
          unawaited(_save());
        });
      });
      setState(() {
        _file = file;
        _editor = editor;
        _loadedMarkdown = source;
        _dirty = false;
        _status = '已保存';
      });
    } catch (error) {
      if (mounted && generation == _generation) {
        setState(() => _status = '无法打开：$error');
      }
    }
  }

  Future<void> _save({bool silent = false}) async {
    final editor = _editor;
    final file = _file;
    if (editor == null || file == null || _saving || !_dirty) return;
    _saving = true;
    final generation = _generation;
    final markdown = documentToMarkdown(editor.document);
    if (mounted && !silent) setState(() => _status = '正在保存…');
    try {
      final current = await file.exists() ? await file.readAsString() : '';
      if (current != _loadedMarkdown) {
        throw const FileSystemException('文件已在其他位置修改，请重新打开后再编辑');
      }
      await file.parent.create(recursive: true);
      await file.writeAsString(markdown, flush: true);
      if (generation != _generation) return;
      _loadedMarkdown = markdown;
      final stillDirty = documentToMarkdown(editor.document) != markdown;
      if (mounted && !silent) {
        setState(() {
          _dirty = stillDirty;
          _status = stillDirty ? '未保存' : '已保存';
        });
      }
      if (stillDirty) {
        _autosave?.cancel();
        _autosave = Timer(const Duration(milliseconds: 700), () {
          unawaited(_save());
        });
      }
    } catch (error) {
      if (mounted && !silent && generation == _generation) {
        setState(() => _status = '保存失败：$error');
      }
    } finally {
      _saving = false;
    }
  }

  @override
  void dispose() {
    _autosave?.cancel();
    if (_dirty) unawaited(_save(silent: true));
    _generation++;
    unawaited(_changes?.cancel());
    _editor?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final editor = _editor;
    return Theme(
      data: ThemeData.light(useMaterial3: true),
      child: ColoredBox(
        color: const Color(0xFFF7F6F3),
        child: CallbackShortcuts(
          bindings: {
            const SingleActivator(LogicalKeyboardKey.keyS, meta: true): () {
              unawaited(_save());
            },
          },
          child: Focus(
            autofocus: true,
            child: Column(
              children: [
                SizedBox(
                  height: 44,
                  child: Row(
                    children: [
                      const SizedBox(width: 18),
                      const Icon(Icons.edit_note_outlined, size: 18),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          widget.filePath == null
                              ? 'AppFlowy Editor · 草稿'
                              : _file?.uri.pathSegments.last ?? 'AppFlowy Editor',
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      Text(_status, style: const TextStyle(fontSize: 12)),
                      const SizedBox(width: 8),
                      TextButton.icon(
                        onPressed: _dirty ? () => unawaited(_save()) : null,
                        icon: const Icon(Icons.save_outlined, size: 16),
                        label: const Text('保存'),
                      ),
                      const SizedBox(width: 12),
                    ],
                  ),
                ),
                const Divider(height: 1),
                Expanded(
                  child: editor == null
                      ? Center(child: Text(_status))
                      : Center(
                          child: ConstrainedBox(
                            constraints: const BoxConstraints(maxWidth: 900),
                            child: Padding(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 32,
                                vertical: 24,
                              ),
                              child: AppFlowyEditor(editorState: editor),
                            ),
                          ),
                        ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
