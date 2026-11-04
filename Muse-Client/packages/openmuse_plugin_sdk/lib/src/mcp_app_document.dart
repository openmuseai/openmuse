import 'dart:convert';

/// An MCP Apps host document. The View receives only standard MCP Apps JSON-RPC
/// over postMessage; the native callback belongs to the Host's outer frame.
/// Host-composed, isolated UI contributed by an independent MCP server.
final class McpAppExtensionDocument {
  const McpAppExtensionDocument({
    required this.id,
    required this.html,
    required this.arguments,
    this.width = 360,
    this.height = 152,
  });
  final String id, html;
  final Map<String, Object?> arguments;
  final int width, height;
}

String buildMcpAppHostDocument({
  String? appHtml,
  String? appUrl,
  String? argumentsUrl,
  required Map<String, Object?> arguments,
  required String platform,
  String theme = 'light',
  String? locale,
  Map<String, Object?> presentation = const {},
  bool agentAvailable = false,
  bool modelContextAvailable = false,
  bool resourceToolsAvailable = false,
  List<McpAppExtensionDocument> extensions = const [],
  bool serverAvailable = false,
  String nativeSend =
      'window.webkit.messageHandlers.mcpApp.postMessage(message)',
}) {
  assert(appHtml != null || appUrl != null);
  final contextAvailable = agentAvailable || modelContextAvailable;
  final extensionData = jsonEncode([
    for (final e in extensions)
      {
        'id': e.id,
        'html': base64Encode(utf8.encode(restrictMcpAppDocument(e.html))),
        'arguments': e.arguments,
        'width': e.width,
        'height': e.height,
      },
  ]).replaceAll('<', r'\u003c');
  final encoded = base64Encode(
    utf8.encode(appHtml == null ? '' : restrictMcpAppDocument(appHtml)),
  );
  final sourceUrl = jsonEncode(appUrl);
  final inputUrl = jsonEncode(argumentsUrl);
  final input = jsonEncode(arguments).replaceAll('<', r'\u003c');
  final hostPlatform = jsonEncode(platform);
  final hostContext = jsonEncode({
    'theme': theme,
    ...presentation,
    if (locale != null) 'locale': locale,
  }).replaceAll('<', r'\u003c');
  return '''<!doctype html><html><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>html,body,iframe{width:100%;height:100%;margin:0;border:0;overflow:hidden}iframe{display:block}</style>
</head><body><iframe id="view" sandbox="allow-scripts allow-downloads"></iframe>
<script>
const frame=document.getElementById('view');
const inputPromise=$inputUrl===null?Promise.resolve($input):fetch($inputUrl).then(response=>{if(!response.ok)throw Error('Host input unavailable');return response.json()});
let initialized=false;
let currentContext=$hostContext;
let appCapabilities={};
const send=message=>frame.contentWindow.postMessage(message,'*');
const extensionFrames=new Map();
function mountExtensions(specs){
 for(const entry of extensionFrames.values()){window.removeEventListener('message',entry.handler);entry.frame.remove();}
 extensionFrames.clear();
 for(const spec of specs)mountExtension(spec);
}
window.__mcpHostReceive=message=>{
 if(message._hostContext){
  currentContext={...currentContext,...message._hostContext};
  const notification={jsonrpc:'2.0',method:'ui/notifications/host-context-changed',params:currentContext};
  if(initialized)send(notification);
  for(const entry of extensionFrames.values())if(entry.initialized)entry.frame.contentWindow.postMessage(notification,'*');
  return;
 }

 if(message._hostExtensionDocuments){mountExtensions(message._hostExtensionDocuments);return;}
 if(message._hostResourceActive!==undefined){
   for(const entry of extensionFrames.values()){
     entry.args={...entry.args,active:message._hostResourceActive};
     entry.frame.contentWindow.postMessage({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:entry.args}},'*');
   }return;
 }
 if(message._hostExtension!==undefined){
   const entry=extensionFrames.get(message._hostExtension);if(!entry)return;
   const clean={...message};delete clean._hostExtension;
   entry.frame.contentWindow.postMessage(clean,'*');
 }else send(message);
};
function mountExtension(spec){
 const extension=document.createElement('iframe');
 extension.title='Resource extension';extension.sandbox='allow-scripts';
 extension.style.cssText='position:absolute;right:12px;bottom:36px;width:min('+spec.width+'px,calc(100% - 24px));height:'+spec.height+'px;z-index:10;background:transparent';
 const entry={frame:extension,args:spec.arguments,initialized:false};extensionFrames.set(spec.id,entry);
 const handler=event=>{
  if(event.source!==extension.contentWindow)return;
  const raw=event.data;if(!raw||raw.jsonrpc!=='2.0')return;
  const reply=value=>extension.contentWindow.postMessage(value,'*');
  if(raw.method==='ui/initialize'){
   reply({jsonrpc:'2.0',id:raw.id,result:{protocolVersion:'2026-01-26',hostInfo:{name:'OpenMuse',version:'1.0.0'},hostCapabilities:{serverTools:{}},hostContext:{...currentContext,platform:$hostPlatform,displayMode:'inline'}}});
  }else if(raw.method==='ui/notifications/initialized'){
   if(entry.initialized)return;entry.initialized=true;
   reply({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:entry.args}});
  }else if(raw.method==='ping')reply({jsonrpc:'2.0',id:raw.id,result:{}});
  else if(['tools/list','tools/call'].includes(raw.method)){
   const message={jsonrpc:'2.0',id:raw.id,method:raw.method,params:raw.params,_hostExtension:spec.id};$nativeSend;
  }else if(raw.id!==undefined&&raw.method)reply({jsonrpc:'2.0',id:raw.id,error:{code:-32601,message:'Extension method unavailable'}});
 };
 entry.handler=handler;window.addEventListener('message',handler);
 document.body.append(extension);
 extension.srcdoc=new TextDecoder().decode(Uint8Array.from(atob(spec.html),c=>c.charCodeAt(0)));
}
mountExtensions($extensionData);
window.addEventListener('message',async event=>{
 if(event.source!==frame.contentWindow)return;
 const message=event.data;
 if(!message||message.jsonrpc!=='2.0')return;
 if(message.method==='ui/initialize'){
   appCapabilities=message.params?.appCapabilities??{};
   send({jsonrpc:'2.0',id:message.id,result:{protocolVersion:'2026-01-26',hostInfo:{name:'OpenMuse',version:'1.0.0'},hostCapabilities:{downloadFile:{},...($serverAvailable?{serverTools:{},serverResources:{}}:{}),...($agentAvailable?{message:{text:{}}}:{}),...($contextAvailable?{updateModelContext:{text:{}}}:{})},hostContext:{...currentContext,platform:$hostPlatform,displayMode:'inline',containerDimensions:{width:innerWidth,height:innerHeight}}}});
 }else if(message.method==='ui/notifications/initialized'){
   if(initialized)return;initialized=true;
   send({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:await inputPromise}});
   send({jsonrpc:'2.0',method:'ui/notifications/tool-result',params:{content:[]}});
   $nativeSend;
 }else if(message.method==='ping'){
   send({jsonrpc:'2.0',id:message.id,result:{}});
 }else if(message.method==='ui/download-file'){
   $nativeSend;
 }else if($serverAvailable&&['tools/list','tools/call','resources/list','resources/read'].includes(message.method)){
   $nativeSend;
 }else if(($contextAvailable&&message.method==='ui/update-model-context')||($agentAvailable&&(message.method==='ui/message'||message.id!==undefined&&!message.method))||($resourceToolsAvailable&&message.id!==undefined&&!message.method)){
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
