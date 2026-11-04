import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/src/word_go_to_panel.dart';

void main() {
  test('GenOffice page syntax resolves absolute, relative and boundary cases', () {
    expect(resolveWordGoToPage('4', 1, 8), 4);
    expect(resolveWordGoToPage('+2', 3, 8), 5);
    expect(resolveWordGoToPage('-2', 3, 8), 1);
    expect(resolveWordGoToPage('', 8, 8), 8);
    expect(resolveWordGoToPage('', 1, 8, dir: -1), 1);
    expect(resolveWordGoToPage('99', 3, 8), 8);
    expect(resolveWordGoToPage('4x', 3, 8), isNull);
    expect(resolveWordGoToPage('4', 1, 0), isNull);
  });
}
