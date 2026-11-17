import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

class _EmptyCatalog implements MobileMarketSource {
  @override
  Future<List<MobileMarketEntry>> list() async => [];
  @override
  Future<List<int>> artifact(MobileMarketEntry entry) async =>
      throw UnimplementedError();
}

class _LiveHttp extends HttpOverrides {}

void main() {
  LiveTestWidgetsFlutterBinding.ensureInitialized();
  for (final mine in [false, true]) {
    testWidgets(
      'host language plugins visible and manageable in ${mine ? 'installed inventory' : 'discover'}',
      (tester) async {
        final previous = HttpOverrides.current;
        HttpOverrides.global = _LiveHttp();
        addTearDown(() => HttpOverrides.global = previous);
        await HttpOverrides.runWithHttpOverrides(() async {
          final directory = await Directory.systemTemp.createTemp(
            'host-market-ui-',
          );
          final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
          var enabled = false;
          final operations = <String>[];
          const id = 'com.openmuse.code-navigation';
          server.listen((request) async {
            final body =
                jsonDecode(await utf8.decoder.bind(request).join()) as Map;
            operations.add(body['operation'] as String);
            if (body['operation'] == 'enable') enabled = true;
            request.response.headers.contentType = ContentType.json;
            request.response.write(
              jsonEncode(
                body['operation'] == 'list'
                    ? {
                        'plugins': [
                          {
                            'id': id,
                            'name': '代码导航服务',
                            'version': '0.1.1',
                            'package': id,
                            'ecosystem': 'code-navigation',
                          },
                          {
                            'id': '$id.dart',
                            'name': 'Dart 语言扩展',
                            'version': '0.1.0',
                            'package': '$id.dart',
                            'ecosystem': 'code-navigation',
                          },
                        ],
                      }
                    : {
                        'id': body['pluginId'],
                        'name': '代码导航服务',
                        'version': '0.1.1',
                        'installedVersion': body['pluginId'] == id
                            ? '0.1.1'
                            : null,
                        'enabled': enabled,
                        'configuration': {
                          'idleSeconds': 300,
                          'maxInstances': 2,
                        },
                        'revision': 0,
                        'detected': true,
                        'instances': 0,
                        'action': 'enable',
                      },
              ),
            );
            await request.response.close();
          });
          try {
            await tester.pumpWidget(
              MaterialApp(
                home: MobileMarketPage(
                  store: MobilePluginStore(directory),
                  source: _EmptyCatalog(),
                  embedded: true,
                  mode: mine
                      ? MobileMarketMode.installed
                      : MobileMarketMode.discover,
                  desktop: PairedDesktopMarketClient(
                    origin: Uri.parse('http://127.0.0.1:${server.port}'),
                    grantRef: 'fixture-grant',
                    expiresAtMs: DateTime.now()
                        .add(const Duration(minutes: 5))
                        .millisecondsSinceEpoch,
                  ),
                ),
              ),
            );
            await tester.pumpAndSettle();
            await tester.scrollUntilVisible(
              find.text('资源宿主插件'),
              250,
              maxScrolls: 12,
            scrollable: find.byType(Scrollable).first,
            );
            for (
              var i = 0;
              i < 30 && find.text('代码导航服务').evaluate().isEmpty;
              i++
            ) {
              await tester.runAsync(
                () => Future<void>.delayed(const Duration(milliseconds: 20)),
              );
              await tester.pump();
            }
            expect(find.text('代码导航服务'), findsOneWidget);
            if (mine) expect(find.text('Dart 语言扩展'), findsNothing);
            await tester.ensureVisible(find.text('安装与配置').first);
            await tester.tap(find.text('安装与配置').first);
            for (
              var i = 0;
              i < 30 && find.text('启用插件').evaluate().isEmpty;
              i++
            ) {
              await tester.runAsync(
                () => Future<void>.delayed(const Duration(milliseconds: 20)),
              );
              await tester.pump();
            }
            expect(find.text('安装与配置目标：已连接 Desktop'), findsOneWidget);
            await tester.ensureVisible(find.text('启用插件'));
            await tester.tap(find.text('启用插件'));
            for (var i = 0; i < 30 && !enabled; i++) {
              await tester.runAsync(
                () => Future<void>.delayed(const Duration(milliseconds: 20)),
              );
              await tester.pump();
            }
            expect(enabled, true);
            expect(operations, contains('enable'));
          } finally {
            await tester.pumpWidget(const SizedBox.shrink());
            await server.close(force: true);
            await directory.delete(recursive: true);
          }
        }, _LiveHttp());
      },
    );
  }
}
