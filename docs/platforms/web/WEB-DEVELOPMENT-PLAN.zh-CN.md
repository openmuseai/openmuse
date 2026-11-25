# OpenMuse Web 开发计划与首个实现切片

状态：执行中；基线 2026-10-07。架构依据：[Web 技术选型与目标架构](WEB-ARCHITECTURE-TECH-SELECTION-IMPLEMENTATION-PLAN.zh-CN.md)。

## 目标和发布边界

Web 使用独立 Flutter 入口承载与 Desktop 共源的工作台、WorkBuddy、remote-workbench 和未来 Office；DSH 对话在右侧窗格同页 DOM 挂载钉住版本的 DSH Web 前端，不使用 iframe。Mobile 的 Flutter 原生 DSH renderer 继续服务 Mobile，Web 不以重写它作为交付条件。完整 Flutter Office 套件尚未开发；当前里程碑只覆盖仓库现有的简化 DOCX 编辑与 XLSX/PPTX/PDF 文本兼容视图，未来完整格式另立项目门禁。

发布拆为两个可独立验收的层级：先发布 Cloud Web（账号、Cloud Workspace、DSH 官方 Web 会话与当前可用 Office 能力），再发布配对 Desktop（浏览器设备身份、opaque E2E Relay 和 DSH Web 插件资源桥接）。配对不能用未经授权的明文代理临时代替 E2E 承诺。

## 实施顺序与验收

| ID / 阶段 | 预计 | 负责人建议 | 实施任务 | 退出证据 |
| --- | --- | --- | --- | --- |
| WUI / Desktop 共源工作台 | 1–3 周 | Flutter + QA | 共源画布、窗格菜单、侧栏、目录行、编辑标签、欢迎区与 tokens；同尺寸视觉/键盘矩阵；默认中间 Viewer；目录懒加载 | [公共验收矩阵](WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md)全部通过；当前共源 UI、基础 Viewer 和同页 DSH 模块已落地，认证端到端仍缺配对 Edge 与 DSH 行为验证 |
| W0 / 入口与同源切片 | 1–2 周 | Flutter + Web/TS | 独立 `app/openmuse_web`；抽共享 Office Widget；Web release 构建；仅本地的 `/app/` + `/dsh/` 同源代理；在真实 DSH 上验证登录后的 index 注入、客户端插件图、`/api`、`/plugins`、WS 和面板挂载 | JS release 构建；Mobile Office 现有测试通过；DSH 无 iframe 打开并完成至少一次真实消息和审批；未登录请求仍由 DSH 拒绝 |
| W1 / 共享应用层 | 2–3 周 | Flutter | 从 Mobile 入口抽 WorkBuddy model/controller 与 Host composition；拆 remote-surface client/server 导出；补 Web route、键盘、深链接 adapter；不导入 `dart:io`/FFI/WebView | Android/iOS 现有回归通过；Web 入口可从服务端数据渲染 Workspace；刷新/后退重建相同选择；共享层 Web analyze/build 通过 |
| W2 / Cloud Web | 2–3 周 | 后端 + Web/TS + Flutter | Web Edge/BFF：HttpOnly 会话、CSRF、账号与 Workspace 授权；DSH 同页启动模块/boot/API/插件资产同源代理；Flutter Cloud catalogue/resource adapter；统一登出和租户隔离 | 真实账号从 Web/Mobile 进入同一 Cloud Workspace/DSH session；历史、消息、审批、附件、重连；跨账号与过期会话拒绝；插件资产版本锁定 |
| WO / 当前 Office 能力 | 2–4 周，与 W2 并行 | Flutter + Rust | Web Engine port；Rust DOCX Wasm bytes ABI；解决 viewers 的浏览器 `getrandom` 阻塞；Cloud Range/CAS；共享 Flutter Widget、字体/IME 和格式能力门禁 | 同一资源在 Mobile/Web 返回相同格式与能力；DOCX 简化编辑保存与冲突回执；只读格式没有保存入口；相同 fixture 的内容与布局比较 |
| WD / DSH 客户端产品集成 | 1–3 周，与 W2 并行 | Web/TS | 固定 DSH 官方发行闭包；会话深链接映射；OpenMuse 返回导航；插件 bundle/CSP/缓存和浏览器能力；实现同页 DOM 挂载，这是产品必需路径，必须完成 mount/dispose、焦点及插件回归 | 真实 DSH Web 客户端的历史、审批、附件、插件和重连回归通过；升级 smoke test；没有 iframe 或另写的 Flutter Web 对话 |
| W3 / 配对 Desktop | 4–7 周，以 PoC 重新估算 | 后端/安全 + Web/TS | Web 设备密钥/配对 grant；WSS opaque E2E client；Desktop outbound；DSH Web 客户端 fetch/WS/bundle 加载桥接；浏览器可执行插件资源的来源与权限审计 | 同一 Desktop/Workspace/session 跨 Web/Mobile 更新；断线、撤销、重放、错设备拒绝；Relay 不见明文；浏览器可在授权后加载插件且无法越权 |
| W4 / Remote Surface | 2–3 周，与 W3 并行 | Flutter + 后端 | 声明式/媒体/快照 profile；snapshot/action/receipt/event 与 Range handle；浏览器无障碍和受控操作 | 同一插件 surface 在 Mobile/Web 得到相同授权结论；掉响应后按 receipt 恢复；过期媒体与跨 Workspace 拒绝 |
| W5 / 发行门禁 | 2–3 周 | QA + 全员 | Chrome/Edge/Safari/Firefox；弱网/重连、中文 IME、键盘、Office 文档 corpus、CSP/CSRF、体积/性能、灰度/回滚 | 浏览器矩阵与安全评审通过；Cloud 和 Paired 按各自能力矩阵发布；所有未实现能力在 UI 中准确标识 |

