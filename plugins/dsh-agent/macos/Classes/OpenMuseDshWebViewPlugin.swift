import Cocoa
import FlutterMacOS
import WebKit

public final class OpenMuseDshWebViewPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    registrar.register(DshWebViewFactory(messenger: registrar.messenger), withId: "com.openmuse.dsh/webview")
  }
}

final class DshWebViewFactory: NSObject, FlutterPlatformViewFactory {
  private let messenger: FlutterBinaryMessenger

  init(messenger: FlutterBinaryMessenger) {
    self.messenger = messenger
  }

  func create(withViewIdentifier viewId: Int64, arguments args: Any?) -> NSView {
    let configuration = WKWebViewConfiguration()
    configuration.websiteDataStore = .default()
    guard
      let values = args as? [String: Any],
      let raw = values["url"] as? String,
      let url = URL(string: raw),
      url.scheme == "http",
      url.host == "127.0.0.1" || url.host == "localhost",
      let port = url.port
    else {
      let webView = WKWebView(frame: .zero, configuration: configuration)
      webView.loadHTMLString("<p>Invalid DSH loopback URL</p>", baseURL: nil)
      return webView
    }
    let channel = FlutterMethodChannel(
      name: "com.openmuse.dsh/webview/\(viewId)",
      binaryMessenger: messenger
    )
    let handler = DshResourceMessageHandler(channel: channel, port: port, method: "resourceOpen")
    let workspaceHandler = DshResourceMessageHandler(channel: channel, port: port, method: "workspaceActivate")
    let clipboardHandler = DshClipboardMessageHandler(port: port)
    configuration.userContentController.add(handler, name: "MuseHostResource")
    configuration.userContentController.add(workspaceHandler, name: "MuseHostWorkspace")
    configuration.userContentController.add(clipboardHandler, name: "MuseHostClipboard")
    configuration.userContentController.addUserScript(WKUserScript(
      source: """
      window.MuseHostResource = { postMessage: function(raw) { window.webkit.messageHandlers.MuseHostResource.postMessage(raw); } };
      window.MuseHostWorkspace = { postMessage: function(raw) { window.webkit.messageHandlers.MuseHostWorkspace.postMessage(raw); } };
      window.MuseHostClipboard = { postMessage: function(raw) { window.webkit.messageHandlers.MuseHostClipboard.postMessage(raw); } };
      window.addEventListener('keydown', function(event) {
        if (!(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== 'c') return;
        if (typeof event.target?.closest === 'function' && event.target.closest('input, textarea, [contenteditable="true"]')) return;
        const selected = window.getSelection()?.toString() ?? '';
        if (!selected || selected.length > 16384) return;
        window.MuseHostClipboard.postMessage(JSON.stringify({ type: 'clipboard.write', text: selected }));
        event.preventDefault();
        event.stopImmediatePropagation();
      }, true);
      """,
      injectionTime: .atDocumentStart,
      forMainFrameOnly: true
    ))
    configuration.userContentController.addUserScript(WKUserScript(
      source: DshCompactSidebarStyle.userScript,
      injectionTime: .atDocumentStart,
      forMainFrameOnly: true
    ))
    if let path = values["activeMountPath"] as? String,
       let data = try? JSONSerialization.data(withJSONObject: [path]),
       let json = String(data: data, encoding: .utf8) {
      configuration.userContentController.addUserScript(WKUserScript(
        source: "window.__OpenMuseDesiredWorkspace = \(json)[0];",
        injectionTime: .atDocumentStart,
        forMainFrameOnly: true
      ))
    }
    let webView = WKWebView(frame: .zero, configuration: configuration)
    webView.setValue(false, forKey: "drawsBackground")
    channel.setMethodCallHandler { [weak webView] call, result in
      switch call.method {
      case "reload":
        webView?.reload()
        result(nil)
      case "activateWorkspace":
        guard let path = call.arguments as? String,
              let data = try? JSONSerialization.data(withJSONObject: [path]),
              let json = String(data: data, encoding: .utf8) else {
          result(FlutterError(code: "invalid_path", message: "Expected a Workspace path", details: nil))
          return
        }
        webView?.evaluateJavaScript("window.__OpenMuseDesiredWorkspace = \(json)[0]; window.OpenMuseDshWorkspace?.activate(window.__OpenMuseDesiredWorkspace);")
        result(nil)
      default:
        result(FlutterMethodNotImplemented)
      }
    }
    webView.load(URLRequest(url: url))
    return webView
  }

