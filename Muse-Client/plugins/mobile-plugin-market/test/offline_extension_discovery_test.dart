import 'dart:convert';
import 'dart:io';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
  test(
    'verified offline MCP discovery separates resource extensions from editors without a server',
    () async {
      final root = await Directory.systemTemp.createTemp('offline-extension-');
      final http = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() async {
        await http.close(force: true);
        await root.delete(recursive: true);
      });
      const html = '<!doctype html><title>Independent extension</title>';
      final catalog = {
        'profile': 'openmuse-resource-extensions/1',
        'extensions': [
          {
            'id': 'test',
            'tool': 'toolbar',
            'mediaTypes': ['application/example'],
            'width': 360,
            'height': 152,
            'resourceTools': ['read_example'],
          },
        ],
      };
      final discovery = {
        'tools': [
          {
            'name': 'toolbar',
            'inputSchema': {'type': 'object'},
            '_meta': {
              'ui': {'resourceUri': 'ui://test/toolbar'},
            },
          },
        ],
        'resources': [
          {
            'uri': 'capability://test/extensions',
            'mimeType': 'application/vnd.openmuse.resource-extensions+json',
            'text': jsonEncode(catalog),
          },
          {
            'uri': 'ui://test/toolbar',
            'mimeType': 'text/html;profile=mcp-app',
            'text': html,
          },
        ],
      };
      final archive = Archive()
        ..add(ArchiveFile.string('web/index.html', html))
        ..add(
          ArchiveFile.string('web/mcp-offline.json', jsonEncode(discovery)),
        );
      final bundle = ZipEncoder().encode(archive),
          digest = sha256.convert(ZipEncoder().encode(archive)).toString();
      final origin = Uri.parse('http://127.0.0.1:${http.port}');
      final entry = RemoteMcpEntry.fromRecord({
        'server': {
          'name': 'example/toolbar',
          'version': '1.0.0',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': '$origin/package.mcpb',
              'fileSha256': digest,
            },
          ],
        },
      });
      http.listen((request) async {
        request.response.add(bundle);
        await request.response.close();
      });
      final market = RemoteMcpMarket(
        origin,
        File('${root.path}/receipts.json'),
      );
      await market.install(entry);
      final extensions = await market.resourceExtensions('application/example');
      expect(extensions.single['html'], html);
      expect(extensions.single['resourceTools'], ['read_example']);
      expect(await market.resourceMediaTypes(entry), isEmpty);
      expect(await market.resourceExtensions('application/other'), isEmpty);
      expect(await market.mcpAppDocument(entry), isNull);
    },
  );
  test(
    'shipping provider discovers live UI offline from its verified artifact',
    () async {
      final root = await Directory.systemTemp.createTemp('offline-provider-');
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() async {
        await server.close(force: true);
        await root.delete(recursive: true);
      });
      final bytes = await File(
        Platform.environment['OPENMUSE_OFFLINE_PROVIDER']!,
      ).readAsBytes();
      final digest = sha256.convert(bytes).toString(),
          origin = Uri.parse('http://127.0.0.1:${server.port}');
      server.listen((r) async {
        r.response.add(bytes);
        await r.response.close();
      });
      final entry = RemoteMcpEntry.fromRecord({
        'server': {
          'name': 'io.openmuse/genoffice-pptx',
          'version': '0.7.1',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': '$origin/p.mcpb',
              'fileSha256': digest,
            },
          ],
        },
      });
      final market = RemoteMcpMarket(origin, File('${root.path}/receipt.json'));
      await market.install(entry);
      expect(
        await market.resourceMediaTypes(entry),
        contains(
          'application/vnd.openxmlformats-officedocument.presentationml.presentation',
        ),
      );
      expect(
        await market.mcpAppDocument(entry),
        contains('Content-Security-Policy'),
      );
    },
    skip: Platform.environment['OPENMUSE_OFFLINE_PROVIDER'] == null,
  );
}
