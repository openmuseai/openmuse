import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/services.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('packaged DOCX ABI is callable', (tester) async {
    final engine = DocxFfiEngine.open();
    expect(engine.abi, 'openmuse-docx-ffi@1');
    await expectLater(
      engine.inspect(OfficeFormat.word, const [1, 2, 3]),
      throwsA(isA<DocxEngineException>()),
    );
  });

  testWidgets('packaged viewer ABI is callable and view only', (tester) async {
    final engine = OfficeViewersFfiEngine.open();
    expect(engine.abi, 'openmuse-office-viewers-ffi@1');
    await expectLater(
      engine.inspect(OfficeFormat.sheet, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    await expectLater(
      engine.inspect(OfficeFormat.slides, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    await expectLater(
      engine.inspect(OfficeFormat.pdf, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    expect(
      () => engine.exportSimple(OfficeFormat.sheet, const [1], const ['x']),
      throwsA(isA<OfficeViewerException>()),
    );
  });

  testWidgets('Android PPTX viewer reads a real GenOffice fixture', (
    tester,
  ) async {
    final fixture = await rootBundle.load(
      'test_assets/genoffice_standard_business.pptx',
    );
    final inspection = await OfficeViewersFfiEngine.open().inspect(
      OfficeFormat.slides,
      fixture.buffer.asUint8List(fixture.offsetInBytes, fixture.lengthInBytes),
    );
    expect(inspection.profile, 'view-only');
    expect(inspection.paragraphs, isNotEmpty);
  });

  testWidgets('Android market installs and starts the PPTX A2UI plugin', (
    tester,
  ) async {
    final directory = await Directory.systemTemp.createTemp(
      'pptx-market-test-',
    );
    try {
      const source = BundledMobileMarketSource();
      final entry = (await source.list()).singleWhere(
        (candidate) => candidate.id == 'com.openmuse.pptx',
      );
      expect(entry.platforms, contains('android'));
      expect(entry.platforms, isNot(contains('ios')));
      final store = MobilePluginStore(directory);
      await store.install(entry, await source.artifact(entry));
      expect(store.isEnabled(entry.id), isTrue);
      final bundle = store.installed(entry.id)!;
      final session = MobilePluginSession(
        pluginId: bundle.id,
        source: bundle.js,
        capability: (_, service, _) async =>
            throw UnsupportedError('denied: $service'),
      );
      await session.start();
      expect(session.lastError.value, isNull);
      await session.close();
    } finally {
      await directory.delete(recursive: true);
    }
  });
}
