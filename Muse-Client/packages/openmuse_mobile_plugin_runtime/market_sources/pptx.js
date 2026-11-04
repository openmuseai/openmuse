// Mobile control surface for the Host-provided PPTX read-only viewer.
// Native PPTX parsing stays in the app; this QuickJS package gets no file path.
const catalogId = 'https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json';
globalThis.openMusePlugin = {
  onStart() {
    return [
      {version:'v0.9',createSurface:{surfaceId:'main',catalogId}},
      {version:'v0.9',updateComponents:{surfaceId:'main',components:[
        {id:'root',component:'Column',children:['title','status','usage','scope']},
        {id:'title',component:'Text',text:'PPTX 阅读器',variant:'h2'},
        {id:'status',component:'Text',text:'插件已启用 · Android 只读预览'},
        {id:'usage',component:'Text',text:'在 Cloud Workspace 中点开 .pptx 文件即可查看幻灯片文字。'},
        {id:'scope',component:'Text',text:'文件由 OpenMuse Host 读取和校验；此 QuickJS 界面不接收原始路径或凭据。'},
      ]}},
    ];
  },
  onAction() { return []; },
};
