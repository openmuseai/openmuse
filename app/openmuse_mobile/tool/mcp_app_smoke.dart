import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final support = await getApplicationSupportDirectory();
  final market = RemoteMcpMarket(
    Uri.parse(
      const String.fromEnvironment(
        'TEST_REGISTRY',
        defaultValue: 'http://127.0.0.1:18803',
      ),
    ),
    File('${support.path}/mcp-smoke-installations.json'),
  );
  const offline = bool.fromEnvironment('TEST_OFFLINE');
  if (offline) HttpOverrides.global = _OfflineOnly();
  final entries = offline
      ? await market.installedEntries()
      : await market.list();
  final entry = entries.singleWhere(
    (e) =>
        e.name ==
        const String.fromEnvironment(
          'TEST_PLUGIN',
          defaultValue: 'io.openmuse/genoffice-pptx',
        ),
  );
  if (!offline) await market.install(entry);
  final html = await market.mcpAppDocument(entry);
  if (html == null) throw StateError('No standard MCP Apps UI resource');
  final file = File('${support.path}/Sample_ppt-smoke.pptx');
  late final Uint8List bytes;
  if (offline) {
    bytes = await file.readAsBytes();
  } else {
    final http = HttpClient();
    final response = await (await http.getUrl(
      Uri.parse(
        const String.fromEnvironment(
          'TEST_FILE',
          defaultValue: 'http://127.0.0.1:18799/dist/sample.pptx',
        ),
      ),
    )).close();
    final data = BytesBuilder();
    await for (final chunk in response) {
      data.add(chunk);
    }
    http.close();
    bytes = data.takeBytes();
    await file.writeAsBytes(bytes);
  }
  runApp(
    MaterialApp(
      home: RemoteMcpOfflineAppPage(
        title: 'MCP Apps · ${entry.title} ${entry.version}',
        digest: entry.packageSha256!,
        appHtml: html,
        fileBytes: bytes,
        fileName: 'Sample_ppt.pptx',
        mediaType: (await market.resourceMediaTypes(entry)).single,
        filePath: file.path,
      ),
    ),
  );
}

final class _OfflineOnly extends HttpOverrides {
  @override
  HttpClient createHttpClient(SecurityContext? context) =>
      throw StateError('Network access is disabled during offline acceptance');
}
