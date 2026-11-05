import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('view/compare titles are unchanged by helix settings', () {
    const settings = HelixSettings.defaults;
    expect(settings.configToml, contains('theme = "openmuse_host"'));
    expect(settings.configToml, contains('F12 = "goto_definition"'));
    expect(settings.configToml, contains('S-F12 = "goto_reference"'));
    expect(settings.configToml, contains('C-F12 = "goto_type_definition"'));
    expect(settings.configToml, contains('F2 = "rename_symbol"'));
    expect(settings.configToml, contains('C-o = "jump_backward"'));
    expect(settings.configToml, contains('F6 = "jump_forward"'));
    expect(settings.configToml, contains('"A-left" = "jump_backward"'));
    expect(settings.configToml, isNot(contains('A-Left')));
    expect(settings.configToml, contains('enable = true'));
  });

  test('helix keymap still writes and keeps host navigation keys', () {
    const settings = HelixSettings(
      theme: 'onedark',
      keymap: HelixKeymapPreset.helix,
      fontFamily: 'Menlo',
      fontSize: 13,
      backgroundColor: Color(0xFF282C34),
      enableLsp: false,
    );
    expect(settings.configToml, contains('enable = false'));
    expect(settings.configToml, contains('F12 = "goto_definition"'));
    expect(settings.configToml, contains('C-o = "jump_backward"'));
  });

  test('project root walks up to git or package markers', () {
    final temp = Directory.systemTemp.createTempSync('helix-root-');
    addTearDown(() => temp.deleteSync(recursive: true));
    File('${temp.path}/.git').writeAsStringSync('');
    final nested = Directory('${temp.path}/lib/src')..createSync(recursive: true);
    final file = File('${nested.path}/main.dart')..writeAsStringSync('void main() {}');
    expect(helixProjectRoot(file).path, temp.path);
  });

  test('overlay theme inherits selected helix theme', () {
    const settings = HelixSettings.defaults;
    expect(settings.overlayThemeToml, contains('inherits = "onedark"'));
    expect(settings.overlayThemeToml, contains('ui.background'));
  });

  test('grammar helpers detect dylibs and dart files', () {
    final temp = Directory.systemTemp.createTempSync('helix-grammar-');
    addTearDown(() => temp.deleteSync(recursive: true));
    final grammars = Directory('${temp.path}/grammars')..createSync();
    File('${grammars.path}/dart.dylib').writeAsBytesSync(const [0]);
    expect(helixGrammarLibraryCount([temp.path]), 1);
    expect(helixHasGrammar('dart', [temp.path]), isTrue);
    expect(helixHasGrammar('rust', [temp.path]), isFalse);
    expect(helixGrammarNameForFile('/proj/lib/main.dart'), 'dart');
  });
}
