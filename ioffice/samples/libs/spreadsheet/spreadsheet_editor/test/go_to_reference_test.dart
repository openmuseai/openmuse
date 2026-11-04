import 'package:flutter_test/flutter_test.dart';
import 'package:spreadsheet_editor/src/go_to_reference.dart';
import 'package:spreadsheet_editor/src/product_definition.dart';

void main() {
  test('accepts bounded cells and rectangular ranges', () {
    final cell = GoToReference.parse(r'$z$100');
    expect(cell?.range, 'Z100');
    expect(cell?.row, 99);
    expect(cell?.col, 25);
    expect(GoToReference.parse('B2:D20')?.range, 'B2:D20');
    expect(GoToReference.parse('XFD1048576'), isNotNull);
  });

  test('rejects reversed, unsupported, and out-of-grid references', () {
    for (final value in [
      'A0',
      'XFE1',
      'A1048577',
      'D20:B2',
      'Sheet1!A1',
      'A1,B2',
      'A:A',
      '1:1',
      r'A$B1',
    ]) {
      expect(GoToReference.parse(value), isNull, reason: value);
    }
  });

  test('older signed chrome without Go To configuration remains readable', () {
    final chrome = SpreadsheetChromeDefinition.fromJson({
      'profile': 'openmuse-spreadsheet-chrome/1',
      'tabs': [],
      'quickAccess': [],
      'sheetTabMenu': '',
      'menus': <String, dynamic>{},
      'colors': <String, dynamic>{},
      'metrics': <String, dynamic>{},
      'initialTab': 'home',
    });
    expect(chrome.goToDialog, isNull);
    expect(chrome.statusMessages.ready, 'Ready');
  });
}
