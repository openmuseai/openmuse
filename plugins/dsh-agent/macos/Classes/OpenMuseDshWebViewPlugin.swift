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

/// Collapsed DSH icon rail sits as a top strip in the Host pane; expanded
/// sidebar stays on the left. Class hashes are pinned to DSH 0.1.7-rc.1.
private enum DshCompactSidebarStyle {
  static let userScript = """
  (function() {
    if (document.getElementById('openmuse-dsh-compact-sidebar')) return;
    const style = document.createElement('style');
    style.id = 'openmuse-dsh-compact-sidebar';
    style.textContent = `
      .pI_x6G_frame[data-sidebar-collapsed] {
        grid-template-rows: 52px minmax(0, 1fr) !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed][data-rightbar-collapsed] {
        grid-template-columns: minmax(0, 1fr) 0 0 !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed]:not([data-rightbar-collapsed]) {
        grid-template-columns: minmax(0, 1fr) 0 minmax(0, 45vw) !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_sidebarCol {
        grid-area: 1 / 1 / 2 / -1;
        border-right: none;
        border-bottom: 0.5px solid var(--dsw-alias-border-l3, #ececf0);
        min-height: 52px;
        overflow: hidden;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_centerCol {
        grid-area: 2 / 1 / 3 / 2;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .pI_x6G_rightbarCol {
        grid-area: 2 / 3 / 3 / 4;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_root.hHd-Xa_collapsed {
        flex-direction: row !important;
        align-items: center !important;
        padding: 8px 72px 8px 10px !important;
        height: 100% !important;
        gap: 4px;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_topStrip,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_regionArea {
        display: none !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_logoRow,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_newSession {
        margin: 0 !important;
        height: 36px !important;
        width: 36px !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_panelList,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_footArea,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_footerActions,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_settingsArea {
        flex-direction: row !important;
        margin: 0 !important;
        width: auto !important;
        gap: 4px !important;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_collapsed .hHd-Xa_footArea {
        margin-left: 0 !important;
        align-items: center !important;
        order: -1;
      }
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_iconButton,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_newSession,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_panelList,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_regionArea,
      .pI_x6G_frame[data-sidebar-collapsed] .hHd-Xa_railIn .hHd-Xa_footArea {
        animation: none !important;
        transform: none !important;
      }
    `;
    document.documentElement.appendChild(style);
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
