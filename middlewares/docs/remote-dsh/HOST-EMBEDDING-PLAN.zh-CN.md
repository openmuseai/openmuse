# Host 嵌入 DSH：开发计划

> 状态：计划 v1（2026-09-10）。设计合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](HOST-EMBEDDING-DESIGN.zh-CN.md)。诊断：[HOST-EMBEDDING-ANALYSIS.zh-CN.md](HOST-EMBEDDING-ANALYSIS.zh-CN.md)。
> 与池阶段 P0–P3 **正交**：P 系列管进程与配额；E 系列管 Host 怎么打开、绑定、排障。E0 不依赖再改 systemd。

---

## 1. 目标与完成定义

用户从「黑屏 / 没有 POST / 假联动」变成：

1. 任何失败都能读出 **stage + code + attachmentId**；
2. 有登录 + 工作区时 Network **一定有** `POST /api/muse/dsh/session/open`；
3. iframe/WebView **只加载** Placement 返回的 URL（生产即 `/u/<hash>/?token=`）；
4. 文档联动只在 bind ACK 之后；
5. Web / Desktop / Android 错误码同一张表，载波仍分叉。

---

## 2. 阶段总览

| 阶段 | 主题 | 平台 | 现网价值 | 阶段文档 |
|---|---|---|---|---|
| **E0** | Attachment 状态机 + 必发 session/open + 禁空黑底 + 排队 | **Web 先** | 解除当前生产面板 | [E0-WEB-ATTACHMENT-SM](phases/E0-WEB-ATTACHMENT-SM.zh-CN.md) |
| **E1** | bind RPC、凭据顺序、关旗错误码 | Web + DSH 注入脚本 | 真联动，不再静默 401 | [E1-BIND-RPC](phases/E1-BIND-RPC.zh-CN.md) |
| **E2** | Desktop 对齐 stage/code；Android 接 session/open | Desktop + Android | 三端合同一致 | [E2-PLATFORM-ADAPTERS](phases/E2-PLATFORM-ADAPTERS.zh-CN.md) |
| **E3** | attachmentId 贯穿、nginx/inject/instance.env 合同、runbook | 部署 + 全端 | 可恢复追踪、防漂移 | [E3-TRACE-AND-DEPLOY](phases/E3-TRACE-AND-DEPLOY.zh-CN.md) |
| **E4** | 工作区 Plugin：Host catalog P0；BFF 禁 `NOT_FOUND`；tree/query P1 | Web + BFF + appflowy-workspace | 能列 AppFlowy 页，不再把 README 当空工作区 | [E4-DOCUMENT-PLANE](phases/E4-DOCUMENT-PLANE.zh-CN.md) |
| **E5** | 同一套 catalog/snapshot 合同接到 Mobile HTTP；Desktop 只落盘本地载波 | Android 先；Desktop 文档 | 远程 DSH 对手机与 Web 同一 Plugin 面 | [E5-MOBILE-DESKTOP-PLUGIN-PLANE](phases/E5-MOBILE-DESKTOP-PLUGIN-PLANE.zh-CN.md) |

每个阶段文档固定三块：**设计 / 实现 / 测试矩阵**。未写进矩阵的行为不算完成。阶段出口 = 矩阵全绿 + 文档中的生产验收项（E0/E3 含现网）。

依赖：

```text
E0 ──► E1 ──► E2
 │              │
 └────► E3 ◄────┘     E3 可与 E1 部分并行（nginx 片段），但 Host header 要等 E0 有 attachmentId
              │
              └──► E4   依赖 E1 HybridLive + E3 BFF 是 /api/muse 入口；不改 systemd
                         │
                         └──► E5  Mobile HTTP 载波 + 同合同 Host 投影；Desktop 不改池
```

E0 可单独上生产 Web。不要等 E2 才修现网黑屏。

---

## 3. 每阶段交付物清单（强制）

