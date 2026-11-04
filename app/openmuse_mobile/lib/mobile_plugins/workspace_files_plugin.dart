import '../mobile_build_locale.dart';

import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'mobile_preview_messages.dart';
import 'mobile_workspace_mirror_client.dart';
import 'paired_mobile_context.dart';
import '../workbuddy/workbuddy_models.dart';

/// First-party plugin. Desktop owns file authority; this class only exposes
/// scoped, read-only operations to its own WebView detail page.
final class MobileWorkspaceFilesPlugin extends ChangeNotifier
    implements
        OpenMusePlugin,
        OpenMuseMobileWorkbenchContributor,
        PairedMobileContextContributor {
  MobileWorkspaceFilesPlugin({OpenMuseStrings? strings})
    : _strings = strings ?? OpenMuseStrings.fallback;
  final OpenMuseStrings _strings;

  PairedDesktopConnection? _connection;
  Future<void> Function(
    OpenMuseResource resource,
    Future<Map<String, Object?>> Function() read,
    ValueChanged<String> reportStep,
  )?
  onOpenResource;

  final Stopwatch _openingClock = Stopwatch();
  late String _step = (_strings.mobilePreparingToOpenTheFile6349c791());
  int _received = 0;
  int? _total;
  bool _opening = false;
  final List<Map<String, Object?>> _timings = [];

  Map<String, Object?> get openingProgress => {
    'step': _step,
    'received': _received,
    'total': _total,
    'elapsedMs': _openingClock.elapsedMilliseconds,
    'opening': _opening,
    'timings': List<Map<String, Object?>>.of(_timings),
  };

  void _report(String step, [int received = 0, int? total]) {
    if (_step != step) {
      _timings.add({'step': step, 'atMs': _openingClock.elapsedMilliseconds});
      // No file name, opaque reference, grant, or document content in logs.
      debugPrint(
        'WorkspaceOpen step=$step elapsedMs=${_openingClock.elapsedMilliseconds}',
      );
    }
    _step = step;
    _received = received;
    _total = total;
  }

  void updateConnection(PairedDesktopConnection? value) {
    if (_connection?.grantRef == value?.grantRef) return;
    _connection = value;
    notifyListeners();
  }

  @override
  void updatePairedContext({
    required PairedDesktopConnection? connection,
    required List<WbWorkspace> workspaces,
    required String? selectedWorkspaceId,
  }) => updateConnection(connection);

  @override
  OpenMusePluginDescriptor get descriptor => OpenMusePluginDescriptor(
    id: 'com.openmuse.mobile.workspace-files',
    name: (_strings.mobileWorkspaceFilesC16b24dd()),
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: ['onStartup'],
    permissions: {'workspace.paired.read'},
  );

  @override
  OpenMuseWorkbenchCard get workbenchCard => OpenMuseWorkbenchCard(
    title: (_strings.mobileWorkspaceFilesC16b24dd()),
    summary: (_strings.mobileBrowseDesktopFoldersAndFiles74092ef2()),
    height: OpenMuseWidgetHeight.medium,
  );

  @override
  Listenable get workbenchChanges => this;

  @override
  List<Map<String, Object?>> buildWorkbenchPreview() => mobilePreviewMessages(
    _connection == null
        ? [
            (_strings.mobileConnectDesktopToBrowseFolders5932deae()),
            (_strings.mobileSearchFilenamesAndPreviewFiles64b34549()),
          ]
        : [
            (_strings.mobileConnectedC2fd31af(
              value1: _connection!.workspaceTitle,
            )),
            (_strings.mobileBrowseFoldersAndSearchFiles478df008()),
          ],
  );

  @override
  String buildMobileDetailDocument(Brightness brightness) {
    final dark = brightness == Brightness.dark;
    final background = dark ? '#171B22' : '#F5F4F1';
    final surface = dark ? '#242A34' : '#FFFFFF';
    final text = dark ? '#F1F3F6' : '#191D25';
    final dim = dark ? '#A9B3C1' : '#667080';
    return '''<!doctype html><html lang="${_strings.locale}"><head>
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1">
<style>
:root{color-scheme:${dark ? 'dark' : 'light'};--bg:$background;--surface:$surface;--text:$text;--dim:$dim;--blue:#${dark ? '5B92FF' : '1765E9'}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:16px -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;padding:18px}
h1{font-size:29px;letter-spacing:-.04em;margin:10px 0 5px}p{color:var(--dim);margin:0 0 18px}
.search{display:flex;gap:8px;margin:16px 0}.search input{flex:1;min-width:0;border:0;border-radius:13px;background:var(--surface);color:var(--text);font:inherit;padding:13px 14px}.search button,button{border:0;border-radius:12px;background:var(--blue);color:white;padding:10px 15px;font:inherit}
.crumb{display:flex;gap:7px;align-items:center;margin:18px 0;color:var(--dim)}.crumb button{background:transparent;color:var(--blue);padding:4px}
.list{background:var(--surface);border-radius:18px;overflow:hidden}.row{width:100%;display:flex;gap:12px;align-items:center;background:transparent;color:var(--text);border-radius:0;text-align:left;padding:16px;border-bottom:1px solid #8883}.row:last-child{border-bottom:0}.row span:first-child{font-size:21px;color:var(--blue)}.row small{display:block;color:var(--dim);font-size:12px;margin-top:3px}
progress{width:100%;height:8px;accent-color:var(--blue)}#opening{background:var(--surface);padding:16px;border-radius:16px;margin-top:16px}#opening small{display:block;color:var(--dim);margin-top:8px}button:disabled{opacity:.5}
#preview{white-space:pre-wrap;overflow-wrap:anywhere;background:var(--surface);border-radius:16px;padding:17px;margin-top:16px;max-height:55vh;overflow:auto}#preview img{max-width:100%;height:auto}#status{color:var(--dim);font-size:14px;margin:12px 0}
</style></head><body>
<h1 data-message="html.workspace_files">${htmlMessageText(_strings.format('html.workspace_files'))}</h1><p data-message="html.browse_authorized_desktop_workspaces">${htmlMessageText(_strings.format('html.browse_authorized_desktop_workspaces'))}</p>
<form class="search" id="searchForm"><input id="query" data-placeholder-message="html.search_filenames" placeholder="${htmlMessageText(_strings.format('html.search_filenames'))}" data-aria-message="html.search_filenames" aria-label="${htmlMessageText(_strings.format('html.search_filenames'))}"><button data-message="html.search">${htmlMessageText(_strings.format('html.search'))}</button></form>
<div class="crumb" id="crumb"></div><div id="status" data-message="html.loading_folders">${htmlMessageText(_strings.format('html.loading_folders'))}</div><div class="list" id="list"></div><div id="opening" hidden role="status" aria-live="polite"><div id="step"></div><progress id="progress"></progress><small id="transfer"></small></div><div id="preview" hidden></div>
${htmlMessageScript(_strings.namespaceData('html'))}<script>
const t=(id,args)=>openMuseMessages.format(id,args),setMessage=(el,id,args)=>openMuseMessages.set(el,id,args),plainText=(el,text)=>{delete el.dataset.message;delete el.dataset.messageArgs;el.textContent=String(text)};

const waiting=new Map();let next=0;let mounts=[];let history=[];
window.openMuseHostResponse=raw=>{const m=JSON.parse(raw);const p=waiting.get(m.id);if(!p)return;waiting.delete(m.id);m.error?p.reject(new Error(m.error)):p.resolve(m.result)};
function call(method,args){return new Promise((resolve,reject)=>{const id=String(++next);waiting.set(id,{resolve,reject});OpenMusePlugin.postMessage(JSON.stringify({id,method,arguments:args}))})}
const list=document.getElementById('list'),status=document.getElementById('status'),crumb=document.getElementById('crumb'),preview=document.getElementById('preview');
function row(icon,name,sub,click){const b=document.createElement('button');b.className='row';const i=document.createElement('span');i.textContent=icon;const t=document.createElement('span');t.textContent=name;const s=document.createElement('small');setMessage(s,sub);t.append(s);b.append(i,t);b.onclick=click;return b}
function showError(e){plainText(status,e.message||e)}
function renderCrumb(){crumb.replaceChildren();const back=document.createElement('button');setMessage(back,history.length?'html.back':'html.folders');back.onclick=()=>{if(history.length){const old=history.pop();old()}};crumb.append(back)}
async function openMounts(){renderCrumb();preview.hidden=true;setMessage(status,'html.loading_workspace_folders');list.replaceChildren();try{const data=await call('mounts',{});mounts=data.mounts||[];setMessage(status,mounts.length?'html.choose_a_folder':'html.no_folders_are_available_on_desktop');for(const m of mounts)list.append(row('▣',m.title,'html.workspace',()=>openDirectory(m.mountRef,m.rootRef,m.title)))}catch(e){showError(e)}}
async function openDirectory(mountRef,parentRef,title){renderCrumb();preview.hidden=true;setMessage(status,'html.loadingNamed',{name:title});list.replaceChildren();try{let cursor=null;let count=0;do{const page=await call('children',{mountRef,parentRef,cursor});for(const entry of page.entries||[]){count++;list.append(row(entry.isDirectory?'▣':'▤',entry.name,entry.isDirectory?'html.folder':'html.file',()=>{history.push(()=>openDirectory(mountRef,parentRef,title));entry.isDirectory?openDirectory(mountRef,entry.nodeRef,entry.name):openPreview(entry.resourceRef,entry.name)}))}cursor=page.nextCursor}while(cursor&&count<300);setMessage(status,count?'html.fileCount':'html.empty_folder',{count});renderCrumb()}catch(e){showError(e)}}
let opening=false;
const openingBox=document.getElementById('opening'),stepText=document.getElementById('step'),bar=document.getElementById('progress'),transfer=document.getElementById('transfer');
function setBusy(value){opening=value;document.querySelectorAll('button,input').forEach(e=>e.disabled=value)}
function showProgress(p){stepText.textContent=p.step;const known=p.total>0;if(known){bar.max=p.total;bar.value=p.received}else{bar.removeAttribute('value')}const mb=n=>(n/1048576).toFixed(2)+' MiB';transfer.textContent=(p.received?mb(p.received)+(known?' / '+mb(p.total):'')+' · ':'')+t('html.elapsed',{seconds:(p.elapsedMs/1000).toFixed(1)})}
async function openPreview(resourceRef,name){if(opening)return;renderCrumb();list.replaceChildren();preview.hidden=true;plainText(status,name);openingBox.hidden=false;setBusy(true);showProgress({step:t('html.preparing_to_open_the_file'),received:0,elapsedMs:0});let polling=false;const timer=setInterval(async()=>{if(polling)return;polling=true;try{showProgress(await call('openingProgress',{}))}catch(_){}finally{polling=false}},250);try{await call('openResource',{resourceRef,name});preview.hidden=false;preview.replaceChildren();const notice=document.createElement('span');setMessage(notice,'html.the_editor_has_closed_select_reopen_to_open_it_again');preview.append(notice)}catch(e){preview.hidden=false;preview.textContent=String(e.message||e)}finally{clearInterval(timer);openingBox.hidden=true;setBusy(false);const retry=document.createElement('button');setMessage(retry,'html.reopen');retry.onclick=()=>openPreview(resourceRef,name);preview.append(document.createElement('br'),retry)}}

document.getElementById('searchForm').onsubmit=async e=>{e.preventDefault();const q=document.getElementById('query').value.trim();if(!q){history=[];openMounts();return}history=[openMounts];renderCrumb();list.replaceChildren();preview.hidden=true;setMessage(status,'html.searching');try{const data=await call('search',{query:q});for(const entry of data.entries||[])list.append(row(entry.isDirectory?'▣':'▤',entry.name,entry.isDirectory?'html.folder':'html.file',()=>entry.isDirectory?openDirectory(entry.mountRef,entry.nodeRef,entry.name):openPreview(entry.resourceRef,entry.name)));setMessage(status,'html.searchResultCount',{count:(data.entries||[]).length,truncated:data.truncated?'yes':'no'})}catch(e){showError(e)}};
openMounts();

openMuseMessages.bind(()=>call('localization',{}));
</script></body></html>''';
  }

  @override
  Future<Object?> handleMobileDetailRequest(
    String method,
    Object? arguments,
  ) async {
    if (method == 'localization') return _strings.namespaceData('html');
    if (method == 'openingProgress') return openingProgress;
    final connection = _connection;
    if (connection == null) {
      throw StateError((_strings.mobileConnectToDesktopFirst31c48b63()));
    }
    final args = arguments is Map
        ? Map<String, Object?>.from(arguments)
        : <String, Object?>{};
    final client = MobileWorkspaceMirrorClient(connection, strings: _strings);
    try {
      return switch (method) {
        'mounts' => await client.mounts(),
        'children' => await client.children(
          mountRef: _required(args, 'mountRef'),
          parentRef: _required(args, 'parentRef'),
          cursor: args['cursor'] as String?,
        ),
        'search' => await client.search(_required(args, 'query')),
        'preview' => await client.preview(_required(args, 'resourceRef')),
        'openResource' || 'openMarkdown' => await _openResource(
          _required(args, 'resourceRef'),
          _required(args, 'name'),
        ),
        _ => throw FormatException(
          (_strings.mobileUnknownFileOperationAe6b4b8a()),
        ),
      };
    } finally {
      client.close();
    }
  }

  Future<Map<String, Object?>> _openResource(
    String resourceRef,
    String name,
  ) async {
    if (_opening) {
      throw StateError(
        (_strings.mobileAFileIsAlreadyOpeningPleaseWait2906e99f()),
      );
    }
    final open = onOpenResource;
    if (open == null) {
      throw StateError((_strings.mobileTheHostIsNotReady222d3501()));
    }
    final resource = OpenMuseResource(
      uri: Uri(scheme: 'workspace', path: '/$name'),
      displayName: name,
      mediaType: name.toLowerCase().endsWith('.md') ? 'text/markdown' : null,
    );
    _opening = true;
    _timings.clear();
    _openingClock
      ..reset()
      ..start();
    _report((_strings.mobilePreparingToOpenTheFile6349c791()));
    try {
      await open(resource, () async {
        final connection = _connection;
        if (connection == null) {
          throw StateError((_strings.mobileDesktopIsDisconnected0021ecf1()));
        }
        final reader = MobileWorkspaceMirrorClient(
          connection,
          strings: _strings,
        );
        try {
          return await reader.readResource(resourceRef, onProgress: _report);
        } finally {
          reader.close();
        }
      }, (step) => _report(step));
      return {'opened': true};
    } finally {
      _opening = false;
      _openingClock.stop();
    }
  }

  String _required(Map<String, Object?> args, String key) {
    final value = args[key];
    if (value is! String || value.isEmpty || value.length > 200) {
      throw FormatException(
        (_strings.mobileInvalidParameter5d4a68c6(value1: key)),
      );
    }
    return value;
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
