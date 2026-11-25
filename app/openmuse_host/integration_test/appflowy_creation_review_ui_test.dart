import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
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
    'creation review retains original inputs without replaying an accepted Cloud write',
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
        'smartdoc-creation-review-host-',
      );
      final evidence = await Directory('${root.path}/evidence').create();
      debugPrint('SMARTDOC_CREATION_REVIEW_EVIDENCE=${evidence.path}');
      final actors = <_Actor>[];
      final faults = _CreationFaultClient();
      for (final name in ['alice', 'bob']) {
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
            users[name == 'bob' ? 'bob' : name]['email'],
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
          service: name == 'bob'
              ? AppFlowyCloudWorkspaceService(
                  baseUri: Uri.parse(fixture['origin']),
                  accessToken: auth.accessToken,
                  refreshAccessToken: () =>
                      auth.accessToken(forceRefresh: true),
                  accountSubject: () => auth.snapshot.identity?.subject,
                  deviceId: 'legacy-capability-$name-${fixture['run']}',
                  allowHttpForTesting: true,
                  client: faults,
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
        if (key.startsWith('dataset.creation-review-')) {
          await until(() {
            final button = t.widget(finder);
            return button is! ButtonStyleButton || button.onPressed != null;
          }, '$key exposes an enabled original action');
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
      final owner = await openDatabase();
      await until(() => owner.canWrite, 'owner ready');
      final fieldId = owner.views.first.columns.first.id;
      faults.titleField = fieldId;
      await domain(
        () => owner.createField(
          '协作状态',
          'singleSelect',
          options: const [
            DatasetOptionDraft(name: '待处理', color: 1),
            DatasetOptionDraft(name: '进行中', color: 2),
          ],
        ),
      );
      final status = owner.views.first.columns.singleWhere(
        (f) => f.name == '协作状态',
      );
      final boardId = await domain(
        () => owner.createBoardView('待核对看板', status),
      );
      const intended = '待核对的新增卡片😀';
      await domain(
        () => owner.configureViewRules(
          owner.views.singleWhere((v) => v.id == boardId),
          filters: [
            {
              'id': 'review-prefill',
              'filter_type': 2,
              'field_id': fieldId,
              'ty': 0,
              'condition': 0,
              'content': intended,
            },
          ],
          sorts: [],
        ),
      );
      await domain(owner.synchronize);
      await showActor(1);
      var member = await openDatabase(refresh: true);
      await until(
        () => member.canWrite && member.views.any((v) => v.id == boardId),
        'original member receives board',
      );
      Future<void> board() async {
        await tap('dataset.view-picker');
        await tap('dataset.view-option.$boardId');
      }

      await board();
      final groupId = status.options.last.id;
      final before = owner.views.first.allRows.map((r) => r.id).toSet();
      faults.mode = 'lost-response';
      await tap('dataset.group-add.$groupId');
      await until(
        () => !member.rowCreationPending && member.creationNeedsReview,
        'unknown creation retained for review',
      );
      expect(member.creationReview!.phase, 'submitted');
      expect(member.creationReview!.rowId, isNull);
      expect(faults.posts, 1);
      await until(
        () => owner.views.first.allRows.length == before.length + 1,
        'other original Host sees one accepted row',
      );
      final lostRow = owner.views.first.allRows
          .singleWhere((r) => !before.contains(r.id))
          .id;
      await shot('01-unknown-result-retains-input');
      await tap('dataset.creation-review');
      expect(find.textContaining('尚未确认服务端'), findsOneWidget);
      expect(
        t
            .widget<SelectableText>(
              find.byKey(const ValueKey('dataset.creation-review-input')),
            )
            .data,
        contains(intended),
      );
      await shot('02-preserved-original-title-and-choice');
      final oldClipboard = await t.runAsync(
        () => Clipboard.getData('text/plain'),
      );
      await tap('dataset.creation-review-copy');
      final copied = await t.runAsync(() => Clipboard.getData('text/plain'));
      expect(copied!.text, allOf(contains(intended), contains('进行中')));
      await t.runAsync(
        () => Clipboard.setData(ClipboardData(text: oldClipboard?.text ?? '')),
      );
      await tap('dataset.creation-review-sync');
      await until(
        () =>
            member.views.first.allRows.any((r) => r.id == lostRow && r.loaded),
        'sync reads original accepted row',
      );
      expect(
        member.creationNeedsReview,
        isTrue,
        reason: 'No value-matching guess for unknown row identity',
      );
      expect(faults.posts, 1);
      checks.add(
        'lost-real-cloud-response-preserves-title-and-choice-without-replay-or-value-matching',
      );
      debugPrint(
        'CREATION_REVIEW_FINISH_READY=${jsonEncode({'pending': member.reviewOperationPending, 'canManage': member.canManageCreationReviews, 'buttonEnabled': t.widget<TextButton>(find.byKey(const ValueKey('dataset.creation-review-finish'))).onPressed != null})}',
      );
      await tap('dataset.creation-review-finish');
      expect(find.textContaining('这不会撤销或删除云端'), findsOneWidget);
      await tap('dataset.creation-review-confirm');
      await until(
        () => !member.creationNeedsReview && !member.reviewOperationPending,
        'manual review saved privately',
      );
      expect(member.reviewedCreations.single.input.cells[fieldId], intended);
      expect(owner.views.first.allRows.any((r) => r.id == lostRow), isTrue);
      expect(faults.posts, 1);
      await tap('dataset.creation-history');
      await shot('03-private-review-archive');
      await tap('dataset.creation-history-close');
      checks.add(
        'manual-review-ends-local-wait-without-cloud-delete-resend-or-creation-undo',
      );
      final tab = actors[1].workspace.activeTab!;
      actors[1].workspace.closeTab(tab);
      await until(
        () => !actors[1].workspace.tabs.contains(tab),
        'final checkpoint released',
      );
      member = await openDatabase();
      await until(
        () => member.canWrite && member.reviewedCreations.length == 1,
        'private archive restored after actual Host reopen',
      );
      expect(member.creationNeedsReview, isFalse);
      expect(faults.posts, 1);
      await tap('dataset.creation-history');
      expect(find.textContaining('不代表云端创建成功'), findsOneWidget);
      await shot('04-private-archive-after-host-reopen');
      await tap('dataset.creation-history-close');
      checks.add(
        'actual-host-reopen-retains-private-input-archive-and-does-not-replay',
      );
      await board();
      faults.mode = 'mismatched-input';
      await tap('dataset.group-add.$groupId');
      await until(
        () => !member.rowCreationPending && member.creationNeedsReview,
        'accepted mismatch retains known original row',
      );
      final accepted = member.creationReview!;
      expect(accepted.phase, 'accepted');
      final knownRow = accepted.rowId!;
      expect(faults.posts, 2);
      await tap('dataset.creation-review');
      await tap('dataset.creation-review-sync');
      await until(
        () =>
            member.views.first.allRows.any((r) => r.id == knownRow && r.loaded),
        'known row read from original Cloud',
      );
      expect(member.creationNeedsReview, isTrue);
      await tap('dataset.creation-review-open-row');
      expect(find.textContaining('未匹配当前筛选'), findsOneWidget);
      expect(find.text('服务实际保存的不同输入'), findsOneWidget);
      await shot('05-known-row-outside-filter-uses-original-record-editor');
      await tap('dataset.record-field.$fieldId');
      await t.enterText(
        find.byKey(const ValueKey('dataset.cell-input')),
        intended,
      );
      await tap('dataset.cell-apply');
      await tap('dataset.record-close');
      await domain(member.synchronize);
      await until(
        () => !member.creationNeedsReview,
        'fresh original membership and row cells confirm known receipt',
      );
      await tap('dataset.creation-review-close');
      await until(
        () => owner.views.first.allRows.any(
          (r) =>
              r.id == knownRow &&
              r.loaded &&
              owner
                      .readRow(db['viewId'] as String, knownRow)
                      .cells[fieldId]
                      ?.value ==
                  intended,
        ),
        'owner receives corrected existing record',
      );
      expect(faults.posts, 2);
      checks.add(
        'known-receipt-opens-filtered-out-original-row-and-fresh-sync-confirms-corrected-cells',
      );
      faults.mode = 'normal';
      await tap('dataset.group-add.$groupId');
      await until(
        () => find
            .byKey(const ValueKey('dataset.record-dialog'))
            .evaluate()
            .isNotEmpty,
        'explicit next creation returns original editor',
      );
      await tap('dataset.record-close');
      await until(
        () => owner.views.first.allRows.length == before.length + 3,
        'three explicit creations, no replay',
      );
      expect(faults.posts, 3);
      expect(member.creationNeedsReview, isFalse);
      await shot('06-explicit-next-creation-no-duplicate-replay');
      checks.add(
        'healthy-explicit-next-creation-keeps-original-receipt-and-only-three-explicit-posts',
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
          'authenticatedHostContexts': actors.length,
          'originalCreationPosts': faults.posts,
          'faults':
              'Real original Cloud writes; only client response delivery or one submitted title is fault-injected',
          'fullProductAcceptance': 'not-complete',
        }),
      );
      debugPrint('SMARTDOC_CREATION_REVIEW_CHECKS=${checks.length}');
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

final class _CreationFaultClient extends Fake implements HttpClient {
  final delegate = HttpClient();
  String mode = 'normal';
  String? titleField;
  int posts = 0;
  @override
  Future<HttpClientRequest> openUrl(String method, Uri url) async {
    final request = await delegate.openUrl(method, url);
    if (method == 'POST' && url.path.endsWith('/row')) {
      posts++;
      return _CreationFaultRequest(request, mode, titleField!);
    }
    return request;
  }

  @override
  void close({bool force = false}) => delegate.close(force: force);
}

final class _CreationFaultRequest extends Fake implements HttpClientRequest {
  _CreationFaultRequest(this.delegate, this.mode, this.titleField);
  final HttpClientRequest delegate;
  final String mode;
  final String titleField;
  @override
  HttpHeaders get headers => delegate.headers;
  @override
  set followRedirects(bool value) => delegate.followRedirects = value;
  @override
  void abort([Object? exception, StackTrace? stackTrace]) =>
      delegate.abort(exception, stackTrace);
  @override
  void write(Object? object) {
    if (mode != 'mismatched-input') {
      delegate.write(object);
      return;
    }
    final body = jsonDecode(object as String) as Map;
    (body['cells'] as Map)[titleField] = '服务实际保存的不同输入';
    delegate.write(jsonEncode(body));
  }

  @override
  Future<HttpClientResponse> close() async {
    final response = await delegate.close();
    if (mode != 'lost-response') return response;
    final body = jsonDecode(await utf8.decoder.bind(response).join()) as Map;
    expect(response.statusCode, 200);
    expect(body['code'], 0);
    throw const SocketException(
      'Synthetic lost response after original Cloud accepted the row',
    );
  }
}
