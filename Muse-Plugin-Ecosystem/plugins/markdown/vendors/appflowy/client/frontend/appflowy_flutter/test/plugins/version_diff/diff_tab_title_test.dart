import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/diff_tab_title.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const resource = MuseResourceRef(
    id: 'resource',
    repository: MuseRepositoryRef(id: 'repo', providerId: 'test'),
    locator: '/workspace/main.dart',
    mediaType: 'text/plain',
    displayName: 'main.dart',
  );
  const actor = MuseActorRef(id: 'actor', displayName: 'Tester');

  MuseVersion version({
    required String id,
    MuseVersionKind kind = MuseVersionKind.committed,
    String? message,
    String? digest,
  }) =>
      MuseVersion(
        ref: MuseVersionRef(id),
        resource: resource,
        kind: kind,
        contentRef: id,
        contentDigest: digest ?? id,
        byteLength: 1,
        createdAt: DateTime.utc(2026, 9, 17, 11, 50),
        actor: actor,
        message: message,
      );

  test('view tab title is filename-hash', () {
    final saved = version(
      id: 'v1',
      message: '自动保存',
      digest: 'a1b2c3d4e5f67890',
    );
    expect(museDiffViewTabTitle('main.dart', saved), 'main.dart-a1b2c3d4');
  });

  test('compare tab title is the filename; icon is applied separately', () {
    final older = version(id: 'v1', message: '自动保存');
    final current = version(
      id: 'working',
      kind: MuseVersionKind.working,
      message: 'Working tree',
    );
    expect(
      museDiffCompareTabTitle('main.dart', older, current),
      'main.dart',
    );
  });

  test('working tree and empty messages fall back to 当前 / timestamp', () {
    expect(museVersionTabName(version(id: 'w', kind: MuseVersionKind.working)),
        '当前');
    expect(museVersionTabName(version(id: 'empty', message: 'Empty')),
        isNot(equals('Empty')));
    expect(museVersionTabName(version(id: 'manual', message: 'Manual snapshot')),
        contains('Sep'));
  });
}
