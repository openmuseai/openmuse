import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_market/openmuse_plugin_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets('built-in market installs Archify through the Host boundary', (
    tester,
  ) async {
    final commands = <String>[];
    final market = OpenMusePluginMarket();
    expect(
      market.descriptor.panels.single.region,
      OpenMuseSurfaceRegion.dialog,
    );
    await market.activate(
      OpenMusePluginContext(
        executeHostCommand: (command, arguments) async {
          commands.add(command);
          if (command == 'plugin.market.providers') return null;
          switch (command) {
            case 'plugin.market.list':
              return [
                {
                  'id': 'archify',
                  'name': 'Archify',
                  'ecosystem': 'dsh',
                  'package': '@tt-a1i/archify-dsh',
                  'version': '1.0.0',
                },
              ];
            case 'plugin.market.plan':
              return {
                'id': 'archify',
                'version': '1.0.0',
                'profile': 'web',
                'installedVersion': null,
                'action': 'install',
              };
            case 'plugin.market.install':
              return {
                'id': 'archify',
                'version': '1.0.0',
                'profile': 'web',
                'installedVersion': '1.0.0',
                'action': 'installed',
              };
          }
          throw UnsupportedError(command);
        },
      ),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Builder(
            builder: (context) => market.buildPanel(context, 'plugin.market')!,
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.textContaining('Archify 1.0.0'), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('plugin-market-search')),
      'missing',
    );
    await tester.pump();
    expect(find.text('没有匹配的插件'), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('plugin-market-search')),
      'archify',
    );
    await tester.pump();
    expect(find.textContaining('Archify 1.0.0'), findsOneWidget);
    await tester.tap(find.text('安装'));
    await tester.pumpAndSettle();
    expect(commands, contains('plugin.market.install'));
    expect(find.textContaining('已安装 1.0.0'), findsOneWidget);
  });

  testWidgets('reviewed MCPB installs from the online market', (tester) async {
    final commands = <String>[];
    final market = OpenMusePluginMarket();
    await market.activate(
      OpenMusePluginContext(
        executeHostCommand: (command, arguments) async {
          commands.add(command);
          if (command == 'plugin.market.providers') return null;
          if (command == 'plugin.market.list') {
            return [
              {
                'id': 'io.openmuse/genoffice-pptx',
                'name': 'GenOffice PPTX',
                'ecosystem': 'mcp',
                'package': 'io.openmuse/genoffice-pptx',
                'version': '0.2.0',
                'categories': ['办公'],
              },
            ];
          }
          if (command == 'plugin.market.plan') {
            return {
              'version': '0.2.0',
              'installedVersion': null,
              'action': 'install',
            };
          }
          if (command == 'plugin.market.install') {
            return {
              'version': '0.2.0',
              'installedVersion': '0.2.0',
              'action': 'installed',
              'enabled': true,
            };
          }
          throw UnsupportedError(command);
        },
      ),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Builder(
            builder: (context) => market.buildPanel(context, 'plugin.market')!,
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('安装'));
    await tester.pumpAndSettle();
    expect(commands, contains('plugin.market.install'));
    expect(find.text('已安装'), findsWidgets);
    expect(commands, isNot(contains('plugin.market.enable')));
  });
}
