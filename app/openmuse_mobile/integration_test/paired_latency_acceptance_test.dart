import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:openmuse_mobile/main.dart' as app;
import 'package:openmuse_mobile/native_dsh_page.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';

const expectedAccount = String.fromEnvironment('LIVE_ACCEPTANCE_ACCOUNT');
void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..shouldPropagateDevicePointerEvents = true;
  testWidgets(
    'live CN paired Desktop latency acceptance',
    (tester) async {
      expect(expectedAccount, isNotEmpty);
      await app.main();
      Future<void> until(
        bool Function() condition,
        String label, {
        int seconds = 90,
      }) async {
        final clock = Stopwatch()..start();
        while (!condition()) {
          if (clock.elapsed > Duration(seconds: seconds)) {
            throw TimeoutException(label);
          }
          await tester.pump(const Duration(milliseconds: 100));
          await Future<void>.delayed(const Duration(milliseconds: 50));
        }
      }

      await until(
        () => find.byType(WorkBuddyShell).evaluate().isNotEmpty,
        'shell',
      );
      WorkBuddyShell shell() =>
          tester.widget<WorkBuddyShell>(find.byType(WorkBuddyShell));
      if (shell().authentication?.snapshot.isAuthenticated != true) {
        final authFile = File(
          '/storage/emulated/0/Android/data/com.openmuseai.mobile/cache/live-acceptance-auth.json',
        );
        await until(
          () =>
              shell().authentication?.snapshot.isAuthenticated == true ||
              authFile.existsSync(),
          'account restore or private login input',
        );
        if (shell().authentication?.snapshot.isAuthenticated != true) {
          final auth = jsonDecode(await authFile.readAsString()) as Map;
          await authFile.delete();
          await shell().authentication!.signInWithPassword(
            auth['email'] as String,
            auth['password'] as String,
          );
          auth.clear();
        }
      }
      await until(
        () => shell().authentication?.snapshot.isAuthenticated == true,
        'account login',
      );
      final paired = shell().pairedDesktop!;
      await until(
        () => paired.devices.any(
          (d) =>
              d.kind == AccountDeviceKind.desktop &&
              d.online &&
              d.supportsPairedDesktop,
        ),
        'online desktop',
      );
      final desktops = paired.devices
          .where(
            (d) =>
                d.kind == AccountDeviceKind.desktop &&
                d.online &&
                d.supportsPairedDesktop,
          )
          .toList();
      expect(desktops.length, 1, reason: 'Select an unambiguous live Desktop');
      final desktop = desktops.single;
      shell().controller.selectDevice('paired.${desktop.deviceRef}');
      expect(await paired.connectDevice(desktop), true);
      final connected = paired.snapshot.connection!;
      final catalog = DshNativeGatewayClient(
        origin: Uri.parse(connected.session.origin),
        bootstrapPath: connected.session.path,
        trace: connected.trace,
      );
      try {
        await catalog.initialize();
        final workspaces = await catalog.listWorkspaces();
        final sessions = await catalog.listSessions();
        final byId = {
          for (final session in sessions) session.sessionId: session,
        };
        shell().controller.replacePairedCatalog(
          deviceId: 'paired.${desktop.deviceRef}',
          workspaces: [
            for (final w in workspaces)
              WbWorkspace(
                id: 'dsh.workspace.${w.workspaceId}',
                name: w.title,
                deviceId: 'paired.${desktop.deviceRef}',
              ),
          ],
          sessions: [
            for (final w in workspaces)
              for (final id in w.sessionIds)
                if (byId[id] case final session?)
                  WbTask(
                    id: 'dsh.session.$id',
                    title: session.title ?? 'Conversation',
                    workspaceId: 'dsh.workspace.${w.workspaceId}',
                    deviceId: 'paired.${desktop.deviceRef}',
                    status: session.running
                        ? WbTaskStatus.running
                        : WbTaskStatus.completed,
                    messages: const [],
                  ),
          ],
        );
        final target = workspaces.firstWhere((w) => w.sessionIds.isNotEmpty);
        shell().controller.selectWorkspace(
          'dsh.workspace.${target.workspaceId}',
        );
      } finally {
        catalog.close();
      }
      expect(shell().authentication?.snapshot.identity?.email, expectedAccount);
      final connection = shell().pairedDesktop!.snapshot.connection!;
      expect(Uri.parse(connection.session.origin).host, 'cn.openmuseai.com');
      final controller = shell().controller;
      controller.selectTab(WbTab.tasks);
      controller.startNewTask();
      await tester.pump(const Duration(milliseconds: 200));
      const marker =
          'CN-DEVICE-20261111-A: Reply only CN_SYNC_OK. Do not use tools.';
      await tester.enterText(find.byKey(const ValueKey('wb-composer')), marker);
      await tester.tap(find.byKey(const ValueKey('wb-send')));
      await until(() {
        final consent = find.byKey(const ValueKey('ai-sharing-allow'));
        return consent.evaluate().isNotEmpty ||
            controller.openTaskId?.startsWith('dsh.session.') == true;
      }, 'session create');
      await tester.pump(const Duration(milliseconds: 300));
      await until(
        () =>
            find
                .byKey(const ValueKey('ai-sharing-allow'))
                .evaluate()
                .isNotEmpty ||
            find
                .byType(NativeDshPage)
                .evaluate()
                .any(
                  (element) =>
                      (element.widget as NativeDshPage).handle?.snapshot?.rows
                          .any((row) => row.text == marker) ==
                      true,
                ),
        'sharing approval',
      );
      if (find
          .byKey(const ValueKey('ai-sharing-allow'))
          .evaluate()
          .isNotEmpty) {
        await tester.tap(find.byKey(const ValueKey('ai-sharing-allow')));
      }
      DshConversationSnapshot? active() {
        for (final element
            in find.byType(NativeDshPage, skipOffstage: false).evaluate()) {
          final page = element.widget as NativeDshPage;
          if (page.active && page.handle != null) return page.handle!.snapshot;
        }
        return null;
      }

      await until(
        () =>
            active()?.rows.any(
              (row) =>
                  row.text == marker && row.kind == DshConversationRowKind.user,
            ) ==
            true,
        'durable mobile echo',
      );
      await until(
        () =>
            active()?.rows.any(
              (row) =>
                  row.kind == DshConversationRowKind.assistant &&
                  row.text.contains('CN_SYNC_OK'),
            ) ==
            true,
        'real Desktop assistant response',
        seconds: 120,
      );
      final first = controller.openTaskId!;
      final grant = shell().pairedDesktop!.snapshot.connection!.grantRef;
      final other = controller.tasks
          .where(
            (task) =>
                task.deviceId == controller.selectedDeviceId &&
                task.id.startsWith('dsh.session.') &&
                task.id != first &&
                task.workspaceId == controller.selectedWorkspaceId,
          )
          .first;
      controller.openTaskById(other.id);
      await tester.pump(const Duration(milliseconds: 500));
      controller.openTaskById(first);
      await tester.pump(const Duration(milliseconds: 100));
      expect(active()?.rows.any((row) => row.text == marker), true);
      expect(shell().pairedDesktop!.snapshot.connection!.grantRef, grant);
      debugPrint('LIVE_ACCEPTANCE_HOT_SWITCH_OK');
      // Allow the host to perform a real Android Home -> resume cycle via the mirror.
      const hold = int.fromEnvironment(
        'LIVE_ACCEPTANCE_HOLD_SECONDS',
        defaultValue: 0,
      );
      if (hold > 0) {
        debugPrint('LIVE_ACCEPTANCE_BACKGROUND_WINDOW');
        final clock = Stopwatch()..start();
        while (clock.elapsed < Duration(seconds: hold)) {
          await tester.pump(const Duration(milliseconds: 100));
          await Future<void>.delayed(const Duration(milliseconds: 100));
        }
        expect(shell().pairedDesktop!.snapshot.connection!.grantRef, grant);
        expect(active()?.rows.any((row) => row.text == marker), true);
      }
      final records = ConnectionTrace.recent;
      final report = {
        'market': 'cn',
        'durableEcho': true,
        'assistantReply': true,
        'hotSwitchPreservedGrant': true,
        'trace': records,
      };
      binding.reportData = report;
      debugPrint(
        'LIVE_ACCEPTANCE_RESULT durableEcho=true assistantReply=true hotSwitchPreservedGrant=true',
      );
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
