import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
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
