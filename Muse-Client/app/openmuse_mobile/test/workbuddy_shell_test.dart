import 'support/localized_test_app.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('发现 tab opens the light plugin market inside the shell', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final directory = Directory.systemTemp.createTempSync('market-tab-test-');
    final store = MobilePluginStore(directory);
    final controller = WorkBuddyController();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
      directory.deleteSync(recursive: true);
    });
    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(
          controller: controller,
          mobileMarketPlugin: OpenMuseMobilePluginMarket(store),
        ),
      ),
    );
    await tester.tap(find.byKey(const ValueKey('wb-tab-应用')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('apps-section-discover')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('mobile-market-search')), findsOneWidget);
    expect(find.text('编辑推荐'), findsOneWidget);
    expect(find.text('定时任务'), findsNothing);
    expect(tester.takeException(), isNull);
  });

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
        localizedTestApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(controller: controller),
        ),
      );
      await tester.pump();

      expect(find.text('OpenMuse，与你一起创造'), findsOneWidget);
      expect(find.byKey(const ValueKey('wb-cloud-developing')), findsNothing);
      expect(find.text('OpenMuse'), findsOneWidget);
      expect(find.text('选择 Desktop'), findsWidgets);
      expect(find.text('任务'), findsWidgets);
      expect(find.byKey(const ValueKey('wb-tab-应用')), findsOneWidget);
      expect(find.byKey(const ValueKey('wb-tab-发现')), findsNothing);
      expect(find.text('我的'), findsOneWidget);
      expect(find.text('发消息'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('wb-device-workspace')));
      await tester.pumpAndSettle();
      expect(find.text('任务运行设置'), findsOneWidget);
      expect(find.text('设备'), findsOneWidget);
      expect(find.text('工作空间'), findsOneWidget);
      expect(find.text('连接 Desktop 后选择工作空间'), findsWidgets);
      await tester.tap(find.byKey(const ValueKey('wb-run-settings-close')));
      await tester.pumpAndSettle();
      expect(find.text('OpenMuse，与你一起创造'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('wb-menu')));
      await tester.pumpAndSettle();
      expect(find.text('新建任务'), findsOneWidget);
      expect(find.textContaining('任务 ('), findsNothing);
      expect(find.text('开发中'), findsNothing);
      expect(find.text('撰写俄乌战争背景与最新情况'), findsNothing);
      expect(find.text('制作大模型架构PPT'), findsNothing);
      expect(find.text('再分析一下它的结构'), findsNothing);
      expect(find.text('助理'), findsNothing);
      expect(find.text('远程工作台'), findsNothing);
      expect(find.text('最近任务'), findsOneWidget);
      expect(find.text('未登录'), findsNothing);
      expect(find.text('点击登录'), findsNothing);
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
      expect(find.textContaining('请先连接 Desktop'), findsWidgets);
      expect(controller.tasks, isEmpty);
      expect(find.widgetWithText(SnackBarAction, '如何连接电脑'), findsOneWidget);
      await tester.tap(find.widgetWithText(SnackBarAction, '如何连接电脑'));
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 100)),
      );
      await tester.pumpAndSettle();
      expect(find.textContaining('How to connect a Desktop'), findsOneWidget);
      expect(
        find.textContaining('Keep Desktop online', findRichText: true),
        findsOneWidget,
      );
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();
      expect(
        tester
            .widget<TextField>(find.byKey(const ValueKey('wb-composer')))
            .controller!
            .text,
        '看一下 README',
      );
    },
  );

  testWidgets('home stays within the viewport when the keyboard is open', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    tester.view.viewInsets = const FakeViewPadding(bottom: 300);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetViewInsets);
    final controller = WorkBuddyController();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
    });

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(controller: controller),
      ),
    );
    await tester.showKeyboard(find.byKey(const ValueKey('wb-composer')));
    await tester.pump();

    expect(tester.takeException(), isNull);
    expect(
      tester.getBottomLeft(find.byKey(const ValueKey('wb-mascot'))).dy,
      lessThanOrEqualTo(
        tester.getTopLeft(find.byKey(const ValueKey('wb-composer'))).dy,
      ),
    );
  });

  testWidgets('workspace chooser scrolls instead of overflowing', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
    });
    const deviceId = 'paired.desktop';
    controller.upsertDevice(
      const WbDevice(id: deviceId, name: 'Desktop', kind: WbDeviceKind.local),
    );
    controller.selectDevice(deviceId);
    controller.replacePairedCatalog(
      deviceId: deviceId,
      workspaces: [
        for (var index = 0; index < 8; index++)
          WbWorkspace(
            id: 'dsh.workspace.$index',
            name: 'Workspace $index',
            deviceId: deviceId,
          ),
      ],
      sessions: const [],
    );

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(controller: controller),
      ),
    );
    await tester.tap(find.byKey(const ValueKey('wb-device-workspace')));
    await tester.pumpAndSettle();
    await tester.tap(find.text('工作空间'));
    await tester.pumpAndSettle();

    expect(tester.takeException(), isNull);
    await tester.scrollUntilVisible(
      find.text('Workspace 7'),
      150,
      scrollable: find.byType(Scrollable).last,
    );
    expect(tester.takeException(), isNull);
    await tester.tap(find.text('Workspace 7'));
    await tester.pumpAndSettle();
    expect(controller.selectedWorkspaceId, 'dsh.workspace.7');
  });

  testWidgets('My tab login opens the sign-in screen', (tester) async {
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
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(
          controller: controller,
          authentication: authentication,
        ),
      ),
    );
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('wb-tab-我的')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('my-account')), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('my-account')));
    await tester.pumpAndSettle();

    expect(find.text('欢迎使用 OpenMuse'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.email')), findsOneWidget);

    authentication.authenticate();
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('auth.email')), findsNothing);
    expect(find.byKey(const ValueKey('wb-tab-我的')), findsOneWidget);
  });

  testWidgets('restored account does not reopen Desktop binding', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    final authentication = _SignedInAuth();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
      authentication.dispose();
    });
    const deviceId = 'paired.desktop';
    controller.upsertDevice(
      const WbDevice(id: deviceId, name: 'Desktop', kind: WbDeviceKind.local),
    );
    controller.selectDevice(deviceId);
    controller.replacePairedCatalog(
      deviceId: deviceId,
      workspaces: const [
        WbWorkspace(
          id: 'dsh.workspace.client',
          name: 'Muse-Client',
          deviceId: deviceId,
        ),
      ],
      sessions: const [
        WbTask(
          id: 'dsh.session.one',
          title: 'OpenMuse 应用改名与重建',
          workspaceId: 'dsh.workspace.client',
          deviceId: deviceId,
          status: WbTaskStatus.completed,
          messages: [],
        ),
      ],
    );

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(
          controller: controller,
          authentication: authentication,
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey('wb-bind-desktop')), findsNothing);

    expect(find.byKey(const ValueKey('wb-cloud-developing')), findsNothing);
    await tester.tap(find.byKey(const ValueKey('wb-menu')));
    await tester.pumpAndSettle();
    expect(find.textContaining('任务 ('), findsNothing);
    expect(find.text('Muse-Client'), findsWidgets);
    expect(find.text('OpenMuse 应用改名与重建'), findsNothing);
  });

  testWidgets('reopen keeps the last bound desktop and skips the bind sheet', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    final authentication = _SignedInAuth();
    final store = _MemoryValues();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
      authentication.dispose();
    });
    await store.write(
      'openmuse.mobile.desktop.${base64Url.encode(utf8.encode('user-1'))}',
      jsonEncode({
        'kind': 'desktop',
        'deviceRef': 'desktop-1',
        'workspaceId': 'dsh.workspace.client',
      }),
    );

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(
          controller: controller,
          authentication: authentication,
          deviceSelectionStore: store,
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(controller.selectedDeviceId, 'paired.desktop-1');
    expect(find.byKey(const ValueKey('wb-bind-desktop')), findsNothing);
    expect(find.byKey(const ValueKey('wb-cloud-developing')), findsNothing);
    expect(find.text('Desktop'), findsWidgets);
    expect(find.text('云端'), findsNothing);
  });

  testWidgets(
    'selected Desktop without a grant keeps the draft and shows disconnected',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final controller = WorkBuddyController();
      final authentication = _SignedInAuth();
      final store = _MemoryValues();
      await store.write(
        'openmuse.mobile.desktop.${base64Url.encode(utf8.encode('user-1'))}',
        jsonEncode({'kind': 'desktop', 'deviceRef': 'desktop-1'}),
      );
      final directory = AccountDeviceDirectoryController(
        authentication: authentication,
        client: AccountDeviceDirectoryClient(
          cloudOrigin: Uri.parse('http://127.0.0.1:1'),
          accessToken: authentication.accessToken,
          allowInsecureLoopback: true,
        ),
        registration: () => const AccountDeviceRegistration(
          deviceRef: 'mobile-1',
          displayName: 'Phone',
          platform: 'android',
          kind: AccountDeviceKind.mobile,
        ),
      );
      final paired = PairedDesktopMobileController.discovered(
        directory: directory,
        accessToken: authentication.accessToken,
        requesterDeviceRef: 'mobile-1',
        allowInsecureLoopback: true,
        allowInsecurePrivateNetworkForTesting: false,
      );
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        await tester.pump(const Duration(seconds: 9));
        paired.dispose();
        directory.dispose();
        controller.dispose();
        authentication.dispose();
      });

      await tester.pumpWidget(
        localizedTestApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(
            controller: controller,
            authentication: authentication,
            pairedDesktop: paired,
            deviceSelectionStore: store,
          ),
        ),
      );
      await tester.pump(const Duration(milliseconds: 300));
      expect(controller.selectedDeviceId, 'paired.desktop-1');
      expect(find.text('Desktop 未连接'), findsOneWidget);

      await tester.enterText(find.byKey(const ValueKey('wb-composer')), '你好');
      await tester.testTextInput.receiveAction(TextInputAction.send);
      await tester.pump(const Duration(seconds: 13));
      expect(find.text('你好'), findsOneWidget);
      expect(find.textContaining('消息已保留'), findsOneWidget);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(seconds: 20));
    },
  );

  testWidgets('device list uses a presence dot and bind state', (tester) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
    });
    controller.upsertDevice(
      const WbDevice(
        id: 'paired.current',
        name: 'DESKTOP-FBRL8RL',
        kind: WbDeviceKind.local,
      ),
    );
    controller.upsertDevice(
      const WbDevice(
        id: 'paired.other',
        name: 'Office-Desktop',
        kind: WbDeviceKind.local,
      ),
    );
    controller.upsertDevice(
      const WbDevice(
        id: 'paired.offline',
        name: 'AlexixideMacBook-Pro.local',
        kind: WbDeviceKind.local,
        online: false,
      ),
    );
    controller.selectDevice('paired.current');

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(controller: controller),
      ),
    );
    await tester.tap(find.byKey(const ValueKey('wb-device-workspace')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('wb-pick-device')));
    await tester.pumpAndSettle();

    expect(find.text('云端'), findsNothing);
    expect(find.text('在线'), findsNothing);
    expect(find.text('重新连接'), findsOneWidget);
    expect(
      find.byKey(const ValueKey('wb-device-bind-paired.current')),
      findsOneWidget,
    );
    expect(find.text('绑定'), findsOneWidget);
    expect(find.text('离线'), findsOneWidget);
    expect(
      (tester
                  .widget<Container>(
                    find.byKey(const ValueKey('wb-device-dot-paired.current')),
                  )
                  .decoration!
              as BoxDecoration)
          .color,
      const Color(0xFF34C759),
    );
    expect(
      (tester
                  .widget<Container>(
                    find.byKey(const ValueKey('wb-device-dot-paired.offline')),
                  )
                  .decoration!
              as BoxDecoration)
          .color,
      const Color(0xFF8E8E93),
    );

    await tester.tap(find.byKey(const ValueKey('wb-device-bind-paired.other')));
    await tester.pumpAndSettle();
    expect(controller.selectedDeviceId, 'paired.other');
  });

  testWidgets(
    'speech partials replace one draft segment and final stays editable',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final controller = WorkBuddyController();
      final speech = _FakeSpeechRecognition();
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        await speech.dispose();
        controller.dispose();
      });

      await tester.pumpWidget(
        localizedTestApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(
            controller: controller,
            speechRecognition: speech,
            debugSpeechSource: const SpeechFileSource('/trusted/test.wav'),
          ),
        ),
      );
      await tester.pump();
      await tester.pump();

      speech.emit(SpeechEventKind.partial, '今天天气');
      await tester.pump();
      speech.emit(SpeechEventKind.partial, '今天天气很好');
      await tester.pump();
      expect(
        tester
            .widget<TextField>(find.byKey(const ValueKey('wb-composer')))
            .controller!
            .text,
        '今天天气很好',
      );

      speech.emit(SpeechEventKind.finalResult, '今天天气很好。');
      await tester.pump();
      final field = tester.widget<TextField>(
        find.byKey(const ValueKey('wb-composer')),
      );
      expect(field.controller!.text, '今天天气很好。');
      expect(field.enabled, isTrue);
    },
  );

  testWidgets(
    'input mode toggle hides keyboard and voice hold shows cancellable waveform',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final controller = WorkBuddyController();
      final speech = _FakeSpeechRecognition();
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        await speech.dispose();
        controller.dispose();
      });

      await tester.pumpWidget(
        localizedTestApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(
            controller: controller,
            speechRecognition: speech,
          ),
        ),
      );
      await tester.pump();
      await tester.showKeyboard(find.byKey(const ValueKey('wb-composer')));
      expect(tester.testTextInput.isVisible, isTrue);

      await tester.tap(find.byKey(const ValueKey('wb-input-mode-toggle')));
      await tester.pump();
      expect(tester.testTextInput.isVisible, isFalse);
      expect(find.byKey(const ValueKey('wb-composer')), findsNothing);
      expect(find.byKey(const ValueKey('wb-voice-hold')), findsOneWidget);
      expect(find.text('按住说话'), findsOneWidget);

      final gesture = await tester.startGesture(
        tester.getCenter(find.byKey(const ValueKey('wb-voice-hold'))),
      );
      await tester.pump(kLongPressTimeout + const Duration(milliseconds: 50));
      expect(speech.openCalls, 1);
      expect(find.byKey(const ValueKey('wb-voice-overlay')), findsOneWidget);
      expect(find.byKey(const ValueKey('wb-voice-waveform')), findsOneWidget);

      await gesture.moveBy(const Offset(0, -100));
      await tester.pump();
      expect(find.text('松手取消'), findsOneWidget);
      await gesture.up();
      await tester.pump();
      expect(speech.cancelCalls, 1);
      expect(find.byKey(const ValueKey('wb-voice-overlay')), findsNothing);

      await tester.tap(find.byKey(const ValueKey('wb-input-mode-toggle')));
      await tester.pump();
      expect(find.byKey(const ValueKey('wb-composer')), findsOneWidget);
    },
  );

  testWidgets(
    'voice result becomes visible and editable without opening keyboard',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final controller = WorkBuddyController();
      final speech = _FakeSpeechRecognition();
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        await speech.dispose();
        controller.dispose();
      });

      await tester.pumpWidget(
        localizedTestApp(
          theme: workBuddyTheme(),
          home: WorkBuddyShell(
            controller: controller,
            speechRecognition: speech,
          ),
        ),
      );
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('wb-input-mode-toggle')));
      await tester.pump();

      final gesture = await tester.startGesture(
        tester.getCenter(find.byKey(const ValueKey('wb-voice-hold'))),
      );
      await tester.pump(kLongPressTimeout + const Duration(milliseconds: 50));
      speech.emit(SpeechEventKind.partial, '今天天气');
      await tester.pump();
      expect(find.text('今天天气'), findsOneWidget);

      await gesture.up();
      await tester.pump();
      expect(find.text('正在完成识别…'), findsOneWidget);
      expect(speech.stopCalls, 1);
      speech.emit(SpeechEventKind.finalResult, '今天天气很好。');
      await tester.pump();

      final field = tester.widget<TextField>(
        find.byKey(const ValueKey('wb-composer')),
      );
      expect(field.controller!.text, '今天天气很好。');
      expect(tester.testTextInput.isVisible, isFalse);
      expect(find.byKey(const ValueKey('wb-voice-overlay')), findsNothing);
    },
  );

  testWidgets('microphone purpose is shown before the system permission', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final controller = WorkBuddyController();
    final speech = _FakeSpeechRecognition(
      permission: SpeechPermissionState.denied,
    );
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      await speech.dispose();
      controller.dispose();
    });

    await tester.pumpWidget(
      localizedTestApp(
        theme: workBuddyTheme(),
        home: WorkBuddyShell(controller: controller, speechRecognition: speech),
      ),
    );
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('wb-input-mode-toggle')));
    await tester.pump();

    final gesture = await tester.startGesture(
      tester.getCenter(find.byKey(const ValueKey('wb-voice-hold'))),
    );
    await tester.pump(kLongPressTimeout + const Duration(milliseconds: 50));
    expect(find.byKey(const ValueKey('wb-microphone-purpose')), findsOneWidget);
    expect(speech.openCalls, 0);

    await tester.tap(find.byKey(const ValueKey('wb-microphone-continue')));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 50));
    expect(speech.openCalls, 1);
    await gesture.up();
    await tester.pump();
  });
}

