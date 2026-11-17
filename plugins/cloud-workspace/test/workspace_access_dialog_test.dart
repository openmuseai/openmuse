import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'access_dialog_test_support.dart';

Future<void> settleAccess(WidgetTester t, AccessDialogFixture f) async {
  for (var i = 0; i < 120; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 5)),
    );
    if (!f.c.busy && i > 8) break;
  }
  expect(f.c.busy, isFalse);
  await t.pumpAndSettle();
}

Future<void> finishAccess(WidgetTester t, AccessDialogFixture f) async {
  await t.pumpWidget(const SizedBox.shrink());
  f.service.closeClient();
  await settleAccess(t, f);
}

Future<void> mountAccess(
  WidgetTester t,
  AccessDialogFixture f, {
  bool compact = false,
}) async {
  await t.pumpWidget(
    MaterialApp(
      theme: ThemeData.dark().copyWith(
        visualDensity: compact ? VisualDensity.compact : VisualDensity.standard,
      ),
      home: Scaffold(
        body: Builder(
          builder: (context) => TextButton(
            key: const ValueKey('open'),
            onPressed: () => showDialog<void>(
              context: context,
              builder: (_) => WorkspaceAccessDialog(
                controller: f.c,
                title: 'Original workspace',
              ),
            ),
            child: const Text('Open'),
          ),
        ),
      ),
    ),
  );
  await t.tap(find.byKey(const ValueKey('open')));
  await settleAccess(t, f);
}

