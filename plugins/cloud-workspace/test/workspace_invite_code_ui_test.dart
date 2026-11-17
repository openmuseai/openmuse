import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'access_dialog_test_support.dart';
import 'workspace_access_dialog_test.dart' show mountAccess, finishAccess;

Future<void> codeUntil(WidgetTester t, bool Function() ready) async {
  for (var i = 0; i < 160; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 3)),
    );
    if (ready()) return;
  }
  fail('Invitation code presentation did not settle');
}

Future<void> codeTap(WidgetTester t, String key) async {
  final f = find.byKey(ValueKey(key));
  final contents = find.byKey(const ValueKey('cloud-access.contents'));
  for (
    var i = 0;
    i < 20 && f.evaluate().isEmpty && contents.evaluate().isNotEmpty;
    i++
  ) {
    await t.drag(contents, const Offset(0, -180));
    await t.pump(const Duration(milliseconds: 100));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 3)),
    );
  }
  for (var i = 0; i < 40; i++) {
    await t.ensureVisible(f);
    await t.pump(const Duration(milliseconds: 40));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 3)),
    );
    if (f.hitTestable().evaluate().length == 1) break;
  }
  expect(
    f.hitTestable(),
    findsOneWidget,
    reason: '$key receives pointer events',
  );
  await t.tap(f);
  await t.pump(const Duration(milliseconds: 250));
}

