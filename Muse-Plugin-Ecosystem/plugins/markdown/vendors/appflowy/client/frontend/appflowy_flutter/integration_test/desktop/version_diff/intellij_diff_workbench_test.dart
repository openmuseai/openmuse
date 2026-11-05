import 'dart:io';
import 'dart:ui' as ui;

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_viewer.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Host macOS workbench matches IntelliJ dual-surface interactions',
      (tester) async {
    const repository = MuseRepositoryRef(id: 'repo', providerId: 'host');
    const actor = MuseActorRef(id: 'user', displayName: 'Current user');
    const resource = MuseResourceRef(
      id: 'resource',
      repository: repository,
      locator: '/workspace/AICombConfig.kt',
      mediaType: 'text/plain',
      displayName: 'AICombConfig.kt',
    );
    final payload = MuseTextDiffProvider().compareText(
      resource: resource,
      baseText: _before,
      targetText: _after,
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

    await tester.pumpWidget(
      MaterialApp(
        theme: ThemeData(brightness: Brightness.dark),
        home: Scaffold(
          body: RepaintBoundary(
            key: const ValueKey('diff-screenshot'),
            child: MuseTextDiffViewer(
              document: MuseTextComparisonDocument(
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
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 250));

    expect(find.byKey(const ValueKey('diff-before-surface')), findsOneWidget);
    expect(find.byKey(const ValueKey('diff-after-surface')), findsOneWidget);
    expect(find.byType(SelectionArea), findsNWidgets(2));

    await tester.tap(find.byTooltip('软换行'));
    await tester.pump();
    await tester.tap(find.byTooltip('在两侧文档中查找'));
    await tester.pump();
    await tester.enterText(
      find.byKey(const ValueKey('diff-search-field')),
      'isPackageSupportRtc',
    );
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 200));

    final boundary = tester.renderObject<RenderRepaintBoundary>(
      find.byKey(const ValueKey('diff-screenshot')),
    );
    final image = await boundary.toImage(pixelRatio: 1);
    final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
    final file = File(
      '${Directory.systemTemp.path}/openmuse-intellij-diff-workbench.png',
    );
    await file.writeAsBytes(bytes!.buffer.asUint8List());
    expect(await file.exists(), isTrue);
    expect(await file.length(), greaterThan(1000));
    // ignore: avoid_print
    print('Wrote Host macOS acceptance screenshot to ${file.path}');
  });
}

const _before = '''
fun isSupportRTC(): Boolean {
    return CombServerParamsUtil.getServerKeyEnabled(false)
}
''';

const _after = '''
fun isSupportRTC(): Boolean {
    if (!isPackageSupportRtc()) {
        return false
    }
    return CombServerParamsUtil.getServerKeyEnabled(false)
}
''';
