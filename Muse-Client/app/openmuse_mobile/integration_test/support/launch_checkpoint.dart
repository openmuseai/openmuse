import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider/path_provider.dart';

Future<void> launchCheckpoint(
  WidgetTester tester,
  IntegrationTestWidgetsFlutterBinding binding,
  String stage,
) async {
  await tester.pump(const Duration(milliseconds: 300));
  final support = await getApplicationSupportDirectory();
  if (Platform.isIOS) {
    final bytes = await binding.takeScreenshot(stage);
    final file = File('${support.path}/overseas-screenshots/$stage.png');
    await file.parent.create(recursive: true);
    await file.writeAsBytes(bytes, flush: true);
  } else if (Platform.isAndroid &&
      await File('${support.path}/acceptance-capture-stages').exists()) {
    await tester.runAsync(() async {
      await File(
        '${support.path}/acceptance-stage',
      ).writeAsString(stage, flush: true);
      final release = File('${support.path}/acceptance-release-$stage');
      final deadline = DateTime.now().add(const Duration(seconds: 45));
      while (!await release.exists() && DateTime.now().isBefore(deadline)) {
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      expect(
        await release.exists(),
        isTrue,
        reason: 'Native screenshot driver did not capture $stage',
      );
    });
  }
}
