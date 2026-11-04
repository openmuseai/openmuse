// Independent Mobile demonstration of a task-board workflow.
// This does not contain code from the DSH task-board package.
const catalogId = 'https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json';
const tasks = ['梳理 Mobile 插件协议', '验证 Android 真机安装'];
function view() {
  const components = [
    {id:'root',component:'Column',children:['title','subtitle','draft','add','divider',...tasks.map((_,i)=>'task'+i)]},
    {id:'title',component:'Text',text:'任务看板',variant:'h2'},
    {id:'subtitle',component:'Text',text:'本地 QuickJS 状态 · Flutter 原生界面'},
    {id:'draft',component:'TextField',label:'新任务',value:{path:'/draft'},variant:'shortText'},
    {id:'add_label',component:'Text',text:'添加任务'},
    {id:'add',component:'Button',child:'add_label',variant:'primary',action:{event:{name:'add',context:{title:{path:'/draft'}}}}},
    {id:'divider',component:'Divider',axis:'horizontal'},
    ...tasks.map((task,i)=>({id:'task'+i,component:'Text',text:(i+1)+'. '+task})),
  ];
  return [{version:'v0.9',updateComponents:{surfaceId:'main',components}}];
}
globalThis.openMusePlugin = {
  onStart() {
    return [{version:'v0.9',createSurface:{surfaceId:'main',catalogId}},
      ...view(),{version:'v0.9',updateDataModel:{surfaceId:'main',path:'/',value:{draft:''}}}];
  },
  onAction(action) {
    if (action.name !== 'add') return [];
    const title = String(action.context?.title ?? '').trim().slice(0,80);
    if (title) tasks.push(title);
    return [...view(),{version:'v0.9',updateDataModel:{surfaceId:'main',path:'/draft',value:''}}];
  },
};