final class _FakeSpeechRecognition implements SpeechRecognitionPort {
  _FakeSpeechRecognition({this.permission = SpeechPermissionState.granted});

  final _session = _FakeSpeechSession();
  final SpeechPermissionState permission;
  int _sequence = 0;
  int openCalls = 0;
  int stopCalls = 0;
  int cancelCalls = 0;

  @override
  Future<SpeechSession> open(SpeechStartRequest request) async {
    openCalls += 1;
    return _session;
  }

  void emit(SpeechEventKind kind, String text) {
    _session.controller.add(
      SpeechEvent(
        session: _session.ref,
        sequence: _sequence++,
        kind: kind,
        phase: kind == SpeechEventKind.finalResult
            ? SpeechSessionPhase.completed
            : SpeechSessionPhase.recognizing,
        text: text,
      ),
    );
  }

  Future<void> dispose() => _session.controller.close();

  @override
  Future<void> cancel(SpeechSessionRef session) async {
    cancelCalls += 1;
    _session.controller.add(
      SpeechEvent(
        session: _session.ref,
        sequence: _sequence++,
        kind: SpeechEventKind.state,
        phase: SpeechSessionPhase.cancelled,
      ),
    );
  }

  @override
  Future<SpeechProbe> probe({bool requestPermission = false}) async =>
      SpeechProbe(
        permission: permission,
        localEngine: SpeechEngineAvailability.ready,
      );

