# Flutter + Rust Office
嵌入式应用插件与平台联动（Embedded Application Plugin Platform）技术方案

版本 2.0 · 面向桌面端优先的可热插拔 Office / Viewer / Editor 架构

本文档整理一套面向 Flutter + Rust Office Suite 的完整插件平台架构：Host 负责工作区、窗口容器、平台 Broker、生命周期与公共服务；Plugin 是自治应用，拥有自己的 UI Runtime、渲染引擎和输入处理。Host 不规定 Flutter、React、Qt、Skia、wgpu 等技术栈，也不参与插件逐帧渲染。Host 与 Plugin、Plugin 与 Plugin 之间通过配置驱动的 Contribution 与统一 Platform Protocol（Command / Event / Service / Context / Capability）联动。

# 1. 目标与核心结论

- Office Host 本身采用 Flutter + Rust，但插件不被限制为 Flutter + Rust。
- DOCX、PPTX、XLSX、PDF、CAD、代码编辑器、File Viewer 等可以作为独立插件开发、发布和升级。
- 插件负责其文档模型、布局、渲染、交互、光标、选择、滚动、IME 等内部能力。
- Host 只提供 PluginSlot（占位区域）和 Native View 的嵌入/布局能力，以及极薄的生命周期与业务控制协议。
- 插件渲染输出直接进入其自己的 Native View；OS 窗口系统负责裁剪、命中测试、焦点和输入路由。
- 插件可以独立运行（Standalone Mode），也可以被 Host 嵌入（Embedded Mode），尽量复用同一套代码。
- 不把 GPU Texture Sharing 作为第一版基础设施；它仅作为未来对特殊合成需求的可选后端。
# 2. 非目标

- 不设计统一的 Office Render API，不要求插件输出 DisplayList、Texture 或像素 Buffer。
- 不要求 Host 理解 Paragraph、Cell、Slide、Shape、Selection 等插件内部语义。
- 第一版不要求插件跨进程 GPU Texture Sharing、Fence、共享内存或复杂 Compositor。
- 不通过 Rust trait 直接形成跨动态库 ABI；跨边界协议必须稳定、语言无关。
- 不强求所有平台使用完全相同的底层窗口句柄；统一的是上层 Embedded View 语义。
# 3. 总体架构

```text
┌─────────────────────────────────────────────────────────────┐
│                     Office Host                            │
│                    Flutter + Rust                          │
│                                                           │
│  Flutter Shell                                            │
│  ┌──────────┬───────────────────────────────────────────┐  │
│  │ Sidebar  │ Toolbar / Tabs                            │  │
│  │          ├───────────────────────────────────────────┤  │
│  │          │ PluginSlot                                │  │
│  │          │ ┌───────────────────────────────────────┐ │  │
│  │          │ │      Plugin Native View               │ │  │
│  │          │ │                                       │ │  │
│  │          │ │  Flutter / React / Qt / wgpu / ...   │ │  │
│  │          │ └───────────────────────────────────────┘ │  │
│  └──────────┴───────────────────────────────────────────┘  │
│                         │                                  │
│                  Rust View Manager                         │
│                         │                                  │
│             Native Window / View Hierarchy                 │
└─────────────────────────┼───────────────────────────────────┘
                          │
             ┌────────────┴─────────────┐
             │                          │
        View Plane                 Control Plane
             │                          │
        Native View              Thin IPC / C ABI
             │                          │
             └────────────┬─────────────┘
                          ▼
                    Plugin Runtime
```

# 4. 核心抽象：Plugin 是 Embedded Application

插件不是“渲染函数”，而是一个可被嵌入的完整应用（Embedded Application）。它可以拥有自己的事件循环、Flutter Engine、Chromium/WebView、Qt Runtime、Skia/wgpu renderer，甚至完全不同的语言实现。

```text
Host 看见的：
    PluginView / NativeViewHandle

Plugin 看见的：
    My Window / My View / My Runtime

Host 不需要知道：
    Flutter Engine?
    React + Chromium?
    Qt?
    Skia?
    wgpu?
    PDFium?
```

# 5. 为什么插件内容会严格出现在 Host 指定区域

关键不是 Flutter 的 Widget 裁剪，而是操作系统的 Window/View hierarchy。Host 的 Flutter PluginSlot 计算出屏幕/窗口坐标和尺寸，Rust View Manager 将插件 Native View 作为 Host 原生窗口体系中的子 View/子 Window，并持续同步其 bounds。

```text
Flutter PluginSlot
    │  Rect(x, y, width, height)
    ▼
Rust ViewManager
    │
    ├─ attach(native_view, container)
    └─ set_bounds(native_view, rect)
             │
             ▼
      OS Window/View Hierarchy
             │
             ▼
      Plugin Native View
```

因此裁剪、可见区域、基本 z-order、焦点归属和命中测试首先由平台窗口系统完成。插件只知道自己的 View 当前大小，不需要知道它位于 Office 的第几个 Tab 或 Sidebar 右侧。

# 6. View 的所有权：推荐 Plugin 创建，Host Attach

为了兼容不同 UI Runtime，推荐插件创建自己真正需要的 Native View，Host 只负责 attach。这样 Flutter、Chromium、Qt、wgpu 等都可以按照各自 Runtime 的生命周期创建窗口/视图。

```text
Plugin
   │ create_view()
   ▼
NativeViewHandle
   │
   ▼
Host ViewManager
   │ attach()
   ▼
PluginSlot / Native Container
```

相比“Host 创建一个固定 Surface 再要求所有插件在其中绘制”，该模型对异构插件更自然。

# 7. Standalone Mode 与 Embedded Mode

