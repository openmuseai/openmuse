import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const repository = MuseRepositoryRef(id: 'repo', providerId: 'local');
  const resource = MuseResourceRef(
    id: 'resource',
    repository: repository,
    locator: 'opaque-resource',
    mediaType: 'text/plain',
    displayName: 'file.txt',
  );

  test('diff request accepts only two-way or three-way inputs', () {
    MuseDiffInput input(MuseDiffSide side, String version) => MuseDiffInput(
          side: side,
          version: MuseVersionRef(version),
          title: version,
        );

    final request = MuseDiffRequest(
      requestId: 'request',
      comparisonId: 'comparison',
      resource: resource,
      inputs: [
        input(MuseDiffSide.before, 'v1'),
        input(MuseDiffSide.after, 'v2'),
      ],
    );
    expect(request.inputs, hasLength(2));
    expect(
      () => MuseDiffRequest(
        requestId: 'invalid',
        comparisonId: 'comparison',
        resource: resource,
        inputs: [input(MuseDiffSide.before, 'v1')],
      ),
      throwsArgumentError,
    );
  });

  test('semantic change set defensively freezes provider output', () {
    final changes = <MuseSemanticChange>[
      MuseSemanticChange(
        id: 'change',
        kind: MuseUniversalChangeKind.modify,
        semanticPath: 'text/function/main',
        label: 'main',
      ),
    ];
    final changeSet = MuseSemanticChangeSet(
      providerId: 'text',
      providerVersion: '1',
      comparisonId: 'comparison',
      resource: resource,
      changes: changes,
      quality: const MuseDiffQuality(kind: MuseDiffQualityKind.exact),
    );
    changes.clear();

    expect(changeSet.changes, hasLength(1));
    expect(
      () => changeSet.changes.add(changeSet.changes.single),
      throwsUnsupportedError,
    );
  });
}
