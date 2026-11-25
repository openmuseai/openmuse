// Independent Mobile demonstration of a usage budget workflow.
// Numbers are local sample values, not DeepSeek account data.
const catalogId = 'https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json';
let spent = 36;
function view() {
  return [{version:'v0.9',updateComponents:{surfaceId:'main',components:[
    {id:'root',component:'Column',children:['title','description','budget','spent','remaining','add']},
    {id:'title',component:'Text',text:'用量与预算',variant:'h2'},
    {id:'description',component:'Text',text:'本地示例数据；接入真实 DSH 用量需 Host 授权能力'},
    {id:'budget',component:'Text',text:'预算：¥100'},
    {id:'spent',component:'Text',text:'已用：¥'+spent},
    {id:'remaining',component:'Text',text:'剩余：¥'+Math.max(0,100-spent)},
    {id:'add_label',component:'Text',text:'模拟增加 ¥5 用量'},
    {id:'add',component:'Button',child:'add_label',variant:'primary',action:{event:{name:'addUsage'}}},
  ]}}];
}
globalThis.openMusePlugin = {
  onStart() { return [{version:'v0.9',createSurface:{surfaceId:'main',catalogId}},...view()]; },
  onAction(action) { if (action.name === 'addUsage') spent += 5; return view(); },
};
