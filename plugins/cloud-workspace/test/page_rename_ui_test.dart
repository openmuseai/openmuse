import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'catalog_test_support.dart';
import 'page_rename_test_support.dart';
import 'workspace_catalog_ui_test.dart' show until, finish;

Finder key(String name) => find.byKey(ValueKey(name));
Future<RenameUiFixture> setup(WidgetTester t, {double width = 430}) async {
  final previous = HttpOverrides.current;
  HttpOverrides.global = null;
  addTearDown(() => HttpOverrides.global = previous);
  t.view.physicalSize = Size(width, 900);
  t.view.devicePixelRatio = 1;
  addTearDown(t.view.resetPhysicalSize);
  addTearDown(t.view.resetDevicePixelRatio);
  final f = (await t.runAsync(RenameUiFixture.create))!;
  addTearDown(() => t.runAsync(f.close));
  return f;
}

Widget app(RenameUiFixture f, {String workspace = catalogWorkspace}) =>
    MaterialApp(
      theme: ThemeData(
        brightness: Brightness.dark,
        visualDensity: VisualDensity.compact,
        materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
      ),
      home: CloudWorkspacePages(
        authentication: f.catalog.auth,
        service: f.catalog.service,
        workspaceId: workspace,
        title: 'Original workspace',
      ),
    );
Future<void> dialog(WidgetTester t) async {
  await until(
    t,
    () =>
        key('cloud-page.rename.$catalogParent').evaluate().isNotEmpty &&
        t
                .widget<IconButton>(key('cloud-page.rename.$catalogParent'))
                .onPressed !=
            null,
  );
  await t.tap(key('cloud-page.rename.$catalogParent'));
  await until(
    t,
    () =>
        key('cloud-rename.name').evaluate().isNotEmpty &&
        t.widget<TextField>(key('cloud-rename.name')).enabled == true,
  );
}

