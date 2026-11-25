import Flutter
import UIKit
import UserNotifications

public final class TaskNotificationsPlugin: NSObject, FlutterPlugin, UNUserNotificationCenterDelegate {
  private let channel: FlutterMethodChannel
  private var activated = false
  private weak var previousDelegate: UNUserNotificationCenterDelegate?
  private let prefix = "openmuse.task."

  private init(channel: FlutterMethodChannel) { self.channel = channel; super.init() }
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "io.openmuse/task_notifications", binaryMessenger: registrar.messenger())
    let plugin = TaskNotificationsPlugin(channel: channel)
    registrar.addMethodCallDelegate(plugin, channel: channel)
    let center = UNUserNotificationCenter.current()
    plugin.previousDelegate = center.delegate
    center.delegate = plugin
  }
  private func status(_ reply: @escaping (String) -> Void) {
    UNUserNotificationCenter.current().getNotificationSettings { settings in
      let state: String
      switch settings.authorizationStatus {
      case .authorized, .provisional, .ephemeral: state = settings.alertSetting == .disabled ? "denied" : "granted"
      case .notDetermined: state = "notDetermined"
      case .denied: state = "denied"
      @unknown default: state = "unavailable"
      }
      DispatchQueue.main.async { reply(state) }
    }
  }
  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    let center = UNUserNotificationCenter.current()
    switch call.method {
    case "authorization": status { result($0) }
    case "request":
      center.requestAuthorization(options: [.alert, .sound]) { _, error in
        if error != nil { DispatchQueue.main.async { result(FlutterError(code: "notification_permission", message: "Notification permission is unavailable", details: nil)) }; return }
        self.status { result($0) }
      }
    case "settings":
      let address: String
      if #available(iOS 16.0, *) { address = UIApplication.openNotificationSettingsURLString }
      else { address = UIApplication.openSettingsURLString }
      guard let url = URL(string: address) else { result(false); return }
      UIApplication.shared.open(url) { opened in
        result(opened ? nil : FlutterError(code: "notification_settings", message: "Unable to open system settings", details: nil))
      }
    case "show":
      guard let args = call.arguments as? [String: Any], let id = args["id"] as? Int, id > 0,
        let title = args["title"] as? String, !title.isEmpty, title.count <= 256,
        let body = args["body"] as? String, !body.isEmpty, body.count <= 256 else {
        result(FlutterError(code: "notification_input", message: "Invalid notification input", details: nil)); return
      }
      status { state in
        guard state == "granted" else { result(nil); return }
        let content = UNMutableNotificationContent()
        content.title = title; content.body = body; content.sound = .default
        content.threadIdentifier = "openmuse.task"
        center.add(UNNotificationRequest(identifier: self.prefix + String(id), content: content, trigger: nil)) { error in
          DispatchQueue.main.async { result(error == nil ? nil : FlutterError(code: "notification_show", message: "Unable to show local notification", details: nil)) }
        }
      }
    case "clear":
      center.getDeliveredNotifications { notifications in
        center.removeDeliveredNotifications(withIdentifiers: notifications.map { $0.request.identifier }.filter { $0.hasPrefix(self.prefix) })
        center.getPendingNotificationRequests { requests in
          center.removePendingNotificationRequests(withIdentifiers: requests.map { $0.identifier }.filter { $0.hasPrefix(self.prefix) })
          DispatchQueue.main.async { result(nil) }
        }
      }
    case "takeActivation": result(activated); activated = false
    default: result(FlutterMethodNotImplemented)
    }
  }
  public func userNotificationCenter(_ center: UNUserNotificationCenter, willPresent notification: UNNotification, withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void) {
    if notification.request.identifier.hasPrefix(prefix) {
      completionHandler([.banner, .list, .sound])
    } else if let previous = previousDelegate, previous.responds(to: #selector(userNotificationCenter(_:willPresent:withCompletionHandler:))) {
      previous.userNotificationCenter?(center, willPresent: notification, withCompletionHandler: completionHandler)
    } else { completionHandler([]) }
  }
  public func userNotificationCenter(_ center: UNUserNotificationCenter, didReceive response: UNNotificationResponse, withCompletionHandler completionHandler: @escaping () -> Void) {
    if response.notification.request.identifier.hasPrefix(prefix) {
      activated = true; channel.invokeMethod("activated", arguments: nil)
      completionHandler()
    } else if let previous = previousDelegate, previous.responds(to: #selector(userNotificationCenter(_:didReceive:withCompletionHandler:))) {
      previous.userNotificationCenter?(center, didReceive: response, withCompletionHandler: completionHandler)
    } else { completionHandler() }
  }
}
