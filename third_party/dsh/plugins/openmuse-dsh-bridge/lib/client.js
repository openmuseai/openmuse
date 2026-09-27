window.__ModuleLoader__.load({
  id: 'openmuse-dsh-bridge',
  factory: () => {
    const exports = {};
    exports.inject = ['sidebarRight', 'sessions', 'uiWorkspace', 'workspaces'];
    exports.apply = (ctx) => {
      const sidebar = ctx.sidebarRight;
      const original = sidebar.openResource;
      sidebar.openResource = function(address, options = {}) {
        const bridge = window.MuseHostResource;
        const prefix = 'dsh-resource://file/session/';
        if (!bridge || typeof address !== 'string' || !address.startsWith(prefix)) {
          return original.call(this, address, options);
        }
        const rest = address.slice(prefix.length);
        const slash = rest.indexOf('/');
        if (slash < 0) return;
        try {
          const sessionId = decodeURIComponent(rest.slice(0, slash));
          const path = rest.slice(slash + 1).split('/').map(decodeURIComponent).join('/');
          const cwd = ctx.sessions.list.getSnapshot().byId[sessionId]?.cwd;
          if (typeof cwd !== 'string' || !cwd || !path) return;
          bridge.postMessage(JSON.stringify({ type: 'resource.open', path, cwd, line: options?.params?.line }));
          return;
        } catch (_) { return; }
      };
      ctx.effect(() => () => { sidebar.openResource = original; }, 'openmuse-dsh-bridge: resource open');

      // The embedded WKWebView does not reliably put a web selection on the
      // Flutter window's responder-chain Copy action. Copy only actual selected
      // conversation text; leave editors and empty selections to WebKit/DSH.
      let selectionMenu = null;
      function selectedText() {
        const value = window.getSelection?.()?.toString() ?? '';
        return value.length <= 16384 ? value : '';
      }
      function editable(target) {
        return typeof target?.closest === 'function' && !!target.closest('input, textarea, [contenteditable="true"]');
      }
      function writeClipboard(value) {
        if (!value || !window.MuseHostClipboard) return;
        window.MuseHostClipboard.postMessage(JSON.stringify({ type: 'clipboard.write', text: value }));
      }
      function selectedPath(value) {
        const path = value.trim().replace(/^`+|`+$/g, '');
        if (!path || path.includes('\n') || path.includes('://')) return null;
        return /^(?:[./~]|[a-zA-Z]:[\\/])/.test(path)
          || /[/\\][^/\\]+\.[a-zA-Z0-9]{1,8}$/.test(path)
          || /^[^/\\\s]+\.[a-zA-Z0-9]{1,8}$/.test(path)
          ? path : null;
      }
      function closeSelectionMenu() {
        selectionMenu?.remove();
        selectionMenu = null;
      }
      function onPointerDown(event) {
        if (!selectionMenu?.contains(event.target)) closeSelectionMenu();
      }
      function onCopyKey(event) {
        if (event.key?.toLowerCase() !== 'c' || !(event.metaKey || event.ctrlKey) || editable(event.target)) return;
        const text = selectedText();
        if (!text || !window.MuseHostClipboard) return;
        writeClipboard(text);
        closeSelectionMenu();
        event.preventDefault();
        event.stopImmediatePropagation();
      }
      function onSelectionMenu(event) {
        if (editable(event.target) || !window.MuseHostClipboard) return;
        const text = selectedText();
        if (!text) return;
        event.preventDefault();
        event.stopImmediatePropagation();
        closeSelectionMenu();
        const menu = document.createElement('div');
        selectionMenu = menu;
        menu.setAttribute('role', 'menu');
        Object.assign(menu.style, {
          position: 'fixed', zIndex: '2147483647', minWidth: '144px',
          background: 'var(--dsw-specific-menu, #fff)', color: 'var(--dsw-alias-label-primary, #24272d)',
          border: '1px solid var(--dsw-alias-border-l1, #dadce2)', borderRadius: '10px',
          boxShadow: '0 8px 28px #0003', padding: '5px', fontSize: '13px',
          left: `${Math.min(event.clientX, window.innerWidth - 170)}px`,
          top: `${Math.min(event.clientY, window.innerHeight - 90)}px`,
        });
        function add(label, value) {
          const item = document.createElement('button');
          item.type = 'button';
          item.setAttribute('role', 'menuitem');
          item.textContent = label;
          Object.assign(item.style, {
            display: 'block', width: '100%', textAlign: 'left', border: '0',
            borderRadius: '6px', padding: '7px 10px', background: 'transparent',
            color: 'inherit', cursor: 'pointer', font: 'inherit',
          });
          item.addEventListener('mousedown', (e) => e.preventDefault());
          item.addEventListener('click', () => { writeClipboard(value); closeSelectionMenu(); });
          menu.appendChild(item);
        }
        add('复制选中内容', text);
        const path = selectedPath(text);
        if (path) add('复制路径', path);
        document.body.appendChild(menu);
      }
      if (window.addEventListener) {
        window.addEventListener('keydown', onCopyKey, true);
        window.addEventListener('contextmenu', onSelectionMenu, true);
        window.addEventListener('pointerdown', onPointerDown, true);
        window.addEventListener('scroll', closeSelectionMenu, true);
        ctx.effect(() => () => {
          window.removeEventListener('keydown', onCopyKey, true);
          window.removeEventListener('contextmenu', onSelectionMenu, true);
          window.removeEventListener('pointerdown', onPointerDown, true);
          window.removeEventListener('scroll', closeSelectionMenu, true);
          closeSelectionMenu();
        }, 'openmuse-dsh-bridge: selection copy');
      }

      const ui = ctx.uiWorkspace;
      const originalOpenWorkspace = ui.openWorkspace;
      const originalOpenSession = ui.openSession;
      function publishActiveSession() {
        const sessionId = ui.mainReference?.sessionId;
        const cwd = ctx.sessions.list.getSnapshot().byId[sessionId]?.cwd;
        if (typeof cwd !== 'string' || !cwd || !window.MuseHostWorkspace) return;
        const workspace = ctx.workspaces.list.getSnapshot().items.find((item) => item.path === cwd);
        if (workspace) {
          window.MuseHostWorkspace.postMessage(JSON.stringify({ type: 'workspace.activate', path: cwd }));
        }
      }
      ui.openWorkspace = async function(workspaceId, ...args) {
        await originalOpenWorkspace.call(this, workspaceId, ...args);
        publishActiveSession();
      };
      ui.openSession = function(...args) {
        const result = originalOpenSession.apply(this, args);
        publishActiveSession();
        return result;
      };
      let desiredPath = null;
      function activateDesired() {
        if (!desiredPath) return;
        const workspace = ctx.workspaces.list.getSnapshot().items.find((item) => item.path === desiredPath);
        if (!workspace) return;
        const current = ui.mainReference?.sessionId;
        if (ctx.sessions.list.getSnapshot().byId[current]?.cwd === desiredPath) {
          desiredPath = null;
          return;
        }
        desiredPath = null;
        Promise.resolve(ui.openWorkspace(workspace.workspaceId)).catch((error) => console.warn('OpenMuse workspace activation failed', error));
      }
      const bridgeApi = {
        activate(path) {
          if (typeof path !== 'string' || path.length > 4096) return;
          desiredPath = path;
          activateDesired();
        },
      };
      window.OpenMuseDshWorkspace = bridgeApi;
      const unsubscribe = ctx.workspaces.list.subscribe(activateDesired);
      if (typeof window.__OpenMuseDesiredWorkspace === 'string') bridgeApi.activate(window.__OpenMuseDesiredWorkspace);
      ctx.effect(() => () => {
        ui.openWorkspace = originalOpenWorkspace;
        ui.openSession = originalOpenSession;
        unsubscribe();
        if (window.OpenMuseDshWorkspace === bridgeApi) delete window.OpenMuseDshWorkspace;
      }, 'openmuse-dsh-bridge: workspace activation');
    };
    return exports;
  },
});
