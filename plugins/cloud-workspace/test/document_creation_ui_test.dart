import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'catalog_test_support.dart';
import 'document_creation_ui_test_support.dart';
import 'workspace_catalog_ui_test.dart' show until, finish;

Finder key(String value) => find.byKey(ValueKey(value));
Future<DocumentUiFixture> setup(WidgetTester t, {double width = 430}) async {
  final previous = HttpOverrides.current;
  HttpOverrides.global = null;
  addTearDown(() => HttpOverrides.global = previous);
  t.view.physicalSize = Size(width, 900);
  t.view.devicePixelRatio = 1;
  addTearDown(t.view.resetPhysicalSize);
  addTearDown(t.view.resetDevicePixelRatio);
  final f = (await t.runAsync(DocumentUiFixture.create))!;
  addTearDown(() => t.runAsync(f.close));
  return f;
}

Widget app(
  DocumentUiFixture f,
  List<String> opened, {
  String workspace = catalogWorkspace,
  bool editor = true,
}) => MaterialApp(
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
    onOpen: editor ? (p) async => opened.add(p.viewId) : null,
  ),
);
Future<void> dialog(WidgetTester t) async {
  await until(
    t,
    () =>
        key('cloud-pages.create').evaluate().isNotEmpty &&
        t.widget<IconButton>(key('cloud-pages.create')).onPressed != null,
  );
  await t.tap(key('cloud-pages.create'));
  await until(t, () => key('cloud-document.parent').evaluate().isNotEmpty);
}

