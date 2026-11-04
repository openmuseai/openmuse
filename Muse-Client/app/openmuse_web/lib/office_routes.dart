import 'package:flutter/widgets.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_flutter/openmuse_office_flutter.dart';

/// A document supplied by an authorized Web resource/engine adapter.
/// The browser app does not invent an Office session before that adapter exists.
final class OpenMuseWebOfficeDocument {
  const OpenMuseWebOfficeDocument({
    required this.title,
    required this.format,
    required this.handle,
    required this.engine,
    required this.ranges,
    required this.generation,
    required this.nowMs,
    this.commits,
  });

  final String title;
  final OfficeFormat format;
  final ResourceHandle handle;
  final OfficeEnginePort engine;
  final ResourceRangePort ranges;
  final OfficeResourceCommitPort? commits;
  final int generation;
  final int Function() nowMs;

  Widget buildScreen() {
    if (format == OfficeFormat.word) {
      final writer = commits;
      if (writer == null) {
        throw StateError('DOCX requires a commit port');
      }
      return DocxEditorScreen(
        title: title,
        handle: handle,
        engine: engine,
        ranges: ranges,
        commits: writer,
        generation: generation,
        nowMs: nowMs,
      );
    }
    return OfficeViewerScreen(
      title: title,
      format: format,
      handle: handle,
      engine: engine,
      ranges: ranges,
      generation: generation,
      nowMs: nowMs,
    );
  }
}
