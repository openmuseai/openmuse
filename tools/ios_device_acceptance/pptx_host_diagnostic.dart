// Diagnostic fixture only. Never use this entry point for a store candidate.
// The only input document is an explicitly supplied synthetic QA deck.
import 'dart:convert';
import 'package:archive/archive.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:webview_flutter/webview_flutter.dart';

void main() => runApp(const MaterialApp(home: PptxDiagnostic()));

class PptxDiagnostic extends StatefulWidget {
  const PptxDiagnostic({super.key});
  @override
  State<PptxDiagnostic> createState() => _PptxDiagnosticState();
}

class _PptxDiagnosticState extends State<PptxDiagnostic> {
  late final WebViewController web;
  bool ready = false;
  @override
  void initState() {
    super.initState();
    web = WebViewController()..setJavaScriptMode(JavaScriptMode.unrestricted);
    start();
  }

  Future<void> start() async {
    const sample = String.fromEnvironment('QA_PPTX_BASE64');
    if (sample.isEmpty) throw StateError('Explicit synthetic QA deck required');
    final bundle = await rootBundle.load('assets/bundled/slides-0.9.7.mcpb');
    final archive = ZipDecoder().decodeBytes(bundle.buffer.asUint8List());
    var html = utf8.decode(
      archive.files.firstWhere((f) => f.name == 'web/index.html').content,
    );
    // Trace only event kinds and button labels inside this synthetic QA View.
    html = html.replaceFirst('</head>', r'''<script>
for(const kind of ['pointerdown','mousedown','click','touchstart','touchend','blur']){
 document.addEventListener(kind,event=>{
  const button=event.target instanceof Element?event.target.closest('button'):null;
  if(button)parent.postMessage({jsonrpc:'2.0',method:'qa/event',params:{kind,button:button.getAttribute('aria-label')||button.textContent,pointerType:event.pointerType||'',detail:event.detail||0}},'*');
 },true);
}
</script></head>''');
    await web.addJavaScriptChannel(
      'McpHost',
      onMessageReceived: (event) async {
        final request = jsonDecode(event.message) as Map<String, dynamic>;
        final method = request['method'];
        debugPrint(
          'QA_MCP ${jsonEncode({'method': method, if (method == 'qa/event') 'event': request['params']})}',
        );
        if (method == 'qa/event' || request['id'] == null) return;
        final response = <String, Object?>{
          'jsonrpc': '2.0',
          'id': request['id'],
        };
        if (method == 'ui/download-file') {
          final contents = (request['params'] as Map)['contents'] as List;
          final resource = (contents.single as Map)['resource'] as Map;
          final bytes = base64Decode(resource['blob'] as String);
          // The diagnostic sink acknowledges the standard download, not a real
          // resource write. Production persistence remains a separate test.
          debugPrint('QA_DOWNLOAD_BYTES ${bytes.length}');
          response['result'] = <String, Object?>{};
        } else {
          response['error'] = {
            'code': -32601,
            'message': 'Diagnostic method unavailable',
          };
        }
        await web.runJavaScript(
          'window.__mcpHostReceive(${jsonEncode(response)})',
        );
      },
    );
    var document = buildMcpAppHostDocument(
      appHtml: html,
      arguments: {'name': 'OpenMuse Introduction.pptx', 'content': sample},
      platform: 'mobile',
      locale: 'zh-Hans',
      nativeSend: 'window.McpHost.postMessage(JSON.stringify(message))',
    );
    document = document.replaceFirst('</body>', r'''<script>
window.addEventListener('message',event=>{
 if(event.source===document.getElementById('view').contentWindow&&event.data?.method==='qa/event')window.McpHost.postMessage(JSON.stringify(event.data));
});
</script></body>''');
    await web.loadHtmlString(document);
    if (mounted) setState(() => ready = true);
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: const Text('PPTX 诊断 · 非发布包')),
    body: SafeArea(
      top: false,
      child: ready
          ? WebViewWidget(controller: web)
          : const Center(child: CircularProgressIndicator()),
    ),
  );
}