void main() {
  testWidgets(
    'Unicode title counter matches SiYuan code points and does not truncate emoji',
    (t) async {
      final f = await setup(t, width: 320);
      await t.pumpWidget(app(f));
      await dialog(t);
      final title = '${'😀' * 511}/';
      await t.enterText(key('cloud-rename.name'), title);
      await t.pump();
      expect(
        t.widget<TextField>(key('cloud-rename.name')).controller!.text,
        title,
      );
      expect(t.widget<Text>(key('cloud-rename.name-counter')).data, '512/512');
      expect(
        t.widget<FilledButton>(key('cloud-rename.submit')).onPressed,
        isNotNull,
      );
      await t.tap(key('cloud-rename.close'));
      await until(t, () => key('cloud-rename.dialog').evaluate().isEmpty);
      expect(f.writes, isEmpty);
      expect(t.takeException(), isNull);
      await finish(t, f.catalog);
    },
  );

  for (final width in [320.0, 430.0]) {
    testWidgets(
      'original rename and cancel at $width preserve identity and readable actions',
      (t) async {
        final f = await setup(t, width: width);
        await t.pumpWidget(app(f));
        await dialog(t);
        expect(
          t.widget<TextField>(key('cloud-rename.name')).controller!.text,
          'Parent document',
        );
        for (final name in [
          'cloud-rename.submit',
          'cloud-rename.close',
          'cloud-rename.refresh',
        ]) {
          expect(t.getSize(key(name)).height, greaterThanOrEqualTo(44));
        }
        await t.enterText(key('cloud-rename.name'), 'Cancelled input');
        await t.pump();
        await t.tap(key('cloud-rename.close'));
        await until(t, () => key('cloud-rename.dialog').evaluate().isEmpty);
        expect(f.writes, isEmpty);
        await dialog(t);
        await t.enterText(key('cloud-rename.name'), '原目录改名😀');
        await t.pump();
        await t.tap(key('cloud-rename.submit'));
        await until(
          t,
          () =>
              key('cloud-rename.dialog').evaluate().isEmpty &&
              find.text('原目录改名😀').evaluate().isNotEmpty,
        );
        expect(f.writes, [
          {'name': '原目录改名😀'},
        ]);
        expect(key('cloud-page.$catalogParent'), findsOneWidget);
        expect(
          f.catalog.service.pendingPageRename(catalogWorkspace, catalogParent),
          isNull,
        );
        expect(key('cloud-page.rename.$catalogChild'), findsNothing);
        expect(t.takeException(), isNull);
        await finish(t, f.catalog);
      },
    );
  }
  testWidgets('unknown rename closes, resumes and confirms with GET only', (
    t,
  ) async {
    final f = await setup(t);
    f.breakAck = true;
    await t.pumpWidget(app(f));
    await dialog(t);
    await t.enterText(key('cloud-rename.name'), 'Retained desired title');
    await t.pump();
    await t.tap(key('cloud-rename.submit'));
    await until(
      t,
      () =>
          key('cloud-rename.check').evaluate().isNotEmpty &&
          t.widget<FilledButton>(key('cloud-rename.check')).onPressed != null,
    );
    expect(
      t.widget<Text>(key('cloud-rename.submitted')).data,
      'Retained desired title',
    );
    await t.tap(key('cloud-rename.close'));
    await until(
      t,
      () =>
          key('cloud-rename.dialog').evaluate().isEmpty &&
          key('cloud-page.rename.$catalogParent').evaluate().isNotEmpty &&
          t
                  .widget<IconButton>(key('cloud-page.rename.$catalogParent'))
                  .onPressed !=
              null,
    );
    await t.tap(key('cloud-page.rename.$catalogParent'));
    await until(
      t,
      () =>
          key('cloud-rename.check').evaluate().isNotEmpty &&
          t.widget<FilledButton>(key('cloud-rename.check')).onPressed != null,
    );
    await t.tap(key('cloud-rename.check'));
    await until(
      t,
      () =>
          key('cloud-rename.observed').evaluate().isNotEmpty &&
          find
              .text('The name is confirmed in the original catalog.')
              .evaluate()
              .isNotEmpty &&
          t.widget<TextButton>(key('cloud-rename.close')).onPressed != null,
    );
    expect(
      t.widget<Text>(key('cloud-rename.observed')).data,
      'Retained desired title',
    );
    await t.tap(key('cloud-rename.close'));
    await until(t, () => key('cloud-rename.dialog').evaluate().isEmpty);
    expect(f.writes, hasLength(1));
    expect(
      f.catalog.service.pendingPageRename(catalogWorkspace, catalogParent),
      isNull,
    );
    expect(t.takeException(), isNull);
    await finish(t, f.catalog);
  });
  testWidgets(
    'explicit review reloads actual current name instead of silently retrying the draft',
    (t) async {
      final f = await setup(t);
      f.publish = false;
      await t.pumpWidget(app(f));
      await dialog(t);
      await t.enterText(key('cloud-rename.name'), 'Desired but unpublished');
      await t.pump();
      await t.tap(key('cloud-rename.submit'));
      await until(
        t,
        () =>
            key('cloud-rename.review-finish').evaluate().isNotEmpty &&
            t.widget<TextButton>(key('cloud-rename.review-finish')).onPressed !=
                null,
      );
      await t.tap(key('cloud-rename.review-finish'));
      await until(
        t,
        () =>
            key('cloud-rename.name').evaluate().isNotEmpty &&
            t.widget<TextField>(key('cloud-rename.name')).enabled == true &&
            t.widget<TextField>(key('cloud-rename.name')).controller!.text ==
                'Parent document',
      );
      expect(
        t.widget<FilledButton>(key('cloud-rename.submit')).onPressed,
        isNull,
      );
      expect(f.writes, hasLength(1));
      await t.tap(key('cloud-rename.close'));
      await until(t, () => key('cloud-rename.dialog').evaluate().isEmpty);
      await finish(t, f.catalog);
    },
  );
  testWidgets(
    'same State workspace rebind retires the modal without old writes or name leakage',
    (t) async {
      final f = await setup(t);
      await t.pumpWidget(app(f));
      await dialog(t);
      await t.enterText(
        key('cloud-rename.name'),
        'Private old workspace input',
      );
      await t.pumpWidget(app(f, workspace: catalogOtherWorkspace));
      await t.pump();
      expect(key('cloud-rename.name'), findsNothing);
      expect(key('cloud-rename.submit'), findsNothing);
      expect(
        find.text('Account or source changed. Close and reopen.'),
        findsOneWidget,
      );
      await t.tap(key('cloud-rename.close'));
      await until(t, () => key('cloud-rename.dialog').evaluate().isEmpty);
      expect(f.writes, isEmpty);
      expect(t.takeException(), isNull);
      await finish(t, f.catalog);
    },
  );
  testWidgets(
    'original Guest catalog offers sharing but no Folder rename action',
    (t) async {
      final f = await setup(t);
      f.role = 'Guest';
      await t.pumpWidget(app(f));
      await until(
        t,
        () => key('cloud-page.$catalogParent').evaluate().isNotEmpty,
      );
      expect(key('cloud-page.rename.$catalogParent'), findsNothing);
      expect(key('cloud-page.share.$catalogParent'), findsOneWidget);
      expect(f.writes, isEmpty);
      await finish(t, f.catalog);
    },
  );
}
