// Device-only acceptance against the existing US account and Linux desktop.
// No credentials, fake directory, test relay or model endpoint are injected.
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_mobile/native_dsh_page.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:path_provider/path_provider.dart';

import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  SemanticsHandle? semantics;
  setUpAll(() async {
    semantics = binding.ensureSemantics();
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  tearDownAll(() => semantics?.dispose());
  testWidgets(
    'iOS production account to US VPS resource lifecycle',
    (tester) async {
      tester.testTextInput.register();
      addTearDown(tester.testTextInput.unregister);
      final support = await getApplicationSupportDirectory();
      final output = await Directory(
        '${support.path}/acceptance-vps-live',
      ).create(recursive: true);
      final previousDebugPrint = debugPrint;
      debugPrint = (String? message, {int? wrapWidth}) {
        if (message?.startsWith('Resource acceptance diagnostic: ') == true) {
          File(
            '${output.path}/resource-errors.log',
          ).writeAsStringSync('$message\n', mode: FileMode.append);
        }
        previousDebugPrint(message, wrapWidth: wrapWidth);
      };
      addTearDown(() => debugPrint = previousDebugPrint);
      final command = File('${support.path}/acceptance-vps-command.json');
      if (await command.exists()) await command.delete();
      final proof = <String, Object?>{
        'run': DateTime.now().toUtc().toIso8601String(),
        'liveAccount': true,
        'liveDesktop': true,
        'destination': 'https://us.openmuseai.com',
      };
      Future<void> phase(String value) async {
        proof['phase'] = value;
        await File(
          '${output.path}/proof.json',
        ).writeAsString(jsonEncode(proof), flush: true);
        debugPrint('VPS acceptance phase: $value');
      }

      Future<void> until(
        bool Function() predicate,
        String label, {
        Duration timeout = const Duration(seconds: 90),
      }) async {
        final deadline = DateTime.now().add(timeout);
        while (DateTime.now().isBefore(deadline)) {
          await tester.pump(const Duration(milliseconds: 150));
          if (predicate()) return;
          await Future<void>.delayed(const Duration(milliseconds: 150));
        }
        throw TestFailure('Timed out: $label');
      }

      Future<void> tap(String key) async {
        final finder = find.byKey(ValueKey(key));
        await until(() => finder.evaluate().isNotEmpty, key);
        await tester.ensureVisible(finder);
        await tester.pump(const Duration(milliseconds: 200));
        await tester.tap(finder);
        await tester.pump(const Duration(milliseconds: 500));
      }

      Future<void> shot(String name) async {
        await tester.pump(const Duration(milliseconds: 500));
        await File(
          '${output.path}/$name.png',
        ).writeAsBytes(await binding.takeScreenshot(name));
      }

      Future<void> gate(String expected) async {
        final deadline = DateTime.now().add(const Duration(minutes: 18));
        while (DateTime.now().isBefore(deadline)) {
          if (await command.exists()) {
            final data = jsonDecode(await command.readAsString()) as Map;
            if (data['phase'] == expected) return;
          }
          await tester.pump(const Duration(milliseconds: 250));
          await Future<void>.delayed(const Duration(milliseconds: 250));
        }
        throw TestFailure('Desktop review gate timed out: $expected');
      }

      try {
        await phase('boot');
        await mobile.main();
        await until(() {
          final us = find.byKey(const ValueKey('region-us'));
          return find
                  .byType(mobile.OpenMuseMobileApplication)
                  .evaluate()
                  .isNotEmpty ||
              us.evaluate().isNotEmpty;
        }, 'region bootstrap');
        if (find.byKey(const ValueKey('region-us')).evaluate().isNotEmpty) {
          await tap('region-us');
          await tap('region-continue');
        }
        await until(
          () => find
              .byType(mobile.OpenMuseMobileApplication)
              .evaluate()
              .isNotEmpty,
          'production application',
        );
        final application = tester.widget<mobile.OpenMuseMobileApplication>(
          find.byType(mobile.OpenMuseMobileApplication),
        );
        await until(
          () => application
              .authenticationPlugin
              .authentication
              .snapshot
              .isAuthenticated,
          'existing account login',
          timeout: const Duration(minutes: 3),
        );
        application.localeController?.selectTag('en');
        final paired = application.pairedDesktopPlugin.controller;
        await phase('discover-desktop');
        await paired.directory!.reconcile();
        await until(
          () => paired.devices.any(
            (d) =>
                d.online &&
                d.platform.toLowerCase().contains('linux') &&
                d.supportsPairedDesktop &&
                d.transportOrigin?.host == 'us.openmuseai.com',
          ),
          'US Linux directory',
        );
        final device = paired.devices.firstWhere(
          (d) =>
              d.online &&
              d.platform.toLowerCase().contains('linux') &&
              d.supportsPairedDesktop &&
              d.transportOrigin?.host == 'us.openmuseai.com',
        );
        expect(
          await paired.connectDevice(device),
          isTrue,
          reason: 'Real account device grant',
        );
        final connection = paired.snapshot.connection!;
        await until(
          () => find.byType(WorkBuddyShell).evaluate().isNotEmpty,
          'workbench',
        );
        final shell = tester.widget<WorkBuddyShell>(
          find.byType(WorkBuddyShell),
        );
        await until(
          () =>
              tester
                  .widget<WorkBuddyShell>(find.byType(WorkBuddyShell))
                  .resources !=
              null,
          'account resource center',
        );
        final center = tester
            .widget<WorkBuddyShell>(find.byType(WorkBuddyShell))
            .resources!;
        Object? lastResourceFailure;
        void recordFailure() {
          final error = center.controller.failure;
          if (error == null || identical(error, lastResourceFailure)) return;
          lastResourceFailure = error;
          File('${output.path}/resource-errors.log').writeAsStringSync(
            'Controller failure: ${error is ResourceFailure ? error.code : error.runtimeType}\n',
            mode: FileMode.append,
          );
        }

        center.controller.addListener(recordFailure);
        addTearDown(() => center.controller.removeListener(recordFailure));
        final buddy = shell.controller;
        final native = DshNativeGatewayClient(
          origin: Uri.parse(connection.session.origin),
          bootstrapPath: connection.session.path,
        );
        await native.initialize();
        final workspaces = await native.listWorkspaces();
        native.close();
        expect(workspaces, isNotEmpty);
        final deviceId = 'paired.${connection.deviceRef}';
        buddy.upsertDevice(
          WbDevice(
            id: deviceId,
            name: connection.deviceName,
            kind: WbDeviceKind.local,
          ),
        );
        buddy.replacePairedCatalog(
          deviceId: deviceId,
          workspaces: [
            for (final w in workspaces)
              WbWorkspace(
                id: 'dsh.workspace.${w.workspaceId}',
                name: w.title,
                deviceId: deviceId,
              ),
          ],
          sessions: [],
        );
        buddy.selectDevice(deviceId);
        final workspace = workspaces.firstWhere(
          (w) => w.title == 'Workspace',
          orElse: () => workspaces.first,
        );
        buddy.selectWorkspace('dsh.workspace.${workspace.workspaceId}');
        await until(
          () => center.hub.visibleProviders.any(
            (p) => !p.source.local && p.source.mountLabel == 'Workspace',
          ),
          'live remote mounts',
        );
        await phase('remote-ui');
        await tap('wb-tab-Home');
        await tap('resource-section-remote');
        await shot('01-remote-workspaces');
        final tree = center.hub.visibleProviders.firstWhere(
          (p) => !p.source.local && p.source.mountLabel == 'Workspace',
        );
        final catalogRollover = paired.connectDevice(device, force: true);
        await tap('resource-source-${tree.source.key}');
        expect(await catalogRollover, isTrue);
        await until(
          () => find.text('Resource Acceptance 20261102').evaluate().isNotEmpty,
          'remote nested directory',
        );
        await tester.tap(find.text('Resource Acceptance 20261102'));
        await tester.pump(const Duration(milliseconds: 500));
        await until(
          () => find.text('Research').evaluate().isNotEmpty,
          'Research directory',
        );
        await tester.tap(find.text('Research'));
        await tester.pump(const Duration(milliseconds: 500));
        await until(
          () => find.text('Remote brief.md').evaluate().isNotEmpty,
          'remote resource',
        );
        await shot('02-remote-directory');
        final openRollover = paired.connectDevice(device, force: true);
        await tester.tap(find.text('Remote brief.md'));
        expect(await openRollover, isTrue);
        await until(
          () => find
              .byKey(const ValueKey('resource-notes-content'))
              .evaluate()
              .isNotEmpty,
          'remote handler',
        );
        expect(
          tester
              .widget<TextField>(
                find.byKey(const ValueKey('resource-notes-content')),
              )
              .readOnly,
          isTrue,
        );
        await shot('03-remote-open');
        await tester.pageBack();
        await tester.pump(const Duration(milliseconds: 500));
        // The section resets the selected source before creating a local file.
        await tap('resource-section-local');
        await tap('resource-new');
        await shot('04-contributed-creators');
        await tap(
          'resource-creator-com.openmuse.resource-notes/notes.markdown',
        );
        final suffix = DateTime.now().millisecondsSinceEpoch;
        final filename = 'VPS phone reference $suffix.md';
        final marker = 'RESOURCE_VPS_IOS_$suffix';
        await tester.enterText(
          find.byKey(const ValueKey('resource-create-name')),
          filename,
        );
        await tap('resource-create-confirm');
        await tap('resource-edit');
        await tester.enterText(
          find.byKey(const ValueKey('resource-notes-content')),
          '# Phone original\n\nVerification phrase: $marker',
        );
        await tap('resource-save');
        await until(
          () => find.text('Saved to the source').evaluate().isNotEmpty,
          'local persistence',
        );
        await shot('05-phone-original-v1');
        await tester.pageBack();
        await tester.pump(const Duration(milliseconds: 500));
        await tap('resource-section-recent');
        final record = center.hub.recent
            .firstWhere((r) => r.record.name == filename)
            .record;
        await tap('resource-more-${record.identity.resource}');
        await tap('resource-menu-copy');
        buddy.startNewTask();
        await tester.pump(const Duration(milliseconds: 500));
        Future<void> sendReference({
          required bool retain,
          required int responses,
        }) async {
          await tap('resource-paste');
          await until(
            () => find
                .byKey(
                  ValueKey('resource-reference-${record.identity.resource}'),
                )
                .evaluate()
                .isNotEmpty,
            'typed reference pasted',
          );
          if (retain) await tap('resource-retain-toggle');
          await shot(retain ? '08-retained-draft' : '06-origin-draft');
          if (find.byKey(const ValueKey('wb-composer')).evaluate().isEmpty) {
            await tap('wb-input-mode-toggle');
          }
          await tester.enterText(
            find.byKey(const ValueKey('wb-composer')),
            'Return only the exact verification phrase from the attached resource. Do not call tools.',
          );
          await tap('wb-send');
          await until(
            () {
              final pages = find.byType(NativeDshPage).evaluate();
              if (pages.isEmpty) return false;
              final snapshot =
                  (pages.first.widget as NativeDshPage).handle?.snapshot;
              return center.draft.isEmpty &&
                  snapshot != null &&
                  !snapshot.running &&
                  snapshot.rows
                          .where(
                            (r) =>
                                (r.kind == DshConversationRowKind.assistant ||
                                    r.kind ==
                                        DshConversationRowKind.liveAssistant) &&
                                r.text.contains(marker),
                          )
                          .length >=
                      responses;
            },
            'actual VPS model response $responses',
            timeout: const Duration(minutes: 3),
          );
          await shot(retain ? '09-retained-response' : '07-origin-response');
        }

        await phase('model-origin');
        await sendReference(retain: false, responses: 1);
        await phase('model-retained');
        await sendReference(retain: true, responses: 2);
        final snapshot = tester
            .widget<NativeDshPage>(find.byType(NativeDshPage))
            .handle!
            .snapshot!;
        final activeConnection = paired.snapshot.connection!;
        final metadata = DshNativeGatewayClient(
          origin: Uri.parse(activeConnection.session.origin),
          bootstrapPath: activeConnection.session.path,
        );
        await metadata.initialize();
        final activeOptions = await metadata.sessionOptions(
          snapshot.sessionId!,
        );
        metadata.close();
        final actualModel = snapshot.model ?? activeOptions.defaultModel.model;
        expect(
          actualModel,
          'deepseek-v4.1-flash',
          reason: 'User-approved existing VPS model',
        );
        final references = <ResourceReference>[];
        for (final row in snapshot.rows.where(
          (r) => r.kind == DshConversationRowKind.user,
        )) {
          for (final match in RegExp(
            r'openmuse-resource://v1/[A-Za-z0-9_-]+',
          ).allMatches(row.text)) {
            final ref = ResourceReference.fromUri(match.group(0)!);
            if (ref.identity.resource == record.identity.resource) {
              references.add(ref);
            }
          }
        }
        expect(references.length, greaterThanOrEqualTo(2));
        await center.documents.commit(
          record.identity.resource,
          record.version,
          utf8.encode(
            'Phone current version V2; historical references must remain V1.',
          ),
          resourceOperationKey('vps-v2'),
          center.hub.access('edit'),
        );
        proof.addAll({
          'marker': marker,
          'resource': record.identity.toJson(),
          'pinnedVersion': record.version.toJson(),
          'filename': filename,
          'sessionId': snapshot.sessionId,
          'model': actualModel,
          'references': references.map((r) => r.toUri().toString()).toList(),
          'assistantResponses': 2,
          'phoneOriginalAdvancedToV2': true,
          'remoteHierarchy': true,
          'remoteReadonly': true,
          'grantRolloverDuringNavigation': true,
        });
        await phase('waiting-online-review');
        await gate('offline');
        buddy.selectDevice(kCloudDeviceId);
        paired.disconnect();
        await center.disconnect();
        await Future<void>.delayed(const Duration(seconds: 15));
        await tester.pump(const Duration(milliseconds: 500));
        proof['phoneOriginBridgeStopped'] = true;
        await phase('offline-ready');
        await gate('finish');
        await phase('passed');
        expect(await paired.connectDevice(device, force: true), isTrue);
        buddy.selectDevice(deviceId);
        buddy.selectWorkspace('dsh.workspace.${workspace.workspaceId}');
        await tester.pump(const Duration(milliseconds: 750));
      } on Object catch (error) {
        proof['failureType'] = error.runtimeType.toString();
        await phase('failed');
        await shot('failure');
        rethrow;
      } finally {
        await tester.pumpWidget(const SizedBox.shrink());
        await tester.pump(const Duration(milliseconds: 500));
      }
    },
    timeout: const Timeout(Duration(minutes: 45)),
  );
}
