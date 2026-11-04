/* Offline product-review prototype. All operations are in-memory simulations. */
(() => {
  'use strict';
  const $ = (q, root = document) => root.querySelector(q);
  const $$ = (q, root = document) => [...root.querySelectorAll(q)];
  const esc = v => String(v ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  const paths = {
    home:'M3 10.5 12 3l9 7.5M5.5 9v11h5v-6h3v6h5V9',
    task:'M5 4h14a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H9l-5 3v-4a2 2 0 0 1-1-1.7V6a2 2 0 0 1 2-2Z',
    workbench:'M4 4h6v6H4zM14 4h6v6h-6zM4 14h6v6H4zM14 14h6v6h-6z',
    expert:'M19.5 12a7.5 7.5 0 1 0-15 0 7.5 7.5 0 0 0 15 0ZM8 10.5h.01M16 10.5h.01M9 14a3.2 3.2 0 0 0 6 0M6.5 6 5 2',
    compass:'M20.5 12a8.5 8.5 0 1 0-17 0 8.5 8.5 0 0 0 17 0ZM15.8 8.2l-2.4 5.2-5.2 2.4 2.4-5.2Z',
    person:'M16 7a4 4 0 1 0-8 0 4 4 0 0 0 8 0ZM4 21v-1a8 6 0 0 1 16 0v1Z',
    search:'M17.5 10a7.5 7.5 0 1 0-15 0 7.5 7.5 0 0 0 15 0ZM16 16l5 5',
    more:'M12 5h.01M12 12h.01M12 19h.01',
    plus:'M12 4v16M4 12h16',
    close:'M6 6l12 12M18 6 6 18',
    back:'M15 4l-8 8 8 8',
    arrow:'M9 5l7 7-7 7',
    down:'m6 9 6 6 6-6',
    up:'m5 11 7-7 7 7M12 4v16',
    file:'M14 3H5v18h14V8ZM14 3v6h5M8 13h8M8 17h5',
    note:'M5 3h14v18H5ZM8 7h8M8 11h8M8 15h5',
    slides:'M3 4h18v13H3ZM12 17v4M8 21h8M7 8h5M7 12h9',
    image:'M3 4h18v16H3ZM3 17l5-6 5 5 3-3 5 5M16 8h.01',
    video:'M3 4h18v16H3ZM10 8l6 4-6 4Z',
    link:'M9 15l6-6M8 8l2-2a4 4 0 0 1 6 6l-2 2M16 16l-2 2a4 4 0 0 1-6-6l2-2',
    folder:'M3 6h7l2 2h9v12H3ZM3 6V4h7l2 2',
    pc:'M3 4h18v13H3ZM12 17v4M8 21h8',
    phone:'M7 2h10v20H7ZM10 18h4',
    clock:'M21 12a9 9 0 1 0-18 0 9 9 0 0 0 18 0ZM12 7v5l3 2',
    copy:'M8 8h12v13H8ZM16 8V3H3v13h5',
    pencil:'M15 4l5 5M4 20l2-7L17 2l5 5-11 11ZM4 20l7-2',
    download:'M12 3v12m-5-5 5 5 5-5M4 16v5h16v-5',
    upload:'M12 16V4m-5 5 5-5 5 5M4 16v5h16v-5',
    check:'m5 12 4 4L19 6',
    remove:'M5 7h14M9 7V3h6v4M7 7l1 14h8l1-14M10 10v7M14 10v7',
    warning:'M12 3 2 21h20ZM12 9v5M12 17h.01',
    cloud:'M6 18a4 4 0 0 1-1-8 6 6 0 0 1 11-3 5.5 5.5 0 0 1 2 11Z',
    refresh:'M20 9a8 8 0 1 0 .5 5M20 3v6h-6',
    globe:'M21 12a9 9 0 1 0-18 0 9 9 0 0 0 18 0ZM3 12h18M12 3a16 16 0 0 1 0 18 16 16 0 0 1 0-18Z',
    paste:'M9 4H5v17h14V4h-4M9 2h6v5H9ZM8 11h8M8 15h6',
    filter:'M4 6h16M7 12h10M10 18h4',
    info:'M21 12a9 9 0 1 0-18 0 9 9 0 0 0 18 0ZM12 11v6M12 7h.01'
  };
  const icon = (name, cls = '') => `<svg class="icon ${cls}" viewBox="0 0 24 24" aria-hidden="true"><path d="${paths[name] || paths.file}"/></svg>`;
  const params = new URLSearchParams(location.search);
  const fixture = window.resourceCenterFixtures;
  let state, resources, timer, createTimer, sheetFocus, sheetReturnSelector;
  function reset() {
    resources = structuredClone(fixture.resources);
    state = {lang:params.get('lang') === 'en' ? 'en' : 'zh',theme:params.get('theme') === 'dark' ? 'dark' : 'light',tab:'home',section:'recent',remoteDepth:0,remoteWorkspace:'product',remoteMount:'creative',localDepth:0,online:true,notes:true,listState:'ready',query:'',removed:new Set(),clipboard:null,plainTextClipboard:'',draft:[],text:'',messages:[],defaults:{},selectedHandler:null,lastScene:'recent',overlay:null,editor:null,target:'local',creating:false};
  }
  reset();
  const t = (zh,en) => state.lang === 'en' ? en : zh;
  const nameOf = r => state.lang === 'en' ? r.nameEn : r.name;
  const locOf = r => state.lang === 'en' ? r.locationEn : r.location;
  const sourceName = r => r.source === 'pc' ? t('工作电脑','Work PC') : r.source === 'url' ? t('网页','Web') : t('本机','This device');
  const metaOf = r => `${sourceName(r)} · ${locOf(r)}${r.source==='pc'&&!state.online?' · '+t('离线','Offline'):''}`;
  const fileIcon = (kind, cls = '') => `<span class="file-icon ${kind === 'folder' ? 'folder' : ''} ${cls}">${icon(kind)}</span>`;
  const button = (id,label,glyph,cls='icon-button') => `<button class="${cls}" data-action="${id}" aria-label="${esc(label)}">${glyph ? icon(glyph) : esc(label)}</button>`;
  const availableHandlers = r => r.handlers.filter(h => state.notes || h !== 'Note Lab');
  function nav() {
    return `<nav class="bottom-nav" aria-label="${t('主导航','Main navigation')}">${[['home','home','首页','Home'],['tasks','task','任务','Tasks'],['workbench','expert','工作台','Workbench'],['discover','compass','发现','Discover'],['account','person','我的','Account']].map(([id,glyph,zh,en]) => `<button data-nav="${id}" class="${state.tab === id ? 'active' : ''}" ${state.tab === id ? 'aria-current="page"' : ''}>${icon(glyph)}<span>${t(zh,en)}</span></button>`).join('')}</nav>`;
  }
  function row(r) {
    return `<div class="resource-row" data-row="${r.id}"><button class="resource-main" data-open="${r.id}" aria-label="${esc(nameOf(r)+' · '+metaOf(r))}">${fileIcon(r.kind)}<span class="resource-labels"><span class="resource-name">${esc(nameOf(r))}</span><span class="resource-meta">${icon(r.source === 'pc' ? 'pc' : r.source === 'url' ? 'globe' : 'phone')}<span>${esc(metaOf(r))}</span></span></span></button><button class="icon-button more-button" data-menu="${r.id}" aria-label="${esc(t('更多操作：','More actions: ')+nameOf(r))}">${icon('more')}</button></div>`;
  }
  function folderRow(id,name,sub) {
    return `<div class="resource-row"><button class="resource-main" data-folder="${id}">${fileIcon('folder')}<span class="resource-labels"><span class="resource-name">${esc(name)}</span><span class="resource-meta">${esc(sub)}</span></span>${icon('arrow','small-chevron')}</button></div>`;
  }
  function empty(glyph,title,body,action,label) {
    return `<div class="empty-state"><div class="empty-icon">${icon(glyph)}</div><h3>${title}</h3><p>${body}</p><div class="empty-actions"><button class="outline-button" data-action="${action}">${label}</button></div></div>`;
  }
  function search() {
    const placeholder = state.section === 'remote' ? t('搜索当前工作区','Search this workspace') : state.section === 'local' ? t('搜索本机资源','Search this device') : t('搜索最近打开的资源','Search recently opened');
    return `<div class="search-box">${icon('search')}<input id="search-input" aria-label="${placeholder}" placeholder="${placeholder}" value="${esc(state.query)}">${button('filters',t('筛选','Filters'),'filter')}</div>`;
  }
  function skeleton() {
    return `<div role="status" aria-label="${t('正在加载资源','Loading resources')}">${Array.from({length:5},()=>'<div class="skeleton"><span class="skeleton-icon"></span><span class="skeleton-copy"><i></i><i></i></span></div>').join('')}</div>`;
  }
  function homeContent() {
    if (state.listState === 'loading') return skeleton()+`<button class="text-link" data-action="retry">${t('完成演示加载','Finish demo loading')}</button>`;
    if (state.listState === 'error') return empty('warning',t('暂时无法加载','Couldn’t load resources'),t('位置和列表已保留，可以重试。','Your location is kept. Try again when ready.'),'retry',t('重试','Try again'));
    if (state.listState === 'empty') return empty('clock',t('从一份资源开始','Start with a resource'),t('打开过的文档、图片与链接，会回到这里。','Documents, images and links you open will appear here.'),'browse-local',t('浏览本机','Browse this device'));
    if (state.section === 'remote' && !state.online) return `<div class="crumbs"><button data-folder="remote-root">${t('远程电脑','Computers')}</button>${icon('arrow')}<button>${t('工作电脑','Work PC')}</button></div>`+empty('pc',t('工作电脑已离线','Work PC is offline'),t('工作区位置已保留。电脑上线后即可继续浏览。','Your workspace location is kept. Reconnect to continue.'),'reconnect',t('重新连接','Reconnect'));
    if (state.query) {
      const results = resources.filter(r => !state.removed.has(r.id) && (state.section !== 'local' || r.source !== 'pc') && (state.section !== 'remote' || r.source === 'pc') && `${nameOf(r)} ${locOf(r)}`.toLowerCase().includes(state.query.toLowerCase()));
      return `<div class="section-label">${t('搜索结果','Search results')}<span>${results.length}</span></div>`+(results.length ? results.map(row).join('') : empty('search',t('没有找到相关资源','No matching resources'),t('试试名称或当前范围内的位置。','Try another name or location in this scope.'),'clear-search',t('清除搜索','Clear search')));
    }
    if (state.section === 'recent') {
      const groups = [['today',t('今天','Today')],['yesterday',t('昨天','Yesterday')],['week',t('近 7 天','Last 7 days')]];
      return groups.map(([id,label]) => {
        const items = resources.filter(r => r.group === id && !state.removed.has(r.id));
        return items.length ? `<div class="section-label">${label}</div>${items.map(row).join('')}` : '';
      }).join('') || empty('clock',t('还没有最近记录','No recent resources'),t('成功打开的资源会显示在这里。','Resources will appear after you open them.'),'browse-local',t('浏览本机','Browse this device'));
    }
    if (state.section === 'local') {
      if (!state.localDepth) return `<div class="section-label">${t('授权的位置','Available locations')}</div><div class="workspace-card">${folderRow('local-docs',t('文档','Documents'),t('OpenMuse 本机资源','OpenMuse on this device'))}${folderRow('local-imports',t('导入的资源','Imported resources'),t('已保存到本机的副本','Copies saved on this device'))}</div><p class="source-note">${t('只显示 OpenMuse 管理及你明确授权的资源。','Only OpenMuse resources and locations you allow.')}</p><div class="section-label">${t('文档中的资源','In Documents')}</div>${resources.filter(r=>r.source==='local'&&r.kind!=='video').map(row).join('')}`;
      const localTitle=state.localDepth===2?t('导入的资源','Imported resources'):state.localDepth===3?state.createdFolder:t('文档','Documents');
      const localItems=state.localDepth===3?[]:resources.filter(r=>r.source==='local'&&(state.localDepth===2?r.location==='导入的资源':r.location!=='导入的资源'));
      return `<div class="crumbs"><button data-folder="local-root">${t('本机','This device')}</button>${icon('arrow')}<button data-folder="local-docs">${t('文档','Documents')}</button>${state.localDepth!==1?icon('arrow')+`<button>${esc(localTitle)}</button>`:''}</div>${state.localDepth===1&&state.createdFolder?folderRow('local-created',state.createdFolder,t('空文件夹','Empty folder')):''}${localItems.length?localItems.map(row).join(''):empty('folder',t('这里还没有资源','This folder is empty'),t('可以新建资源，或导入已有文件。','Create a resource or import a file.'),'new',t('新建资源','Create resource'))}`;
    }
    if (!state.remoteDepth) return `<div class="workspace-head">${icon('pc')}<strong>${t('工作电脑','Work PC')}</strong><span class="connection-state">${t('在线','Online')}</span></div><div class="workspace-card">${folderRow('remote-workspace',t('产品设计','Product design'),t('2 个资源位置 · 已授权','2 locations · Access allowed'))}${folderRow('remote-notes',t('个人笔记','Personal notes'),t('1 个资源位置 · 只读','1 location · Read only'))}</div><div class="workspace-head">${icon('pc')}<strong>${t('家用电脑','Home PC')}</strong><span class="connection-state">${t('离线','Offline')}</span></div><div class="workspace-card">${folderRow('offline',t('个人文件','Personal files'),t('上次连接的工作区','Previously connected workspace'))}</div><p class="source-note">${t('文件按电脑和工作区归属展示，不混合不同位置的目录。','Resources stay grouped by computer and workspace.')}</p>`;
    const isNotes=state.remoteWorkspace==='notes';const isShared=state.remoteMount==='shared';
    const crumbs = `<div class="crumbs"><button data-folder="remote-root">${t('工作电脑','Work PC')}</button>${icon('arrow')}<button data-folder="${isNotes?'remote-notes':'remote-workspace'}">${isNotes?t('个人笔记','Personal notes'):t('产品设计','Product design')}</button>${state.remoteDepth>1 ? icon('arrow')+`<button data-folder="remote-mount">${isNotes?t('笔记','Notes'):isShared?t('共享素材','Shared assets'):t('创作资料','Creative files')}</button>` : ''}${state.remoteDepth>2 ? icon('arrow')+`<button>${t('资料','Materials')}</button>` : ''}</div>`;
    if (state.remoteDepth===1) return crumbs+`<div class="section-label">${t('工作区资源位置','Workspace locations')}</div><div class="workspace-card">${folderRow('remote-mount',isNotes?t('笔记','Notes'):t('创作资料','Creative files'),t('工作区资源 · 只读','Workspace resources · Read only'))}${!isNotes?folderRow('remote-shared',t('共享素材','Shared assets'),t('工作区资源 · 只读','Workspace resources · Read only')):''}</div>`;
    if (state.remoteDepth===2) return crumbs+`<div class="section-label">${t('创作资料','Creative files')}<span>${t('只读','Read only')}</span></div>${folderRow('remote-materials',t('资料','Materials'),t('提案与评审文档','Proposals and review notes'))}${folderRow('remote-materials',t('品牌素材','Brand assets'),t('图片与设计文件','Images and design files'))}`;
    return crumbs+`<div class="section-label">${t('资料','Materials')}<span>${t('只读','Read only')}</span></div>${isNotes?empty('folder',t('这里还没有资源','This folder is empty'),t('此工作区仅开放读取。','This workspace is read only.'),'browse-remote',t('返回工作区','Back to workspace')):resources.filter(r=>r.source==='pc'&&(r.mount||'creative')===state.remoteMount).map(row).join('')}`;
  }
  function renderHome() {
    return `<section class="screen"><header class="page-head"><h1 class="page-title">${t('首页','Home')}</h1>${button('focus-search',t('搜索资源','Search resources'),'search')}</header><div class="subtabs" role="tablist" aria-label="${t('资源来源','Resource sources')}">${[['recent','最近','Recent'],['local','本机','This Device'],['remote','远程（PC）','Remote (PC)']].map(([id,zh,en])=>`<button role="tab" aria-selected="${state.section===id}" class="${state.section===id?'active':''}" data-section="${id}">${t(zh,en)}</button>`).join('')}</div><div class="scroll-area" id="resource-scroll">${search()}<div id="resource-content">${homeContent()}</div></div><button class="fab" data-action="new" aria-label="${t('新建资源','Create resource')}">${icon('plus')}</button></section>`;
  }
  function chip(r, removable = false) {
    return `<div class="resource-chip"><button class="chip-main" data-open="${r.id}">${icon(r.kind)}<span class="chip-copy"><strong>${esc(nameOf(r))}</strong><small>${esc(metaOf(r))}</small></span>${icon('link','small-chevron')}</button>${removable ? `<button class="icon-button" data-detach="${r.id}" aria-label="${esc(t('移除引用：','Remove reference: ')+nameOf(r))}">${icon('close')}</button>` : ''}</div>`;
  }
  function renderChat() {
    return `<section class="screen"><header class="page-head"><h1 class="page-title">${t('任务','Tasks')}</h1>${button('new-task',t('新对话','New conversation'),'plus')}</header><div class="chat-scroll"><div class="chat-context">${icon('task')}${t('当前对话 · 草稿会保留','Current conversation · Draft kept')}</div>${state.messages.length ? state.messages.map(m=>`<div class="message">${m.text ? `<div>${esc(m.text)}</div>` : ''}${m.resources.map(chip).join('')}</div>`).join('') : `<div class="chat-intro">${icon('link')}<h2>${t('围绕资源，<br>继续对话。','Bring a resource.<br>Continue the conversation.')}</h2><p>${t('把文件或链接粘贴到草稿中，<br>确认后再发送。','Paste a file or link into your draft.<br>Send when you’re ready.')}</p></div>`}</div><div class="composer-wrap"><div id="draft-resources">${state.draft.map(r=>chip(r,true)).join('')}</div><textarea id="composer" aria-label="${t('对话草稿','Message draft')}" placeholder="${t('输入你的想法…','What would you like to do?')}">${esc(state.text)}</textarea><div class="composer-tools"><button class="paste-button" data-action="paste" ${state.clipboard ? '' : 'disabled'}>${icon('paste')}${t('粘贴资源','Paste resource')}</button><button class="send-button" data-action="send" aria-label="${t('发送消息','Send message')}">${icon('up')}</button></div></div></section>`;
  }
  function renderEditor() {
    const r = state.editor.resource;
    return `<section class="screen"><header class="page-head">${button('close-editor',t('返回','Back'),'back')}<span class="resource-name">${esc(nameOf(r))}</span>${button('editor-menu',t('更多','More'),'more')}</header><div class="editor"><div class="editor-top"><span>${esc(state.editor.handler)}</span><span class="read-only">${t('查看','View')}</span></div><article class="document-sheet"><p class="doc-eyebrow">${r.kind==='slides'?'OPENMUSE / LAUNCH':t('资源预览','RESOURCE PREVIEW')}</p>${['image','video','link'].includes(r.kind) ? `<div class="preview-art">${icon(r.kind)}</div>` : ''}<h2>${esc(t(r.title||r.name,r.titleEn||r.nameEn))}</h2><p>${esc(t(r.text||'每份资源保留自己的来源，也能在新的上下文中继续使用。',r.textEn||'Every resource keeps its source, ready to use in a new context.'))}</p><div class="document-rule"></div><h3>${t('位置','Location')}</h3><p>${esc(metaOf(r))}</p><p>${t('文件与对话之间，保留同一个引用。','Keep the same reference between a file and a conversation.')}</p></article></div></section>`;
  }
  function renderOther() {
    if (state.tab==='discover') return `<section class="screen"><header class="page-head"><h1 class="page-title">${t('发现','Discover')}</h1></header><div class="subtabs"><button class="active">${t('发现','Discover')}</button><button data-action="installed-plugins">${t('我的插件','My Plugins')}</button></div><div class="scroll-area"><div class="section-label">${t('资源工具','Resource tools')}</div><div class="plugin-row">${fileIcon('note')}<div class="plugin-info"><h3>Note Lab</h3><p>${t('笔记与 Markdown · 打开和创建','Notes & Markdown · Open and create')}</p></div><button data-action="toggle-notes">${state.notes?t('已安装','Installed'):t('安装','Install')}</button></div><div class="plugin-row">${fileIcon('slides')}<div class="plugin-info"><h3>GenOffice</h3><p>${t('演示文稿','Presentations')}</p></div><button data-action="plugin-info">${t('已安装','Installed')}</button></div></div></section>`;
    if (state.tab==='account') return `<section class="screen"><header class="page-head"><h1 class="page-title">${t('我的','Account')}</h1></header><div class="scroll-area"><div class="settings-group"><button class="menu-action" data-action="theme">${icon('compass')}${t('主题','Appearance')}<small>${state.theme==='dark'?t('深色','Dark'):t('浅色','Light')}</small></button><button class="menu-action" data-action="language">${icon('globe')}${t('语言','Language')}<small>${t('中文','English')}</small></button></div><div class="settings-group"><button class="menu-action" data-action="about">${icon('info')}${t('关于 OpenMuse','About OpenMuse')}</button></div></div></section>`;
    return `<section class="screen"><header class="page-head"><h1 class="page-title">${t('工作台','Workbench')}</h1></header><div class="scroll-area"><div class="prototype-card"><h3>${t('工作区文件','Workspace files')}</h3><p>${t('浏览已授权电脑的工作区与目录。','Browse workspaces on an authorized computer.')}</p><button class="outline-button" data-action="browse-remote">${icon('folder')}${t('浏览资源','Browse resources')}</button></div>${state.notes?`<div class="prototype-card"><h3>Note Lab</h3><p>${t('从一份笔记开始。','Start with a note.')}</p><button class="outline-button" data-creator="note">${icon('plus')}${t('新建笔记','New note')}</button></div>`:''}</div></section>`;
  }
  function render() {
    const phone = $('#phone');phone.classList.toggle('dark',state.theme==='dark');phone.lang=state.lang==='en'?'en':'zh-CN';
    $('#app').innerHTML=(state.editor ? renderEditor() : state.tab==='home' ? renderHome() : state.tab==='tasks' ? renderChat() : renderOther())+nav();
    bind($('#app'));
    $$('#theme-controls button').forEach(b=>b.setAttribute('aria-pressed',b.dataset.theme===state.theme));
    $$('#language-controls button').forEach(b=>b.setAttribute('aria-pressed',b.dataset.lang===state.lang));
    $('#notes-plugin').checked=state.notes;$('#pc-online').checked=state.online;$('#list-state').value=state.listState;
    if (state.overlay) renderSheet();
  }
  function opened(r,handler) {
    state.overlay=null;state.editor={resource:r,handler};state.removed.delete(r.id);
    resources=resources.filter(x=>x.id!==r.id);r.group='today';resources.unshift(r);render();closeSheet(false);
  }
  function openResource(id,force=false) {
    const r=resources.find(x=>x.id===id);if(!r)return;
    if(r.source==='pc'&&!state.online){showSheet('unavailable',r);return;}
    const handlers=availableHandlers(r);
    if(!handlers.length){showSheet('no-handler',r);return;}
    const preference=state.defaults[r.kind];
    if(!force&&(handlers.length===1||handlers.includes(preference))){opened(r,preference||handlers[0]);return;}
    state.selectedHandler=handlers[0];showSheet('handlers',r);
  }
  function showSheet(type,resource=null,extra={}) {
    if(!state.overlay){
      sheetFocus=document.activeElement;
      const attr=['data-action','data-menu','data-open','data-scene','data-creator'].find(k=>sheetFocus?.hasAttribute(k));
      sheetReturnSelector=attr?`[${attr}="${CSS.escape(sheetFocus.getAttribute(attr))}"]`:null;
    }
    state.overlay={type,resource,...extra};renderSheet();
    requestAnimationFrame(()=>$('#overlay-root [data-action="close-sheet"]')?.focus({preventScroll:true}));
  }
  function closeSheet(renderAgain=true) {
    if(state.creating){clearTimeout(createTimer);state.creating=false;}
    state.overlay=null;$('#overlay-root').replaceChildren();
    $('#app').inert=false;
    if(renderAgain)render();
    const restore=sheetFocus?.isConnected?sheetFocus:sheetReturnSelector?$(sheetReturnSelector):$('#app button');
    restore?.focus({preventScroll:true});
  }
  function actionRow(id,label,glyph,detail='',disabled=false) {
    return `<button class="menu-action" data-action="${id}" ${disabled?'disabled':''}>${icon(glyph)}<span>${label}</span>${detail?`<small>${detail}</small>`:''}</button>`;
  }
  function currentCreators() {return fixture.creators.filter(c=>state.notes||c.plugin!=='Note Lab');}
  function renderSheet() {
    const o=state.overlay;if(!o)return;const r=o.resource;
    let title='',body='';
    const summary=r?`<div class="selected-resource">${fileIcon(r.kind)}<span class="resource-labels"><span class="resource-name">${esc(nameOf(r))}</span><span class="resource-meta">${esc(metaOf(r))}</span></span></div>`:'';
    if(o.type==='menu') {
      title=t('资源操作','Resource actions');
      body=summary+actionRow('open-selected',t('打开','Open'),'file')+actionRow('open-with',t('打开方式…','Open with…'),'workbench')+actionRow('add-to-chat',t('添加到对话…','Add to conversation…'),'task')+actionRow('copy-resource',t('复制资源','Copy resource'),'copy')+actionRow('copy-path',t('复制资源路径','Copy resource path'),'link')+actionRow('rename',t('重命名','Rename'),'pencil',!r.writable?t('此位置只读','Read-only location'):'',!r.writable)+(r.source==='pc'?actionRow('save-copy',t('另存副本到本机…','Save a copy to this device…'),'download'): '')+(state.section==='recent'?actionRow('remove-recent',t('从最近移除','Remove from Recent'),'clock'):'');
    } else if(o.type==='handlers') {
      title=t('打开方式','Open with');
      body=summary+availableHandlers(r).map(h=>`<button class="handler-row" data-handler="${esc(h)}"><span class="handler-icon">${icon(h==='GenOffice'?'slides':'file')}</span><span class="handler-info"><strong>${esc(h)}</strong><small>${t('本机可用 · 查看','On this device · View')}</small></span>${state.selectedHandler===h?icon('check'):''}</button>`).join('')+`<label class="choice-box"><input id="remember-handler" type="checkbox">${t('始终使用此方式打开同类资源','Always use for this resource type')}</label><button class="primary-button" data-action="confirm-handler">${t('打开','Open')}</button>`;
    } else if(o.type==='create') {
      title=t('新建','Create');
      body=(state.section==='remote'?`<div class="notice">${icon('info')}<span>${t('当前远程目录只读，新资源将保存到本机文档。','This remote location is read-only. New resources will be saved in Documents on this device.')}</span></div>`:'')+`<button class="destination" data-action="destination">${icon('folder')}<span class="dest-text"><small>${t('保存到','Save in')}</small><span>${t('本机 · 文档','This device · Documents')}</span></span>${icon('arrow','small-chevron')}</button><div class="section-label">${t('创建资源','Create a resource')}</div><div class="creator-grid">${currentCreators().map(c=>`<button class="creator-tile" data-creator="${c.id}">${icon(c.kind)}<strong>${esc(t(c.name,c.nameEn))}</strong><small>${esc(c.plugin)}</small></button>`).join('')}</div><div class="section-label">${t('导入与整理','Import & organize')}</div>${actionRow('import',t('导入文件','Import a file'),'upload')}${actionRow('new-folder',t('新建文件夹','New folder'),'folder')}`;
    } else if(o.type==='create-form'||o.type==='rename') {
      const c=o.creator;title=o.type==='rename'?t('重命名','Rename'):t(c.name,c.nameEn);
      const val=o.value??(r?nameOf(r):t(c.defaultName,c.defaultNameEn));
      body=`<p class="helper">${o.type==='rename'?esc(metaOf(r)):esc(c.plugin)+' · '+t('本机 / 文档','This device / Documents')}</p><label class="form-label" for="resource-name">${t('名称','Name')}</label><input class="form-input" id="resource-name" maxlength="120" value="${esc(val)}" autocomplete="off">${c?.kind==='link'?`<label class="form-label" for="resource-url">${t('网址','Web address')}</label><input class="form-input" id="resource-url" type="url" placeholder="https://" value="${esc(o.url||'')}">`:''}<div class="form-error" id="form-error" role="alert">${esc(o.error||'')}</div><div class="form-actions"><button class="primary-button" data-action="${o.type==='rename'?'confirm-rename':'confirm-create'}" ${state.creating?'disabled':''}>${state.creating?t('正在创建…','Creating…'):o.type==='rename'?t('保存名称','Save name'):t('创建并打开','Create & open')}</button></div>`;
    } else if(o.type==='destination') {
      title=t('保存位置','Save location');body=actionRow('choose-local',t('本机 · 文档','This device · Documents'),'phone',t('可写','Writable'))+actionRow('choose-remote',t('工作电脑 · 产品设计','Work PC · Product design'),'pc',t('只读','Read only'),true);
    } else if(o.type==='chat-target') {
      title=t('添加到对话','Add to conversation');body=summary+actionRow('attach-current',t('当前对话','Current conversation'),'task',t('保留现有草稿','Keep current draft'))+actionRow('attach-new',t('新对话','New conversation'),'plus')+`<p class="helper">${t('仅加入草稿，不会自动发送或读取资源内容。','Adds a reference to your draft. Nothing is sent or read automatically.')}</p>`;
    } else if(o.type==='no-handler') {
      title=t('还没有打开方式','No compatible app');body=summary+`<p class="helper">${t('本机尚未安装能打开此类资源的插件。资源仍可复制或添加到草稿。','No compatible plugin is installed on this device. You can still copy this resource or add it to a draft.')}</p><div class="form-actions"><button class="primary-button" data-action="find-plugin">${t('查看兼容插件','Find compatible plugins')}</button><button class="outline-button" data-action="copy-resource">${t('复制资源','Copy resource')}</button></div>`;
    } else if(o.type==='unavailable') {
      title=t('电脑离线','Computer offline');body=summary+`<p class="helper">${t('连接工作电脑后再打开，当前列表位置会保留。','Reconnect to Work PC to open this resource. Your list position is kept.')}</p><button class="primary-button" data-action="reconnect">${t('重新连接','Reconnect')}</button>`;
    } else if(o.type==='filters') {
      title=t('筛选资源','Filter resources');body=`<p class="helper">${t('仅筛选当前列表，不跨电脑搜索全文。','Filters this list, without searching other computers.')}</p>${[['all','全部资源','All resources'],['file','文档','Documents'],['image','图片','Images'],['link','网址','Web links'],['video','视频','Videos']].map(([k,zh,en])=>actionRow('filter-'+k,t(zh,en),k==='all'?'folder':k)).join('')}`;
    } else if(o.type==='import') {
      title=t('导入文件','Import a file');body=`<p class="helper">${t('选中的文件将作为副本保存到本机文档。','The selected file will be saved as a copy in Documents.')}</p>${actionRow('import-sample',t('品牌素材清单.md','Brand asset list.md'),'file',t('系统文件','Files'))}`;
    } else if(o.type==='folder') {
      title=t('新建文件夹','New folder');body=`<p class="helper">${t('本机 / 文档','This device / Documents')}</p><label class="form-label" for="resource-name">${t('文件夹名称','Folder name')}</label><input class="form-input" id="resource-name" placeholder="${t('未命名文件夹','Untitled folder')}"><div class="form-error" id="form-error"></div><div class="form-actions"><button class="primary-button" data-action="confirm-folder">${t('创建','Create')}</button></div>`;
    }
    $('#app').inert=true;
    $('#overlay-root').innerHTML=`<div class="scrim"><section class="sheet" role="dialog" aria-modal="true" aria-labelledby="sheet-title"><div class="sheet-handle" aria-hidden="true"></div><header class="sheet-head"><h2 id="sheet-title">${title}</h2>${button('close-sheet',t('关闭面板','Close sheet'),'close')}</header><div class="sheet-body">${body}</div></section></div>`;
    bind($('#overlay-root'));
    $('.scrim').addEventListener('click',e=>{if(e.target===e.currentTarget)closeSheet();});
  }
  function toast(text,actionLabel,action) {
    clearTimeout(timer);
    $('#toast-root').innerHTML=`<div class="toast" role="status"><span>${esc(text)}</span>${action?`<button id="toast-action">${esc(actionLabel)}</button>`:''}</div>`;
    $('#toast-action')?.addEventListener('click',()=>{$('#toast-root').replaceChildren();action();});
    timer=setTimeout(()=>$('#toast-root').replaceChildren(),4500);
  }
  function copyResource(r) {
    state.clipboard={resourceId:r.id,resourceRef:r.ref};closeSheet();toast(t('已复制资源引用','Resource reference copied'),t('去粘贴','Paste in Tasks'),()=>{state.tab='tasks';state.editor=null;render();});
  }
  function attach(r) {
    if(state.draft.some(x=>x.ref===r.ref)){toast(t('草稿中已有此资源','Already in this draft'));return;}
    if(state.draft.length>=10){toast(t('一次最多添加 10 项资源','Add up to 10 resources'));return;}
    state.draft.push(r);
  }
  function validName(value,id) {
    if(!value.trim())return t('请输入名称','Enter a name');
    if(/[\\/\x00-\x1f]/.test(value)||value==='.'||value==='..')return t('名称中不能含路径分隔符或控制字符','Use a name without path separators or control characters');
    if(resources.some(r=>r.id!==id&&r.source==='local'&&nameOf(r).toLowerCase()===value.toLowerCase()))return t('此位置已有同名资源，请换一个名称','This name is already used in this location');
    return '';
  }
  function safeWebAddress(value){try{const u=new URL(value);return ['http:','https:'].includes(u.protocol)&&Boolean(u.hostname);}catch{return false;}}
  function action(id) {
    const o=state.overlay,r=o?.resource;
    if(id==='close-sheet'){closeSheet();return;}
    if(id==='new'){state.target='local';showSheet('create');setNotes('create');return;}
    if(id==='focus-search'){$('#search-input')?.focus();return;}
    if(id==='filters'){showSheet('filters');return;}
    if(id==='retry'){state.listState='ready';render();return;}
    if(id==='clear-search'){state.query='';render();return;}
    if(id==='browse-local'||id==='browse-remote'){state.tab='home';state.editor=null;state.section=id==='browse-local'?'local':'remote';state.listState='ready';render();return;}
    if(id==='reconnect'){state.online=true;state.listState='ready';closeSheet();toast(t('已重新连接工作电脑','Work PC reconnected'));return;}
    if(id==='open-selected'){closeSheet();openResource(r.id);return;}
    if(id==='open-with'){state.selectedHandler=availableHandlers(r)[0];showSheet(availableHandlers(r).length?'handlers':'no-handler',r);return;}
    if(id==='confirm-handler'){if($('#remember-handler')?.checked)state.defaults[r.kind]=state.selectedHandler;opened(r,state.selectedHandler);return;}
    if(id==='copy-resource'){copyResource(r);return;}
    if(id==='copy-path'){state.clipboard=null;state.plainTextClipboard=metaOf(r)+' / '+nameOf(r);closeSheet();toast(t('已复制逻辑资源路径','Logical resource path copied'));return;}
    if(id==='add-to-chat'){showSheet('chat-target',r);return;}
    if(id==='attach-current'||id==='attach-new'){if(id==='attach-new'){state.draft=[];state.text='';state.messages=[];}attach(r);state.tab='tasks';state.editor=null;closeSheet();setNotes('chat');return;}
    if(id==='paste'){if(!state.clipboard)return;const resource=resources.find(x=>x.id===state.clipboard.resourceId);if(resource)attach(resource);render();return;}
    if(id==='send'){if(!state.text.trim()&&!state.draft.length){toast(t('先输入内容或添加资源','Add text or a resource first'));return;}state.messages.push({text:state.text,resources:[...state.draft]});state.text='';state.draft=[];render();toast(t('引用已加入消息；原型未调用模型','Reference added to the message; no model was called'));return;}
    if(id==='new-task'){state.text='';state.draft=[];state.messages=[];render();return;}
    if(id==='rename'){if(r.writable)showSheet('rename',r);return;}
    if(id==='confirm-rename'){const value=$('#resource-name').value.trim(),extension=nameOf(r).match(/\.[a-z0-9]{1,10}$/i)?.[0],error=validName(value,r.id)||(extension&&!value.toLowerCase().endsWith(extension.toLowerCase())?t('请保留原文件扩展名','Keep the original file extension'):'');if(error){$('#form-error').textContent=error;return;}r.name=value;r.nameEn=value;closeSheet();toast(t('名称已更新','Name updated'));return;}
    if(id==='remove-recent'){state.removed.add(r.id);closeSheet();toast(t('已从最近移除，源文件保留','Removed from Recent; the original is kept'),t('撤销','Undo'),()=>{state.removed.delete(r.id);render();});return;}
    if(id==='save-copy'){const copy={...r,id:'copy-'+Date.now(),ref:'demo-copy-'+Date.now(),source:'local',writable:true,location:'文档',locationEn:'Documents'};resources.unshift(copy);closeSheet();toast(t('副本已保存到本机文档','Copy saved in Documents'));return;}
    if(id==='destination'){showSheet('destination');return;}
    if(id==='choose-local'){showSheet('create');return;}
    if(id==='new-folder'){showSheet('folder');return;}
    if(id==='confirm-folder'){const v=$('#resource-name').value.trim(),err=validName(v);if(err){$('#form-error').textContent=err;return;}state.createdFolder=v;state.tab='home';state.section='local';state.localDepth=1;closeSheet();toast(t('文件夹已创建','Folder created'));return;}
    if(id==='import'){showSheet('import');return;}
    if(id==='import-sample'){const imported={...fixture.resources[5],id:'import-'+Date.now(),ref:'demo-import-'+Date.now(),name:'品牌素材清单.md',nameEn:'Brand asset list.md',group:'today'};resources.unshift(imported);closeSheet();state.section='local';state.tab='home';state.editor=null;render();toast(t('已保存本机副本','Saved a copy on this device'));return;}
    if(id==='confirm-create'){const c=o.creator,value=$('#resource-name').value.trim(),url=$('#resource-url')?.value||'',error=validName(value)||(c.kind==='link'&&!safeWebAddress(url)?t('请输入 http 或 https 网址','Enter an http or https address'):'');if(error){$('#form-error').textContent=error;return;}state.creating=true;state.overlay.value=value;state.overlay.url=url;renderSheet();createTimer=setTimeout(()=>{if(!currentCreators().some(x=>x.id===c.id)){state.creating=false;closeSheet();toast(t('创建插件已不可用，未创建资源','Creator unavailable; no resource was created'));return;}const created={id:'new-'+Date.now(),ref:'demo-new-'+Date.now(),name:value,nameEn:value,kind:c.kind,source:'local',location:'文档',locationEn:'Documents',size:'1 KB',group:'today',writable:true,handlers:[c.plugin],title:value,titleEn:value,text:'',textEn:''};resources.unshift(created);state.creating=false;opened(created,c.plugin);toast(t('资源已创建并打开','Created and opened'));},650);return;}
    if(id==='find-plugin'){state.tab='discover';state.editor=null;closeSheet();toast(t('原型未预设视频兼容插件，可返回复制引用','No video handler is assumed in this prototype'));return;}
    if(id==='toggle-notes'){toggleNotes(!state.notes);toast(state.notes?t('已安装：新建类型已刷新','Installed: create actions updated'):t('已停用：相关能力已移除','Disabled: related actions removed'));return;}
    if(id==='installed-plugins'){toast(t('列表显示本稿的示例插件','Showing the prototype’s example plugins'));return;}
    if(id==='plugin-info'){toast(t('创建贡献是本稿提议，非当前插件能力承诺','Creation contribution is proposed, not a current plugin claim'));return;}
    if(id==='close-editor'){state.editor=null;render();return;}
    if(id==='editor-menu'){showSheet('menu',state.editor.resource);return;}
    if(id==='theme'){state.theme=state.theme==='light'?'dark':'light';render();return;}
    if(id==='language'){state.lang=state.lang==='zh'?'en':'zh';render();return;}
    if(id==='about'){toast(t('本稿只新增 Home 与资源联动，其余设置沿用当前 App','This proposal adds Home; other settings follow the current app'));return;}
    if(id.startsWith('filter-')){const kind=id.slice(7);closeSheet();if(kind!=='all'){const list=resources.filter(r=>r.kind===kind&&!state.removed.has(r.id)&&(state.section!=='local'||r.source!=='pc')&&(state.section!=='remote'||r.source==='pc'));$('#resource-content').innerHTML=`<div class="section-label">${t('筛选结果','Filtered resources')}</div>`+list.map(row).join('');bind($('#resource-content'));}else render();return;}
  }
  function bind(root) {
    $$('[data-action]',root).forEach(b=>b.addEventListener('click',()=>action(b.dataset.action)));
    $$('[data-nav]',root).forEach(b=>b.addEventListener('click',()=>{state.tab=b.dataset.nav;state.editor=null;state.overlay=null;render();if(state.tab==='tasks')setNotes('chat');}));
    $$('[data-section]',root).forEach(b=>b.addEventListener('click',()=>{state.section=b.dataset.section;state.query='';state.listState='ready';render();setNotes(state.section);}));
    $$('[data-open]',root).forEach(b=>b.addEventListener('click',()=>{if(b.dataset.longpress==='yes'){delete b.dataset.longpress;return;}openResource(b.dataset.open);}));
    $$('[data-menu]',root).forEach(b=>b.addEventListener('click',()=>{showSheet('menu',resources.find(r=>r.id===b.dataset.menu));setNotes('menu');}));
    $$('[data-open]',root).forEach(b=>{let hold,start;const cancel=()=>clearTimeout(hold);b.addEventListener('pointerdown',e=>{if(e.button!==0)return;start={x:e.clientX,y:e.clientY};hold=setTimeout(()=>{b.dataset.longpress='yes';showSheet('menu',resources.find(r=>r.id===b.dataset.open));setNotes('menu');},450);});b.addEventListener('pointermove',e=>{if(start&&Math.hypot(e.clientX-start.x,e.clientY-start.y)>10)cancel();});['pointerup','pointercancel','pointerleave'].forEach(ev=>b.addEventListener(ev,cancel));b.addEventListener('contextmenu',e=>{e.preventDefault();cancel();showSheet('menu',resources.find(r=>r.id===b.dataset.open));setNotes('menu');});});
    $$('[data-detach]',root).forEach(b=>b.addEventListener('click',()=>{state.draft=state.draft.filter(r=>r.id!==b.dataset.detach);render();}));
    $$('[data-handler]',root).forEach(b=>b.addEventListener('click',()=>{const remember=$('#remember-handler')?.checked;state.selectedHandler=b.dataset.handler;renderSheet();if($('#remember-handler'))$('#remember-handler').checked=remember;}));
    $$('[data-creator]',root).forEach(b=>b.addEventListener('click',()=>{const c=currentCreators().find(x=>x.id===b.dataset.creator);if(c)showSheet('create-form',null,{creator:c});}));
    $$('[data-folder]',root).forEach(b=>b.addEventListener('click',()=>{const id=b.dataset.folder;if(id==='offline'){state.online=false;}else if(id.startsWith('local')){state.section='local';state.localDepth=id==='local-root'?0:id==='local-imports'?2:id==='local-created'?3:1;}else{state.section='remote';if(id==='remote-workspace'){state.remoteWorkspace='product';state.remoteMount='creative';}if(id==='remote-notes'){state.remoteWorkspace='notes';state.remoteMount='creative';}if(id==='remote-shared')state.remoteMount='shared';if(id==='remote-mount'&&state.remoteDepth===1)state.remoteMount='creative';state.remoteDepth=id==='remote-root'?0:id==='remote-workspace'||id==='remote-notes'?1:id==='remote-mount'||id==='remote-shared'?2:3;}state.query='';render();}));
    $('#search-input',root)?.addEventListener('input',e=>{state.query=e.target.value;$('#resource-content').innerHTML=homeContent();bind($('#resource-content'));});
    $('#composer',root)?.addEventListener('input',e=>{state.text=e.target.value;});
  }
  function toggleNotes(value) {
    state.notes=value;
    if(!value&&state.overlay?.creator?.plugin==='Note Lab'){closeSheet();toast(t('插件已停用，创建表单已关闭','Plugin disabled; creation form closed'));}
    render();
  }
  const sceneNotes = {
    recent:['一个列表，接住所有来源。','最近按成功打开时间排序。名称下明确显示来源和逻辑位置，不让用户猜文件在哪里。','找到资源 → 本机插件打开 → 回到最近'],
    local:['本机，是这部手机。','只展示应用管理和明确授权的资源。导入保存副本，不把授权范围扩大为扫描整个手机。','本机 → 文档 → 点击或长按资源'],
    remote:['先工作区，再目录。','每台电脑下独立展示 Workspace，逐层进入 mount 与目录。浏览文件不切换任务执行位置。','工作电脑 → 产品设计 → 创作资料 → 资料'],
    create:['新建类型，由插件带来。','关闭右侧“笔记插件”，可立即看到新建入口减少；重新安装即可恢复，不改 Host 类型列表。','选择位置 → 创建类型 → 名称 → 创建并打开'],
    menu:['复制引用，不搬动源文件。','长按与更多按钮是同一菜单。远程只读资源不能重命名；移除最近只影响记录。','更多 → 复制资源 → Tasks → 粘贴资源'],
    chat:['像链接一样，把资源带过去。','草稿显示可点的引用卡片，来源可见、可以移除，不自动发送或读取。正式实现需要会话独立授权。','复制 / 添加到对话 → 草稿引用 → 明确发送']
  };
  function setNotes(scene) {
    state.lastScene=scene;const note=sceneNotes[scene]||sceneNotes.recent;
    $('#scene-title').textContent=note[0];$('#scene-note').textContent=note[1];$('#flow-copy').textContent=note[2];
    $$('.scenario-nav button').forEach(b=>b.classList.toggle('active',b.dataset.scene===scene));
  }
  function scene(id) {
    closeSheet(false);state.editor=null;state.tab='home';state.query='';state.listState='ready';state.section=['local','remote'].includes(id)?id:'recent';state.remoteDepth=0;state.localDepth=0;render();setNotes(id);
    if(id==='create')showSheet('create');
    if(id==='menu')showSheet('menu',resources.find(r=>r.id==='proposal'));
    if(id==='chat'){state.clipboard={resourceId:'proposal',resourceRef:'demo-resource-proposal'};state.draft=[resources.find(r=>r.id==='proposal')];state.text=t('帮我梳理这份提案的重点。','Summarize the key points in this proposal.');state.tab='tasks';render();}
  }
  document.addEventListener('keydown',e=>{
    if(e.key==='Escape'&&state.overlay){e.preventDefault();closeSheet();}
    if(e.key==='Tab'&&state.overlay){const items=$$('button:not(:disabled),input,select,textarea,[tabindex="0"]',$('#overlay-root')).filter(el=>el.getClientRects().length);if(!items.length)return;const first=items[0],last=items.at(-1);if(e.shiftKey&&document.activeElement===first){e.preventDefault();last.focus();}else if(!e.shiftKey&&document.activeElement===last){e.preventDefault();first.focus();}}
  });
  $$('#theme-controls button').forEach(b=>b.addEventListener('click',()=>{state.theme=b.dataset.theme;render();}));
  $$('#language-controls button').forEach(b=>b.addEventListener('click',()=>{state.lang=b.dataset.lang;render();}));
  $$('.scenario-nav button').forEach(b=>b.addEventListener('click',()=>scene(b.dataset.scene)));
  $('#notes-plugin').addEventListener('change',e=>toggleNotes(e.target.checked));
  $('#pc-online').addEventListener('change',e=>{state.online=e.target.checked;render();});
  $('#list-state').addEventListener('change',e=>{state.listState=e.target.value;state.editor=null;state.tab='home';closeSheet();});
  $('#reset-demo').addEventListener('click',()=>{clearTimeout(timer);reset();$('#toast-root').replaceChildren();closeSheet();scene('recent');});
  if(params.get('embed')==='1')document.body.classList.add('embed');
  render();scene(params.get('screen')||'recent');
  if(params.get('screen')==='remote-folder'){state.section='remote';state.remoteDepth=3;render();setNotes('remote');}
  if(params.get('screen')==='open-with'){state.selectedHandler='Note Lab';showSheet('handlers',resources.find(r=>r.id==='review'));setNotes('menu');}
  if(params.get('screen')==='offline'){state.section='remote';state.online=false;render();setNotes('remote');}
  if(params.get('screen')==='empty'){state.listState='empty';render();}
  if(params.get('screen')==='error'){state.listState='error';render();}
  // Deliberately limited review hooks for repeatable local screenshot checks.
  window.resourceCenterDemo = {scene,snapshot:()=>({tab:state.tab,section:state.section,notes:state.notes,online:state.online,overlay:state.overlay?.type||null,resourceCount:resources.length,draft:state.draft.map(r=>r.ref),clipboard:state.clipboard,defaults:{...state.defaults},messages:state.messages.length})};
})();
