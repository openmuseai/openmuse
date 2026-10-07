import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cli_plugin/openmuse_cli_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets('CLI plugin exposes a bottom console without starting a shell', (
    tester,
  ) async {
    final plugin = OpenMuseCliPlugin();
    expect(plugin.descriptor.id, 'com.openmuse.cli');
    expect(
      plugin.descriptor.panels.single.region,
      OpenMuseSurfaceRegion.bottomPanel,
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 260,
            child: const OpenMuseCliConsole(workingDirectory: '/'),
          ),
        ),
      ),
    );
    expect(find.text('控制台'), findsOneWidget);
    expect(find.byKey(const ValueKey('cli-new-terminal')), findsOneWidget);
  });
}
