import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late Directory root;
  late File file;
  late MuseTextVersionRepository repository;
  late MuseTextVersionDiffService service;

  setUp(() async {
    root = await Directory.systemTemp.createTemp('openmuse-version-diff-');
    file = await File('${root.path}/plan.md').writeAsString(
      '# Plan\n\nOriginal scope\n',
    );
    repository = MuseTextVersionRepository(rootResolver: () async => root);
    service = MuseTextVersionDiffService(repository: repository);
  });

  tearDown(() async {
    if (await root.exists()) await root.delete(recursive: true);
  });

  test('saved versions survive repository recreation', () async {
    final saved = await service.saveVersion(file, message: 'Baseline');
    final reloaded = MuseTextVersionRepository(rootResolver: () async => root);

    final versions = await reloaded.listVersions(file);
    expect(versions, hasLength(1));
    expect(versions.single.ref.id, saved.ref.id);
    expect(versions.single.message, 'Baseline');
    expect(await reloaded.resolve(versions.single), await file.readAsBytes());
  });

  test('end-to-end save edit compare and audit', () async {
    await service.saveVersion(file, message: 'Before agent edit');
    await file.writeAsString(
      '# Plan\n\nExpanded scope\n\n- audit\n- semantic diff\n',
    );

    const agent = MuseActorRef(
      id: 'agent:dsh-test',
      displayName: 'DSH Test Agent',
      kind: MuseActorKind.agent,
    );
    final document = await service.compareWithLatest(file, actor: agent);

    expect(document, isNotNull);
    expect(document!.diff.payload.hunks, isNotEmpty);
    expect(document.diff.payload.additions, 4);
    expect(document.diff.payload.deletions, 1);
    expect(document.diff.payload.hunks.first.semanticLabel, 'Plan');
    final audit = await repository.listAudit(file);
    expect(
      audit.map((event) => event.type),
      containsAll([
        'version.capture',
        'comparison.create',
      ]),
    );
    final comparison = audit.firstWhere(
      (event) => event.type == 'comparison.create',
    );
    expect(comparison.actor.kind, MuseActorKind.agent);
    expect(comparison.actor.displayName, 'DSH Test Agent');
    final summary = comparison.metadata['changeSummary'] as Map;
    expect(summary['insertions'], 0);
    expect(summary['deletions'], 0);
    expect(summary['modifications'], 1);
    expect(summary['addedLines'], 4);
    expect(summary['deletedLines'], 1);
    expect(comparison.metadata['changes'], isNull);
    expect(comparison.metadata['changeIds'], hasLength(1));
    expect(comparison.metadata['changeSetSchema'], 'muse.diff.changeset.v1');
    expect(comparison.metadata['quality'], 'exact');
    expect(comparison.metadata['changeSetDigest'], startsWith('sha256:'));

    final reopenedService = MuseTextVersionDiffService(
      repository: MuseTextVersionRepository(rootResolver: () async => root),
    );
    final initialChangeId = document.diff.payload.hunks.first.changes.first.id;
    final reopened = await reopenedService.openComparison(
      file,
      document.diff.comparison.id,
      initialChangeId: initialChangeId,
    );
    expect(reopened, isNotNull);
    expect(
      reopened!.diff.payload.changeCount,
      document.diff.payload.changeCount,
    );
    expect(reopened.initialChangeId, initialChangeId);
    expect(reopened.target.contentDigest, document.target.contentDigest);
  });

  test('compare after two saves diffs previous committed versions', () async {
    await service.saveVersion(file, message: 'v1');
    await file.writeAsString('# Plan\n\nSecond snapshot\n');
    await service.saveVersion(file, message: 'v2');

    final document = await service.compareWithLatest(file);
    expect(document, isNotNull);
    expect(document!.diff.payload.hunks, isNotEmpty);
    expect(document.diff.payload.changeCount, greaterThan(0));
    expect(document.base.message, 'v1');
    expect(document.target.message, 'v2');
  });

  test('viewVersionChanges shows the edits introduced by that snapshot',
      () async {
    await service.saveVersion(file, message: 'v1');
    await file.writeAsString('# Plan\n\nSecond snapshot\n');
    final v2 = await service.saveVersion(file, message: 'v2');

    final changes = await service.viewVersionChanges(file, v2);
    expect(changes.diff.payload.hunks, isNotEmpty);
    expect(changes.target.ref.id, v2.ref.id);
  });

  test('saveIfDirty captures once then no-ops when clean', () async {
    expect(await service.hasUncommittedChanges(file), isTrue);
    final first = await service.saveIfDirty(file, message: 'auto-1');
    expect(first, isNotNull);
    expect(await service.hasUncommittedChanges(file), isFalse);
    expect(await service.saveIfDirty(file), isNull);

    await file.writeAsString('# Plan\n\nEdited\n');
    expect(await service.hasUncommittedChanges(file), isTrue);
    final second = await service.saveIfDirty(file, message: 'auto-2');
    expect(second, isNotNull);
    expect(second!.message, 'auto-2');
    expect(await repository.listCommitted(file), hasLength(2));
  });
}
