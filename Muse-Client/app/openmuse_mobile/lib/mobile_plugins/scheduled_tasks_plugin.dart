import '../mobile_build_locale.dart';

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../workbuddy/workbuddy_models.dart';
import 'mobile_preview_messages.dart';
import 'mobile_scheduled_task_client.dart';
import 'paired_mobile_context.dart';

final class MobileScheduledTasksPlugin extends ChangeNotifier
    implements
        OpenMusePlugin,
        OpenMuseMobileWorkbenchContributor,
        PairedMobileContextContributor {
  MobileScheduledTasksPlugin({OpenMuseStrings? strings})
    : _strings = strings ?? OpenMuseStrings.fallback;
  final OpenMuseStrings _strings;

  PairedDesktopConnection? _connection;
  List<WbWorkspace> _workspaces = const [];
  String? _defaultWorkspaceId;
  List<Map<String, Object?>> _tasks = const [];

  void updateContext({
    required PairedDesktopConnection? connection,
    required List<WbWorkspace> workspaces,
    required String? selectedWorkspaceId,
  }) {
    final connectionChanged = _connection?.grantRef != connection?.grantRef;
    final nextWorkspaces = workspaces
        .where((w) => w.id.startsWith('dsh.workspace.'))
        .toList(growable: false);
    final nextDefaultWorkspaceId =
        selectedWorkspaceId?.startsWith('dsh.workspace.') == true
        ? selectedWorkspaceId!.substring('dsh.workspace.'.length)
        : nextWorkspaces.isEmpty
        ? null
        : nextWorkspaces.first.id.substring('dsh.workspace.'.length);
    final workspacesChanged =
        _workspaces.length != nextWorkspaces.length ||
        Iterable.generate(nextWorkspaces.length).any(
          (index) =>
              _workspaces[index].id != nextWorkspaces[index].id ||
              _workspaces[index].name != nextWorkspaces[index].name,
        );
    if (!connectionChanged &&
        !workspacesChanged &&
        _defaultWorkspaceId == nextDefaultWorkspaceId) {
      return;
    }
    _connection = connection;
    _workspaces = nextWorkspaces;
    _defaultWorkspaceId = nextDefaultWorkspaceId;
    notifyListeners();
    if (connectionChanged && connection != null) unawaited(_refresh());
  }

  @override
  void updatePairedContext({
    required PairedDesktopConnection? connection,
    required List<WbWorkspace> workspaces,
    required String? selectedWorkspaceId,
  }) => updateContext(
    connection: connection,
    workspaces: workspaces,
    selectedWorkspaceId: selectedWorkspaceId,
  );

  Future<void> _refresh() async {
    final connection = _connection;
    if (connection == null) return;
    final client = MobileScheduledTaskClient(connection, strings: _strings);
    try {
      final result = await client.request({'operation': 'list'});
      if (_connection?.grantRef != connection.grantRef) return;
      _tasks = [
        for (final item in result['tasks'] as List? ?? const [])
          if (item is Map) Map<String, Object?>.from(item),
      ];
      notifyListeners();
    } on Object {
      // The card keeps its last verified snapshot while Desktop reconnects.
    } finally {
      client.close();
    }
  }

  @override
  OpenMusePluginDescriptor get descriptor => OpenMusePluginDescriptor(
    id: 'com.openmuse.mobile.scheduled-tasks',
    name: (_strings.mobileScheduledTasksD8f7d270()),
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: ['onStartup'],
    permissions: {'dsh.paired.schedule'},
  );

  @override
  OpenMuseWorkbenchCard get workbenchCard => OpenMuseWorkbenchCard(
    title: (_strings.mobileScheduledTasksD8f7d270()),
    summary: (_strings.mobileScheduleMessagesForADesktopWorkspace9367bb21()),
    height: OpenMuseWidgetHeight.medium,
  );

  @override
  Listenable get workbenchChanges => this;

  @override
  List<Map<String, Object?>> buildWorkbenchPreview() {
    final pending = _tasks.where((task) => task['state'] == 'pending').toList();
    pending.sort((a, b) => '${a['dueAt']}'.compareTo('${b['dueAt']}'));
    return mobilePreviewMessages([
      if (_connection == null)
        (_strings.mobileConnectDesktopToScheduleATask45c76ebb())
      else if (pending.isEmpty)
        (_strings.mobileNoPendingTasks492f3c23())
      else
        (_strings.mobilePendingNextA4e2950f(
          value1: pending.length,
          value2: pending.first['dueAt'],
        )),
      (_strings.mobileChooseAWorkspaceTimeAndMessageF0776a30()),
    ]);
  }

  @override
  String buildMobileDetailDocument(Brightness brightness) {
    final dark = brightness == Brightness.dark;
    return '''<!doctype html><html lang="${_strings.locale}"><head>
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1">
<style>
:root{color-scheme:${dark ? 'dark' : 'light'};--bg:${dark ? '#171B22' : '#F5F4F1'};--surface:${dark ? '#242A34' : '#FFFFFF'};--text:${dark ? '#F1F3F6' : '#191D25'};--dim:${dark ? '#A9B3C1' : '#667080'};--blue:${dark ? '#5B92FF' : '#1765E9'}}
*{box-sizing:border-box}body{margin:0;padding:20px;background:var(--bg);color:var(--text);font:16px -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}h1{font-size:30px;letter-spacing:-.04em;margin:8px 0 4px}p{color:var(--dim);margin:0 0 22px}.panel{background:var(--surface);border-radius:20px;padding:18px;margin-bottom:16px}label{display:block;font-size:13px;color:var(--dim);margin:14px 0 7px}input,textarea,select{display:block;width:100%;border:1px solid #8884;border-radius:12px;padding:12px;background:var(--bg);color:var(--text);font:inherit}textarea{min-height:96px;resize:vertical}button{border:0;border-radius:12px;background:var(--blue);color:white;padding:12px 18px;font:inherit;margin-top:16px}.task{padding:14px 0;border-bottom:1px solid #8883}.task:last-child{border:0}.task small{display:block;color:var(--dim);margin-top:5px}.task button{background:transparent;color:var(--blue);padding:4px 0;margin:8px 0 0}#status{color:var(--dim);font-size:14px;margin-bottom:16px}
</style></head><body>
<h1 data-message="html.scheduled_tasks">${htmlMessageText(_strings.format('html.scheduled_tasks'))}</h1><p data-message="html.send_a_message_to_a_workspace_at_a_scheduled_time">${htmlMessageText(_strings.format('html.send_a_message_to_a_workspace_at_a_scheduled_time'))}</p>
<div class="panel"><form id="form"><label for="workspace" data-message="html.workspace">${htmlMessageText(_strings.format('html.workspace'))}</label><select id="workspace" required></select><label for="due" data-message="html.scheduled_time">${htmlMessageText(_strings.format('html.scheduled_time'))}</label><input id="due" type="datetime-local" required><label for="message" data-message="html.message">${htmlMessageText(_strings.format('html.message'))}</label><textarea id="message" maxlength="4000" data-placeholder-message="html.enter_a_message" placeholder="${htmlMessageText(_strings.format('html.enter_a_message'))}" required></textarea><button data-message="html.schedule_task">${htmlMessageText(_strings.format('html.schedule_task'))}</button></form></div>
<div id="status" data-message="html.loading_scheduled_tasks">${htmlMessageText(_strings.format('html.loading_scheduled_tasks'))}</div><div class="panel" id="tasks"></div>
${htmlMessageScript(_strings.namespaceData('html'))}<script>
const t=(id,args)=>openMuseMessages.format(id,args),setMessage=(el,id,args)=>openMuseMessages.set(el,id,args),plainText=(el,text)=>{delete el.dataset.message;delete el.dataset.messageArgs;el.textContent=String(text)};

const waiting=new Map();let next=0;
window.openMuseHostResponse=raw=>{const m=JSON.parse(raw),p=waiting.get(m.id);if(!p)return;waiting.delete(m.id);m.error?p.reject(new Error(m.error)):p.resolve(m.result)};
function call(method,args){return new Promise((resolve,reject)=>{const id=String(++next);waiting.set(id,{resolve,reject});OpenMusePlugin.postMessage(JSON.stringify({id,method,arguments:args}))})}
const status=document.getElementById('status'),tasks=document.getElementById('tasks');
function row(task){const box=document.createElement('div');box.className='task';const title=document.createElement('div');title.textContent=task.message;const sub=document.createElement('small');const date=document.createElement('span');date.dataset.date=task.dueAt;date.textContent=new Date(task.dueAt).toLocaleString(document.documentElement.lang);const state=document.createElement('span');setMessage(state,'html.taskState',{state:task.state});sub.append(date,document.createTextNode(' · '),state);box.append(title,sub);if(task.state==='pending'){const cancel=document.createElement('button');setMessage(cancel,'html.cancel');cancel.onclick=async()=>{try{await call('cancel',{id:task.id});await refresh()}catch(e){plainText(status,e.message||e)}};box.append(cancel)}return box}
async function refresh(){try{const data=await call('bootstrap',{});const select=document.getElementById('workspace');select.replaceChildren();for(const w of data.workspaces){const o=document.createElement('option');o.value=w.id;o.textContent=w.name;select.append(o)}if(data.defaultWorkspaceId)select.value=data.defaultWorkspaceId;tasks.replaceChildren();for(const t of data.tasks)tasks.append(row(t));setMessage(status,'html.taskCount',{count:data.tasks.length});if(!data.workspaces.length)setMessage(status,'html.connect_and_select_a_desktop_workspace_first')}catch(e){plainText(status,e.message||e)}}
document.getElementById('form').onsubmit=async e=>{e.preventDefault();const workspaceId=document.getElementById('workspace').value,message=document.getElementById('message').value.trim(),local=document.getElementById('due').value;if(!workspaceId||!message||!local)return;try{setMessage(status,'html.creating');await call('create',{workspaceId,message,dueAt:new Date(local).toISOString()});document.getElementById('message').value='';await refresh()}catch(e){plainText(status,e.message||e)}};
refresh();

openMuseMessages.bind(()=>call('localization',{}));
</script></body></html>''';
  }

  @override
  Future<Object?> handleMobileDetailRequest(
    String method,
    Object? arguments,
  ) async {
    if (method == 'localization') return _strings.namespaceData('html');
    final connection = _connection;
    if (connection == null) {
      throw StateError((_strings.mobileConnectToDesktopFirst31c48b63()));
    }
    final args = arguments is Map
        ? Map<String, Object?>.from(arguments)
        : <String, Object?>{};
    final client = MobileScheduledTaskClient(connection, strings: _strings);
    try {
      switch (method) {
        case 'bootstrap':
          final response = await client.request({'operation': 'list'});
          _tasks = [
            for (final item in response['tasks'] as List? ?? const [])
              if (item is Map) Map<String, Object?>.from(item),
          ];
          notifyListeners();
          return {
            'tasks': _tasks,
            'workspaces': [
              for (final w in _workspaces)
                {'id': w.id.substring('dsh.workspace.'.length), 'name': w.name},
            ],
            'defaultWorkspaceId': _defaultWorkspaceId,
          };
        case 'create':
          final id = args['workspaceId'];
          final message = args['message'];
          final due = args['dueAt'];
          if (id is! String ||
              !_workspaces.any((w) => w.id == 'dsh.workspace.$id') ||
              message is! String ||
              message.trim().isEmpty ||
              due is! String ||
              DateTime.tryParse(due) == null) {
            throw FormatException(
              (_strings.mobileInvalidScheduledTaskInput2dac2d75()),
            );
          }
          return client.request({
            'operation': 'create',
            'workspaceId': id,
            'message': message,
            'dueAt': due,
          });
        case 'cancel':
          final id = args['id'];
          if (id is! String || id.isEmpty || id.length > 80) {
            throw FormatException((_strings.mobileInvalidTaskIdBee5d4a5()));
          }
          return client.request({'operation': 'cancel', 'id': id});
        default:
          throw FormatException(
            (_strings.mobileUnknownScheduledTaskActionD2a60cc3()),
          );
      }
    } finally {
      client.close();
    }
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