void main() {
  for (final width in [320.0, 430.0]) {
    testWidgets(
      'create and open uses the original catalog at $width, dark and compact theme',
      (t) async {
        final f = await setup(t, width: width), opened = <String>[];
        await t.pumpWidget(app(f, opened));
        await dialog(t);
        for (final name in [
          'cloud-document.create',
          'cloud-document.close',
          'cloud-document.refresh',
          'cloud-document.parent',
        ]) {
          expect(t.getSize(key(name)).height, greaterThanOrEqualTo(44));
        }
        await t.enterText(key('cloud-document.name'), '新建文档😀');
        await t.pump();
        await t.tap(key('cloud-document.create'));
        await until(t, () => opened.isNotEmpty);
        expect(f.bodies, hasLength(1));
        expect(opened, [f.bodies.single['view_id']]);
        expect(f.bodies.single['view_id'], f.bodies.single['collab_id']);
        expect(
          f.catalog.service.pendingDocumentCreation(catalogWorkspace),
          isNull,
        );
        expect(key('cloud-document.dialog'), findsNothing);
        expect(t.takeException(), isNull);
        await finish(t, f.catalog);
      },
    );
  }
  testWidgets(
    'cancel does not POST and reopening gets fresh original locations',
    (t) async {
      final f = await setup(t), opened = <String>[];
      await t.pumpWidget(app(f, opened));
      await dialog(t);
      await t.enterText(key('cloud-document.name'), 'Cancelled input');
      await t.tap(key('cloud-document.close'));
      await until(t, () => key('cloud-document.dialog').evaluate().isEmpty);
      await dialog(t);
      expect(
        t.widget<TextField>(key('cloud-document.name')).controller!.text,
        isEmpty,
      );
      expect(f.bodies, isEmpty);
      await t.tap(key('cloud-document.close'));
      await until(t, () => key('cloud-document.dialog').evaluate().isEmpty);
      await finish(t, f.catalog);
    },
  );
  testWidgets(
    'broken ACK closes and resumes with GET confirmation and opens same ID',
    (t) async {
      final f = await setup(t), opened = <String>[];
      f.breakAck = true;
      await t.pumpWidget(app(f, opened));
      await dialog(t);
      await t.enterText(key('cloud-document.name'), 'Retained unknown draft');
      await t.pump();
      await t.tap(key('cloud-document.create'));
      await until(t, () => key('cloud-document.check').evaluate().isNotEmpty);
      expect(
        t.widget<Text>(key('cloud-document.pending-name')).data,
        'Retained unknown draft',
      );
      expect(opened, isEmpty);
      await t.tap(key('cloud-document.close'));
      await until(
        t,
        () =>
            key('cloud-pages.creation-resume').evaluate().isNotEmpty &&
            key('cloud-document.dialog').evaluate().isEmpty &&
            t
                    .widget<TextButton>(key('cloud-pages.creation-resume'))
                    .onPressed !=
                null,
      );
      await t.tap(key('cloud-pages.creation-resume'));
      await until(
        t,
        () =>
            key('cloud-document.check').evaluate().isNotEmpty &&
            t.widget<FilledButton>(key('cloud-document.check')).onPressed !=
                null,
      );
      await t.tap(key('cloud-document.check'));
      await until(
        t,
        () =>
            key('cloud-document.open').evaluate().isNotEmpty &&
            t.widget<FilledButton>(key('cloud-document.open')).onPressed !=
                null,
      );
      await t.tap(key('cloud-document.open'));
      await until(t, () => opened.isNotEmpty);
      expect(f.bodies, hasLength(1));
      expect(opened.single, f.bodies.single['view_id']);
      expect(t.takeException(), isNull);
      await finish(t, f.catalog);
    },
  );
  testWidgets(
    'explicit review ends unknown intent without replaying or deleting',
    (t) async {
      final f = await setup(t), opened = <String>[];
      f.publish = false;
      await t.pumpWidget(app(f, opened));
      await dialog(t);
      await t.enterText(key('cloud-document.name'), 'Unpublished draft');
      await t.pump();
      await t.tap(key('cloud-document.create'));
      await until(
        t,
        () => key('cloud-document.review-finish').evaluate().isNotEmpty,
      );
      await t.tap(key('cloud-document.review-finish'));
      await until(
        t,
        () =>
            key('cloud-document.name').evaluate().isNotEmpty &&
            key('cloud-document.parent').evaluate().isNotEmpty &&
            t
                .widget<TextField>(key('cloud-document.name'))
                .controller!
                .text
                .isEmpty &&
            t.widget<TextField>(key('cloud-document.name')).enabled == true,
      );
      expect(
        t.widget<TextField>(key('cloud-document.name')).controller!.text,
        isEmpty,
      );
      expect(f.bodies, hasLength(1));
      expect(opened, isEmpty);
      expect(f.catalog.seen.any((s) => s.startsWith('DELETE ')), isFalse);
      await t.tap(key('cloud-document.close'));
      await until(t, () => key('cloud-document.dialog').evaluate().isEmpty);
      await finish(t, f.catalog);
    },
  );
  testWidgets('workspace rebind retires modal input and prevents old submit', (
    t,
  ) async {
    final f = await setup(t), opened = <String>[];
    await t.pumpWidget(app(f, opened));
    await dialog(t);
    await t.enterText(key('cloud-document.name'), 'Old workspace input');
    await t.pumpWidget(app(f, opened, workspace: catalogOtherWorkspace));
    await t.pump();
    expect(key('cloud-document.name'), findsNothing);
    expect(key('cloud-document.create'), findsNothing);
    expect(
      find.text('Account or source changed. Close and reopen the workspace.'),
      findsOneWidget,
    );
    await t.tap(key('cloud-document.close'));
    await until(t, () => key('cloud-document.dialog').evaluate().isEmpty);
    expect(f.bodies, isEmpty);
    expect(opened, isEmpty);
    expect(t.takeException(), isNull);
    await finish(t, f.catalog);
  });
  for (final guest in [false, true]) {
    testWidgets(
      'no new-document action without ${guest ? 'original write role' : 'document editor'}',
      (t) async {
        final f = await setup(t), opened = <String>[];
        if (guest) f.role = 'Guest';
        await t.pumpWidget(app(f, opened, editor: guest));
        await until(
          t,
          () => key('cloud-page.$catalogChild').evaluate().isNotEmpty,
        );
        expect(key('cloud-pages.create'), findsNothing);
        expect(f.bodies, isEmpty);
        await finish(t, f.catalog);
      },
    );
  }
}
