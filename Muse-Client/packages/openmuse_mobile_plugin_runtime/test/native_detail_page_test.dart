import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets(
    'compiled detail mounts the plugin widget without evaluating HTML',
    (tester) async {
      final plugin = _CompiledPlugin();
      await tester.pumpWidget(
        MaterialApp(
          home: MobilePluginDetailPage(plugin: plugin, contribution: plugin),
        ),
      );
      await tester.pump();
      expect(find.text('Native document surface'), findsOneWidget);
      expect(tester.takeException(), isNull);
    },
  );
}

final class _CompiledPlugin
    implements
        OpenMusePlugin,
        OpenMuseMobileWorkbenchContributor,
        OpenMuseMobileNativeDetailContributor {
  final _changes = ChangeNotifier();
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'test.compiled',
    name: 'Compiled client',
    version: '1.0.0',
    runtime: OpenMusePluginRuntime.builtIn,
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildMobileDetail(BuildContext context) =>
      const Center(child: Text('Native document surface'));
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnimplementedError();
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
  @override
  OpenMuseWorkbenchCard get workbenchCard =>
      const OpenMuseWorkbenchCard(title: 'Compiled', summary: 'Local client');
  @override
  Listenable get workbenchChanges => _changes;
  @override
  List<Map<String, Object?>> buildWorkbenchPreview() => [];
  @override
  String buildMobileDetailDocument(Brightness brightness) =>
      throw StateError('Native contributions must not evaluate HTML');
  @override
  Future<Object?> handleMobileDetailRequest(String method, Object? arguments) =>
      throw UnimplementedError();
}
