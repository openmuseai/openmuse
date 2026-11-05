# Office Plugin Platform 联动协议技术方案

版本 1.0

## 1. 文档目的

Embedded Application / Native View 解决了 Plugin 的渲染自治问题，但一个真正的 Office Plugin Platform 还需要解决 Host、Plugin、其他 Plugin 以及未来 AI Agent 之间如何联动。

本方案的目标是建立一套与 UI 技术栈无关、跨 Desktop / Mobile / Web 保持统一语义的 Platform Protocol。

核心思想：

> View Plane 负责渲染与输入；Platform Plane 负责协作与能力。

## 2. 总体模型

```text
                           Office Platform
                                │
              ┌─────────────────┴─────────────────┐
              │                                   │
          View Plane                       Platform Plane
              │                                   │
 Native View / UIView / iframe            Platform Broker
              │                                   │
 render / input / focus / IME      ┌──────────────┼──────────────┐
                                   │              │              │
                                Command         Event          Service
                                   │              │              │
                                   └──── Context / Capability ───┘
```

Plugin 的 UI/Renderer 可以完全自治，但所有平台级协作都通过 Broker。

## 3. 静态扩展：Contribution

插件通过 Manifest 声明它贡献给平台的静态能力。

```yaml
id: com.openmuse.docx
name: DOCX Editor
version: 1.0.0

activationEvents:
  - onFileType:docx

contributes:
  commands:
    - id: docx.insertTable
      title: Insert Table

  menus:
    editor.toolbar:
      - command: docx.insertTable
        when: activeEditor == "docx"

  keybindings:
    - command: document.save
      key: Ctrl+S
      when: activeEditor == "docx"

  editors:
    - id: docx.editor
      extensions: [".docx", ".dotx"]

  panels:
    - id: docx.outline
      title: Outline

  services:
    - id: document.convert
      formats: ["docx->pdf"]
```

Host 启动时扫描 Manifest 并构建 Contribution Registry。Host UI 不针对具体插件硬编码。

## 4. Context：动态条件系统

Contribution 的显示和可执行状态通过 Context 驱动：

```text
activeEditor == "docx"
document.readonly == false
selection.type == "table"
plugin.docx.ready == true
```

Plugin：

```text
setContext("selection.type", "table")
```

Host Manifest：

```yaml
when: activeEditor == "docx" && selection.type == "table"
```

Context Store 更新后重新计算 Menu、Toolbar、Keybinding 和 Command enablement。

## 5. Platform Broker

Broker 是平台的通信中心：

```text
                     Platform Broker
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
    Host Shell          DOCX Plugin         PDF Plugin
       │                   │                   │
       └──── Command / Event / Service ────────┘
```

Broker 管理：

- Command Registry
- Event Bus
- Service Registry
- Context Store
- Capability Registry
- Contribution Registry
- Permission Manager
- Routing
- Timeout / Cancellation

Host Shell 也只是 Participant，而不是所有交互的硬编码中转站。

## 6. Command

Command 表达“请求执行一个动作”。

```text
workspace.openResource
window.openSettings
document.save
git.showDiff
docx.insertTable
```

统一形式：

```text
executeCommand(commandId, arguments) -> result
```

Plugin 不直接调用另一个 Plugin：

```text
DOCX
  ↓ workspace.openResource(file.pdf)
Broker
  ↓ Editor Registry
PDF capability provider
  ↓
PDF Plugin
```

Toolbar、Menu、Keyboard、Command Palette、Plugin 和 AI Agent 都执行同一个 Command。

## 7. Event

Event 表达“某件事情已经发生”。

```text
theme.changed
window.stateChanged
activeEditor.changed
document.opened
document.dirtyChanged
selection.changed
plugin.activated
```

Event 是 publish/subscribe，不表达命令。

```text
Publisher
   ↓ emit
Event Bus
   ├─ Subscriber A
   ├─ Subscriber B
   └─ Subscriber C
```

订阅必须与 Plugin 生命周期绑定并自动释放。

## 8. Service

Service 表达可复用能力：

