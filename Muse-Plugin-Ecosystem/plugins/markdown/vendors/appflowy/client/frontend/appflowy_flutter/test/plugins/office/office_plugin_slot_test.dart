import 'package:appflowy/plugins/office/office.dart';
import 'package:appflowy/plugins/word/word.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/application/view/view_ext.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/import/import_type.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  setUp(OfficePluginCatalog.ensureInstalled);

  tearDown(OfficePluginRegistry.instance.resetForTest);

  test('catalog reserves Word=9 Excel=10 Slides=11 Pdf=12', () {
    final registry = OfficePluginRegistry.instance;
    expect(registry.mustGet('word').layout.value, 9);
    expect(registry.mustGet('excel').layout.value, 10);
    expect(registry.mustGet('slides').layout.value, 11);
    expect(registry.mustGet('pdf').layout.value, 12);
    expect(ViewLayoutPB.Word.value, 9);
    expect(ViewLayoutPB.Excel.value, 10);
    expect(ViewLayoutPB.Slides.value, 11);
    expect(ViewLayoutPB.Pdf.value, 12);
  });

  test('Word stays engine-bound; other office slots are placeholders', () {
    final registry = OfficePluginRegistry.instance;
    expect(registry.mustGet('word').engineBound, isTrue);
    expect(registry.mustGet('word').pageBuilder, isNotNull);
    for (final id in ['excel', 'slides', 'pdf']) {
      final manifest = registry.mustGet(id);
      expect(manifest.engineBound, isFalse);
      expect(manifest.pageBuilder, isNull);
      expect(manifest.creatable, isTrue);
    }
  });

  test('Word wrappers still expose PluginType.word and layout 9', () {
    expect(WordPluginBuilder().layoutType, ViewLayoutPB.Word);
    expect(WordPluginBuilder().pluginType, PluginType.word);
    expect(WordPluginConfig().creatable, isTrue);
  });

  test('view_ext maps office layouts to plugin types and not Document', () {
    ViewPB view(ViewLayoutPB layout) => ViewPB()
      ..id = 'v-${layout.name}'
      ..layout = layout;

    expect(view(ViewLayoutPB.Word).pluginType, PluginType.word);
    expect(view(ViewLayoutPB.Excel).pluginType, PluginType.excel);
    expect(view(ViewLayoutPB.Slides).pluginType, PluginType.slides);
    expect(view(ViewLayoutPB.Pdf).pluginType, PluginType.pdf);

    for (final layout in [
      ViewLayoutPB.Word,
      ViewLayoutPB.Excel,
      ViewLayoutPB.Slides,
      ViewLayoutPB.Pdf,
    ]) {
      expect(layout.isDocumentView, isFalse);
      expect(layout.isDatabaseView, isFalse);
      expect(layout.pluginHeight, 450);
      expect(isOfficeLayout(layout), isTrue);
    }
  });

  test('empty templates are zip or PDF magic, not Document CRDT', () {
    final zip = emptyZipOfficePackage();
    expect(zip[0], 0x50);
    expect(zip[1], 0x4b);
    final pdf = emptyPdf();
    expect(String.fromCharCodes(pdf.take(5)), '%PDF-');
    expect(() => validatePdf(zip), throwsFormatException);
  });

  test('import types bind to office catalog ids', () {
    expect(ImportType.wordDocx.officePluginId, 'word');
    expect(ImportType.excelXlsx.allowedExtensions, ['xlsx']);
    expect(ImportType.slidesPptx.allowedExtensions, ['pptx']);
    expect(ImportType.pdfFile.allowedExtensions, ['pdf']);
    expect(ImportType.markdownOrText.officePluginId, isNull);
  });

  test('OfficePluginBuilder.forId resolves registered slots', () {
    expect(OfficePluginBuilder.forId('excel').menuName, 'Excel');
    expect(OfficePluginBuilder.forId('slides').layoutType, ViewLayoutPB.Slides);
    expect(OfficePluginBuilder.forId('pdf').pluginType, PluginType.pdf);
  });
}
