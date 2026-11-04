import 'package:flutter_test/flutter_test.dart';
import 'package:ppt_render/ppt_render.dart';

void main() {
  test('legacy single-page envelope is promoted to one page', () {
    final envelope = RenderEnvelope.fromJsonString(
      '{"ok":true,"pipeline":"v0","width":960,"height":540,"commands":[{"type":"DrawRect"}]}',
    );
    expect(envelope.pageCount, 1);
    expect(envelope.pages, hasLength(1));
    expect(envelope.pageAt(0).commands, hasLength(1));
  });

  test('document envelope preserves page order and page-local commands', () {
    final envelope = RenderEnvelope.fromJsonString('''
      {
        "ok": true,
        "pipeline": "document-v1",
        "width": 960,
        "height": 540,
        "commands": [{"type":"DrawText","text":"second"}],
        "page_count": 2,
        "selected_page": 0,
        "pages": [
          {"index":0,"width":960,"height":540,"commands":[{"type":"DrawText","text":"second"}]},
          {"index":1,"width":960,"height":540,"commands":[{"type":"DrawText","text":"first"}]}
        ]
      }
    ''');
    expect(envelope.pageCount, 2);
    expect(envelope.pages.map((page) => page.index), [0, 1]);
    expect(envelope.pageAt(0).commands.single['text'], 'second');
    expect(envelope.pageAt(1).commands.single['text'], 'first');
    expect(envelope.pageAt(99).index, 1);
  });
}
