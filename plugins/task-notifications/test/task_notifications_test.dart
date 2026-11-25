import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_task_notifications/openmuse_task_notifications.dart';

final class FakePort implements TaskNotificationPort {
  var state = NotificationAuthorization.granted;
  var requests = 0, clears = 0;
  bool activation = false, showFails = false;
  Completer<void>? authorizationWait, showWait;
  VoidCallback? activated;
  final shown = <String>[];
  @override
  Future<NotificationAuthorization> authorization() async {
    await authorizationWait?.future;
    return state;
  }

  @override
  Future<NotificationAuthorization> requestAuthorization() async {
    requests++;
    return state;
  }

  @override
  Future<void> openSettings() async {}
  @override
  Future<void> clear() async {
    clears++;
    shown.clear();
  }

  @override
  Future<bool> takeActivation() async {
    final value = activation;
    activation = false;
    return value;
  }

  @override
  void setActivationHandler(VoidCallback? handler) {
    activated = handler;
  }

  @override
  Future<void> show({
    required int id,
    required String title,
    required String body,
    required String channelName,
  }) async {
    await showWait?.future;
    if (showFails) throw StateError('native error');
    shown.add('$title $body');
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late FakePort port;
  late OpenMuseTaskNotifications notifications;
  String? saved;
  bool writeFails = false;
  setUp(() async {
    saved = null;
    writeFails = false;
    port = FakePort();
    notifications = OpenMuseTaskNotifications(
      port: port,
      readPreference: () async => saved,
      writePreference: (value) async {
        if (writeFails) throw StateError('storage unavailable');
        saved = value;
      },
    );
    await notifications.ready;
    notifications.setScope('cn:account-a:grant-a');
    await Future<void>.delayed(Duration.zero);
  });
  tearDown(() {
    notifications.dispose();
  });
  Future<void> outcome(
    int seq, {
    bool succeeded = true,
    String session = 's-1',
  }) => notifications.observe(
    sessionId: session,
    sequence: seq,
    succeeded: succeeded,
    strings: OpenMuseStrings.fallback,
  );

  test(
    'defaults are off and do not request permission or replay disabled outcomes',
    () async {
      expect(notifications.enabled, isFalse);
      expect(port.requests, 0);
      await outcome(1);
      expect(port.shown, isEmpty);
      await notifications.setEnabled(true);
      expect(saved, 'true');
      expect(port.requests, 1);
      await outcome(1);
      expect(port.shown, isEmpty);
      await outcome(2);
      expect(port.shown, hasLength(1));
    },
  );
  test(
    'results are content-free, deduplicated, and failure is distinguished',
    () async {
      await notifications.setEnabled(true);
      await outcome(2);
      await outcome(2);
      await outcome(1);
      await outcome(3, succeeded: false);
      expect(port.shown, hasLength(2));
      expect(port.shown[0], isNot(port.shown[1]));
      expect(port.shown.join(), isNot(contains('account-a')));
      expect(port.shown.join(), isNot(contains('s-1')));
    },
  );
  test('denial and revocation never show notifications', () async {
    port.state = NotificationAuthorization.denied;
    await notifications.setEnabled(true);
    await outcome(1);
    expect(port.shown, isEmpty);
    port.state = NotificationAuthorization.granted;
    await notifications.refreshAuthorization();
    await outcome(2);
    expect(port.shown, hasLength(1));
    port.state = NotificationAuthorization.denied;
    await outcome(3);
    expect(port.shown, hasLength(1));
  });
  test(
    'an account change during permission lookup prevents the stale show',
    () async {
      await notifications.setEnabled(true);
      port.authorizationWait = Completer<void>();
      final delivery = outcome(1);
      await Future<void>.delayed(Duration.zero);
      notifications.setScope('us:account-b:grant-b');
      port.authorizationWait!.complete();
      await delivery;
      await Future<void>.delayed(Duration.zero);
      expect(port.shown, isEmpty);
    },
  );
  test(
    'an account change while the native show is pending clears its result',
    () async {
      await notifications.setEnabled(true);
      port.showWait = Completer<void>();
      final delivery = outcome(1);
      await Future<void>.delayed(Duration.zero);
      notifications.setScope(null);
      port.showWait!.complete();
      await delivery;
      await Future<void>.delayed(Duration.zero);
      expect(port.shown, isEmpty);
    },
  );
  test(
    'turning off clears previous notifications and consumes later events',
    () async {
      await notifications.setEnabled(true);
      await outcome(1);
      await notifications.setEnabled(false);
      await outcome(2);
      expect(port.shown, isEmpty);
      expect(saved, 'false');
      await notifications.setEnabled(true);
      await outcome(2);
      expect(port.shown, isEmpty);
    },
  );
  test('preference failure restores the switch and reports failure', () async {
    writeFails = true;
    await notifications.setEnabled(true);
    expect(notifications.enabled, isFalse);
    expect(notifications.failed, isTrue);
    await outcome(1);
    expect(port.shown, isEmpty);
  });
  test(
    'native failure is isolated and does not retry the same event',
    () async {
      await notifications.setEnabled(true);
      port.showFails = true;
      await outcome(1);
      expect(notifications.failed, isTrue);
      port.showFails = false;
      await outcome(1);
      expect(port.shown, isEmpty);
      await outcome(2);
      expect(port.shown, hasLength(1));
    },
  );
  test(
    'notification activation has no destination until authenticated scope exists',
    () async {
      var opened = 0;
      notifications.onOpenTasks = () => opened++;
      port.activated!();
      expect(opened, 1);
      notifications.setScope(null);
      port.activated!();
      expect(opened, 1);
      port.activation = true;
      await notifications.consumeActivation();
      expect(opened, 1);
    },
  );
  test(
    'deactivation clears results and blocks stale scope and activation',
    () async {
      await notifications.setEnabled(true);
      await outcome(1);
      await notifications.deactivate();
      expect(port.shown, isEmpty);
      expect(port.activated, isNull);
      notifications.setScope('cn:stale-account:stale-grant');
      await outcome(2);
      expect(port.shown, isEmpty);
    },
  );
  test('obsolete shell cleanup cannot clear the new account scope', () async {
    notifications.setScope('us:account-b:grant-b');
    notifications.clearScope('cn:account-a:grant-a');
    await notifications.setEnabled(true);
    await outcome(1);
    expect(port.shown, hasLength(1));
  });
  testWidgets('settings support 200 percent text on a narrow screen', (
    tester,
  ) async {
    tester.view.resetPhysicalSize();
    tester.view.physicalSize = const Size(320, 640);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await tester.pumpWidget(
      MaterialApp(
        builder: (context, child) => MediaQuery(
          data: MediaQuery.of(
            context,
          ).copyWith(textScaler: TextScaler.linear(2)),
          child: child!,
        ),
        home: TaskNotificationsSettings(notifications: notifications),
      ),
    );
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('task-notifications-switch')),
      findsOneWidget,
    );
    expect(tester.takeException(), isNull);
  });
}
