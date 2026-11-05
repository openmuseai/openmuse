import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/resource_surface/engines/ioffice.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;
import 'package:word_editor/word_editor.dart';

class IofficeWordResourceSurface extends StatefulWidget {
  const IofficeWordResourceSurface({super.key, required this.file});
  final File file;

  @override
  State<IofficeWordResourceSurface> createState() =>
      _IofficeWordResourceSurfaceState();
}

class _IofficeWordResourceSurfaceState
    extends State<IofficeWordResourceSurface> {
  late final WordEditorController _controller;
  String? _error;
  var _loading = true;

  @override
  void initState() {
    super.initState();
    _controller = WordEditorController();
    _open();
  }

  Future<void> _open() async {
    final extension =
        p.extension(widget.file.path).toLowerCase().replaceFirst('.', '');
    if (!iofficeSupportedExtensions.contains(extension)) {
      _finish('iOffice Word accepts .docx resources in this release.');
      return;
    }
    try {
      final bytes = Uint8List.fromList(await widget.file.readAsBytes());
      WordBlobStore.validateDocx(bytes);
      await _controller.init();
      if (!_controller.engine.ffiReady) {
        _finish(
          'iOffice Word runtime unavailable: ${_controller.engine.source}',
        );
        return;
      }
      await _controller.openBytes(bytes, name: p.basename(widget.file.path));
      _finish(_controller.envelope == null ? _controller.status : null);
    } on Object catch (error) {
      _finish('Unable to render Word document: $error');
    }
  }

  void _finish(String? error) {
    if (!mounted) return;
    setState(() {
      _loading = false;
      _error = error;
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_loading) return const Center(child: CircularProgressIndicator());
    if (_error != null) return _Message(text: _error!);
    return Column(
      children: [
        Container(
          height: 34,
          alignment: Alignment.centerLeft,
          padding: const EdgeInsets.symmetric(horizontal: 12),
          color: const Color(0xFF185ABD),
          child: const Text(
            'iOffice Word · read-only resource session',
            style: TextStyle(color: Colors.white, fontSize: 12),
          ),
        ),
        Expanded(
          child: WordWorkspace(
            controller: _controller,
            showChrome: false,
            readOnly: true,
          ),
        ),
      ],
    );
  }
}

class _Message extends StatelessWidget {
  const _Message({required this.text});
  final String text;
  @override
  Widget build(BuildContext context) => Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Text(text, textAlign: TextAlign.center),
        ),
      );
}
