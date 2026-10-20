import Cocoa
import FlutterMacOS
import WebKit

/// A generic MCP Apps surface. Document formats and editing stay in the View.
final class McpAppWebViewFactory: NSObject, FlutterPlatformViewFactory {
  func create(withViewIdentifier viewId: Int64, arguments args: Any?) -> NSView {
    guard let values = args as? [String: Any] else {
      return NSTextField(labelWithString: "Invalid MCP App")
    }
    let configuration = WKWebViewConfiguration()
    configuration.websiteDataStore = .nonPersistent()
    let navigation: McpAppNavigation
    if let html = values["html"] as? String {
      guard html.utf8.count <= 128 * 1024 * 1024 else {
        return NSTextField(labelWithString: "MCP App exceeds budget")
      }
      navigation = McpAppNavigation(origin: nil, filePath: values["filePath"] as? String)
      configuration.userContentController.add(navigation, name: "mcpApp")
      configuration.userContentController.add(navigation, name: "mcpDiagnostics")
      configuration.userContentController.addUserScript(WKUserScript(source: """
      window.addEventListener('error',event=>window.webkit.messageHandlers.mcpDiagnostics.postMessage(String(event.message)));
      window.addEventListener('unhandledrejection',event=>window.webkit.messageHandlers.mcpDiagnostics.postMessage(String(event.reason)));
      """, injectionTime: .atDocumentStart, forMainFrameOnly: false))
      let webView = WKWebView(frame: .zero, configuration: configuration)
      navigation.webView = webView
      webView.navigationDelegate = navigation
      let host = McpAppHostView(webView: webView, navigation: navigation)
      // Give Web Crypto a secure context without starting a local HTTP server.
      // Navigation stays restricted to the isolated document below.
      webView.loadHTMLString(html, baseURL: URL(string: "https://mcp-app.invalid/"))
      return host
    }
    guard let raw = values["url"] as? String,
          let url = URL(string: raw),
          url.scheme == "https" || (url.scheme == "http" &&
            (url.host == "127.0.0.1" || url.host == "localhost")) else {
      return NSTextField(labelWithString: "Invalid MCP App URL")
    }
    navigation = McpAppNavigation(origin: url, filePath: nil)
    let webView = WKWebView(frame: .zero, configuration: configuration)
    webView.navigationDelegate = navigation
    let host = McpAppHostView(webView: webView, navigation: navigation)
    webView.load(URLRequest(url: url))
    return host
  }

  func createArgsCodec() -> (FlutterMessageCodec & NSObjectProtocol)? {
    FlutterStandardMessageCodec.sharedInstance()
  }
}

private final class McpAppNavigation: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
  let origin: URL?
  let filePath: String?
  weak var webView: WKWebView?
  init(origin: URL?, filePath: String?) { self.origin = origin; self.filePath = filePath }

  func userContentController(_ controller: WKUserContentController, didReceive message: WKScriptMessage) {
    if message.name == "mcpDiagnostics" {
      NSLog("MCP App JavaScript: %@", String(describing: message.body).prefix(500).description)
      return
    }
    // Only the Host's outer frame owns the native bridge. The sandboxed View
    // uses standard MCP Apps postMessage to its parent.
    guard message.frameInfo.isMainFrame,
          let request = message.body as? [String: Any],
          request["jsonrpc"] as? String == "2.0",
          request["method"] as? String == "ui/download-file",
          let id = request["id"],
          let params = request["params"] as? [String: Any],
          let contents = params["contents"] as? [[String: Any]], contents.count == 1,
          contents[0]["type"] as? String == "resource",
          let resource = contents[0]["resource"] as? [String: Any] else { return }
    var response: [String: Any] = ["jsonrpc": "2.0", "id": id]
    do {
      let data: Data
      if let blob = resource["blob"] as? String, blob.count <= 90 * 1024 * 1024,
         let decoded = Data(base64Encoded: blob) { data = decoded }
      else if let text = resource["text"] as? String { data = Data(text.utf8) }
      else { throw NSError(domain: "McpApp", code: 1) }
      guard data.count <= 64 * 1024 * 1024 else { throw NSError(domain: "McpApp", code: 2) }
      let destination: URL?
      if let filePath = filePath {
        // This path is supplied by the Host for the selected resource only.
        destination = URL(fileURLWithPath: filePath)
      } else {
        let panel = NSSavePanel()
        panel.nameFieldStringValue = URL(string: resource["uri"] as? String ?? "")?.lastPathComponent ?? "Export"
        destination = panel.runModal() == .OK ? panel.url : nil
      }
      if let destination = destination {
        try data.write(to: destination, options: .atomic)
        response["result"] = [String: Any]()
      } else { response["result"] = ["isError": true] }
    } catch {
      response["error"] = ["code": -32603, "message": "Unable to save the authorized resource"]
    }
    if let data = try? JSONSerialization.data(withJSONObject: response),
       let json = String(data: data, encoding: .utf8) {
      webView?.evaluateJavaScript("window.__mcpHostReceive(\(json))", completionHandler: nil)
    }
  }

  func webView(_ webView: WKWebView, decidePolicyFor action: WKNavigationAction,
               decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
    guard let next = action.request.url else { decisionHandler(.cancel); return }
    if origin == nil {
      let localDocument = next.absoluteString == "https://mcp-app.invalid/" && action.navigationType == .other
      decisionHandler(next.scheme == "about" || localDocument ? .allow : .cancel)
      return
    }
    guard next.scheme == origin?.scheme, next.host == origin?.host,
          next.port == origin?.port else { decisionHandler(.cancel); return }
    decisionHandler(.allow)
  }
}

private final class McpAppHostView: NSView {
  let webView: WKWebView
  let navigation: McpAppNavigation
  init(webView: WKWebView, navigation: McpAppNavigation) {
    self.webView = webView; self.navigation = navigation
    super.init(frame: .zero)
    autoresizesSubviews = true
    addSubview(webView)
    webView.autoresizingMask = [.width, .height]
  }
  required init?(coder: NSCoder) { nil }
  override func layout() { super.layout(); webView.frame = bounds }
}
