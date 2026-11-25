import Flutter
import UIKit
public class ResourceClipboardPlugin: NSObject, FlutterPlugin {
  private static let kind="com.openmuse.resource-reference"
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel=FlutterMethodChannel(name:"io.openmuse/resource_clipboard",binaryMessenger:registrar.messenger())
    registrar.addMethodCallDelegate(ResourceClipboardPlugin(),channel:channel)
  }
  public func handle(_ call: FlutterMethodCall,result: @escaping FlutterResult) {
    switch call.method {
    case "write":
      guard let args=call.arguments as? [String:Any],let text=args["text"] as? String,
        let reference=args["reference"] as? String,reference.utf8.count<=8192 else {
        result(FlutterError(code:"INVALID_REFERENCE",message:"Invalid reference",details:nil));return
      }
      UIPasteboard.general.setItems([["public.utf8-plain-text":text,Self.kind:Data(reference.utf8)]],options:[.localOnly:true])
      result(nil)
    case "read":
      result(UIPasteboard.general.data(forPasteboardType:Self.kind).flatMap{String(data:$0,encoding:.utf8)})
    default:result(FlutterMethodNotImplemented)
    }
  }
}
