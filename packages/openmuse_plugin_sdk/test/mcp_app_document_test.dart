import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/src/mcp_app_document.dart';

void main() {
  test('browser printing is opt-in and only relays the mounted View', () async {
    for (final enabled in [false, true]) {
      final html = buildMcpAppHostDocument(
        appHtml: '<p>Independent view</p>',
        arguments: {},
        platform: 'mobile',
        browserPrintAvailable: enabled,
        nativeSend: 'nativeMessages.push(message)',
      );
      final script =
          '''
const vm=require('node:vm'),assert=require('node:assert/strict'),sent=[],nativeMessages=[],listeners=[];
const contentWindow={postMessage:m=>sent.push(m)},frame={contentWindow};
const window={addEventListener:(t,fn)=>listeners.push(fn),removeEventListener:()=>{}};
vm.runInNewContext(${jsonEncode(html)}.match(/<script>([\\s\\S]*?)<\\/script>/)[1],{window,document:{getElementById:()=>frame},TextDecoder,Uint8Array,atob,innerWidth:691,innerHeight:660,nativeMessages});
(async()=>{
 const deliver=m=>listeners[0]({source:contentWindow,data:m});
 await deliver({jsonrpc:'2.0',id:1,method:'ui/initialize',params:{}});
 assert.equal(!!sent[0].result.hostCapabilities.experimental?.['io.openmuse/browser-print'],$enabled);
 const request={jsonrpc:'2.0',id:2,method:'io.openmuse/browser-print',params:{html:'<p>Own page</p>',widthMils:8270,heightMils:11690}};
 await listeners[0]({source:{},data:request});assert.equal(nativeMessages.length,0);
 await deliver(request);assert.equal(nativeMessages.length,$enabled?1:0);
 if(!$enabled)assert.equal(sent.at(-1).error.code,-32601);
})().catch(e=>{console.error(e);process.exitCode=1});
''';
      final result = await Process.run('node', ['-e', script]);
      expect(result.exitCode, 0, reason: '${result.stderr}');
    }
  });
  test(
    'document change context remains available without enabling Agent messages',
    () async {
      final html = buildMcpAppHostDocument(
        appHtml: '<p>Document</p>',
        arguments: {},
        platform: 'desktop',
        modelContextAvailable: true,
        nativeSend: 'nativeMessages.push(message)',
      );
      final script =
          '''
const vm=require('node:vm'),assert=require('node:assert/strict');
const html=${jsonEncode(html)},sent=[],nativeMessages=[],listeners=[];
const contentWindow={postMessage:message=>sent.push(message)};
const frame={contentWindow};
const window={addEventListener:(type,callback)=>listeners.push(callback),removeEventListener:()=>{}};
const context={window,document:{getElementById:()=>frame},TextDecoder,Uint8Array,atob,innerWidth:1000,innerHeight:700,nativeMessages};
vm.runInNewContext(html.match(/<script>([\\s\\S]*?)<\\/script>/)[1],context);
const deliver=message=>listeners[0]({source:contentWindow,data:message});
(async()=>{
 await deliver({jsonrpc:'2.0',id:1,method:'ui/initialize',params:{}});
 const capabilities=sent[0].result.hostCapabilities;
 assert.ok(capabilities.updateModelContext);
 assert.equal(capabilities.message,undefined);
 await deliver({jsonrpc:'2.0',id:2,method:'ui/update-model-context',params:{structuredContent:{dirty:true}}});
 assert.equal(nativeMessages.length,1);
 assert.equal(nativeMessages[0].params.structuredContent.dirty,true);
 await deliver({jsonrpc:'2.0',id:3,method:'ui/message',params:{content:[]}});
 assert.equal(nativeMessages.length,1);
 assert.equal(sent.at(-1).error.code,-32601);
})().catch(error=>{console.error(error);process.exitCode=1});
''';
      final result = await Process.run('node', ['-e', script]);
      expect(result.exitCode, 0, reason: '${result.stderr}');
    },
  );
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
