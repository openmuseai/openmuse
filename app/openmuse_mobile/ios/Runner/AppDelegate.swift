import CryptoKit
import Flutter
import Security
import UIKit

@_silgen_name("openmuse_docx_abi_version")
private func openMuseDocxAbiVersion() -> UInt32

@_silgen_name("openmuse_paired_abi_version")
private func openMusePairedAbiVersion() -> UInt32

@_silgen_name("openmuse_office_viewers_abi_version")
private func openMuseOfficeViewersAbiVersion() -> UInt32

private struct OpenMuseOfficeViewerBuffer {
  var ptr: UnsafeMutablePointer<UInt8>?
  var len: Int
  var capacity: Int
  var status: Int32
}

@_silgen_name("openmuse_xlsx_inspect")
private func openMuseXlsxInspect(
  _ bytes: UnsafePointer<UInt8>?,
  _ length: Int
) -> OpenMuseOfficeViewerBuffer

@_silgen_name("openmuse_pptx_inspect")
private func openMusePptxInspect(
  _ bytes: UnsafePointer<UInt8>?,
  _ length: Int
) -> OpenMuseOfficeViewerBuffer

@_silgen_name("openmuse_pdf_inspect")
private func openMusePdfInspect(
  _ bytes: UnsafePointer<UInt8>?,
  _ length: Int
) -> OpenMuseOfficeViewerBuffer

@_silgen_name("openmuse_office_viewer_buffer_free")
private func openMuseOfficeViewerBufferFree(_ buffer: OpenMuseOfficeViewerBuffer)

@_silgen_name("openmuse_paired_device_public")
private func openMusePairedDevicePublic(
  _ seed: UnsafePointer<UInt8>?,
  _ seedLength: Int,
  _ output: UnsafeMutablePointer<UInt8>?,
  _ outputLength: Int
) -> Int32

private struct OpenMusePairedBuffer {
  var ptr: UnsafeMutablePointer<UInt8>?
  var len: Int
  var capacity: Int
  var status: Int32
}

@_silgen_name("openmuse_paired_issue_offer")
private func openMusePairedIssueOffer(
  _ seed: UnsafePointer<UInt8>?,
  _ seedLength: Int,
  _ accountRef: UnsafePointer<UInt8>?,
  _ accountRefLength: Int,
  _ deviceRef: UnsafePointer<UInt8>?,
  _ deviceRefLength: Int,
  _ nonce: UnsafePointer<UInt8>?,
  _ nonceLength: Int,
  _ registrationGeneration: UInt64
) -> OpenMusePairedBuffer

@_silgen_name("openmuse_paired_buffer_free")
private func openMusePairedBufferFree(_ buffer: OpenMusePairedBuffer)

@_silgen_name("openmuse_paired_begin_handshake")
private func openMusePairedBeginHandshake(
  _ seed: UnsafePointer<UInt8>?, _ seedLength: Int,
  _ localOffer: UnsafePointer<UInt8>?, _ localOfferLength: Int,
  _ remoteOffer: UnsafePointer<UInt8>?, _ remoteOfferLength: Int,
  _ localRegistration: UnsafePointer<UInt8>?, _ localRegistrationLength: Int,
  _ remoteRegistration: UnsafePointer<UInt8>?, _ remoteRegistrationLength: Int
) -> OpenMusePairedBuffer

@_silgen_name("openmuse_paired_confirm_handshake")
private func openMusePairedConfirmHandshake(
  _ handshakeHandle: UInt64,
  _ confirmationCode: UnsafePointer<UInt8>?,
  _ confirmationCodeLength: Int
) -> OpenMusePairedBuffer

@_silgen_name("openmuse_paired_channel_seal")
private func openMusePairedChannelSeal(
  _ channelHandle: UInt64,
  _ plaintext: UnsafePointer<UInt8>?,
  _ plaintextLength: Int
) -> OpenMusePairedBuffer

