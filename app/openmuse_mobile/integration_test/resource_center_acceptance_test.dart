import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_clipboard/openmuse_resource_clipboard.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';
import 'package:path_provider/path_provider.dart';
import '../test/support/design_fixture.dart';
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
    'resource center: native UI, paired PC, real DSH, typed clipboard and exact versions',
    (tester) async {
      // Profile-mode iOS rejects the debug-only text-input client id -1.
      // Register the integration keyboard so enterText sends a real client id;
      // resource UI, persistence, clipboard, pairing and runtime remain native.
      if (Platform.isIOS) {
        tester.testTextInput.register();
        addTearDown(tester.testTextInput.unregister);
      }
      final support = await getApplicationSupportDirectory();
      final input = Map<String, Object?>.from(
        jsonDecode(
              await File(
                '${support.path}/acceptance-resources.json',
              ).readAsString(),
            )
            as Map,
      );
      final origin = Uri.parse(input['origin'] as String),
          run = input['run'] as String;
      final output = await Directory(
        '${support.path}/acceptance-resources/$run',
      ).create(recursive: true);
      final previousErrorHandler = FlutterError.onError;
      FlutterError.onError = (details) {
        File(
          '${output.path}/flutter-errors.log',
        ).writeAsStringSync('${details.toString()}\n\n', mode: FileMode.append);
        previousErrorHandler?.call(details);
      };
      addTearDown(() => FlutterError.onError = previousErrorHandler);
      final token = input['accessToken'] as String;
      Future<Map<String, Object?>> control(String path) async {
        final http = HttpClient();
        try {
          final request = await http.getUrl(
            origin.replace(port: input['controlPort'] as int).resolve(path),
          );
          request.headers.set('authorization', 'Bearer $token');
          final response = await request.close();
          if (response.statusCode != 200) {
            throw StateError(
              'Acceptance control failed: ${response.statusCode}',
            );
          }
          return Map<String, Object?>.from(
            jsonDecode(await response.transform(utf8.decoder).join()) as Map,
          );
        } finally {
          http.close(force: true);
        }
      }

      await control('/arm-pairing');
      await control('/clear-projections');
      final pairClient = PairedDesktopClient(
        origin: origin,
        accessToken: () async => token,
        deviceRef: 'qa-${Platform.operatingSystem}-$run',
        allowInsecureLoopback: true,
        allowInsecurePrivateNetworkForTesting: true,
      );
      final paired = PairedDesktopMobileController.direct(pairClient);
      expect(await paired.pair(input['pairingCode'] as String), isTrue);
      final connection = paired.snapshot.connection!;
      final locale = OpenMuseLocaleController(initialTag: 'en');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (a, b) async => null,
        ),
      );
      registry.install(OpenMuseResourceNotesPlugin());
      await registry.activate('com.openmuse.resource-notes');
      final center = await NativeResourceCenter.open(
        scope: input['account'] as String,
        deviceRef: 'qa-${Platform.operatingSystem}-$run',
        deviceLabel: Platform.isIOS ? 'iPhone' : 'Android',
        registry: registry,
        storageRoot: Directory('${output.path}/local'),
      );
      await center.connect(
        key: connection.grantRef,
        peerRef: connection.deviceRef,
        endpoint: origin.resolve('/openmuse/resource-center/v1'),
        headers: () => {
          HttpHeaders.cookieHeader: 'OpenMuse-Paired=${connection.grantRef}',
        },
        allowPrivateNetwork: true,
      );
      final native = DshNativeGatewayClient(
        origin: origin,
        bootstrapPath: connection.session.path,
        allowInsecureLoopback: true,
        allowInsecurePrivateNetworkForTesting: true,
      );
      await native.initialize();
      final workspaces = await native.listWorkspaces();
      native.close();
      final buddy = WorkBuddyController()..selectTab(WbTab.home);
      final device = 'paired.${connection.deviceRef}';
      buddy.upsertDevice(
        WbDevice(
          id: device,
          name: connection.deviceName,
          kind: WbDeviceKind.local,
        ),
      );
      buddy.replacePairedCatalog(
        deviceId: device,
        workspaces: [
          for (final w in workspaces)
            WbWorkspace(
              id: 'dsh.workspace.${w.workspaceId}',
              name: w.title,
              deviceId: device,
            ),
        ],
        sessions: [],
      );
      buddy.selectDevice(device);
      final shellKey = GlobalKey();
      Widget app(Brightness brightness) => DesignHost(
        locale: locale,
        brightness: brightness,
        home: ResourceLinkRouting(
          open: (uri) async {
            await openResourcePage(
              shellKey.currentContext!,
              center.controller,
              ResourceReference.fromUri(uri),
            );
          },
          child: WorkBuddyShell(
            key: shellKey,
            controller: buddy,
            pluginRegistry: registry,
            pairedDesktop: paired,
            resources: center,
          ),
        ),
      );
      Future<void> waitFor(
        Finder finder, {
        Duration timeout = const Duration(seconds: 25),
      }) async {
        final end = DateTime.now().add(timeout);
        while (DateTime.now().isBefore(end)) {
          await tester.pump(const Duration(milliseconds: 120));
          if (finder.evaluate().isNotEmpty) return;
          await Future<void>.delayed(const Duration(milliseconds: 80));
        }
        throw TestFailure('Timed out waiting for $finder');
      }

      Future<void> tap(String key) async {
        final finder = find.byKey(ValueKey(key));
        await waitFor(finder);
        await tester.ensureVisible(finder);
        await tester.pump(const Duration(milliseconds: 200));
        await tester.tap(finder);
        await tester.pump(const Duration(milliseconds: 450));
        debugPrint('Resource UI action: $key');
      }

      if (Platform.isAndroid) await binding.convertFlutterSurfaceToImage();
      Future<void> shot(String name) async {
        await tester.pump(const Duration(milliseconds: 500));
        final bytes = await binding.takeScreenshot(name);
        await File('${output.path}/$name.png').writeAsBytes(bytes);
      }

      try {
        await tester.pumpWidget(app(Brightness.light));
        await waitFor(find.byKey(const ValueKey('resource-new')));
        expect(find.byKey(const ValueKey('wb-tab-Home')), findsOneWidget);
        await shot('01-home-empty');
        await tap('resource-new');
        await shot('02-plugin-creators');
        await tap(
          'resource-creator-com.openmuse.resource-notes/notes.markdown',
        );
        await tester.enterText(
          find.byKey(const ValueKey('resource-create-name')),
          'Field notes.md',
        );
        await tap('resource-create-confirm');
        await waitFor(find.byKey(const ValueKey('resource-notes-content')));
        await tap('resource-edit');
        final marker =
            'RESOURCE_QA_V1_${Platform.operatingSystem.toUpperCase()}';
        await tester.enterText(
          find.byKey(const ValueKey('resource-notes-content')),
          '# Field notes\n\nVerification phrase: $marker',
        );
        await tester.pump();
        await tap('resource-save');
        await waitFor(find.text('Saved to the source'));
        await shot('03-editor-saved');
        await tester.pageBack();
        await tester.pump(const Duration(milliseconds: 500));
        final record = center.hub.recent.single.record;
        expect(
          utf8.decode(
            (await center.documents.read(
              record.identity.resource,
              null,
              center.hub.access('verify'),
            )).bytes,
          ),
          contains(marker),
        );
        await tap('resource-more-${record.identity.resource}');
        await shot('04-resource-actions');
        await tap('resource-menu-rename');
        await tester.enterText(
          find.byKey(const ValueKey('resource-rename-input')),
          'Field notes reviewed.md',
        );
        await tap('resource-rename-save');
        await waitFor(find.text('Field notes reviewed.md'));
        final renamed = await center.documents.describe(
          record.identity.resource,
          center.hub.access('verify'),
        );
        expect(renamed.identity, record.identity);
        await tap('resource-more-${record.identity.resource}');
        await tap('resource-menu-copy');
        // ColorOS clipboard preview briefly owns focus. Framework pointer
        // events cannot dismiss a system window, so wait for its native timeout
        // before testing the explicit paste/read action.
        await Future<void>.delayed(const Duration(seconds: 7));
        await tester.pump(const Duration(milliseconds: 300));
        final copied = await ResourceClipboard.read();
        expect(copied.identity, record.identity);
        await tap('wb-tab-Tasks');
        await shot('04b-tasks-before-paste');
        await tap('resource-paste');
        await waitFor(
          find.byKey(
            ValueKey('resource-reference-${record.identity.resource}'),
          ),
        );
        expect(center.draft.items.length, 1);
        expect((await control('/state'))['projectionCount'], 0);
        await shot('05-chat-reference-draft');
        if (find.byKey(const ValueKey('wb-composer')).evaluate().isEmpty) {
          await tap('wb-input-mode-toggle');
        }
        await tester.enterText(
          find.byKey(const ValueKey('wb-composer')),
          'Return only the exact verification phrase in the attached resource. Do not call tools.',
        );
        await tap('wb-send');
        final end = DateTime.now().add(const Duration(seconds: 90));
        while (DateTime.now().isBefore(end)) {
          await tester.pump(const Duration(milliseconds: 250));
          if (center.draft.isEmpty &&
              find.textContaining(marker).evaluate().isNotEmpty) {
            break;
          }
          await Future<void>.delayed(const Duration(milliseconds: 200));
        }
        expect(
          center.draft.isEmpty,
          isTrue,
          reason:
              'Only clear the reference draft after native prompt acceptance',
        );
        await waitFor(
          find.textContaining(marker),
          timeout: const Duration(seconds: 60),
        );
        await shot('06-real-pc-response');
        await tap('resource-paste');
        await waitFor(
          find.byKey(
            ValueKey('resource-reference-${record.identity.resource}'),
          ),
        );
        await tap('resource-retain-toggle');
        await tester.enterText(
          find.byKey(const ValueKey('wb-composer')),
          'Read this resource again and return its exact verification phrase only.',
        );
        await tap('wb-send');
        final secondEnd = DateTime.now().add(const Duration(seconds: 90));
        while (DateTime.now().isBefore(secondEnd)) {
          await tester.pump(const Duration(milliseconds: 200));
          if (center.draft.isEmpty &&
              find.textContaining(marker).evaluate().length >= 2) {
            break;
          }
          await Future<void>.delayed(const Duration(milliseconds: 200));
        }
        expect(center.draft.isEmpty, isTrue);
        await shot('06b-retained-reference-response');
        final state = await control('/state');
        expect(state['projectionCount'] as int, greaterThan(0));
        await control('/conversation');
        // The desktop UI remains open for independent UI automation/screenshot.
        await center.documents.commit(
          record.identity.resource,
          renamed.version,
          utf8.encode('Phone current version V2'),
          resourceOperationKey('v2'),
          center.hub.access('edit'),
        );
        await File('${output.path}/proof.json').writeAsString(
          jsonEncode({
            'scope': center.hub.scope,
            'origin': record.identity.toJson(),
            'pinnedVersion': record.version.toJson(),
            'marker': marker,
            'pcProjectionCreated': true,
            'nativePromptAccepted': true,
          }),
        );
        if (input['pcReviewGate'] == true) {
          final deadline = DateTime.now().add(const Duration(minutes: 2));
          while (DateTime.now().isBefore(deadline)) {
            if ((await control('/review-gate'))['released'] == true) break;
            await tester.pump(const Duration(milliseconds: 300));
            await Future<void>.delayed(const Duration(milliseconds: 300));
          }
        }
        await tap('wb-tab-Home');
        await tap('resource-section-remote');
        await shot('07-remote-workspaces');
        final tree = center.hub.visibleProviders.firstWhere(
          (p) => !p.source.local && p.source.mountLabel == 'Project files',
        );
        await tap('resource-source-${tree.source.key}');
        await waitFor(find.text('Research'));
        await tester.tap(find.text('Research'));
        await tester.pump(const Duration(milliseconds: 600));
        await waitFor(find.text('Remote brief.md'));
        await shot('08-remote-directory');
        await tester.tap(find.text('Remote brief.md'));
        await waitFor(find.byKey(const ValueKey('resource-notes-content')));
        expect(
          tester
              .widget<TextField>(
                find.byKey(const ValueKey('resource-notes-content')),
              )
              .readOnly,
          isTrue,
        );
        await shot('09-remote-open-readonly');
        await tester.pageBack();
        await tester.pump(const Duration(milliseconds: 500));
        await tester.pumpWidget(app(Brightness.dark));
        await tester.pump(const Duration(milliseconds: 500));
        await shot('10-home-dark');
        expect(tester.takeException(), isNull);
      } on Object {
        await shot('failure');
        rethrow;
      } finally {
        await tester.pumpWidget(const SizedBox.shrink());
        await tester.pump(const Duration(milliseconds: 500));
        await center.close();
        paired.dispose();
        buddy.dispose();
        registry.dispose();
        locale.dispose();
        pairClient.close();
      }
    },
  );
}
