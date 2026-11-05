import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_presentation.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const resource = MuseResourceRef(
    id: 'resource',
    repository: MuseRepositoryRef(id: 'repo', providerId: 'test'),
    locator: '/workspace/main.dart',
    mediaType: 'text/plain',
    displayName: 'main.dart',
  );

  test('projects complete documents into aligned continuous rows', () {
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: 'one\ntwo\nthree\nfour\n',
      targetText: 'one\ntwo changed\nthree\ninserted\nfour\n',
    );

    final presentation = const MuseTextPresentationBuilder().build(
      payload,
      collapseUnchanged: false,
    );

    expect(presentation.runs, hasLength(2));
    expect(presentation.rows.first.left?.text, 'one');
    expect(presentation.rows.first.right?.text, 'one');
    expect(
      presentation.rows.any(
        (row) => row.left?.text == 'two' && row.right?.text == 'two changed',
      ),
      isTrue,
    );
    expect(
      presentation.rows.any(
        (row) =>
            row.left?.kind == MuseTextCellKind.filler &&
            row.right?.text == 'inserted',
      ),
      isTrue,
    );
  });

  test('folds and expands long unchanged ranges without losing changes', () {
    final before = [for (var i = 0; i < 40; i++) 'line $i'];
    final after = [...before]..[20] = 'line twenty changed';
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: before.join('\n'),
      targetText: after.join('\n'),
    );
    const builder = MuseTextPresentationBuilder();

    final folded = builder.build(payload);
    final fold = folded.rows.firstWhere((row) => row.isFold);
    expect(fold.hiddenLineCount, greaterThan(0));
    expect(fold.hiddenLeftStart, isNotNull);
    expect(fold.hiddenLeftEnd, greaterThan(fold.hiddenLeftStart!));
    expect(folded.runs, hasLength(1));

    final expanded = builder.build(
      payload,
      expandedFoldIds: {fold.foldId!},
    );
    expect(
      expanded.rows.where((row) => row.isFold).length,
      lessThan(folded.rows.where((row) => row.isFold).length),
    );
    expect(expanded.rows.length, greaterThan(folded.rows.length));
    expect(expanded.runs, hasLength(1));
  });
}
