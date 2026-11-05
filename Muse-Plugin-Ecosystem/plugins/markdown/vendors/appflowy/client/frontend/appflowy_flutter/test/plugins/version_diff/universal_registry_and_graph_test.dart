import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_graph.dart';
import 'package:appflowy/plugins/version_diff/generic/binary_metadata_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/image/image_overlay_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
  const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
  const textResource = MuseResourceRef(
    id: 'text',
    repository: repository,
    locator: '/workspace/main.rs',
    mediaType: 'text/plain',
    displayName: 'main.rs',
  );
  const imageResource = MuseResourceRef(
    id: 'image',
    repository: repository,
    locator: '/workspace/image.png',
    mediaType: 'image/png',
    displayName: 'image.png',
  );

  test('provider registry prefers domain provider and retains binary fallback',
      () {
    final registry = MuseSemanticDiffProviderRegistry()
      ..register(MuseBinaryMetadataDiffProvider())
      ..register(MuseTextDiffProvider());

    expect(registry.providerFor(textResource), isA<MuseTextDiffProvider>());
    expect(
      registry.providerFor(imageResource),
      isA<MuseBinaryMetadataDiffProvider>(),
    );

    registry.register(MuseImageOverlayDiffProvider());
    expect(
      registry.providerFor(imageResource)?.semanticProviderId,
      'muse.diff.image-overlay.v1',
    );
  });

  test('binary fallback reports changed bytes without claiming exact semantics',
      () async {
    final base = _version('v1', imageResource, 'digest-a', 100);
    final target =
        _version('v2', imageResource, 'digest-b', 120, parents: [base.ref]);
    final comparison = MuseComparison(
      id: 'comparison',
      resource: imageResource,
      base: base.ref,
      target: target.ref,
      createdAt: DateTime.utc(2026),
      actor: actor,
      rendererType: 'binary',
    );

    final result = await MuseBinaryMetadataDiffProvider().compareSemantic(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: const _UnusedResolver(),
    );

    expect(result.quality.kind, MuseDiffQualityKind.binaryOnly);
    expect(result.schema, 'muse.diff.binary-metadata.changeset.v1');
    expect(result.changes, hasLength(1));
    expect(result.changes.single.kind, MuseUniversalChangeKind.modify);
    expect(result.summary['changed'], isTrue);
  });

  test('renderer registry negotiates schema and preferred mode', () {
    final registry = MuseDiffRendererRegistry()
      ..register(
        MuseDiffRendererManifest(
          id: 'text-renderer',
          supportedChangeSchemas: {'muse.diff.changeset.v1'},
          modes: {MuseDiffViewMode.sideBySide, MuseDiffViewMode.unified},
          capabilities: const MuseDiffRendererCapabilities(
            selection: true,
            syncScroll: true,
          ),
        ),
      );

    expect(
      registry
          .resolve(
            changeSetSchema: 'muse.diff.changeset.v1',
            preferredMode: MuseDiffViewMode.sideBySide,
          )
          ?.id,
      'text-renderer',
    );
    expect(
      registry.resolve(
        changeSetSchema: 'unknown',
        preferredMode: MuseDiffViewMode.sideBySide,
      ),
      isNull,
    );
  });

  test('version graph projects branches, merge edges and missing parents', () {
    final root = _version('root', textResource, 'a', 1);
    final left = _version('left', textResource, 'b', 1, parents: [root.ref]);
    final right = _version('right', textResource, 'c', 1, parents: [root.ref]);
    final merge = _version(
      'merge',
      textResource,
      'd',
      1,
      parents: [left.ref, right.ref, const MuseVersionRef('remote-missing')],
    );

    final graph = const MuseVersionGraphProjector().project([
      root,
      left,
      right,
      merge,
    ]);

    expect(graph.edges, hasLength(4));
    expect(
      graph.nodes.singleWhere((node) => node.version.ref.id == 'merge').isHead,
      isTrue,
    );
    expect(
      graph.nodes.singleWhere((node) => node.version.ref.id == 'merge').depth,
      2,
    );
    expect(
      graph.nodes
          .singleWhere((node) => node.version.ref.id == 'merge')
          .missingParentIds,
      ['remote-missing'],
    );
  });
}

MuseVersion _version(
  String id,
  MuseResourceRef resource,
  String digest,
  int bytes, {
  List<MuseVersionRef> parents = const [],
}) =>
    MuseVersion(
      ref: MuseVersionRef(id),
      resource: resource,
      kind: MuseVersionKind.committed,
      contentRef: 'sha256:$digest',
      contentDigest: digest,
      byteLength: bytes,
      createdAt: DateTime.utc(2026).add(Duration(seconds: bytes)),
      actor: const MuseActorRef(id: 'actor', displayName: 'Tester'),
      parents: parents,
    );

final class _UnusedResolver implements MuseContentResolver {
  const _UnusedResolver();

  @override
  Future<Uint8List> resolve(MuseVersion version) =>
      throw UnsupportedError('Binary metadata diff does not read content');
}
