import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:openmuse_mobile/main.dart' as application;
import 'package:openmuse_mobile/native_dsh_page.dart';
import 'package:openmuse_mobile/mobile_dsh_connection.dart';
import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  recordDeviceResult(binding);
  testWidgets(
    'China account, reviewed CI App and native DSH control the same live project',
    (tester) async {
      final milestones = <String>[];
      final report = <String, Object?>{'milestones': milestones};
      binding.reportData = report;
      final support = await getApplicationSupportDirectory();
      final bootstrap = File('${support.path}/ci-e2e-account.json');
      expect(
        await bootstrap.exists(),
        isTrue,
        reason: 'Provide an app-private one-time live account bootstrap.',
      );
      final config = jsonDecode(await bootstrap.readAsString()) as Map;
      await bootstrap.delete();
      Future<void> until(
        bool Function() ready,
        String phase, {
        int seconds = 60,
      }) async {
        final deadline = DateTime.now().add(Duration(seconds: seconds));
        while (!ready() && DateTime.now().isBefore(deadline)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump();
        }
        expect(ready(), isTrue, reason: phase);
        milestones.add(phase);
        debugPrint('CI_MOBILE_PASS $phase');
      }

      // Exercise the real configured Host composition; first-use legal UI has
      // its own acceptance suite and is never auto-accepted by this test.
      final release = await loadInstalledRelease();
      expect(release.market, 'china');
      final deployment = release.regions
          .singleWhere((r) => r.id == 'cn')
          .bundle!;
      await tester.pumpWidget(
        await application.bootOpenMuseMobile(deployment, 'zh-Hans'),
      );
      await until(
        () => find
            .byType(application.OpenMuseMobileApplication)
            .evaluate()
            .isNotEmpty,
        'full China Mobile Host composition booted; onboarding tested separately',
      );
      final host = tester.widget<application.OpenMuseMobileApplication>(
        find.byType(application.OpenMuseMobileApplication),
      );
      expect(
        host.runtime!.client.current.serviceUri(ServiceId.cloud).host,
        'cn.openmuseai.com',
      );
      final auth = host.authenticationPlugin.authentication;
      await tester.runAsync(
        () => auth.signInWithPassword(
          config['email'] as String,
          config['password'] as String,
        ),
      );
      await until(
        () => auth.snapshot.identity != null,
        'real China account authenticated and stored in Android Keystore',
      );
      final app = host.remoteApps.single;
      final uiTools = <String>[];
      app.onRequestCompleted = (method, name, agent) {
        if (name != null) uiTools.add(name);
      };
      await until(
        () => find.text('Muse-CI').evaluate().isNotEmpty,
        'installed plugin entry visible in full Mobile Host',
      );
      await tester.tap(find.text('Muse-CI').first);
      await until(
        () => find.byType(WebViewWidget).evaluate().isNotEmpty,
        'reviewed native CI WebView mounted',
      );
      await until(
        () =>
            uiTools.contains('ci_capabilities') &&
            uiTools.contains('ci_projects_list') &&
            uiTools.contains('ci_project_get'),
        'real CI UI initialized and called backend through native MCP Apps bridge',
      );
      report['uiTools'] = uiTools.toSet().toList();
      await tester.runAsync(() async {
        await File('${support.path}/ci-e2e-stage').writeAsString('ci-ui');
        await Future<void>.delayed(const Duration(seconds: 5));
      });
      Navigator.of(tester.element(find.byType(WebViewWidget))).pop();
      await tester.pump();
      final paired = host.pairedDesktopPlugin;
      await tester.runAsync(() => paired.directory!.refresh());
      await until(
        () => paired.controller.devices.any(
          (d) => d.online && d.deviceRef == config['desktopDeviceRef'],
        ),
        'same-account Desktop discovered through China device directory',
        seconds: 90,
      );
      final device = paired.controller.devices.singleWhere(
        (d) => d.deviceRef == config['desktopDeviceRef'],
      );
      final connected = await tester.runAsync(
        () => paired.controller.connectDevice(device),
      );
      expect(
        connected,
        isTrue,
        reason: paired.controller.snapshot.failureMessage,
      );
      final connection = paired.controller.snapshot.connection!;
      expect(Uri.parse(connection.session.origin).host, 'cn.openmuseai.com');
      milestones.add('same-account China relay Desktop connection established');
      final transport = MobileDshConnection(
        connection,
        MobileDshRecentMessages('ci-live-e2e'),
      );
      String? preparationFailure;
      await tester.runAsync(() async {
        try {
          await transport.prepare();
        } catch (error) {
          preparationFailure = error.toString();
        }
      });
      expect(
        preparationFailure,
        isNull,
        reason:
            'China relay native DSH bootstrap must succeed before reading sessions.',
      );
      final workspaces = (await tester.runAsync(
        transport.client.listWorkspaces,
      ))!;
      expect(workspaces, isNotEmpty);
      final created = (await tester.runAsync(
        () => transport.client.createSession(
          workspaceId: workspaces.first.workspaceId,
        ),
      ))!;
      report['mobileSessionId'] = created.sessionId;
      final handle = NativeDshSessionHandle();
      final navigationContext = tester.element(find.text('Muse-CI').first);
      unawaited(
        Navigator.of(navigationContext).push(
          MaterialPageRoute<void>(
            builder: (_) => Scaffold(
              appBar: AppBar(title: const Text('Mobile CI 真实会话验收')),
              body: NativeDshPage(
                session: connection.session,
                connection: transport,
                workspaceTitle: workspaces.first.title,
                requestedSessionId: created.sessionId,
                handle: handle,
              ),
            ),
          ),
        ),
      );
      await until(
        () => handle.ready && handle.snapshot?.phase == DshConnectionPhase.live,
        'native Mobile DSH joined real Desktop session over China relay',
      );
      final projectId = config['projectId'] as String;
      final marker = 'MOBILE_CI_E2E_${DateTime.now().millisecondsSinceEpoch}';
      final prompt =
          '这是手机 CI 操作验收。只允许通过 resources 的 apps_call_tool 操作项目 $projectId，禁止修改其他项目，禁止使用 shell。先用 ci_project_get 读取版本和计划，然后用 ci_plan_save 保留原有 title、type、budget，仅在 goal 末尾追加验收标记 $marker。再以最新 version 和新的 commandId 用 ci_run_command 执行 rerun、nodeId 为 product-spec、auto:false，查询到该节点最新尝试成功后再回复 $marker 和保存后的版本号、runId。';
      await tester.enterText(
        find.byKey(const ValueKey('dsh-native.composer')),
        prompt,
      );
      await tester.tap(find.byKey(const ValueKey('dsh-native.send')));
      await until(
        () => find
            .byKey(const ValueKey('ai-sharing-allow'))
            .evaluate()
            .isNotEmpty,
        'per-send AI recipient and sharing disclosure displayed',
      );
      // The explicit test request authorizes this synthetic CI prompt.
      await tester.tap(find.byKey(const ValueKey('ai-sharing-allow')));
      await until(
        () => handle.snapshot!.rows.any(
          (row) =>
              row.kind == DshConversationRowKind.assistant &&
              row.text.contains(marker),
        ),
        'real Mobile DSH model completed CI tool write',
        seconds: 180,
      );
      final result = await tester.runAsync(
        () => app.request('tools/call', {
          'name': 'ci_project_get',
          'arguments': {'projectId': projectId},
        }),
      );
      final project = (result!['structuredContent'] as Map)['project'] as Map;
      expect(project['goal'], contains(marker));
      final productNode = (project['graph']['nodes'] as List).singleWhere(
        (n) => n['id'] == 'product-spec',
      );
      expect(productNode['status'], 'succeeded');
      expect((productNode['attempts'] as List).length, greaterThanOrEqualTo(3));
      report['mobileNodeAttempt'] = productNode['attempts'].last['id'];
      report['projectId'] = projectId;
      report['marker'] = marker;
      report['projectVersion'] = project['version'];
      milestones.add(
        'Mobile DSH write confirmed by independent CI backend read',
      );
      await tester.runAsync(
        () => File('${support.path}/ci-e2e-stage').writeAsString('native-dsh'),
      );
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(seconds: 5)),
      );
      await File(
        '${support.path}/ci-host-e2e.json',
      ).writeAsString(jsonEncode(report), flush: true);
      app.onRequestCompleted = null;
    },
    timeout: const Timeout(Duration(minutes: 8)),
  );
}
