import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_clipboard/openmuse_resource_clipboard.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';
import 'package:path_provider/path_provider.dart';
import 'support/device_result.dart';

// Supplemental OS-level regression. The peer is an explicitly isolated HTTP
// service on this device; this does not claim a new phone-to-VPS model run.
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
    'native local resources survive remote failure and same-key reconnect',
    (tester) async {
      final support = await getApplicationSupportDirectory();
      final output = await Directory(
        '${support.path}/acceptance-offline-recovery',
      ).create(recursive: true);
      final root = await Directory.systemTemp.createTemp(
        'resource-offline-native-',
      );
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      registry.install(OpenMuseResourceNotesPlugin());
      await registry.activate('com.openmuse.resource-notes');
      final center = await NativeResourceCenter.open(
        scope: 'offline-native-qa',
        deviceRef: 'phone',
        deviceLabel: 'This phone',
        registry: registry,
        storageRoot: Directory('${root.path}/phone'),
      );
      final peer = await NativeResourceCenter.open(
        scope: 'offline-native-qa',
        deviceRef: 'peer',
        deviceLabel: 'Isolated peer',
        registry: registry,
        storageRoot: Directory('${root.path}/peer'),
      );
      var allowed = false;
      final server = ResourceRpcHttpHost(
        service: peer.rpc,
        authorize: (_) async => allowed
            ? ResourcePeer(
                scope: 'offline-native-qa',
                deviceRef: 'phone',
                workspaceRef: 'personal',
                expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
              )
            : null,
      );
      await server.start();
      addTearDown(() async {
        await center.close();
        await server.close();
        await peer.close();
        registry.dispose();
        if (await root.exists()) await root.delete(recursive: true);
      });
      Future<void> connect() => center.connect(
        key: 'same-key',
        peerRef: 'peer',
        endpoint: Uri.parse(
          'http://127.0.0.1:${server.port}/openmuse/resource-center/v1',
        ),
        headers: () => {},
      );
      await expectLater(connect(), throwsA(isA<ResourceFailure>()));
      allowed = true;
      await connect();
      expect(center.hub.visibleProviders.any((p) => !p.source.local), isTrue);
      final record = await center.documents.create(
        parentRef: 'root',
        name: 'Offline field notes.md',
        mediaType: 'text/markdown',
        bytes: 'Local bytes remain available'.codeUnits,
        idempotencyKey: 'native-offline-notes',
        access: center.hub.access('setup'),
      );
      allowed = false;
      await tester.pumpWidget(
        MaterialApp(
          locale: const Locale('en'),
          supportedLocales: const [Locale('en')],
          localizationsDelegates: [
            OpenMuseStringsDelegate(OpenMuseCatalog.builtIn),
          ],
          home: Scaffold(
            body: ResourceCenterView(controller: center.controller),
          ),
        ),
      );
      Future<void> tap(String key) async {
        await tester.pumpAndSettle();
        await tester.runAsync(() async {
          await tester.tap(find.byKey(ValueKey(key)));
          await Future<void>.delayed(const Duration(milliseconds: 200));
        });
        await tester.pumpAndSettle();
      }

      if (Platform.isAndroid) await binding.convertFlutterSurfaceToImage();
      Future<void> shot(String name) async {
        await tester.pumpAndSettle();
        final bytes = await binding.takeScreenshot(name);
        await File('${output.path}/$name.png').writeAsBytes(bytes);
      }

      await tap('resource-section-remote');
      await tap('resource-refresh');
      expect(center.controller.failure, isA<ResourceFailure>());
      await shot('01-remote-unavailable');
      await tap('resource-section-local');
      await tap('resource-refresh');
      expect(center.controller.failure, isNull);
      await tap('resource-source-${center.documents.source.key}');
      expect(find.text(record.name), findsOneWidget);
      await shot('02-local-available');
      await tester.runAsync(() async {
        await tester.tap(find.text(record.name));
        await Future<void>.delayed(const Duration(milliseconds: 300));
      });
      await tester.pumpAndSettle();
      final field = tester.widget<TextField>(
        find.byKey(const ValueKey('resource-notes-content')),
      );
      expect(field.controller!.text, 'Local bytes remain available');
      await shot('03-local-opened');
      await tester.runAsync(() async {
        await ResourceClipboard.write(record.reference());
        expect(
          (await ResourceClipboard.read()).toUri(),
          record.reference().toUri(),
        );
      });
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );
}
