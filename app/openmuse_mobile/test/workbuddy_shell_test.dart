import 'dart:async';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
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
      expect(find.byKey(const ValueKey('wb-cloud-developing')), findsNothing);
      expect(find.text('OpenMuse'), findsOneWidget);
      expect(find.text('选择 Desktop'), findsWidgets);
      expect(find.text('任务'), findsWidgets);
      expect(find.text('专家'), findsOneWidget);
      expect(find.text('资料库'), findsOneWidget);
      expect(find.text('定时任务'), findsOneWidget);
      expect(find.text('项目'), findsOneWidget);
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
      expect(find.textContaining('请先连接 Desktop'), findsWidgets);
      expect(controller.tasks, isEmpty);
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
      MaterialApp(
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
      MaterialApp(
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

    authentication.authenticate();
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('auth.email')), findsNothing);
    expect(find.byKey(const ValueKey('wb-menu')), findsOneWidget);
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
      MaterialApp(
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
        MaterialApp(
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
        MaterialApp(
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
        MaterialApp(
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
}

final class _FakeSpeechRecognition implements SpeechRecognitionPort {
  final _session = _FakeSpeechSession();
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
      const SpeechProbe(
        permission: SpeechPermissionState.granted,
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