  @override
  Future<void> stop(SpeechSessionRef session) async {
    stopCalls += 1;
  }

  @override
  Future<void> updateContext(
    SpeechSessionRef session,
    SpeechContext context,
  ) async {}
}

final class _FakeSpeechSession implements SpeechSession {
  final controller = StreamController<SpeechEvent>.broadcast();

  @override
  final ref = const SpeechSessionRef('test-speech-session');

  @override
  Stream<SpeechEvent> get events => controller.stream;
}

final class _MemoryValues implements SecureValueStore {
  final values = <String, String>{};

  @override
  Future<void> delete(String key) async {
    values.remove(key);
  }

  @override
  Future<String?> read(String key) async => values[key];

  @override
  Future<void> write(String key, String value) async {
    values[key] = value;
  }
}

final class _SignedInAuth extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot get snapshot =>
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: 'user-1',
          email: 'test_user1@example.com',
        ),
      );

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}

final class _SignedOutAuth extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  OpenMuseAuthenticationSnapshot _snapshot =
      const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  OpenMuseAuthenticationSnapshot get snapshot => _snapshot;

  void authenticate() {
    _snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.authenticated,
      identity: OpenMuseAuthenticatedIdentity(
        subject: 'user-1',
        email: 'test_user1@example.com',
      ),
    );
    notifyListeners();
  }

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}
