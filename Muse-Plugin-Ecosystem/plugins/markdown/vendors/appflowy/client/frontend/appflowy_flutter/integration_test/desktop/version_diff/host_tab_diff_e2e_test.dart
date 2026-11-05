import 'dart:io';
import 'dart:ui' as ui;

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_plugin.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_viewer.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import '../../shared/util.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Host Tab save, edit, compare and audit in the current window',
      (tester) async {
    await tester.initializeAppFlowy(windowSize: const Size(1800, 1200));
    await tester.tapAnonymousSignInButton();
    await tester.expectToSeeHomePageWithGetStartedPage();

    final sandbox =
        await Directory.systemTemp.createTemp('openmuse-host-diff-');
    addTearDown(() async {
      if (await sandbox.exists()) await sandbox.delete(recursive: true);
    });
    final file = File('${sandbox.path}/AICombConfig.kt');
    await file.writeAsString(_before);

    final service = getIt<MuseTextVersionDiffService>();
    await service.saveVersion(file, message: 'Host baseline');
    await file.writeAsString(_after);
    final document = await service.compareWithLatest(file);
    expect(document, isNotNull);
    expect(document!.diff.payload.hunks, isNotEmpty);

    getIt<TabsBloc>().openExternalPlugin(MuseTextDiffPlugin(document));
    await tester.pumpAndSettle();

    expect(find.byType(MuseTextDiffViewer), findsOneWidget);
    expect(find.textContaining('AICombConfig.kt · Diff'), findsWidgets);
    expect(find.byKey(const ValueKey('diff-before-surface')), findsOneWidget);
    expect(find.byKey(const ValueKey('diff-after-surface')), findsOneWidget);
    expect(find.byKey(const ValueKey('diff-host-workbench')), findsOneWidget);

    final auditToggle = find.byTooltip('审计信息');
    await tester.ensureVisible(auditToggle);
    await tester.tap(auditToggle);
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('diff-audit-inspector')), findsOneWidget);
    expect(find.text('Current user'), findsWidgets);

    final audit = await getIt<MuseTextVersionRepository>().listAudit(file);
    expect(
      audit.map((event) => event.type),
      containsAll(['version.capture', 'comparison.create']),
    );
    expect(audit.last.metadata['changes'], isNull);

    await tester.tap(find.byTooltip('在两侧文档中查找'));
    await tester.pump();
    await tester.enterText(
      find.byKey(const ValueKey('diff-search-field')),
      'isPackageSupportRtc',
    );
    await tester.pumpAndSettle();

    final boundary = tester.renderObject<RenderRepaintBoundary>(
      find.byKey(const ValueKey('diff-host-workbench')),
    );
    final image = await boundary.toImage(pixelRatio: 1);
    final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
    final screenshot = File(
      '${Directory.systemTemp.path}/openmuse-host-tab-diff-workbench.png',
    );
    await screenshot.writeAsBytes(bytes!.buffer.asUint8List());
    expect(await screenshot.exists(), isTrue);
    expect(await screenshot.length(), greaterThan(1000));
    // ignore: avoid_print
    print('Wrote Host Tab E2E screenshot to ${screenshot.path}');
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
