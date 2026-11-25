import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;
import 'package:spreadsheet_editor/spreadsheet_editor.dart'
    show SpreadsheetProductShell;
import 'package:word_editor/word_editor.dart' show WordProductShell;

import 'package:openmuse_mobile/office_viewer_screen.dart';
import 'package:openmuse_mobile_office_lab/pdf_product_screen.dart';
import 'package:openmuse_mobile_office_lab/spreadsheet_product_screen.dart';
import 'package:openmuse_mobile_office_lab/word_product_screen.dart';

const _origin = String.fromEnvironment(
  'OPENMUSE_SAMPLE_ORIGIN',
  defaultValue: 'http://127.0.0.1:18798/',
);

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  final cases = <(String, OfficeFormat)>[
    ('机械制造与自动化毕业论文.docx', OfficeFormat.word),
    ('出入库管理明细表.xlsx', OfficeFormat.sheet),
    ('可视化图形图表区.xlsx', OfficeFormat.sheet),
    ('学术研究.pptx', OfficeFormat.slides),
    ('sqs67d81926-df73-471f-968c-d9c205760673.pdf', OfficeFormat.pdf),
  ];
  for (final (name, format) in cases) {
    testWidgets('opens $name in Android OpenMuse', (tester) async {
      final bytes = await tester.runAsync(() => _download(name));
      expect(bytes, isNotEmpty);
      if (format != OfficeFormat.pdf) {
        final engine = format == OfficeFormat.word
            ? DocxFfiEngine.open()
            : OfficeViewersFfiEngine.open();
        final inspection = await engine.inspect(format, bytes!);
        expect(inspection.paragraphs, isNotEmpty);
        expect(inspection.capabilities, contains(OfficeCapability.view));
      }
      final now = DateTime.now().millisecondsSinceEpoch;
      final handle = ResourceHandle(
        resourceRef: name,
        revision: 'sample-r1',
        audience: 'openmuse-mobile-office',
        generation: 1,
        expiresAtMs: now + 600000,
        size: bytes!.length,
        mediaType: switch (format) {
          OfficeFormat.word =>
            'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
          OfficeFormat.sheet =>
            'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
          OfficeFormat.slides =>
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          OfficeFormat.pdf => 'application/pdf',
        },
      );
      final ranges = _MemoryRanges(bytes);
      final content = switch (format) {
        OfficeFormat.word => MobileWordProductScreen(
          title: name,
          handle: handle,
          ranges: ranges,
          generation: 1,
        ),
        OfficeFormat.sheet => MobileSpreadsheetProductScreen(
          title: name,
          handle: handle,
          ranges: ranges,
          generation: 1,
        ),
        OfficeFormat.slides => OfficeViewerScreen(
          title: name,
          format: format,
          handle: handle,
          engine: OfficeViewersFfiEngine.open(),
          ranges: ranges,
          generation: 1,
          nowMs: () => DateTime.now().millisecondsSinceEpoch,
        ),
        OfficeFormat.pdf => MobilePdfProductScreen(
          title: name,
          handle: handle,
          ranges: ranges,
          generation: 1,
        ),
      };
      await tester.pumpWidget(MaterialApp(home: content));
      for (var i = 0; i < 120; i++) {
        await tester.runAsync(
          () async => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump(const Duration(milliseconds: 100));
        final ready = switch (format) {
          OfficeFormat.word =>
            find.byType(WordProductShell).evaluate().isNotEmpty,
          OfficeFormat.sheet =>
            find.byType(SpreadsheetProductShell).evaluate().isNotEmpty,
          OfficeFormat.slides =>
            find
                .byKey(const ValueKey('office-view-only'))
                .evaluate()
                .isNotEmpty,
          OfficeFormat.pdf => find.byType(PdfWorkspace).evaluate().isNotEmpty,
        };
        if (ready) break;
      }
      switch (format) {
        case OfficeFormat.word:
          expect(find.byType(WordProductShell), findsOneWidget);
        case OfficeFormat.sheet:
          expect(find.byType(SpreadsheetProductShell), findsOneWidget);
        case OfficeFormat.slides:
          expect(
            find.byKey(const ValueKey('office-view-only')),
            findsOneWidget,
          );
        case OfficeFormat.pdf:
          expect(find.byType(PdfWorkspace), findsOneWidget);
      }
      await tester.pump(const Duration(milliseconds: 500));
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(seconds: 1));
    });
  }
}

Future<Uint8List> _download(String name) async {
  final client = HttpClient();
  try {
    final uri = Uri.parse(_origin).resolve(Uri.encodeComponent(name));
    final response = await (await client.getUrl(uri)).close();
    if (response.statusCode != HttpStatus.ok) {
      throw StateError('sample fetch failed: ${response.statusCode} $name');
    }
    final result = BytesBuilder(copy: false);
    await for (final chunk in response) {
      result.add(chunk);
    }
    return result.takeBytes();
  } finally {
    client.close();
  }
}

final class _MemoryRanges implements ResourceRangePort {
  const _MemoryRanges(this.bytes);
  final Uint8List bytes;

  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async => bytes.sublist(start, endExclusive);
}
