import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_file_viewer_flutter/src/review_document.dart';
import 'package:openmuse_file_viewer_flutter/src/review_view.dart';

Map<String, dynamic> fixture() => {
  'schema': 'openmuse.review-viewer/1',
  'title': 'Native review',
  'files': [
    {
      'path': 'src/nested/a.txt',
      'status': 'modified',
      'before': 'one\ntwo\nold\nfour\nfive',
      'after': 'one\ntwo\nnew\nfour\nfive',
      'additions': 1,
      'deletions': 1,
      'hunks': [
        {
          'oldStart': 3,
          'oldLines': 1,
          'newStart': 3,
          'newLines': 1,
          'lines': [
            {'kind': 'delete', 'oldLine': 3, 'text': 'old'},
            {'kind': 'add', 'newLine': 3, 'text': 'new'},
          ],
        },
      ],
    },
    {
      'path': 'src/other/b.txt',
      'status': 'added',
      'before': '',
      'after': 'x',
      'additions': 1,
      'deletions': 0,
      'hunks': [],
    },
  ],
};
void main() {
  test('tree collapses a directory without merging same-name paths', () {
    final d = ViewerReviewDocument(jsonEncode(fixture()));
    final tree = reviewTree(d.files, {'src/nested'});
    expect(tree.any((r) => r.path == 'src/nested/a.txt'), isFalse);
    expect(tree.any((r) => r.path == 'src/other/b.txt'), isTrue);
    expect(
      () => ViewerReviewDocument('{"schema":"wrong","files":[]}'),
      throwsFormatException,
    );
  });
  testWidgets(
    'native diff expands and folds untouched lines, opens exact full version',
    (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: OpenMuseReviewBody(contents: jsonEncode(fixture())),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.text('展开 2 行未变更内容'), findsNWidgets(2));
      await tester.tap(find.byKey(const ValueKey('review-fold-1:1')));
      await tester.pumpAndSettle();
      expect(find.text('折叠 2 行未变更内容'), findsOneWidget);
      expect(find.text('one'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('review-fold-1:1')));
      await tester.pumpAndSettle();
      expect(find.text('one'), findsNothing);
      await tester.tap(find.byKey(const ValueKey('review-file-menu')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('新文件'));
      await tester.pumpAndSettle();
      expect(
        find.byKey(const ValueKey('file-viewer-code-line-3')),
        findsOneWidget,
      );
      expect(find.text('new'), findsOneWidget);
    },
  );
  testWidgets('desktop sidebar collapses without losing current snapshot', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OpenMuseReviewBody(contents: jsonEncode(fixture())),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('review-tree-src/nested/a.txt')),
      findsOneWidget,
    );
    await tester.tap(find.byKey(const ValueKey('review-sidebar-toggle')));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('review-tree-src/nested/a.txt')),
      findsNothing,
    );
    expect(find.text('src/nested/a.txt'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('review-sidebar-toggle')));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('review-tree-src/nested/a.txt')),
      findsOneWidget,
    );
  });
}
