import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/image/image_overlay_diff_provider.dart';
import 'package:crypto/crypto.dart';

final class MuseImageComparisonDocument {
  const MuseImageComparisonDocument({
    required this.file,
    required this.base,
    required this.target,
    required this.diff,
    this.semanticChangeSet,
  });

  final File file;
  final MuseVersion base;
  final MuseVersion target;
  final MuseResourceDiff<MuseImageOverlayDiffPayload> diff;
  final MuseSemanticChangeSet? semanticChangeSet;
}

final class MuseImageOverlayDiffService {
  MuseImageOverlayDiffService({
    required this.repository,
    MuseImageOverlayDiffProvider? provider,
  }) : provider = provider ?? MuseImageOverlayDiffProvider();

  final MuseTextVersionRepository repository;
  final MuseImageOverlayDiffProvider provider;

  bool supports(File file) =>
      MuseImageOverlayDiffProvider.supportsFile(file.path);

  Future<MuseVersion> saveVersion(
    File file, {
    String message = 'Manual snapshot',
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) {
    return repository.capture(
      file,
      kind: MuseVersionKind.committed,
      message: message,
      actor: actor,
    );
  }

  Future<MuseImageComparisonDocument?> compareWithLatest(
    File file, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) async {
    final base = await repository.latestCommitted(file);
    if (base == null) return null;
    final target = await repository.capture(
      file,
      kind: MuseVersionKind.working,
      message: 'Working tree',
      persistMetadata: false,
      actor: actor,
    );
    final now = DateTime.now().toUtc();
    final comparison = MuseComparison(
      id: 'comparison:${sha256.convert(utf8.encode('${base.ref.id}:${target.ref.id}:${now.microsecondsSinceEpoch}'))}',
      resource: base.resource,
      base: base.ref,
      target: target.ref,
      createdAt: now,
      actor: actor,
      rendererType: provider.rendererType,
    );
    final diff = await provider.compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: repository,
    );
    final semanticChangeSet = provider.toSemanticChangeSet(
      comparison: comparison,
      payload: diff.payload,
    );
    await repository.recordComparison(
      comparison,
      base: base,
      target: target,
      changeMetadata: {
        'changeSetSchema': semanticChangeSet.schema,
        'changeSetDigest':
            'sha256:${sha256.convert(utf8.encode(semanticChangeSet.changes.map((change) => change.id).join('\n')))}',
        'providerId': semanticChangeSet.providerId,
        'providerVersion': semanticChangeSet.providerVersion,
        'quality': semanticChangeSet.quality.kind.name,
        'changeSummary': {
          'changed': diff.payload.changed,
          'beforeDigest': diff.payload.beforeDigest,
          'afterDigest': diff.payload.afterDigest,
        },
        'changeIds': [
          for (final change in semanticChangeSet.changes) change.id,
        ],
      },
    );
    return MuseImageComparisonDocument(
      file: file,
      base: base,
      target: target,
      diff: diff,
      semanticChangeSet: semanticChangeSet,
    );
  }
}
