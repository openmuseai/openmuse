import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:webview_flutter/webview_flutter.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('production Markdown MCP tool opens editor inside Android', (
    tester,
  ) async {
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-editor-e2e-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final entry = RemoteMcpEntry(
      'io.openmuse/appflowy-editor',
      'AppFlowy Editor',
      '0.1.4',
      'Markdown editor',
      Uri.parse('https://plugins.openmuseai.com/mcp'),
      null,
      null,
      null,
    );
    final market = RemoteMcpMarket(
      Uri.parse('https://plugins.openmuseai.com'),
      File('${directory.path}/receipts.json'),
    );
    expect(await market.acceptsContent(entry, 'text/markdown'), isTrue);
    final uri = await market.openContent(
      entry,
      mediaType: 'text/markdown',
      content: '# Android production acceptance\n',
      name: 'mobile-e2e.md',
    );
    expect(uri.host, 'plugins.openmuseai.com');
    await tester.pumpWidget(
      MaterialApp(
        home: RemoteMcpAppPage(title: 'mobile-e2e.md', uri: uri),
      ),
    );
    await tester.pumpAndSettle();
    final controller = tester
        .widget<WebViewWidget>(find.byType(WebViewWidget))
        .platform
        .params
        .controller;
    String? saved;
    for (var attempt = 0; attempt < 30; attempt++) {
      final result = await controller.runJavaScriptReturningResult(
        "window.localStorage.getItem('appflowy-editor-plugin.document.v1')",
      );
      if (result is String && result.contains('mobile-e2e.md')) {
        saved = result;
        break;
      }
      await Future<void>.delayed(const Duration(seconds: 1));
    }
    expect(
      saved,
      isNotNull,
      reason:
          'Editor should save the imported document in its in-app WebView storage',
    );
    final raw = jsonDecode(saved!);
    final decoded = (raw is String ? jsonDecode(raw) : raw) as Map;
    expect(decoded['title'], 'mobile-e2e.md');
    expect(
      jsonEncode(decoded['document']),
      contains('Android production acceptance'),
    );
  });
}
