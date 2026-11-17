# 远程 Surface 与可选社媒插件：开发计划与测试矩阵

状态：RS0 协议闭环，以及可点击的验收桌面，已在 `feature/remote-surface-protocol` 落地（2026-10-05）  
规格：[`MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md`](MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md)、[`EASEL-DSH-OPTIONAL-SOCIAL-PLUGIN-DESIGN.zh-CN.md`](EASEL-DSH-OPTIONAL-SOCIAL-PLUGIN-DESIGN.zh-CN.md)、[`OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md`](OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md)

本文件是实施顺序和验收矩阵。它不放宽上述规格：业务插件仍不进入 Mobile 安装包，发布仍必须走带确认令牌的外部副作用命令。

Desktop 编辑区下方的控制台由可停用的 `com.openmuse.cli` 插件提供。它的 `openmuse` 命令入口与 Mobile `remote-workbench` 的 control action 共用 Host 领域命令服务和 Job receipt；Mobile 不接收本机 shell。CLI 插件的实现关卡见上方 CLI 设计，RS0–RS7 的 fake 业务验收不证明 Easel CLI 或真发已经接通。

## 1. 阶段

| 阶段 | 目标 | 状态 |
| --- | --- | --- |
| RS0 | 线协议、Dart/Rust 投影、内存 Host、fake 业务面、`remote-workbench` 渲染与断线恢复；Mobile 默认安装该插件 | 本分支 |
| RS1 | Paired gateway 的 `/openmuse/remote-surface/v1` 在反代前返回；Desktop 侧栏经本机 loopback grant 打开同一台验收主机，并可停用社媒发布。不把这条路径转给 DSH | 本分支：路由、桌面侧栏、抽屉和网关点击。真机发现未做 |
| RS2 | 媒体 handle 的 TTL / 设备 / Workspace 绑定，界面经 Range 读取已授权字节 | 本分支：句柄与 Range。HLS 播放列表未做 |
| RS3 | `web-snapshot` 界面；`web-interactive` 在 mobile hello 上不可打开；虚拟 origin 拒绝 localhost 与 cookie | 本分支：快照与守卫。浏览器 CSP 沙箱未做 |
| RS4 | 审计记录 actor、设备、Workspace、action、状态与 decision。不记录 action 输入 | 本分支：本地 JSONL。生产 Relay 未做 |
| RS5 | 同一界面依次走发布预览/确认和视频裁剪。确认只执行一次 | 本分支：验收桌面。小红书真发未做 |
| RS6 | 目录清单按 sha256 装载，停用后撤销该插件的 surface | 本分支：清单与摘要。进程级 Worker 与代码签名未做 |
| RS7 | Job ledger 落盘。新 Host 用同一幂等键返回原 receipt，不再次执行 | 本分支：JSONL ledger。许可与 SBOM 未做 |

RS0 故意不接 Easel 脚本、浏览器 profile、真实社媒登录或真发。`FakePublishSurfaceProvider` 与 `FakeVideoEditProvider` 只用于证明协议可被第二个业务复用。

## 2. RS0 交付

- `contracts/openmuse-remote-surface/v1/messages.json`：hello、descriptor、snapshot、control request、receipt、event 的正反 fixture。
- `packages/muse_remote_surface_contract`：严格 Dart 投影。拒绝未知字段、定位符（`file://`、`http://`、`localhost`、`127.0.0.1`）、调用方自报的 actor/device，以及必需的未知组件。
- `crates/openmuse-remote-surface`：同一 fixture 的 Rust 校验。
- `packages/muse_remote_surface_core`：内存 Surface Host。按 Workspace 与读权限过滤；协商展示模式；校验 action schema、revision 与幂等键；插件停用只使该插件的 generation 失效。
- `plugins/remote-workbench`：默认 Mobile 插件。用原生组件渲染组件树，可选未知组件退化为说明卡。外部副作用超时后按 requestId/idempotencyKey 查询 receipt，不再次提交。
- `app/openmuse_mobile` 在应用启动时安装并激活 `com.openmuse.remote-workbench`。Debug 构建的任务抽屉打开验收桌面；release 只有在 `OPENMUSE_REMOTE_SURFACE_ACCEPTANCE` 为真时才启用它，否则显示“等待已配对的 Desktop”。

展示模式优先级是 `declarative`、`media`、`web-snapshot`、`web-interactive`。客户端没有对应能力时跳过该模式，不会降级成任意网页代理。