  func createArgsCodec() -> (FlutterMessageCodec & NSObjectProtocol)? {
    FlutterStandardMessageCodec.sharedInstance()
  }
}

/// Embedded Host chrome: DSH's icon rail is always a top strip, matching the
/// macOS / latest DSH layout. Class hashes are pinned to DSH 0.1.7-rc.1/rc.2
/// and must stay in sync with plugins/dsh-agent/windows/openmuse_dsh_plugin.cpp.
private enum DshCompactSidebarStyle {
  static let userScript = """
  (function() {
    const STYLE_ID = 'openmuse-dsh-compact-sidebar';
    const CSS = '.pI_x6G_frame{grid-template-rows:52px minmax(0,1fr)!important;grid-template-columns:minmax(0,1fr) 0 0!important}.pI_x6G_frame:not([data-rightbar-collapsed]){grid-template-columns:minmax(0,1fr) 0 minmax(0,45vw)!important}.pI_x6G_sidebarCol{grid-area:1/1/2/-1!important;border-right:none!important;border-bottom:0.5px solid var(--dsw-alias-border-l3,#ececf0);min-height:52px;max-height:52px;overflow:hidden}.pI_x6G_centerCol{grid-area:2/1/3/2!important}.pI_x6G_rightbarCol{grid-area:2/3/3/4!important}.hHd-Xa_root{flex-direction:row!important;align-items:center!important;padding:8px 40px 8px 10px!important;height:52px!important;max-height:52px!important;gap:4px}.hHd-Xa_topStrip,.hHd-Xa_regionArea,.hHd-Xa_brandName,.hHd-Xa_fallbackBrandName,.hHd-Xa_localBuildBrand,.hHd-Xa_newSessionLabel{display:none!important}.hHd-Xa_logoRow,.hHd-Xa_newSession{margin:0!important;height:36px!important;width:36px!important}.hHd-Xa_panelList,.hHd-Xa_footArea,.hHd-Xa_footerActions,.hHd-Xa_settingsArea{flex-direction:row!important;margin:0!important;width:auto!important;gap:4px!important}.hHd-Xa_footArea{margin-left:0!important;align-items:center!important;order:-1}.hHd-Xa_railIn .hHd-Xa_iconButton,.hHd-Xa_railIn .hHd-Xa_newSession,.hHd-Xa_railIn .hHd-Xa_panelList,.hHd-Xa_railIn .hHd-Xa_regionArea,.hHd-Xa_railIn .hHd-Xa_footArea{animation:none!important;transform:none!important}#openmuse-dsh-reload{flex:none;display:inline-flex;align-items:center;justify-content:center;width:36px;height:36px;margin:0;padding:0;border:none;border-radius:50%;background:transparent;color:inherit;cursor:pointer}#openmuse-dsh-reload:hover{background:var(--dsw-alias-interactive-bg-hover,rgba(0,0,0,.06))}';
    function ensureStyle() {
      let style = document.getElementById(STYLE_ID);
      if (!style) {
        style = document.createElement('style');
        style.id = STYLE_ID;
        (document.head || document.documentElement).appendChild(style);
      }
      if (style.textContent !== CSS) style.textContent = CSS;
    }
    function setImp(el, name, value) {
      if (!el) return;
      if (el.style.getPropertyValue(name) === value && el.style.getPropertyPriority(name) === 'important') return;
      el.style.setProperty(name, value, 'important');
    }
    function applyLayout() {
      const frame = document.querySelector('.pI_x6G_frame');
      if (frame) {
        const rightOpen = !frame.hasAttribute('data-rightbar-collapsed');
        setImp(frame, 'grid-template-rows', '52px minmax(0,1fr)');
        setImp(frame, 'grid-template-columns', rightOpen ? 'minmax(0,1fr) 0 minmax(0,45vw)' : 'minmax(0,1fr) 0 0');
        setImp(frame, 'display', 'grid');
      }
      const side = document.querySelector('.pI_x6G_sidebarCol');
      setImp(side, 'grid-row', '1');
      setImp(side, 'grid-column', '1 / -1');
      setImp(side, 'max-height', '52px');
      setImp(side, 'min-height', '52px');
      setImp(side, 'height', '52px');
      setImp(side, 'overflow', 'hidden');
      const center = document.querySelector('.pI_x6G_centerCol');
      setImp(center, 'grid-row', '2');
      setImp(center, 'grid-column', '1 / -1');
      const root = document.querySelector('.hHd-Xa_root');
      setImp(root, 'flex-direction', 'row');
      setImp(root, 'align-items', 'center');
      setImp(root, 'height', '52px');
      setImp(root, 'max-height', '52px');
      setImp(root, 'width', '100%');
      setImp(root, 'padding', '8px 40px 8px 10px');
    }
    function ensureReload() {
      if (!/Win/i.test(navigator.platform || navigator.userAgent || '')) return;
      if (document.getElementById('openmuse-dsh-reload')) return;
      const root = document.querySelector('.hHd-Xa_root');
      if (!root) return;
      const btn = document.createElement('button');
      btn.id = 'openmuse-dsh-reload';
      btn.type = 'button';
      btn.title = '刷新 DSH 面板';
      btn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/></svg>';
      btn.addEventListener('click', function(event) {
        event.preventDefault();
        event.stopPropagation();
        location.reload();
      });
      const list = document.querySelector('.hHd-Xa_panelList');
      if (list && list.parentNode) {
        list.parentNode.insertBefore(btn, list.nextSibling);
      } else {
        root.appendChild(btn);
      }
    }
    let scheduled = false;
    function tick() {
      if (scheduled) return;
      scheduled = true;
      requestAnimationFrame(function() {
        scheduled = false;
        ensureStyle();
        applyLayout();
        ensureReload();
      });
    }
    tick();
    new MutationObserver(tick).observe(document.documentElement, {childList:true, subtree:true, attributes:true});
  })();
  """
}

