import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ppt_render/ppt_render.dart';
import 'package:ppt_show/ppt_show.dart';

RenderEnvelope twoPages() => RenderEnvelope.fromJsonString('''
{
  "ok": true,
  "pipeline": "document-v1",
  "width": 960,
  "height": 540,
  "commands": [{"type":"DrawText","text":"p0"}],
  "page_count": 2,
  "selected_page": 0,
  "pages": [
    {"index":0,"width":960,"height":540,"commands":[{"type":"DrawText","text":"p0"}]},
    {"index":1,"width":960,"height":540,"commands":[{"type":"DrawText","text":"p1"}]}
  ]
}
''');

void main() {
  test('legacy envelope promotes to one page', () {
    final env = RenderEnvelope.fromJsonString(
      '{"ok":true,"width":960,"height":540,"commands":[{"type":"DrawRect"}]}',
    );
    expect(env.pageCount, 1);
    expect(env.pages, hasLength(1));
  });

  test('setPage clamps and next/previous stay in range', () {
    final c = PptShowController();
    c.showGolden(twoPages(), name: 'two');
    expect(c.pageIndex, 0);
    expect(c.currentPage.commands.single['text'], 'p0');
    c.setPage(99);
    expect(c.pageIndex, 1);
    expect(c.currentPage.commands.single['text'], 'p1');
    c.nextPage();
    expect(c.pageIndex, 1);
    c.previousPage();
    expect(c.pageIndex, 0);
    c.previousPage();
    expect(c.pageIndex, 0);
  });

  testWidgets('page rail selects current commands', (tester) async {
    final c = PptShowController();
    await c.showGolden(twoPages(), name: 'two');
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: PptShowWorkspace(controller: c)),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('Page 1/2'), findsOneWidget);
    expect(find.byType(ListTile), findsNWidgets(2));
    await tester.ensureVisible(find.byType(ListTile).at(1));
    await tester.tap(find.byType(ListTile).at(1));
    await tester.pump();
    expect(c.pageIndex, 1);
    expect(find.text('Page 2/2'), findsOneWidget);
  });
}
