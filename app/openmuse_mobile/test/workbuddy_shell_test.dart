import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'landing shell never manufactures Desktop workspaces or replies',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final controller = WorkBuddyController();
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        controller.dispose();
      });
      await tester.pumpWidget(
        MaterialApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(controller: controller),
        ),
      );
      await tester.pump();

      expect(find.text('OpenMuse，与你一起创造'), findsOneWidget);
      expect(find.text('OpenMuse'), findsOneWidget);
      expect(find.text('云端'), findsOneWidget);
      expect(find.text('任务'), findsWidgets);
      expect(find.text('专家'), findsOneWidget);
      expect(find.text('资料库'), findsOneWidget);
      expect(find.text('定时任务'), findsOneWidget);
      expect(find.text('项目'), findsOneWidget);
      expect(find.text('发消息或按住说话'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('wb-device-workspace')));
      await tester.pumpAndSettle();
      expect(find.text('任务运行设置'), findsOneWidget);
      expect(find.text('设备'), findsOneWidget);
      expect(find.text('工作空间'), findsOneWidget);
      expect(find.text('登录后加载 Workspace'), findsWidgets);
      await tester.tap(find.byKey(const ValueKey('wb-run-settings-close')));
      await tester.pumpAndSettle();
      expect(find.text('OpenMuse，与你一起创造'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('wb-menu')));
      await tester.pumpAndSettle();
      expect(find.text('新建任务'), findsOneWidget);
      expect(find.text('撰写俄乌战争背景与最新情况'), findsNothing);
      expect(find.text('制作大模型架构PPT'), findsNothing);
      expect(find.text('再分析一下它的结构'), findsNothing);
      expect(find.text('助理'), findsOneWidget);
      expect(find.text('未登录'), findsOneWidget);
      expect(find.text('点击登录'), findsOneWidget);
      expect(find.text('切尔西的匕首'), findsNothing);
      expect(find.text('体验版'), findsNothing);
      expect(find.text('471.57'), findsNothing);

      controller.closeDrawer();
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('wb-composer')),
        '看一下 README',
      );
      await tester.testTextInput.receiveAction(TextInputAction.send);
      await tester.pumpAndSettle();
      expect(find.textContaining('请先从设备列表连接'), findsOneWidget);
      expect(controller.tasks, isEmpty);
    },
  );

  testWidgets('account sheet login opens the sign-in screen', (tester) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    final authentication = _SignedOutAuth();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
      authentication.dispose();
    });
    await tester.pumpWidget(
      MaterialApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(
          controller: controller,
          authentication: authentication,
        ),
      ),
    );
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('wb-menu')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('wb-account')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('wb-sign-in')), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('wb-sign-in')));
    await tester.pumpAndSettle();

    expect(find.text('Welcome to OpenMuse'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.email')), findsOneWidget);
  });
}

final class _SignedOutAuth extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot get snapshot =>
      const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}
