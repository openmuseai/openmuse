#include "openmuse_dsh_plugin.h"

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>
#include <shlobj.h>
#include <wrl/client.h>
#include <wrl/event.h>

#include <WebView2.h>

#include <flutter/standard_method_codec.h>

#include <memory>
#include <string>

using Microsoft::WRL::Callback;
using Microsoft::WRL::ComPtr;

namespace {

std::wstring Utf16FromUtf8(const std::string& utf8) {
  if (utf8.empty()) return std::wstring();
  const int size = ::MultiByteToWideChar(
      CP_UTF8, MB_ERR_INVALID_CHARS, utf8.data(), static_cast<int>(utf8.size()),
      nullptr, 0);
  if (size <= 0) return std::wstring();
  std::wstring utf16(static_cast<size_t>(size), L'\0');
  const int written = ::MultiByteToWideChar(
      CP_UTF8, MB_ERR_INVALID_CHARS, utf8.data(), static_cast<int>(utf8.size()),
      utf16.data(), size);
  return written > 0 ? utf16 : std::wstring();
}

std::string Utf8FromUtf16(const wchar_t* utf16) {
  if (utf16 == nullptr) return std::string();
  const int input_length =
      static_cast<int>(wcsnlen(utf16, UNICODE_STRING_MAX_CHARS));
  const int target_length = ::WideCharToMultiByte(
      CP_UTF8, WC_ERR_INVALID_CHARS, utf16, input_length, nullptr, 0, nullptr,
      nullptr);
  if (target_length <= 0) return std::string();
  std::string utf8(static_cast<size_t>(target_length), '\0');
  const int written = ::WideCharToMultiByte(
      CP_UTF8, WC_ERR_INVALID_CHARS, utf16, input_length, utf8.data(),
      target_length, nullptr, nullptr);
  return written > 0 ? utf8 : std::string();
}

double ReadNumber(const flutter::EncodableMap& arguments, const char* key) {
  const auto found = arguments.find(flutter::EncodableValue(key));
  if (found == arguments.end()) return 0.0;
  if (const auto* value = std::get_if<double>(&found->second)) return *value;
  if (const auto* value = std::get_if<int32_t>(&found->second)) {
    return static_cast<double>(*value);
  }
  if (const auto* value = std::get_if<int64_t>(&found->second)) {
    return static_cast<double>(*value);
  }
  return 0.0;
}

int ParseLoopbackPort(const std::wstring& url) {
  const std::wstring scheme = L"http://";
  if (url.compare(0, scheme.size(), scheme) != 0) return 0;
  std::wstring rest = url.substr(scheme.size());
  std::wstring host;
  if (rest.compare(0, 10, L"127.0.0.1:") == 0) {
    host = L"127.0.0.1";
  } else if (rest.compare(0, 10, L"localhost:") == 0) {
    host = L"localhost";
  } else {
    return 0;
  }
  rest = rest.substr(host.size() + 1);
  int port = 0;
  for (const wchar_t ch : rest) {
    if (ch < L'0' || ch > L'9') break;
    port = port * 10 + (ch - L'0');
    if (port > 65535) return 0;
  }
  return port;
}

bool SourceMatches(const std::wstring& source, int port) {
  if (port <= 0) return false;
  const std::wstring prefixes[] = {
      L"http://127.0.0.1:" + std::to_wstring(port),
      L"http://localhost:" + std::to_wstring(port),
  };
  for (const auto& prefix : prefixes) {
    if (source.compare(0, prefix.size(), prefix) != 0) continue;
    if (source.size() == prefix.size()) return true;
    const wchar_t next = source[prefix.size()];
    if (next == L'/' || next == L'?' || next == L'#') return true;
  }
  return false;
}

std::wstring UserDataFolder() {
  PWSTR local = nullptr;
  std::wstring root;
  if (SUCCEEDED(::SHGetKnownFolderPath(FOLDERID_LocalAppData, KF_FLAG_CREATE,
                                       nullptr, &local)) &&
      local != nullptr) {
    root = local;
    ::CoTaskMemFree(local);
  } else {
    wchar_t temp[MAX_PATH];
    const DWORD length = ::GetTempPathW(MAX_PATH, temp);
    if (length == 0 || length >= MAX_PATH) return std::wstring();
    root = temp;
  }
  const std::wstring folder = root + L"\\OpenMuse\\WebView2";
  const int created = ::SHCreateDirectoryExW(nullptr, folder.c_str(), nullptr);
  if (created != ERROR_SUCCESS && created != ERROR_ALREADY_EXISTS &&
      created != ERROR_FILE_EXISTS) {
    return std::wstring();
  }
  return folder;
}

constexpr wchar_t kBridgeScript[] =
    L"window.MuseHostResource = { postMessage: function(raw) {"
    L" if (window.chrome && window.chrome.webview) {"
    L" window.chrome.webview.postMessage(String(raw));"
    L" }"
    L"} };";

}  // namespace

class DshView : public std::enable_shared_from_this<DshView> {
 public:
  DshView(flutter::MethodChannel<flutter::EncodableValue>* channel, HWND parent)
      : channel_(channel), parent_(parent) {}

