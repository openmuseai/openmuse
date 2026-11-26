import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:openmuse_dsh_plugin/src/dsh_navigation.dart';
import 'package:openmuse_dsh_plugin/src/dsh_web_view.dart';

void main() {
  test(
    'authenticated DSH UI navigation is bounded and cannot carry an Agent command',
    () {
      expect(
        parseDshNavigationActivation({
          'type': 'navigation.activate',
          'kind': 'session',
          'sessionId': 's1',
        }),
        (kind: 'session', id: 's1'),
      );
      expect(
        parseDshNavigationActivation({
          'type': 'navigation.activate',
          'kind': 'panel',
          'panelId': null,
        }),
        (kind: 'panel', id: null),
      );
      for (final value in [
        {'type': 'agent.prompt', 'kind': 'session', 'sessionId': 's1'},
        {
          'type': 'navigation.activate',
          'kind': 'prompt',
          'panelId': 'settings',
        },
        {'type': 'navigation.activate', 'kind': 'session', 'sessionId': null},
        {'type': 'navigation.activate', 'kind': 'panel', 'panelId': 'x' * 257},
      ]) {
        expect(
          () => parseDshNavigationActivation(value),
          throwsFormatException,
        );
      }
    },
  );
  test(
    'sidebar selection projects session and panel, revocation rejects further changes',
    () async {
      final supervisor = DshSidecarSupervisor();
      supervisor.selectNavigation('session', 's1');
      supervisor.selectNavigation('panel', 'settings');
      expect(supervisor.requestedSession.value, 's1');
      expect(supervisor.requestedPanel.value, 'settings');
      supervisor.selectNavigation('session', 's2');
      expect(supervisor.requestedPanel.value, isNull);
      expect(supervisor.requestedSession.value, 's2');
      supervisor.aiSharingAllowed = false;
      supervisor.selectNavigation('session', 'private');
      expect(supervisor.requestedSession.value, 's2');
      expect(supervisor.launchCount, 0);
      supervisor.dispose();
    },
  );
  testWidgets(
    'denied AI sharing never launches a sidecar or mounts the original DSH sidebar',
    (tester) async {
      final supervisor = DshSidecarSupervisor()..aiSharingAllowed = false;
      final mount = ValueNotifier<String?>('/current');
      var activated = 0, opened = 0;
      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: DshNavigation(
              supervisor: supervisor,
              activeMount: mount,
              onActivateWorkspace: (_) async {
                activated++;
              },
              onOpenResource: (_) async {
                opened++;
              },
            ),
          ),
        ),
      );
      await tester.pump();
      expect(find.byType(DshWebView), findsNothing);
      expect(
        find.text('Confirm AI data sharing in the Agent pane.'),
        findsOneWidget,
      );
      expect(supervisor.launchCount, 0);
      expect(supervisor.endpoint, isNull);
      expect(activated + opened, 0);
      await tester.pumpWidget(const SizedBox.shrink());
      mount.dispose();
      supervisor.dispose();
    },
  );
}