void main() {
  for (final width in [320.0, 430.0, 1100.0]) {
    testWidgets(
      'unknown sharing target is visibly fenced with retained draft and44px refresh at $width',
      (t) async {
        t.view.physicalSize = Size(width, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final oldHttp = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(
          () => AccessDialogFixture.create(page: true),
        ))!;
        f.mutationStatus = 503;
        f.applyWrites = false;
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = oldHttp;
        });
        try {
          await mountAccess(t, f, compact: true);
          final email = find.byKey(const ValueKey('cloud-access.email'));
          await t.ensureVisible(email);
          await t.enterText(email, accessEmail);
          final picker = find.byKey(const ValueKey('cloud-access.level'));
          await t.ensureVisible(picker);
          await t.tap(picker);
          await t.pumpAndSettle();
          await t.tap(find.text('Can edit').last);
          await settleAccess(t, f);
          final submit = find.byKey(const ValueKey('cloud-access.submit'));
          await t.ensureVisible(submit);
          await settleAccess(t, f);
          final oldSubmit = t.widget<FilledButton>(submit).onPressed!;
          await t.tap(submit);
          await settleAccess(t, f);
          expect(f.mutations, hasLength(1));
          expect(f.c.failure, 'unconfirmed');
          expect(t.widget<FilledButton>(submit).onPressed, isNull);
          expect(t.widget<TextFormField>(email).controller!.text, accessEmail);
          oldSubmit();
          await settleAccess(t, f);
          expect(f.mutations, hasLength(1));
          final pending = find.byKey(
            ValueKey('cloud-access.pending.share.$accessEmail'),
          );
          await t.ensureVisible(pending);
          await settleAccess(t, f);
          expect(pending, findsOneWidget);
          final refresh = find.byKey(const ValueKey('cloud-access.refresh'));
          await t.ensureVisible(refresh);
          await settleAccess(t, f);
          expect(t.getSize(refresh).height, greaterThanOrEqualTo(44));
          await t.tap(refresh);
          await settleAccess(t, f);
          expect(f.c.failure, 'unconfirmed');
          expect(f.mutations, hasLength(1));
          expect(t.widget<FilledButton>(submit).onPressed, isNull);
          // A different explicit target is still usable; no request is sent by typing.
          await t.ensureVisible(email);
          await t.enterText(email, 'other@example.test');
          await t.pump();
          expect(t.widget<FilledButton>(submit).onPressed, isNotNull);
          expect(f.mutations, hasLength(1));
          expect(t.takeException(), isNull);
        } finally {
          await finishAccess(t, f);
        }
      },
    );
  }

  for (final page in [false, true]) {
    for (final size in [
      const Size(320, 420),
      const Size(430, 900),
      const Size(1100, 700),
    ]) {
      testWidgets('original access dialog page=$page at $size', (t) async {
        t.view.physicalSize = size;
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final previousHttp = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(
          () => AccessDialogFixture.create(page: page),
        ))!;
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = previousHttp;
        });
        try {
          await t.pumpWidget(
            MaterialApp(
              theme: ThemeData.dark(),
              home: Scaffold(
                body: Builder(
                  builder: (context) => TextButton(
                    key: const ValueKey('open'),
                    onPressed: () => showDialog<void>(
                      context: context,
                      builder: (_) => WorkspaceAccessDialog(
                        controller: f.c,
                        title: '原工作区及较长的资源名称',
                      ),
                    ),
                    child: const Text('Original workspace'),
                  ),
                ),
              ),
            ),
          );
          await t.tap(find.byKey(const ValueKey('open')));
          await settleAccess(t, f);
          expect(t.takeException(), isNull);
          expect(f.c.failure, isNull);
          final email = find.byKey(const ValueKey('cloud-access.email'));
          await t.ensureVisible(email);
          await settleAccess(t, f);
          expect(t.getSize(email).width, greaterThanOrEqualTo(160));
          await t.enterText(email, accessEmail);
          final picker = find.byKey(
            ValueKey(page ? 'cloud-access.level' : 'cloud-access.role'),
          );
          await t.ensureVisible(picker);
          await t.tap(picker);
          await t.pumpAndSettle();
          await t.tap(find.text(page ? 'Can edit' : 'Guest').last);
          await settleAccess(t, f);
          final submit = find.byKey(
            ValueKey(page ? 'cloud-access.submit' : 'cloud-access.update-role'),
          );
          await t.ensureVisible(submit);
          await settleAccess(t, f);
          expect(t.getSize(submit).height, greaterThanOrEqualTo(44));
          await t.tap(submit);
          await settleAccess(t, f);
          expect(f.mutations, hasLength(1));
          final body = f.mutations.single.body as Map;
          expect(body[page ? 'access_level' : 'role'], page ? 30 : 'Guest');
          expect(
            f.mutations.single.path,
            page
                ? '/api/sharing/workspace/$accessWorkspace/view'
                : '/api/workspace/$accessWorkspace/member',
          );
          expect(t.takeException(), isNull);
        } finally {
          await finishAccess(t, f);
        }
      });
    }
  }
  testWidgets(
    'resizing preserves the original controller, email draft, selected role and focus without sending a mutation',
    (t) async {
      t.view.physicalSize = const Size(430, 700);
      t.view.devicePixelRatio = 1;
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetDevicePixelRatio);
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(
        () => AccessDialogFixture.create(page: false),
      ))!;
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        final email = find.byKey(const ValueKey('cloud-access.email'));
        await t.enterText(email, accessEmail);
        await t.tap(find.byKey(const ValueKey('cloud-access.role')));
        await t.pumpAndSettle();
        await t.tap(find.text('Guest').last);
        await settleAccess(t, f);
        await t.showKeyboard(email);
        await settleAccess(t, f);
        final field = t.widget<TextFormField>(email);
        final draft = field.controller!;
        FocusNode inputFocus() => t
            .widget<EditableText>(
              find.descendant(of: email, matching: find.byType(EditableText)),
            )
            .focusNode;
        final focus = inputFocus();
        final readCount = f.reads.length;
        expect(focus.hasFocus, isTrue);
        for (final width in [1100.0, 320.0, 430.0]) {
          t.view.physicalSize = Size(width, 700);
          await settleAccess(t, f);
          expect(
            t
                .widget<WorkspaceAccessDialog>(
                  find.byType(WorkspaceAccessDialog),
                )
                .controller,
            same(f.c),
          );
          expect(t.widget<TextFormField>(email).controller, same(draft));
          expect(draft.text, accessEmail);
          expect(
            find.descendant(
              of: find.byKey(const ValueKey('cloud-access.role')),
              matching: find.text('Guest'),
            ),
            findsOneWidget,
          );
          expect(inputFocus(), same(focus));
          expect(focus.hasFocus, isTrue);
          expect(f.c.busy, isFalse);
          expect(f.mutations, isEmpty);
          expect(f.reads.length, readCount);
          expect(t.takeException(), isNull);
        }
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  for (final page in [false, true]) {
    testWidgets(
      'Guest access stays readonly and account loss hides prior grants, page=$page',
      (t) async {
        t.view.physicalSize = const Size(320, 420);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final oldHttp = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = (await t.runAsync(
          () => AccessDialogFixture.create(page: page, role: 'Guest'),
        ))!;
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = oldHttp;
        });
        try {
          await mountAccess(t, f);
          expect(
            find.byKey(const ValueKey('cloud-access.email')),
            findsNothing,
          );
          expect(
            find.byKey(const ValueKey('cloud-access.submit')),
            findsNothing,
          );
          expect(f.mutations, isEmpty);
          f.auth.switchTo('bob');
          f.auth.switchTo('alice');
          await settleAccess(t, f);
          expect(f.c.retired, isTrue);
          expect(f.c.people, isEmpty);
          expect(f.c.members, isEmpty);
          expect(
            find.byKey(const ValueKey('cloud-access.failure')),
            findsOneWidget,
          );
          expect(
            t
                .widget<TextButton>(
                  find.byKey(const ValueKey('cloud-access.refresh')),
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
    'narrow inherited grant management edits its original parent and returns to the same child controller',
    (t) async {
      t.view.physicalSize = const Size(320, 620);
      t.view.devicePixelRatio = 1;
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetDevicePixelRatio);
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = (await t.runAsync(
        () => AccessDialogFixture.create(page: true, inherited: true),
      ))!;
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        final source = find.byKey(
          const ValueKey('cloud-access.source.$accessParent.$accessEmail'),
        );
        await t.ensureVisible(source);
        await settleAccess(t, f);
        expect(t.getSize(source).height, greaterThanOrEqualTo(44));
        await t.tap(source);
        final parent = find.byWidgetPredicate(
          (w) =>
              w is WorkspaceAccessDialog && w.controller.viewId == accessParent,
        );
        for (var i = 0; i < 120; i++) {
          await t.pump(const Duration(milliseconds: 20));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 5)),
          );
          if (parent.evaluate().length == 1 &&
              !t.widget<WorkspaceAccessDialog>(parent).controller.busy)
            break;
        }
        expect(parent, findsOneWidget);
        final control = find.descendant(
          of: parent,
          matching: find.byKey(
            const ValueKey('cloud-access.person-permission.$accessEmail'),
          ),
        );
        await t.ensureVisible(control);
        await t.pumpAndSettle();
        await t.tap(control);
        await t.pumpAndSettle();
        await t.tap(find.text('Can edit').last);
        await settleAccess(t, f);
        expect(f.mutations, hasLength(1));
        expect((f.mutations.single.body as Map)['view_id'], accessParent);
        final close = find.descendant(
          of: parent,
          matching: find.byKey(const ValueKey('cloud-access.close')),
        );
        await t.tap(close);
        await settleAccess(t, f);
        expect(
          t
              .widget<WorkspaceAccessDialog>(find.byType(WorkspaceAccessDialog))
              .controller,
          same(f.c),
        );
        expect(f.c.viewId, accessView);
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
}
