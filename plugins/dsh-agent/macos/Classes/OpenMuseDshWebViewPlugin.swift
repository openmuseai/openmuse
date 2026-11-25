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
    // Cookie scope excludes ports. A shared localhost jar accumulates a cookie
    // for every DSH ephemeral port until Node rejects its header with HTTP 431.
    configuration.websiteDataStore = .nonPersistent()
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
    configuration.userContentController.add(DshPageDiagnostics(), name: "MuseHostDiagnostics")
    if let script=values["resourceReferenceScript"] as? String,script.utf8.count<65536 {
      configuration.userContentController.addUserScript(WKUserScript(source:script,injectionTime:.atDocumentEnd,forMainFrameOnly:true))
    }
    configuration.userContentController.addUserScript(WKUserScript(source: """
      window.addEventListener('error', event => window.webkit.messageHandlers.MuseHostDiagnostics.postMessage(String(event.message)));
      window.addEventListener('unhandledrejection', event => window.webkit.messageHandlers.MuseHostDiagnostics.postMessage(String(event.reason)));
      """, injectionTime: .atDocumentStart, forMainFrameOnly: true))
    configuration.userContentController.addUserScript(WKUserScript(
      source: """
      window.MuseHostResource = { postMessage: function(raw) { window.webkit.messageHandlers.MuseHostResource.postMessage(raw); } };
      document.addEventListener('click', function(event) {
        const link=event.target?.closest?.('a[href]');
        const uri=link?.getAttribute('href');
        if(!uri?.startsWith('openmuse-resource://v1/'))return;
        event.preventDefault();event.stopImmediatePropagation();
        window.MuseHostResource.postMessage(JSON.stringify({type:'resource.reference.open',uri:uri}));
      },true);
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
    webView.setValue(true, forKey: "drawsBackground")
    let navigation = DshNavigation(channel: channel)
    webView.navigationDelegate = navigation
    let host = DshWebHostView(webView: webView, navigation: navigation)
    channel.setMethodCallHandler { [weak webView] call, result in
      switch call.method {
      case "openSession":
        guard let sessionId = call.arguments as? String,
              let data = try? JSONSerialization.data(withJSONObject: [sessionId]),
              let json = String(data: data, encoding: .utf8) else { result(FlutterMethodNotImplemented); return }
        webView?.evaluateJavaScript("window.__OpenMuseDesiredSession = \(json)[0]; window.OpenMuseDshWorkspace?.openSession(window.__OpenMuseDesiredSession);")
        result(nil)
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
    navigation.loadAuthenticated(url, into: webView)
    return host
  }

  func createArgsCodec() -> (FlutterMessageCodec & NSObjectProtocol)? {
    FlutterStandardMessageCodec.sharedInstance()
  }
}

/// Flutter sets the platform view's frame. WKWebView does not follow that
/// frame on its own, so a zero-size first layout stays blank.
final class DshWebHostView: NSView {
  let webView: WKWebView
  let navigation: DshNavigation

  init(webView: WKWebView, navigation: DshNavigation) {
    self.webView = webView
    self.navigation = navigation
    super.init(frame: .zero)
    autoresizesSubviews = true
    addSubview(webView)
    webView.autoresizingMask = [.width, .height]
  }

  required init?(coder: NSCoder) { nil }

  override func setFrameSize(_ newSize: NSSize) {
    super.setFrameSize(newSize)
    webView.frame = bounds
  }

  override func layout() {
    super.layout()
    webView.frame = bounds
  }
}

final class DshNavigation: NSObject, WKNavigationDelegate, URLSessionTaskDelegate {
  let channel: FlutterMethodChannel
  var bootstrapOrigin: URL?
  init(channel: FlutterMethodChannel) { self.channel = channel }
  func loadAuthenticated(_ url: URL, into webView: WKWebView) {
    // Exchange DSH's one-process launch URL with a cookie-aware HTTP session,
    // then install the authenticated HttpOnly cookie before loading the clean
    // URL in WebKit. Never render the empty token-exchange redirect as a page.
    let configuration = URLSessionConfiguration.ephemeral
    configuration.timeoutIntervalForRequest = 15
    bootstrapOrigin = url
    let session = URLSession(configuration: configuration, delegate: self, delegateQueue: nil)
    session.dataTask(with: url) { [weak self, weak webView] data, response, error in
      guard let self = self else { session.invalidateAndCancel(); return }
      guard error == nil, let http = response as? HTTPURLResponse,
            http.statusCode == 200, http.url?.host == url.host,
            http.url?.port == url.port,
            let data = data, !data.isEmpty else {
        DispatchQueue.main.async { self.channel.invokeMethod("failed", arguments: "DSH 认证页面加载失败，请重试。") }
        session.invalidateAndCancel()
        return
      }
      let cookies = session.configuration.httpCookieStorage?.cookies ?? []
      DispatchQueue.main.async {
        guard let webView = webView else { session.invalidateAndCancel(); return }
        let group = DispatchGroup()
        for cookie in cookies {
          group.enter()
          webView.configuration.websiteDataStore.httpCookieStore.setCookie(cookie) { group.leave() }
        }
        group.notify(queue: .main) {
          var clean = URLComponents(url: url, resolvingAgainstBaseURL: false)!
          clean.query = nil
          webView.load(URLRequest(url: clean.url!))
          session.finishTasksAndInvalidate()
        }
      }
    }.resume()
  }
  func urlSession(_ session: URLSession, task: URLSessionTask,
                  willPerformHTTPRedirection response: HTTPURLResponse,
                  newRequest request: URLRequest,
                  completionHandler: @escaping (URLRequest?) -> Void) {
    let next = request.url
    completionHandler(next?.scheme == bootstrapOrigin?.scheme && next?.host == bootstrapOrigin?.host && next?.port == bootstrapOrigin?.port ? request : nil)
  }
  func webView(_ webView: WKWebView, decidePolicyFor response: WKNavigationResponse,
               decisionHandler: @escaping (WKNavigationResponsePolicy) -> Void) {
    if response.isForMainFrame, let http = response.response as? HTTPURLResponse {
      NSLog("DSH WebView HTTP %ld", http.statusCode)
      if http.statusCode >= 400 {
        channel.invokeMethod("failed", arguments: "DSH 页面加载失败：HTTP \(http.statusCode)")
        decisionHandler(.cancel)
        return
      }
    }
    decisionHandler(.allow)
  }
  func webView(_ webView: WKWebView, didFailProvisionalNavigation navigation: WKNavigation!, withError error: Error) {
    let value = error as NSError
    if value.domain == NSURLErrorDomain && value.code == NSURLErrorCancelled { return }
    NSLog("DSH WebView navigation failed: %@ %ld", value.domain, value.code)
    channel.invokeMethod("failed", arguments: "DSH 页面加载失败：\(value.domain) \(value.code)")
  }
  func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    NSLog("DSH WebView loaded; bounds %.0f x %.0f", webView.bounds.width, webView.bounds.height)
    webView.evaluateJavaScript("JSON.stringify({ready:document.readyState,textLength:document.body?.innerText.length,rootChildren:document.getElementById('root')?.children.length})") { value, _ in
      NSLog("DSH WebView DOM: %@", String(describing: value))
    }
  }
  func webViewWebContentProcessDidTerminate(_ webView: WKWebView) {
    channel.invokeMethod("failed", arguments: "DSH 页面进程已退出，请重新加载。")
  }
}

private final class DshPageDiagnostics: NSObject, WKScriptMessageHandler {
  func userContentController(_ controller: WKUserContentController, didReceive message: WKScriptMessage) {
    guard message.frameInfo.isMainFrame, let text = message.body as? String else { return }
    let safe = text.replacingOccurrences(of: "token=[^&\\s]+", with: "token=<redacted>", options: .regularExpression)
    NSLog("DSH WebView JavaScript: %@", String(safe.prefix(500)))
  }
}

/// Embedded Host chrome: DSH's icon rail is always a top strip, matching the
/// macOS / latest DSH layout. Class hashes are checked against DSH 0.2.0-rc.2
/// and must stay in sync with plugins/dsh-agent/windows/openmuse_dsh_plugin.cpp.
private enum DshCompactSidebarStyle {
  static let userScript = """
  (function() {
    const STYLE_ID = 'openmuse-dsh-compact-sidebar';
    const CSS = '.pI_x6G_frame[data-sidebar-collapsed]{grid-template-rows:52px minmax(0,1fr)!important;grid-template-columns:minmax(0,1fr) 0 0!important}.pI_x6G_frame[data-sidebar-collapsed]:not([data-rightbar-collapsed]){grid-template-columns:minmax(0,1fr) 0 minmax(0,45vw)!important}.pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_sidebarCol{grid-area:1/1/2/-1!important;border-right:none!important;border-bottom:0.5px solid var(--dsw-alias-border-l3,#ececf0);min-height:52px;max-height:52px;overflow:hidden}.pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_centerCol{grid-area:2/1/3/2!important}.pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_rightbarCol{grid-area:2/3/3/4!important}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_root{flex-direction:row!important;align-items:center!important;padding:8px 40px 8px 10px!important;height:52px!important;max-height:52px!important;gap:4px;width:100%}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_topStrip,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_regionArea,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_brandName,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_fallbackBrandName,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_localBuildBrand,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_newSessionLabel{display:none!important}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_logoRow,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_newSession{margin:0!important;height:36px!important;width:36px!important}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_panelList,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_footArea,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_footerActions,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_settingsArea{flex-direction:row!important;margin:0!important;width:auto!important;gap:4px!important}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_footArea{align-items:center!important;order:-1}.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_iconButton,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_newSession,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_panelList,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_regionArea,.pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_footArea{animation:none!important;transform:none!important}#openmuse-dsh-reload{flex:none;display:inline-flex;align-items:center;justify-content:center;width:36px;height:36px;margin:0;padding:0;border:none;border-radius:50%;background:transparent;color:inherit;cursor:pointer}#openmuse-dsh-reload:hover{background:var(--dsw-alias-interactive-bg-hover,rgba(0,0,0,.06))}';
    function ensureStyle() {
      let style = document.getElementById(STYLE_ID);
      if (!style) {
        style = document.createElement('style');
        style.id = STYLE_ID;
        (document.head || document.documentElement).appendChild(style);
      }
      if (style.textContent !== CSS) style.textContent = CSS;
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
