/* Review-only fixtures. Not a plugin manifest or production protocol. */
window.resourceCenterFixtures = {
  resources: [
    {id:'proposal',ref:'demo-resource-proposal',name:'秋季发布提案.pptx',nameEn:'Autumn launch proposal.pptx',kind:'slides',source:'pc',workspace:'product',location:'产品设计 / 资料',locationEn:'Product design / Materials',size:'2.8 MB',group:'today',time:'14:32',writable:false,handlers:['GenOffice'],title:'让创作回到内容本身',titleEn:'Bring the focus back to the work',text:'把分散的素材、讨论与文档，整理成一条可以继续的创作路径。',textEn:'Bring the materials, conversations and documents into one place to continue the work.'},
    {id:'cover',ref:'demo-resource-cover',name:'品牌封面.png',nameEn:'Brand cover.png',kind:'image',source:'local',location:'文档 / 品牌素材',locationEn:'Documents / Brand assets',size:'684 KB',group:'today',time:'12:18',writable:true,handlers:['OpenMuse Image'],title:'品牌封面',titleEn:'Brand cover'},
    {id:'review',ref:'demo-resource-review',name:'接口评审记录.md',nameEn:'Interface review notes.md',kind:'file',source:'pc',workspace:'product',location:'产品设计 / 资料',locationEn:'Product design / Materials',size:'12 KB',group:'today',time:'09:47',writable:false,handlers:['Note Lab','Open File Viewer'],title:'接口评审记录',titleEn:'Interface review notes',text:'资源描述负责说明内容；授权在打开与使用时重新确认。界面展示位置，但不把位置当作访问权限。',textEn:'A resource descriptor explains the content. Access is checked when opening and using it. A displayed location does not grant access.'},
    {id:'video',ref:'demo-resource-video',name:'产品演示片段.mp4',nameEn:'Product walkthrough.mp4',kind:'video',source:'local',location:'导入的资源',locationEn:'Imported resources',size:'18.6 MB',group:'yesterday',time:'18:06',writable:true,handlers:[],title:'产品演示片段',titleEn:'Product walkthrough'},
    {id:'url',ref:'demo-resource-url',name:'设计规范站点',nameEn:'Design guidelines',kind:'link',source:'url',location:'example.com',locationEn:'example.com',size:'URL',group:'yesterday',time:'15:24',writable:true,handlers:['OpenMuse Links'],title:'设计规范站点',titleEn:'Design guidelines',text:'网页也是资源。它保留来源，通过支持链接的本机插件打开。',textEn:'A web address is also a resource, opened through a compatible local handler.'},
    {id:'brief',ref:'demo-resource-brief',name:'产品方向备忘.md',nameEn:'Product direction.md',kind:'file',source:'local',location:'文档',locationEn:'Documents',size:'8 KB',group:'week',time:'10月29日',writable:true,handlers:['Note Lab','Open File Viewer'],title:'产品方向',titleEn:'Product direction',text:'尽量少打断：资源可以被找回、打开、带到对话里。',textEn:'Keep the work moving: find a resource, open it, and bring it into a conversation.'},
    {id:'remote-asset',ref:'demo-resource-remote-asset',name:'渠道标识.png',nameEn:'Channel artwork.png',kind:'image',source:'pc',workspace:'product',mount:'shared',location:'产品设计 / 共享素材 / 品牌素材',locationEn:'Product design / Shared assets / Brand assets',size:'218 KB',group:'not-opened',writable:false,handlers:['OpenMuse Image'],title:'渠道标识',titleEn:'Channel artwork'}
  ],
  creators: [
    {id:'slides',name:'演示文稿',nameEn:'Presentation',plugin:'GenOffice',kind:'slides',defaultName:'未命名演示文稿.pptx',defaultNameEn:'Untitled presentation.pptx'},
    {id:'note',name:'空白笔记',nameEn:'Blank note',plugin:'Note Lab',kind:'note',defaultName:'未命名笔记.md',defaultNameEn:'Untitled note.md'},
    {id:'markdown',name:'Markdown',nameEn:'Markdown',plugin:'Note Lab',kind:'file',defaultName:'未命名文档.md',defaultNameEn:'Untitled document.md'},
    {id:'link',name:'网址收藏',nameEn:'Web link',plugin:'OpenMuse Links',kind:'link',defaultName:'新网址',defaultNameEn:'New web link'}
  ]
};
