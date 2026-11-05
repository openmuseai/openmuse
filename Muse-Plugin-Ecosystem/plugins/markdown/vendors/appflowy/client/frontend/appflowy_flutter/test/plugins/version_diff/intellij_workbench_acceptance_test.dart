import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_viewer.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('IntelliJ-style dual surface workbench screenshot',
      (tester) async {
    tester.view.devicePixelRatio = 2;
    tester.view.physicalSize = const Size(2880, 1600);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetPhysicalSize);

    const repository = MuseRepositoryRef(id: 'repo', providerId: 'host');
    const actor = MuseActorRef(id: 'user', displayName: 'Current user');
    const resource = MuseResourceRef(
      id: 'resource',
      repository: repository,
      locator: '/workspace/wps/moffice/ai/logic/command/comb/AICombConfig.kt',
      mediaType: 'text/plain',
      displayName: 'AICombConfig.kt',
    );
    final base = MuseVersion(
      ref: const MuseVersionRef('base'),
      resource: resource,
      kind: MuseVersionKind.committed,
      contentRef: 'base',
      contentDigest: '8f08ee72',
      byteLength: _before.length,
      createdAt: DateTime.utc(2026, 9, 17),
      actor: actor,
    );
    final target = MuseVersion(
      ref: const MuseVersionRef('target'),
      resource: resource,
      kind: MuseVersionKind.working,
      contentRef: 'target',
      contentDigest: '1df3671f',
      byteLength: _after.length,
      createdAt: DateTime.utc(2026, 9, 17),
      actor: actor,
    );
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: _before,
      targetText: _after,
    );
    final document = MuseTextComparisonDocument(
      file: File(resource.locator),
      base: base,
      target: target,
      diff: MuseResourceDiff(
        comparison: MuseComparison(
          id: 'comparison',
          resource: resource,
          base: base.ref,
          target: target.ref,
          createdAt: DateTime.utc(2026, 9, 17),
          actor: actor,
          rendererType: 'muse.diff-viewer.text.v1',
        ),
        payload: payload,
        changeCount: payload.changeCount,
        summary: 'kotlin change',
      ),
    );

    await tester.pumpWidget(
      MaterialApp(
        theme: ThemeData(
          brightness: Brightness.dark,
          colorScheme: const ColorScheme.dark(
            primary: Color(0xFF6B9BFA),
            surface: Color(0xFF1E1F22),
            surfaceContainerLow: Color(0xFF2B2D30),
            surfaceContainerHighest: Color(0xFF3C3F41),
            outlineVariant: Color(0xFF4E5157),
          ),
          fontFamily: 'monospace',
        ),
        home: Scaffold(
          body: RepaintBoundary(
            key: const ValueKey('diff-screenshot'),
            child: SizedBox(
              width: 1440,
              height: 800,
              child: MuseTextDiffViewer(document: document),
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 200));

    expect(find.text('基线版本'), findsOneWidget);
    expect(find.text('当前版本'), findsOneWidget);
    expect(find.textContaining('isSupportRTC'), findsWidgets);
    expect(find.byKey(const ValueKey('diff-connector-layer')), findsOneWidget);
    expect(find.byType(SelectionArea), findsNWidgets(2));

    await tester.tap(find.byTooltip('在两侧文档中查找'));
    await tester.pump();
    await tester.enterText(
      find.byKey(const ValueKey('diff-search-field')),
      'isPackageSupportRtc',
    );
    await tester.pump();
    await tester.tap(find.byTooltip('软换行'));
    await tester.pump();
    expect(find.byType(SelectionArea), findsNWidgets(2));
    expect(
      find.byKey(const ValueKey('diff-left-selection')),
      findsOneWidget,
    );
    await tester.pumpWidget(const SizedBox.shrink());
  });
}

const _before = '''
fun isSupportRTC(): Boolean {
    return CombServerParamsUtil.getServerKeyEnabled(AiAssistantUtil.ID_COMB_GLOBAL_AI_READ_CALL_PRAMS, false)
}

fun isAiRtcShow(): Boolean {
    val compName = EnvInfoFill.getComponentName()
    val isAiRtcOn = CombServerParamsUtil.isServerParamsOn(AiAssistantUtil.ID_COMB_GLOBAL_AI_READ_CALL_PRAMS, false)
    if (!isSupportRTC()) {
        return false
    }
    return true
}
''';

const _after = '''
fun isSupportRTC(): Boolean {
    if (!isPackageSupportRtc()) {
        return false
    }
    if (VersionManager.isReduceApkSizeVersion()) {
        return false
    }
    return CombServerParamsUtil.getServerKeyEnabled(AiAssistantUtil.ID_COMB_GLOBAL_AI_READ_CALL_PRAMS, false)
}

fun isAiRtcShow(): Boolean {
    if (!isPackageSupportRtc()) {
        return false
    }
    val compName = EnvInfoFill.getComponentName()
    val isAiRtcOn = CombServerParamsUtil.isServerParamsOn(AiAssistantUtil.ID_COMB_GLOBAL_AI_READ_CALL_PRAMS, false)
    if (!isSupportRTC()) {
        return false
    }
    return true
}
''';
