import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/application/image_overlay_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/image/image_overlay_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/presentation/image_overlay_diff_viewer.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
  const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
  const resource = MuseResourceRef(
    id: 'image',
    repository: repository,
    locator: '/workspace/hero.png',
    mediaType: 'image/png',
    displayName: 'hero.png',
  );

  test('image overlay provider uses page-region anchors without pixel payloads',
      () async {
    final before = _redPng;
    final after = _greenPng;
    final base = _version('base', resource, before);
    final target = _version('target', resource, after);
    final provider = MuseImageOverlayDiffProvider();
    final comparison = MuseComparison(
      id: 'comparison',
      resource: resource,
      base: base.ref,
      target: target.ref,
      createdAt: DateTime.utc(2026),
      actor: actor,
      rendererType: provider.rendererType,
    );
    final result = await provider.compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: _MemoryResolver({
        base.ref.id: before,
        target.ref.id: after,
      }),
    );
    final changeSet = provider.toSemanticChangeSet(
      comparison: comparison,
      payload: result.payload,
    );

    expect(result.payload.changed, isTrue);
    expect(result.payload.beforeWidth, 1);
    expect(result.payload.afterHeight, 1);
    expect(changeSet.schema, 'muse.diff.image-overlay.changeset.v1');
    expect(changeSet.changes.single.before.single, isA<MusePageRegionAnchor>());
    expect(jsonEncode(changeSet.summary).contains('PNG'), isFalse);
    expect(changeSet.changes.single.properties.containsKey('pixels'), isFalse);
  });

  testWidgets('image overlay viewer blends two surfaces', (tester) async {
    final before = _redPng;
    final after = _greenPng;
    final provider = MuseImageOverlayDiffProvider();
    final base = _version('base', resource, before);
    final target = _version('target', resource, after);
    final comparison = MuseComparison(
      id: 'comparison',
      resource: resource,
      base: base.ref,
      target: target.ref,
      createdAt: DateTime.utc(2026),
      actor: actor,
      rendererType: provider.rendererType,
    );
    final payload = MuseImageOverlayDiffPayload(
      beforeBytes: before,
      afterBytes: after,
      beforeDigest: sha256.convert(before).toString(),
      afterDigest: sha256.convert(after).toString(),
      changed: true,
      beforeWidth: 1,
      beforeHeight: 1,
      afterWidth: 1,
      afterHeight: 1,
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: MuseImageOverlayDiffViewer(
            document: MuseImageComparisonDocument(
              file: File(resource.locator),
              base: base,
              target: target,
              diff: MuseResourceDiff(
                comparison: comparison,
                payload: payload,
                changeCount: 1,
                summary: 'changed',
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    expect(find.byKey(const ValueKey('diff-image-before')), findsOneWidget);
    expect(find.byKey(const ValueKey('diff-image-after')), findsOneWidget);
    expect(find.byKey(const ValueKey('diff-image-blend')), findsOneWidget);
    await tester.pumpWidget(const SizedBox.shrink());
  });
}

MuseVersion _version(
  String id,
  MuseResourceRef resource,
  Uint8List bytes,
) =>
    MuseVersion(
      ref: MuseVersionRef(id),
      resource: resource,
      kind: MuseVersionKind.committed,
      contentRef: id,
      contentDigest: sha256.convert(bytes).toString(),
      byteLength: bytes.length,
      createdAt: DateTime.utc(2026),
      actor: const MuseActorRef(id: 'actor', displayName: 'Tester'),
    );

final class _MemoryResolver implements MuseContentResolver {
  const _MemoryResolver(this.values);

  final Map<String, Uint8List> values;

  @override
  Future<Uint8List> resolve(MuseVersion version) async =>
      values[version.ref.id]!;
}

final Uint8List _redPng = Uint8List.fromList(
  base64Decode(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==',
  ),
);

final Uint8List _greenPng = Uint8List.fromList(
  base64Decode(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==',
  ),
);