同一个插件应尽量支持两种运行模式，以降低开发和测试成本。

| 维度 | Standalone Mode | Embedded Mode |
| --- | --- | --- |
| 顶层容器 | 插件自己创建 Top-level Window | Host 提供嵌入位置，插件 View 被 attach |
| 渲染引擎 | 插件自己的 | 完全相同 |
| 输入 | OS → Plugin | OS → Plugin Native View |
| 业务控制 | 本地 App 控制 | Host Control Plane |
| 核心代码 | 复用 | 复用 |

# 8. 不同技术栈插件如何工作

## 8.1 Flutter + Rust Plugin

```text
Plugin
  ├─ Flutter UI
  │    ↓
  │  Flutter Engine / Impeller
  │    ↓
  │  Native View
  └─ Rust Engine
       ├─ parser
       ├─ document model
       ├─ layout
       └─ domain logic
```

Standalone 时 Flutter View 位于插件自己的顶层窗口；Embedded 时同一 Flutter View 被 Host attach 到 PluginSlot。Flutter Engine 仍负责自身 Widget、动画、文本输入、手势和绘制。

## 8.2 React Plugin

```text
React
  ↓
Chromium / WebView Runtime
  ↓
Native View
  ↓
Host PluginSlot
```

Host 不需要理解 DOM 或 React。插件 Runtime 接收平台输入并渲染到自己的 Native View。若插件本质是 Web UI，可用系统 WebView 或自带 Chromium Runtime，具体取决于产品体积、兼容性和安全策略。

## 8.3 Qt / wgpu / Skia / PDFium 等

原则完全一致：插件自行拥有 renderer 和 native view。Host 不暴露统一 GPU API，因此不会强迫 PDFium、CAD/wgpu、代码编辑器等改造成某一种绘制模型。

# 9. 输入事件：Host 为什么可以基本不管

当 Plugin Native View 真正进入 OS 的窗口/视图层级后，输入由平台窗口系统根据命中测试和焦点直接分发给该 View。

```text
Mouse / Touch / Wheel
        │
        ▼
   OS Hit Testing
        │
        ▼
Plugin Native View
        │
        ├─ Flutter → GestureBinding / Widgets
        ├─ Chromium → DOM / React
        ├─ Qt → Qt Event Loop
        └─ wgpu app → plugin event loop

Keyboard / IME
        │
        ▼
  Focused Native View
        │
        ▼
      Plugin
```

因此 Control Plane 不应承载 PointerMove、KeyDown、IME Composition 等高频事件。Host 只在需要全局快捷键、跨插件 Drag & Drop、统一菜单等功能时参与协调。

# 10. Host 的职责边界

| Host 负责 | Host 不负责 |
| --- | --- |
| Plugin discovery / install / enable / disable | 文档内部 Layout |
| PluginSlot 与 Native View attach/detach | 插件逐帧 Rendering |
| 位置、尺寸、显示、隐藏、Tab 生命周期 | 鼠标/键盘/IME 的常规转发 |
| 插件崩溃/退出检测（若为独立进程） | 插件内部 Selection / Cursor |
| 可选 Host Services | 强制统一 Flutter/React/Qt 技术栈 |
| 少量业务控制协议 | 统一 GPU Texture API |

# 11. View Plane 与 Control Plane 分离

```text
                     Plugin
                ┌────────┴────────┐
                │                 │
            View Plane       Control Plane
                │                 │
          Native OS View     Stable Protocol
                │                 │
       render / input        open / save / close
                             activate / theme / state
```

View Plane 追求自治和低耦合；Control Plane 只解决 Office 工作区需要协调的业务动作。两者必须解耦，避免把渲染热路径变成 Host RPC。

# 12. 建议的最小 Control Protocol

```text
Plugin Lifecycle
- initialize()
- activate()
- deactivate()
- shutdown()

Document
- open(uri)
- close(document_id)
- save(document_id)
- save_as(document_id, uri)

State
- get_title()
- is_dirty()
- can_close()

Host → Plugin notifications
- theme_changed()
- locale_changed()
- app_activated()
- app_deactivated()
```

协议可以使用 C ABI（同进程动态库）或 IPC（独立进程）。上层语义保持一致，从而允许未来把高风险插件从 in-process 平滑迁移为 out-of-process。

# 13. Host Services：统一能力，而不是统一 Renderer

Host 可以向插件提供可选公共能力，但这些能力应与渲染技术栈无关。插件在 Standalone Mode 下可以使用自己的默认实现，在 Embedded Mode 下可注入 Host 实现。

- FileSystem / Virtual File System
- Clipboard
- Logger / Telemetry
- Theme / Localization
- Command Registry
- Notification
- Authentication / Account
- AI service
- Git service
- Recent files / Workspace
- Permission / capability checks
```text
Plugin Core
   │
   ├─ Renderer Runtime        ← plugin-owned
   │
   └─ PlatformServices
        │
        ├─ StandaloneServices
        └─ HostServicesAdapter
```

# 14. Plugin Manifest

```text
{
  "id": "com.openmuse.docx",
  "name": "DOCX Editor",
  "version": "1.0.0",
  "pluginApiVersion": 1,
  "entry": "openmuse-docx",
  "runtime": "embedded-app",
  "fileTypes": ["docx", "dotx"],
  "capabilities": [
    "document.open",
    "document.edit",
    "document.save"
  ],
  "view": {
    "kind": "native"
  }
}
```

Manifest 用于发现和能力匹配，不描述插件内部 renderer。Host 根据 fileTypes/capabilities 选择插件，然后创建或启动其 Embedded View。

# 15. Rust Host 内部模块建议

