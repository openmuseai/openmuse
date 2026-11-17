import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'catalog_test_support.dart';

Future<void> until(WidgetTester t, bool Function() ready) async {
  for (var i = 0; i < 200; i++) {
    await t.pump(const Duration(milliseconds: 50));
    if (ready()) return;
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 10)),
    );
  }
  fail('Catalog UI did not reach its bounded state');
}

Future<void> finish(WidgetTester t, CatalogFixture f) async {
  await t.pumpWidget(const SizedBox());
  f.service.closeClient();
  await t.runAsync(
    () => Future<void>.delayed(const Duration(milliseconds: 20)),
  );
  await t.pump();
}

void main() {
  for (final width in [320.0, 430.0]) {
    testWidgets(
      'original catalog collapse/search/clear and long names at $width',
      (t) async {
        final previous = HttpOverrides.current;
        HttpOverrides.global = null;
        addTearDown(() => HttpOverrides.global = previous);
        t.view.physicalSize = Size(width, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final f = (await t.runAsync(CatalogFixture.create))!;
        addTearDown(() => t.runAsync(f.close));
        f.childName =
            'Nested original database with a long name for bounded original resource navigation';
        final opened = <String>[];
        await t.pumpWidget(
          MaterialApp(
            theme: ThemeData(
              visualDensity: VisualDensity.compact,
              materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
            ),
            home: CloudWorkspacePages(
              authentication: f.auth,
              service: f.service,
              workspaceId: catalogWorkspace,
              title: 'An original shared workspace with a long title',
              onOpen: (page) async => opened.add(page.viewId),
            ),
          ),
        );
        final child = find.byKey(const ValueKey('cloud-page.$catalogChild'));
        await until(t, () => child.evaluate().isNotEmpty);
        for (final key in [
          'cloud-page.share.$catalogChild',
          'cloud-page.toggle.$catalogParent',
          'cloud-pages.refresh',
        ]) {
          expect(
            t.getSize(find.byKey(ValueKey(key))).height,
            greaterThanOrEqualTo(44),
          );
        }
        expect(find.text('Parent document'), findsNWidgets(3));
        await t.tap(
          find.byKey(const ValueKey('cloud-page.toggle.$catalogParent')),
        );
        await t.pump();
        expect(child, findsNothing);
        await t.enterText(
          find.byKey(const ValueKey('cloud-pages.search')),
          'ORIGINAL DATABASE',
        );
        await t.pump();
        expect(child, findsOneWidget);
        expect(
          find.byKey(const ValueKey('cloud-page.$catalogSibling')),
          findsNothing,
        );
        await t.tap(find.byKey(const ValueKey('cloud-pages.clear-search')));
        await t.pump();
        expect(child, findsNothing);
        await t.tap(
          find.byKey(const ValueKey('cloud-page.toggle.$catalogParent')),
        );
        await t.pump();
        expect(child, findsOneWidget);
        expect(t.takeException(), isNull);
        await t.tap(child);
        await t.pump();
        expect(opened, [catalogChild]);
        await finish(t, f);
      },
    );
  }

  testWidgets(
    'same widget State rebinds workspace and rejects the old delayed catalog',
    (t) async {
      final previous = HttpOverrides.current;
      HttpOverrides.global = null;
      addTearDown(() => HttpOverrides.global = previous);
      final f = (await t.runAsync(CatalogFixture.create))!;
      addTearDown(() => t.runAsync(f.close));
      final arrived = Completer<void>(), release = Completer<void>();
      f.intercept = (r) async {
        if (r.uri.path != '/api/workspace/$catalogWorkspace/folder')
          return false;
        arrived.complete();
        await release.future;
        final old = f.folder(catalogWorkspace);
        old['children'] = [
          {
            'view_id': catalogSibling,
            'name': 'OLD ACCOUNT RESPONSE',
            'layout': 0,
            'children': [],
          },
        ];
        await f.reply(r, old);
        return true;
      };
      Widget app(String workspace) => MaterialApp(
        home: CloudWorkspacePages(
          authentication: f.auth,
          service: f.service,
          workspaceId: workspace,
          title: workspace == catalogWorkspace
              ? 'Old workspace'
              : 'New workspace',
        ),
      );
      await t.pumpWidget(app(catalogWorkspace));
      await until(t, () => arrived.isCompleted);
      await t.pumpWidget(app(catalogOtherWorkspace));
      await until(
        t,
        () => find
            .byKey(const ValueKey('cloud-page.$catalogChild'))
            .evaluate()
            .isNotEmpty,
      );
      release.complete();
      await until(
        t,
        () => f.seen.where((s) => s.endsWith('/database')).length == 2,
      );
      await t.pump();
      expect(find.text('New workspace'), findsOneWidget);
      expect(find.text('OLD ACCOUNT RESPONSE'), findsNothing);
      expect(
        find.byKey(const ValueKey('cloud-page.$catalogChild')),
        findsOneWidget,
      );
      expect(t.takeException(), isNull);
      await finish(t, f);
    },
  );

  testWidgets(
    'account retirement clears search and visible resources permanently',
    (t) async {
      final previous = HttpOverrides.current;
      HttpOverrides.global = null;
      addTearDown(() => HttpOverrides.global = previous);
      final f = (await t.runAsync(CatalogFixture.create))!;
      addTearDown(() => t.runAsync(f.close));
      await t.pumpWidget(
        MaterialApp(
          home: CloudWorkspacePages(
            authentication: f.auth,
            service: f.service,
            workspaceId: catalogWorkspace,
            title: 'Workspace',
          ),
        ),
      );
      await until(
        t,
        () => find
            .byKey(const ValueKey('cloud-page.$catalogChild'))
            .evaluate()
            .isNotEmpty,
      );
      await t.enterText(
        find.byKey(const ValueKey('cloud-pages.search')),
        'private query',
      );
      f.auth.switchTo('bob');
      f.auth.switchTo('alice');
      await t.pump();
      expect(find.byType(ListTile), findsNothing);
      expect(
        t
            .widget<TextField>(find.byKey(const ValueKey('cloud-pages.search')))
            .controller!
            .text,
        isEmpty,
      );
      expect(
        t
            .widget<IconButton>(
              find.byKey(const ValueKey('cloud-pages.refresh')),
            )
            .onPressed,
        isNull,
      );
      expect(
        find.text('Account changed. Reopen the workspace.'),
        findsOneWidget,
      );
      expect(t.takeException(), isNull);
      await finish(t, f);
    },
  );
}
