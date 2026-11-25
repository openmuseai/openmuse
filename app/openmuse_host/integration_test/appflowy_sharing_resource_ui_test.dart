import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_host/src/host/openmuse_app.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  WidgetController.hitTestWarningShouldBeFatal = true;
  testWidgets(
    'original AppFlowy groups and page sharing in the actual Native Host',
    (t) async {
      HttpOverrides.global = null;
      const fixturePath = String.fromEnvironment('SMARTDOC_SHARING_FIXTURE');
      expect(fixturePath, isNotEmpty);
      final f = jsonDecode(await File(fixturePath).readAsString()) as Map;
      final origin = Uri.parse(f['origin'] as String);
      final workspaceId = f['workspaceId'] as String;
      final users = f['users'] as Map;
      final root = await Directory.systemTemp.createTemp(
        'smartdoc-sharing-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_SHARING_EVIDENCE=${evidence.path}');
      GoTrueAuthenticationController auth() => GoTrueAuthenticationController(
        provider: GoTrueHttpClient(
          config: GoTrueClientConfig(
            origin: Uri.parse(f['authOrigin']),
            allowInsecureLoopback: true,
          ),
        ),
        store: MemoryAuthSessionStore(),
        bootstrapper: AppFlowyAccountBootstrapper(
          cloudOrigin: origin,
          allowInsecureLoopback: true,
        ),
      );
      final alice = auth(), guestAuth = auth();
      await t.runAsync(() async {
        await alice.signInWithPassword(
          users['alice']['email'],
          'SmartdocLocal123!',
        );
        await guestAuth.signInWithPassword(
          users['outsider']['email'],
          'SmartdocLocal123!',
        );
      });
      expect(alice.snapshot.isAuthenticated, isTrue);
      expect(guestAuth.snapshot.isAuthenticated, isTrue);
      late final AppFlowyDocumentWorkspace online;
      final cloud = OpenMuseCloudWorkspacePlugin(
        authentication: alice,
        cloudOrigin: origin,
        deviceId: 'sharing-host-${f['run']}',
        allowInsecureLoopback: true,
        onOpenDocument: (page) => online.openDocument(page),
      );
      online = AppFlowyDocumentWorkspace(
        authentication: alice,
        service: cloud.service,
        checkpointRoot: root,
      );
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final plugins = createOpenMuseBuiltInPlugins(cloudDocuments: online)
          .where(
            (p) => {
              'io.openmuse.smart-document',
              'io.openmuse.dataset',
            }.contains(p.descriptor.id),
          )
          .toList();
      registry.install(cloud);
      for (final plugin in plugins) {
        registry.install(plugin);
      }
      await t.runAsync(() async {
        await registry.activate(cloud.descriptor.id);
        for (final plugin in plugins) {
          await registry.activate(plugin.descriptor.id);
        }
      });
      late AppFlowyDocumentPage page;
      await t.runAsync(() async {
        final folder = await cloud.service.workspaceFolder(workspaceId);
        final space = (folder['children'] as List).cast<Map>().firstWhere(
          (n) => n['is_space'] == true,
        );
        final id = await cloud.service.createDocumentPage(
          workspaceId,
          space['view_id'],
          '原有群组分享 ${f['run']}',
        );
        page = (await cloud.service.documentPages(
          workspaceId,
        )).singleWhere((p) => p.viewId == id);
      });
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final center = await t.runAsync(
        () => NativeResourceCenter.open(
          scope: alice.snapshot.identity!.subject,
          deviceRef: 'sharing-host',
          deviceLabel: 'Mac',
          registry: registry,
          storageRoot: Directory('${root.path}/resources'),
        ),
      );
      online.attach(center!.controller);
      final centers = ValueNotifier<NativeResourceCenter?>(center);
      final checks = <String>[];
      Future<void> until(bool Function() ready, String reason) async {
        for (var i = 0; i < 250; i++) {
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

      Future<void> tap(String key) async {
        final target = find.byKey(ValueKey(key));
        await until(() => target.evaluate().isNotEmpty, key);
        final popup = key.startsWith('cloud-access.person-level.');
        if (popup) {
          await t.pumpAndSettle();
        } else {
          await t.ensureVisible(target);
        }
        final pointerTarget = popup
            ? find.descendant(of: target, matching: find.byType(Text)).first
            : target;
        await until(
          () => pointerTarget.hitTestable().evaluate().length == 1,
          '$key receives pointer events',
        );
        await t.tap(pointerTarget);
        await t.pump(const Duration(milliseconds: 350));
      }

      Future<T> domain<T>(Future<T> Function() action) async {
        var done = false;
        T? value;
        Object? error;
        StackTrace? stack;
        await t.runAsync(() async {
          action().then(
            (result) {
              value = result;
              done = true;
            },
            onError: (Object e, StackTrace s) {
              error = e;
              stack = s;
              done = true;
            },
          );
        });
        await until(() => done, 'domain operation settled');
        if (error != null) Error.throwWithStackTrace(error!, stack!);
        return value as T;
      }

      Future<void> idle() => until(
        () =>
            find.byKey(const ValueKey('cloud-access.busy')).evaluate().isEmpty,
        'access settled',
      );
      Future<void> enterEmail(String email) async {
        final field = find.byKey(const ValueKey('cloud-access.email'));
        final dialog = t.widget<WorkspaceAccessDialog>(
          find.byType(WorkspaceAccessDialog),
        ).controller;
        debugPrint(
          'SHARING_EMAIL_READY=${jsonEncode({'fields': field.evaluate().length, 'role': dialog.role?.name, 'retired': dialog.retired, 'busy': dialog.busy})}',
        );
        await t.scrollUntilVisible(
          field,
          -200,
          scrollable: find.descendant(
            of: find.byKey(const ValueKey('cloud-access.contents')),
            matching: find.byType(Scrollable),
          ).first,
        );
        await t.ensureVisible(field);
        await until(
          () => field.hitTestable().evaluate().length == 1 &&
              find.descendant(of: field, matching: find.byType(EditableText))
                  .evaluate().length == 1,
          'original sharing email is visible and editable',
        );
        await t.enterText(field, email);
      }

      Future<void> shot(String name) async {
        await t.pump(const Duration(milliseconds: 200));
        final surface = t.renderObject<RenderRepaintBoundary>(
          find.byKey(const ValueKey('sharing.acceptance.surface')),
        );
        final image = await surface.toImage(pixelRatio: 1);
        final data = await image.toByteData(format: ui.ImageByteFormat.png);
        await File(
          '${evidence.path}/$name.png',
        ).writeAsBytes(data!.buffer.asUint8List());
        image.dispose();
      }

      await t.pumpWidget(
        RepaintBoundary(
          key: const ValueKey('sharing.acceptance.surface'),
          child: OpenMuseHostApp(
            registry: registry,
            workspace: workspace,
            resources: centers,
          ),
        ),
      );
      await tap('cloud-workspace.open');
      await tap('cloud-workspace.members.$workspaceId');
      await idle();
      expect(
        find.byKey(ValueKey('cloud-access.member.${users['bob']['email']}')),
        findsOneWidget,
      );
      checks.add('original-gotrue-and-workspace-member-list-in-native-host');
      await shot('01-original-workspace-members');
      await tap('cloud-access.member-menu.${users['bob']['email']}');
      await tap('cloud-access.member-action.guest');
      await t.pump(const Duration(milliseconds: 350));
      await idle();
      expect(
        (await t.runAsync(
          () => cloud.service.workspaceMembers(workspaceId),
        ))!.any((m) => m.email == users['bob']['email']),
        isFalse,
      );
      await enterEmail(users['bob']['email']);
      await tap('cloud-access.update-role');
      await t.pump(const Duration(milliseconds: 350));
      await idle();
      expect(
        (await t.runAsync(
          () => cloud.service.workspaceMembers(workspaceId),
        ))!.singleWhere((m) => m.email == users['bob']['email']).role,
        AppFlowyWorkspaceRole.member,
      );
      checks.add(
        'original-owner-member-guest-role-mutation-and-confirmed-readback',
      );
      await tap('cloud-access.close');
      await tap('cloud-workspace.$workspaceId');
      await tap('cloud-page.${page.viewId}');
      await until(
        () => find.byType(SmartDocumentView).evaluate().isNotEmpty,
        'cloud document editor',
      );
      final document = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .controller;
      expect(document.objectId, page.viewId);
      await until(() => document.canWrite, 'owner write capability');
      await until(
        () => find.byType(AppFlowyRichText).evaluate().isNotEmpty,
        'native editor',
      );
      await t.tap(find.byType(AppFlowyRichText).first);
      await t.pump(const Duration(milliseconds: 200));
      final previousClipboard = await Clipboard.getData(Clipboard.kTextPlain);
      try {
        await Clipboard.setData(const ClipboardData(text: '原有云端共享正文😀'));
        await t.sendKeyDownEvent(LogicalKeyboardKey.metaLeft);
        await t.sendKeyEvent(LogicalKeyboardKey.keyV);
        await t.sendKeyUpEvent(LogicalKeyboardKey.metaLeft);
        await until(
          () => document.blocks.first.delta!.toPlainText().contains('原有云端共享'),
          'native cloud keyboard input',
        );
      } finally {
        await Clipboard.setData(
          ClipboardData(text: previousClipboard?.text ?? ''),
        );
      }
      await domain(document.synchronize);
      checks.add(
        'original-folder-document0-native-editor-keyboard-and-http-sync',
      );
      document.editor.selection = Selection.single(
        path: [0],
        startOffset: 0,
        endOffset: 4,
      );
      await t.pump();
      await tap('smart-document.link');
      await t.enterText(
        find.byKey(const ValueKey('smart-document.link-address')),
        'https://example.com/shared-document',
      );
      await tap('smart-document.link-apply');
      await until(
        () => jsonEncode(
          document.blocks.first.delta!.toJson(),
        ).contains('https://example.com/shared-document'),
        'cloud hyperlink mark',
      );
      checks.add('native-original-hyperlink-dialog-in-cloud-document');
      final png = await File(
        '/Users/mac/src/openmuse-io/Muse-Plugin-Ecosystem/plugins/markdown/smart_document/flutter/test/fixtures/test-image.png',
      ).readAsBytes();
      final media = t
          .widget<SmartDocumentView>(find.byType(SmartDocumentView))
          .media!;
      final asset = await domain(
        () => media.repository.put(
          owner: page.viewId,
          bytes: png,
          mediaType: 'image/png',
        ),
      );
      expect(asset.provider, 'appflowy.private-assets/1');
      final imageId = 'sharing-image-${f['run']}';
      await domain(
        () => document.insertImage(
          blockId: imageId,
          asset: asset,
          width: 128,
          height: 128,
          after: DocumentBlockReference(
            objectId: page.viewId,
            blockId: document.blocks.first.id,
          ),
        ),
      );
      await until(
        () => t
            .widgetList<Image>(find.byType(Image))
            .any(
              (image) =>
                  image.image is MemoryImage &&
                  resourceDigest((image.image as MemoryImage).bytes) ==
                      asset.sha256,
            ),
        'original cloud private image rendered',
      );
      await domain(document.synchronize);
      await tap('smart-document.share');
      await idle();
      await enterEmail(users['outsider']['email']);
      await tap('cloud-access.submit');
      await idle();
      final confirmed = await t.runAsync(
        () => cloud.service.pageAccess(workspaceId, page.viewId),
      );
      expect(confirmed!.people.single.access, AppFlowyAccessLevel.readOnly);
      checks.add('native-page-share-default-readonly-original-server-grant');
      await shot('02-original-page-readonly-share');
      final guestProvider = AppFlowyCollaborationProvider(
        origin: origin,
        workspaceId: workspaceId,
        accessToken: () async => (await guestAuth.accessToken())!,
        deviceId: 'sharing-guest-${f['run']}',
        boundObjectId: page.viewId,
      );
      debugPrint('SHARING_STEP=guest-open');
      final guest = await domain(
        () => SmartDocumentController.open(
          core: NativeDocumentCore(),
          store: _Memory(),
          objectId: page.viewId,
          provider: guestProvider,
          syncInterval: null,
        ),
      );
      expect(guest.blocks.first.delta!.toPlainText(), contains('原有云端共享'));
      final guestAssets = CloudAssetRepository(
        origin: origin,
        workspaceId: workspaceId,
        accessToken: () async => (await guestAuth.accessToken())!,
      );
      expect(await domain(() => guestAssets.read(asset)), png);
      checks.add(
        'original-cloud-private-image-upload-native-render-and-guest-read',
      );
      final wire = NativeRealtimeWireCore();
      final connection = AppFlowyRealtimeConnection(
        origin: origin,
        workspaceId: workspaceId,
        deviceId: 'sharing-guest-${f['run']}',
        clientId: '88991133',
        accessToken: () async => (await guestAuth.accessToken())!,
        codec: AppFlowyRealtimeCodec(wire),
      );
      final realtime = DocumentRealtimeSession(
        controller: guest,
        connection: connection,
        presenceCore: wire,
      );
      debugPrint('SHARING_STEP=guest-realtime');
      await domain(realtime.start);
      await until(
        () => realtime.connected && guest.canRead && !guest.canWrite,
        'guest read-only server capability',
      );
      checks.add('original-guest-http-read-and-ws2-readonly-capability');
      final sibling = (await t.runAsync(
        () => cloud.service.documentPages(workspaceId),
      ))!.firstWhere((p) => p.viewId != page.viewId);
      final denied = AppFlowyCollaborationProvider(
        origin: origin,
        workspaceId: workspaceId,
        accessToken: () async => (await guestAuth.accessToken())!,
        deviceId: 'sharing-guest-probe',
      );
      await t.runAsync(
        () => expectLater(
          denied.readState(sibling.viewId),
          throwsA(isA<SmartDocumentFailure>()),
        ),
      );
      denied.close();
      checks.add('guest-unshared-sibling-denied-by-original-cloud');
      final guestCatalog = AppFlowyCloudWorkspaceService(
        baseUri: origin,
        accessToken: guestAuth.accessToken,
        accountSubject: () => guestAuth.snapshot.identity?.subject,
        deviceId: 'guest-catalog',
        allowHttpForTesting: true,
      );
      expect(
        await domain(() => guestCatalog.workspaceRole(workspaceId)),
        AppFlowyWorkspaceRole.guest,
      );
      final guestPages = await domain(
        () => guestCatalog.documentPages(workspaceId),
      );
      expect(guestPages.map((p) => p.viewId), [page.viewId]);
      guestCatalog.closeClient();
      checks.add('original-guest-workspace-and-filtered-shared-page-catalog');
      await enterEmail(users['bob']['email']);
      await tap('cloud-access.submit');
      await idle();
      final bob = await domain(
        () => SmartDocumentController.open(
          core: NativeDocumentCore(),
          store: _Memory(),
          objectId: page.viewId,
          syncInterval: null,
          provider: AppFlowyCollaborationProvider(
            origin: origin,
            workspaceId: workspaceId,
            accessToken: () async => users['bob']['accessToken'],
            deviceId: 'member-inheritance',
            boundObjectId: page.viewId,
          ),
        ),
      );
      expect(bob.canWrite, isTrue);
      await domain(
        () => bob.editor.apply(
          bob.editor.transaction..insertText(
            bob.blocks.first,
            bob.blocks.first.delta!.toPlainText().trimRight().length,
            '·群组成员',
          ),
        ),
      );
      await domain(bob.synchronize);
      await domain(bob.close);
      await until(
        () => document.blocks.first.delta!.toPlainText().contains('群组成员'),
        'workspace member write despite page read grant',
      );
      checks.add(
        'original-workspace-member-write-is-not-downgraded-by-page-read-grant',
      );
      await tap('cloud-access.person-permission.${users['outsider']['email']}');
      await t.pump(const Duration(milliseconds: 350));
      await tap('cloud-access.person-level.${users['outsider']['email']}.30');
      await t.pump(const Duration(milliseconds: 350));
      await idle();
      connection.send(
        RealtimeManifest(
          objectId: guest.objectId,
          collabType: 0,
          stateVector: base64Decode(guest.snapshot['stateVector']),
        ),
      );
      await until(() => guest.canWrite, 'original grant upgrade');
      await domain(
        () => guest.editor.apply(
          guest.editor.transaction..insertText(guest.blocks.first, 0, '访客编辑·'),
        ),
      );
      await domain(guest.synchronize);
      await tap('cloud-access.close');
      await until(
        () => document.blocks.first.delta!.toPlainText().startsWith('访客编辑·'),
        'guest edit reaches owner native surface',
      );
      checks.add(
        'native-share-upgrade-original-guest-edit-and-owner-live-projection',
      );
      await shot('03-original-guest-edit-visible-in-host');
      await tap('smart-document.share');
      await idle();
      await tap('cloud-access.revoke.${users['outsider']['email']}');
      await idle();
      expect(
        (await t.runAsync(
          () => cloud.service.pageAccess(workspaceId, page.viewId),
        ))!.people.any((person) => person.email == users['outsider']['email']),
        isFalse,
      );
      connection.send(
        RealtimeManifest(
          objectId: guest.objectId,
          collabType: 0,
          stateVector: base64Decode(guest.snapshot['stateVector']),
        ),
      );
      await until(
        () => !guest.canRead && !guest.canWrite,
        'guest revoke server capability',
      );
      expect(guest.blocks.first.delta!.toPlainText(), startsWith('访客编辑·'));
      await domain(
        () => expectLater(
          guestAssets.read(asset),
          throwsA(
            isA<AssetFailure>().having((e) => e.code, 'code', 'FORBIDDEN'),
          ),
        ),
      );
      guestAssets.close();
      checks.add('revoked-document-guest-cannot-read-original-private-image');
      checks.add(
        'native-revoke-server-ws2-denial-retains-existing-guest-draft',
      );
      await shot('04-revoked-page-grant');
      await tap('cloud-access.close');
      final captured = document.snapshot['docState'];
      center.controller.editorLifetime.invalidate();
      expect(document.canRead, isFalse);
      expect(document.canWrite, isFalse);
      document.acceptRealtimeAccess(
        canRead: true,
        canWrite: true,
        reason: 'late-grant',
      );
      expect(document.canRead, isFalse);
      expect(document.snapshot['docState'], captured);
      checks.add(
        'native-session-account-lifetime-cannot-be-restored-by-late-grant',
      );
      await domain(realtime.close);
      await domain(guest.close);
      await t.pumpWidget(const SizedBox());
      await t.pump();
      await domain(center.close);
      for (final plugin in plugins) {
        await domain(plugin.deactivate);
      }
      await t.runAsync(cloud.deactivate);
      online.dispose();
      alice.dispose();
      guestAuth.dispose();
      await File('${evidence.path}/result.json').writeAsString(
        jsonEncode({
          'pass': true,
          'checks': checks,
          'workspaceId': workspaceId,
          'viewId': page.viewId,
          'fixtureRun': f['run'],
          'originalAuth': true,
          'emailSent': false,
          'globalCloudCasClaim': false,
        }),
      );
      debugPrint('SMARTDOC_SHARING_CHECKS=${checks.length}');
    },
    timeout: const Timeout(Duration(minutes: 8)),
  );
}

class _Memory implements DocumentStore {
  Map<String, dynamic>? value;
  @override
  Future<Map<String, dynamic>?> read() async => value;
  @override
  Future<void> write(Map<String, dynamic> state) async {
    value = jsonDecode(jsonEncode(state));
  }
}