```text
filesystem.read
clipboard.write
fileDialog.open
ai.chat
git.diff
document.convert
storage.get
```

Service 必须具备：

```text
request / response
error
timeout
cancellation
permission
```

例如：

```text
DOCX Plugin
    ↓ service.call(fileDialog.open)
Broker
    ↓
Host FileDialog Service
    ↓
platform implementation
```

## 9. Command / Event / Service 的边界

```text
Command = 请做什么
Event   = 发生了什么
Service = 我需要一个能力
Context = 当前条件是什么
Capability = 谁能够提供什么
Contribution = 插件静态贡献什么
```

这六个概念构成 Platform Protocol 的核心语义。

## 10. Host Resize

几何 resize 不通过 Event Bus：

```text
Flutter layout
   ↓
PluginSlot Rect
   ↓
ViewManager.setBounds
   ↓
Native View / iframe / UIView
   ↓
Plugin Runtime
```

这是 View Plane。

只有语义状态才走 Event：

```text
fullscreen changed
window activated
theme changed
workspace changed
```

因此：

> Geometry → View Plane；Semantic State → Platform Plane。

## 11. Plugin 调用 Host

Host 能力通过 Service/Command 暴露，而不是让 Plugin 引用 Host 实现。

例如插入图片：

```text
DOCX UI
  ↓
fileDialog.open
  ↓
Broker
  ↓
Host FileDialog Service
  ↓
OS picker
```

同一 Plugin 在 Windows、macOS、Android、iOS、Web 上无需分别实现平台 Picker。

## 12. Plugin 调用 Plugin

默认禁止 implementation-level direct dependency。

错误：

```text
DOCX → PdfPlugin.convert()
```

正确：

```text
DOCX
  ↓ service.call(document.convert, docx→pdf)
Broker
  ↓ Capability Registry
Provider
```

Plugin 请求 Capability，而不是请求某个具体插件。

只有 Open With 等明确用户选择场景才指定 preferred provider。

## 13. RPC-friendly

从 V1 开始就采用可 RPC 化的边界：

```text
request
response
event
cancellation
stream (必要时)
```

不要暴露跨边界语言对象：

```text
&mut Document
QObject*
Flutter Widget
Rust trait object
```

示例：

```json
{
  "type": "request",
  "id": 42,
  "method": "service.call",
  "params": {
    "service": "filesystem",
    "method": "read",
    "args": {
      "uri": "file:///foo.docx"
    }
  }
}
```

因此 Transport 可以替换：

```text
Desktop V1  → in-process adapter / C ABI
Desktop V2  → IPC
Mobile      → internal bridge
Web         → MessageChannel/postMessage
```

Protocol 不变。

## 14. Desktop

View：

```text
Windows → HWND / child view
macOS   → NSView
Linux   → backend-specific view/surface
```

Plugin Runtime 可以是 Flutter、React/Chromium、Qt、wgpu、Skia、PDFium 等。

主要卡点：

- Native View 与 Flutter compositor 的 z-order；
- Overlay/Popup/Menu/Tooltip；
- Focus 与 IME；
- Keybinding 冲突；
- DPI / Retina / multi-monitor；
- Drag & Drop；
- 多 Runtime 内存和 GPU 成本；
- Linux Wayland；
- 同进程插件崩溃隔离。

## 15. Web

View：

```text
iframe / DOM container / Web Component / Canvas
```

通信：

```text
postMessage / MessageChannel / Worker messaging
```

Plugin 可以使用 React、Vue、Flutter Web、WASM、Canvas、WebGPU。

主要卡点：

- iframe sandbox；
- CSP/CORS；
- browser permissions；
- large payload copy；
- WASM/Worker 限制；
- Web API 与 Desktop Native API 不等价。

## 16. Mobile

View：

```text
Android View
iOS UIView
```

V1 建议所有 native Plugin 编译进 App：

```text
Office Mobile
├─ docx_plugin
├─ pdf_plugin
├─ pptx_plugin
└─ xlsx_plugin
```

运行时仍通过 PluginManager 和 Broker 激活。