  // Empty string means the view was accepted. Synchronous failures are
  // returned to the method channel; asynchronous WebView2 failures use Fail.
  std::string Show(RECT bounds, const std::wstring& url) {
    if (closed_) return "DSH WebView 已关闭";
    bounds_ = bounds;
    has_bounds_ = true;
    const int port = ParseLoopbackPort(url);
    if (port == 0) return "DSH 地址必须是本机 http loopback";
    if (url != url_) {
      url_ = url;
      port_ = port;
      navigate_pending_ = true;
    }
    if (failed_) {
      failed_ = false;
      creating_ = false;
    }
    if (!controller_) {
      if (!creating_) {
        const std::string error = BeginCreate();
        if (!error.empty()) return error;
      }
      return std::string();
    }
    controller_->put_IsVisible(TRUE);
    ApplyBounds();
    if (navigate_pending_) MaybeNavigate();
    return std::string();
  }

  void Hide() {
    if (controller_) controller_->put_IsVisible(FALSE);
  }

  void Reload() {
    if (webview_) webview_->Reload();
  }

  void Close() {
    closed_ = true;
    if (controller_) {
      controller_->Close();
      controller_ = nullptr;
    }
    webview_ = nullptr;
    environment_ = nullptr;
  }

 private:
  std::string BeginCreate() {
    creating_ = true;
    const std::wstring data = UserDataFolder();
    if (data.empty()) {
      creating_ = false;
      return "无法创建 WebView2 用户数据目录";
    }
    const auto weak = weak_from_this();
    const HRESULT hr = CreateCoreWebView2EnvironmentWithOptions(
        nullptr, data.c_str(), nullptr,
        Callback<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>(
            [weak](HRESULT result, ICoreWebView2Environment* env) -> HRESULT {
              const auto self = weak.lock();
              if (!self || self->closed_) return S_OK;
              if (FAILED(result) || env == nullptr) {
                self->Fail(result == HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND)
                               ? "未找到 WebView2 Runtime"
                               : "无法启动 WebView2");
                return S_OK;
              }
              self->environment_ = env;
              self->CreateController();
              return S_OK;
            })
            .Get());
    if (FAILED(hr)) {
      creating_ = false;
      return "无法启动 WebView2";
    }
    return std::string();
  }

  void CreateController() {
    if (!environment_ || parent_ == nullptr) {
      Fail("无法创建 DSH WebView2");
      return;
    }
    const auto weak = weak_from_this();
    const HRESULT hr = environment_->CreateCoreWebView2Controller(
        parent_,
        Callback<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>(
            [weak](HRESULT result,
                   ICoreWebView2Controller* controller) -> HRESULT {
              const auto self = weak.lock();
              if (!self || self->closed_) return S_OK;
              if (FAILED(result) || controller == nullptr) {
                self->Fail("无法创建 DSH WebView2");
                return S_OK;
              }
              self->OnControllerCreated(controller);
              return S_OK;
            })
            .Get());
    if (FAILED(hr)) Fail("无法创建 DSH WebView2");
  }

  void OnControllerCreated(ICoreWebView2Controller* controller) {
    controller_ = controller;
    creating_ = false;
    ComPtr<ICoreWebView2> webview;
    if (FAILED(controller->get_CoreWebView2(&webview)) || !webview) {
      Fail("无法创建 DSH WebView2");
      return;
    }
    webview_ = webview;
    ComPtr<ICoreWebView2Settings> settings;
    if (SUCCEEDED(webview_->get_Settings(&settings)) && settings) {
      settings->put_IsWebMessageEnabled(TRUE);
      settings->put_IsStatusBarEnabled(FALSE);
    }
    HookMessages();
    controller_->put_IsVisible(TRUE);
    ApplyBounds();
    MaybeNavigate();
  }

  void HookMessages() {
    const auto weak = weak_from_this();
    EventRegistrationToken token{};
    webview_->add_WebMessageReceived(
        Callback<ICoreWebView2WebMessageReceivedEventHandler>(
            [weak](ICoreWebView2*,
                   ICoreWebView2WebMessageReceivedEventArgs* args) -> HRESULT {
              const auto self = weak.lock();
              if (!self || self->closed_ || args == nullptr) return S_OK;
              self->OnWebMessage(args);
              return S_OK;
            })
            .Get(),
        &token);
  }

  void OnWebMessage(ICoreWebView2WebMessageReceivedEventArgs* args) {
    LPWSTR source = nullptr;
    if (FAILED(args->get_Source(&source)) || source == nullptr) return;
    const bool allowed = SourceMatches(source, port_);
    ::CoTaskMemFree(source);
    if (!allowed) return;
    LPWSTR message = nullptr;
    if (FAILED(args->TryGetWebMessageAsString(&message)) || message == nullptr) {
      return;
    }
    const bool too_long = wcsnlen(message, 16385) > 16384;
    const std::string payload = too_long ? std::string() : Utf8FromUtf16(message);
    ::CoTaskMemFree(message);
    if (too_long || payload.empty() || channel_ == nullptr) return;
    channel_->InvokeMethod(
        "resourceOpen", std::make_unique<flutter::EncodableValue>(payload));
  }

  void MaybeNavigate() {
    if (!webview_ || url_.empty()) return;
    if (!script_added_) {
      if (script_adding_) return;
      script_adding_ = true;
      const auto weak = weak_from_this();
      webview_->AddScriptToExecuteOnDocumentCreated(
          kBridgeScript,
          Callback<
              ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler>(
              [weak](HRESULT result, LPCWSTR) -> HRESULT {
                const auto self = weak.lock();
                if (!self || self->closed_) return S_OK;
                self->script_adding_ = false;
                if (FAILED(result)) {
                  self->Fail("无法注入 DSH 页面桥接");
                  return S_OK;
                }
                self->script_added_ = true;
                self->navigate_pending_ = false;
                if (self->webview_ && !self->url_.empty()) {
                  self->webview_->Navigate(self->url_.c_str());
                }
                return S_OK;
              })
              .Get());
      return;
    }
    navigate_pending_ = false;
    webview_->Navigate(url_.c_str());
  }

  void ApplyBounds() {
    if (!controller_ || !has_bounds_) return;
    controller_->put_Bounds(bounds_);
  }

  void Fail(const std::string& message) {
    failed_ = true;
    creating_ = false;
    if (closed_ || channel_ == nullptr) return;
    channel_->InvokeMethod(
        "failed", std::make_unique<flutter::EncodableValue>(message));
  }

  flutter::MethodChannel<flutter::EncodableValue>* channel_;
  HWND parent_;
  ComPtr<ICoreWebView2Environment> environment_;
  ComPtr<ICoreWebView2Controller> controller_;
  ComPtr<ICoreWebView2> webview_;
  std::wstring url_;
  RECT bounds_{};
  int port_ = 0;
  bool has_bounds_ = false;
  bool creating_ = false;
  bool failed_ = false;
  bool closed_ = false;
  bool navigate_pending_ = false;
  bool script_added_ = false;
  bool script_adding_ = false;
};

