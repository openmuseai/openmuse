# E5 Mobile / Desktop Plugin 面 — 设计 / 实现 / 测试

> 阶段：E5。依赖 E1（HybridLive 之后才 contribute）、E2（Android Placement / Desktop sidecar 映射）、E4（catalog / snapshot 合同与 BFF 501）。合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](../HOST-EMBEDDING-DESIGN.zh-CN.md) §1 / §4 / D1–D5。
> **本阶段先打通 Mobile**（与 Web 同为 remote DSH）。Desktop 只落盘合同与 Identity 面，不改成远程池。

---

## 1. 设计

### 1.1 三端同一套 Plugin，载波分叉

Everything is Plugin：**工作区 / 文档是业务 Plugin**，不是 Attachment SM 的胶水。Web 已在 E4 把 `workspace.catalog` 与 `markdown.snapshot` 接到 Host。Mobile / Desktop 必须投递**同一套** Facet 类型与 digest，不得另发明「移动专用树 API」。

| 端 | Placement | Presentation | Collaboration 载波 | Catalog / Snapshot 真相（P0） |
|---|---|---|---|---|
| Web | `POST /api/muse/dsh/session/open` → `/u/<hash>/` | iframe | postMessage → 注入脚本 XHR → parent-bridge | 侧栏 outline + 可见 editor |
| **Android** | **同 Web**（`DshSessionApi`） | WebView `fromWebUrl` | **HTTPS parent-bridge**（`X-Muse-Client: mobile`） | Folder `getAllViews` + `GetDocumentText` |
| Desktop | 本地 `DshSidecar.ensureStarted` | loopback WebView `dsh web:` | **hint 文件**（无 parent-bridge HTTP） | 后续：hint 旁路 catalog JSON；UDS 读正文。**不走 BFF** |

不变量：JWT 永不进 DSH；Mobile 出站只用 device token；配置 URL ≠ 会话 URL；禁止 iframe/WebView `/dsh/`。

### 1.2 现网缺口（相对 Web 已通）

1. **HTTP 载波默认关。** `nativeHttpSse` 仅当 `MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST=1`。现网 capabilities 为 `nativeHttpSse:false`。Flutter `MuseParentBridgeAdapter` 要求 `nativeHttpSse==true` 且 `mode==exclusive-test`，否则 `HOST_UPGRADE_REQUIRED`，面板 degrade，**即使** Placement 的 `/u/` 已经 200。
2. **Host 未投递 E4 Facet。** `AppFlowyDshControlHost` 只 contribute `workspace.focus` + 打开文档时的 `markdown.surface/selection`。`_forward` 丢掉其它 contextType。Agent 在 Mobile 上仍会把 scratch README 当成「工作区」。
3. **旗标名是测试旗。** 池已是一租户一进程，生产使用 `MUSE_MOBILE_BRIDGE=1` 打开载波。`mode: exclusive-test` 仍写在 capabilities 里（Flutter 硬检查），**不再表示 Web/Mobile 互斥**。跨租户不会 409（不同进程）。
4. **Desktop** E2 已映射 stage/code；协作仍是 `DshWorkspaceBridge` hint（Identity pin cwd）。本阶段不验收 Desktop catalog，也不为对齐 Web 打开 `MUSE_DOCUMENT_CLOUD_URL`。

同一租户 `ACTIVE=1`：Web inject（XHR + EventSource）与 Mobile HTTP/SSE **共用一个 DSH 进程、同一套 bind / catalog / snapshot / intent fanout**。第二路 Host 同 workspace 是 attach，不是抢 lease。`HOST_IN_USE` 只表示实例内 Host 附件达到上限（8）。跨 workspace 才是 `SCOPE_MISMATCH`（403）。不要为此启动 Docker `muse-dsh`。

### 1.3 决策

