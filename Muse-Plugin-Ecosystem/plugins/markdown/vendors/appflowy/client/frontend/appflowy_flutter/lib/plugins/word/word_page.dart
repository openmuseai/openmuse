import 'dart:async';

import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:appflowy/plugins/word/word_apply.dart';
import 'package:appflowy/plugins/word/word_backend.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_file_open.dart';
import 'package:appflowy/plugins/word/word_muse_toolbar.dart';
import 'package:appflowy/plugins/word/word_plain_text.dart';
import 'package:appflowy/plugins/word/word_snapshot_cache.dart';
import 'package:appflowy/plugins/word/word_status_page.dart';
import 'package:appflowy_backend/log.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/user_profile.pb.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:muse_word_surface/muse_word_surface.dart';
import 'package:word_editor/word_editor.dart';

/// Hosts the Word canvas inside the Muse plugin shell.
class WordPage extends StatefulWidget {
  const WordPage({
    super.key,
    required this.view,
    required this.user,
    this.onDeleted,
  });

  final ViewPB view;
  final UserProfilePB user;
  final VoidCallback? onDeleted;

  @override
  State<WordPage> createState() => _WordPageState();
}

class _WordPageState extends State<WordPage> {
  late final WordEditorController _controller;
  MuseWordSurfaceBinding? _facet;
  WordBlobStore? _store;
  var _booting = true;
  String? _bootError;
  var _missingBlob = false;

  @override
  void initState() {
    super.initState();
    _controller = WordEditorController();
    _controller.addListener(_onSelection);
    _boot();
  }

  void _onSelection() {
    final binding = _facet;
    if (binding == null) return;
    unawaited(
      binding.publishSelection(
        caretCp: _controller.caretCp,
        selStart: _controller.selStart,
        selEnd: _controller.selEnd,
        pageIndex: _controller.visiblePage,
      ),
    );
  }

  Future<void> _boot() async {
    if (kIsWeb) {
      setState(() {
        _booting = false;
        _bootError = wordWebUnsupportedMessage;
      });
      return;
    }
    _store = await WordBackendService.storeFor(widget.user);
    _facet = MuseWordSurfaceBinding.open(
      viewId: widget.view.id,
      title: widget.view.name,
    );
    await _controller.init();
    if (!mounted) return;
    if (!_controller.engine.ffiReady) {
      await _facet?.publishSurface(
        title: widget.view.name,
        mode: 'error',
        ffiReady: false,
      );
      setState(() {
        _booting = false;
        _bootError =
            'This device does not support Word yet.\n${_controller.engine.source}';
      });
      return;
    }
    final loaded = await _store!.get(widget.view.id);
    Uint8List bytes;
    if (loaded == null) {
      bytes = buildMinimalDocx();
      try {
        await _store!.put(widget.view.id, bytes);
      } catch (_) {
        _missingBlob = true;
      }
    } else {
      bytes = loaded.$1;
    }
    // Let the spinner paint before sync FFI layout blocks the UI isolate.
    await Future<void>.delayed(Duration.zero);
    Log.info('[word] openBytes ${bytes.length} name=${widget.view.name}');
    await _controller.openBytes(bytes, name: '${widget.view.name}.docx');
    Log.info('[word] openBytes done status=${_controller.status}');
    WordSnapshotCache.instance.put(widget.view.id, plainTextFromDocx(bytes));
    await _facet?.publishSurface(
      title: widget.view.name,
      mode: 'edit',
      ffiReady: true,
    );
    if (!mounted) return;
    setState(() => _booting = false);
  }

  Future<void> _openDocument() async {
    final files = await pickLocalDocxFiles(allowMultiple: false);
    if (files.isEmpty) return;
    final file = files.first;
    try {
      WordBlobStore.validateDocx(file.bytes);
    } catch (_) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Not a valid .docx file')),
      );
      return;
    }
    await _store?.put(widget.view.id, file.bytes);
    WordSnapshotCache.instance.put(widget.view.id, plainTextFromDocx(file.bytes));
    Log.info('[word] replaceBytes ${file.bytes.length} name=${file.name}');
    await _controller.openBytes(file.bytes, name: file.name);
  }

  Future<void> _onSave() async {
    refuseStaleDocxSave();
  }

  @override
  void dispose() {
    _controller.removeListener(_onSelection);
    WordSnapshotCache.instance.remove(widget.view.id);
    final closing = _facet?.close();
    if (closing != null) {
      unawaited(closing);
    }
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_booting) {
      return const ColoredBox(
        color: Colors.transparent,
        child: Center(child: CircularProgressIndicator()),
      );
    }
    if (_bootError != null) {
      return WordStatusPage(message: _bootError!);
    }
    if (_missingBlob) {
      return const WordStatusPage(message: wordMissingBlobMessage);
    }
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        WordMuseToolbar(
          controller: _controller,
          onImport: _openDocument,
          onSave: _controller.canExportDocx ? _onSave : null,
          canSave: _controller.canExportDocx,
        ),
        Expanded(
          child: WordWorkspace(
            controller: _controller,
            showChrome: false,
            onOpenDocument: _openDocument,
          ),
        ),
      ],
    );
  }
}
