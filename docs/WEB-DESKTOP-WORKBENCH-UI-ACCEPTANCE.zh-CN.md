# Desktop / Web 共源工作台：UI、交互与验收矩阵

基线：2026-10-08。适用范围是已授权登录后的 OpenMuse 工作台，目标视口为 Desktop 支持的 960 px 以上；更窄窗口保留同一组件并允许文案换行。Web 不拥有第二套窗格结构、颜色、图标或菜单。平台仅注入 Workspace 资源端口、DSH 客户端挂载和 Office Engine。

## 设计和代码边界

| 层 | 共源实现 | Desktop 注入 | Web 注入 |
| --- | --- | --- | --- |
| 分屏树、持久化模型、几何与方向邻居 | `openmuse_workbench_layout/layout.dart` | 文件 LayoutStore | 浏览器 localStorage，按 Desktop/Workspace 作用域 |
| 画布、边框、焦点、分隔拖拽、覆盖菜单 | `WorkbenchCanvas` + `OpenMusePaneResizer` | 本地 Plugin Surface | 浏览器 Surface |
| 菜单、品牌、侧栏、树行、编辑标签、欢迎区 | `workbench_ui.dart` | 本地挂载/文件/插件目录 | 授权镜像/插件能力 |
| 颜色、字号、间距 | `openmuse_host_shell/OpenMuseTokens` | 系统字体回退 | 同一字体回退链；中文首帧可读 |
| 默认窗格 | 同一 `createDefaultWorkbenchLayout` | Workspace / `editor.primary` / DSH / Cloud | 完全相同的绑定；中间保留 Viewer/Office Surface |

默认首屏按 Desktop 的侧栏视觉组织：左上品牌与设置/收起、搜索、Project Workspace 和目录树、插件/回收站；中间以 `Blank page` 标签和共享欢迎区开始，选择文件后该编辑组承载 `open-file-viewer` 或其他授权编辑器；右侧 DSH 与 Cloud 面板维持原有分屏和背景。Web 的 DSH 必须在右侧面板同页 DOM 挂载原 DSH Web 客户端，禁止 iframe 与跳走当前工作台。浏览器文件树先获取挂载元数据，展开目录时请求该目录的一页子项，加载更多显式分页；绝对路径只在 Desktop 保留。

## 公共验收矩阵

“通过”只表示本次可执行的共源 UI 切片通过；生产级 Cloud/Paired 端到端仍以对应集成门禁为准。

| 项目 | 同一实现与验收方法 | 当前结果 |
| --- | --- | --- |
| 默认 Workspace / 编辑组 / DSH / Cloud 拓扑 | Desktop 12 项布局测试；Web Widget 渲染同一 snapshot | **通过** |
| 账号登录后进入工作台；Desktop 不可用时留在应用内重试 | 共用 GoTrue 登录 UI/状态机，Web HTTP adapter；设备发现失败的浏览器 Widget 测试验证侧栏、编辑组和 DSH 分区仍可见 | **前端门禁通过；生产账号登录请求当前 HTTP 504，真实登录回归阻塞** |
| 侧栏品牌、搜索、Project Workspace、目录行、插件/回收站的文本、图标、尺寸与颜色 | 两端引用 `OpenMuseWorkspaceSidebar`、`OpenMuseWorkspaceRow` 和同一 tokens；Web 浏览器截图复核 | **通过，窄视口文字可截断** |
| 编辑 `Blank page` 标签、文件图标、关闭按钮及欢迎区 | 两端引用 `OpenMuseEditorTabStrip` 与 `OpenMuseWorkbenchWelcome`；Web Widget 测试 | **通过** |
| 窗格菜单、切分、方向交换、重置、关闭确认 | 共用菜单；Desktop 布局测试、Web Widget 测试；平台处理资源销毁 | **通过可用操作；Web 特定资源销毁待端口接入** |
| 分隔拖拽、焦点边框、窗格实例保活 | 共用 `WorkbenchCanvas`；Desktop 几何测试、Web Widget 测试及浏览器复核 | **通过基础操作** |
| 中文、图标和窄视口溢出 | 主题增加跨端中文字体回退，欢迎区使用 Wrap；浏览器复核 | **通过当前浏览器视口；浏览器矩阵待测** |
| 已连接 Desktop 默认显示相同挂载，目录懒加载/分页 | Desktop 授权目录端点、Web port、控制器测试；不下发绝对路径 | **协议/单测通过；真实 Web 配对 Edge 尚未联通** |
| 中间打开真实文件，默认 `open-file-viewer` 并预览 | Desktop Viewer 与 Web 复用 Flutter Markdown/文本渲染；opaque ResourceRef 由 grant scoped 端点读取 | **Markdown/文本/图片组件与授权读取单测通过；PDF/视频、Range 和真实 Web 配对链路未通过** |
| DSH 原客户端留在右侧面板 | 钉住 `0.1.7-rc.1` 的 `AppWebEntry(container)`/`dispose()` 编为按需面板模块；从同源受鉴权页面安装启动注入；Flutter `HtmlElementView` 提供普通 DOM 容器 | **同页模块与 401 诊断浏览器验证通过、iframe 数为 0；真实认证 boot、对话/审批/插件图尚未通过** |
| Cloud Workspace 实际面板和插件目录 | 要求 Cloud Web adapter 和授权目录 | **未通过；当前面板为占位状态** |
| 无障碍/键盘/屏幕阅读器、四大浏览器、Office 大文档 | 浏览器矩阵及未来 Office 套件 corpus | **未执行，不能计入发行验收** |