```text
P0 Mobile
  1. 实例 env MUSE_MOBILE_BRIDGE=1 → capabilities.nativeHttpSse=true
  2. bind ACK 之后 Host contribute:
       workspace.catalog.v1   digest sha256:2538c0adb882241b625b48c0013e4a0dd248d8b391cac22ebdc14e154c50548e
       markdown.snapshot.v1   digest sha256:e4ea242dbdbc52565fb474cd042af4da590ae88a9d4715e9694ce031eaee03b3
     仅当有 latestOpenView 且 GetDocumentText 成功才发 snapshot；否则 DSH 侧 NO_CURRENT_SELECTION
  3. 60s heartbeat 重发 catalog（+ 有焦点则 snapshot）
  4. muse_dsh_mobile 保持 Host-agnostic；投影只放 AppFlowy ControlHost
  5. SharedHostSession：同 workspace 多 Host（Web + N 台 Mobile）；detach 不清 bind/catalog
     capabilities 增加 sharedHosts:true；mode 仍写 exclusive-test（Flutter 硬检查，改了会 HOST_UPGRADE_REQUIRED）

P0 Desktop（文档合同，本阶段不改执行器）
  Binding = publish hint 成功
  Catalog 后续文件： $DSH_HOME/bindings/current-appflowy-catalog.json
  Snapshot 后续：UDS / 本机 collab，禁止 BFF
```

`MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST=1` 仍有效（CI / 旧测）。生产用 `MUSE_MOBILE_BRIDGE=1`。两者任一为 1 即打开载波。

### 1.4 不变量

| ID | 陈述 |
|---|---|
| E5-M1 | Android `Presenting` URI 仍满足 C1（`/u/<32hex>/`，非 `/dsh/`） |
| E5-M2 | `nativeHttpSse=false` 时 Host 不得假装 HybridLive（保持 degrade / HOST_UPGRADE_REQUIRED） |
| E5-M3 | catalog / snapshot 合同字段与 Web E4 相同；含 `access_token` 子串则整包不投递 |
| E5-M4 | 无当前 viewId 不发 snapshot |
| E5-M5 | Desktop 测试套件零请求 `/api/muse/dsh/session/`（C3） |
| E5-M6 | 打开 Mobile HTTP **不**等于把 Desktop 改成远程 Facet |
| E5-M7 | 同实例 Web EventSource 与 Mobile SSE 可同时 200；异 workspace → 403 `SCOPE_MISMATCH`，不是 409 |

### 1.5 不做

- 不在本阶段做 iOS。
- 不把 Desktop sidecar 改走 `session/open`。
- 不覆盖生产 `presentation-facets.js` stub，不上传本地 `ContextFacetInbox`。
- 不为 Mobile 单独实现 `/api/muse/workspace/tree`（P1 仍是 BFF；P0 fallback Host catalog）。
- 不 `docker compose up muse-dsh`。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `plugins/dsh-appflowy/src/mobile-lease.ts` | `SharedHostSession`：同 workspace 多 Host；`HOST_IN_USE` 仅附件上限；`ExclusiveMobileLease` 保留给旧测 |
| `plugins/dsh-appflowy/src/parent-bridge.ts` | `mobileEnabled` ← `MUSE_MOBILE_BRIDGE=1` **或** 原 exclusive 旗；去掉 Web 四条 409 门；`peer.close` = detach |
| `deploy/instance.env.example` | `MUSE_MOBILE_BRIDGE=1` |
| `dsh-pool/tests/embedding-deploy-contract.test.ts` | 断言 example 含该键 |
| `appflowy_flutter/.../dsh_workspace_catalog.dart` | 有界 flatten（≤64、depth≤4），纯函数 |
| `appflowy_flutter/.../dsh_markdown_snapshot.dart` | ≤32KB UTF-8；空文本不构建 |
| `appflowy_flutter/.../appflowy_dsh_control_host.dart` | bind 后 + heartbeat 投递 catalog/snapshot |
| Desktop | **本阶段仅文档**；hint 路径不变 |

生产部署（DSH 侧，手动）：

