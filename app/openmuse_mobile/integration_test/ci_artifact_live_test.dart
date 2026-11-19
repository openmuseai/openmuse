import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as application;
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mcp_app_runtime/src/mcp_resource_source.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  recordDeviceResult(binding);
  testWidgets(
    'installed CI artifact opens digest-pinned read-only native SmartDoc',
    (tester) async {
      final support = await getApplicationSupportDirectory(),
          ids =
              jsonDecode(
                    await File(
                      '${(await getApplicationSupportDirectory()).path}/ci-artifact-e2e.json',
                    ).readAsString(),
                  )
                  as Map;
      final milestones = <String>[];
      binding.reportData = {'milestones': milestones};
      Future<void> until(bool Function() ready, String label) async {
        final end = DateTime.now().add(const Duration(seconds: 90));
        while (!ready() && DateTime.now().isBefore(end)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump(const Duration(milliseconds: 200));
        }
        expect(ready(), isTrue, reason: label);
        milestones.add(label);
        debugPrint('CI_ARTIFACT_PASS $label');
      }

      await application.main();
      await tester.pump();
      await until(
        () => find
            .byType(application.OpenMuseMobileApplication)
            .evaluate()
            .isNotEmpty,
        'normal installed Host started',
      );
      final host = tester.widget<application.OpenMuseMobileApplication>(
        find.byType(application.OpenMuseMobileApplication),
      );
      await until(
        () =>
            host.authenticationPlugin.authentication.snapshot.identity != null,
        'existing China account restored',
      );
      await until(
        () =>
            find.byType(WorkBuddyShell).evaluate().isNotEmpty &&
            tester
                    .widget<WorkBuddyShell>(find.byType(WorkBuddyShell).first)
                    .resources !=
                null,
        'native resource Hub ready',
      );
      final app = host.remoteApps.single;
      expect(app.definition['version'], '0.4.0');
      var uiConnected = false, dataReads = 0;
      app.onRequestCompleted = (method, name, agent) {
        if (name == 'ci_project_get') uiConnected = true;
        if (method == 'resources/read') dataReads++;
      };
      await until(
        () =>
            find.text('Muse-CI').hitTestable().evaluate().isNotEmpty ||
            find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty,
        'normal CI entry ready',
      );
      if (find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty) {
        final top = tester
            .getTopLeft(find.byKey(const ValueKey('wb-bind-desktop')))
            .dy;
        await tester.tapAt(Offset(12, top / 2));
        await tester.pump();
      }
      await until(
        () => find.text('Muse-CI').hitTestable().evaluate().isNotEmpty,
        'normal CI entry visible',
      );
      await tester.tap(find.text('Muse-CI').hitTestable().first);
      await until(
        () => uiConnected,
        'actual installed 0.4 UI connected through MCP',
      );
      Map? artifact;
      await tester.runAsync(() async {
        final result = await app.request('tools/call', {
          'name': 'ci_artifact_get',
          'arguments': {
            'projectId': ids['projectId'],
            'nodeId': ids['nodeId'],
            'attempt': ids['attempt'],
            'index': ids['index'],
          },
        });
        expect(result['isError'], isNot(true));
        artifact = result['structuredContent'] as Map;
      });
      expect(artifact, isNotNull);
      expect((artifact!['artifact'] as Map)['sha256'], ids['sha256']);
      expect(artifact!['smartDocument'], isA<Map>());
      milestones.add(
        'historical artifact digest and original native document verified',
      );
      final reference = checkedMcpResourceLink(
        artifact!['resourceLink'],
        host.authenticationPlugin.authentication.snapshot.identity!.subject,
        app.id,
      );
      final routing = ResourceLinkRouting.maybeOf(
        tester.element(find.byType(WebViewWidget).first),
      );
      expect(routing, isNotNull);
      await tester.runAsync(() async {
        unawaited(routing!.open(reference.toUri().toString()));
      });
      await until(
        () =>
            find.byType(SmartDocumentView).evaluate().isNotEmpty ||
            find
                .byKey(const ValueKey('resource-handler-open'))
                .evaluate()
                .isNotEmpty,
        'artifact reached normal native resource dispatch',
      );
      if (find
          .byKey(const ValueKey('resource-handler-open'))
          .evaluate()
          .isNotEmpty) {
        await tester.tap(find.byKey(const ValueKey('resource-handler-open')));
        await tester.pump();
      }
      await until(
        () => find.byType(SmartDocumentView).evaluate().isNotEmpty,
        'historical artifact opened full SmartDocumentView',
      );
      final view = tester.widget<SmartDocumentView>(
        find.byType(SmartDocumentView).first,
      );
      expect(view.controller.objectId, ids['nativeObjectId']);
      expect(
        view.controller.blocks.any(
          (b) =>
              b.delta?.toPlainText().contains(ids['expectedText'] as String) ==
              true,
        ),
        isTrue,
      );
      milestones.add('original artifact content rendered by native SmartDoc');
      expect(view.controller.canWrite, isFalse);
      expect(view.editor.editable, isFalse);
      expect(view.controller.provider, isNull);
      expect(view.realtime, isNull);
      milestones.add(
        'snapshot is read-only and never attaches to the mutable cloud document',
      );
      final session = ResourceEditorSessionScope.maybeOf(
        tester.element(find.byType(SmartDocumentView).first),
      );
      expect(session, isNotNull);
      expect(session!.record.version.digest, ids['sha256']);
      expect(session.record.readOnly, isTrue);
      expect(dataReads, greaterThan(0));
      milestones.add(
        'generic MCP data provider validated exact bytes and current account',
      );
      final report = {
        'passed': true,
        'milestones': milestones,
        'projectId': ids['projectId'],
        'nodeId': ids['nodeId'],
        'attempt': ids['attempt'],
        'sha256': ids['sha256'],
        'nativeObjectId': view.controller.objectId,
        'readOnly': true,
        'nativeDataReads': dataReads,
        'uiVersion': app.definition['version'],
      };
      await File(
        '${support.path}/ci-artifact-live-e2e.json',
      ).writeAsString(jsonEncode(report), flush: true);
      app.onRequestCompleted = null;
    },
    timeout: const Timeout(Duration(minutes: 7)),
  );
}
