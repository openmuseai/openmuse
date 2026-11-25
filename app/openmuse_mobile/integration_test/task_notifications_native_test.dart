import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_task_notifications/openmuse_task_notifications.dart';

/// Native adapter acceptance only. Events are synthetic, no account or model is
/// connected, and this entry point is excluded from the production application.
/// Use a fresh, isolated application id. An operator handles the OS permission
/// dialog and taps the resulting notification; no platform API is mocked.
/// Enable explicitly with --dart-define=OPENMUSE_NOTIFICATION_MANUAL_ACCEPTANCE=true.
void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  binding.framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  testWidgets(
    'native permission, local result, safe activation and cleanup',
    (tester) async {
      String? preference;
      final notifications = OpenMuseTaskNotifications(
        readPreference: () async => preference,
        writePreference: (value) async => preference = value,
      );
      addTearDown(notifications.dispose);
      await notifications.ready;
      expect(notifications.enabled, isFalse);
      expect(
        notifications.authorization,
        NotificationAuthorization.notDetermined,
      );
      notifications.setScope('qa:isolated:synthetic');
      var opened = 0;
      notifications.onOpenTasks = () => opened++;
      await tester.pumpWidget(
        MaterialApp(
          home: TaskNotificationsSettings(notifications: notifications),
        ),
      );
      await tester.pumpAndSettle();
      final toggle = find.byKey(const ValueKey('task-notifications-switch'));
      await tester.tap(toggle);
      await tester.pumpAndSettle();
      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();
      expect(notifications.enabled, isFalse);
      expect(
        notifications.authorization,
        NotificationAuthorization.notDetermined,
      );
      await tester.tap(toggle);
      await tester.pumpAndSettle();
      await tester.tap(find.text('Continue'));
      await tester.pump();
      debugPrint('NOTIFICATION_NATIVE_QA: Awaiting OS permission decision');
      await tester.runAsync(() async {
        final deadline = DateTime.now().add(const Duration(seconds: 90));
        while (notifications.busy && DateTime.now().isBefore(deadline)) {
          await Future<void>.delayed(const Duration(milliseconds: 200));
        }
      });
      expect(notifications.busy, isFalse);
      expect(notifications.authorization, NotificationAuthorization.granted);
      expect(preference, 'true');
      await notifications.observe(
        sessionId: 'synthetic-session',
        sequence: 1,
        succeeded: true,
        strings: OpenMuseStrings.fallback,
      );
      debugPrint(
        'NOTIFICATION_NATIVE_QA: Success delivered; tap OS notification',
      );
      await tester.runAsync(() async {
        final deadline = DateTime.now().add(const Duration(seconds: 90));
        while (opened == 0 && DateTime.now().isBefore(deadline)) {
          await Future<void>.delayed(const Duration(milliseconds: 200));
        }
      });
      expect(opened, greaterThanOrEqualTo(1));
      // The platform port supports cold activation too. Consume it after the live
      // callback to ensure it cannot address any arbitrary task or resource path.
      notifications.setScope(null);
      await notifications.consumeActivation();
      final afterClearing = opened;
      await notifications.observe(
        sessionId: 'synthetic-session',
        sequence: 2,
        succeeded: false,
        strings: OpenMuseStrings.fallback,
      );
      expect(opened, afterClearing);
      await notifications.setEnabled(false);
      expect(preference, 'false');
      debugPrint('NOTIFICATION_NATIVE_QA: Activation and scope cleanup passed');
      await tester.pumpWidget(const SizedBox.shrink());
    },
    timeout: const Timeout(Duration(minutes: 5)),
    skip: !const bool.fromEnvironment(
      'OPENMUSE_NOTIFICATION_MANUAL_ACCEPTANCE',
    ),
  );
}
