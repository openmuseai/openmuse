import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  private var scopedWorkspaceURLs: [String: URL] = [:]
  private let bookmarksKey = "OpenMuseWorkspaceBookmarks"

  private func restoreWorkspaceBookmarks() {
    let stored = UserDefaults.standard.dictionary(forKey: bookmarksKey) ?? [:]
    var refreshed = stored
    for (path, value) in stored {
      guard let data = value as? Data else { continue }
      do {
        var stale = false
        let url = try URL(resolvingBookmarkData: data, options: [.withSecurityScope],
                          relativeTo: nil, bookmarkDataIsStale: &stale)
        if url.startAccessingSecurityScopedResource() {
          scopedWorkspaceURLs[path] = url
          if stale {
            refreshed[path] = try url.bookmarkData(options: [.withSecurityScope],
                                                   includingResourceValuesForKeys: nil, relativeTo: nil)
          }
        }
      } catch {
        // Keep the bookmark for explicit user reauthorization; never broaden access.
        NSLog("Workspace bookmark restore failed: %@", error.localizedDescription)
      }
    }
    UserDefaults.standard.set(refreshed, forKey: bookmarksKey)
  }

  private func retainWorkspaceAccess(_ url: URL) throws {
    let data = try url.bookmarkData(options: [.withSecurityScope],
                                    includingResourceValuesForKeys: nil, relativeTo: nil)
    var stored = UserDefaults.standard.dictionary(forKey: bookmarksKey) ?? [:]
    stored[url.path] = data
    UserDefaults.standard.set(stored, forKey: bookmarksKey)
    if scopedWorkspaceURLs[url.path] == nil && url.startAccessingSecurityScopedResource() {
      scopedWorkspaceURLs[url.path] = url
    }
  }

  func releaseWorkspaceAccess() {
    for url in scopedWorkspaceURLs.values { url.stopAccessingSecurityScopedResource() }
    scopedWorkspaceURLs.removeAll()
  }

  override func awakeFromNib() {
    disableImpellerForDesktop()
    sanitizeWorkingDirectory()
    restoreWorkspaceBookmarks()
    let flutterViewController = FlutterViewController()
    let windowFrame = self.frame
    self.contentViewController = flutterViewController
    self.setFrame(windowFrame, display: true)
    self.minSize = NSSize(width: 1100, height: 650)
    self.titleVisibility = .hidden
    self.titlebarAppearsTransparent = true
    self.styleMask.insert(.fullSizeContentView)
    self.isMovableByWindowBackground = false
    if windowFrame.width < 1100 || windowFrame.height < 650 {
      let visible = self.screen?.visibleFrame ?? NSScreen.main?.visibleFrame
      let targetWidth = min(1280, visible?.width ?? 1280)
      let targetHeight = min(800, visible?.height ?? 800)
      self.setContentSize(NSSize(width: targetWidth, height: targetHeight))
      self.center()
    }

    RegisterGeneratedPlugins(registry: flutterViewController)
    flutterViewController.registrar(forPlugin: "OpenMuseMcpAppWebView")
      .register(McpAppWebViewFactory(messenger: flutterViewController.engine.binaryMessenger), withId: "com.openmuse.mcp/app-view")

    let workspaceChannel = FlutterMethodChannel(
      name: "com.openmuse.host/workspace",
      binaryMessenger: flutterViewController.engine.binaryMessenger
    )
    workspaceChannel.setMethodCallHandler { [weak self] call, result in
      switch call.method {
      case "chooseDirectory":
        let panel = NSOpenPanel()
        panel.canChooseFiles = false
        panel.canChooseDirectories = true
        panel.allowsMultipleSelection = false
        panel.canCreateDirectories = true
        guard panel.runModal() == .OK, let url = panel.url else {
          result(nil)
          return
        }
        do {
          try self?.retainWorkspaceAccess(url)
          result(url.path)
        } catch {
          result(FlutterError(code: "workspace-authorization", message: error.localizedDescription, details: nil))
        }
      case "reveal":
        guard
          let values = call.arguments as? [String: Any],
          let path = values["path"] as? String
        else {
          result(FlutterError(code: "invalid-arguments", message: "Missing path", details: nil))
          return
        }
        NSWorkspace.shared.activateFileViewerSelecting([URL(fileURLWithPath: path)])
        result(nil)
      default:
        result(FlutterMethodNotImplemented)
      }
    }

    let windowChannel = FlutterMethodChannel(
      name: "com.openmuse.host/window",
      binaryMessenger: flutterViewController.engine.binaryMessenger
    )
    windowChannel.setMethodCallHandler { [weak self] call, result in
      switch call.method {
      case "startDrag":
        if let event = NSApp.currentEvent {
          self?.performDrag(with: event)
        }
        result(nil)
      case "queryFullscreen":
        result(self?.styleMask.contains(.fullScreen) ?? false)
      default:
        result(FlutterMethodNotImplemented)
      }
    }
    NotificationCenter.default.addObserver(
      forName: NSWindow.didEnterFullScreenNotification,
      object: self,
      queue: .main
    ) { _ in
      windowChannel.invokeMethod("fullscreen", arguments: true)
    }
    NotificationCenter.default.addObserver(
      forName: NSWindow.didExitFullScreenNotification,
      object: self,
      queue: .main
    ) { _ in
      windowChannel.invokeMethod("fullscreen", arguments: false)
    }

    super.awakeFromNib()
  }
}
