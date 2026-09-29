import 'package:flutter/material.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class DocxEditorScreen extends StatefulWidget {
  const DocxEditorScreen({
    super.key,
    required this.title,
    required this.handle,
    required this.engine,
    required this.ranges,
    required this.commits,
    required this.generation,
    required this.nowMs,
  });

  final String title;
  final ResourceHandle handle;
  final OfficeEnginePort engine;
  final ResourceRangePort ranges;
  final OfficeResourceCommitPort commits;
  final int generation;
  final int Function() nowMs;

  @override
  State<DocxEditorScreen> createState() => _DocxEditorScreenState();
}

final class _DocxEditorScreenState extends State<DocxEditorScreen> {
  late final OfficeResourceTransaction transaction;
  late final Future<OfficeDocumentSession> opening;
  OfficeDocumentSession? session;
  List<TextEditingController>? controllers;
  OfficeResourceCommitReceipt? receipt;
  Object? saveError;
  bool saving = false;
  int saveSequence = 0;

  @override
  void initState() {
    super.initState();
    transaction = OfficeResourceTransaction(
      engine: widget.engine,
      ranges: widget.ranges,
      commits: widget.commits,
    );
    opening = transaction.open(
      widget.handle,
      generation: widget.generation,
      nowMs: widget.nowMs(),
    );
  }

  @override
  void dispose() {
    for (final controller in controllers ?? const <TextEditingController>[]) {
      controller.dispose();
    }
    super.dispose();
  }

  void _bind(OfficeDocumentSession value) {
    if (session != null) return;
    session = value;
    controllers = value.inspection.paragraphs
        .map((paragraph) => TextEditingController(text: paragraph))
        .toList(growable: false);
  }

  Future<void> _save() async {
    final current = session;
    final editors = controllers;
    if (current == null || editors == null || saving) return;
    setState(() {
      saving = true;
      saveError = null;
    });
    try {
      final value = await transaction.saveSimple(
        session: current,
        paragraphs: editors.map((editor) => editor.text).toList(),
        expectedRevision: current.handle.revision,
        idempotencyKey:
            'docx:${current.handle.resourceRef}:${current.handle.revision}:${saveSequence++}',
        generation: widget.generation,
        nowMs: widget.nowMs(),
      );
      if (!mounted) return;
      setState(() => receipt = value);
    } on Object catch (error) {
      if (!mounted) return;
      setState(() => saveError = error);
    } finally {
      if (mounted) setState(() => saving = false);
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.title)),
    body: FutureBuilder<OfficeDocumentSession>(
      future: opening,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return const Center(child: Text('DOCX 无法安全打开'));
        }
        final value = snapshot.data;
        if (value == null) {
          return const Center(child: CircularProgressIndicator());
        }
        _bind(value);
        final editable =
            value.inspection.capabilities.contains(OfficeCapability.edit) &&
            value.inspection.capabilities.contains(OfficeCapability.export);
        return ListView(
          padding: const EdgeInsets.all(20),
          children: [
            Text('DOCX · ${value.inspection.profile}'),
            const SizedBox(height: 12),
            for (var index = 0; index < controllers!.length; index++)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: TextField(
                  key: ValueKey('docx-paragraph-$index'),
                  controller: controllers![index],
                  readOnly: !editable,
                  maxLines: null,
                  decoration: InputDecoration(labelText: '段落 ${index + 1}'),
                ),
              ),
            if (editable)
              FilledButton(
                key: const ValueKey('docx-save'),
                onPressed: saving ? null : _save,
                child: Text(saving ? '保存中…' : '保存'),
              ),
            if (!editable) const Text('此文档包含未认证结构，仅支持查看'),
            if (receipt != null) Text('已保存 · ${receipt!.newRevision}'),
            if (saveError != null) const Text('保存冲突或服务不可用'),
          ],
        );
      },
    ),
  );
}