1. 同步 `parent-bridge.js`（若改了 `mobileEnabled`）。
2. `/opt/muse-dsh/instance.env` 增加 `MUSE_MOBILE_BRIDGE=1`（不要打印 DEEPSEEK / JWT）。
3. **只** `systemctl restart muse-dsh-<hash>`（不要重启 `dsh-pool`，以免丢掉租户→端口表）。
4. 回环 `GET :13081/muse/v1/parent-bridge/capabilities` → `nativeHttpSse: true`。
5. Flutter Host 变更随 App 构建分发；**不能**用拷 Web dist 的方式装到手机。无新 APK 时，生产只验证载波已开。

---

## 3. 测试方案

- Dart：catalog flatten / 截断 / 禁 token 子串；snapshot 空文本与 32KB；control plane 在 fake contribute 上断言顺序（bind 后才发 catalog）。
- Vitest：`MUSE_MOBILE_BRIDGE=1` 且 exclusive=0 时 capabilities 为 true；=0 且无 MOBILE_BRIDGE 时仍 false。
- 禁止以真机打生产作为唯一证据。

---

## 4. 测试矩阵

| ID | 场景 | 期望 | 类型 |
|---|---|---|---|
| E5-T1 | 平铺 views，含 parent | depth 由祖先链计算，≤4 | Dart |
| E5-T2 | >64 items | truncated=true，只 64 条 | Dart |
| E5-T3 | title 含 `access_token` | 该条丢弃或整包拒绝，不 contribute | Dart |
| E5-T4 | 无 viewId | 不构建 snapshot | Dart |
| E5-T5 | 文本 >32KB | truncated + byteLength≤32768 | Dart |
| E5-T6 | `MUSE_MOBILE_BRIDGE=1` | capabilities.nativeHttpSse true | Vitest |
| E5-T7 | 两旗皆关 | nativeHttpSse false；mobile POST 403 | Vitest（沿用 mobile-http） |
| E5-T8 | bind 后 fake host | contribute 含 `workspace.catalog` | Dart |
| E5-T9 | Desktop 套件 | 无 session/open | 沿用 E2-T9 |
| E5-T10 | Mobile 占用时 Web bind 同 workspace、GET /events | 非 409；第二 connection 200；异 workspace 403 | Vitest |

---

## 5. 验收

| ID | 端 | 通过 |
|---|---|---|
| E5-P1 | 生产实例 capabilities | `nativeHttpSse: true`、`sharedHosts: true`（重启 unit 后） |
| E5-P2 | 本地 Flutter 测 | E5-T1–T5、T8 绿 |
| E5-P3 | 有 APK / 模拟器 + 登录 | session/open → `/u/` 200 → Agent 能列出侧栏标题；未打开文档 → `NO_CURRENT_SELECTION` |
| E5-P4 | Desktop 本地 | 仍不出现 session/open；缺 key 仍 `NEED_API_KEY` |
| E5-P5 | Android 已连时打开 Web `/u/` | iframe parent-bridge **不是** 409 `HOST_IN_USE`；Agent 侧栏/焦点与手机共享 |

排障：capabilities false → instance.env；HOST_UPGRADE_REQUIRED → 载波未开或 mode 不是 `exclusive-test`；Web 403 `SCOPE_MISMATCH` → 与已 attach 的 Host 不是同一 workspace；409 → 仅 Host 附件已满（8），不是「手机占了实例」。

---

## 6. 实现记录（2026-09-11）

| 项 | 状态 |
|---|---|
| `mobileEnabled` | `MUSE_MOBILE_BRIDGE=1` 或 exclusive 测试旗 |
| `instance.env.example` | 含 `MUSE_MOBILE_BRIDGE=1` |
| Flutter catalog/snapshot 纯函数 + ControlHost 投递 | 已合入 |
| E5-T1–T5、T8 | `flutter test` 绿 |
| E5-T6/T7/T10 | `mobile-http` + SharedHostSession vitest |
| 生产载波 | **2026-09-11** `parent-bridge.js` + `MUSE_MOBILE_BRIDGE=1` |
| 共享会话 | **2026-09-11** `SharedHostSession` 替换实例内 Web/Mobile 互斥；需同步 `parent-bridge.js` + `mobile-lease.js` 并重启 **tenant unit**（Android 会短暂重连）。不重启 `dsh-pool` |
