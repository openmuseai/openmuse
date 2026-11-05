import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:appflowy/plugins/word/word_apply.dart';
import 'package:appflowy/plugins/word/word_backend.dart';
import 'package:appflowy/plugins/word/word_blob_store.dart';
import 'package:appflowy/plugins/word/word_file_open.dart';
import 'package:appflowy/plugins/word/word.dart';
import 'package:appflowy/plugins/word/word_plain_text.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/workspace/application/view/view_ext.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/import/import_type.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/word_editor.dart';

void main() {
  test('P1-T4 view_ext Word maps to PluginType.word', () {
    final view = ViewPB()
      ..id = 'view-word'
      ..layout = ViewLayoutPB.Word;
    expect(view.layout.value, 9);
    expect(view.pluginType, PluginType.word);
    expect(view.layout.isDocumentView, isFalse);
    expect(view.layout.isDatabaseView, isFalse);
    expect(view.pluginType, isNot(PluginType.document));
  });

  test('P1-T5 Word plugin is creatable with layout Word', () {
    expect(WordPluginBuilder().layoutType, ViewLayoutPB.Word);
    expect(WordPluginConfig().creatable, isTrue);
    expect(WordPluginBuilder().pluginType, PluginType.word);
  });

  test('P1-T6 blob store rejects non-zip and leaves no file', () async {
    final dir = Directory.systemTemp.createTempSync('word-blob-');
    addTearDown(() => dir.deleteSync(recursive: true));
    final store = WordBlobStore(root: dir);
    expect(
      () => store.put('v1', Uint8List.fromList('nope'.codeUnits)),
      throwsFormatException,
    );
    expect(File('${dir.path}${Platform.pathSeparator}v1.docx').existsSync(), isFalse);
  });

  test('P1-T7 blob store put/get/duplicate/delete', () async {
    final dir = Directory.systemTemp.createTempSync('word-blob-ok-');
    addTearDown(() => dir.deleteSync(recursive: true));
    final store = WordBlobStore(root: dir);
    final bytes = buildMinimalDocx();
    final rev = await store.put('src', bytes);
    expect(rev, hasLength(64));
    final loaded = await store.get('src');
    expect(loaded, isNotNull);
    expect(loaded!.$1, bytes);
    expect(loaded.$2, rev);
    await store.put('dst', loaded.$1);
    final copy = await store.get('dst');
    expect(copy!.$1, bytes);
    await store.delete('src');
    expect(await store.get('src'), isNull);
  });

  test('P1-T10 Save stays disabled without toDocx', () {
    final controller = WordEditorController();
    expect(controller.canExportDocx, isFalse);
    expect(() => refuseStaleDocxSave(), throwsA(isA<WordApplyBlocked>()));
  });

  test('P1-T11 / P4 plain text from minimal docx', () {
    expect(
      plainTextFromDocx(buildMinimalDocx(text: 'OpenMuse Word')),
      contains('OpenMuse Word'),
    );
    expect(plainTextFromDocx(buildMinimalDocx()), isEmpty);
  });

  test('F1.2 import panel type is docx-only', () {
    expect(ImportType.wordDocx.allowedExtensions, ['docx']);
    expect(ImportType.wordDocx.enableOnRelease, isTrue);
    expect(ImportType.wordDocx.toString(), 'Word (.docx)');
  });

  test('F1.2 page name comes from the docx file name', () {
    expect(WordBackendService.pageNameFromFileName('报告.docx'), '报告');
    expect(WordBackendService.pageNameFromFileName('/tmp/notes.docx'), 'notes');
    expect(WordBackendService.pageNameFromFileName(''), 'Word');
  });

  test('F1.1 empty create template is not a sample document', () {
    expect(WordBackendService.emptyTemplate(), buildMinimalDocx());
    expect(plainTextFromDocx(WordBackendService.emptyTemplate()), isEmpty);
  });

  test('F1.2 safe open dir is not Documents and does not plant a sample', () async {
    final dir = await wordSafeOpenDirectory();
    expect(dir.path.contains('${Platform.pathSeparator}Documents'), isFalse);
    expect(dir.path.contains('word-open'), isTrue);
  });
}