private final class DshClipboardMessageHandler: NSObject, WKScriptMessageHandler {
  private let port: Int

  init(port: Int) { self.port = port }

  func userContentController(_ userContentController: WKUserContentController,
                             didReceive message: WKScriptMessage) {
    let origin = message.frameInfo.securityOrigin
    guard message.frameInfo.isMainFrame,
          origin.`protocol` == "http",
          (origin.host == "127.0.0.1" || origin.host == "localhost"),
          origin.port == port,
          let raw = message.body as? String,
          raw.utf8.count <= 20_000,
          let data = raw.data(using: .utf8),
          let payload = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          payload["type"] as? String == "clipboard.write",
          let value = payload["text"] as? String,
          !value.isEmpty,
          value.utf8.count <= 16_384 else { return }
    NSPasteboard.general.clearContents()
    NSPasteboard.general.setString(value, forType: .string)
  }
}

private final class DshResourceMessageHandler: NSObject, WKScriptMessageHandler {
  private let channel: FlutterMethodChannel
  private let port: Int
  private let method: String

  init(channel: FlutterMethodChannel, port: Int, method: String) {
    self.channel = channel
    self.port = port
    self.method = method
  }

  func userContentController(_ userContentController: WKUserContentController,
                             didReceive message: WKScriptMessage) {
    let origin = message.frameInfo.securityOrigin
    guard message.frameInfo.isMainFrame,
          origin.`protocol` == "http",
          (origin.host == "127.0.0.1" || origin.host == "localhost"),
          origin.port == port,
          let payload = message.body as? String,
          payload.utf8.count <= 16_384 else { return }
    channel.invokeMethod(method, arguments: payload)
  }
}
