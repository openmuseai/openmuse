import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as application;
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_appflowy_editor/appflowy_editor.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'support/device_result.dart';

// Actual normal Host + installed CI + original native SmartDocument + CN Cloud.
// Native routing uses the CI-returned account-bound locator. Desktop separately
// verifies the visible CI button invokes the same generic ui/open-link route.
void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..framePolicy = LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  recordDeviceResult(binding);
  testWidgets(
    'CI document opens full native editor and exchanges human Agent updates through CN Cloud',
    (tester) async {
      final support = await getApplicationSupportDirectory();
      final ids =
          jsonDecode(
                await File('${support.path}/ci-cloud-e2e.json').readAsString(),
              )
              as Map;
      final milestones = <String>[];
      final report = <String, Object?>{
        'milestones': milestones,
        'scope': 'normal-main-native-SmartDocument-China-Cloud',
        'projectId': ids['projectId'],
        'documentId': ids['documentId'],
      };
      binding.reportData = report;
      Future<void> until(
        bool Function() ready,
        String step, {
        int seconds = 90,
      }) async {
        final end = DateTime.now().add(Duration(seconds: seconds));
        while (!ready() && DateTime.now().isBefore(end)) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await tester.pump(const Duration(milliseconds: 200));
        }
        if (!ready()) {
          if (find.byType(SmartDocumentView).evaluate().isNotEmpty) {
            final v = tester.widget<SmartDocumentView>(
              find.byType(SmartDocumentView).first,
            );
            report['realtimeState'] = {
              'state': v.realtime?.connection.state.name,
              'phase': v.realtime?.connection.connectionPhase,
              'failure': v.realtime?.connection.connectionFailureCode,
              'documentFailure': v.controller.collaborationFailure,
              'sync': v.controller.syncState.name,
            };
            debugPrint(
              'CI_SMARTDOC_REALTIME ${jsonEncode(report['realtimeState'])}',
            );
          }
          report['visibleTextAtFailure'] = find
              .byType(Text)
              .evaluate()
              .map((e) => (e.widget as Text).data)
              .whereType<String>()
              .take(60)
              .toList();
          report['nativeWidgetTypes'] = find
              .byWidgetPredicate(
                (w) => RegExp(
                  r'Smart|Document|Editor|Resource|Progress',
                ).hasMatch(w.runtimeType.toString()),
              )
              .evaluate()
              .map((e) => e.widget.runtimeType.toString())
              .toSet()
              .toList();
          debugPrint(
            'CI_SMARTDOC_WIDGETS ${jsonEncode(report['nativeWidgetTypes'])}',
          );
          debugPrint(
            'CI_SMARTDOC_VISIBLE ${jsonEncode(report['visibleTextAtFailure'])}',
          );
        }
        expect(ready(), isTrue, reason: step);
        milestones.add(step);
        debugPrint('CI_SMARTDOC_PASS $step');
      }

      await application.main();
      await tester.pump();
      await until(
        () => find
            .byType(application.OpenMuseMobileApplication)
            .evaluate()
            .isNotEmpty,
        'normal main restored installed China Host',
      );
      final host = tester.widget<application.OpenMuseMobileApplication>(
        find.byType(application.OpenMuseMobileApplication),
      );
      expect(
        host.runtime!.client.current.serviceUri(ServiceId.cloud).host,
        'cn.openmuseai.com',
      );
      await until(
        () =>
            host.authenticationPlugin.authentication.snapshot.identity != null,
        'existing Keystore China account restored',
      );
      final app = host.remoteApps.single;
      await until(
        () =>
            find.byType(WorkBuddyShell).evaluate().isNotEmpty &&
            tester
                    .widget<WorkBuddyShell>(find.byType(WorkBuddyShell).first)
                    .resources !=
                null,
        'native resource center ready for current account',
      );
      final uiTools = <String>{};
      app.onRequestCompleted = (method, name, agent) {
        if (name != null) uiTools.add(name);
      };
      await until(
        () =>
            find.text('Muse-CI').hitTestable().evaluate().isNotEmpty ||
            find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty,
        'normal home ready',
      );
      if (find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isNotEmpty) {
        await tester.pumpAndSettle(const Duration(milliseconds: 100));
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
        () => uiTools.contains('ci_project_get'),
        'actual installed CI UI connected through MCP',
      );
      Future<Map> tool(
        String name,
        Map<String, Object?> args, {
        bool agent = false,
      }) async {
        final result = await app.request('tools/call', {
          'name': name,
          'arguments': args,
        }, agent: agent);
        expect(result['isError'], isNot(true), reason: name);
        return result['structuredContent'] as Map;
      }

      Map? doc;
      await tester.runAsync(() async {
        doc = await tool('ci_document_get', {
          'projectId': ids['projectId'],
          'documentId': ids['documentId'],
        });
      });
      final uri = doc!['resourceUri'] as String;
      if (const bool.fromEnvironment('CI_NATIVE_SOCKET_PROBE')) {
        await tester.runAsync(() async {
          final cloud = (doc!['document'] as Map)['cloud'] as Map;
          final token = await host.authenticationPlugin.authentication
              .accessToken();
          final endpoint = Uri.parse(cloud['origin'] as String).replace(
            scheme: 'wss',
            path: '/ws/v2/${cloud['workspaceId']}',
            queryParameters: {
              'token': token,
              'deviceId': 'ci-native-mobile-transport-probe',
              'clientId': DateTime.now().microsecondsSinceEpoch.toString(),
            },
          );
          final client = HttpClient()
            ..connectionTimeout = const Duration(seconds: 15);
          WebSocket? socket;
          try {
            socket = await WebSocket.connect(
              endpoint.toString(),
              customClient: client,
            ).timeout(const Duration(seconds: 20));
            report['nativeSocketProbe'] = {
              'connected': socket.readyState == WebSocket.open,
            };
          } on Object catch (error) {
            report['nativeSocketProbe'] = {
              'connected': false,
              'failure': error is TimeoutException
                  ? 'TIMEOUT'
                  : error.runtimeType.toString(),
              'status': error is WebSocketException
                  ? error.httpStatusCode
                  : null,
            };
          } finally {
            try {
              await socket?.close().timeout(const Duration(seconds: 2));
            } on Object {
              // Closing a diagnostic socket cannot change the acceptance result.
            }
            client.close(force: true);
          }
          debugPrint(
            'CI_NATIVE_SOCKET_PROBE ${jsonEncode(report['nativeSocketProbe'])}',
          );
        });
      }
      final reference = ResourceReference.fromUri(uri);
      expect(
        reference.identity.scope,
        host.authenticationPlugin.authentication.snapshot.identity!.subject,
      );
      final route = ResourceLinkRouting.maybeOf(
        tester.element(find.byType(WebViewWidget).first),
      );
      expect(route, isNotNull);
      await tester.runAsync(() async {
        unawaited(route!.open(uri));
      });
      await until(
        () =>
            find.byType(SmartDocumentView).evaluate().isNotEmpty ||
            find
                .byKey(const ValueKey('resource-handler-open'))
                .evaluate()
                .isNotEmpty,
        'native document or handler selection reached',
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
        'CI native resource locator opened full SmartDocumentView',
      );
      final view = tester.widget<SmartDocumentView>(
            find.byType(SmartDocumentView).first,
          ),
          controller = view.controller;
      expect(controller.provider, isA<AppFlowyCollaborationProvider>());
      expect(
        (controller.provider as AppFlowyCollaborationProvider).origin.host,
        'cn.openmuseai.com',
      );
      await until(
        () => view.realtime?.connected == true,
        'original authenticated Cloud WS2 realtime connected',
      );
      expect(
        controller.blocks.any(
          (b) => b.delta?.toPlainText().contains('CI Agent 已通过真实国内云端') == true,
        ),
        isTrue,
      );
      milestones.add('full native editor read real Agent cloud contribution');
      if (ids['expectedDesktopText'] is String) {
        expect(
          controller.blocks.any(
            (b) =>
                b.delta?.toPlainText().contains(
                  ids['expectedDesktopText'] as String,
                ) ==
                true,
          ),
          isTrue,
          reason:
              'the same cloud document retains the Desktop native contribution',
        );
        milestones.add(
          'Mobile native editor read the actual Desktop contribution',
        );
      }
      // Mobile opens in reading mode. Enter editing through its actual dock;
      // applying a transaction to a read-only projection is not a user edit.
      await tester.tap(
        find
            .descendant(
              of: find.byType(SmartDocumentView),
              matching: find.text('编辑'),
            )
            .hitTestable()
            .first,
      );
      await until(
        () =>
            view.editor.editable &&
            (view.presentation?.canWrite ?? controller.canWrite),
        'native Mobile edit action enabled the authorized presentation',
      );
      final nonce = DateTime.now().microsecondsSinceEpoch.toString();
      final mobileText = ' Mobile 原生编辑器贡献 $nonce';
      final node = view.editor.document.root.children.last;
      await tester.runAsync(() async {
        final transaction = view.editor.transaction
          ..insertText(
            node,
            node.delta!.length,
            mobileText,
            attributes: {'bold': true},
          );
        await view.editor.apply(transaction);
      });
      await until(
        () => controller.blocks.any(
          (b) => b.delta?.toPlainText().contains(mobileText) == true,
        ),
        'native content owner accepted the Mobile editor transaction',
      );
      // Observe the native session's own autosave/HTTP debounce. An awaited
      // exchange may have begun in the test clock's zone before this edit;
      // keep pumping while it settles rather than manually driving sync.
      await until(
        () =>
            !controller.hasPendingLocalChanges &&
            controller.syncState == DocumentSyncState.accepted &&
            controller.localSave == LocalSaveState.durable,
        'native Cloud sync accepted the current local mutation epoch',
      );
      expect(controller.contentFailure, isNull);
      expect(controller.hasPendingLocalChanges, isFalse);
      milestones.add(
        'native editor rich text transaction accepted by real Cloud full-sync',
      );
      await tester.runAsync(() async {
        doc = await tool('ci_document_get', {
          'projectId': ids['projectId'],
          'documentId': ids['documentId'],
        }, agent: true);
      });
      expect(jsonEncode(doc!['blocks']), contains(mobileText));
      expect(jsonEncode(doc!['blocks']), contains('"bold":true'));
      milestones.add(
        'CI Agent read back Mobile rich text from original cloud document',
      );
      var realTimeUpdates = 0;
      final updates = view.realtime!.connection.packets.listen((packet) {
        final value = packet.toJson();
        if (value['kind'] == 'update' &&
            value['objectId'] == controller.objectId) {
          realTimeUpdates++;
        }
      });
      Map? contribution;
      final agentText = 'CI Agent 实时回复 Mobile $nonce';
      await tester.runAsync(() async {
        contribution = await tool('ci_agent_document_edit', {
          'projectId': ids['projectId'],
          'documentId': ids['documentId'],
          'expectedVersion': doc!['projectVersion'],
          'expectedRevision': (doc!['document'] as Map)['revision'],
          'commandId': 'native-agent-$nonce',
          'summary': '回应 Mobile 原生富文本贡献',
          'edits': [
            {'op': 'append', 'text': agentText},
          ],
        }, agent: true);
      });
      expect(contribution!['cloudSync'], 'accepted');
      expect((contribution!['agentPresence'] as Map)['connected'], isTrue);
      await until(
        () => controller.blocks.any(
          (b) => b.delta?.toPlainText().contains(agentText) == true,
        ),
        'native editor received actual Agent content from Cloud realtime',
        seconds: 40,
      );
      expect(realTimeUpdates, greaterThan(0));
      await updates.cancel();
      expect(
        controller.blocks.any(
          (b) => b.delta?.toPlainText().contains(mobileText) == true,
        ),
        isTrue,
      );
      milestones.add('human and delegated Agent contributions both preserved');
      if (ids['expectedChildNativeObjectId'] is String) {
        // Follow the real rendered href in reading mode. This exercises the
        // document's generic Host routing port, not a direct test router call.
        final done = find.byKey(const ValueKey('smart-document.mobile.done'));
        if (done.hitTestable().evaluate().isNotEmpty) {
          await tester.tap(done.hitTestable().first);
          await tester.pump();
        }
        const childLabel = 'CI Agent 子文档协作验收';
        final childLink = find.byWidgetPredicate(
          (w) => w is RichText && w.text.toPlainText().contains(childLabel),
        );
        await until(
          () => childLink.evaluate().isNotEmpty,
          'native parent renders the child reference',
        );
        await tester.ensureVisible(childLink.first);
        await tester.pump();
        // A paragraph can also contain an unlinked human contribution. Its
        // centre is not necessarily on the href; tap the rendered link glyphs.
        final rich = tester.widget<RichText>(childLink.first);
        final start = rich.text.toPlainText().indexOf(childLabel);
        final paragraph = tester.renderObject<RenderParagraph>(childLink.first);
        final box = paragraph
            .getBoxesForSelection(
              TextSelection(
                baseOffset: start,
                extentOffset: start + childLabel.length,
              ),
            )
            .first;
        // The gesture starts real TLS/file IO. Keep it in runAsync just like
        // the normal entry's initial native-resource open above.
        await tester.runAsync(
          () => tester.tapAt(
            tester.getTopLeft(childLink.first) + box.toRect().center,
          ),
        );
        await tester.pump();
        final open = find.byKey(const ValueKey('smart-document.link-open'));
        final childView = find.byWidgetPredicate(
          (w) =>
              w is SmartDocumentView &&
              w.controller.objectId == ids['expectedChildNativeObjectId'],
        );
        await until(
          () =>
              childView.evaluate().isNotEmpty ||
              open.hitTestable().evaluate().isNotEmpty ||
              find
                  .byKey(const ValueKey('resource-handler-open'))
                  .evaluate()
                  .isNotEmpty,
          'rendered native child link initiated native navigation',
        );
        if (open.hitTestable().evaluate().isNotEmpty) {
          await tester.runAsync(() => tester.tap(open.hitTestable().first));
          await tester.pump();
        }
        if (find
            .byKey(const ValueKey('resource-handler-open'))
            .evaluate()
            .isNotEmpty) {
          await tester.tap(find.byKey(const ValueKey('resource-handler-open')));
          await tester.pump();
        }
        await until(
          () => childView.evaluate().isNotEmpty,
          'rendered native child href opened the child through the normal Host',
        );
        final child = tester.widget<SmartDocumentView>(childView.first);
        expect(
          child.controller.blocks.any(
            (b) => b.delta?.toPlainText().contains('NATIVECHILDLIVE') == true,
          ),
          isTrue,
        );
        expect(
          child.controller.blocks.any(
            (b) => b.delta?.toPlainText().contains('CI Agent 子文档云端回复') == true,
          ),
          isTrue,
        );
        await until(
          () => child.realtime?.connected == true,
          'native child established original Cloud realtime',
        );
        report['nativeChildObjectId'] = child.controller.objectId;
      }
      report.addAll({
        'passed': true,
        'mobileContribution': mobileText,
        'agentContribution': agentText,
        'cloudRealtimeFrames': realTimeUpdates,
        'uiTools': uiTools.toList(),
        'nativeObjectId': controller.objectId,
      });
      await File(
        '${support.path}/ci-smartdoc-live-e2e.json',
      ).writeAsString(jsonEncode(report), flush: true);
      app.onRequestCompleted = null;
    },
    timeout: const Timeout(Duration(minutes: 8)),
  );
}