```text
office_core/
├─ plugin/
│  ├─ plugin_manager.rs
│  ├─ manifest.rs
│  ├─ registry.rs
│  ├─ lifecycle.rs
│  └─ control.rs
│
├─ view/
│  ├─ view_manager.rs
│  ├─ native_handle.rs
│  ├─ windows.rs
│  ├─ macos.rs
│  └─ linux.rs
│
├─ services/
│  ├─ filesystem.rs
│  ├─ clipboard.rs
│  ├─ theme.rs
│  └─ command.rs
│
└─ ffi/
   └─ flutter_bridge.rs
```

# 16. Flutter Host API 建议

```text
class PluginView extends StatefulWidget {
  final String pluginId;
  final String? documentUri;
}

PluginView lifecycle:
  mount
    → create/activate plugin
    → obtain NativeView
    → attach

  layout changed
    → update bounds

  visibility changed
    → show/hide

  dispose
    → detach
    → release view
```

Flutter 只把 Widget 几何信息和生命周期交给 Rust View Manager。不要在 Dart 层实现插件的渲染协议。

# 17. 平台适配

| 平台 | Host 容器 | Plugin View | 重点风险 |
| --- | --- | --- | --- |
| Windows | HWND hierarchy / child container | HWND / framework-owned child view | focus、DPI、IME、z-order、popup |
| macOS | NSWindow/NSView hierarchy | NSView / FlutterView / framework view | first responder、layer、menu、IME |
| Linux | X11/Wayland backend-specific | native surface/view | Wayland 嵌入模型差异最大 |
| Web | DOM container | iframe / Web component / Web app | 不能复用 HWND/NSView 模型 |

# 18. Web 方案

Web 不存在 HWND/NSView，因此不能把 Desktop Native View 机制原样复制。应统一“Embedded Application”概念，而不是统一底层句柄。

```text
Embedded Application
        │
   ┌────┴───────────────┐
   │                    │
Desktop                Web
   │                    │
Native Child View     iframe / DOM container
   │                    │
Flutter/Qt/wgpu       React / Flutter Web / WASM
```

因此 Plugin API 的跨平台稳定层应是 create/attach/resize/show/hide/destroy 等语义；NativeViewHandle 只属于 Desktop backend。

# 19. 热插拔生命周期

1. 扫描插件目录或插件注册表，读取 Manifest。
1. 校验 pluginApiVersion、平台、架构、签名与权限。
1. 注册 fileTypes/capabilities。
1. 用户打开文件时选择对应插件。
1. 启动/加载插件并初始化 Control Plane。
1. 插件 create_view()，返回 Native View 或等价句柄。
1. Host 将 View attach 到当前 PluginSlot。
1. Flutter 布局变化时 Host 同步 bounds。
1. 切换 Tab 时 show/hide 或 detach/reattach。
1. 关闭文档时先询问 can_close/is_dirty，再释放 View 和 Document。
1. 禁用/升级插件时确保没有活跃 View，完成 shutdown 后卸载/替换。
# 20. 同进程与独立进程的演进路线

| 阶段 | 方案 | 适用目的 |
| --- | --- | --- |
| V1 | in-process 动态库 + 稳定 C ABI + Native View | 最快验证架构与产品体验 |
| V2 | 部分插件 out-of-process + IPC Control Plane | 故障隔离、安全隔离 |
| V3 | 按需引入共享资源/高级 compositor | 只有 profiling/产品需求证明必要时 |

如果插件本身需要完整 Flutter Runtime，独立进程往往比在同一进程中管理多个复杂 Runtime 更容易隔离；但是否从 V1 就采用进程隔离，应由 PoC 的平台行为和部署需求决定。

# 21. 为什么第一版不采用 GPU Texture Sharing

- Shared Texture 需要跨进程 GPU resource handle、同步 primitive、frame scheduling、device lost 恢复等基础设施。
- 输入仍然需要 Host/协议参与，因为插件没有独立的 OS View 接收常规输入。
- 不同平台需要 D3D/Metal/IOSurface/DMA-BUF/AHardwareBuffer 等不同实现。
- Web 无法原样复用 native external texture handle。
- 对于 Office 主文档区这种大矩形独占 View，Native View 的组合能力通常已经足够。
Shared Texture 仍可保留为未来 View Backend：例如必须与 Flutter 内容进行复杂透明混合、Transform 或统一 compositor 时再引入。

# 22. 主要风险与限制

| 风险 | 影响 | 应对 |
| --- | --- | --- |
| Native View 与 Flutter 合成边界 | 复杂 Clip/Opacity/Transform/Overlay 可能不自然 | 将 PluginSlot 限定为矩形主内容区；避免花式 Flutter 变换 |
| z-order / popup | Host 菜单、浮层可能被 Native View 遮挡 | PoC 验证平台策略；必要时使用独立原生 popup 或调整层级 |
| Focus / IME | Office 编辑体验的关键路径 | Windows/macOS 首批 PoC 必测中文 IME、焦点切换、快捷键 |
| Drag & Drop | 跨 Host/Plugin 需要协调 | 定义少量跨边界 DnD 协议或依赖 OS 原生 DnD |
| Accessibility | 不同 Runtime 各自暴露语义树 | 优先让插件 Runtime 直接接入 OS accessibility；Host 管工作区级语义 |
| 插件崩溃 | 同进程会拖垮 Host | V2 对第三方/高风险插件启用独立进程 |
| Wayland | 跨进程/跨 toolkit 嵌入能力限制更明显 | Linux 单独做技术验证，不假设等价于 Win32 |
| 资源占用 | 多个 Flutter/Chromium Runtime 成本高 | 按 Tab 激活/休眠；复用 runtime 的优化放到后续 |

# 23. 性能模型

