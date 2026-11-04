import Flutter
import UIKit

@main
@objc class AppDelegate: FlutterAppDelegate, FlutterImplicitEngineDelegate, UIDocumentPickerDelegate {
  private var pendingSave: FlutterResult?

  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }

  func didInitializeImplicitFlutterEngine(_ engineBridge: FlutterImplicitEngineBridge) {
    GeneratedPluginRegistrant.register(with: engineBridge.pluginRegistry)
    let channel = FlutterMethodChannel(
      name: "cn.wps.pdf_rebuild_flutter/document_io",
      binaryMessenger: engineBridge.applicationRegistrar.messenger()
    )
    channel.setMethodCallHandler { [weak self] call, result in
      guard let self = self else { return }
      guard call.method == "savePdf" else {
        result(FlutterMethodNotImplemented)
        return
      }
      guard self.pendingSave == nil else {
        result(FlutterError(code: "busy", message: "A document save is already in progress", details: nil))
        return
      }
      guard let arguments = call.arguments as? [String: Any],
            let sourcePath = arguments["sourcePath"] as? String,
            FileManager.default.fileExists(atPath: sourcePath) else {
        result(FlutterError(code: "source", message: "PDF source file is missing", details: nil))
        return
      }
      guard let presenter = self.window?.rootViewController else {
        result(FlutterError(code: "picker", message: "Document picker is unavailable", details: nil))
        return
      }
      let source = URL(fileURLWithPath: sourcePath)
      let picker: UIDocumentPickerViewController
      if #available(iOS 14.0, *) {
        picker = UIDocumentPickerViewController(forExporting: [source], asCopy: true)
      } else {
        picker = UIDocumentPickerViewController(url: source, in: .exportToService)
      }
      picker.delegate = self
      self.pendingSave = result
      presenter.present(picker, animated: true)
    }
  }

  func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
    pendingSave?(true)
    pendingSave = nil
  }

  func documentPickerWasCancelled(_ controller: UIDocumentPickerViewController) {
    pendingSave?(false)
    pendingSave = nil
  }
}
