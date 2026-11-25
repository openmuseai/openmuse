import 'dart:convert';

/// Passive, digest-pinned comparison document. Paths are labels, never file access.
final class ViewerReviewDocument {
  ViewerReviewDocument(String source) {
    if (utf8.encode(source).length > 8 * 1024 * 1024) {
      throw const FormatException('Review too large');
    }
    final value = jsonDecode(source);
    if (value is! Map ||
        value['schema'] != 'openmuse.review-viewer/1' ||
        value['files'] is! List ||
        (value['files'] as List).length > 300) {
      throw const FormatException('Invalid review document');
    }
    title = value['title'] as String;
    if (title.length > 4096) throw const FormatException('Invalid title');
    incomplete = value['incomplete'] == true;
    final paths = <String>{};
    files = (value['files'] as List).map((f) {
      final file = Map<String, dynamic>.from(f as Map);
      if (file['path'] is! String ||
          (file['path'] as String).length > 1024 ||
          !paths.add(file['path'] as String) ||
          ![
            'added',
            'deleted',
            'modified',
            'renamed',
            'copied',
            'unchanged',
          ].contains(file['status']) ||
          ['before', 'after'].any(
            (key) =>
                file[key] != null &&
                (file[key] is! String ||
                    (file[key] as String).length > 1000000),
          ) ||
          ['additions', 'deletions'].any(
            (key) => file[key] is! int || file[key] < 0 || file[key] > 1000000,
          ) ||
          file['hunks'] is! List ||
          (file['hunks'] as List).length > 1000)
        throw const FormatException('Invalid file');
      var count = 0;
      for (final h in file['hunks'] as List) {
        if (h is! Map ||
            h['lines'] is! List ||
            [
              'oldStart',
              'newStart',
              'oldLines',
              'newLines',
            ].any((key) => h[key] is! int || h[key] < 0 || h[key] > 1000000)) {
          throw const FormatException('Invalid hunk');
        }
        for (final line in h['lines'] as List) {
          count++;
          if (line is! Map ||
              line['text'] is! String ||
              !['add', 'delete', 'context', 'meta'].contains(line['kind']) ||
              [
                line['oldLine'],
                line['newLine'],
              ].any((n) => n != null && (n is! int || n < 0 || n > 1000000))) {
            throw const FormatException('Invalid diff line');
          }
        }
      }
      if (count > 40000) throw const FormatException('Too many lines');
      return file;
    }).toList();
  }
  late final String title;
  late final bool incomplete;
  late final List<Map<String, dynamic>> files;
}

/// Directory nodes preserve identity even when directories have the same name.
List<({String path, String label, int depth, bool directory})> reviewTree(
  List<Map<String, dynamic>> files,
  Set<String> collapsed,
) {
  final rows = <({String path, String label, int depth, bool directory})>[];
  final seen = <String>{};
  final paths = files.map((f) => f['path'] as String).toList()..sort();
  for (final path in paths) {
    final parts = path.split('/');
    var hidden = false;
    for (var i = 0; i < parts.length - 1; i++) {
      final parent = parts.take(i + 1).join('/');
      if (seen.add(parent))
        rows.add((path: parent, label: parts[i], depth: i, directory: true));
      if (collapsed.contains(parent)) {
        hidden = true;
        break;
      }
    }
    if (!hidden)
      rows.add((
        path: path,
        label: parts.last,
        depth: parts.length - 1,
        directory: false,
      ));
  }
  return rows;
}