该方案的核心优势是渲染数据不需要经过 Host。DOCX Plugin 的 Layout/Render 直接进入自身 GPU/Native View，Host 只在布局变化时更新窗口几何信息。因此文档滚动、动画、光标闪烁等高频路径不会形成 Host↔Plugin 的逐帧 RPC。

```text
高频路径：
OS → Plugin View → Plugin Runtime → Renderer → GPU

低频控制路径：
Host → open/save/theme/lifecycle → Plugin

低频布局路径：
Flutter PluginSlot → Rect → ViewManager → Native View bounds
```

对于超大文档，性能仍主要由插件自身的增量解析、增量布局、虚拟化、缓存和局部重绘决定；Host 嵌入层不应成为主要瓶颈。

# 24. 安全模型

- 插件包必须有 Manifest、版本与 API 兼容性检查。
- 第三方插件建议支持签名校验和来源策略。
- Host Services 采用 capability/permission 模型，不默认暴露所有文件、网络、账号能力。
- in-process 插件视为与 Host 同权限；不能提供真正安全隔离。
- 需要不可信插件生态时，应升级为独立进程并限制 IPC 能力与文件访问。
# 25. 推荐 PoC

在继续设计复杂协议之前，先验证最关键的工程假设。

1. 实现 Flutter + Rust Host：左侧 Flutter Sidebar，右侧固定 PluginSlot。
1. Plugin A：独立 Flutter + Rust 应用；Standalone 可运行，Embedded 时提供 Native View。
1. Plugin B：React + WebView/Chromium 示例，用于验证异构 Runtime。
1. Windows：验证 attach、resize、hide/show、detach/reattach、DPI、多显示器。
1. macOS：验证 NSView attach、first responder、resize、Retina scale。
1. 两平台重点验证鼠标、滚轮、键盘、中文 IME、Clipboard、Drag & Drop。
1. 验证 Flutter Host 的菜单、Dialog、Tooltip、Popup 与 Plugin Native View 的 z-order。
1. 连续切换 1000 次 Tab/attach/detach，检查崩溃、泄漏、黑屏和焦点丢失。
1. 记录 CPU/GPU/内存，确认多 Runtime 的资源成本。
1. PoC 通过后再固定 Plugin SDK 与 Manifest v1。
# 26. 验收标准

- Plugin A/B 在 Standalone 和 Embedded 两种模式下复用主要 UI/业务代码。
- Host 不参与插件逐帧渲染。
- Host 不需要转发普通鼠标、滚轮、键盘、IME 事件。
- PluginSlot resize 后插件稳定跟随，无明显闪烁和黑帧。
- Windows/macOS 的中文 IME、Clipboard、Focus 可用。
- 插件可 detach 后重新 attach 到另一个 Tab/Slot。
- 插件禁用/升级时能可靠释放 View 与资源。
- Host API 不出现 Flutter/React/wgpu/PDFium 等 renderer-specific 类型。
- 至少两种不同 UI Runtime 的插件证明架构不是 Flutter-only。
# 27. 建议的 V1 Plugin SDK 边界

```text
// 伪接口：语义示意，不作为 Rust ABI 直接发布

interface EmbeddedPlugin {
    PluginInfo info();

    void initialize(HostServices? services);

    DocumentHandle open(Uri uri);
    void close(DocumentHandle doc);

    NativeViewHandle createView(DocumentHandle doc);
    void destroyView(NativeViewHandle view);

    bool canClose(DocumentHandle doc);
    bool save(DocumentHandle doc);

    void activate();
    void deactivate();
    void shutdown();
}

interface HostViewManager {
    void attach(NativeViewHandle view, SlotId slot);
    void setBounds(NativeViewHandle view, Rect rect);
    void show(NativeViewHandle view);
    void hide(NativeViewHandle view);
    void detach(NativeViewHandle view);
}
```

真正跨动态库时使用 C ABI；真正跨进程时使用 IPC schema。上面的接口只表达稳定语义，不应直接作为 Rust trait ABI。

# 28. 典型 DOCX 插件示例

```text
DOCX Plugin
├─ app/
│  ├─ Flutter UI
│  └─ Standalone / Embedded bootstrap
├─ engine/
│  └─ Rust
│     ├─ OOXML parser
│     ├─ document model
│     ├─ layout
│     ├─ editing
│     └─ persistence
└─ platform/
   ├─ windows view bridge
   ├─ macOS view bridge
   └─ plugin control adapter

Standalone:
  bootstrap → top-level window → FlutterView

Embedded:
  bootstrap → FlutterView → NativeViewHandle → Host attach

两种模式：
  Flutter UI + Rust engine 基本相同
```

# 29. 架构决策总结

| 决策项 | 结论 |
| --- | --- |
| 插件定义 | Embedded Application，而不是 Render Plugin |
| 渲染所有权 | Plugin |
| 输入所有权 | Plugin Native View + OS |
| Host UI | Flutter Shell / Workspace |
| Host 渲染职责 | 不参与插件内容渲染 |
| Host 窗口职责 | PluginSlot、attach、bounds、visibility、lifecycle |
| 跨技术栈 | 通过 Native View，而非统一 renderer |
| 统一依赖 | Host Services，可选注入 |
| Desktop | Native View/Child Window |
| Web | iframe/DOM container 等 Embedded App backend |
| V1 隔离 | 优先简单实现；稳定 C ABI |
| 未来隔离 | 可升级独立进程 + IPC |
| GPU Texture Sharing | 非 V1 基础设施，仅作为特殊需求的可选 backend |

# 30. 最终架构图

