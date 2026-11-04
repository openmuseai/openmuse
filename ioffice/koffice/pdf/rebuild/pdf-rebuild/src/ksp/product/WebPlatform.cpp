#include "core/fxge/cfx_gemodule.h"
#include "core/fxge/systemfontinfo_iface.h"

#include <memory>

// Browser builds have no filesystem font directory. PDFium can still render
// embedded fonts and its built-in substitutions; a browser font provider can
// be added here without changing the product ABI.
class WebPlatform final : public CFX_GEModule::PlatformIface {
 public:
  void Init() override {}
  void Terminate() override {}

  std::unique_ptr<SystemFontInfoIface> CreateDefaultSystemFontInfo() override {
    return nullptr;
  }
};

std::unique_ptr<CFX_GEModule::PlatformIface>
CFX_GEModule::PlatformIface::Create() {
  return std::make_unique<WebPlatform>();
}
