import 'package:appflowy/plugins/dsh_agent/dsh_body_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_host_plugin_plane.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_markdown_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_word_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_workspace_catalog.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('E5-T1 parent chain becomes depth', () {
    final catalog = buildWorkspaceCatalog(
      workspaceId: 'ws-1',
      views: const [
        DshCatalogViewInput(
          viewId: 'root',
          title: 'Space',
          parentViewId: '',
          layout: 0,
          isSpace: true,
        ),
        DshCatalogViewInput(
          viewId: 'child',
          title: 'Doc',
          parentViewId: 'root',
          layout: 0,
          isSpace: false,
        ),
        DshCatalogViewInput(
          viewId: 'grid',
          title: 'Grid',
          parentViewId: 'child',
          layout: 1,
          isSpace: false,
        ),
      ],
    )!;
    expect(catalog.items.singleWhere((item) => item.viewId == 'root').depth, 0);
    expect(catalog.items.singleWhere((item) => item.viewId == 'child').depth, 1);
    expect(catalog.items.singleWhere((item) => item.viewId == 'grid').depth, 2);
    expect(catalog.items.singleWhere((item) => item.viewId == 'grid').layout, 'grid');
  });

  test('E5-T2 caps catalog at 64 items', () {
    final views = List<DshCatalogViewInput>.generate(
      80,
      (index) => DshCatalogViewInput(
        viewId: 'v$index',
        title: 'Page $index',
        parentViewId: '',
        layout: 0,
        isSpace: false,
      ),
    );
    final catalog = buildWorkspaceCatalog(workspaceId: 'ws-1', views: views)!;
    expect(catalog.items, hasLength(64));
    expect(catalog.truncated, isTrue);
  });

  test('E5-T3 drops items that look like secrets', () {
    final catalog = buildWorkspaceCatalog(
      workspaceId: 'ws-1',
      views: const [
        DshCatalogViewInput(
          viewId: 'ok',
          title: 'Notes',
          parentViewId: '',
          layout: 0,
          isSpace: false,
        ),
        DshCatalogViewInput(
          viewId: 'bad',
          title: 'access_token dump',
          parentViewId: '',
          layout: 0,
          isSpace: false,
        ),
      ],
    )!;
    expect(catalog.items.map((item) => item.viewId), ['ok']);
    expect(catalog.truncated, isTrue);
  });

  test('E5-T4 empty view or empty text skips snapshot', () {
    expect(
      buildMarkdownSnapshot(viewId: '', text: 'body'),
      isNull,
    );
    expect(
      buildMarkdownSnapshot(viewId: 'view-1', text: '   '),
      isNull,
    );
  });

  test('E5-T5 truncates snapshot at 32KiB', () {
    final snapshot = buildMarkdownSnapshot(
      viewId: 'view-1',
      text: 'a' * 40000,
      workspaceId: 'ws-1',
    )!;
    expect(snapshot.truncated, isTrue);
    expect(snapshot.byteLength, lessThanOrEqualTo(markdownSnapshotMaxBytes));
    expect(snapshot.text.length, lessThanOrEqualTo(markdownSnapshotMaxBytes));
  });

  test('E5-T8 plane contribute order is catalog then optional snapshot', () {
    final plane = DshHostPluginPlane(clock: () => 1000);
    final catalog = plane.catalog(
      workspaceId: 'ws-1',
      views: const [
        DshCatalogViewInput(
          viewId: 'doc-1',
          title: 'Readme',
          parentViewId: '',
          layout: 0,
          isSpace: false,
        ),
      ],
    )!;
    expect(catalog.contextType, 'workspace.catalog');
    expect(catalog.contextSchemaDigest, workspaceCatalogDigest);
    expect((catalog.payload as Map)['items'], isNotEmpty);
    expect(
      plane.snapshot(workspaceId: 'ws-1', viewId: 'doc-1', text: ''),
      isNull,
    );
    final snapshot = plane.snapshot(
      workspaceId: 'ws-1',
      viewId: 'doc-1',
      text: '# Hello',
    )!;
    expect(snapshot.contextType, 'markdown.snapshot');
    expect(snapshot.contextSchemaDigest, markdownSnapshotDigest);
  });

  test('P4-T1 layout 9 is word', () {
    expect(dshCatalogLayoutOf(9), 'word');
  });

  test('P4-T2 layout 0 is still document', () {
    expect(dshCatalogLayoutOf(0), 'document');
  });

  test('P4-T3 unknown layout is not document', () {
    expect(dshCatalogLayoutOf(99), 'unknown');
  });

  test('P4-T4 Word snapshot truncates at 32KiB', () {
    final snapshot = buildWordSnapshot(
      viewId: 'word-1',
      text: 'w' * 40000,
      workspaceId: 'ws-1',
    )!;
    expect(snapshot.truncated, isTrue);
    expect(snapshot.byteLength, lessThanOrEqualTo(wordSnapshotMaxBytes));
  });

  test('P4-T5 Word snapshot drops token-like text', () {
    expect(
      buildWordSnapshot(viewId: 'word-1', text: 'access_token dump'),
      isNull,
    );
  });

  test('P4-T11 empty Word template still emits snapshot', () {
    final plane = DshHostPluginPlane(clock: () => 1000);
    final snapshot = plane.wordSnapshot(
      workspaceId: 'ws-1',
      viewId: 'word-1',
      text: '',
    )!;
    expect(snapshot.contextType, 'word.snapshot');
    expect(snapshot.contextSchemaDigest, wordSnapshotDigest);
    expect((snapshot.payload as Map)['text'], '');
    expect(plane.snapshot(workspaceId: 'ws-1', viewId: 'doc-1', text: ''), isNull);
  });

  test('P4-T6 focus Document is markdown not word', () {
    expect(
      dshBodySnapshotKind(layout: 0, viewId: 'doc-1'),
      DshBodySnapshotKind.markdown,
    );
  });

  test('P4-T7 focus Word is word not markdown', () {
    expect(
      dshBodySnapshotKind(layout: 9, viewId: 'word-1'),
      DshBodySnapshotKind.word,
    );
  });

  test('P4-T8 no focus emits no body snapshot', () {
    expect(dshBodySnapshotKind(layout: 9, viewId: ''), DshBodySnapshotKind.none);
    expect(dshBodySnapshotKind(layout: 0, viewId: null), DshBodySnapshotKind.none);
  });
}