@_silgen_name("openmuse_paired_channel_open")
private func openMusePairedChannelOpen(
  _ channelHandle: UInt64,
  _ envelope: UnsafePointer<UInt8>?,
  _ envelopeLength: Int
) -> OpenMusePairedBuffer

@_silgen_name("openmuse_paired_native_handle_close")
private func openMusePairedNativeHandleClose(_ handle: UInt64) -> Int32

@main
@objc class AppDelegate: FlutterAppDelegate, FlutterImplicitEngineDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    precondition(openMuseDocxAbiVersion() == 1, "Unsupported OpenMuse DOCX ABI")
    precondition(openMusePairedAbiVersion() == 1, "Unsupported OpenMuse Paired ABI")
    precondition(openMuseOfficeViewersAbiVersion() == 1, "Unsupported Office Viewers ABI")
    let viewerProbe = openMuseXlsxInspect(nil, 0)
    precondition(viewerProbe.status != 0, "Office Viewers fail-closed probe failed")
    openMuseOfficeViewerBufferFree(viewerProbe)
    let slidesProbe = openMusePptxInspect(nil, 0)
    precondition(slidesProbe.status != 0, "Office Slides fail-closed probe failed")
    openMuseOfficeViewerBufferFree(slidesProbe)
    let pdfProbe = openMusePdfInspect(nil, 0)
    precondition(pdfProbe.status != 0, "Office PDF fail-closed probe failed")
    openMuseOfficeViewerBufferFree(pdfProbe)
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
          case "publicIdentity":
            let arguments = try dictionary(call.arguments)
            result(try publicIdentity(keyRef: try reference(arguments["keyRef"])))
          case "issueOffer":
            let arguments = try dictionary(call.arguments)
            result(
              try issueOffer(
                keyRef: try reference(arguments["keyRef"]),
                accountRef: try reference(arguments["accountRef"]),
                deviceRef: try reference(arguments["deviceRef"]),
                registrationGeneration: try generation(arguments["registrationGeneration"])
              )
            )
          case "beginHandshake":
            let arguments = try dictionary(call.arguments)
            result(
              try beginHandshake(
                keyRef: try reference(arguments["keyRef"]),
                localOfferJson: try json(arguments["localOfferJson"]),
                remoteOfferJson: try json(arguments["remoteOfferJson"]),
                localRegistrationJson: try json(arguments["localRegistrationJson"]),
                remoteRegistrationJson: try json(arguments["remoteRegistrationJson"])
              )
            )
          case "confirmHandshake":
            let arguments = try dictionary(call.arguments)
            result(
              try confirmHandshake(
                handle: try handle(arguments["handshakeHandle"]),
                code: try confirmationCode(arguments["confirmationCode"])
              )
            )
          case "channelSeal":
            let arguments = try dictionary(call.arguments)
            result(
              try channelSeal(
                handle: try handle(arguments["channelHandle"]),
                plaintext: try bytes(arguments["plaintext"])
              )
            )
          case "channelOpen":
            let arguments = try dictionary(call.arguments)
            result(
              FlutterStandardTypedData(
                bytes: try channelOpen(
                  handle: try handle(arguments["channelHandle"]),
                  envelopeJson: try json(arguments["envelopeJson"])
                )
              )
            )
          case "closeNativeHandle":
            let arguments = try dictionary(call.arguments)
            guard openMusePairedNativeHandleClose(try handle(arguments["handle"])) == 0
            else { throw KeyStoreError.operation }
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

  private static func publicIdentity(keyRef: String) throws -> [String: Any] {
    try withDeviceSeed(keyRef: keyRef) { seed in
      var output = [UInt8](repeating: 0, count: 64)
      defer {
        output.withUnsafeMutableBytes { buffer in
          buffer.initializeMemory(as: UInt8.self, repeating: 0)
        }
      }
      let status = seed.withUnsafeBytes { seedBuffer in
        output.withUnsafeMutableBytes { outputBuffer in
          openMusePairedDevicePublic(
            seedBuffer.bindMemory(to: UInt8.self).baseAddress,
            seedBuffer.count,
            outputBuffer.bindMemory(to: UInt8.self).baseAddress,
            outputBuffer.count
          )
        }
      }
      guard status == 0 else { throw KeyStoreError.operation }
      return [
        "signingPublic": Data(output[0..<32]),
        "agreementPublic": Data(output[32..<64]),
      ]
    }
  }

  private static func issueOffer(
    keyRef: String,
    accountRef: String,
    deviceRef: String,
    registrationGeneration: UInt64
  ) throws -> String {
    var nonce = Data(count: 32)
    guard nonce.withUnsafeMutableBytes({ buffer in
      SecRandomCopyBytes(kSecRandomDefault, buffer.count, buffer.baseAddress!)
    }) == errSecSuccess else { throw KeyStoreError.operation }
    defer { nonce.resetBytes(in: 0..<nonce.count) }
    let account = Data(accountRef.utf8)
    let device = Data(deviceRef.utf8)
    return try withDeviceSeed(keyRef: keyRef) { seed in
      let buffer = seed.withUnsafeBytes { seedBuffer in
        account.withUnsafeBytes { accountBuffer in
          device.withUnsafeBytes { deviceBuffer in
            nonce.withUnsafeBytes { nonceBuffer in
              openMusePairedIssueOffer(
                seedBuffer.bindMemory(to: UInt8.self).baseAddress,
                seedBuffer.count,
                accountBuffer.bindMemory(to: UInt8.self).baseAddress,
                accountBuffer.count,
                deviceBuffer.bindMemory(to: UInt8.self).baseAddress,
                deviceBuffer.count,
                nonceBuffer.bindMemory(to: UInt8.self).baseAddress,
                nonceBuffer.count,
                registrationGeneration
              )
            }
          }
        }
      }
      defer { openMusePairedBufferFree(buffer) }
      guard buffer.status == 0, let pointer = buffer.ptr,
        let value = String(bytes: UnsafeBufferPointer(start: pointer, count: buffer.len), encoding: .utf8)
      else { throw KeyStoreError.operation }
      return value
    }
  }

  private static func beginHandshake(
    keyRef: String,
    localOfferJson: String,
    remoteOfferJson: String,
    localRegistrationJson: String,
    remoteRegistrationJson: String
  ) throws -> String {
    let localOffer = Data(localOfferJson.utf8)
    let remoteOffer = Data(remoteOfferJson.utf8)
    let localRegistration = Data(localRegistrationJson.utf8)
    let remoteRegistration = Data(remoteRegistrationJson.utf8)
    return try withDeviceSeed(keyRef: keyRef) { seed in
      let buffer = seed.withUnsafeBytes { seedBytes in
        localOffer.withUnsafeBytes { localOfferBytes in
          remoteOffer.withUnsafeBytes { remoteOfferBytes in
            localRegistration.withUnsafeBytes { localRegistrationBytes in
              remoteRegistration.withUnsafeBytes { remoteRegistrationBytes in
                openMusePairedBeginHandshake(
                  seedBytes.bindMemory(to: UInt8.self).baseAddress, seedBytes.count,
                  localOfferBytes.bindMemory(to: UInt8.self).baseAddress, localOfferBytes.count,
                  remoteOfferBytes.bindMemory(to: UInt8.self).baseAddress, remoteOfferBytes.count,
                  localRegistrationBytes.bindMemory(to: UInt8.self).baseAddress,
                  localRegistrationBytes.count,
                  remoteRegistrationBytes.bindMemory(to: UInt8.self).baseAddress,
                  remoteRegistrationBytes.count
                )
              }
            }
          }
        }
      }
      guard let value = String(data: try consume(buffer), encoding: .utf8)
      else { throw KeyStoreError.operation }
      return value
    }
  }

  private static func confirmHandshake(handle: UInt64, code: String) throws -> String {
    let codeData = Data(code.utf8)
    let buffer = codeData.withUnsafeBytes { codeBytes in
      openMusePairedConfirmHandshake(
        handle,
        codeBytes.bindMemory(to: UInt8.self).baseAddress,
        codeBytes.count
      )
    }
    guard let value = String(data: try consume(buffer), encoding: .utf8)
    else { throw KeyStoreError.operation }
    return value
  }

  private static func channelSeal(handle: UInt64, plaintext: Data) throws -> String {
    guard !plaintext.isEmpty, plaintext.count <= 64 * 1024 else { throw KeyStoreError.invalid }
    let buffer = plaintext.withUnsafeBytes { plaintextBytes in
      openMusePairedChannelSeal(
        handle,
        plaintextBytes.bindMemory(to: UInt8.self).baseAddress,
        plaintextBytes.count
      )
    }
    guard let value = String(data: try consume(buffer), encoding: .utf8)
    else { throw KeyStoreError.operation }
    return value
  }

  private static func channelOpen(handle: UInt64, envelopeJson: String) throws -> Data {
    let envelope = Data(envelopeJson.utf8)
    let buffer = envelope.withUnsafeBytes { envelopeBytes in
      openMusePairedChannelOpen(
        handle,
        envelopeBytes.bindMemory(to: UInt8.self).baseAddress,
        envelopeBytes.count
      )
    }
    return try consume(buffer)
  }

  private static func consume(_ buffer: OpenMusePairedBuffer) throws -> Data {
    defer { openMusePairedBufferFree(buffer) }
    guard buffer.status == 0, let pointer = buffer.ptr else {
      throw KeyStoreError.operation
    }
    return Data(bytes: pointer, count: buffer.len)
  }

  private static func withDeviceSeed<T>(
    keyRef: String,
    operation: (Data) throws -> T
  ) throws -> T {
    try validateKeyRef(keyRef)
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: service,
      kSecAttrAccount as String: keyRef,
      kSecAttrSynchronizable as String: kCFBooleanFalse as Any,
      kSecReturnData as String: kCFBooleanTrue as Any,
      kSecMatchLimit as String: kSecMatchLimitOne,
    ]
    var item: CFTypeRef?
    guard SecItemCopyMatching(query as CFDictionary, &item) == errSecSuccess,
      var seed = item as? Data,
      seed.count == 32
    else { throw KeyStoreError.operation }
    defer { seed.resetBytes(in: 0..<seed.count) }
    return try operation(seed)
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
    try validateKeyRef(keyRef)
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

  private static func validateKeyRef(_ keyRef: String) throws {
    guard keyRef.range(of: #"^device-key:[0-9a-f]{64}$"#, options: .regularExpression) != nil
    else { throw KeyStoreError.invalid }
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

  private static func generation(_ value: Any?) throws -> UInt64 {
    guard let number = value as? NSNumber, number.int64Value > 0
    else { throw KeyStoreError.invalid }
    return number.uint64Value
  }

  private static func handle(_ value: Any?) throws -> UInt64 {
    guard let number = value as? NSNumber, number.int64Value > 0
    else { throw KeyStoreError.invalid }
    return number.uint64Value
  }

  private static func confirmationCode(_ value: Any?) throws -> String {
    guard let code = value as? String,
      code.range(of: #"^[0-9]{6}$"#, options: .regularExpression) != nil
    else { throw KeyStoreError.invalid }
    return code
  }

  private static func json(_ value: Any?) throws -> String {
    guard let json = value as? String, !json.isEmpty, json.utf8.count <= 64 * 1024
    else { throw KeyStoreError.invalid }
    return json
  }

  private static func bytes(_ value: Any?) throws -> Data {
    if let typed = value as? FlutterStandardTypedData { return typed.data }
    if let data = value as? Data { return data }
    throw KeyStoreError.invalid
  }

  private enum KeyStoreError: Error { case invalid, operation }
}
