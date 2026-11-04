#include "workspace_channel.h"

#include <flutter/encodable_value.h>
#include <flutter/method_channel.h>
#include <flutter/standard_method_codec.h>
#include <shobjidl.h>
#include <windows.h>

#include <memory>
#include <string>

#include "utils.h"

namespace {

std::unique_ptr<flutter::MethodChannel<flutter::EncodableValue>> g_channel;

std::wstring Utf16FromUtf8(const std::string& utf8) {
  if (utf8.empty()) {
    return std::wstring();
  }
  const int size = ::MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS,
                                         utf8.data(),
                                         static_cast<int>(utf8.size()),
                                         nullptr, 0);
  if (size <= 0) {
    return std::wstring();
  }
  std::wstring utf16(static_cast<size_t>(size), L'\0');
  const int written = ::MultiByteToWideChar(
      CP_UTF8, MB_ERR_INVALID_CHARS, utf8.data(),
      static_cast<int>(utf8.size()), utf16.data(), size);
  if (written <= 0) {
    return std::wstring();
  }
  return utf16;
}

class DialogRelease {
 public:
  explicit DialogRelease(IFileOpenDialog* dialog) : dialog_(dialog) {}
  ~DialogRelease() {
    if (dialog_ != nullptr) {
      dialog_->Release();
    }
  }
  IFileOpenDialog* get() const { return dialog_; }

 private:
  IFileOpenDialog* dialog_;
};

class ItemRelease {
 public:
  explicit ItemRelease(IShellItem* item) : item_(item) {}
  ~ItemRelease() {
    if (item_ != nullptr) {
      item_->Release();
    }
  }
  IShellItem* get() const { return item_; }

 private:
  IShellItem* item_;
};

void ChooseDirectory(
    HWND owner,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  IFileOpenDialog* created = nullptr;
  const HRESULT created_hr = ::CoCreateInstance(
      CLSID_FileOpenDialog, nullptr, CLSCTX_INPROC_SERVER,
      IID_PPV_ARGS(&created));
  if (FAILED(created_hr) || created == nullptr) {
    result->Error("dialog-failed", "无法创建文件夹选择器");
    return;
  }
  DialogRelease dialog(created);

  DWORD options = 0;
  if (FAILED(dialog.get()->GetOptions(&options))) {
    result->Error("dialog-failed", "无法读取文件夹选择器选项");
    return;
  }
  options |= FOS_PICKFOLDERS | FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST |
             FOS_NOCHANGEDIR;
  if (FAILED(dialog.get()->SetOptions(options))) {
    result->Error("dialog-failed", "无法配置文件夹选择器");
    return;
  }
  dialog.get()->SetTitle(L"选择工作区文件夹");

  const HRESULT shown = dialog.get()->Show(owner);
  if (shown == HRESULT_FROM_WIN32(ERROR_CANCELLED)) {
    result->Success();
    return;
  }
  if (FAILED(shown)) {
    result->Error("dialog-failed", "无法打开文件夹选择器");
    return;
  }

  IShellItem* chosen = nullptr;
  if (FAILED(dialog.get()->GetResult(&chosen)) || chosen == nullptr) {
    result->Error("dialog-failed", "无法读取所选文件夹");
    return;
  }
  ItemRelease item(chosen);

  PWSTR path = nullptr;
  if (FAILED(item.get()->GetDisplayName(SIGDN_FILESYSPATH, &path)) ||
      path == nullptr) {
    result->Error("dialog-failed", "无法读取所选文件夹路径");
    return;
  }
  const std::string utf8 = Utf8FromUtf16(path);
  ::CoTaskMemFree(path);
  if (utf8.empty()) {
    result->Error("dialog-failed", "所选文件夹路径无效");
    return;
  }
  result->Success(flutter::EncodableValue(utf8));
}

void RevealPath(
    HWND owner,
    const flutter::MethodCall<flutter::EncodableValue>& call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  const auto* arguments = std::get_if<flutter::EncodableMap>(call.arguments());
  if (arguments == nullptr) {
    result->Error("invalid-arguments", "Missing path");
    return;
  }
  const auto found = arguments->find(flutter::EncodableValue("path"));
  if (found == arguments->end()) {
    result->Error("invalid-arguments", "Missing path");
    return;
  }
  const auto* path = std::get_if<std::string>(&found->second);
  if (path == nullptr || path->empty()) {
    result->Error("invalid-arguments", "Missing path");
    return;
  }
  const std::wstring wide = Utf16FromUtf8(*path);
  if (wide.empty() || wide.find(L'"') != std::wstring::npos) {
    result->Error("invalid-arguments", "Invalid path");
    return;
  }
  const std::wstring parameters = L"/select,\"" + wide + L"\"";
  const auto launched = reinterpret_cast<intptr_t>(::ShellExecuteW(
      owner, L"open", L"explorer.exe", parameters.c_str(), nullptr,
      SW_SHOWNORMAL));
  if (launched <= 32) {
    result->Error("reveal-failed", "无法在资源管理器中显示该路径");
    return;
  }
  result->Success();
}

void HandleWorkspaceCall(
    HWND owner,
    const flutter::MethodCall<flutter::EncodableValue>& call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (call.method_name() == "chooseDirectory") {
    ChooseDirectory(owner, std::move(result));
    return;
  }
  if (call.method_name() == "reveal") {
    RevealPath(owner, call, std::move(result));
    return;
  }
  result->NotImplemented();
}

}  // namespace

void RegisterWorkspaceChannel(flutter::BinaryMessenger* messenger, HWND owner) {
  g_channel = std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
      messenger, "com.openmuse.host/workspace",
      &flutter::StandardMethodCodec::GetInstance());
  g_channel->SetMethodCallHandler(
      [owner](const flutter::MethodCall<flutter::EncodableValue>& call,
              std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>>
                  result) { HandleWorkspaceCall(owner, call, std::move(result)); });
}

void UnregisterWorkspaceChannel() {
  if (g_channel) {
    g_channel->SetMethodCallHandler(nullptr);
    g_channel.reset();
  }
}
