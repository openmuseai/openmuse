import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized().framePolicy =
      LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  testWidgets('reviewed MCP Apps bundle opens a PPTX offline in the Android Host', (
    tester,
  ) async {
    final support = await getApplicationSupportDirectory();
    final market = RemoteMcpMarket(
      Uri.parse('http://127.0.0.1:18803'),
      File('${support.path}/pptx-e2e-installations.json'),
    );
    final entries = await market.list();
    final entry = entries.singleWhere(
      (e) => e.name == 'io.openmuse/genoffice-pptx',
    );
    await market.install(entry);
    expect(
      await market.resourceMediaTypes(entry),
      contains(
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      ),
    );
    final html = await market.mcpAppDocument(entry);
    expect(html, isNotNull);
    final http = HttpClient();
    final response = await (await http.getUrl(
      Uri.parse('http://127.0.0.1:18799/dist/sample.pptx'),
    )).close();
    final data = BytesBuilder();
    await for (final chunk in response) {
      data.add(chunk);
    }
    http.close();
    final bytes = data.takeBytes();
    final file = File('${support.path}/Sample_ppt-e2e.pptx');
    await file.writeAsBytes(bytes);
    await tester.pumpWidget(
      MaterialApp(
        home: RemoteMcpOfflineAppPage(
          title: 'PPTX MCP Apps E2E · ${entry.version}',
          digest: entry.packageSha256!,
          appHtml: html,
          fileBytes: bytes,
          fileName: 'Sample_ppt.pptx',
          mediaType:
              'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          filePath: file.path,
        ),
      ),
    );
    await tester.pumpAndSettle();
    await Future<void>.delayed(const Duration(seconds: 15));

    // The external browser fidelity harness verifies the actual native WebView
    // while this integration test keeps it mounted.
    debugPrint('PPTX_ANDROID_VIEW_MOUNTED');
    await Future<void>.delayed(const Duration(seconds: 50));
    await tester.pumpWidget(const SizedBox());
    await tester.pumpAndSettle();
  });
}
