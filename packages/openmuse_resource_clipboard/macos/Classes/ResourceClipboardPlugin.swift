import FlutterMacOS
import AppKit
public class ResourceClipboardPlugin:NSObject,FlutterPlugin {
  private static let kind=NSPasteboard.PasteboardType("com.openmuse.resource-reference")
  public static func register(with registrar:FlutterPluginRegistrar) {
    let channel=FlutterMethodChannel(name:"io.openmuse/resource_clipboard",binaryMessenger:registrar.messenger)
    registrar.addMethodCallDelegate(ResourceClipboardPlugin(),channel:channel)
  }
  public func handle(_ call:FlutterMethodCall,result:@escaping FlutterResult) {
    switch call.method {
    case "write":
      guard let args=call.arguments as? [String:Any],let text=args["text"] as? String,
        let reference=args["reference"] as? String,reference.utf8.count<=8192 else {
        result(FlutterError(code:"INVALID_REFERENCE",message:"Invalid reference",details:nil));return
      }
      let item=NSPasteboardItem();item.setString(text,forType:.string);item.setString(reference,forType:Self.kind)
      NSPasteboard.general.clearContents();NSPasteboard.general.writeObjects([item]);result(nil)
    case "read":result(NSPasteboard.general.string(forType:Self.kind))
    default:result(FlutterMethodNotImplemented)
    }
  }
}
