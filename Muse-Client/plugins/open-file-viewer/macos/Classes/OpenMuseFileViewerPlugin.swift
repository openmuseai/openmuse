import AVFoundation
import AVKit
import Cocoa
import FlutterMacOS
import WebKit

public final class OpenMuseFileViewerPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    registrar.register(OpenMuseViewerFactory(), withId: "com.openmuse.viewer")
  }
}

final class OpenMuseViewerFactory: NSObject, FlutterPlatformViewFactory {
  private static let allowedExtensions: Set<String> = [
    "png", "jpg", "jpeg", "gif", "webp", "svg", "pdf",
    "mp4", "m4v", "mov", "webm",
  ]
  private static let videoExtensions: Set<String> = [
    "mp4", "m4v", "mov", "webm",
  ]

  func create(withViewIdentifier viewId: Int64, arguments args: Any?) -> NSView {
    let configuration = WKWebViewConfiguration()
    if #available(macOS 11.0, *) {
      configuration.defaultWebpagePreferences.allowsContentJavaScript = false
    } else {
      configuration.preferences.javaScriptEnabled = false
    }
    configuration.websiteDataStore = .nonPersistent()
    let webView = WKWebView(frame: .zero, configuration: configuration)
    webView.setValue(false, forKey: "drawsBackground")

    guard let values = args as? [String: Any], let path = values["path"] as? String else {
      showError("Missing structured viewer path", in: webView)
      return webView
    }
    let url = URL(fileURLWithPath: path).standardizedFileURL
    guard Self.allowedExtensions.contains(url.pathExtension.lowercased()) else {
      showError("Unsupported local file type", in: webView)
      return webView
    }
    guard FileManager.default.fileExists(atPath: url.path) else {
      showError("Local file does not exist", in: webView)
      return webView
    }
    if Self.videoExtensions.contains(url.pathExtension.lowercased()) {
      return OpenMuseVideoView(url: url)
    }
    webView.loadFileURL(url, allowingReadAccessTo: url.deletingLastPathComponent())
    return webView
  }

  func createArgsCodec() -> (FlutterMessageCodec & NSObjectProtocol)? {
    FlutterStandardMessageCodec.sharedInstance()
  }

  private func showError(_ message: String, in webView: WKWebView) {
    let escaped = message
      .replacingOccurrences(of: "&", with: "&amp;")
      .replacingOccurrences(of: "<", with: "&lt;")
      .replacingOccurrences(of: ">", with: "&gt;")
    webView.loadHTMLString(
      "<html><body style='font: 15px -apple-system; padding: 24px'>\(escaped)</body></html>",
      baseURL: nil
    )
  }
}

final class OpenMuseVideoView: NSView {
  private let player: AVPlayer
  private let playerView: AVPlayerView

  init(url: URL) {
    player = AVPlayer(url: url)
    playerView = AVPlayerView(frame: .zero)
    super.init(frame: .zero)
    playerView.player = player
    playerView.controlsStyle = .floating
    playerView.translatesAutoresizingMaskIntoConstraints = false
    addSubview(playerView)
    NSLayoutConstraint.activate([
      playerView.leadingAnchor.constraint(equalTo: leadingAnchor),
      playerView.trailingAnchor.constraint(equalTo: trailingAnchor),
      playerView.topAnchor.constraint(equalTo: topAnchor),
      playerView.bottomAnchor.constraint(equalTo: bottomAnchor),
    ])
  }

  required init?(coder: NSCoder) {
    return nil
  }
}
