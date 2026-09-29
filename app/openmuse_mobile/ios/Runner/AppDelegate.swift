import CryptoKit
import Flutter
import Security
import UIKit

@_silgen_name("openmuse_docx_abi_version")
private func openMuseDocxAbiVersion() -> UInt32

@main
@objc class AppDelegate: FlutterAppDelegate, FlutterImplicitEngineDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    precondition(openMuseDocxAbiVersion() == 1, "Unsupported OpenMuse DOCX ABI")
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }

  func didInitializeImplicitFlutterEngine(_ engineBridge: FlutterImplicitEngineBridge) {
    GeneratedPluginRegistrant.register(with: engineBridge.pluginRegistry)
    DeviceKeyStoreBridge.register(messenger: engineBridge.applicationRegistrar.messenger())
  }
}

private enum DeviceKeyStoreBridge {
  private static let channelName = "io.openmuse/device_keystore"
  private static let service = "io.openmuse.device-seed.v1"

  static func register(messenger: FlutterBinaryMessenger) {
    FlutterMethodChannel(name: channelName, binaryMessenger: messenger)
      .setMethodCallHandler { call, result in
        do {
          switch call.method {
          case "ensure":
            let arguments = try dictionary(call.arguments)
            result(
              try ensure(
                accountRef: try reference(arguments["accountRef"]),
                deviceRef: try reference(arguments["deviceRef"])
              )
            )
          case "delete":
            let arguments = try dictionary(call.arguments)
            try delete(keyRef: try reference(arguments["keyRef"]))
            result(nil)
          default:
            result(FlutterMethodNotImplemented)
          }
        } catch {
          result(
            FlutterError(
              code: "device_keystore",
              message: "Device key operation failed",
              details: nil
            )
          )
        }
      }
  }

  private static func ensure(accountRef: String, deviceRef: String) throws -> [String: Any] {
    let identity = Data((accountRef + "\0" + deviceRef).utf8)
    let digest = SHA256.hash(data: identity).map { String(format: "%02x", $0) }.joined()
    let keyRef = "device-key:\(digest)"
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: service,
      kSecAttrAccount as String: keyRef,
      kSecAttrSynchronizable as String: kCFBooleanFalse as Any,
      kSecMatchLimit as String: kSecMatchLimitOne,
    ]
    let lookup = SecItemCopyMatching(query as CFDictionary, nil)
    var created = false
    if lookup == errSecItemNotFound {
      var seed = Data(count: 32)
      let status = seed.withUnsafeMutableBytes { buffer in
        SecRandomCopyBytes(kSecRandomDefault, 32, buffer.baseAddress!)
      }
      guard status == errSecSuccess else { throw KeyStoreError.operation }
      defer { seed.resetBytes(in: 0..<seed.count) }
      let add: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: keyRef,
        kSecAttrSynchronizable as String: kCFBooleanFalse as Any,
        kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly,
        kSecValueData as String: seed,
      ]
      guard SecItemAdd(add as CFDictionary, nil) == errSecSuccess else {
        throw KeyStoreError.operation
      }
      created = true
    } else if lookup != errSecSuccess {
      throw KeyStoreError.operation
    }
    return [
      "keyRef": keyRef,
      "storage": "ios-keychain-this-device-only",
      "hardwareBacked": false,
      "created": created,
    ]
  }

  private static func delete(keyRef: String) throws {
    guard keyRef.range(of: #"^device-key:[0-9a-f]{64}$"#, options: .regularExpression) != nil
    else { throw KeyStoreError.invalid }
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: service,
      kSecAttrAccount as String: keyRef,
      kSecAttrSynchronizable as String: kCFBooleanFalse as Any,
    ]
    let status = SecItemDelete(query as CFDictionary)
    guard status == errSecSuccess || status == errSecItemNotFound else {
      throw KeyStoreError.operation
    }
  }

  private static func dictionary(_ value: Any?) throws -> [String: Any] {
    guard let result = value as? [String: Any] else { throw KeyStoreError.invalid }
    return result
  }

  private static func reference(_ value: Any?) throws -> String {
    guard let result = value as? String,
      !result.isEmpty,
      result.count <= 256,
      !result.contains("\0")
    else { throw KeyStoreError.invalid }
    return result
  }

  private enum KeyStoreError: Error { case invalid, operation }
}
