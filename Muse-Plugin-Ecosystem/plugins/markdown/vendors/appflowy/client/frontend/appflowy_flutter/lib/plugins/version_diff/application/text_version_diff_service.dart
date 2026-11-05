import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:crypto/crypto.dart';

final class MuseTextComparisonDocument {
  const MuseTextComparisonDocument({
    required this.file,
    required this.base,
    required this.target,
    required this.diff,
    this.semanticChangeSet,
    this.initialChangeId,
  });

  final File file;
  final MuseVersion base;
  final MuseVersion target;
  final MuseResourceDiff<MuseTextDiffPayload> diff;
  final MuseSemanticChangeSet? semanticChangeSet;
  final String? initialChangeId;
}

final class MuseTextVersionDiffService {
  MuseTextVersionDiffService({
    required this.repository,
    MuseTextDiffProvider? provider,
  }) : provider = provider ?? MuseTextDiffProvider();

  final MuseTextVersionRepository repository;
  final MuseTextDiffProvider provider;

  bool supports(File file) {
    final extension = file.path.split('.').last.toLowerCase();
    return MuseTextDiffProvider.supportedExtensions.contains(extension);
  }

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

  Future<bool> hasUncommittedChanges(File file) {
    return repository.hasUncommittedChanges(file);
  }

  Future<MuseVersion?> saveIfDirty(
    File file, {
    String message = '自动保存',
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) {
    return repository.captureIfDirty(
      file,
      message: message,
      actor: actor,
    );
  }

  Future<MuseTextComparisonDocument?> compareWithLatest(
    File file, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) async {
    final committed = await repository.listCommitted(file);
    if (committed.isEmpty) return null;
    final latest = committed.last;
    final working = await repository.capture(
      file,
      kind: MuseVersionKind.working,
      message: 'Working tree',
      persistMetadata: false,
      actor: actor,
    );
    if (working.contentDigest != latest.contentDigest) {
      return compareVersions(file, latest, working, actor: actor);
    }
    if (committed.length < 2) {
      return compareVersions(file, latest, working, actor: actor);
    }
    return compareVersions(
      file,
      committed[committed.length - 2],
      latest,
      actor: actor,
    );
  }

  Future<MuseTextComparisonDocument> compareVersions(
    File file,
    MuseVersion base,
    MuseVersion target, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
    bool recordAudit = true,
  }) async {
    final now = DateTime.now().toUtc();
    final idSource =
        '${base.ref.id}:${target.ref.id}:${now.microsecondsSinceEpoch}';
    final comparison = MuseComparison(
      id: 'comparison:${sha256.convert(utf8.encode(idSource))}',
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
      contentResolver: _resolverFor(base, target),
    );
    final semanticChangeSet = provider.toSemanticChangeSet(
      comparison: comparison,
      payload: diff.payload,
    );
    if (recordAudit) {
      await repository.recordComparison(
        comparison,
        base: base,
        target: target,
        changeMetadata: _auditMetadata(diff.payload, semanticChangeSet),
      );
    }
    return MuseTextComparisonDocument(
      file: file,
      base: base,
      target: target,
      diff: diff,
      semanticChangeSet: semanticChangeSet,
    );
  }

  Future<MuseTextComparisonDocument> viewVersionChanges(
    File file,
    MuseVersion version, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) async {
    final committed = await repository.listCommitted(file);
    final index =
        committed.indexWhere((candidate) => candidate.ref.id == version.ref.id);
    if (index <= 0) {
      return compareVersions(
        file,
        _emptyParent(version),
        version,
        actor: actor,
        recordAudit: false,
      );
    }
    return compareVersions(
      file,
      committed[index - 1],
      version,
      actor: actor,
      recordAudit: false,
    );
  }

  Future<MuseVersion> snapshotWorking(
    File file, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) {
    return repository.capture(
      file,
      kind: MuseVersionKind.working,
      message: 'Working tree',
      persistMetadata: false,
      actor: actor,
    );
  }

  Future<MuseTextComparisonDocument> compareVersionWithCurrent(
    File file,
    MuseVersion base, {
    MuseActorRef actor = MuseTextVersionRepository.defaultActor,
  }) async {
    final target = await snapshotWorking(file, actor: actor);
    return compareVersions(file, base, target, actor: actor);
  }

  MuseContentResolver _resolverFor(MuseVersion base, MuseVersion target) {
    if (base.ref.id != 'version:empty' && target.ref.id != 'version:empty') {
      return repository;
    }
    return _InlineContentResolver(
      repository,
      {
        if (base.ref.id == 'version:empty') base.ref.id: Uint8List(0),
        if (target.ref.id == 'version:empty') target.ref.id: Uint8List(0),
      },
    );
  }

  MuseVersion _emptyParent(MuseVersion version) {
    final digest = sha256.convert(const <int>[]).toString();
    return MuseVersion(
      ref: const MuseVersionRef('version:empty'),
      resource: version.resource,
      kind: MuseVersionKind.committed,
      contentRef: 'sha256:$digest',
      contentDigest: digest,
      byteLength: 0,
      createdAt: version.createdAt,
      actor: version.actor,
      message: 'Empty',
    );
  }

  Future<MuseTextComparisonDocument?> openComparison(
    File file,
    String comparisonId, {
    String? initialChangeId,
  }) async {
    final comparison = await repository.comparisonById(file, comparisonId);
    if (comparison == null) return null;
    final base = await repository.versionById(file, comparison.base.id);
    final target = await repository.versionById(file, comparison.target.id);
    if (base == null || target == null) return null;
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
    return MuseTextComparisonDocument(
      file: file,
      base: base,
      target: target,
      diff: diff,
      semanticChangeSet: semanticChangeSet,
      initialChangeId: initialChangeId,
    );
  }

  Map<String, Object?> _auditMetadata(
    MuseTextDiffPayload payload,
    MuseSemanticChangeSet semanticChangeSet,
  ) {
    var insertions = 0;
    var deletions = 0;
    var modifications = 0;
    for (final hunk in payload.hunks) {
      for (final change in hunk.changes) {
        switch (change.kind) {
          case MuseTextChangeKind.insert:
            insertions++;
          case MuseTextChangeKind.delete:
            deletions++;
          case MuseTextChangeKind.replace:
            modifications++;
        }
      }
    }
    final changeIds = semanticChangeSet.changes
        .map((change) => change.id)
        .toList(growable: false);
    final changeSetDigest = sha256.convert(utf8.encode(changeIds.join('\n')));
    return {
      'changeSetSchema': semanticChangeSet.schema,
      'changeSetDigest': 'sha256:$changeSetDigest',
      'providerId': semanticChangeSet.providerId,
      'providerVersion': semanticChangeSet.providerVersion,
      'quality': semanticChangeSet.quality.kind.name,
      'changeSummary': {
        'insertions': insertions,
        'deletions': deletions,
        'modifications': modifications,
        'addedLines': payload.additions,
        'deletedLines': payload.deletions,
      },
      'changeIds': changeIds,
    };
  }
}

final class _InlineContentResolver implements MuseContentResolver {
  const _InlineContentResolver(this.inner, this.overrides);

  final MuseContentResolver inner;
  final Map<String, Uint8List> overrides;

  @override
  Future<Uint8List> resolve(MuseVersion version) async {
    final override = overrides[version.ref.id];
    if (override != null) return override;
    return inner.resolve(version);
  }
}
