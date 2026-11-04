#ifndef FLUTTER_PLUGIN_OPENMUSE_DSH_PLUGIN_H_
#define FLUTTER_PLUGIN_OPENMUSE_DSH_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

class DshView;

class OpenMuseDshWebViewPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows* registrar);
  explicit OpenMuseDshWebViewPlugin(flutter::PluginRegistrarWindows* registrar);
  ~OpenMuseDshWebViewPlugin() override;

 private:
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue>& call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);

  flutter::PluginRegistrarWindows* registrar_;
  std::unique_ptr<flutter::MethodChannel<flutter::EncodableValue>> channel_;
  std::shared_ptr<DshView> view_;
};

#endif
