//
//  Generated file. Do not edit.
//

import FlutterMacOS
import Foundation

import file_picker
import openmuse_dsh_plugin
import openmuse_resource_clipboard
import url_launcher_macos

func RegisterGeneratedPlugins(registry: FlutterPluginRegistry) {
  FilePickerPlugin.register(with: registry.registrar(forPlugin: "FilePickerPlugin"))
  OpenMuseDshWebViewPlugin.register(with: registry.registrar(forPlugin: "OpenMuseDshWebViewPlugin"))
  ResourceClipboardPlugin.register(with: registry.registrar(forPlugin: "ResourceClipboardPlugin"))
  UrlLauncherPlugin.register(with: registry.registrar(forPlugin: "UrlLauncherPlugin"))
}
