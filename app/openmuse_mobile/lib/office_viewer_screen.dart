import 'package:flutter/material.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class OfficeViewerScreen extends StatefulWidget {
  const OfficeViewerScreen({
    super.key,
    required this.title,
    required this.format,
    required this.handle,
    required this.engine,
    required this.ranges,
    required this.generation,
    required this.nowMs,
  });

  final String title;
  final OfficeFormat format;
  final ResourceHandle handle;
  final OfficeEnginePort engine;
  final ResourceRangePort ranges;
  final int generation;
  final int Function() nowMs;

  @override
  State<OfficeViewerScreen> createState() => _OfficeViewerScreenState();
}

final class _OfficeViewerScreenState extends State<OfficeViewerScreen> {
  late final Future<OfficeDocumentSession> opening;

  @override
  void initState() {
    super.initState();
    opening = OfficeResourceTransaction(
      engine: widget.engine,
      ranges: widget.ranges,
      format: widget.format,
    ).open(widget.handle, generation: widget.generation, nowMs: widget.nowMs());
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.title)),
    body: FutureBuilder<OfficeDocumentSession>(
      future: opening,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return const Center(child: Text('Office 文件无法安全打开'));
        }
        final session = snapshot.data;
        if (session == null) {
          return const Center(child: CircularProgressIndicator());
        }
        return ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Chip(
              key: ValueKey('office-view-only'),
              label: Text(switch (widget.format) {
                OfficeFormat.slides => 'PPTX · 只读兼容视图',
                OfficeFormat.pdf => 'PDF · 文本兼容只读视图',
                _ => 'XLSX · 只读兼容视图',
              }),
            ),
            const SizedBox(height: 8),
            for (
              var index = 0;
              index < session.inspection.paragraphs.length;
              index++
            )
              SelectableText(
                session.inspection.paragraphs[index],
                key: ValueKey('office-view-row-$index'),
              ),
          ],
        );
      },
    ),
  );
}
