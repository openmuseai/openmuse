import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
  test(
    'launch pins filter discovery, restore and direct entry points',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'mcp-launch-pins-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final origin = Uri.parse('http://127.0.0.1:${server.port}');
      final bytes = ZipEncoder().encode(
        Archive()..add(
          ArchiveFile.string('web/index.html', '<title>Bundled editor</title>'),
        ),
      );
      final digest = sha256.convert(bytes).toString();
      Map<String, dynamic> record(String name, {String? hash}) => {
        'server': {
          'name': name,
          'version': '1.0.0',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': '$origin/package.mcpb',
              'fileSha256': hash ?? digest,
            },
          ],
        },
      };
      final approved = RemoteMcpEntry.fromRecord(record('example/approved'));
      final legacy = RemoteMcpEntry.fromRecord(record('example/legacy'));
      final original = RemoteMcpMarket(
        origin,
        File('${directory.path}/receipts.json'),
      );
      await original.install(approved, bundledArtifact: bytes);
      await original.install(legacy, bundledArtifact: bytes);
      var requests = 0;
      server.listen((request) async {
        requests++;
        request.response.write(
          jsonEncode({
            'metadata': {},
            'servers': [
              record('example/approved'),
              record('example/legacy'),
              record('example/approved', hash: '0' * 64),
            ],
          }),
        );
        await request.response.close();
      });
      addTearDown(() => server.close(force: true));
      final pins = {'example/approved@1.0.0': digest};
      final release = RemoteMcpMarket(
        origin,
        original.receiptFile,
        allowedPackages: pins,
      );
      pins['example/legacy@1.0.0'] =
          digest; // Caller mutation cannot expand the pins.
      expect((await release.list()).map((e) => e.name), ['example/approved']);
      expect((await release.installedEntries()).map((e) => e.name), [
        'example/approved',
      ]);
      expect(await release.installed(), {'example/approved@1.0.0'});
      await expectLater(
        release.install(legacy, bundledArtifact: bytes),
        throwsFormatException,
      );
      final tampered = RemoteMcpEntry.fromRecord(
        record('example/approved', hash: '0' * 64),
      );
      await expectLater(
        release.install(tampered, bundledArtifact: bytes),
        throwsFormatException,
      );
      final remote = RemoteMcpEntry(
        approved.name,
        approved.title,
        approved.version,
        approved.description,
        origin.resolve('/mcp'),
        approved.packageUri,
        digest,
        null,
      );
      await expectLater(
        release.withClient(remote, (_) async => true),
        throwsFormatException,
      );
      expect(() => release.packageDirectory(legacy), throwsFormatException);
      expect(
        requests,
        1,
        reason: 'Rejected entries must not make package or MCP requests',
      );
      await release.install(approved, bundledArtifact: bytes);
      expect(
        (await original.installedEntries()).map((e) => e.name).toSet(),
        {'example/approved', 'example/legacy'},
        reason: 'Do not erase old user records',
      );
    },
  );

  test(
    'bundled offline install verifies bytes without a registry request',
    () async {
      final directory = await Directory.systemTemp.createTemp('mcp-bundled-');
      addTearDown(() => directory.delete(recursive: true));
      final bytes = ZipEncoder().encode(
        Archive()..add(
          ArchiveFile.string(
            'web/index.html',
            '<title>OpenMuse Slides</title>',
          ),
        ),
      );
      final entry = RemoteMcpEntry.fromRecord({
        'server': {
          'name': 'example/bundled',
          'version': '1.0.0',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': 'https://unreachable.invalid/plugin.mcpb',
              'fileSha256': sha256.convert(bytes).toString(),
            },
          ],
        },
      });
      final market = RemoteMcpMarket(
        Uri.parse('https://unreachable.invalid'),
        File('${directory.path}/receipts.json'),
      );
      await expectLater(
        market.install(entry, bundledArtifact: [...bytes, 0]),
        throwsFormatException,
      );
      expect(await market.installed(), isEmpty);
      await market.install(entry, bundledArtifact: bytes);
      expect(market.hasOfflineUi(entry), isTrue);
      expect(await market.installed(), contains('example/bundled@1.0.0'));
    },
  );

  test(
    'a verified installed web plugin reopens without its registry',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'mcp-offline-test-',
      );
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() async {
        await server.close(force: true);
        await directory.delete(recursive: true);
      });
      final archive = Archive()
        ..add(ArchiveFile.string('manifest.json', '{"manifest_version":"0.3"}'))
        ..add(
          ArchiveFile.string(
            'web/index.html',
            '<!doctype html><title>offline</title>',
          ),
        );
      final bundle = ZipEncoder().encode(archive);
      final digest = sha256.convert(bundle).toString();
      final origin = Uri.parse('http://127.0.0.1:${server.port}');
      final record = {
        'server': {
          'name': 'example/offline-editor',
          'version': '1.0.0',
          'title': 'Offline editor',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': '$origin/packages/$digest.mcpb',
              'fileSha256': digest,
            },
          ],
        },
      };
      final requests = server.listen((request) async {
        if (request.uri.path == '/v0.1/servers') {
          request.response.headers.contentType = ContentType.json;
          request.response.write(
            jsonEncode({
              'servers': [record],
              'metadata': {},
            }),
          );
        } else {
          request.response.add(bundle);
        }
        await request.response.close();
      });
      final market = RemoteMcpMarket(
        origin,
        File('${directory.path}/receipts.json'),
      );
      final entry = (await market.list()).single;
      await market.install(entry);
      expect(market.hasOfflineUi(entry), isTrue);
      expect(
        await File(
          '${market.packageDirectory(entry).path}/index.html',
        ).readAsString(),
        contains('offline'),
      );

      await requests.cancel();
      await server.close(force: true);
      final installedEntries = await market.installedEntries();
      expect(installedEntries.single.name, 'example/offline-editor');
      final offlineEntries = await market.list();
      expect(offlineEntries.single.name, 'example/offline-editor');
      expect(
        await market.installed(),
        contains('example/offline-editor@1.0.0'),
      );
    },
  );
}