幂等规则：同一次提交（相同 idempotency key 与相同 input）不重复执行；另一次用户确认使用新的 key，允许再次执行。外部副作用在响应丢失时查询已有 receipt；查不到则标记 `outcome_unknown`，仍不自动重试。

## 3. 测试矩阵

命令：`scripts/test_remote_surface.sh`。验收桌面不连接社媒网站，也不包含真机、生产 Relay 或 HLS。

| ID | 断言 | 位置 | 阶段 |
| --- | --- | --- | --- |
| RS-WIRE | 有效 fixture 往返，无效 fixture 拒绝 | Dart `fixture_test`，Rust `fixtures_match_the_closed_loop_contract` | RS0 |
| RS-TREE | 深度、节点数、重复 nodeId、必需未知组件、可选未知组件 | `tree_test` | RS0 |
| RS-HOST-01 | 停用发布插件后，视频插件的会话与 `clip.trim` 仍可用 | `host_test` | RS0 |
| RS-HOST-02 | 相同幂等键不重复执行；新的确认键会再次执行 | `host_test` | RS0 |
| RS-HOST-03 | 旧 generation 返回 `STALE_GENERATION`，provider 不再执行 | `host_test` | RS0 |
| RS-HOST-04 | 缺少发布权限时 `DENIED`，不执行 | `host_test` | RS0 |
| RS-HOST-05 | 必需的未知组件拒绝打开，且不留下会话 | `host_test` | RS0 |
| RS-HOST-06 | 仅 `web-interactive` 且客户端不支持时拒绝，不选择代理模式 | `host_test` | RS0 |
| RS-HOST-07 | Workspace 不匹配的 surface 不可发现 | `host_test` | RS0 |
| RS-HOST-08 | provider 抛错时该次调用失败，另一个插件仍可执行 | `host_test` | RS0 |
| RS-CLIENT-01 | 响应在 Host 执行后丢失：查询 receipt，submit 次数不增加 | `client_test` | RS0 |
| RS-CLIENT-02 | 请求未到达 Host：`outcome_unknown`，不补发 | `client_test` | RS0 |
| RS-CLIENT-03 | 重连先取 snapshot，再按 `afterSeq` 补事件 | `client_test` | RS0 |
| RS-UI-01 | 渲染文本、图片替代、视频时长、表单，并完成预览 | `workbench_test` | RS0 |
| RS-UI-02 | 可选未知组件显示为说明卡，页面仍可用 | `workbench_test` | RS0 |
| RS-UI-03 | 发布确认在丢响应后仍只提交一次 | `workbench_test` | RS0 |
| RS-APP | Mobile 应用构造时安装 `com.openmuse.remote-workbench`，现有登录页不回归 | `mobile_app_test` | RS0 |
| RS-PAIR | `/openmuse/remote-surface/v1` 使用配对 cookie，且不请求 DSH；无 dispatcher 时不反代 | `remote_surface_gateway_test` | RS1 |
| RS-MEDIA | handle 过期、跨设备、跨 Workspace、带斜杠的句柄拒绝；Range 不进入 DSH | `lab_test`，`remote_surface_gateway_test` | RS2 |
| RS-WEB | 只允许虚拟 https origin；localhost、`127.0.0.1`、cookie 拒绝 | `lab_test` | RS3 |
| RS-EASEL | sha256 不符时装载失败且不改注册表；清单移除后插件消失 | `lab_test` | RS6 |
| RS-VIDEO | 任务抽屉经配对网关完成一次发布确认和一次裁剪，并看到快照来源；客户端自报 actor 不进入审计 | `remote_workbench_gateway_test` | RS1/RS5 |
| RS-DESKTOP | Desktop 侧栏经本机网关完成一次发布确认、一次裁剪、看到快照来源，然后停用社媒发布 | `remote_workbench_desktop_test` | RS1/RS5 |
| RS-LEDGER | 新 Host 对同一幂等键返回原 receipt；不同输入冲突；审计不含标题正文 | `lab_test` | RS4/RS7 |

## 4. 仍未进入生产的部分

- 小红书真发使用测试账号和 Desktop 上预先准备的登录态。Mobile 不接收 cookie、profile 路径或 AppSecret。
- 长视频的 HLS/Range、浏览器 CSP 沙箱、生产 Relay、代码签名 Worker、许可与 SBOM 仍未实现。
- 真发、profile 清理和多平台登录留在这些门禁之后。验收桌面的“确认发布”只写入本地账本。
