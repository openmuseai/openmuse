import Flutter
import UIKit

public class SwiftAppFlowyBackendPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    // Keep a live native reference so ld cannot drop the Rust staticlib.
    dummyMethodToEnforceBundling()
    let channel = FlutterMethodChannel(name: "appflowy_backend", binaryMessenger: registrar.messenger())
    let instance = SwiftAppFlowyBackendPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }

  public static func dummyMethodToEnforceBundling() {
    link_me_please()
  }
}
