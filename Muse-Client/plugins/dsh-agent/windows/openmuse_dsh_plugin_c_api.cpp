#include "include/openmuse_dsh_plugin/openmuse_dsh_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "openmuse_dsh_plugin.h"

void OpenmuseDshPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  OpenMuseDshWebViewPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