## 本次执行证据

- `flutter analyze`：共享布局包、Desktop Host、Web 入口、Paired Gateway 均通过。
- `flutter test test/workbench_layout_test.dart`：Desktop 12 项通过。
- `flutter test`：共享镜像控制器 2 项通过。
- `flutter test test/workbench_ui_test.dart`：Web 共源 UI、菜单与懒加载后打开共享 Viewer 的 2 项测试通过。
- `flutter test test/workspace_mirror_service_test.dart`：Desktop 目录分页、opaque 引用、不泄漏路径及授权文件读取通过。
- `flutter test`（共享 Viewer 包）与原 File Viewer Markdown 测试：共源 Markdown 渲染通过。
- `flutter test test/paired_desktop_gateway_test.dart`：包含 grant 与 Workspace 作用域校验，通过。
- `flutter build web --release --base-href /app/ --no-wasm-dry-run`：通过；本地同源页面浏览器截图复核品牌、侧栏、编辑标签、欢迎区、DSH/Cloud 分区和中文。
- `flutter build macos --debug`（Desktop Host）：通过，确认共源组件接入后原生构建可用。
- `npm run build`（`web/dsh-pane`）：钉住 DSH 客户端的同页模块编译通过；浏览器在未认证 DSH 上显示明确身份验证状态，DOM 中没有 iframe。
- `flutter test --platform chrome test/browser_adapters_test.dart`：GoTrue 请求合同、设备发现与 grant 请求、发现返回 401 时仍展示工作台共 4 项通过；普通 `flutter test` 仍通过。
- 本地 `/app/` 代理已将设备目录路由到 Cloud、`/v1/account/open` 与 Workspace/DSH 路由到 Desktop 网关；`/v1/status` 返回 ready，未认证响应分别与 Cloud 和 Desktop 网关一致。Desktop UI 确认同一测试账号及在线设备。2026-10-08 使用该账号请求生产 GoTrue `/gotrue/token?grant_type=password` 多次返回 HTTP 504；健康接口返回 200。故本次无法证明真实 Web Workspace 同步或 DSH 会话通过。

## 剩余实施门禁

1. 已验证钉住 DSH 客户端的 `AppWebEntry(container)`/`dispose()` 导出，并由 Flutter Web `HtmlElementView` 同页挂载；下一步在受控认证会话下验证 boot、历史、审批、附件、重连、插件弹层、焦点、IME 与 CSP。
2. 已为 opaque `resourceRef` 加入 grant scoped 的 16 MB 内读取和 Markdown/文本/图片 Flutter Viewer；下一步补 Range、PDF/视频、文件格式与能力判定。连接时只同步挂载列表，任何目录子项请求必须由展开动作触发。
3. 接通生产 Web 配对 Edge 的浏览器身份、grant、E2E Relay 与目录端点；刷新、断线、撤销和跨 Workspace 拒绝后再将镜像标为端到端通过。
4. 对 Desktop 和 Web 同尺寸截屏做逐项视觉审核；全量浏览器、键盘、无障碍及 Office corpus 达标后才能声明完整 UI/产品验收通过。
