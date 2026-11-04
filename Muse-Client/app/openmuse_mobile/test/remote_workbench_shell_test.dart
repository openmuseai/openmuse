import 'support/localized_test_app.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('RS-UI-04 walks both lab surfaces from the task drawer', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final desktop = AcceptanceDesktop();
    addTearDown(desktop.controller.dispose);
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
    });

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: RemoteWorkbenchPage(controller: desktop.controller),
      ),
    );
    await tester.pump();

    expect(find.text(acceptanceLabNotice), findsOneWidget);
    expect(find.text('社媒发布'), findsOneWidget);
    expect(find.text('视频编辑'), findsOneWidget);
    expect(find.text('网页快照'), findsOneWidget);
    expect(find.text('交互网页'), findsOneWidget);
    expect(find.text('UNSUPPORTED_MODE'), findsOneWidget);
    expect(
      tester
          .widget<ListTile>(
            find.byKey(
              const ValueKey(
                'remote-surface-com.openmuse.fake-web-interactive-interactive-page',
              ),
            ),
          )
          .enabled,
      isFalse,
    );

    await tester.tap(
      find.byKey(
        const ValueKey(
          'remote-surface-com.openmuse.fake-publish-publish-workflow',
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('封面'), findsOneWidget);
    expect(find.byKey(const ValueKey('remote-media-cover')), findsOneWidget);
    expect(find.byType(Image), findsWidgets);

    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.preview')),
    );
    await tester.pumpAndSettle();
    expect(find.text('预览已生成：周末散步'), findsOneWidget);
    await tester.ensureVisible(
      find.byKey(const ValueKey('remote-action-social.publish.commit')),
    );
    await tester.tap(
      find.byKey(const ValueKey('remote-action-social.publish.commit')),
    );
    await tester.pumpAndSettle();
    expect(find.text('发布已提交'), findsOneWidget);
    expect(find.text('任务状态 running'), findsOneWidget);
    expect(desktop.publish.externalExecutions, 1);
    expect(
      desktop.audit.entries.any(
        (entry) =>
            entry.actionId == 'social.publish.commit' &&
            entry.status == 'accepted' &&
            entry.actorRef == 'actor.acceptance' &&
            entry.decisionRef != null,
      ),
      isTrue,
    );

    await tester.tap(find.byKey(const ValueKey('remote-surface-list')));
    await tester.pumpAndSettle();
    await tester.tap(
      find.byKey(
        const ValueKey('remote-surface-com.openmuse.fake-video-edit-timeline'),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('3 秒'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('remote-action-clip.trim')));
    await tester.pumpAndSettle();
    expect(find.text('2 秒'), findsOneWidget);
    expect(desktop.video.trimExecutions, 1);
    expect(desktop.publish.externalExecutions, 1);

    await tester.tap(find.byKey(const ValueKey('remote-surface-list')));
    await tester.pumpAndSettle();
    await tester.tap(
      find.byKey(
        const ValueKey(
          'remote-surface-com.openmuse.fake-web-snapshot-page-snapshot',
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('只读网页快照'), findsOneWidget);
    expect(find.byKey(const ValueKey('remote-media-shot')), findsOneWidget);
    expect(find.text('交互网页'), findsNothing);
  });

  testWidgets('RS-UI-05 the drawer no longer offers the remote workbench', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final buddy = WorkBuddyController();
    addTearDown(buddy.dispose);
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
    });
    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(controller: buddy),
      ),
    );
    await tester.tap(find.byKey(const ValueKey('wb-menu')));
    await tester.pumpAndSettle();
    expect(find.text('远程工作台'), findsNothing);
    expect(find.text('助理'), findsNothing);
    expect(find.text('最近任务'), findsOneWidget);
    expect(buddy.tasks, isEmpty);
  });
}
