import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_viewer.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('opens in split layout and can switch to unified layout',
      (tester) async {
    tester.view.devicePixelRatio = 1;
    tester.view.physicalSize = const Size(1400, 900);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetPhysicalSize);

    const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
    const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
    const resource = MuseResourceRef(
      id: 'resource',
      repository: repository,
      locator: '/workspace/main.ts',
      mediaType: 'text/plain',
      displayName: 'main.ts',
    );
    final base = MuseVersion(
      ref: const MuseVersionRef('base'),
      resource: resource,
      kind: MuseVersionKind.committed,
      contentRef: 'base',
      contentDigest: 'base',
      byteLength: 42,
      createdAt: DateTime.utc(2026),
      actor: actor,
    );
    final target = MuseVersion(
      ref: const MuseVersionRef('target'),
      resource: resource,
      kind: MuseVersionKind.working,
      contentRef: 'target',
      contentDigest: 'target',
      byteLength: 46,
      createdAt: DateTime.utc(2026),
      actor: actor,
    );
    final comparison = MuseComparison(
      id: 'comparison',
      resource: resource,
      base: base.ref,
      target: target.ref,
      createdAt: DateTime.utc(2026),
      actor: actor,
      rendererType: 'muse.diff-viewer.text.v1',
    );
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: 'export function name() {\n  return "Muse";\n}\n',
      targetText: 'export function name() {\n  return "OpenMuse";\n}\n',
    );
    final document = MuseTextComparisonDocument(
      file: File(resource.locator),
      base: base,
      target: target,
      diff: MuseResourceDiff(
        comparison: comparison,
        payload: payload,
        changeCount: payload.changeCount,
        summary: 'text change',
      ),
    );

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            width: 1200,
            height: 800,
            child: MuseTextDiffViewer(document: document),
          ),
        ),
      ),
    );
    await tester.pump();

    expect(find.text('1 处变更'), findsOneWidget);
    expect(find.text('+1'), findsOneWidget);
    expect(find.text('-1'), findsOneWidget);
    expect(find.text('基线版本'), findsOneWidget);
    expect(find.text('当前版本'), findsOneWidget);
    expect(find.textContaining('return "Muse"'), findsOneWidget);
    expect(find.textContaining('return "OpenMuse"'), findsOneWidget);
    expect(
      find.byKey(const ValueKey('diff-before-surface')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey('diff-after-surface')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey('diff-connector-layer')),
      findsOneWidget,
    );
    expect(find.byKey(const ValueKey('diff-changes-tree')), findsOneWidget);
    expect(
      tester
          .widget<SegmentedButton<MuseDiffLayout>>(
            find.byType(SegmentedButton<MuseDiffLayout>),
          )
          .selected,
      {MuseDiffLayout.split},
    );

    await tester.tap(find.text('统一'));
    await tester.pump();
    expect(
      tester
          .widget<SegmentedButton<MuseDiffLayout>>(
            find.byType(SegmentedButton<MuseDiffLayout>),
          )
          .selected,
      {MuseDiffLayout.unified},
    );

    await tester.tap(find.byTooltip('审计信息'));
    await tester.pump();
    expect(
      find.byKey(const ValueKey('diff-audit-inspector')),
      findsOneWidget,
    );

    await tester.tap(find.text('并排'));
    await tester.pump();
    await tester.tap(find.byTooltip('在两侧文档中查找'));
    await tester.pump();
    await tester.enterText(
        find.byKey(const ValueKey('diff-search-field')), 'OpenMuse');
    await tester.pump();
    expect(find.text('1 / 1'), findsWidgets);

    await tester.tap(find.byTooltip('软换行'));
    await tester.pump();
    expect(find.byType(SelectionArea), findsNWidgets(2));
    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('expands folds for search, overview click, and filler skip copy',
      (tester) async {
    tester.view.devicePixelRatio = 1;
    tester.view.physicalSize = const Size(1400, 900);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetPhysicalSize);

    final before = [
      for (var i = 0; i < 40; i++) i == 5 ? 'unique_fold_needle' : 'line $i',
    ];
    final after = [...before]..[20] = 'line twenty changed';
    const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
    const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
    const resource = MuseResourceRef(
      id: 'resource',
      repository: repository,
      locator: '/workspace/fold.kt',
      mediaType: 'text/plain',
      displayName: 'fold.kt',
    );
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: '${before.join('\n')}\n',
      targetText: '${after.join('\n')}\n',
    );
    final document = MuseTextComparisonDocument(
      file: File(resource.locator),
      base: MuseVersion(
        ref: const MuseVersionRef('base'),
        resource: resource,
        kind: MuseVersionKind.committed,
        contentRef: 'base',
        contentDigest: 'base',
        byteLength: before.join('\n').length,
        createdAt: DateTime.utc(2026),
        actor: actor,
      ),
      target: MuseVersion(
        ref: const MuseVersionRef('target'),
        resource: resource,
        kind: MuseVersionKind.working,
        contentRef: 'target',
        contentDigest: 'target',
        byteLength: after.join('\n').length,
        createdAt: DateTime.utc(2026),
        actor: actor,
      ),
      diff: MuseResourceDiff(
        comparison: MuseComparison(
          id: 'comparison',
          resource: resource,
          base: const MuseVersionRef('base'),
          target: const MuseVersionRef('target'),
          createdAt: DateTime.utc(2026),
          actor: actor,
          rendererType: 'muse.diff-viewer.text.v1',
        ),
        payload: payload,
        changeCount: payload.changeCount,
        summary: 'fold change',
      ),
    );

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            width: 1200,
            height: 800,
            child: MuseTextDiffViewer(document: document),
          ),
        ),
      ),
    );
    await tester.pump();

    expect(find.textContaining('展开'), findsWidgets);
    expect(find.text('unique_fold_needle'), findsNothing);

    await tester.tap(find.byTooltip('在两侧文档中查找'));
    await tester.pump();
    await tester.enterText(
      find.byKey(const ValueKey('diff-search-field')),
      'unique_fold_needle',
    );
    await tester.pump();
    expect(find.text('unique_fold_needle'), findsWidgets);

    await tester.tap(find.byKey(const ValueKey('diff-left-overview')));
    await tester.pump();
    expect(find.byKey(const ValueKey('diff-changes-tree')), findsOneWidget);

    await tester.tap(find.byTooltip('对齐变化'));
    await tester.pump();
    expect(find.byType(SelectionContainer), findsWidgets);
    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('inserted-only rows keep unselectable fillers', (tester) async {
    tester.view.devicePixelRatio = 1;
    tester.view.physicalSize = const Size(1400, 900);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetPhysicalSize);

    const repository = MuseRepositoryRef(id: 'repo', providerId: 'test');
    const actor = MuseActorRef(id: 'actor', displayName: 'Tester');
    const resource = MuseResourceRef(
      id: 'resource',
      repository: repository,
      locator: '/workspace/insert.ts',
      mediaType: 'text/plain',
      displayName: 'insert.ts',
    );
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: 'keep\n',
      targetText: 'keep\ninserted\n',
    );
    final document = MuseTextComparisonDocument(
      file: File(resource.locator),
      base: MuseVersion(
        ref: const MuseVersionRef('base'),
        resource: resource,
        kind: MuseVersionKind.committed,
        contentRef: 'base',
        contentDigest: 'base',
        byteLength: 5,
        createdAt: DateTime.utc(2026),
        actor: actor,
      ),
      target: MuseVersion(
        ref: const MuseVersionRef('target'),
        resource: resource,
        kind: MuseVersionKind.working,
        contentRef: 'target',
        contentDigest: 'target',
        byteLength: 14,
        createdAt: DateTime.utc(2026),
        actor: actor,
      ),
      diff: MuseResourceDiff(
        comparison: MuseComparison(
          id: 'comparison',
          resource: resource,
          base: const MuseVersionRef('base'),
          target: const MuseVersionRef('target'),
          createdAt: DateTime.utc(2026),
          actor: actor,
          rendererType: 'muse.diff-viewer.text.v1',
        ),
        payload: payload,
        changeCount: payload.changeCount,
        summary: 'insert',
      ),
    );

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            width: 1200,
            height: 800,
            child: MuseTextDiffViewer(document: document),
          ),
        ),
      ),
    );
    await tester.pump();
    expect(find.byKey(const ValueKey('diff-filler-cell')), findsWidgets);
    await tester.pumpWidget(const SizedBox.shrink());
  });
}
