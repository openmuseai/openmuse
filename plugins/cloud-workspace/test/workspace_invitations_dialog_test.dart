import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'invitations_test_support.dart';

Future<void> settle(WidgetTester t) async {
  for (var i = 0; i < 12; i++) {
    await t.pump(const Duration(milliseconds: 25));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 5)),
    );
  }
  await t.pumpAndSettle();
}

Future<void> finish(WidgetTester t, InvitationFixture f) async {
  await t.pumpWidget(const SizedBox.shrink());
  f.service.closeClient();
  await settle(t);
}

void main() {
  for (final size in [
    const Size(320, 420),
    const Size(430, 900),
    const Size(1100, 700),
  ]) {
    testWidgets(
      'original invitation dialog accepts and returns to the same workspace context at $size',
      (t) async {
        t.view.physicalSize = size;
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final previousHttp = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(InvitationFixture.create))!;
        addTearDown(() async {
          await t.pumpWidget(const SizedBox.shrink());
          await settle(t);
          await t.runAsync(f.close);
          HttpOverrides.global = previousHttp;
        });
        await t.pumpWidget(
          MaterialApp(
            theme: ThemeData.dark(),
            home: Scaffold(
              body: Builder(
                builder: (context) => TextButton(
                  key: const ValueKey('open'),
                  onPressed: () => showWorkspaceInvitationsDialog(
                    context,
                    authentication: f.auth,
                    service: f.service,
                    onJoined: () async {
                      f.joined++;
                    },
                  ),
                  child: const Text('Original workspace catalog'),
                ),
              ),
            ),
          ),
        );
        await t.tap(find.byKey(const ValueKey('open')));
        await settle(t);
        final accept = find.byKey(
          const ValueKey('cloud-invitations.accept.$invitationId'),
        );
        expect(accept, findsOneWidget);
        await t.ensureVisible(accept);
        await settle(t);
        expect(t.getSize(accept).height, greaterThanOrEqualTo(44));
        await t.tap(accept);
        await settle(t);
        expect(f.posts, 1);
        expect(f.joined, 1);
        expect(
          t
              .widget<Text>(
                find.byKey(
                  const ValueKey('cloud-invitations.status.$invitationId'),
                ),
              )
              .data,
          'Accepted',
        );
        expect(accept, findsNothing);
        expect(t.takeException(), isNull);
        await t.tap(find.byKey(const ValueKey('cloud-invitations.close')));
        await settle(t);
        expect(
          find.byKey(const ValueKey('cloud-invitations.dialog')),
          findsNothing,
        );
        expect(find.text('Original workspace catalog'), findsOneWidget);
        await finish(t, f);
      },
    );
  }
  testWidgets(
    'account A-B-A hides prior invitations and retires the open dialog',
    (t) async {
      final previousHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(InvitationFixture.create))!;
      addTearDown(() async {
        await t.pumpWidget(const SizedBox.shrink());
        await settle(t);
        await t.runAsync(f.close);
        HttpOverrides.global = previousHttp;
      });
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: WorkspaceInvitationsDialog(
              authentication: f.auth,
              service: f.service,
            ),
          ),
        ),
      );
      await settle(t);
      expect(
        find.byKey(const ValueKey('cloud-invitations.row.$invitationId')),
        findsOneWidget,
      );
      f.auth.switchTo('bob');
      f.auth.switchTo('alice');
      await settle(t);
      expect(
        find.byKey(const ValueKey('cloud-invitations.row.$invitationId')),
        findsNothing,
      );
      expect(
        find.byKey(const ValueKey('cloud-invitations.failure')),
        findsOneWidget,
      );
      expect(
        t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-invitations.refresh')),
            )
            .onPressed,
        isNull,
      );
      expect(f.posts, 0);
      expect(t.takeException(), isNull);
      await finish(t, f);
    },
  );
  testWidgets('unknown server status has no accept affordance', (t) async {
    final previousHttp = HttpOverrides.current;
    HttpOverrides.global = null;
    final f = (await t.runAsync(InvitationFixture.create))!
      ..status = 'FutureStatus';
    addTearDown(() async {
      await t.pumpWidget(const SizedBox.shrink());
      await settle(t);
      await t.runAsync(f.close);
      HttpOverrides.global = previousHttp;
    });
    await t.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: WorkspaceInvitationsDialog(
            authentication: f.auth,
            service: f.service,
          ),
        ),
      ),
    );
    await settle(t);
    expect(
      find.byKey(const ValueKey('cloud-invitations.row.$invitationId')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey('cloud-invitations.accept.$invitationId')),
      findsNothing,
    );
    expect(f.posts, 0);
    expect(t.takeException(), isNull);
    await finish(t, f);
  });
}
