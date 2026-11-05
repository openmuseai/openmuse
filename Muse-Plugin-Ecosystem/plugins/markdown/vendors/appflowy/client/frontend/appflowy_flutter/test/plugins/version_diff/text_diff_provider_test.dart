import 'dart:convert';
import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
  const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
  const resource = MuseResourceRef(
    id: 'resource',
    repository: repository,
    locator: '/workspace/readme.md',
    mediaType: 'text/markdown',
    displayName: 'readme.md',
  );

  test('produces stable semantic hunks and inline changes', () async {
    final provider = MuseTextDiffProvider(contextLines: 1);
    final base = _version('base', resource, '# Intro\nname: Muse\nkeep\n');
    final target =
        _version('target', resource, '# Intro\nname: OpenMuse\nkeep\nnew\n');
    final resolver = _MemoryResolver({
      base.ref.id: '# Intro\nname: Muse\nkeep\n',
      target.ref.id: '# Intro\nname: OpenMuse\nkeep\nnew\n',
    });
    final comparison = MuseComparison(
      id: 'comparison',
      resource: resource,
      base: base.ref,
      target: target.ref,
      createdAt: DateTime.utc(2026),
      actor: actor,
      rendererType: provider.rendererType,
    );

    final first = await provider.compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: resolver,
    );
    final second = await provider.compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: resolver,
    );

    expect(first.payload.language, 'markdown');
    expect(first.payload.additions, 2);
    expect(first.payload.deletions, 1);
    expect(first.payload.hunks, hasLength(1));
    expect(first.payload.hunks.single.semanticLabel, 'Intro');
    expect(first.payload.changeCount, 2);
    expect(
      first.payload.hunks.single.changes.map((change) => change.id),
      second.payload.hunks.single.changes.map((change) => change.id),
    );
    expect(
      first.payload.hunks.single.rows
          .expand((row) => row.inlineSpans)
          .any((span) => span.changed),
      isTrue,
    );

    final changeSet = provider.toSemanticChangeSet(
      comparison: comparison,
      payload: first.payload,
    );
    expect(changeSet.schema, 'muse.diff.changeset.v1');
    expect(changeSet.quality.kind, MuseDiffQualityKind.exact);
    expect(changeSet.changes, hasLength(2));
    expect(changeSet.changes.first.before.single, isA<MuseTextRangeAnchor>());
    expect(changeSet.changes.first.attribution?.actor.id, actor.id);
  });

  test('registry resolves text provider without exposing line concepts', () {
    final registry = MuseDiffProviderRegistry()
      ..register(MuseTextDiffProvider());

    expect(registry.providerFor(resource)?.id, 'muse.diff.text.v1');
    expect(
      registry.providerFor(
        const MuseResourceRef(
          id: 'movie',
          repository: repository,
          locator: '/workspace/demo.mp4',
          mediaType: 'video/mp4',
          displayName: 'demo.mp4',
        ),
      ),
      isNull,
    );
  });
}

MuseVersion _version(String id, MuseResourceRef resource, String content) =>
    MuseVersion(
      ref: MuseVersionRef(id),
      resource: resource,
      kind: MuseVersionKind.committed,
      contentRef: id,
      contentDigest: id,
      byteLength: utf8.encode(content).length,
      createdAt: DateTime.utc(2026),
      actor: const MuseActorRef(id: 'actor', displayName: 'Tester'),
    );

final class _MemoryResolver implements MuseContentResolver {
  const _MemoryResolver(this.values);

  final Map<String, String> values;

  @override
  Future<Uint8List> resolve(MuseVersion version) async =>
      Uint8List.fromList(utf8.encode(values[version.ref.id]!));
}