```text
                         OpenMuse / Office Host
                              Flutter
                                 │
                      Workspace / PluginSlot
                                 │
                         Rust ViewManager
                                 │
               ┌─────────────────┴──────────────────┐
               │                                    │
          View Plane                           Control Plane
               │                                    │
      Native View Hierarchy                 Stable API / IPC
               │                                    │
     ┌─────────┼──────────┬───────────┐             │
     │         │          │           │             │
   DOCX      PDF        Code         Git            │
   Plugin    Plugin     Plugin       Plugin          │
     │         │          │           │             │
  Flutter   PDFium     Helix/wgpu   React/WebView   │
  + Rust                                              
     │         │          │           │
     └─────────┴──────────┴───────────┘
                     │
              OS Window System
                     │
        render / input / focus / IME

Host Services (optional):
File · Clipboard · Theme · Command · AI · Git · Auth · Logging

原则：
Host 管“容器和协作”；Plugin 管“自己的应用世界”。
```

# 31. 下一阶段建议

优先实现 Windows + macOS 双平台最小 PoC，同时实现 Platform Broker 的最小闭环：Manifest/Contribution、Command、Event、Service、Context、ViewManager。PoC 必须同时证明两件事：第一，异构 Native View 能稳定完成 attach、resize、focus、IME、popup/z-order 与 detach/reattach；第二，Host、Flutter Plugin、React/Web Plugin 能通过同一 Broker 完成 Command、Event、Service 与 Context 联动。完成后再冻结 Plugin SDK v1。Linux/Wayland、Mobile 与 Web 作为不同 View/Transport backend 继续验证，但保持同一 Platform Protocol。


## 30. Plugin Platform：从“嵌入应用”升级为“可联动平台”

Native View / Embedded Application 只解决“插件如何拥有独立 UI 与 Renderer”。完整 Office 插件平台还必须解决：

- Host 如何声明和发现 Plugin 提供的静态能力；
- Host 与 Plugin 如何进行动态交互；
- Plugin 如何调用 Host 能力；
- Plugin 如何间接调用其他 Plugin 的能力而不形成强依赖；
- Host resize、主题切换、激活状态等如何传递；
- Desktop、Mobile、Web 如何保持统一语义而允许不同底层实现。

因此整体架构增加第二条独立平面：

```text
                           Office Platform
                                │
              ┌─────────────────┴─────────────────┐
              │                                   │
          View Plane                       Platform Plane
              │                                   │
   Native View / iframe / View           Platform Broker / Bus
              │                                   │
      render / input / IME       ┌───────────────┼───────────────┐
                                 │               │               │
                              Command          Event           Service
                                 │               │               │
                                 └─────── Context / Capability ──┘
```

核心原则：

> View Plane 解决“怎么显示、怎么输入”；Platform Plane 解决“怎么协作”。

两条平面不能混在一起。高频 Render/Input 不应经过 Platform Bus，业务联动也不应依赖 Renderer。

## 31. 静态联动：Everything is Contribution

Host 的菜单、工具栏、编辑器关联、快捷键、Panel、Command 等不应该通过针对插件 ID 的硬编码生成，而应由插件 Manifest 声明。

例如：

```yaml
id: com.openmuse.docx
name: DOCX Editor
version: 1.0.0

activationEvents:
  - onFileType:docx

fileTypes:
  - docx
  - dotx

contributes:
  commands:
    - id: docx.insertTable
      title: Insert Table
    - id: docx.exportPdf
      title: Export PDF

  menus:
    editor.toolbar:
      - command: docx.insertTable
        group: insert
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

Host 启动：

```text
scan plugins
    ↓
read manifest
    ↓
Contribution Registry
    ├─ Command Registry
    ├─ Menu Registry
    ├─ Keybinding Registry
    ├─ Editor Registry
    ├─ Panel Registry
    └─ Service/Capability Registry
    ↓
Host Shell 根据配置生成 UI 与路由
```

这意味着 Host 不需要：

```text
if plugin == docx:
    add_docx_toolbar()
```

Plugin 只声明“我贡献什么”，Host 负责“这些贡献在平台中如何呈现”。

## 32. Context：配置驱动 UI 的动态条件

仅有静态 Manifest 不够。Office UI 大量依赖运行时状态，例如：

```text
activeEditor == "docx"
document.readonly == false
selection.type == "table"
workspace.trusted == true
plugin.docx.ready == true
```

因此平台需要 `ContextKey` 系统。

Plugin 可以更新 Context：

```text
setContext("selection.type", "table")
setContext("document.readonly", false)
```

Manifest 使用 `when` 表达式：

```yaml
- command: docx.mergeCells
  when: activeEditor == "docx" && selection.type == "table"
```

执行链：

```text
DOCX Plugin
    │
    │ setContext(selection.type = table)
    ▼
Context Store
    │
    ├─ reevaluate menus
    ├─ reevaluate toolbar
    ├─ reevaluate keybindings
    └─ reevaluate command enablement
```

Host 不需要理解 DOCX Table 的内部模型，只需要理解一个字符串化、可组合的 Context Contract。

## 33. 动态联动：Platform Broker

建议在 Rust Core 中引入独立 `Platform Broker`，不要让 Flutter Host 自己承担所有插件通信。

```text
                         Flutter Host
                              │
                       Platform Broker
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
     Host Shell            Plugin A              Plugin B
        │                     │                     │
        └──────── Command / Event / Service ────────┘
