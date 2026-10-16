import Cocoa
import FlutterMacOS
import WebKit

/// Displays an MCP App's standard HTML resource inside the Host window.
/// This view exposes no privileged JavaScript bridge to the plugin page.
final class McpAppWebViewFactory: NSObject, FlutterPlatformViewFactory {
  func create(withViewIdentifier viewId: Int64, arguments args: Any?) -> NSView {
    guard let values = args as? [String: Any],
          let raw = values["url"] as? String,
          let url = URL(string: raw),
          url.scheme == "https" || (url.scheme == "http" &&
            (url.host == "127.0.0.1" || url.host == "localhost")) else {
      return NSTextField(labelWithString: "Invalid MCP App URL")
    }
    let configuration = WKWebViewConfiguration()
    configuration.websiteDataStore = .default()
    let webView = WKWebView(frame: .zero, configuration: configuration)
    let navigation = McpAppNavigation(origin: url)
    webView.navigationDelegate = navigation
    let host = McpAppHostView(webView: webView, navigation: navigation)
    webView.load(URLRequest(url: url))
    return host
  }

  func createArgsCodec() -> (FlutterMessageCodec & NSObjectProtocol)? {
    FlutterStandardMessageCodec.sharedInstance()
  }
}

private final class McpAppNavigation: NSObject, WKNavigationDelegate {
  let origin: URL
  init(origin: URL) { self.origin = origin }

  func webView(_ webView: WKWebView, decidePolicyFor action: WKNavigationAction,
               decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
    guard let next = action.request.url,
          next.scheme == origin.scheme,
          next.host == origin.host,
          next.port == origin.port else {
      decisionHandler(.cancel)
      return
    }
    decisionHandler(.allow)
  }
}

private final class McpAppHostView: NSView {
  let webView: WKWebView
  // WKWebView keeps navigationDelegate weak; the Host owns it for this view.
  let navigation: McpAppNavigation

  init(webView: WKWebView, navigation: McpAppNavigation) {
    self.webView = webView
    self.navigation = navigation
    super.init(frame: .zero)
    autoresizesSubviews = true
    addSubview(webView)
    webView.autoresizingMask = [.width, .height]
  }

  required init?(coder: NSCoder) { nil }

  override func layout() {
    super.layout()
    webView.frame = bounds
  }
}
