import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

void main() {
  test('macOS Viewer bundle is a WKWebView-compatible classic script', () {
    final root = Directory('assets/engines/open-file-viewer');
    final entry = File('${root.path}/viewer.js');
    final index = File('${root.path}/index.html');

    expect(entry.existsSync(), isTrue, reason: 'viewer assets were not built');
    final html = index.readAsStringSync();
    expect(html, contains('<script src="viewer.js"></script>'));
    expect(html, isNot(contains('type="module"')));
    expect(
      entry.lengthSync(),
      lessThan(24 * 1024 * 1024),
      reason: 'classic macOS fallback unexpectedly exceeded 24 MiB',
    );
  });
}