```

Broker 负责：

```text
Command Registry
Service Registry
Event Bus
Context Store
Capability Registry
Contribution Registry
Permission Manager
Routing
Cancellation / Timeout
```

Host Shell 本身也只是一个 Platform Participant。因此通信模型不是：

```text
Plugin A → Host → Plugin B
```

而是：

```text
Plugin A → Platform Broker → Capability/Command Provider
```

这使 Host UI、插件、未来 AI Agent 都能使用同一套平台能力。

## 34. Command：一切“动作”统一成命令

Command 表达“我要让某件事情发生”。

典型 Command：

```text
workspace.openResource
workspace.closeEditor
window.openSettings
window.showNotification
document.save
git.showDiff
docx.insertTable
```

统一接口：

```text
executeCommand(commandId, arguments) -> result
```

例如 DOCX Plugin 想打开生成后的 PDF：

```json
{
  "command": "workspace.openResource",
  "arguments": {
    "uri": "file:///tmp/result.pdf"
  }
}
```

路由：

```text
DOCX Plugin
    ↓ executeCommand(workspace.openResource)
Platform Broker
    ↓
Workspace
    ↓ query Editor Registry
*.pdf → PDF capability
    ↓
activate PDF Plugin
    ↓
create/open view
    ↓
new editor tab
```

DOCX Plugin 不依赖 PDF Plugin 的实现，也不需要知道 PDF Plugin 的 ID。

### Host 自己也使用 Command

Host 的 Menu、Toolbar、Keybinding、Command Palette 不应该分别调用业务函数，而应统一执行 Command：

```text
Toolbar ───────────────┐
Menu ──────────────────┤
Keyboard ──────────────┤
Command Palette ───────┼──→ Command Registry → Handler
Plugin ────────────────┤
AI Agent ──────────────┘
```

因此可以形成：

> Everything actionable is Command.

## 35. Event：一切“已发生的状态变化”统一发布

Event 表达事实，而不是请求。

例如：

```text
theme.changed
window.stateChanged
activeEditor.changed
document.opened
document.closed
document.dirtyChanged
selection.changed
plugin.activated
workspace.changed
```

模型：

```text
Publisher
   │ emit(event)
   ▼
Event Bus
   │
   ├─ subscriber A
   ├─ subscriber B
   └─ subscriber C
```

Event 应支持订阅生命周期，避免插件销毁后留下悬空订阅。

原则：

> Command 是“请做什么”；Event 是“发生了什么”。

## 36. Service：一切“可复用能力”通过服务发现

Service 表达可请求、可返回结果的能力。

例如：

```text
filesystem.read
filesystem.write
clipboard.read
clipboard.write
fileDialog.open
ai.chat
git.diff
document.convert
storage.get
```

Service 与 Command 的区别：

```text
Command = 用户/系统动作语义
Service = 可复用能力语义
Event   = 状态变化通知
```

Service 调用需要明确：

```text
request
response
error
timeout
cancellation
permission
```

例如：

```text
DOCX Plugin
    │ service.call("fileDialog.open")
    ▼
Platform Broker
    ▼
Host FileDialog Service
    ▼
Windows / macOS / Android / iOS / Web implementation
```

Plugin 不需要分别实现 NSOpenPanel、Windows Picker、Android SAF、iOS UIDocumentPicker 等平台代码。

## 37. Plugin 不直接依赖 Plugin

禁止把跨插件协作设计为：

```text
DOCX Plugin → import PDF Plugin API
```

否则长期会形成依赖网：

```text
DOCX → PDF
DOCX → AI
DOCX → Git
PPTX → Image
XLSX → Chart
PDF → AI
...
```

正确模型：

```text
                   Platform Broker
              ┌─────────┼──────────┐
              │         │          │
            DOCX       PDF        Git
              │         │          │
              └─ Capability/Service ┘
```

Plugin 请求的是 Intent 或 Capability，例如：

```text
workspace.openResource(uri)
document.convert(from, to)
ai.chat(...)
git.diff(...)
```

默认不指定 provider。Broker 根据 Registry、用户偏好、权限和当前平台选择 provider。

只有 `Open With...` 等明确场景才允许指定 preferred provider。

## 38. Resize 的正确分层

Host resize 同时包含两种完全不同的信息。

### 38.1 几何 Resize：属于 View Plane

```text
Host Flutter layout
      ↓
PluginSlot Rect changed
      ↓
ViewManager.setBounds()
      ↓
Native View / iframe / mobile View resize
      ↓
Plugin Runtime 自然收到 viewport/layout change
```

这不应该通过 Event Bus 高频发送。

Flutter Plugin 会自然看到新的 View constraints/MediaQuery；Web Plugin 会看到 viewport resize；Qt/wgpu 也通过各自平台窗口机制收到尺寸变化。

### 38.2 语义状态：属于 Platform Plane

例如：

```text
window entered fullscreen
window became inactive
theme changed
workspace changed
```

这类状态才发送 Event：

```json
{
  "event": "window.stateChanged",
  "data": {
    "fullscreen": true
  }
}
```

原则：

> Geometry → View Plane；Semantic State → Platform Plane。

## 39. Host Services：Host 与 Plugin 的标准能力边界

建议 V1 Host Services 至少包含：

```text
workspace
window
filesystem
fileDialog
clipboard
storage
theme
localization
command
notification
logging
authentication
permission
```

未来可以扩展：

```text
AI
Git
Search
Index
Collaboration
Versioning
Export/Convert
```

Plugin SDK 示例：

```typescript
interface PluginContext {
  commands: CommandAPI;
  events: EventAPI;
  services: ServiceAPI;
  context: ContextAPI;
  workspace: WorkspaceAPI;
  window: WindowAPI;
  storage: StorageAPI;
}
```

插件内部：

```typescript
async function activate(ctx: PluginContext) {
  ctx.commands.register("docx.exportPdf", exportPdf);

  ctx.events.subscribe("theme.changed", onThemeChanged);

  ctx.context.set("docx.ready", true);
}
```

这套 API 与插件使用 Flutter、React、Qt、Rust/wgpu 无关。

## 40. RPC-friendly Protocol：从第一天就为跨进程准备

即使 V1 使用同进程 dylib，也不应该设计：

```rust
fn document() -> &mut Document
```

这样的共享内存对象 API。

边界应天然是：

```text
request
response
notification/event
stream (必要时)
cancellation
```

例如：

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

V1：

```text
Plugin SDK → in-process adapter → Rust Broker
```

V2：

```text
Plugin SDK → IPC adapter → Rust Broker
```

Web：

```text
Plugin SDK → MessageChannel/postMessage → Broker
```

业务协议不变，只替换 Transport。

## 41. 跨平台一致性原则

跨平台统一的不是 Native Handle 或 Transport，而是 Platform Protocol。

统一：

```text
Manifest
Contribution
Command
Event
Service
Context
Capability
Lifecycle
Permission
```

平台特定：

```text
View Backend
Transport
Sandbox
Runtime Loading
Process Model
Native Services implementation
```

因此：

```text
                     Plugin Protocol
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
    Desktop              Mobile               Web
       │                   │                   │
 Native View           UIView/View        iframe/DOM
 dylib/process         built-in/WASM      Worker/WASM
 C ABI/IPC             internal bridge    postMessage
