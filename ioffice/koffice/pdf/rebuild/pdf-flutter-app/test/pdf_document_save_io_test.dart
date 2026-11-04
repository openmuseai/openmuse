import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/pdf_document_save_io.dart';

void main() {
  test(
    'configured generated directory is used and unavailable path falls back',
    () async {
      final root = await Directory.systemTemp.createTemp('pdf-output-policy-');
      try {
        final chosen = '${root.path}/chosen';
        final fallback = '${root.path}/fallback';
      var fallbackRead = false;
        expect(
          await resolveGeneratedPdfDirectory(
            configuredDirectory: chosen,
          fallbackDirectory: () async {
            fallbackRead = true;
            return fallback;
          },
          ),
          chosen,
        );
      expect(fallbackRead, isFalse);
        final blocked = File('${root.path}/not-a-directory');
        await blocked.writeAsString('occupied');
        expect(
          await resolveGeneratedPdfDirectory(
            configuredDirectory: blocked.path,
          fallbackDirectory: () async => fallback,
          ),
          fallback,
        );
        expect(await blocked.readAsString(), 'occupied');
      } finally {
        await root.delete(recursive: true);
      }
    },
  );

  test(
    'generated extract keeps earlier output and chooses next suffix',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'pdf-extract-test-',
      );
      try {
        final first = await writeGeneratedPdfToDirectory(
          directory.path,
          'paper-p1.pdf',
          [1, 2],
        );
        final second = await writeGeneratedPdfToDirectory(
          directory.path,
          'paper-p1.pdf',
          [3, 4],
        );
        expect(first, '${directory.path}/paper-p1.pdf');
        expect(second, '${directory.path}/paper-p1-2.pdf');
        expect(await File(first).readAsBytes(), [1, 2]);
        expect(await File(second).readAsBytes(), [3, 4]);
      } finally {
        await directory.delete(recursive: true);
      }
    },
  );

  test('batch export writes every PDF after one directory choice', () async {
    final directory = await Directory.systemTemp.createTemp('pdf-batch-test-');
    try {
      expect(
        await writePdfBatchToDirectory(directory.path, {
          'part-1.pdf': [1, 2, 3],
          'part-2.pdf': [4, 5, 6],
        }),
        2,
      );
      expect(await File('${directory.path}/part-1.pdf').readAsBytes(), [
        1,
        2,
        3,
      ]);
      expect(await File('${directory.path}/part-2.pdf').readAsBytes(), [
        4,
        5,
        6,
      ]);
      expect(directory.listSync().whereType<Directory>(), isEmpty);
    } finally {
      await directory.delete(recursive: true);
    }
  });

  test('batch export never replaces an existing target', () async {
    final directory = await Directory.systemTemp.createTemp('pdf-batch-test-');
    try {
      final existing = File('${directory.path}/part-2.pdf');
      await existing.writeAsBytes([9]);
      await expectLater(
        writePdfBatchToDirectory(directory.path, {
          'part-1.pdf': [1],
          'part-2.pdf': [2],
        }),
        throwsA(isA<FileSystemException>()),
      );
      expect(await existing.readAsBytes(), [9]);
      expect(await File('${directory.path}/part-1.pdf').exists(), isFalse);
    } finally {
      await directory.delete(recursive: true);
    }
  });
}