按 2 名 Flutter/Dart、1 名 Web/TS、1 名后端/安全及共享 QA 粗估，Cloud 纵向切片为 5–8 周，包含生产级配对、远程插件及当前 Office 能力的首个完整发行约 14–26 周。估算不是已完成进度，W0 的真实 DSH 启动和 W3 的 E2E 插件加载 PoC 后必须重估。

## Desktop 共源工作台阶段（2026-10-08）

[UI、交互与公共验收矩阵](WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md)是此阶段的逐项门禁。当前已抽出 `packages/openmuse_workbench_layout`，Desktop 和 Web 共同使用分屏模型、画布、分隔拖拽、窗格菜单、侧栏骨架、目录行、编辑标签与欢迎区，主题字体回退也共源。Web 默认保持左 Workspace / 中间 `editor.primary` / 右 DSH+Cloud；连接 Desktop 后通过授权目录端点只取挂载元数据，展开目录才分页取子项。Markdown/文本/图片 Viewer 共享 Flutter body 与授权资源读取已接入；钉住的 DSH 原客户端可在同页 DOM 容器挂载，未认证请求已在浏览器验证。真实 Web 配对 Edge、Viewer 的 PDF/视频/Range 能力和 DSH 认证会话功能仍未通过，不能将此阶段标为完整产品验收。

2026-10-08 增量：Web 已复用 GoTrue 登录状态机与 UI，并加入浏览器 HTTP adapter、同账号设备发现和 `/v1/account/open` grant 请求。登录后总是进入共源工作台；Desktop 连接失败、无在线设备及重试在工作台内处理。本地开发代理将账号/设备控制面与 Desktop 配对数据面分路。真实生成环境密码登录接口本次返回 HTTP 504，故 Workspace 同步与 DSH 会话的生产端到端验收仍阻塞；浏览器 adapter 和失败状态测试已通过。

下一顺序：WUI-1 真实同尺寸视觉与键盘验收；WUI-2 grant scoped ResourceRef/Viewer；WD-1 DSH mount/dispose 稳定入口；W3 Web 配对数据面；W5 四浏览器与 Office corpus。任何功能在矩阵仍为“未通过”时不能以静态占位视作交付。

## 当前落地状态

已完成的首个代码切片：

1. 新建 `app/openmuse_web`，使用共享 `openmuse_host_shell` 主题；工作台已改用 Desktop 共源分屏、侧栏、编辑标签、菜单和欢迎区；早期跳转 `/dsh/` 的按钮已从工作台移除。钉住 DSH 原客户端的同页面板模块已编译，认证后的对话/审批仍待验收。
2. 将 `DocxEditorScreen`、`OfficeViewerScreen` 搬至 `packages/openmuse_office_flutter`；Mobile 原文件保留导出兼容。Web Office route 接受真实授权 handle、Engine、Range 和 Commit port，未接入时不虚构文档。
3. 新增 `web/dev-edge.mjs`，只监听 loopback，将 Flutter 构建放在 `/app/`，DSH 原 Web 页面与其 API/插件静态资源转发到同一源。它仅用于开发验证，不含生产账号 BFF、CSRF 或配对授权。
4. 本地验证：`flutter build web --release --base-href /app/`、Web analyze、共享 Office analyze、Mobile DOCX/Viewer 回归通过；连接现有 DSH 进程时 `/app/` 返回 200、未认证 `/dsh/` 返回 401、DSH 静态 JS 返回 200；伪造 Host/跨源 Origin 请求返回 403。**尚未完成带认证的 DSH 页面、消息/审批和插件图浏览器 E2E**。

下一步按依赖优先级执行：先用受控测试账号完成 W0 的真实 DSH boot/消息/审批，再确定 Cloud BFF 的会话交换和 DSH 页面代理合同；并行把 WorkBuddy/Office 的 `dart:io`/FFI 边界抽成端口。配对 E2E 插件 bundle 加载需单独 PoC，不能以本地开发代理的成功替代。

## 变更和质量门禁

- 每个 Dart 共享包必须能被 Web 入口编译，同时保持 Mobile 测试通过；不在共享包内加入浏览器或移动平台分支散落调用。
- DSH Web 发行包保持单一版本锁；升级时测试 index bootstrap、对话/审批/附件、插件图和授权失败路径。
- Office 每种格式独立开启 `view/edit/export`：Engine、共享 Flutter renderer、资源事务与对应 corpus 全通过才启用保存。
- Cloud 与 Paired 的认证、明文位置和资源权限分开记录；浏览器端密钥安全级别不得等同硬件 Keystore。
