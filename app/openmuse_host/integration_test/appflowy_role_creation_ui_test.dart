import 'dart:async';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
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
    'original database creation survives Member Guest Member transitions',
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
        'smartdoc-role-creation-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_ROLE_CREATION_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      final legacyCapabilities = _LegacyCapabilitiesClient();
      for (final name in ['alice', 'bob', 'compat']) {
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
            users[name == 'compat' ? 'bob' : name]['email'],
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
          service: name == 'compat'
              ? AppFlowyCloudWorkspaceService(
                  baseUri: Uri.parse(fixture['origin']),
                  accessToken: auth.accessToken,
                  refreshAccessToken: () =>
                      auth.accessToken(forceRefresh: true),
                  accountSubject: () => auth.snapshot.identity?.subject,
                  deviceId: 'legacy-capability-$name-${fixture['run']}',
                  allowHttpForTesting: true,
                  client: legacyCapabilities,
                )
              : null,
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
        if ((key.startsWith('cloud-access.person-level.') ||
            key.startsWith('cloud-access.level.'))) {
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
            (key.startsWith('cloud-access.person-level.') ||
                    key.startsWith('cloud-access.level.')) ||
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
      Future<void> shareParent({int? level, bool revoke = false}) async {
        await catalog();
        await tap('cloud-page.share.${db['parentViewId']}');
        await idle();
        if (revoke) {
          await tap('cloud-access.revoke.${users['bob']['email']}');
        } else if (level == 10 &&
            find
                .byKey(
                  ValueKey(
                    'cloud-access.person-permission.${users['bob']['email']}',
                  ),
                )
                .evaluate()
                .isEmpty) {
          await t.enterText(
            find.byKey(const ValueKey('cloud-access.email')),
            users['bob']['email'],
          );
          await tap('cloud-access.submit');
        } else {
          await tap('cloud-access.person-permission.${users['bob']['email']}');
          await tap(
            'cloud-access.person-level.${users['bob']['email']}.$level',
          );
        }
        await idle();
        await tap('cloud-access.close');
        await until(
          () => find.byType(BackButton).evaluate().length == 1,
          'one current route back button',
        );
        await until(
          () =>
              find.byType(BackButton).evaluate().length == 1 &&
              find.byType(BackButton).hitTestable().evaluate().length == 1,
          'one interactive route back button',
        );
        await t.tap(find.byType(BackButton));
        await t.pump(const Duration(milliseconds: 350));
        await until(
          () => find.byType(BackButton).evaluate().length == 1,
          'one current route back button',
        );
        await until(
          () =>
              find.byType(BackButton).evaluate().length == 1 &&
              find.byType(BackButton).hitTestable().evaluate().length == 1,
          'one interactive route back button',
        );
        await t.tap(find.byType(BackButton));
        await t.pump(const Duration(milliseconds: 350));
      }

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
      await catalog();
      await tap('cloud-page.share.${db['parentViewId']}');
      await idle();
      await t.enterText(
        find.byKey(const ValueKey('cloud-access.email')),
        users['bob']['email'],
      );
      await tap('cloud-access.level');
      await tap('cloud-access.level.30');
      await tap('cloud-access.submit');
      await idle();
      await tap('cloud-access.close');
      await until(
        () =>
            find.byType(BackButton).evaluate().length == 1 &&
            find.byType(BackButton).hitTestable().evaluate().length == 1,
        'one interactive route back button',
      );
      await t.tap(find.byType(BackButton));
      await t.pump(const Duration(milliseconds: 350));
      await until(
        () =>
            find.byType(BackButton).evaluate().length == 1 &&
            find.byType(BackButton).hitTestable().evaluate().length == 1,
        'one interactive route back button',
      );
      await t.tap(find.byType(BackButton));
      await t.pump(const Duration(milliseconds: 350));
      final owner = await openDatabase();
      await until(() => owner.canWrite, 'owner ready');
      await showActor(1);
      final member = await openDatabase(refresh: true);
      await until(() => member.canWrite, 'original Member capability');
      final original = member;
      final fieldId = member.views.first.columns.first.id;
      Future<String> createAndRegister(String reason) async {
        final before = member.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .map((r) => r.id)
            .toSet();
        debugPrint(
          'ROLE_ROW_CREATION_READY=${jsonEncode({'stage': reason, 'canWrite': member.canWrite, 'pending': member.rowCreationPending, 'review': member.creationNeedsReview, 'sync': member.syncState.name, 'buttonEnabled': t.widget<TextButton>(find.byKey(const ValueKey('dataset.add-row'))).onPressed != null})}',
        );
        await until(
          () => t.widget<TextButton>(
            find.byKey(const ValueKey('dataset.add-row')),
          ).onPressed != null,
          '$reason exposes an enabled original creation action',
        );
        await tap('dataset.add-row');
        await until(
          () =>
              !member.rowCreationPending &&
              member.views
                      .singleWhere((v) => v.id == db['viewId'])
                      .rows
                      .length ==
                  before.length + 1,
          '$reason creates one row',
        );
        final row = member.views
            .singleWhere((v) => v.id == db['viewId'])
            .rows
            .singleWhere((r) => !before.contains(r.id))
            .id;
        await until(
          () => owner.views
              .singleWhere((v) => v.id == db['viewId'])
              .rows
              .any((r) => r.id == row && r.loaded),
          '$reason registered at original server and visible in other Host',
        );
        expect(member.creationNeedsReview, isFalse);
        return row;
      }

      Future<void> changeRole({required bool guest}) async {
        await tap('cloud-workspace.open');
        await tap('cloud-workspace.members.$wid');
        await idle();
        if (guest) {
          await tap('cloud-access.member-menu.${users['bob']['email']}');
          await tap('cloud-access.member-action.guest');
        } else {
          await t.enterText(
            find.byKey(const ValueKey('cloud-access.email')),
            users['bob']['email'],
          );
          await tap('cloud-access.update-role');
        }
        await idle();
        await tap('cloud-access.close');
        await until(
          () =>
              find.byType(BackButton).evaluate().length == 1 &&
              find.byType(BackButton).hitTestable().evaluate().length == 1,
          'one interactive route back button',
        );
        await t.tap(find.byType(BackButton));
        await t.pump(const Duration(milliseconds: 350));
        await until(
          () => find.byType(DatasetSurface).evaluate().isNotEmpty,
          'original owner route restored',
        );
      }

      final memberRow = await createAndRegister('Member before downgrade');
      await edit(memberRow, fieldId, '成员阶段的原记录😀');
      await domain(member.synchronize);
      checks.add(
        'member-creation-registered-in-original-server-and-other-host',
      );
      await shot('01-member-creation-before-role-change');
      await showActor(0);
      await changeRole(guest: true);
      await showActor(1);
      await tap('dataset.sync');
      await until(
        () => member.canWrite,
        'Guest inherits original parent edit grant',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(original),
      );
      final guestRow = await createAndRegister(
        'Guest after live Member downgrade',
      );
      checks.add(
        'same-live-controller-creates-registered-row-after-member-to-guest-transition',
      );
      await showActor(0);
      await edit(guestRow, fieldId, '所有者保留的角色切换记录😀');
      await until(
        () =>
            member.views.first.rows
                .singleWhere((r) => r.id == guestRow)
                .cells[fieldId]!
                .value ==
            '所有者保留的角色切换记录😀',
        'Guest receives peer content',
      );
      await showActor(1);
      await tap('dataset.undo');
      await until(
        () => !owner.views.first.rows.any((r) => r.id == guestRow),
        'own creation undo reaches original root',
      );
      await tap('dataset.redo');
      await until(
        () => owner.views.first.rows.any(
          (r) =>
              r.id == guestRow && r.cells[fieldId]?.value == '所有者保留的角色切换记录😀',
        ),
        'same-row redo preserves original peer edit',
      );
      await until(
        () =>
            member.canRead &&
            member.canWrite &&
            member.views.first.rows.any(
              (r) =>
                  r.id == guestRow &&
                  r.cells[fieldId]?.value == '所有者保留的角色切换记录😀',
            ),
        'Guest row access and editable UI recover after own redo',
      );
      expect(find.text('所有者保留的角色切换记录😀'), findsOneWidget);
      checks.add(
        'guest-transition-creation-own-undo-redo-keeps-peer-row-content',
      );
      await shot('02-guest-creation-after-live-downgrade');
      await showActor(0);
      await shareParent(level: 10);
      await showActor(1);
      await tap('dataset.sync');
      await until(
        () => member.canRead && !member.canWrite,
        'Guest viewer narrowed by original grant',
      );
      expect(
        t
            .widget<TextButton>(find.byKey(const ValueKey('dataset.add-row')))
            .onPressed,
        isNull,
      );
      Object? denial;
      try {
        await domain(member.addRow);
      } catch (error) {
        denial = error;
      }
      expect(
        denial,
        isA<DatasetFailure>().having((e) => e.code, 'code', 'READ_ONLY'),
      );
      expect(member.rowCreationPending, isFalse);
      checks.add('viewer-ui-and-domain-refuse-creation-without-pending-intent');
      await shot('03-viewer-refuses-creation');
      await showActor(0);
      await changeRole(guest: false);
      await showActor(1);
      await tap('dataset.sync');
      await until(
        () => member.canWrite,
        'restored Member overrides page read grant',
      );
      expect(
        t.widget<DatasetSurface>(find.byType(DatasetSurface)).controller,
        same(original),
      );
      final restoredRow = await createAndRegister(
        'Member after Guest promotion',
      );
      await edit(restoredRow, fieldId, '恢复成员后的原记录😀');
      await until(
        () => owner.views.first.rows.any(
          (r) =>
              r.id == restoredRow && r.cells[fieldId]?.value == '恢复成员后的原记录😀',
        ),
        'restored Member original row live edit',
      );
      checks.add(
        'same-controller-creates-and-edits-after-guest-to-member-promotion',
      );
      await shot('04-member-creation-after-restoration');
      final tab = actors[1].workspace.activeTab!;
      actors[1].workspace.closeTab(tab);
      await until(
        () => !actors[1].workspace.tabs.contains(tab),
        'Member closes final checkpoint',
      );
      final reopened = await openDatabase();
      await until(
        () => reopened.views.first.rows.any(
          (r) => r.id == restoredRow && r.loaded,
        ),
        'reopen original registered rows',
      );
      expect(
        reopened.views.first.rows
            .singleWhere((r) => r.id == memberRow)
            .cells[fieldId]!
            .value,
        '成员阶段的原记录😀',
      );
      expect(
        reopened.views.first.rows
            .singleWhere((r) => r.id == guestRow)
            .cells[fieldId]!
            .value,
        '所有者保留的角色切换记录😀',
      );
      expect(
        reopened.views.first.rows
            .singleWhere((r) => r.id == restoredRow)
            .cells[fieldId]!
            .value,
        '恢复成员后的原记录😀',
      );
      checks.add(
        'cloud-reopen-retains-original-rows-created-across-role-transitions',
      );
      await shot('05-reopen-original-rows-across-roles');
      final finalTab = actors[1].workspace.activeTab!;
      actors[1].workspace.closeTab(finalTab);
      await until(
        () => !actors[1].workspace.tabs.contains(finalTab),
        'release account checkpoint writer before legacy-capability context',
      );
      await showActor(2);
      final compatibility = await openDatabase(refresh: true);
      await until(
        () => compatibility.canWrite,
        'existing data remains editable without creation capability',
      );
      expect(legacyCapabilities.probes, greaterThan(0));
      expect(
        t
            .widget<TextButton>(find.byKey(const ValueKey('dataset.add-row')))
            .onPressed,
        isNull,
      );
      final count = compatibility.views.first.rows.length;
      Object? unsupported;
      try {
        await domain(compatibility.addRow);
      } catch (error) {
        unsupported = error;
      }
      expect(
        unsupported,
        isA<DatasetFailure>().having(
          (e) => e.code,
          'code',
          'ROW_CREATION_UNSUPPORTED',
        ),
      );
      expect(compatibility.creationNeedsReview, isFalse);
      expect(compatibility.rowCreationPending, isFalse);
      expect(compatibility.views.first.rows.length, count);
      expect(owner.views.first.rows.length, count);
      expect(legacyCapabilities.creationPosts, 0);
      await edit(restoredRow, fieldId, '已有记录继续编辑😀');
      await until(
        () =>
            owner.views.first.rows
                .singleWhere((r) => r.id == restoredRow)
                .cells[fieldId]!
                .value ==
            '已有记录继续编辑😀',
        'legacy-capability context edits the original registered row',
      );
      checks.add(
        'missing-creation-capability-refuses-ui-and-domain-without-raw-fallback-and-keeps-existing-edits',
      );
      await shot('06-unsupported-creation-capability-existing-edit');

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
          'memberRowId': memberRow,
          'guestRowId': guestRow,
          'restoredRowId': restoredRow,
          'sameControllerAcrossRoles': true,
          'authenticatedHostContexts': actors.length,
          'legacyCapabilitySimulation':
              'only capability GET returns HTTP 404; content and authorization use original Cloud',
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_ROLE_CREATION_CHECKS=${checks.length}');
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

// Simulate only an old server's missing feature endpoint. All content, roles,
// real HTTP/WS2 authorization and existing edits use the original owned Cloud.
final class _LegacyCapabilitiesClient extends Fake implements HttpClient {
  final delegate = HttpClient();
  int probes = 0, creationPosts = 0;
  @override
  Future<HttpClientRequest> openUrl(String method, Uri url) async {
    if (method == 'GET' && url.path.endsWith('/row/creation-capabilities')) {
      probes++;
      return _MissingCapabilityRequest();
    }
    if (method == 'POST' && url.path.endsWith('/row')) creationPosts++;
    return delegate.openUrl(method, url);
  }

  @override
  void close({bool force = false}) => delegate.close(force: force);
}

final class _MissingCapabilityRequest extends Fake
    implements HttpClientRequest {
  @override
  final HttpHeaders headers = _DiscardedHeaders();
  @override
  set followRedirects(bool value) {}
  @override
  void abort([Object? exception, StackTrace? stackTrace]) {}
  @override
  Future<HttpClientResponse> close() async => _MissingCapabilityResponse();
}

final class _DiscardedHeaders extends Fake implements HttpHeaders {
  @override
  void set(String name, Object value, {bool preserveHeaderCase = false}) {}
}

final class _MissingCapabilityResponse extends Stream<List<int>>
    implements HttpClientResponse {
  @override
  int get statusCode => 404;
  @override
  StreamSubscription<List<int>> listen(
    void Function(List<int>)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) => const Stream<List<int>>.empty().listen(
    onData,
    onError: onError,
    onDone: onDone,
    cancelOnError: cancelOnError,
  );
  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnimplementedError();
}
