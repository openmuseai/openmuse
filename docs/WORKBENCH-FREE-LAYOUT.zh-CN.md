# Workbench 自由布局

## 范围

首期支持主窗口内运行时切分、关闭、绑定、移动、相邻交换和尺寸调整。系统级独立窗口只保留位置与迁移协议，不在本期创建原生窗口。

## 架构约束

- `PaneNode` / `SplitNode` 只描述布局；Pane 不持有插件 Widget，也不执行渲染。
- `SurfaceBinding` 将 Pane 绑定到稳定的 `instanceRef`。绑定可在运行时移动或交换。
- Surface Widget 作为根 `Stack` 的稳定子节点，以 `instanceRef` 为 Key。交换位置只更新 `Positioned` 几何，不改变 Surface 的 Element/State 或 PlatformView 所属关系。
- 编辑区由 `EditorGroup` 表示。每个组独立保存标签页与活动资源，资源打开命令路由到当前 focused group。
- 布局写入 `layout-v1.json`。旧侧栏宽度只在首次没有布局文件时用于默认布局迁移。

## 操作语义

- 横向切分：在目标 Pane 右侧创建空 Pane。
- 纵向切分：在目标 Pane 下方创建空 Pane。
- 关闭：空 Pane 直接关闭；有绑定的 Pane 必须确认，确认后销毁该 Surface 会话。
- 移动到空 Pane：迁移绑定，Surface 会话保持。
- 移动到已占用 Pane：交换两个绑定，两个 Surface 会话均保持。
- 相邻交换：按几何位置选择左、右、上、下候选；先按主方向距离，再按正交重叠和距离确定唯一邻居。
- Reset Layout：恢复 Workspace / Editor / DSH 默认三区布局。

布局限制为最大深度 8、最多 31 个节点，并约束最小 Pane 尺寸与分割比例，避免不可操作布局。

## PlatformView 生命周期

主窗口内 move/swap 不执行原生 view reparent，也不重新创建 Surface Widget。因此 Viewer、Native Text 和 DSH 的 Flutter State 与 PlatformView identity 应保持不变。

隐藏 Workspace 或 DSH 属于显式可见性切换，当前会卸载对应 Surface；再次显示时允许重建视图。会话数据必须由插件模型或 sidecar 持有，不能只存在于 PlatformView。系统独立窗口迁移未来采用“会话保留、视图可重建”的语义，不承诺保留 IME marked text、WebView 页面内临时 UI 或原生 first responder。

`SurfaceMutationGuards` 可在危险生命周期阶段返回 allow、defer 或 deny。Host 在 move、swap、close 前聚合检查；defer/deny 时不修改布局并向用户显示原因。Native Text marked-text 和 DSH 忙状态的真实检测应在各平台适配器具备可靠信号后接入，不能以时间延迟代替。

## 稳定性门禁

自动门禁：

- 布局 JSON 往返、损坏恢复和原子写入
- split / close / bind / move / swap 与方向邻居
- 多 EditorGroup 资源路由
- move/swap 后 Surface State identity 保持
- 200 次 split/swap/close 压力循环
- mutation guard 聚合规则
- macOS 集成测试中的 Native Text 与 Viewer PlatformView State identity

macOS 发布前手工门禁：

1. 在 Native Text 中输入组合文本并保留选区，交换相邻 Pane；确认内容、选区和输入法行为正常。
2. 在 Viewer 中缩放图片或滚动 PDF，交换 Pane；确认页面、缩放和滚动状态不丢失。
3. 在 DSH WebView 中完成登录/会话操作并滚动页面，交换 Pane；确认 sidecar 会话、页面和焦点可继续使用。
4. 连续执行切分、交换、关闭、重启应用，确认布局恢复且没有孤立 PlatformView、崩溃或明显内存增长。

## 后续独立窗口

`SurfaceMobility` 和 `SurfaceLocation` 已定义 embedded/floating 迁移契约。实施前仍需完成 macOS/Windows 多窗口事件循环、窗口关闭事务、全局焦点路由、PlatformView 重建恢复以及崩溃回收设计。默认采用独占迁移，不克隆同一 Surface 会话。