void main() {
  WidgetController.hitTestWarningShouldBeFatal = true;
  for (final width in [320.0, 430.0, 1100.0]) {
    testWidgets(
      'original code management is hidden until read and confirms resets/disable at $width',
      (t) async {
        t.view.physicalSize = Size(width, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final old = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(
          () => AccessDialogFixture.create(page: false),
        ))!;
        f.inviteCodes = true;
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = old;
        });
        try {
          await mountAccess(t, f, compact: true);
          expect(f.reads.where((p) => p.endsWith('/invite-code')), isEmpty);
          await codeTap(t, 'cloud-code.panel');
          await codeUntil(
            t,
            () =>
                find
                    .byKey(const ValueKey('cloud-code.generate'))
                    .evaluate()
                    .isNotEmpty &&
                t
                        .widget<TextButton>(
                          find.byKey(const ValueKey('cloud-code.generate')),
                        )
                        .onPressed !=
                    null,
          );
          expect(
            find.text('Validity for the next generated code'),
            findsOneWidget,
          );
          final generate = find.byKey(const ValueKey('cloud-code.generate'));
          final oldGenerate = t.widget<TextButton>(generate).onPressed!;
          expect(t.getSize(generate).height, greaterThanOrEqualTo(44));
          await codeTap(t, 'cloud-code.generate');
          await codeUntil(
            t,
            () =>
                find
                    .byKey(const ValueKey('cloud-code.notice'))
                    .evaluate()
                    .isNotEmpty &&
                f.codeGenerations == 1,
          );
          expect(find.text(f.inviteCode!), findsNothing);
          oldGenerate();
          await t.pump();
          expect(f.codeGenerations, 1);
          await codeTap(t, 'cloud-code.generate');
          await codeTap(t, 'cloud-code.confirm-cancel');
          expect(f.codeGenerations, 1);
          await codeTap(t, 'cloud-code.generate');
          await codeTap(t, 'cloud-code.confirm');
          await codeUntil(
            t,
            () =>
                f.codeGenerations == 2 &&
                t
                        .widget<TextButton>(
                          find.byKey(const ValueKey('cloud-code.copy')),
                        )
                        .onPressed !=
                    null,
          );
          await codeTap(t, 'cloud-code.disable');
          await codeTap(t, 'cloud-code.confirm');
          await codeUntil(
            t,
            () =>
                f.inviteCode == null &&
                t
                        .widget<TextButton>(
                          find.byKey(const ValueKey('cloud-code.generate')),
                        )
                        .onPressed !=
                    null,
          );
          expect(
            f.mutations.where((m) => m.path.endsWith('/invite-code')).length,
            3,
          );
          expect(
            t
                .widget<TextButton>(
                  find.byKey(const ValueKey('cloud-code.copy')),
                )
                .onPressed,
            isNull,
          );
          expect(t.takeException(), isNull);
        } finally {
          await finishAccess(t, f);
        }
      },
    );
  }
  testWidgets(
    'code copy is explicit and prior captured generate cannot act after account retirement',
    (t) async {
      final old = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(
        () => AccessDialogFixture.create(page: false),
      ))!;
      f.inviteCodes = true;
      String copied = 'unchanged';
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(SystemChannels.platform, (call) async {
            if (call.method == 'Clipboard.setData')
              copied = (call.arguments as Map)['text'] as String;
            return null;
          });
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = old;
        TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
            .setMockMethodCallHandler(SystemChannels.platform, null);
      });
      try {
        await mountAccess(t, f, compact: true);
        await codeTap(t, 'cloud-code.panel');
        await codeUntil(
          t,
          () =>
              t
                  .widget<TextButton>(
                    find.byKey(const ValueKey('cloud-code.generate')),
                  )
                  .onPressed !=
              null,
        );
        await codeTap(t, 'cloud-code.generate');
        await codeUntil(
          t,
          () =>
              f.inviteCode != null &&
              t
                      .widget<TextButton>(
                        find.byKey(const ValueKey('cloud-code.copy')),
                      )
                      .onPressed !=
                  null,
        );
        expect(copied, 'unchanged');
        await codeTap(t, 'cloud-code.copy');
        expect(copied, f.inviteCode);
        final stale = t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-code.generate')),
            )
            .onPressed!;
        f.auth.switchTo('bob');
        f.auth.switchTo('alice');
        await t.pump();
        stale();
        await t.pump();
        expect(find.byKey(const ValueKey('cloud-code.confirm')), findsNothing);
        expect(f.codeGenerations, 1);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  testWidgets(
    'unknown code generation shows an unconfirmed state and query cannot permit reset or disable',
    (t) async {
      final old = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(
        () => AccessDialogFixture.create(page: false),
      ))!;
      f.inviteCodes = true;
      f.mutationStatus = 503;
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = old;
      });
      try {
        await mountAccess(t, f, compact: true);
        await codeTap(t, 'cloud-code.panel');
        await codeUntil(
          t,
          () =>
              t
                  .widget<TextButton>(
                    find.byKey(const ValueKey('cloud-code.generate')),
                  )
                  .onPressed !=
              null,
        );
        final stale = t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-code.generate')),
            )
            .onPressed!;
        await codeTap(t, 'cloud-code.generate');
        await codeUntil(
          t,
          () =>
              find
                  .byKey(const ValueKey('cloud-code.failure'))
                  .evaluate()
                  .isNotEmpty &&
              find.byKey(const ValueKey('cloud-code.busy')).evaluate().isEmpty,
        );
        expect(find.text('Current code state is unconfirmed'), findsOneWidget);
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('cloud-code.generate')),
              )
              .onPressed,
          isNull,
        );
        stale();
        await t.pump();
        expect(f.codeGenerations, 1);
        await codeTap(t, 'cloud-code.refresh');
        await codeUntil(
          t,
          () =>
              t
                  .widget<TextButton>(
                    find.byKey(const ValueKey('cloud-code.copy')),
                  )
                  .onPressed !=
              null,
        );
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('cloud-code.generate')),
              )
              .onPressed,
          isNull,
        );
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('cloud-code.disable')),
              )
              .onPressed,
          isNull,
        );
        expect(f.codeGenerations, 1);
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  for (final width in [320.0, 430.0, 1100.0]) {
    testWidgets(
      'original code preview requires explicit join and reads the original role at $width',
      (t) async {
        t.view.physicalSize = Size(width, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final old = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(
          () => AccessDialogFixture.create(page: false, role: 'Guest'),
        ))!;
        f.inviteCodes = true;
        var refreshed = 0;
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = old;
        });
        try {
          await t.pumpWidget(
            MaterialApp(
              theme: ThemeData(visualDensity: VisualDensity.compact),
              home: Scaffold(
                body: WorkspaceInviteCodeJoinDialog(
                  authentication: f.auth,
                  service: f.service,
                  onJoined: () async {
                    refreshed++;
                  },
                ),
              ),
            ),
          );
          final field = find.byKey(const ValueKey('cloud-code-join.input'));
          expect(t.widget<TextField>(field).obscureText, isTrue);
          await t.enterText(field, 'AAAAAAAAAAAAAAAA');
          await codeTap(t, 'cloud-code-join.preview');
          await codeUntil(
            t,
            () => find
                .byKey(const ValueKey('cloud-code-join.workspace'))
                .evaluate()
                .isNotEmpty,
          );
          expect(f.mutations, isEmpty);
          final join = find.byKey(const ValueKey('cloud-code-join.accept'));
          final stale = t.widget<TextButton>(join).onPressed!;
          expect(t.getSize(join).height, greaterThanOrEqualTo(44));
          await codeTap(t, 'cloud-code-join.accept');
          await codeUntil(
            t,
            () => find
                .byKey(const ValueKey('cloud-code-join.joined'))
                .evaluate()
                .isNotEmpty,
          );
          expect(refreshed, 1);
          expect(find.textContaining('Guest'), findsOneWidget);
          stale();
          await t.pump();
          expect(f.mutations.length, 1);
          expect(t.takeException(), isNull);
        } finally {
          await finishAccess(t, f);
        }
      },
    );
  }
  testWidgets(
    'changing draft or account retires the earlier original join callback and clears credential field',
    (t) async {
      final old = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(
        () => AccessDialogFixture.create(page: false),
      ))!;
      f.inviteCodes = true;
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = old;
      });
      try {
        await t.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: WorkspaceInviteCodeJoinDialog(
                authentication: f.auth,
                service: f.service,
              ),
            ),
          ),
        );
        final field = find.byKey(const ValueKey('cloud-code-join.input'));
        await t.enterText(field, 'AAAAAAAAAAAAAAAA');
        await codeTap(t, 'cloud-code-join.preview');
        await codeUntil(
          t,
          () => find
              .byKey(const ValueKey('cloud-code-join.accept'))
              .evaluate()
              .isNotEmpty,
        );
        final stale = t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-code-join.accept')),
            )
            .onPressed!;
        await t.enterText(field, 'BBBBBBBBBBBBBBBB');
        stale();
        await t.pump();
        expect(f.mutations, isEmpty);
        expect(
          find.byKey(const ValueKey('cloud-code-join.workspace')),
          findsNothing,
        );
        f.auth.switchTo('bob');
        f.auth.switchTo('alice');
        await t.pump();
        expect(t.widget<TextField>(field).controller!.text, isEmpty);
        stale();
        await t.pump();
        expect(f.mutations, isEmpty);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
}