```

## 42. Desktop 方案

Desktop 是插件自由度最高的平台。

Plugin 可以是：

```text
Flutter + Rust
React + Chromium/WebView
Qt
Rust + wgpu
C++ + Skia
PDFium
独立 executable
动态库
```

View Backend：

```text
Windows → HWND / child native view
macOS   → NSView / framework-owned view
Linux   → backend-specific native surface/view
```

Platform Plane：

```text
V1: C ABI / in-process adapter
V2: IPC / local socket / platform channel
```

### Desktop 主要技术卡点

- Flutter 与 Native View 的 z-order/composition；
- Popup、Menu、Tooltip、Dialog 与插件 View 的层级；
- Focus、快捷键抢占、中文/日文/韩文 IME；
- DPI、Retina、多显示器；
- Drag & Drop；
- 插件崩溃隔离；
- 多 Flutter/Chromium Runtime 的 CPU/GPU/内存成本；
- Linux Wayland 的嵌入能力与 Windows/macOS 不完全等价。

## 43. Web 方案

Web 没有 HWND/NSView，但 Embedded Application 概念仍然成立。

View Backend：

```text
iframe
DOM container
Web Component
Canvas/WebGPU
```

推荐第三方/强隔离插件优先：

```text
Host
  └─ iframe
       └─ Plugin Web App
```

Platform Plane：

```text
postMessage
MessageChannel
Worker messaging
```

Plugin 可以是：

```text
React
Vue
Flutter Web
WASM
Canvas
WebGPU
```

Command/Event/Service/Context 语义可以与 Desktop 保持一致。

Web 的主要卡点：

- iframe sandbox 与权限；
- CSP/CORS；
- Clipboard/File System 等浏览器权限模型；
- 跨 origin 通信；
- 大文件传输与 Structured Clone 成本；
- WASM/Worker 的线程和共享内存限制；
- Web 插件不能获得 Desktop Native API，只能通过 Host Services。

## 44. Mobile 方案

Mobile 最大限制不是 View，而是动态代码部署。

View Backend 可以自然映射为：

```text
Android → View / SurfaceView / Platform View
iOS     → UIView
```

但桌面式“下载一个 dylib/可执行程序并运行”的插件生态不适合作为 Mobile 基线。

### V1 推荐：Built-in Plugin

```text
Office Mobile App
├─ docx_plugin
├─ pdf_plugin
├─ pptx_plugin
└─ xlsx_plugin
```

所有 native 插件编译进 App，但运行时仍通过同一个 PluginManager：

```text
discover/register
activate
deactivate
command/event/service
```

因此上层架构仍然是 Plugin，只是 Deployment Model 不同。

### 动态扩展

若未来需要动态插件，可考虑受限 Runtime：

```text
WebView
JavaScript
WASM
```

这类插件访问系统能力必须经过 Host Services / Permission Manager。

Mobile 主要卡点：

- iOS 动态代码与 App Store 分发限制；
- Android/iOS 生命周期；
- 前后台切换；
- 内存压力导致 Runtime 回收；
- Keyboard/IME 与软键盘 viewport；
- 文件访问权限模型；
- 第三方插件的安全沙箱。

## 45. Keybinding Resolution

Office Host 与插件都会注册快捷键，因此必须有统一解析器。

例如：

```text
Ctrl+P
Ctrl+B
Ctrl+Shift+P
```

建议解析输入：

```text
Keybinding
+
Context
+
Focus Owner
+
Priority
```

然后得到 Command：

```text
Keyboard
   ↓
Keybinding Resolver
   ↓
Context evaluation
   ↓
Command Registry
   ↓
executeCommand()
```

例如：

```text
activeEditor == docx && focus == editor
    Ctrl+B → docx.toggleBold

focus == global
    Ctrl+Shift+P → workbench.commandPalette
```

不要让每个 Plugin 在 OS 层无条件抢全局快捷键。

## 46. 跨 Plugin Drag & Drop

普通文件拖放优先利用 OS/Browser DnD。

对于平台内部对象，需要统一 `DataTransfer`：

```text
DataTransfer
├─ mime/type
├─ metadata
├─ URI/reference
└─ optional payload
```

例如：

```text
application/x-openmuse-image-ref
application/x-openmuse-document-fragment
text/plain
text/uri-list
```

跨插件大对象尽量传 URI/Handle/Reference，而不是在 Platform Bus 中复制大块二进制。

## 47. Plugin 生命周期与多 Runtime 资源控制

多个 Flutter Engine、Chromium Runtime 同时常驻成本很高，因此生命周期不能只有 loaded/unloaded。

建议：

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
    │
    ├─ View A
    └─ View B
```

