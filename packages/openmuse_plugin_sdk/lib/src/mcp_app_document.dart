import 'dart:convert';

/// An MCP Apps host document. The View receives only standard MCP Apps JSON-RPC
/// over postMessage; the native callback belongs to the Host's outer frame.
String buildMcpAppHostDocument({
  String? appHtml,
  String? appUrl,
  String? argumentsUrl,
  required Map<String, Object?> arguments,
  required String platform,
  String nativeSend =
      'window.webkit.messageHandlers.mcpApp.postMessage(message)',
}) {
  assert(appHtml != null || appUrl != null);
  final encoded = base64Encode(
    utf8.encode(appHtml == null ? '' : restrictMcpAppDocument(appHtml)),
  );
  final sourceUrl = jsonEncode(appUrl);
  final inputUrl = jsonEncode(argumentsUrl);
  final input = jsonEncode(arguments).replaceAll('<', r'\u003c');
  final hostPlatform = jsonEncode(platform);
  return '''<!doctype html><html><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>html,body,iframe{width:100%;height:100%;margin:0;border:0;overflow:hidden}iframe{display:block}</style>
</head><body><iframe id="view" sandbox="allow-scripts allow-downloads"></iframe>
<script>
const frame=document.getElementById('view');
const inputPromise=$inputUrl===null?Promise.resolve($input):fetch($inputUrl).then(response=>{if(!response.ok)throw Error('Host input unavailable');return response.json()});
let initialized=false;
const send=message=>frame.contentWindow.postMessage(message,'*');
window.__mcpHostReceive=message=>send(message);
window.addEventListener('message',async event=>{
 if(event.source!==frame.contentWindow)return;
 const message=event.data;
 if(!message||message.jsonrpc!=='2.0')return;
 if(message.method==='ui/initialize'){
   send({jsonrpc:'2.0',id:message.id,result:{protocolVersion:'2026-01-26',hostInfo:{name:'OpenMuse',version:'1.0.0'},hostCapabilities:{downloadFile:{}},hostContext:{theme:'light',locale:'zh-CN',platform:$hostPlatform,displayMode:'inline',containerDimensions:{width:innerWidth,height:innerHeight}}}});
 }else if(message.method==='ui/notifications/initialized'){
   if(initialized)return;initialized=true;
   send({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:await inputPromise}});
   send({jsonrpc:'2.0',method:'ui/notifications/tool-result',params:{content:[]}});
 }else if(message.method==='ping'){
   send({jsonrpc:'2.0',id:message.id,result:{}});
 }else if(message.method==='ui/download-file'){
   $nativeSend;
 }else if(message.id!==undefined&&message.method){
   send({jsonrpc:'2.0',id:message.id,error:{code:-32601,message:'Host method unavailable'}});
 }
});
if($sourceUrl!==null)frame.src=$sourceUrl;else frame.srcdoc=new TextDecoder().decode(Uint8Array.from(atob('$encoded'),c=>c.charCodeAt(0)));
</script></body></html>''';
}

/// The Host enforces the offline View's network boundary before plugin HTML.
/// A second policy in plugin HTML cannot loosen this policy.
String restrictMcpAppDocument(String html) =>
    '<meta http-equiv="Content-Security-Policy" content="default-src \'none\'; '
    'script-src \'unsafe-inline\' \'wasm-unsafe-eval\'; style-src \'unsafe-inline\'; '
    'img-src data: blob:; font-src data: blob:; media-src data: blob:; '
    'connect-src \'none\'; base-uri \'none\'; form-action \'none\'">$html';