| 交付物 | 位置 |
|---|---|
| 设计（不变量、状态、不做） | 本阶段文档 §1 |
| 实现（路径、接口、迁移） | 本阶段文档 §2 |
| 测试方案 + 矩阵 | 本阶段文档 §3–§4 |
| 自动化 | 矩阵中标「单测/组件测」的必须有 CI |
| 生产验收 | §5；E2 Desktop 为本地手工 |

禁止：只改代码不改矩阵；矩阵写「应能打开」这种不可证伪句。

---

## 4. 与已有工作的关系

| 已有 | 处理 |
|---|---|
| 池 P1 `session/open` 现网 200 | E0 **消费**它，不再改执行器 |
| `canMountDshIframe` | E0 保留并接到 mount 前硬校验 |
| `sessionRequestId` / `resolveDshAccessToken` | E0 收进 generation + 判别联合 |
| Android `DshSessionApi` / error contract | E2 接线，不重写 API |
| Desktop sidecar | E2 映射 stage，不改信任模型 |
| `web-workspace-ingress.md` | E1 更新为 RPC 合同；旧 fire-and-forget 作废 |

---

## 5. 风险与并行

- **E0 部署后仍无 POST：** 先查矩阵 E0-T4/T5（真缺登录/工作区）再查 E0-T1。
- **冷启动 3 分钟：** E0 用 `COLD_START` 文案；预编译属池 P2，不挡 E0。
- **inject 测试仍断言 `dsh.openmuseai.com`：** E0/E3 改 `test_inject_web_config.py`，CSP 改为 apex。
- **Android exclusive-test：** E2 改实例内锁，需实例 flag 与 Host 一起发，否则只有 WebView 无桥。

---

## 6. 建议排期（相对顺序，非日历）

1. E0 实现 + 单测 + 现网硬刷新验收（Web）。
2. E1 注入脚本 RPC + Host bind 等待（可同周）。
3. E3 nginx/BFF 片段对齐（短，可插在 E1 旁）。
4. E2 Android 接线；Desktop 错误码映射（不挡 Web 生产）。
5. E4 工作区 Plugin：先 BFF 501 + Host catalog（现网即可验收「有哪些页面」），再 collab query。
6. E5 Mobile：打开实例 `MUSE_MOBILE_BRIDGE` + ControlHost catalog/snapshot；Desktop 保持 sidecar，不挡 Mobile。

## 7. 落地状态（2026-09-10）

| 阶段 | 代码 | 自动化矩阵 | 生产验收 |
|---|---|---|---|
| E0 | 已合入 Web 面板 / session / attachment | E0-T1–T13（Jest + W1-05 py） | **2026-09-10** 已打 Web dist（CSP apex + lazy chunk 含 `session/open`）。E0-P1–P6 需登录后硬刷新看 Network |
| E1 | hello/bind 等 token；8s ACK；失败叠层保 iframe | T1–T12 已自动化 | 现网 `parent-bridge` 已换 RPC 并重启活实例（loopback 200）。E1-P* 需登录后看 iframe 子帧 |
| E2 | Android Placement + session/open；Desktop errorCode | T1–T12（T8 spawn 为 controller 映射，非真进程） | E2-P* 手工 |
| E3 | BFF attachment 日志；nginx `:8010`；instance.env 模板；runbook | T1–T8 自动化（T6 沿用 inject py） | 现网 `auth_request` 已是 BFF `:8010`；BFF 已打 `attachmentId` 结构化日志。E3-P1/P4 需登录 UI |
| E4 | P0 已合入（catalog + snapshot + BFF 501） | T1–T6、T8 + 本地 e2e | **2026-09-10** Web Host + 插件已通；生产勿覆盖 `presentation-facets.js` stub |
| E5 | Mobile HTTP 旗 + Flutter catalog/snapshot + **同实例 SharedHostSession** | T1–T10 本地绿 | **2026-09-11** 载波已开。共享会话需同步 `parent-bridge.js`+`mobile-lease.js` 并重启 tenant unit（不重启 pool）。Host catalog 随 Flutter 包分发 |