若未来需要动态扩展，可考虑 WebView / JavaScript / WASM，但系统能力必须通过 Host Services 暴露。

主要卡点：

- iOS 动态代码与分发限制；
- App lifecycle；
- memory pressure；
- soft keyboard / IME；
- file permissions；
- sandbox。

## 17. Keybinding Resolution

统一解析：

```text
Key
+
Focus Owner
+
Context
+
Priority
    ↓
Command
```

例如：

```text
activeEditor == docx && focus == editor
Ctrl+B → docx.toggleBold
```

全局快捷键与 Plugin 快捷键必须经过同一 Resolver，避免各 Runtime 无条件抢键。

## 18. Drag & Drop

普通文件优先使用平台 DnD。

跨 Plugin 内部对象使用统一 DataTransfer：

```text
DataTransfer
├─ mime/type
├─ metadata
├─ uri/reference
└─ optional payload
```

大对象优先传 URI/Handle/Reference，而不是通过 Broker 复制二进制。

## 19. Lifecycle

建议状态机：

```text
installed
  ↓
registered
  ↓
activated
  ↓
active
  ↓
background
  ↓
suspended
  ↓
deactivated
  ↓
unloaded
```

Document 与 View 分离：

```text
DocumentSession
   ├─ View A
   └─ View B
```

这样可以支持多视图、Tab 切换、后台休眠和内存回收。

## 20. Capability 与 Permission

Plugin Manifest 声明：

```yaml
permissions:
  - filesystem.workspace
  - clipboard.write
  - network
  - ai.use
```

调用链：

```text
Plugin
  ↓
Broker
  ↓
Permission Manager
  ↓
Capability Registry
  ↓
Provider
```

统一安全语义，各平台实现自己的授权方式。

## 21. AI Agent

AI Agent 也作为 Platform Participant：

```text
AI
 ↓ discover commands/services
Broker
 ↓
Plugin
```

例如：

```text
AI → docx.insertTable → DOCX Plugin
AI → document.convert → provider
```

因此 Command/Service 应具备机器可发现的参数 Schema，并预留用户确认与权限控制。

## 22. Rust 模块

```text
platform/
├─ plugin/
├─ contribution/
├─ broker/
│  ├─ command.rs
│  ├─ event.rs
│  ├─ service.rs
│  ├─ context.rs
│  ├─ capability.rs
│  ├─ permission.rs
│  └─ routing.rs
├─ view/
├─ transport/
└─ services/
```

## 23. 十条架构原则

1. Everything is Contribution.
2. Everything actionable is Command.
3. Everything observable is Event.
4. Everything reusable is Service.
5. Everything conditional is Context.
6. Plugin talks to Platform, not Plugin.
7. View Plane != Platform Plane.
8. Protocol is cross-platform; Transport is platform-specific.
9. Protocol is RPC-friendly from day one.
10. Host is a participant; Platform Broker is the routing center.

## 24. 最终架构

```text
┌──────────────────────────────────────────────────────────────┐
│                      Office Platform                         │
│                                                              │
│  Flutter Host Shell                                          │
│  Window / Tabs / Toolbar / Sidebar / PluginSlot              │
│                 │                                            │
│                 ▼                                            │
│          Platform Broker (Rust)                              │
│  Command / Event / Service / Context / Capability / Permission│
│                 │                                            │
│      ┌──────────┼───────────┬───────────┐                    │
│      │          │           │           │                    │
│    DOCX        PDF         Git          AI                    │
│    Plugin      Plugin      Plugin       Agent                 │
│      │          │           │           │                    │
│      └────── Platform Protocol / SDK ───┘                    │
│                                                              │
│  View Plane:                                                 │
│  PluginSlot → ViewManager → platform View backend            │
│                           → plugin-owned renderer/input       │
└──────────────────────────────────────────────────────────────┘
```

最终边界：

> Host Shell 管工作区和平台 UI；View Manager 管插件放在哪里；Plugin 管自己的应用世界；Platform Broker 管所有参与者如何协作。