关闭一个 Tab/View 不一定销毁 DocumentSession。

Host 可以根据：

```text
visible
active
memory pressure
last used
background policy
```

决定暂停或销毁 Plugin View。

## 48. Capability 与 Permission

Service Registry 不仅解决发现，也必须解决权限。

例如 Plugin 声明：

```yaml
permissions:
  - filesystem.workspace
  - clipboard.write
  - network
  - ai.use
```

Broker 调用：

```text
Plugin
  ↓ service.call
Permission Manager
  ↓
Capability Registry
  ↓
Provider
```

这样 Desktop、Mobile、Web 可以共享同一个安全语义，而由各平台实现实际授权机制。

## 49. AI Agent 作为 Platform Participant

如果未来 Office 是 AI-native，统一 Command/Service 模型会直接产生额外收益。

AI 不需要理解每个插件内部 API，只需要发现：

```text
commands
services
capabilities
schemas
```

例如：

```text
AI Agent
   ↓
executeCommand("docx.insertTable", ...)
   ↓
Platform Broker
   ↓
DOCX Plugin
```

或者：

```text
AI Agent
   ↓
service.call("document.convert", ...)
   ↓
Capability Provider
```

因此 Command/Service Schema 应尽量机器可发现、参数结构化，并预留权限确认机制。

## 50. Rust Platform Runtime 建议目录

```text
platform/
├─ plugin/
│  ├─ manager.rs
│  ├─ manifest.rs
│  ├─ activation.rs
│  └─ lifecycle.rs
│
├─ contribution/
│  ├─ registry.rs
│  ├─ menu.rs
│  ├─ keybinding.rs
│  └─ editor.rs
│
├─ broker/
│  ├─ command.rs
│  ├─ event.rs
│  ├─ service.rs
│  ├─ context.rs
│  ├─ capability.rs
│  ├─ permission.rs
│  └─ routing.rs
│
├─ view/
│  ├─ manager.rs
│  ├─ native_handle.rs
│  ├─ windows.rs
│  ├─ macos.rs
│  ├─ linux.rs
│  ├─ mobile.rs
│  └─ web.rs
│
├─ transport/
│  ├─ in_process.rs
│  ├─ ipc.rs
│  └─ web_message.rs
│
└─ services/
   ├─ filesystem.rs
   ├─ clipboard.rs
   ├─ dialog.rs
   ├─ theme.rs
   ├─ storage.rs
   └─ notification.rs
```

## 51. 完整架构原则

1. **Everything is Contribution**  
   Menu、Toolbar、Editor、Panel、Keybinding、Command 等静态扩展全部配置驱动。

2. **Everything actionable is Command**  
   Host、用户、Plugin、AI Agent 使用同一 Command Registry。

3. **Everything observable is Event**  
   已发生的状态变化通过 Event 发布。

4. **Everything reusable is Service**  
   File、Clipboard、AI、Git、Convert 等能力通过 Service Registry 暴露。

5. **Everything conditional is Context**  
   UI 显隐、Command enablement、Keybinding 匹配由 Context/when 表达式驱动。

6. **Plugin talks to Platform, not Plugin**  
   Plugin 依赖 Intent/Capability，不直接依赖其他 Plugin implementation。

7. **View Plane != Platform Plane**  
   Render/Input 走 View Backend；业务协作走 Platform Broker。

8. **Protocol is cross-platform; Transport is platform-specific**  
   Desktop/Mobile/Web 统一语义，不强求统一 native 技术。

9. **Protocol is RPC-friendly from day one**  
   即使 V1 同进程，也不暴露共享语言对象作为跨边界契约。

10. **Host is a participant, not the center of every interaction**  
    Platform Broker 才是路由中心，Host Shell 与 Plugin 都是 Participant。

## 52. 完整目标架构

```text
┌──────────────────────────────────────────────────────────────────┐
│                         Office Platform                          │
│                                                                  │
│  ┌────────────────────── Host Shell ──────────────────────────┐  │
│  │ Flutter: Window / Tabs / Sidebar / Toolbar / PluginSlot    │  │
│  └────────────────────────────┬───────────────────────────────┘  │
│                               │                                  │
│                    ┌──────────▼──────────┐                       │
│                    │   Platform Broker   │                       │
│                    │       (Rust)        │                       │
│                    ├─────────────────────┤                       │
│                    │ Command Registry    │                       │
│                    │ Event Bus           │                       │
│                    │ Service Registry    │                       │
│                    │ Context Store       │                       │
│                    │ Capability Registry │                       │
│                    │ Permission Manager  │                       │
│                    └──────────┬──────────┘                       │
│                               │                                  │
│      ┌────────────────────────┼──────────────────────────┐       │
│      │                        │                          │       │
│  DOCX Plugin              PDF Plugin                Git Plugin  │
│  Flutter+Rust             PDFium/wgpu               React/Web   │
│      │                        │                          │       │
│      └──────── Platform Protocol / SDK ─────────────────┘       │
│                                                                  │
│  View Plane:                                                     │
│  PluginSlot → ViewManager → NativeView / UIView / iframe        │
│                            → Plugin-owned Renderer/Input          │
└──────────────────────────────────────────────────────────────────┘
```

最终边界可以概括为：

> Host Shell 管工作区与平台级 UI；View Manager 管“放在哪里”；Plugin 管“里面是什么”；Platform Broker 管“大家如何协作”。

