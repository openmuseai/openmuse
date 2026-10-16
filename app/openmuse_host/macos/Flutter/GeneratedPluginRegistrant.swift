//
//  Generated file. Do not edit.
//

import FlutterMacOS
import Foundation

import device_info_plus
import file_picker
import flutter_secure_storage_macos
import openmuse_dsh_plugin
import openmuse_file_viewer
import openmuse_native_text_gate
import package_info_plus
import url_launcher_macos

func RegisterGeneratedPlugins(registry: FlutterPluginRegistry) {
  DeviceInfoPlusMacosPlugin.register(with: registry.registrar(forPlugin: "DeviceInfoPlusMacosPlugin"))
  FilePickerPlugin.register(with: registry.registrar(forPlugin: "FilePickerPlugin"))
  FlutterSecureStoragePlugin.register(with: registry.registrar(forPlugin: "FlutterSecureStoragePlugin"))
  OpenMuseDshWebViewPlugin.register(with: registry.registrar(forPlugin: "OpenMuseDshWebViewPlugin"))
  OpenMuseFileViewerPlugin.register(with: registry.registrar(forPlugin: "OpenMuseFileViewerPlugin"))
  OpenMuseNativeTextPlugin.register(with: registry.registrar(forPlugin: "OpenMuseNativeTextPlugin"))
  FPPPackageInfoPlusPlugin.register(with: registry.registrar(forPlugin: "FPPPackageInfoPlusPlugin"))
  UrlLauncherPlugin.register(with: registry.registrar(forPlugin: "UrlLauncherPlugin"))
}
