import Cocoa
import FlutterMacOS

/// Desktop `flutter run --no-enable-impeller` sets FLUTTER_ENGINE_SWITCH_* in
/// the process environment. `open .app` does not, and FLTEnableImpeller in
/// Info.plist is not read on macOS — Impeller then blanks the window
/// (WKWebView platform views / newer macOS). Apply the same switch here
/// before the engine starts.
func disableImpellerForDesktop() {
  setenv("FLUTTER_ENGINE_SWITCHES", "1", 1)
  setenv("FLUTTER_ENGINE_SWITCH_1", "enable-impeller=false", 1)
}

/// `open .app` starts with cwd `/`. Anything that lists Directory.current
/// then hangs on automounts / TCC-protected folders.
func sanitizeWorkingDirectory() {
  let cwd = FileManager.default.currentDirectoryPath
  if cwd == "/" {
    FileManager.default.changeCurrentDirectoryPath(
      FileManager.default.homeDirectoryForCurrentUser.path
    )
  }
}

@main
class AppDelegate: FlutterAppDelegate {
  private var menuChannel: FlutterMethodChannel?
  @IBOutlet weak var agentViewItem: NSMenuItem?
  @IBOutlet weak var ideViewItem: NSMenuItem?
  private var storeDistribution = false

  private func applyStoreDistribution() {
    storeDistribution = true
    for item in [agentViewItem, ideViewItem].compactMap({ $0 }) {
      item.menu?.removeItem(item)
    }
    if let menu = NSApp.mainMenu {
      for item in menu.items where ["终端", "Terminal"].contains(item.title) {
        menu.removeItem(item)
      }
    }
  }

  override func applicationWillFinishLaunching(_ notification: Notification) {
    disableImpellerForDesktop()
    sanitizeWorkingDirectory()
    super.applicationWillFinishLaunching(notification)
  }

  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
    return true
  }

  override func applicationWillTerminate(_ notification: Notification) {
    (mainFlutterWindow as? MainFlutterWindow)?.releaseWorkspaceAccess()
    super.applicationWillTerminate(notification)
  }

  override func applicationSupportsSecureRestorableState(_ app: NSApplication) -> Bool {
    return true
  }

  override func applicationDidFinishLaunching(_ notification: Notification) {
    super.applicationDidFinishLaunching(notification)
    if Bundle.main.object(forInfoDictionaryKey: "OpenMuseDistributionChannel") as? String == "app-store" {
      applyStoreDistribution()
    }
    guard let controller = mainFlutterWindow?.contentViewController as? FlutterViewController else {
      return
    }
    let channel = FlutterMethodChannel(
      name: "com.openmuse.host/menu",
      binaryMessenger: controller.engine.binaryMessenger
    )
    menuChannel = channel
    channel.setMethodCallHandler { [weak self] call, result in
      if call.method == "configureDistribution", call.arguments as? String == "app-store" {
        self?.applyStoreDistribution()
        result(nil)
      } else if call.method == "setCheckedView", let name = call.arguments as? String {
        self?.setCheckedView(name)
        result(nil)
      } else {
        result(FlutterMethodNotImplemented)
      }
    }
    setCheckedView("agent")
  }

  @IBAction func openSettings(_ sender: Any?) {
    menuChannel?.invokeMethod("settings", arguments: nil)
  }

  @IBAction func showAgentView(_ sender: Any?) {
    guard !storeDistribution else { return }
    setCheckedView("agent")
    menuChannel?.invokeMethod("view", arguments: "agent")
  }

  @IBAction func showIdeView(_ sender: Any?) {
    guard !storeDistribution else { return }
    setCheckedView("ide")
    menuChannel?.invokeMethod("view", arguments: "ide")
  }

  @IBAction func showViewHelp(_ sender: Any?) {
    menuChannel?.invokeMethod("view", arguments: "help")
  }

  @IBAction func toggleTerminal(_ sender: Any?) {
    guard !storeDistribution else { return }
    menuChannel?.invokeMethod("terminal", arguments: nil)
  }

  private func setCheckedView(_ name: String) {
    agentViewItem?.state = name == "agent" ? .on : .off
    ideViewItem?.state = name == "ide" ? .on : .off
  }
}

