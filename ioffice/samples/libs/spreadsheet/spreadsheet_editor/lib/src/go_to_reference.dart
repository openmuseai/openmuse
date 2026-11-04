/// A bounded A1 reference understood by the spreadsheet engine and viewport.
final class GoToReference {
  const GoToReference(this.range, this.row, this.col);

  final String range;
  final int row;
  final int col;

  static GoToReference? parse(String input) {
    final value = input.trim().toUpperCase();
    final match = RegExp(
      r'^\$?([A-Z]{1,3})\$?([1-9][0-9]{0,6})(?::\$?([A-Z]{1,3})\$?([1-9][0-9]{0,6}))?$',
    ).firstMatch(value);
    if (match == null) return null;
    int column(String letters) {
      var result = 0;
      for (final unit in letters.codeUnits) {
        result = result * 26 + unit - 64;
      }
      return result - 1;
    }

    final col = column(match[1]!);
    final row = int.parse(match[2]!) - 1;
    final endCol = match[3] == null ? col : column(match[3]!);
    final endRow = match[4] == null ? row : int.parse(match[4]!) - 1;
    if (col >= 16384 ||
        endCol >= 16384 ||
        row >= 1048576 ||
        endRow >= 1048576 ||
        endCol < col ||
        endRow < row) {
      return null;
    }
    final range =
        '${match[1]}${match[2]}'
        '${match[3] == null ? '' : ':${match[3]}${match[4]}'}';
    return GoToReference(range, row, col);
  }
}
