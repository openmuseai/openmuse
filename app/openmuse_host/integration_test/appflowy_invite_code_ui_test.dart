import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart' as ae;
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart' as doc;
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original workspace code joins existing document and database collaboration',
    (t) async {
      HttpOverrides.global = null;
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      expect(fixture['origin'], 'http://127.0.0.1:18001');
      final db = fixture['database'] as Map, users = fixture['users'] as Map;
      final wid = fixture['workspaceId'] as String,
          rootId = db['rootId'] as String;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-invite-code-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_INVITE_CODE_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      final transports = <String, _CodeLossClient>{};
      for (final name in ['alice', 'outsider']) {
        final auth = GoTrueAuthenticationController(
          provider: GoTrueHttpClient(
            config: GoTrueClientConfig(
              origin: Uri.parse(fixture['authOrigin']),
              allowInsecureLoopback: true,
            ),
          ),
          store: MemoryAuthSessionStore(),
          bootstrapper: AppFlowyAccountBootstrapper(
            cloudOrigin: Uri.parse(fixture['origin']),
            allowInsecureLoopback: true,
          ),
        );
        await t.runAsync(
          () => auth.signInWithPassword(
            users[name]['email'],
            'SmartdocLocal123!',
          ),
        );
        expect(auth.snapshot.isAuthenticated, isTrue);
        late final AppFlowyDocumentWorkspace online;
        final transport = _CodeLossClient(HttpClient());
        transports[name] = transport;
        final service = AppFlowyCloudWorkspaceService(
          baseUri: Uri.parse(fixture['origin']),
          accessToken: auth.accessToken,
          refreshAccessToken: () => auth.accessToken(forceRefresh: true),
          accountSubject: () => auth.snapshot.isAuthenticated
              ? auth.snapshot.identity?.subject
              : null,
          deviceId: 'code-outcome-$name-${fixture['run']}',
          client: transport,
          allowHttpForTesting: true,
        );
        final cloud = OpenMuseCloudWorkspacePlugin(
          service: service,
          authentication: auth,
          cloudOrigin: Uri.parse(fixture['origin']),
          deviceId: 'database-ui-$name-${fixture['run']}',
          allowInsecureLoopback: true,
          onOpenDocument: (page) => online.openDocument(page),
          onOpenDatabase: (page) => online.databases.openDatabase(page),
        );
        online = AppFlowyDocumentWorkspace(
          authentication: auth,
          service: cloud.service,
          checkpointRoot: root,
        );
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => null,
          ),
        );
        registry.install(cloud);
        final plugins = createOpenMuseBuiltInPlugins(cloudDocuments: online)
            .where(
              (p) => {
                'io.openmuse.smart-document',
                'io.openmuse.dataset',
              }.contains(p.descriptor.id),
            )
            .toList();
        for (final plugin in plugins) {
          registry.install(plugin);
        }
        await t.runAsync(() async {
          await registry.activate(cloud.descriptor.id);
          for (final plugin in plugins) {
            await registry.activate(plugin.descriptor.id);
          }
        });
        final workspace = LocalWorkspaceController(
          rootPath: root.path,
          initialResources: const [],
        );
        final center = await t.runAsync(
          () => NativeResourceCenter.open(
            scope: auth.snapshot.identity!.subject,
            deviceRef: 'database-ui-$name',
            deviceLabel: 'Mac',
            registry: registry,
            storageRoot: Directory('${root.path}/$name'),
          ),
        );
        online.attach(center!.controller);
        actors.add(
          _Actor(auth, cloud, online, registry, plugins, workspace, center),
        );
      }
      final active = ValueNotifier<int>(0);
      final checks = <String>[];
      Future<void> until(bool Function() ready, String reason) async {
        for (var i = 0; i < 300; i++) {
          await t.pump(const Duration(milliseconds: 100));
          if (ready()) return;
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
        }
        fail(
          '$reason: ${t.widgetList<Text>(find.byType(Text)).map((v) => v.data).join(' | ')}',
        );
      }

      Future<T> domain<T>(Future<T> Function() action) async {
        var done = false;
        T? value;
        Object? error;
        StackTrace? stack;
        await t.runAsync(() async {
          action().then(
            (v) {
              value = v;
              done = true;
            },
            onError: (Object e, StackTrace s) {
              error = e;
              stack = s;
              done = true;
            },
          );
        });
        await until(() => done, 'domain settled');
        if (error != null) Error.throwWithStackTrace(error!, stack!);
        return value as T;
      }

      Future<void> tap(String key) async {
        final finder = find.byKey(ValueKey(key));
        if (key.startsWith('cloud-code.') &&
            !key.startsWith('cloud-code.confirm')) {
          final contents = find.byKey(const ValueKey('cloud-access.contents'));
          for (
            var i = 0;
            i < 20 &&
                finder.evaluate().isEmpty &&
                contents.evaluate().isNotEmpty;
            i++
          ) {
            await t.drag(contents, const Offset(0, -180));
            await t.pump(const Duration(milliseconds: 100));
            await t.runAsync(
              () => Future<void>.delayed(const Duration(milliseconds: 5)),
            );
          }
        }
        if (key.startsWith('cloud-access.member-menu.') &&
            finder.evaluate().isEmpty) {
          final contents = find.byKey(const ValueKey('cloud-access.contents'));
          for (
            var i = 0;
            i < 20 &&
                finder.evaluate().isEmpty &&
                contents.evaluate().isNotEmpty;
            i++
          ) {
            await t.drag(contents, const Offset(0, 240));
            await t.pump(const Duration(milliseconds: 100));
            await t.runAsync(
              () => Future<void>.delayed(const Duration(milliseconds: 5)),
            );
          }
        }
        await until(() => finder.evaluate().isNotEmpty, key);
        if (key.startsWith('cloud-access.person-level.')) {
          // The short popup already lays out all four choices. Forcing
          // ensureVisible interferes with DropdownRoute's selected-item offset.
          await t.pumpAndSettle();
          final boundary = t.renderObject<RenderRepaintBoundary>(
            find.byKey(ValueKey('database-host-${active.value}')),
          );
          final image = await boundary.toImage(pixelRatio: 1);
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '${evidence.path}/permission-menu-${key.split('.').last}.png',
          ).writeAsBytes(data!.buffer.asUint8List());
          image.dispose();
        } else {
          await t.ensureVisible(finder);
        }
        final cell = t.widget(finder);
        final target =
            key.startsWith('cloud-access.person-level.') ||
                cell is DropdownMenuItem ||
                (cell is InkWell && cell.onTap == null)
            ? find.descendant(of: finder, matching: find.byType(Text)).first
            : finder;
        await until(
          () => target.hitTestable().evaluate().length == 1,
          '$key receives pointer events',
        );
        await t.tap(target);
        await t.pump(const Duration(milliseconds: 350));
      }

      Future<void> showActor(int index) async {
        FocusManager.instance.primaryFocus?.unfocus();
        active.value = index;
        await t.pump(const Duration(milliseconds: 400));
      }

      Future<void> catalog({bool refresh = false}) async {
        await tap('cloud-workspace.open');
        if (refresh) await tap('cloud-workspace.refresh');
        await tap('cloud-workspace.$wid');
      }

      Future<void> idle() => until(
        () =>
            find.byKey(const ValueKey('cloud-access.busy')).evaluate().isEmpty,
        'sharing settled',
      );
      Future<DatasetController> openDatabase({bool refresh = false}) async {
        await catalog(refresh: refresh);
        await tap('cloud-page.${db['viewId']}');
        await until(
          () => find.byType(DatasetSurface).evaluate().isNotEmpty,
          'actual Host database',
        );
        final surface = t.widget<DatasetSurface>(find.byType(DatasetSurface));
        expect(surface.viewId, db['viewId']);
        expect(surface.controller.objectId, rootId);
        return surface.controller;
      }

      Future<void> edit(String row, String field, String text) async {
        await tap('dataset.cell.$row.$field');
        await until(
          () => find
              .byKey(const ValueKey('dataset.cell-input'))
              .evaluate()
              .isNotEmpty,
          'cell dialog',
        );
        await t.enterText(
          find.byKey(const ValueKey('dataset.cell-input')),
          text,
        );
        await tap('dataset.cell-apply');
      }

      Future<void> shot(String name) async {
        await t.pump(const Duration(milliseconds: 250));
        final surface = t.renderObject<RenderRepaintBoundary>(
          find.byKey(ValueKey('database-host-${active.value}')),
        );
        final image = await surface.toImage(pixelRatio: 1);
        final data = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(data!.buffer.asUint8List());
        image.dispose();
      }

      await t.pumpWidget(
        ValueListenableBuilder<int>(
          valueListenable: active,
          builder: (_, index, _) => Stack(
            alignment: Alignment.topLeft,
            children: [
              for (var i = 0; i < actors.length; i++)
                Offstage(
                  offstage: i != index,
                  child: RepaintBoundary(
                    key: ValueKey('database-host-$i'),
                    child: OpenMuseHostApp(
                      registry: actors[i].registry,
                      workspace: actors[i].workspace,
                      resources: actors[i].centers,
                    ),
                  ),
                ),
            ],
          ),
        ),
      );
      final secrets = <String>[];
      Future<void> retainCode(String code) async {
        secrets.add(code);
        final private = File(
          '${File(fixturePath).parent.path}/invite-codes.private.json',
        );
        if (!await private.exists()) {
          await private.create();
          final permission = await Process.run('/bin/chmod', [
            '600',
            private.path,
          ]);
          expect(permission.exitCode, 0);
        }
        await private.writeAsString(jsonEncode({'codes': secrets}));
      }

      Future<void> denied(
        Future<dynamic> Function() action,
        String reason,
      ) async {
        var rejected = false;
        try {
          await domain(action);
        } on Exception {
          rejected = true;
        }
        expect(rejected, isTrue, reason: reason);
      }

      final owner = await openDatabase();
      final rowId = owner.views.first.rows.first.id;
      final fieldId = owner.views.first.columns.first.id;
      await edit(rowId, fieldId, '原群组邀请前的原行😀');
      await domain(owner.synchronize);
      final fullSize = t.view.physicalSize;
      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      addTearDown(t.view.resetPhysicalSize);
      await t.pump(const Duration(milliseconds: 350));
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.members.$wid');
      await tap('cloud-code.panel');
      expect(
        t
            .widget<Text>(
              find.byKey(const ValueKey('cloud-code.validity-label')),
            )
            .data,
        anyOf('生成 / 重置后的有效期', 'Validity for the next generated code'),
      );
      await until(
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
        'original code read',
      );
      final oldGenerate = t
          .widget<TextButton>(find.byKey(const ValueKey('cloud-code.generate')))
          .onPressed!;
      expect(
        t.getSize(find.byKey(const ValueKey('cloud-code.generate'))).height,
        greaterThanOrEqualTo(44),
      );
      await tap('cloud-code.generate');
      await until(
        () =>
            find
                .byKey(const ValueKey('cloud-code.notice'))
                .evaluate()
                .isNotEmpty &&
            t
                    .widget<TextButton>(
                      find.byKey(const ValueKey('cloud-code.copy')),
                    )
                    .onPressed !=
                null,
        'formal original code generation',
      );
      final generated = await domain(
        () => actors[0].cloud.service.workspaceInviteCode(wid),
      );
      expect(generated.code != null, isTrue);
      final firstCode = generated.code!;
      await retainCode(firstCode);
      expect(find.text(firstCode), findsNothing);
      expect(
        t.getSize(find.byKey(const ValueKey('database-host-0'))).width,
        430,
      );
      checks.add(
        'native430-original-Owner-generates-private-code-with44px-action-without-displaying-credential',
      );
      // The independent read above intentionally retired the old observation.
      await tap('cloud-code.refresh');
      await until(
        () =>
            t
                .widget<TextButton>(
                  find.byKey(const ValueKey('cloud-code.copy')),
                )
                .onPressed !=
            null,
        'current code read for explicit copy',
      );
      await shot('07-formal-owner-code-generation-at-430');
      final previousClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await tap('cloud-code.copy');
        expect(
          (await Clipboard.getData(Clipboard.kTextPlain))?.text == firstCode,
          isTrue,
        );
      } finally {
        await Clipboard.setData(
          ClipboardData(text: previousClipboard?.text ?? ''),
        );
      }
      oldGenerate();
      await t.pump(const Duration(milliseconds: 250));
      expect(
        (await domain(
              () => actors[0].cloud.service.workspaceInviteCode(wid),
            )).code ==
            firstCode,
        isTrue,
      );
      checks.add(
        'actual-system-clipboard-copy-is-explicit-and-captured-old-generate-cannot-reset-current-code',
      );
      await tap('cloud-access.close');
      await until(
        () => find.byType(BackButton).evaluate().length == 1,
        'owner chooser back',
      );
      await t.tap(find.byType(BackButton));
      await t.pump(const Duration(milliseconds: 350));
      await showActor(1);
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'code preview is not membership',
      );
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.join-code');
      final input = find.byKey(const ValueKey('cloud-code-join.input'));
      expect(t.widget<TextField>(input).obscureText, isTrue);
      await t.enterText(input, firstCode);
      await tap('cloud-code-join.preview');
      await until(
        () => find
            .byKey(const ValueKey('cloud-code-join.workspace'))
            .evaluate()
            .isNotEmpty,
        'original authenticated code preview',
      );
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'preview still grants no membership',
      );
      await shot('08-original-workspace-code-preview-masked');
      checks.add(
        'native430-original-workspace-preview-keeps-code-masked-and-grants-no-membership',
      );
      final oldJoin = t
          .widget<TextButton>(
            find.byKey(const ValueKey('cloud-code-join.accept')),
          )
          .onPressed!;
      await tap('cloud-code-join.accept');
      await until(
        () => find
            .byKey(const ValueKey('cloud-code-join.joined'))
            .evaluate()
            .isNotEmpty,
        'original code membership confirmed',
      );
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      oldJoin();
      await t.pump(const Duration(milliseconds: 200));
      await shot('09-original-code-join-confirmed');
      checks.add(
        'formal-code-acceptance-reads-original-Member-role-and-cannot-repeat-captured-join',
      );
      await tap('cloud-code-join.close');
      await until(
        () => actors[1].cloud.controller.snapshot.workspaces.any(
          (w) => w.workspaceRef == wid,
        ),
        'original directory refreshed by code acceptance',
      );
      checks.add(
        'original-code-join-refreshes-existing-directory-without-a-parallel-workspace-model',
      );
      t.view.physicalSize = fullSize;
      await t.pump(const Duration(milliseconds: 350));
      await tap('cloud-workspace.$wid');
      await tap('cloud-page.${db['parentViewId']}');
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'original parent document after group acceptance',
      );
      final parent = t
          .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .controller;
      expect(parent.objectId, db['parentViewId']);
      await until(() => parent.canWrite, 'member can edit original document');
      final paragraph = find.byType(ae.AppFlowyRichText).first;
      await until(
        () => paragraph.hitTestable().evaluate().isNotEmpty,
        'native paragraph',
      );
      await t.tap(paragraph);
      await t.pump(const Duration(milliseconds: 200));
      final clipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '受邀成员写入原正文😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => parent.blocks.any(
            (b) => b.delta?.toPlainText().contains('受邀成员写入原正文') ?? false,
          ),
          'native member document edit',
        );
      } finally {
        await Clipboard.setData(ClipboardData(text: clipboard?.text ?? ''));
      }
      await domain(parent.synchronize);
      await shot('03-invited-member-edits-the-original-document');
      checks.add(
        'invited-member-opens-and-edits-the-original-document-through-the-existing-registry',
      );
      await showActor(0);
      await catalog();
      await tap('cloud-page.${db['parentViewId']}');
      await until(
        () => find.byType(doc.SmartDocumentView).evaluate().isNotEmpty,
        'owner original document',
      );
      final ownerParent = t
          .widget<doc.SmartDocumentView>(find.byType(doc.SmartDocumentView))
          .controller;
      await until(
        () => ownerParent.blocks.any(
          (b) => b.delta?.toPlainText().contains('受邀成员写入原正文') ?? false,
        ),
        'owner receives invited member original document update',
      );
      checks.add(
        'owner-reads-invited-member-content-from-the-same-original-document-object',
      );
      await showActor(1);
      final member = await openDatabase(refresh: true);
      expect(member.objectId, owner.objectId);
      expect(member.canWrite, isTrue);
      await edit(rowId, fieldId, '受邀成员更新原行😀');
      await domain(member.synchronize);
      await until(
        () =>
            owner.views.first.rows
                .singleWhere((r) => r.id == rowId)
                .cells[fieldId]!
                .value ==
            '受邀成员更新原行😀',
        'owner original row convergence',
      );
      await shot('04-invited-member-shares-original-root-and-row');
      checks.add(
        'invited-member-edits-the-original-root-row-and-owner-converges-without-copies',
      );

      await showActor(0);
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.members.$wid');
      await tap('cloud-code.panel');
      await until(
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
        'Owner code before reset',
      );
      await tap('cloud-code.generate');
      await tap('cloud-code.confirm-cancel');
      expect(
        (await domain(
              () => actors[0].cloud.service.workspaceInviteCode(wid),
            )).code ==
            firstCode,
        isTrue,
      );
      await tap('cloud-code.refresh');
      await until(
        () =>
            t
                .widget<TextButton>(
                  find.byKey(const ValueKey('cloud-code.generate')),
                )
                .onPressed !=
            null,
        'fresh reset observation',
      );
      await tap('cloud-code.generate');
      await tap('cloud-code.confirm');
      await until(
        () =>
            find
                .byKey(const ValueKey('cloud-code.notice'))
                .evaluate()
                .isNotEmpty &&
            t
                    .widget<TextButton>(
                      find.byKey(const ValueKey('cloud-code.copy')),
                    )
                    .onPressed !=
                null,
        'formal reset confirmed',
      );
      final secondCode = (await domain(
        () => actors[0].cloud.service.workspaceInviteCode(wid),
      )).code!;
      await retainCode(secondCode);
      expect(secondCode != firstCode, isTrue);
      await denied(
        () => actors[1].cloud.service.previewWorkspaceInviteCode(firstCode),
        'old code invalid after original reset',
      );
      final stalePreview = await domain(
        () => actors[1].cloud.service.previewWorkspaceInviteCode(secondCode),
      );
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      checks.add(
        'formal-reset-cancel-does-not-mutate-and-confirmed-reset-retires-old-code-without-removing-member',
      );
      await tap('cloud-code.refresh');
      await until(
        () =>
            t
                .widget<TextButton>(
                  find.byKey(const ValueKey('cloud-code.disable')),
                )
                .onPressed !=
            null,
        'fresh disable observation',
      );
      await tap('cloud-code.disable');
      await tap('cloud-code.confirm');
      await until(
        () =>
            find
                .byKey(const ValueKey('cloud-code.notice'))
                .evaluate()
                .isNotEmpty &&
            t
                    .widget<TextButton>(
                      find.byKey(const ValueKey('cloud-code.copy')),
                    )
                    .onPressed ==
                null,
        'formal code disabled',
      );
      expect(
        (await domain(
              () => actors[0].cloud.service.workspaceInviteCode(wid),
            )).code ==
            null,
        isTrue,
      );
      await denied(
        () => actors[1].cloud.service.joinWorkspaceByInviteCode(stalePreview),
        'disabled original code cannot join',
      );
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      await shot('10-original-code-disabled-existing-members-kept');
      checks.add(
        'formal-disable-invalidates-current-code-and-stale-preview-without-removing-original-member',
      );
      expect(
        (await domain(() => actors[0].cloud.service.workspaceMembers(wid))).any(
          (m) =>
              m.email == users['outsider']['email'] &&
              m.role == AppFlowyWorkspaceRole.member,
        ),
        isTrue,
      );
      await tap('cloud-access.member-menu.${users['outsider']['email']}');
      await tap('cloud-access.member-action.remove');
      await idle();
      await until(
        () => find
            .byKey(
              ValueKey('cloud-access.member.${users['outsider']['email']}'),
            )
            .evaluate()
            .isEmpty,
        'original membership removed',
      );
      await tap('cloud-access.close');
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'removed member loses original membership',
      );
      checks.add(
        'existing-owner-member-management-removes-the-invited-member-through-original-cloud',
      );
      await denied(
        () => actors[1].cloud.service.joinWorkspaceByInviteCode(stalePreview),
        'disabled code cannot restore removed membership',
      );
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'replay has not restored membership',
      );
      checks.add(
        'disabled-code-replay-cannot-regrant-membership-after-original-owner-removal',
      );
      await showActor(1);
      await tap('dataset.sync');
      await until(
        () => !member.canWrite,
        'original resource controller observes membership loss',
      );
      await shot('05-original-membership-loss-disables-live-database');
      checks.add(
        'original-resource-session-observes-removed-membership-without-replacing-the-live-owner',
      );
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.join-code');
      final endedInput = find.byKey(const ValueKey('cloud-code-join.input'));
      await t.enterText(endedInput, firstCode);
      final staleRead = t
          .widget<TextButton>(
            find.byKey(const ValueKey('cloud-code-join.preview')),
          )
          .onPressed!;
      await domain(actors[1].auth.signOut);
      await until(
        () =>
            t.widget<TextField>(endedInput).controller!.text.isEmpty &&
            find
                .byKey(const ValueKey('cloud-code-join.failure'))
                .evaluate()
                .isNotEmpty,
        'account end clears code credential',
      );
      await domain(
        () => actors[1].auth.signInWithPassword(
          users['outsider']['email'],
          'SmartdocLocal123!',
        ),
      );
      staleRead();
      await t.pump(const Duration(milliseconds: 250));
      expect(
        t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-code-join.preview')),
            )
            .onPressed,
        isNull,
      );
      expect(
        find.byKey(const ValueKey('cloud-code-join.workspace')),
        findsNothing,
      );
      await shot('11-account-end-clears-code-and-retires-old-join-dialog');
      checks.add(
        'real-A-B-A-auth-lifetime-erases-code-and-fences-captured-preview-join-session',
      );
      await showActor(0);
      final beforeOutcomeSize = t.view.physicalSize;
      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      await t.pump(const Duration(milliseconds: 350));
      final independent = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse(fixture['origin']),
        accessToken: actors[0].auth.accessToken,
        refreshAccessToken: () =>
            actors[0].auth.accessToken(forceRefresh: true),
        accountSubject: () => actors[0].auth.snapshot.isAuthenticated
            ? actors[0].auth.snapshot.identity?.subject
            : null,
        deviceId: 'code-independent-review-${fixture['run']}',
        allowHttpForTesting: true,
      );
      try {
        await tap('cloud-workspace.members.$wid');
        await tap('cloud-code.panel');
        await until(
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
          'fresh original code observation before response loss',
        );
        final capturedGenerate = t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-code.generate')),
            )
            .onPressed!;
        final transport = transports['alice']!;
        final beforeWrites = transport.codeWrites;
        transport.loseNextCodeReply = true;
        await tap('cloud-code.generate');
        await until(
          () =>
              find
                  .byKey(const ValueKey('cloud-code.failure'))
                  .evaluate()
                  .isNotEmpty &&
              find.byKey(const ValueKey('cloud-code.busy')).evaluate().isEmpty,
          'original code result unknown',
        );
        final state = t
            .widget<Text>(find.byKey(const ValueKey('cloud-code.state')))
            .data;
        expect(
          state == '当前邀请码状态待确认' || state == 'Current code state is unconfirmed',
          isTrue,
        );
        expect(transport.codeWrites, beforeWrites + 1);
        final actual = await domain(() => independent.workspaceInviteCode(wid));
        expect(actual.code != null, isTrue);
        await retainCode(actual.code!);
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('cloud-code.generate')),
              )
              .onPressed,
          isNull,
        );
        await shot('12-unknown-original-code-generation-at-430');
        checks.add(
          'original-Cloud-code-generation-applies-before-lost-response-while-formal-UI-remains-unconfirmed',
        );
        capturedGenerate();
        await t.pump(const Duration(milliseconds: 250));
        expect(transport.codeWrites, beforeWrites + 1);
        checks.add(
          'captured-original-code-generate-cannot-replay-an-unknown-post',
        );
        await tap('cloud-code.refresh');
        await until(
          () =>
              t
                      .widget<TextButton>(
                        find.byKey(const ValueKey('cloud-code.copy')),
                      )
                      .onPressed !=
                  null &&
              find.byKey(const ValueKey('cloud-code.busy')).evaluate().isEmpty,
          'current code fact read without claiming causal receipt',
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
        expect(transport.codeWrites, beforeWrites + 1);
        await shot('13-code-fact-readable-with-unknown-write-still-fenced');
        checks.add(
          'reading-current-original-code-does-not-discard-unknown-write-or-replay-reset-delete',
        );
        await tap('cloud-access.close');
        await tap('cloud-workspace.members.$wid');
        await tap('cloud-code.panel');
        await until(
          () =>
              find
                  .byKey(const ValueKey('cloud-code.copy'))
                  .evaluate()
                  .isNotEmpty &&
              t
                      .widget<TextButton>(
                        find.byKey(const ValueKey('cloud-code.copy')),
                      )
                      .onPressed !=
                  null,
          'reopened code fact',
        );
        expect(
          t
              .widget<TextButton>(
                find.byKey(const ValueKey('cloud-code.generate')),
              )
              .onPressed,
          isNull,
        );
        final reopened = t
            .widget<WorkspaceAccessDialog>(find.byType(WorkspaceAccessDialog))
            .controller;
        expect(reopened.canMutateTarget(users['bob']['email']), isTrue);
        expect(transport.codeWrites, beforeWrites + 1);
        checks.add(
          'reopening-original-member-dialog-preserves-code-unknown-fence-with-independent-member-controls',
        );
        // Explicit independent fixture cleanup; this does not reconcile or
        // silently replay the original UI's unknown request.
        await domain(() => independent.deleteWorkspaceInviteCode(actual));
        await tap('cloud-access.close');
      } finally {
        independent.closeClient();
        t.view.physicalSize = beforeOutcomeSize;
        await t.pump(const Duration(milliseconds: 350));
      }
      await t.pumpWidget(const SizedBox());
      await t.pump();
      for (final actor in actors) {
        await domain(actor.center.close);
        for (final plugin in actor.plugins) {
          await domain(plugin.deactivate);
        }
        await domain(actor.cloud.deactivate);
        actor.online.dispose();
        actor.auth.dispose();
      }
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'workspaceId': wid,
          'rootId': rootId,
          'parentViewId': db['parentViewId'],
          'rowId': rowId,
          'invitationCodesRecorded': false,
          'originalWorkspaceInviteCode': true,
          'originalCloudInvitationsAndMembership': true,
          'twoAuthenticatedHostContexts': true,
          'emailSent': false,
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_INVITE_CODE_CHECKS=${checks.length}');
    },
    timeout: const Timeout(Duration(minutes: 8)),
  );
}

