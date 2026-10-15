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
    final client = MobileScheduledTaskClient(connection);
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
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.mobile.scheduled-tasks',
    name: '定时任务',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: ['onStartup'],
    permissions: {'dsh.paired.schedule'},
  );

  @override
  OpenMuseWorkbenchCard get workbenchCard => const OpenMuseWorkbenchCard(
    title: '定时任务',
    summary: '向 Desktop Workspace 定时发送消息',
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
        '连接 Desktop 后创建计划'
      else if (pending.isEmpty)
        '暂无待执行任务'
      else
        '${pending.length} 个待执行 · 下一项 ${pending.first['dueAt']}',
      '指定 Workspace、时间与消息',
    ]);
  }

  @override
  String buildMobileDetailDocument(Brightness brightness) {
    final dark = brightness == Brightness.dark;
    return '''<!doctype html><html lang="zh-CN"><head>
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1">
<style>
:root{color-scheme:${dark ? 'dark' : 'light'};--bg:${dark ? '#171B22' : '#F5F4F1'};--surface:${dark ? '#242A34' : '#FFFFFF'};--text:${dark ? '#F1F3F6' : '#191D25'};--dim:${dark ? '#A9B3C1' : '#667080'};--blue:${dark ? '#5B92FF' : '#1765E9'}}
*{box-sizing:border-box}body{margin:0;padding:20px;background:var(--bg);color:var(--text);font:16px -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}h1{font-size:30px;letter-spacing:-.04em;margin:8px 0 4px}p{color:var(--dim);margin:0 0 22px}.panel{background:var(--surface);border-radius:20px;padding:18px;margin-bottom:16px}label{display:block;font-size:13px;color:var(--dim);margin:14px 0 7px}input,textarea,select{display:block;width:100%;border:1px solid #8884;border-radius:12px;padding:12px;background:var(--bg);color:var(--text);font:inherit}textarea{min-height:96px;resize:vertical}button{border:0;border-radius:12px;background:var(--blue);color:white;padding:12px 18px;font:inherit;margin-top:16px}.task{padding:14px 0;border-bottom:1px solid #8883}.task:last-child{border:0}.task small{display:block;color:var(--dim);margin-top:5px}.task button{background:transparent;color:var(--blue);padding:4px 0;margin:8px 0 0}#status{color:var(--dim);font-size:14px;margin-bottom:16px}
</style></head><body>
<h1>定时任务</h1><p>在指定时间向 Workspace 发送消息</p>
<div class="panel"><form id="form"><label for="workspace">Workspace</label><select id="workspace" required></select><label for="due">执行时间</label><input id="due" type="datetime-local" required><label for="message">消息</label><textarea id="message" maxlength="4000" placeholder="输入要发送的消息" required></textarea><button>创建定时任务</button></form></div>
<div id="status">正在读取计划…</div><div class="panel" id="tasks"></div>
<script>
const waiting=new Map();let next=0;
window.openMuseHostResponse=raw=>{const m=JSON.parse(raw),p=waiting.get(m.id);if(!p)return;waiting.delete(m.id);m.error?p.reject(new Error(m.error)):p.resolve(m.result)};
function call(method,args){return new Promise((resolve,reject)=>{const id=String(++next);waiting.set(id,{resolve,reject});OpenMusePlugin.postMessage(JSON.stringify({id,method,arguments:args}))})}
const status=document.getElementById('status'),tasks=document.getElementById('tasks');
function row(task){const box=document.createElement('div');box.className='task';const title=document.createElement('div');title.textContent=task.message;const sub=document.createElement('small');sub.textContent=new Date(task.dueAt).toLocaleString()+' · '+task.state;box.append(title,sub);if(task.state==='pending'){const cancel=document.createElement('button');cancel.textContent='取消';cancel.onclick=async()=>{try{await call('cancel',{id:task.id});await refresh()}catch(e){status.textContent=String(e.message||e)}};box.append(cancel)}return box}
async function refresh(){try{const data=await call('bootstrap',{});const select=document.getElementById('workspace');select.replaceChildren();for(const w of data.workspaces){const o=document.createElement('option');o.value=w.id;o.textContent=w.name;select.append(o)}if(data.defaultWorkspaceId)select.value=data.defaultWorkspaceId;tasks.replaceChildren();for(const t of data.tasks)tasks.append(row(t));status.textContent=data.tasks.length?data.tasks.length+' 个任务':'暂无定时任务';if(!data.workspaces.length)status.textContent='请先连接并选择 Desktop Workspace'}catch(e){status.textContent=String(e.message||e)}}
document.getElementById('form').onsubmit=async e=>{e.preventDefault();const workspaceId=document.getElementById('workspace').value,message=document.getElementById('message').value.trim(),local=document.getElementById('due').value;if(!workspaceId||!message||!local)return;try{status.textContent='正在创建…';await call('create',{workspaceId,message,dueAt:new Date(local).toISOString()});document.getElementById('message').value='';await refresh()}catch(e){status.textContent=String(e.message||e)}};
refresh();
</script></body></html>''';
  }

  @override
  Future<Object?> handleMobileDetailRequest(
    String method,
    Object? arguments,
  ) async {
    final connection = _connection;
    if (connection == null) throw StateError('请先连接 Desktop');
    final args = arguments is Map
        ? Map<String, Object?>.from(arguments)
        : <String, Object?>{};
    final client = MobileScheduledTaskClient(connection);
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
            throw const FormatException('定时任务输入无效');
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
            throw const FormatException('任务 ID 无效');
          }
          return client.request({'operation': 'cancel', 'id': id});
        default:
          throw const FormatException('未知定时任务操作');
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
