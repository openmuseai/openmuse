import 'dart:convert';
import 'dart:io';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';

/// Checks DOCX, XLSX and PPTX with the same native inspection engines used by
/// the Android Host. PDFs use the separate PDF product renderer.
Future<void> main(List<String> args) async {
  if (args.length != 3) {
    stderr.writeln(
      'usage: dart run tool/inspect_office_samples.dart '
      '<samples-dir> <docx-dylib> <viewers-dylib>',
    );
    exitCode = 2;
    return;
  }
  final samples = Directory(args[0]);
  final docx = DocxFfiEngine.open(libraryPath: args[1]);
  final viewers = OfficeViewersFfiEngine.open(libraryPath: args[2]);
  final files =
      samples
          .listSync()
          .whereType<File>()
          .where((file) => !file.uri.pathSegments.last.startsWith('.~'))
          .toList()
        ..sort((a, b) => a.path.compareTo(b.path));
  final results = <Map<String, Object>>[];
  for (final file in files) {
    final name = file.uri.pathSegments.last;
    final format = switch (name.split('.').last.toLowerCase()) {
      'docx' => OfficeFormat.word,
      'xlsx' => OfficeFormat.sheet,
      'pptx' => OfficeFormat.slides,
      _ => null,
    };
    if (format == null) continue;
    stderr.writeln('Checking $name');
    final bytes = await file.readAsBytes();
    final inspection = await (format == OfficeFormat.word ? docx : viewers)
        .inspect(format, bytes);
    if (inspection.format != format ||
        !inspection.capabilities.contains(OfficeCapability.view)) {
      throw StateError('inspection did not enable viewing: $name');
    }
    results.add({
      'name': name,
      'format': format.name,
      'bytes': bytes.length,
      'profile': inspection.profile,
      'paragraphs': inspection.paragraphs.length,
    });
  }
  if (results.length != 4 ||
      results.where((result) => result['format'] == 'sheet').length != 2) {
    throw StateError('expected DOCX, PPTX and two XLSX samples');
  }
  stdout.writeln(const JsonEncoder.withIndent('  ').convert(results));
}