void OpenMuseDshWebViewPlugin::RegisterWithRegistrar(
    flutter::PluginRegistrarWindows* registrar) {
  auto plugin = std::make_unique<OpenMuseDshWebViewPlugin>(registrar);
  registrar->AddPlugin(std::move(plugin));
}

OpenMuseDshWebViewPlugin::OpenMuseDshWebViewPlugin(
    flutter::PluginRegistrarWindows* registrar)
    : registrar_(registrar) {
  channel_ = std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
      registrar->messenger(), "com.openmuse.dsh/webview",
      &flutter::StandardMethodCodec::GetInstance());
  channel_->SetMethodCallHandler([this](const auto& call, auto result) {
    HandleMethodCall(call, std::move(result));
  });
}

OpenMuseDshWebViewPlugin::~OpenMuseDshWebViewPlugin() {
  channel_->SetMethodCallHandler(nullptr);
  if (view_) {
    view_->Close();
    view_.reset();
  }
}

void OpenMuseDshWebViewPlugin::HandleMethodCall(
    const flutter::MethodCall<flutter::EncodableValue>& call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (call.method_name() == "hide") {
    if (view_) view_->Hide();
    result->Success();
    return;
  }
  if (call.method_name() == "reload") {
    if (view_) view_->Reload();
    result->Success();
    return;
  }
  if (call.method_name() != "show") {
    result->NotImplemented();
    return;
  }
  const auto* arguments = std::get_if<flutter::EncodableMap>(call.arguments());
  if (arguments == nullptr) {
    result->Error("invalid-arguments", "Expected a bounds map");
    return;
  }
  const auto found = arguments->find(flutter::EncodableValue("url"));
  if (found == arguments->end()) {
    result->Error("invalid-arguments", "Missing url");
    return;
  }
  const auto* url = std::get_if<std::string>(&found->second);
  if (url == nullptr || url->empty()) {
    result->Error("invalid-arguments", "Missing url");
    return;
  }
  const HWND parent = registrar_->GetView()->GetNativeWindow();
  if (parent == nullptr) {
    result->Error("webview-failed", "Flutter 窗口尚未就绪");
    return;
  }
  if (!view_) {
    view_ = std::make_shared<DshView>(channel_.get(), parent);
  }
  const auto atLeastOne = [](double value) -> LONG {
    const auto rounded = static_cast<LONG>(value);
    return rounded < 1 ? 1 : rounded;
  };
  RECT bounds;
  bounds.left = static_cast<LONG>(ReadNumber(*arguments, "x"));
  bounds.top = static_cast<LONG>(ReadNumber(*arguments, "y"));
  bounds.right = bounds.left + atLeastOne(ReadNumber(*arguments, "width"));
  bounds.bottom = bounds.top + atLeastOne(ReadNumber(*arguments, "height"));
  const std::string error = view_->Show(bounds, Utf16FromUtf8(*url));
  if (!error.empty()) {
    result->Error("webview-failed", error);
    return;
  }
  result->Success();
}