class _Actor {
  _Actor(
    this.auth,
    this.cloud,
    this.online,
    this.registry,
    this.plugins,
    this.workspace,
    this.center,
  ) : centers = ValueNotifier(center);
  final GoTrueAuthenticationController auth;
  final OpenMuseCloudWorkspacePlugin cloud;
  final AppFlowyDocumentWorkspace online;
  final OpenMusePluginRegistry registry;
  final List<OpenMusePlugin> plugins;
  final LocalWorkspaceController workspace;
  final NativeResourceCenter center;
  final ValueNotifier<NativeResourceCenter?> centers;
}

class _CodeLossClient implements HttpClient {
  _CodeLossClient(this.inner);
  final HttpClient inner;
  bool loseNextCodeReply = false;
  int codeWrites = 0;
  @override
  Future<HttpClientRequest> openUrl(String method, Uri url) async =>
      _CodeLossRequest(await inner.openUrl(method, url), this, method, url);
  @override
  void close({bool force = false}) => inner.close(force: force);
  @override
  dynamic noSuchMethod(Invocation invocation) =>
      throw UnsupportedError('Unexpected acceptance transport member');
}

class _CodeLossRequest implements HttpClientRequest {
  _CodeLossRequest(this.inner, this.owner, this.method, this.url);
  final HttpClientRequest inner;
  final _CodeLossClient owner;
  @override
  final String method;
  final Uri url;
  @override
  HttpHeaders get headers => inner.headers;
  @override
  bool get followRedirects => inner.followRedirects;
  @override
  set followRedirects(bool value) {
    inner.followRedirects = value;
  }

  @override
  void write(Object? object) => inner.write(object);
  @override
  void abort([Object? exception, StackTrace? stackTrace]) =>
      inner.abort(exception, stackTrace);
  @override
  Future<HttpClientResponse> close() async {
    final response = await inner.close();
    if (method == 'POST' &&
        url.path.startsWith('/api/workspace/') &&
        url.path.endsWith('/invite-code')) {
      owner.codeWrites++;
      if (owner.loseNextCodeReply) {
        owner.loseNextCodeReply = false;
        if (response.statusCode != 200) {
          throw StateError(
            'Original code operation did not return success before simulated loss',
          );
        }
        await response.drain<void>();
        throw const SocketException(
          'Synthetic response loss after original code reply',
        );
      }
    }
    return response;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) =>
      throw UnsupportedError('Unexpected acceptance request member');
}
