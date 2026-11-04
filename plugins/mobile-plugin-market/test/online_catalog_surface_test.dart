import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

class _LiveHttp extends HttpOverrides {}

void main() {
  LiveTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'published plugins appear inline without a protocol category; My retains delisted installs',
    (tester) async {
      final previousHttp = HttpOverrides.current;
      HttpOverrides.global = _LiveHttp();
      addTearDown(() => HttpOverrides.global = previousHttp);
      await HttpOverrides.runWithHttpOverrides(() async {
        final root = await Directory.systemTemp.createTemp('inline-market-');
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        Map<String, Object?> record(String version) => {
          'server': {
            'name': 'example/editor',
            'title': 'Example Editor',
            'version': version,
            'description': 'Edit documents',
            'remotes': [
              {'type': 'streamable-http', 'url': 'https://example.com/mcp'},
            ],
          },
        };
        final current = record('2.0.0'), old = record('1.0.0');
        server.listen((r) async {
          r.response.headers.contentType = ContentType.json;
          r.response.write(
            jsonEncode({
              'servers': [current],
              'metadata': {},
            }),
          );
          await r.response.close();
        });
        final receipts = File('${root.path}/receipts.json');
        await receipts.writeAsString(jsonEncode(['example/editor@1.0.0']));
        // The installed snapshot is provider-independent, as in the production store.
        final market = RemoteMcpMarket(
          Uri.parse('http://127.0.0.1:${server.port}'),
          receipts,
        );
        final saved = File('${root.path}/mcp-installed-records.json');
        await saved.writeAsString(jsonEncode([old]));
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: RemoteMcpMarketPage(
                origin: market.origin,
                receiptFile: receipts,
                embedded: true,
              ),
            ),
          ),
        );
        for (
          var i = 0;
          i < 30 && find.text('Example Editor').evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 30)),
          );
          await tester.pump();
        }
        expect(find.text('Example Editor'), findsOneWidget);
        expect(find.text('2.0.0'), findsOneWidget);
        expect(find.text('在线 MCP 插件'), findsNothing);
        expect(find.byType(AppBar), findsNothing);
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: RemoteMcpMarketPage(
                key: const ValueKey('mine'),
                origin: market.origin,
                receiptFile: receipts,
                embedded: true,
                installedOnly: true,
                visibleCategory: '全部',
              ),
            ),
          ),
        );
        for (
          var i = 0;
          i < 30 && find.textContaining('Delisted').evaluate().isEmpty;
          i++
        ) {
          await Future<void>.delayed(const Duration(milliseconds: 30));
          await tester.pump();
        }
        expect(find.textContaining('1.0.0 · Delisted'), findsOneWidget);
        expect(find.text('2.0.0'), findsNothing);
        await tester.pumpWidget(const SizedBox());
        await server.close(force: true);
        await root.delete(recursive: true);
      }, _LiveHttp());
    },
  );
}
