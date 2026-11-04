import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  test(
    'failed pairing setup can reconnect with the same session key',
    () async {
      final root = await Directory.systemTemp.createTemp('resource-recovery-');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final pc = await NativeResourceCenter.open(
        scope: 'account-1',
        deviceRef: 'pc-1',
        deviceLabel: 'PC',
        registry: registry,
        storageRoot: Directory('${root.path}/pc'),
      );
      final phone = await NativeResourceCenter.open(
        scope: 'account-1',
        deviceRef: 'phone-1',
        deviceLabel: 'Phone',
        registry: registry,
        storageRoot: Directory('${root.path}/phone'),
      );
      var authorized = false;
      final server = ResourceRpcHttpHost(
        service: pc.rpc,
        authorize: (_) async => authorized
            ? ResourcePeer(
                scope: 'account-1',
                deviceRef: 'phone-1',
                workspaceRef: 'personal',
                expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
              )
            : null,
      );
      await server.start();
      addTearDown(() async {
        await phone.close();
        await server.close();
        await pc.close();
        registry.dispose();
        await root.delete(recursive: true);
      });
      Future<void> connect() => phone.connect(
        key: 'same-pairing-key',
        peerRef: 'pc-1',
        endpoint: Uri.parse(
          'http://127.0.0.1:${server.port}/openmuse/resource-center/v1',
        ),
        headers: () => {},
      );
      await expectLater(connect(), throwsA(isA<ResourceFailure>()));
      authorized = true;
      await connect();
      expect(
        phone.hub.visibleProviders.where((p) => !p.source.local),
        isNotEmpty,
      );
    },
  );
}
