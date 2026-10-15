import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:webview_flutter/webview_flutter.dart';

import 'package:openmuse_mobile/mobile_plugin_detail_page.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('plugin detail WebView renders and uses its scoped Host bridge', (
    tester,
  ) async {
    WebViewController? controller;
    final plugin = _ProbePlugin();
    await tester.pumpWidget(
      MaterialApp(
        home: MobilePluginDetailPage(
          plugin: plugin,
          contribution: plugin,
          onControllerReady: (value) => controller = value,
        ),
      ),
    );
    await tester.pumpAndSettle(const Duration(milliseconds: 300));
    expect(controller, isNotNull);
    await Future<void>.delayed(const Duration(seconds: 2));
    final body = await controller!.runJavaScriptReturningResult(
      'document.body.innerText',
    );
    expect(body.toString(), contains('Host 已响应：probe-ok'));
    expect(plugin.requests, ['probe']);

    await controller!.runJavaScript(
      "location.href='https://example.com/escaped'",
    );
    await Future<void>.delayed(const Duration(seconds: 1));
    final url = await controller!.currentUrl();
    expect(url, isNot(contains('example.com')));
  });
}

final class _ProbePlugin extends ChangeNotifier
    implements OpenMusePlugin, OpenMuseMobileWorkbenchContributor {
  final requests = <String>[];

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.probe',
    name: 'Probe',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: ['onStartup'],
  );

  @override
  OpenMuseWorkbenchCard get workbenchCard =>
      const OpenMuseWorkbenchCard(title: 'Probe', summary: 'Bridge probe');

  @override
  Listenable get workbenchChanges => this;

  @override
  List<Map<String, Object?>> buildWorkbenchPreview() => const [];

  @override
  String buildMobileDetailDocument(Brightness brightness) => '''
<!doctype html><html><head></head><body><main id="output">加载中</main>
<script>
window.openMuseHostResponse = raw => {
  const response = JSON.parse(raw);
  document.getElementById('output').textContent =
    'Host 已响应：' + response.result;
};
OpenMusePlugin.postMessage(JSON.stringify({id:'1',method:'probe',arguments:{}}));
</script></body></html>''';

  @override
  Future<Object?> handleMobileDetailRequest(
    String method,
    Object? arguments,
  ) async {
    requests.add(method);
    return 'probe-ok';
  }

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}
