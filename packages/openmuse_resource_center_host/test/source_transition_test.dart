import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  for (final scenario in ['list', 'describe', 'read', 'read-denied']) {
    final denied = scenario == 'read-denied';
    final operation = denied ? 'read' : scenario;
    test(
      denied
          ? 'replacement grant denial is preserved without further retries'
          : '$operation survives an in-flight grant/catalog replacement',
      () async {
        final root = await Directory.systemTemp.createTemp(
          'resource-transition-',
        );
        final registry = OpenMusePluginRegistry(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => null,
          ),
        );
        final pc = await NativeResourceCenter.open(
          scope: 'account',
          deviceRef: 'pc',
          deviceLabel: 'PC',
          registry: registry,
          storageRoot: Directory('${root.path}/pc'),
        );
        final phone = await NativeResourceCenter.open(
          scope: 'account',
          deviceRef: 'phone',
          deviceLabel: 'Phone',
          registry: registry,
          storageRoot: Directory('${root.path}/phone'),
        );
        final record = await pc.documents.create(
          parentRef: 'root',
          name: 'Proof.md',
          mediaType: 'text/markdown',
          bytes: utf8.encode('Fixed V1'),
          idempotencyKey: 'create-proof',
          access: pc.hub.access('setup'),
        );
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        final started = Completer<void>(), release = Completer<void>();
        var blockOld = false, freshReads = 0;
        server.listen((request) async {
          try {
            final body = Map<String, Object?>.from(
              jsonDecode(await utf8.decoder.bind(request).join()) as Map,
            );
            final old = request.headers.value('x-generation') == 'old';
            if (blockOld && old && body['operation'] == operation) {
              if (!started.isCompleted) started.complete();
              await release.future;
            }
            if (!old && body['operation'] == operation) {
              freshReads++;
              if (denied) {
                request.response.statusCode = 403;
                request.response.write(jsonEncode({'error': 'DENIED'}));
                await request.response.close();
                return;
              }
            }
            final value = await pc.rpc.dispatch(
              body,
              ResourcePeer(
                scope: 'account',
                deviceRef: 'phone',
                workspaceRef: 'personal',
                expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
              ),
            );
            request.response.headers.contentType = ContentType.json;
            request.response.write(jsonEncode(value));
            await request.response.close();
          } on Object {
            // The old client is deliberately closed while its response is held.
            try {
              await request.response.close();
            } on Object {
              /* closed */
            }
          }
        });
        addTearDown(() async {
          if (!release.isCompleted) release.complete();
          await phone.close();
          await server.close(force: true);
          await pc.close();
          registry.dispose();
          await root.delete(recursive: true);
        });
        Future<void> connect(String generation) => phone.connect(
          key: generation,
          peerRef: 'pc',
          endpoint: Uri.parse(
            'http://127.0.0.1:${server.port}/openmuse/resource-center/v1',
          ),
          headers: () => {'x-generation': generation},
        );
        await connect('old');
        final source = phone.hub.provider(record.identity);
        blockOld = true;
        final Future<Object?> pending = switch (operation) {
          'list' =>
            phone.controller
                .selectSource(source)
                .then((_) => phone.controller.entries),
          'describe' => phone.controller.describe(record.identity),
          _ => phone.controller.resolve(record.reference(pin: true)),
        };
        await started.future.timeout(const Duration(seconds: 5));
        await connect('new');
        if (denied) {
          await expectLater(
            pending,
            throwsA(
              isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED'),
            ),
          );
          expect(freshReads, 1);
          release.complete();
          return;
        }
        final result = await pending.timeout(const Duration(seconds: 5));
        release.complete();
        expect(phone.controller.failure, isNull);
        expect(freshReads, 1, reason: 'Only the replaced read is retried once');
        switch (operation) {
          case 'list':
            expect(
              (result as List<ResourceDirectoryEntry>).single.name,
              'Proof.md',
            );
          case 'describe':
            expect((result as ResourceRecord).identity, record.identity);
          case 'read':
            expect(
              utf8.decode((result as OpenedResource).content.bytes),
              'Fixed V1',
            );
        }
      },
    );
  }
}
