//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <flutter_secure_storage_windows/flutter_secure_storage_windows_plugin.h>
#include <openmuse_dsh_plugin/openmuse_dsh_plugin_c_api.h>
#include <openmuse_native_text_gate/openmuse_native_text_gate_plugin_c_api.h>

void RegisterPlugins(flutter::PluginRegistry* registry) {
  FlutterSecureStorageWindowsPluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("FlutterSecureStorageWindowsPlugin"));
  OpenmuseDshPluginCApiRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("OpenmuseDshPluginCApi"));
  OpenmuseNativeTextGatePluginCApiRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("OpenmuseNativeTextGatePluginCApi"));
}
