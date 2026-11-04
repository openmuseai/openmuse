import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/src/mcp_app_document.dart';

void main() {
  test(
    'MCP host context updates preserve the frame and always signal readiness to native',
    () async {
      final html = buildMcpAppHostDocument(
        appHtml: '<p>Independent view</p>',
        arguments: {'resource': 'test'},
        platform: 'desktop',
        locale: 'en',
        presentation: {
          'openmuse': {
            'schemaVersion': 1,
            'settings': {
              'parameters': {'label': '</script>'},
            },
          },
        },
        nativeSend: 'nativeMessages.push(message)',
      );
      // Unit-test the generated bridge in a VM with a minimal transport/DOM, without
      // running a browser or trusting script supplied by the plugin.
      final script =
          '''
const vm=require('node:vm'),assert=require('node:assert/strict');
const html=${jsonEncode(html)},sent=[],nativeMessages=[],listeners=[];
const contentWindow={postMessage:message=>sent.push(message)};
let mounts=0;const frame={contentWindow,set srcdoc(value){mounts++}};
const window={addEventListener:(type,callback)=>listeners.push(callback),removeEventListener:()=>{}};
const context={window,document:{getElementById:()=>frame},TextDecoder,Uint8Array,atob,innerWidth:1000,innerHeight:700,nativeMessages};
vm.runInNewContext(html.match(/<script>([\\s\\S]*?)<\\/script>/)[1],context);
window.__mcpHostReceive({_hostContext:{locale:'fr'}});
assert.equal(sent.length,0);
const deliver=message=>listeners[0]({source:contentWindow,data:message});
(async()=>{
 await deliver({jsonrpc:'2.0',id:1,method:'ui/initialize',params:{}});
 assert.equal(sent[0].result.hostContext.locale,'fr');
 assert.equal(sent[0].result.hostContext.openmuse.settings.parameters.label,'</scr'+'ipt>');
 await deliver({jsonrpc:'2.0',method:'ui/notifications/initialized'});
 assert.equal(nativeMessages[0].method,'ui/notifications/initialized');
 await deliver({jsonrpc:'2.0',method:'ui/notifications/initialized'});
 assert.equal(nativeMessages.length,1);
 window.__mcpHostReceive({_hostContext:{locale:'de',theme:'dark'}});
 assert.equal(sent.at(-1).method,'ui/notifications/host-context-changed');
 assert.equal(sent.at(-1).params.locale,'de');assert.equal(mounts,1);
 await listeners[0]({source:{},data:{jsonrpc:'2.0',id:9,method:'ui/initialize'}});
 assert.notEqual(sent.at(-1).id,9);
})().catch(error=>{console.error(error);process.exitCode=1});
''';
      final result = await Process.run('node', ['-e', script]);
      expect(result.exitCode, 0, reason: '${result.stderr}');
    },
  );
}
