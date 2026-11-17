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
    'original workspace invitations join existing document and database collaboration',
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
        'smartdoc-invitation-management-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_INVITATION_MANAGEMENT_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
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
        final cloud = OpenMuseCloudWorkspacePlugin(
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
      var inviteId = (fixture['invitation'] as Map)['id'] as String;
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

      t.view.physicalSize = Size(
        430 * t.view.devicePixelRatio,
        900 * t.view.devicePixelRatio,
      );
      addTearDown(t.view.resetPhysicalSize);
      final owner = await openDatabase();
      final rowId = owner.views.first.rows.first.id;
      final fieldId = owner.views.first.columns.first.id;
      await edit(rowId, fieldId, '原群组邀请前的原行😀');
      await domain(owner.synchronize);
      final endedId = inviteId;
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.members.$wid');
      await until(
        () => find
            .byKey(ValueKey('cloud-sent.cancel.$inviteId'))
            .evaluate()
            .isNotEmpty,
        'Owner Pending invitation',
      );
      final withdraw = t
          .widget<TextButton>(
            find.byKey(ValueKey('cloud-sent.cancel.$inviteId')),
          )
          .onPressed!;
      expect(
        t.getSize(find.byKey(ValueKey('cloud-sent.cancel.$inviteId'))).height,
        greaterThanOrEqualTo(44),
      );
      expect(
        t.getSize(find.byKey(const ValueKey('database-host-0'))).width,
        430,
      );
      await t.ensureVisible(
        find.byKey(ValueKey('cloud-sent.cancel.$inviteId')),
      );
      await shot('07-owner-original-pending-at-430');
      checks.add('original-owner-query-and-formal-withdrawal-target-at-430');
      await tap('cloud-sent.cancel.$inviteId');
      await until(
        () =>
            find
                .byKey(ValueKey('cloud-sent.cancel.$inviteId'))
                .evaluate()
                .isEmpty &&
            find
                .byKey(const ValueKey('cloud-sent.notice'))
                .evaluate()
                .isNotEmpty,
        'withdrawal confirmed',
      );
      final ended = await domain(
        () => actors[0].cloud.service.ownerWorkspaceInvitations(
          wid,
          email: users['outsider']['email'],
        ),
      );
      expect(ended.items.singleWhere((r) => r.id == endedId).wireStatus, 2);
      expect(
        ended.items.singleWhere((r) => r.id == endedId).memberRoleId,
        isNull,
      );
      withdraw();
      await t.pump(const Duration(milliseconds: 300));
      expect(owner.objectId, rootId);
      await shot('08-owner-ended-original-invitation');
      checks.add(
        'formal-owner-withdrawal-preserves-root-and-fences-old-callback',
      );
      await tap('cloud-access.close');
      await until(
        () => find.byType(BackButton).evaluate().length == 1,
        'current catalog back button',
      );
      await t.tap(find.byType(BackButton));
      await t.pump(const Duration(milliseconds: 300));
      await showActor(1);
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.invitations');
      await until(
        () => find
            .byKey(ValueKey('cloud-invitations.status.$endedId'))
            .evaluate()
            .isNotEmpty,
        'recipient ended original invitation',
      );
      expect(
        find.byKey(ValueKey('cloud-invitations.accept.$endedId')),
        findsNothing,
      );
      await denied(
        () => actors[1].cloud.service.acceptWorkspaceInvitation(endedId),
        'withdrawn original link cannot accept',
      );
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'withdrawal grants no membership',
      );
      await shot('09-recipient-old-link-ended');
      checks.add(
        'withdrawn-original-invitation-disables-recipient-acceptance-and-old-link',
      );
      await tap('cloud-invitations.close');
      await until(
        () => find.byType(BackButton).evaluate().length == 1,
        'current catalog back button',
      );
      await t.tap(find.byType(BackButton));
      await t.pump(const Duration(milliseconds: 300));
      await showActor(0);
      await domain(
        () => actors[0].cloud.service.inviteWorkspaceMembers(
          wid,
          [users['outsider']['email']],
          role: AppFlowyWorkspaceRole.member,
          skipEmailSend: true,
        ),
      );
      final fresh = await domain(
        () => actors[0].cloud.service.ownerWorkspaceInvitations(
          wid,
          email: users['outsider']['email'],
          status: 0,
        ),
      );
      final pendingObservation = fresh.items.single;
      inviteId = pendingObservation.id;
      expect(inviteId, isNot(endedId));
      checks.add(
        'new-original-invitation-id-after-ended-history-with-email-suppressed',
      );

      await denied(
        () => actors[0].cloud.service.workspaceInvitation(inviteId),
        'nonrecipient cannot read the invitation',
      );
      await denied(
        () => actors[0].cloud.service.acceptWorkspaceInvitation(inviteId),
        'nonrecipient cannot accept the invitation',
      );
      expect(
        (await domain(
          () => actors[1].cloud.service.workspaceInvitation(inviteId),
        )).status,
        AppFlowyWorkspaceInvitationStatus.pending,
      );
      checks.add(
        'original-invitation-server-rejects-nonrecipient-reads-and-acceptance-without-changing-pending-state',
      );

      await showActor(1);
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.refresh');
      await until(
        () => !actors[1].cloud.controller.snapshot.loading,
        'recipient catalog',
      );
      expect(
        actors[1].cloud.controller.snapshot.workspaces.any(
          (w) => w.workspaceRef == wid,
        ),
        isFalse,
      );
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'pending invitation is not membership',
      );
      checks.add(
        'pending-original-workspace-invitation-does-not-grant-workspace-membership',
      );
      await tap('cloud-workspace.invitations');
      await until(
        () => find
            .byKey(ValueKey('cloud-invitations.accept.$inviteId'))
            .evaluate()
            .isNotEmpty,
        'formal pending workspace invitation',
      );
      await shot('01-original-pending-invitation-in-recipient-host');
      await tap('cloud-invitations.accept.$inviteId');
      await until(
        () =>
            find
                .byKey(ValueKey('cloud-invitations.accept.$inviteId'))
                .evaluate()
                .isEmpty &&
            find
                .byKey(ValueKey('cloud-invitations.status.$inviteId'))
                .evaluate()
                .isNotEmpty,
        'original acceptance confirmed in UI',
      );
      expect(
        (await domain(
          () => actors[1].cloud.service.workspaceInvitation(inviteId),
        )).status,
        AppFlowyWorkspaceInvitationStatus.accepted,
      );
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      await shot('02-original-accepted-invitation-and-server-membership');
      checks.add(
        'formal-host-acceptance-uses-original-invitation-transaction-and-original-member-role',
      );
      await tap('cloud-invitations.close');
      await until(
        () => actors[1].cloud.controller.snapshot.workspaces.any(
          (w) => w.workspaceRef == wid,
        ),
        'original catalog refreshed after acceptance',
      );
      expect(find.byKey(ValueKey('cloud-workspace.$wid')), findsOneWidget);
      checks.add(
        'confirmed-acceptance-refreshes-the-existing-workspace-directory-without-a-second-catalog',
      );
      await denied(
        () => actors[1].cloud.service.acceptWorkspaceInvitation(inviteId),
        'accepted invitation cannot be replayed',
      );
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      checks.add('original-pending-status-check-rejects-repeated-acceptance');

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
      await t.tap(find.text('编辑'));
      await t.pump(const Duration(milliseconds: 350));
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
      await until(
        () => find
            .byKey(ValueKey('cloud-sent.status.$inviteId'))
            .evaluate()
            .isNotEmpty,
        'Owner accepted invitation',
      );
      expect(find.byKey(ValueKey('cloud-sent.cancel.$inviteId')), findsNothing);
      final accepted = await domain(
        () => actors[0].cloud.service.cancelOwnerWorkspaceInvitation(
          pendingObservation,
        ),
      );
      expect(accepted.outcome, 'accepted');
      expect(accepted.invitation.memberRole, AppFlowyWorkspaceRole.member);
      expect(
        await domain(() => actors[1].cloud.service.workspaceRole(wid)),
        AppFlowyWorkspaceRole.member,
      );
      await shot('10-accepted-invitation-keeps-original-membership');
      checks.add(
        'accepted-first-original-transaction-cannot-be-withdrawn-as-member-removal',
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
      await until(() {
        final f = find.byKey(ValueKey('cloud-sent.member.$inviteId'));
        if (f.evaluate().isEmpty) return false;
        final value = t.widget<Text>(f).data ?? '';
        return value.contains('Not currently a member') ||
            value.contains('当前未加入');
      }, 'current invitation member fact refreshed');
      checks.add(
        'original-member-removal-refreshes-existing-invitation-member-fact',
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
        () => actors[1].cloud.service.acceptWorkspaceInvitation(inviteId),
        'historical accepted invitation cannot restore removed membership',
      );
      await denied(
        () => actors[1].cloud.service.workspaceRole(wid),
        'replay has not restored membership',
      );
      checks.add(
        'accepted-invitation-replay-cannot-regrant-membership-after-owner-removal',
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
      await tap('cloud-workspace.invitations');
      await until(
        () => find
            .byKey(ValueKey('cloud-invitations.status.$inviteId'))
            .evaluate()
            .isNotEmpty,
        'accepted invitation history remains readable',
      );
      expect(
        find.byKey(ValueKey('cloud-invitations.accept.$inviteId')),
        findsNothing,
      );
      await domain(actors[1].auth.signOut);
      await until(
        () =>
            find
                .byKey(const ValueKey('cloud-invitations.failure'))
                .evaluate()
                .isNotEmpty &&
            find
                .byKey(ValueKey('cloud-invitations.row.$inviteId'))
                .evaluate()
                .isEmpty,
        'account end hides prior invitation',
      );
      await domain(
        () => actors[1].auth.signInWithPassword(
          users['outsider']['email'],
          'SmartdocLocal123!',
        ),
      );
      expect(
        find.byKey(ValueKey('cloud-invitations.row.$inviteId')),
        findsNothing,
      );
      expect(
        t
            .widget<TextButton>(
              find.byKey(const ValueKey('cloud-invitations.refresh')),
            )
            .onPressed,
        isNull,
      );
      await shot('06-account-lifetime-retires-old-invitation-dialog');
      checks.add(
        'actual-signout-and-same-account-signin-cannot-reactivate-the-old-invitation-dialog',
      );
      await showActor(0);
      await tap('cloud-workspace.members.$wid');
      await until(
        () => find
            .byKey(ValueKey('cloud-sent.row.$inviteId'))
            .evaluate()
            .isNotEmpty,
        'Owner rows before account end',
      );
      final oldRefresh = t
          .widget<TextButton>(find.byKey(const ValueKey('cloud-sent.refresh')))
          .onPressed!;
      await domain(actors[0].auth.signOut);
      await until(
        () =>
            find
                .byKey(ValueKey('cloud-sent.row.$inviteId'))
                .evaluate()
                .isEmpty &&
            find
                .byKey(const ValueKey('cloud-access.failure'))
                .evaluate()
                .isNotEmpty,
        'Owner account end clears restricted projection',
      );
      oldRefresh();
      await t.pump(const Duration(milliseconds: 300));
      expect(find.byKey(ValueKey('cloud-sent.row.$inviteId')), findsNothing);
      await shot('11-owner-account-end-retires-management');
      checks.add(
        'owner-account-end-clears-sent-invitations-and-fences-captured-refresh',
      );
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
          'invitationId': inviteId,
          'originalCloudInvitationsAndMembership': true,
          'twoAuthenticatedHostContexts': true,
          'emailSent': false,
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_INVITATION_MANAGEMENT_CHECKS=${checks.length}');
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
