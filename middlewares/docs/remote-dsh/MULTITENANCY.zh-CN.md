# Remote DSH 多租户实例池（Instance Pool）方案

> 状态：方案 **v0.2**（可执行；相对 v0.1 的变更见文末）
> 范围：mobile / web 的 `remote-dsh` 模式多租户改造；桌面端 `dsh-native`（本地 sidecar）不受影响。
> 前置阅读：[ARCHITECTURE.zh-CN.md](ARCHITECTURE.zh-CN.md)、[GATEWAY.zh-CN.md](GATEWAY.zh-CN.md)、[RUNTIME-COMPARISON.zh-CN.md](RUNTIME-COMPARISON.zh-CN.md)、阶段文档 [P0](phases/P0-HOST-CHANNEL.zh-CN.md) / [P1](phases/P1-INSTANCE-POOL.zh-CN.md) / [P2](phases/P2-SYSTEMD-EXECUTOR.zh-CN.md) / [P3](phases/P3-SCHEDULING.zh-CN.md)、[ADR-001](ADR-001-nomad-executor.zh-CN.md)。Host 嵌入（状态机 / 三端适配 / 排障）见 [HOST-EMBEDDING-ANALYSIS](HOST-EMBEDDING-ANALYSIS.zh-CN.md)、[DESIGN](HOST-EMBEDDING-DESIGN.zh-CN.md)、[PLAN](HOST-EMBEDDING-PLAN.zh-CN.md)（E0–E3，与 P 系列正交）。
> 约束前提（业务给定）：宿主 **4 核 / 4GB** Linux，同机或邻近运行 Cloud（GoTrue + Postgres），成本敏感；生产 **不用 Docker 跑实例**。

---

## 0. 结论先读（本版相对 v0.1 的硬决策）

1. **顺序不可颠倒**：先打通与宿主 AppFlowy-Cloud 的鉴权通道（P0），再做实例池（P1）。鉴权 ≠ 隔离；但没有鉴权的池化等于按用户分好房间却不上锁。
2. **生产执行载体**：裸进程 + `systemd-run` 瞬时 unit + cgroup v2；Docker 只留在构建 / CI / 可复现验收。见 RUNTIME-COMPARISON。
3. **调度分层**：领域控制面（租户路由、session、等待室、idle TTL、launch token）**永远自研一层薄的**；进程拉起做成可替换 `Executor`。单机阶段 **不引入 Nomad / k3s**。本地测试通过后，用明确门槛评估 Nomad `raw_exec` 是否值得作为 **Executor 后端**（不是替代控制面）。**本期不做跨机器调度**，但接口按「以后能换 Nomad」预留。
4. **同机入口**：DSH 公网路径收到 **Cloud 同源 nginx**（`https://<domain>/dsh/` 与后续 `/u/<tenantHash>/`），废弃「`dsh.<domain>` 裸子域」作为生产入口——否则 Cloud Cookie 跨站，L0 网络鉴权无法落地。
5. **容量瓶颈是内存不是核数**：4C 相对旧文档 2C 只抬高 **ACTIVE**（真在跑 agent 的并发），不抬高 READY 常驻实例数。

---

## 1. 问题陈述

### 1.1 现状：单实例、单 profile、进程级单例 = 无法多用户

当前 remote 模式是一个共享常驻 DSH Host（`docker-compose.remote.yml` 单容器、`restart: always`、单 `DSH_HOME` volume），
所有 mobile/web 用户都连它。**这不是部署疏忽，而是代码结构性决定的**——`@muse/dsh-appflowy` 把状态做成进程级单例：

| 现状证据 | 位置 | 后果 |
|---|---|---|
| `lastWorkspaceSurface` / `sseClients` / `mobileLease` / `lastBoundHint` 皆是模块级变量 | `plugins/dsh-appflowy/src/parent-bridge.ts` | 后绑定的用户会 `removeSurface(previous)` **顶掉**前一个用户的 workspace |
| `ExclusiveMobileLease` 单控制器独占，`HOST_IN_USE` | `plugins/dsh-appflowy/src/mobile-lease.ts` | **已改为** `SharedHostSession`：同实例 Web+Mobile 共享；409 仅附件上限。`ExclusiveMobileLease` 仅保留给旧测 |
| capabilities 自述 `scopedMultiTenant: false`、`mode: "exclusive-test"` | `parent-bridge.ts` `/capabilities` | 无租户域概念 |
| web 的 `workspace.bind` **不校验** deviceToken（只 `rememberDeviceAuth` 做句法过滤） | `parent-bridge.ts` `rememberDeviceAuth` | 任意能打到 parent-bridge 的请求都能绑任意 workspaceRef |
| `remote-disable-dsh-passwords.sh` 删除 DSH 本地密码 | `deploy/scripts/remote-disable-dsh-passwords.sh` | 经 nginx 反代后 Web UI 入口无鉴权 |
| 生产入口是独立子域 `dsh.<domain>` | ARCHITECTURE §2、GATEWAY §4 | 与 Cloud Cookie 不同源，无法用登录态挡入口 |
| `POST /api/muse/dsh/device-token/verify` 取 `UserUuid`（GoTrue JWT） | `Muse-Server/AppFlowy-Cloud/src/api/muse.rs` | DSH **不能**拿着 device token 调这个接口；宿主回源应走 `MuseActor`（已支持 JWT **或** HMAC device token） |

Cloud 侧其实已经有半条通道，只是 **Web / nginx / 子域** 没接上：

| 已有资产 | 位置 | 现状用法 |
|---|---|---|
| 签发 / 校验 / 撤销两段式 device token | `biz/muse_dsh.rs`，路径 `/api/muse/dsh/device-token*` | Web / Flutter 已能签发；撤销在 Redis |
| `MuseActor`：GoTrue JWT **或** HMAC device token | `biz/muse_auth.rs` | document / workspace API 已用 |
| 工作区成员校验 | `POST /api/muse/workspace/current` | **仅 mobile** `verifyMobileWorkspace` 在用 |
| BFF 路径预留 | GATEWAY §4 | `session/open` 尚未存在 |

### 1.2 结论：隔离单元必须是“实例（进程）”，而不是 workspace 条目

DSH 是桌面级 harness（单 profile、单 web server、单 index tap、插件可全局改写），
在**单个进程内**做多租户（多用户共享 agent runtime / System Prompt / 插件域 / 模型 key）改造风险
远大于省下的一点点内存。方案定为：**实例即租户**——每个租户独立进程、独立 `DSH_HOME`、独立端口、独立 cgroup。

配合 **懒创建（scale-to-zero）+ 空闲回收 + 幂等去重**，让 4C/4G 也付得起。

---

## 2. 目标与不变量

### 2.1 目标

1. 每个租户（默认粒度：**账号 × AppFlowy workspace**，见 §5.1）拥有**独立的** DSH 实例；
2. 首次打开才创建（lazy / scale-to-zero），**重复打开不重建**（幂等命中已有实例）；
3. 空闲超时自动回收资源：实例进程销毁，但**状态（`DSH_HOME` 目录）保留**，下次打开秒级恢复；
4. 同机容量受控：并发实例数、内存、CPU 全部有硬上限与排队策略；
5. 安全：Cloud JWT 不进实例；实例只见自己的 device token 与 launch token；
6. **P0 即上线**：即使仍是共享实例，未登录 / 伪 token / 跨 workspace 也进不去。

### 2.2 不变量（每条都是验收依据）

- **I0 宿主通道**：任何 `parent-hello` / `workspace.bind` 必须经 Cloud 确认「该 device token 的 actor 是该 workspace 成员」；失败 fail-closed（401/403），不允许「先绑再鉴权」。
- **I1 唯一性**：同一 `tenantKey = H(accountRef, workspaceRef)` 同时最多存在 **1** 个实例（幂等去重）。
- **I2 状态持久**：实例销毁 ≠ 状态销毁。销毁只回收进程/内存；`DSH_HOME` 落在宿主盘，重建即恢复。
- **I3 资源预算**：READY 实例数 ≤ `READY_QUOTA`，并发「真在跑 agent」的 ≤ `ACTIVE_QUOTA`，超限进入等待室。
- **I4 隔离**：实例间 cgroup 隔离；文件系统互不可见（专属 UID + `0700`）；网络只绑 loopback，公网仅经 Cloud nginx。
- **I5 凭据隔离**：实例只见两段式 device token / 单实例 launch token，不见 Cloud JWT；模型 key 走共享 env，不落用户态。
- **I6 冷启动**：`starting → ready` 目标 < 10s（预构建 runtime + node_modules 去重 + 盘上 profile 就绪）。
- **I7 执行器可替换**：控制面不直接 `spawn`；只调 `Executor.start/stop/inspect`。本期实现 `local` + `systemd`；`nomad` 只留类型，不实现。

---

## 3. 术语

| 词 | 定义 |
|---|---|
| 租户单元（tenant unit） | 隔离与收费的最小单位。默认 `(accountRef, workspaceRef)` 二元组（工作区租户） |
| tenantKey | `sha256(salt \|\| accountRef \|\| '\0' \|\| workspaceRef)`——池内去重与路由键，不出现在 URL |
| tenantHash | `tenantKey` 的 ≥128bit 公开截断，用于 `/u/<tenantHash>/`，防枚举 |
| 实例（instance） | 一个 DSH harness 进程（`dsh --profile web`），独立 `DSH_HOME`、独立 loopback 端口、独立 cgroup |
| session | Gateway 侧的租户会话记录（`SessionGatewayV2.sessionRef`）：跨设备、跨 attachment 存在 |
| device / attachment | 某一时刻接入该 session 的浏览器 tab / 移动设备（`SessionAttachmentV2`）；同 session 可多设备 |
| launch token | DSH 每进程 Web UI 的授权令牌（`dsh web:` 日志行，见 `dsh_web_auth.dart` `extractLaunchToken`）——天然一对一，是给 iframe/WebView 的**会话 URL 凭据** |
| 控制面（pool） | 本方案自研的薄调度：租户表、配额、等待室、idle、把 webUrl 交给客户端 |
| Executor | 控制面背后的进程管家接口；systemd-run / 本地 spawn /（未来）Nomad 都是它的实现 |
| 等待室（waiting room） | 实例配额耗尽时，请求排队并在前端显示「排队中」 |

---

## 4. 目标架构

```text
公网
  │ https  （TLS 终止在 Cloud nginx；DSH 不再占用独立公网子域）
  ▼
Cloud nginx（已有）
  ├─ /app…                              → AppFlowy-Web SPA（已有）
  ├─ /api/muse/dsh/device-token         → Cloud BFF（已有：签发/校验/撤销）
  ├─ /api/muse/dsh/ingress-auth         → Cloud BFF（P0 新增：nginx auth_request）
  ├─ /api/muse/dsh/session/*            → Cloud BFF → Pool（P1 新增）
  ├─ /dsh/                              → 共享实例（仅 P0 过渡；auth_request 强制登录）
  └─ /u/<tenantHash>/                   → Pool 反代（P1）：查路由表 → 127.0.0.1:<port>
        │
        ▼
Pool 控制面（同机轻量 Node，~30–50MB）
  ├─ 复用 @muse/remote-runtime 的类型：
  │    DeviceTokenAuthorityV2 / SessionGatewayV2 / ResumableStreamV2
  │    （生产签发仍以 Cloud BFF 为准，Pool 不另造一套 HMAC）
  ├─ 路由表：tenantKey → {instanceRef, port, state, ttl, launchToken}
  ├─ 等待室 + LRU + idle
  └─ Executor（可替换）
        ├─ local     ：child_process（macOS / CI，无 cgroup）
        ├─ systemd   ：systemd-run 瞬时 unit（生产单机）
        └─ nomad     ：预留，本期不实现
        ▼
Instance Pool（同机，每租户一进程，cgroup 限额）
  ├─ tenant A:  node harness  DSH_HOME=/srv/muse-dsh/<hashA>/home  PORT=13081  UID=16xxx
  ├─ tenant B:  …                                                   PORT=13082
  └─ …（≤ READY_QUOTA；其余 suspended，只占盘）
        └─ bind 时回源 Cloud：POST /api/muse/workspace/current
           Authorization: Bearer <deviceToken>
           X-Muse-Device-Id: <deviceId>
```

Web / Mobile 打开链路（P1 完成后两端同构；P0 仍走静态 `/dsh/`，但 L0+L2 已锁门）：

1. 客户端已有登录态（Cloud JWT / GoTrue Cookie）；
2. `POST /api/muse/dsh/device-token`（已有）→ 两段式 device token；
3. P1：`POST /api/muse/dsh/session/open {workspaceRef, deviceId}` → Pool 查/建实例，返回 `{sessionRef, instanceRef, webUrl(with launch token), expiresAt}`；
   P0：iframe/WebView 仍指向同源 `/dsh/`；
4. Web：iframe `src = webUrl`；Mobile：WebView `loadRequest(webUrl)`；
5. `frame-ready` → `parent-hello`（**必须**带 deviceToken）→ `workspace.bind`（Cloud 成员校验通过才 pin）；
6. 断开/关闭 → `session/close`（P1；P0 仅释放 mobile lease / 停心跳）。

**动态端口不要让 nginx reload**：nginx 只把 `/u/` 和 `/dsh/` 打到 Pool（或 P0 的固定 loopback）。Pool 用 Node 反代 WebSocket/SSE。4G 上这比 lua 写 upstream + `nginx -s reload` 更稳，也更利于以后把「反代」和「Executor」拆到不同机器。

---

## 5. 组件设计

### 5.1 租户粒度决策（默认：账号 × 工作区）

| 粒度 | 优点 | 缺点 | 结论 |
|---|---|---|---|
| **per (account, workspace)**（默认） | 与 DSH workspace 一对一直映；parent-bridge 单 workspace 语义无需改；去重/计费最简单 | 同账号多 workspace 并发 = 多实例内存 | 4C/4G 下同账号同时开 2 个以上工作区属低频，额度内可承受 |
| per account | 省内存 | 跨 workspace 会触发 `removeSurface` 顶绑；需把 parent-bridge 状态改成 per-workspace keyed | 资源不足时的降级项，不推荐先做 |

**推荐：per (account, workspace)。** 完全复用现有 `bindAppFlowyWorkspaceAt` 的「一个实例一个 workspace」语义，几乎不动 `@muse/dsh-appflowy` 内部。

### 5.2 P0：与宿主 AppFlowy 的健全通道（先于任何池化）

「健全」= 身份、归属、入口、回源 四件事都闭合，且 **Cloud JWT 永不进入 DSH 进程**。

```text
浏览器 / App
  │  ① GoTrue JWT / Cookie（只到 Cloud）
  ▼
Cloud BFF
  │  ② 换 device token（已有）
  │  ③ nginx auth_request → ingress-auth（新增）
  ▼
DSH parent-bridge
  │  ④ parent-hello / workspace.bind 携带 deviceToken
  │  ⑤ DSH → Cloud POST /api/muse/workspace/current（MuseActor）
  ▼
Postgres 成员表   ← 唯一授权源
```

#### 5.2.1 为何现有通道是断的

| 断点 | 说明 | P0 修法 |
|---|---|---|
| Web bind 不回源 | `rememberDeviceAuth` 只拒绝 `sk-` 前缀和非法两段式，**不调 Cloud** | `bindWorkspace` / `parent-hello` 强制走与 mobile 同一套 `verifyHostWorkspace`（现 `verifyMobileWorkspace` 改名复用） |
| verify 接口不对 DSH 开放 | `/device-token/verify` 要 `UserUuid` | DSH **不要**调它；回源用已支持 device token 的 `/api/muse/workspace/current` |
| 独立子域 | `dsh.example.com` 拿不到 `example.com` 的登录 Cookie | 入口改挂 Cloud nginx 同源 `/dsh/` |
| parent-bridge HTTP 无认证 | 注入脚本 `POST /muse/v1/parent-bridge` 任意 JSON | 协议层 fail-closed；nginx L0 挡匿名页面 |
| 密码被删 | `remote-disable-dsh-passwords.sh` | 生产禁止跑；P0 以 nginx+协议为准，不把 DSH 本地密码当主锁（iframe 不好用 basic auth） |

#### 5.2.2 鉴权四层（P0 必须 L0+L2+L3；L1 随 P1 实例化自动到位）

| 层 | 机制 | 谁做 | 何时 |
|---|---|---|---|
| **L0 网络** | Cloud nginx `auth_request /api/muse/dsh/ingress-auth`；上游仅 `127.0.0.1` | 运维 + Cloud | P0 |
| **L1 实例入口** | 每实例 launch token（`dsh web:` 日志行）；token 只经 `session/open` 发给已认证用户 | Pool | P1 |
| **L2 协议** | device token + `workspace.current` 成员校验；`parent-hello` / `workspace.bind` 强制 | parent-bridge | P0 |
| **L3 运营** | 试点账号白名单（BFF 拒名单外）；生产禁止 disable-passwords | BFF / 运维 | P0 |

`ingress-auth` 契约（新增，专供 nginx，**无 body**）：

| 项 | 值 |
|---|---|
| 方法 | `GET` 或 `HEAD`（`auth_request` 默认 GET） |
| 身份 | `MuseActor`：`Authorization: Bearer`（GoTrue JWT **或** device token）**或** 已有 Web Cookie 会话 |
| 成功 | `200`，空 body |
| 失败 | `401` |
| 禁止 | 把 Cloud JWT 以任何请求头转给 DSH 上游 |

Mobile WebView 常常没有 Cloud Cookie：L0 对 `/dsh/` 与 `/u/` 同时接受 `Authorization` 或 `X-Muse-Device-Token`。P1 之后 L1 launch token 可作 URL 查询参数，由 **Pool** 校验（不要让 nginx 解析 DSH 日志）。

#### 5.2.3 parent-bridge 行为（P0 代码改动面）

`handleParentInbound` 在 `parent-hello` / `workspace.bind`：

1. 读取 `deviceToken` + `deviceId`；缺失 → `401 NO_DEVICE_TOKEN`（不再静默 bind）；
2. `workspaceRef` 缺失 → hello 可以只记身份，**不允许**绑定空 workspace；
3. 调用 `verifyHostWorkspace`（即今天的 `verifyMobileWorkspace`）：
   - `POST ${MUSE_DOCUMENT_CLOUD_URL}/api/muse/workspace/current`
   - `Authorization: Bearer <deviceToken>`，`X-Muse-Device-Id: <deviceId>`
   - body `{ workspaceId: workspaceRef }`
4. Cloud `code !== 0` 或返回的 `workspaceId` 不一致 → `403 SCOPE_MISMATCH` / `401 DEVICE_AUTH_REJECTED`；
5. 通过后才 `applyWorkspaceHint` / pin。

门控：

| 变量 | 远程生产 | 桌面 sidecar |
|---|---|---|
| `MUSE_REQUIRE_HOST_AUTH` | **必须 on**（缺省 on；`0` 仅本地测试） | off（hint 文件 + UDS，不走 HTTP iframe） |
| `MUSE_WEB_WORKSPACE_BIND` | 远程 **禁止**关 | 保持现有 |
| `MUSE_PARENT_BRIDGE` | 保持「有 Cloud URL 则开」 | 桌面可用 hint 文件 |

校验结果 **缓存 ≤ 20s**（mobile lease 已有 `verifiedAt` 窗口），避免每次 postMessage 都打 Cloud；缓存键 = `(token fingerprint, workspaceId)`。

P0 **不**拆除 `ExclusiveMobileLease`：共享实例阶段 mobile 仍只能一人；Web 在共享实例上仍会顶绑——这是已知残缺，用 L2 保证「外人绑不进别人的 workspace」，用 P1 消除顶绑。

#### 5.2.4 P0 验收

- 未登录打开 `/dsh/` → nginx 401，页面不进 DSH；
- 伪 device token / 过期 token 的 `workspace.bind` → 401；
- 合法用户绑 **别人的** workspaceId → 403；
- 试点账号：hello → bind → context 全链路可用；
- 桌面 sidecar 回归：不受 `MUSE_REQUIRE_HOST_AUTH` 影响；
- DSH 进程环境 / 日志 / parent-bridge 响应中 **无** GoTrue JWT、无 `sk-` 模型 key。

### 5.3 控制面 Pool（P1 新增轻量 Node，同机）

**不要把 Pool 做成通用集群调度器。** 它只做本领域的事：

| 职责 | 非职责 |
|---|---|
| `tenantKey` 幂等、状态机、配额、等待室、idle TTL | 跨机器 bin-pack、服务发现、容器网络 |
| 抽出 launch token、拼 `webUrl` | 改 DSH harness 源码 |
| 调 `Executor.start/stop/inspect` | 直接 `posix_spawn` 写死 systemd |
| 反代 `/u/<hash>/` → 实例端口 | 让 nginx 热改 upstream |

**REST（挂 Cloud BFF，JWT 已校验后再转 Pool；Pool 只听 loopback）：**

| 端点 | 入参 | 出参 | 语义 |
|---|---|---|---|
| `POST /api/muse/dsh/device-token` | 账号 Bearer | `{token, deviceId, kid, expiresAt}` | **已有** |
| `POST /api/muse/dsh/session/open` | `{workspaceRef, deviceId}` + Bearer | `{sessionRef, instanceRef, webUrl, expiresAt, queuePosition?}` | **幂等**：有实例→attach；suspended→同 `DSH_HOME` resume；无且有配额→start；满→等待室 |
| `POST /api/muse/dsh/session/close` | `{sessionRef, deviceId}` | `{ok}` | 释放 attachment；无剩余 → idle 计时 |
| `POST /api/muse/dsh/session/heartbeat` | `{sessionRef}` | `{ok}` | 保活 |

`accountRef` **禁止**由客户端声称：BFF 从 JWT / `MuseActor` 取出，再与 `workspaceRef` 做成员校验（复用 `require_member_workspace`），然后才 `tenantKey`。

包划分：

| 包 | 角色 |
|---|---|
| `@muse/remote-runtime` | **保持**会话/token/流类型与单测；不在这里 fork 进程 |
| `@muse/dsh-pool`（新建，`middlewares/dsh/core/dsh-pool`） | 控制面 + Executor + 反代 |

### 5.4 Executor 接口（调度可替换的唯一缝）

```ts
interface InstanceSpec {
  tenantKey: string;
  tenantHash: string;
  homeDir: string;          // /srv/muse-dsh/<hash>/home
  port: number;             // 127.0.0.1 独占
  uid?: number;             // 生产必填；local 可省略
  memoryMaxBytes: number;   // 默认 512MiB
  cpuWeight: number;
  env: Record<string, string>;
}

interface InstanceHandle {
  instanceRef: string;
  executor: "local" | "systemd" | "nomad";
  pid?: number;
  unitName?: string;        // systemd
  allocId?: string;         // 未来 Nomad
  port: number;
  startedAt: number;
}

interface Executor {
  readonly kind: InstanceHandle["executor"];
  start(spec: InstanceSpec): Promise<InstanceHandle>;
  stop(handle: InstanceHandle): Promise<void>;
  inspect(handle: InstanceHandle): Promise<{
    alive: boolean;
    rssBytes?: number;
  }>;
  waitReady(handle: InstanceHandle, timeoutMs: number): Promise<void>;
}
```

| 实现 | 环境 | 行为 |
|---|---|---|
| `LocalProcessExecutor` | macOS 开发机、CI | `child_process` + 独占端口 + 临时 `DSH_HOME`；**无** cgroup / 专属 UID |
| `SystemdRunExecutor` | 生产 4C/4G、Linux CI | `systemd-run --unit=muse-dsh-<hash> --uid= --property=MemoryMax=512M …`；日志进 journald |
| `NomadExecutor` | **不实现** | 接口留 `allocId`；见 §5.5 |

启动命令与现网 entrypoint 同构（只读 runtime 一份）：

```text
/opt/muse-dsh/runtime/node --import tsx/esm apps/cli/src/bin.ts
  --profile web --patch /opt/muse-dsh/patch.yml
  --host 127.0.0.1 --port <allocated>
```

每实例环境：

| env | 值 |
|---|---|
| `DSH_HOME` | `/srv/muse-dsh/<tenantHash>/home`（0700，专属 UID） |
| `PORT` / `DSH_LOOPBACK_PORT` | 分配的 loopback 端口（如 13080–13127） |
| `MUSE_APPFLOWY_DSH_WORKSPACE_ROOT` / `MUSE_APPFLOWY_DSH_WORKSPACE` | 钉死该租户 workspace 目录 |
| `MUSE_DOCUMENT_CLOUD_URL` | Cloud 基址（回源用） |
| `MUSE_REQUIRE_HOST_AUTH` | `1` |
| `DEEPSEEK_API_KEY` 等 | **全实例同一份 env**，不进用户态 |
| `DSH_TRUSTED_HOST` | Cloud 同源 origin |

状态机：

```text
absent → starting → ready
ready --idle TTL--> idle → suspended（kill 进程，保盘）
suspended --session/open--> starting（同一 DSH_HOME）
ready/crashed --backoff--> starting
suspended --目录 TTL（可选）--> destroyed
```

探活：`GET http://127.0.0.1:<port>/muse/v1/parent-bridge/capabilities`；启动超时 60s → fail → 5/15/60s backoff。

### 5.5 开源调度器：何时采用、何时自研

控制面（租户幂等、launch token、等待室、DSH_HOME 恢复）**没有**开源项目能直接替换——那是产品语义，不是通用 Job。
开源调度器最多替换 **Executor**（在哪台机器起哪个二进制、cgroup、健康检查）。

| 候选 | 单机 4C/4G | 跨机器（未来） | 与本方案契合力 | 结论 |
|---|---|---|---|---|
| **systemd-run** | 零常驻、cgroup 原生、PID1 即保姆 | 无 | 执行器完美匹配 | **P1 生产 Executor** |
| **HashiCorp Nomad `raw_exec` / `exec`** | agent ~50–80MB，占 DSH 预算的 2–3% | **原生多节点** | 不懂 tenantKey / launch token；需我们当 Nomad API 客户端 | **跨机器时的第一候选 Executor**；单机不划算 |
| k3s / k0s / microk8s | kubelet+containerd ≥300MB，且把我们拽回容器 | 是 | 差 | 拒绝 |
| Knative / OpenFaaS / Fission | 依赖 k8s | 是 | 差 | 拒绝 |
| supervisord / s6 / runit | ~10–30MB，无 cgroup | 无 | 只当无 systemd 主机的后备 | 非首选 |
| Firecracker / cloud-hypervisor / Kata | 微 VM 内存税 | 可 | 隔离过剩 | 拒绝 |
| Docker Swarm / Mesos | 依赖容器或已死 | — | 差 | 拒绝 |
| Temporal / Cadence | 工作流引擎 | — | 错层 | 拒绝 |

**采用 Nomad 的门槛（全部满足才换 Executor，控制面不动）：**

1. 单机 `SystemdRunExecutor` 已在 Linux 上通过 P2 验收（证明控制面正确）；
2. 明确要上 **第二台** DSH 机器，或 systemd-run 在目标发行版上不可用；
3. Nomad agent RSS 实测 < 80MB，且 `raw_exec` 能设 MemoryMax 等价限额 + 指定 UID；
4. 不强迫引入 Consul 作为硬依赖（Nomad 单集群可以自洽）。

未达门槛就继续自研薄控制面 + systemd。跨机器时推荐形态：

```text
每台 DSH 机：Nomad client + 本 Pool 的 Executor=nomad
（或一台 Pool 调多 client）
Cloud nginx / BFF 仍在原机
本期把 tenantKey→nodeId 做成「恒等于本机」，路由表字段预留 nodeId。
```

### 5.6 实例运行时

- 一份只读安装 `/opt/muse-dsh/runtime`（`stage-dsh-runtime.sh` 产物，**不必**在生产 `docker run`）；`@muse/*` 符号链接去重，多实例磁盘 ≈ 1 份 runtime + N 份 `DSH_HOME`。
- 构建 / CI 仍可用 `build-dsh-image.sh` / `dev-up.sh` 作为可复现载体，与生产 Executor 解耦。
- Web UI 由 launch token 保护；Pool 把带 token 的 `webUrl` 交给对应租户。

### 5.7 客户端适配（web / mobile）

| 端 | P0 | P1 |
|---|---|---|
| Web `DshAgentPanel.tsx` | `parent-hello` **必须**带已签发的 deviceToken（已有 `fetchDshDeviceToken`）；iframe 改为同源 `/dsh/` | `url` 改为 `session/open` 的 `webUrl`；关面板 / 换 workspace → `session/close` |
| Mobile `muse_dsh_mobile` | 已有 lease + `verifyMobileWorkspace`；改为走同源 URL，去掉对裸 `dsh.<domain>` 的依赖 | `DshRemoteConfig.publicUri` 改为运行时 `webUrl`；`close()` 补发 `session/close` |
| 桌面 Flutter `DshAgentPanel` | 不改 sidecar | 不改 |

心跳：mobile 60s context heartbeat；web SSE 15s；Pool 以此刷新 idle。

### 5.8 对外路由（nginx）

P0：

```nginx
location = /api/muse/dsh/ingress-auth { internal; proxy_pass http://cloud_upstream; … }

location /dsh/ {
  auth_request /api/muse/dsh/ingress-auth;
  proxy_pass http://127.0.0.1:3080/;   # P1 改为 Pool
  proxy_http_version 1.1;
  proxy_set_header Upgrade $http_upgrade;
  proxy_set_header Connection $connection_upgrade;
  proxy_set_header Authorization $http_authorization;
  # 明确不转发 Cookie 里的 JWT 到 DSH？Cookie 可留在 auth_request；
  # proxy_pass 到 DSH 时丢掉 Cookie / Authorization 中的 GoTrue JWT。
}
```

P1：`location /u/` → Pool（`127.0.0.1:<pool-port>`）；Pool 按前缀查 `tenantHash`。实例端口永不 bind `0.0.0.0`。

---

## 6. 容量模型（4 核 / 4GB）

> 启动改造前先做 §10。下表是**目标预算**（假设单实例 idle RSS ≈ 400MB）。4C 不增加可常驻实例数——RSS 仍按 4GB 算。

| 项 | 预算/取值 |
|---|---|
| OS + nginx + Cloud（GoTrue/Postgres/BFF）+ Pool | ~1.3–1.6GB（Pool 替代 dockerd 后略省） |
| **DSH 实例可用内存** | **~2.2–2.5GB** |
| `READY_QUOTA`（常驻实例上限） | **3**（可热备到 4，若实测 idle ≤ 350MB） |
| `ACTIVE_QUOTA`（真在跑 agent） | **3**（4 核；LSP/code-runtime 仍 CPU 密；超出排队）。旧 2C 文档为 2 |
| 单实例 cgroup | `memory.max=512MB`，`CPUWeight=100`；超限杀进程由 Executor 重启 |
| idle TTL（→ suspended） | **20–30 min** 无活动 |
| suspended → destroyed | 2–4 h（仅目录占盘） |
| 冷启动目标 | `< 10s` |
| 同机在线租户容量 | ~10–20（绝大多数 suspended） |

**排队**：`session/open` 配额满 → 等最近空闲实例转 suspended 后复用额度；10s 轮询；前端「排队中」。禁止为排队用户无限开进程。

**规模治理**：实测 idle ≥ 600MB 或 ACTIVE>3 是常态 → **加内存到 8GB** 优先于任何调度优化。4C/4G 长期目标是低并发热备 + 快速恢复，不是高并发。

---

## 7. 关键时序

### 7.1 P0 共享实例上的打开（必须先于 7.2）

1. 浏览器带 Cloud Cookie 请求 `/dsh/` → nginx `auth_request` → 未登录 401；
2. 已登录：iframe 加载；`fetchDshDeviceToken`；`parent-hello` + `workspace.bind`；
3. DSH 回源 `/api/muse/workspace/current`；非成员 403；
4. 仍是**同一进程**：第二用户绑自己的 workspace 会顶掉第一用户——P0 接受并在产品上限制试点人数。

### 7.2 首次打开（P1 懒创建）

1. `session/open` → BFF 校验 JWT + 成员 → Pool 未命中 → `Executor.start`（`starting`）；
2. 探活 ready → 记录 `tenantKey → instanceRef`（I1）；
3. 返回 `webUrl` → iframe/WebView → `workspace.bind` 落盘该租户目录；
4. 该用户独享该实例。

### 7.3 重复打开（不重建）

1. 同 `tenantKey` → 命中 → 新 attachment + 同一 `webUrl`（或 refresh launch token）；
2. 若 suspended：同一 `DSH_HOME` 重启（quick start）；
3. 若 destroyed：按 7.2 重建，目录仍在则恢复原状态。

### 7.4 销毁

| 层级 | 触发 | 结果 |
|---|---|---|
| 进程（suspended） | idle TTL / 配额挤占 / 崩溃放弃 / 滚动升级 | 进程消失；`DSH_HOME` 保留 |
| 目录 | 低峰清理 / 注销账号（可选） | 按 Cloud 账号策略；DSH 不主动乱删 |

关闭 App 只 `session/close`（卸 attachment），不立刻杀进程。

### 7.5 升级与崩溃

- 升级：替换 `/opt/muse-dsh/runtime` → 旧实例 `REMOTE_HANDLE_STALE` → 客户端重开；
- 崩溃：Executor backoff 重启，同一 `DSH_HOME`。

---

## 8. 安全与隔离

| 风险 | 对策 |
|---|---|
| 共享实例被顶绑 / 串数据 | P0：外人绑不进；P1：进程级隔离 |
| 公网 URL 枚举 | 同源 `/u/<tenantHash>` + launch token |
| Cloud JWT 进实例 | nginx 对 DSH 上游剥离 JWT；实例只见 device token |
| 单实例跑飞 | cgroup 512MB / CPUWeight；超限 kill |
| 跨租户文件 | `DSH_HOME` 0700 + **专属 UID**（裸进程下这是数据隔离主控件） |
| web bind 无鉴权 | P0：`verifyHostWorkspace` fail-closed |
| 子域裸奔 | 废弃 `dsh.<domain>` 作为生产入口 |

---

## 9. 开发计划（按依赖排列，可并行处已标）

原则：每阶段有独立可上线的验收；**未完成 P0 不得在生产切 P1 流量**。桌面 sidecar 全程冻结行为。

### P0 —— 宿主通道（约 5–8 工作日）· 可先上生产共享实例

| ID | 任务 | 主要路径 | 验收 |
|---|---|---|---|
| P0.1 | 抽出 `verifyHostWorkspace`，web `parent-hello`/`workspace.bind` 强制回源 | `mobile-lease.ts`、`parent-bridge.ts` | 单测：无 token / 伪 token / 跨 workspace 失败；有 token 才 pin |
| P0.2 | `MUSE_REQUIRE_HOST_AUTH` 远程默认 on；远程禁止关掉 `MUSE_WEB_WORKSPACE_BIND` | `parent-bridge.ts` | 生产配置契约测试 |
| P0.3 | Cloud `GET/HEAD /api/muse/dsh/ingress-auth`（`MuseActor` 或 Cookie） | `Muse-Server/AppFlowy-Cloud/src/api/muse.rs` | 无凭证 401；JWT 或 device token 200 |
| P0.4 | nginx：DSH 改挂同源 `/dsh/` + `auth_request`；上游 loopback；**不**把 JWT 转给 DSH | `middlewares/dsh/deploy/` | curl 未登录 401；登录后 iframe 可开 |
| P0.5 | 生产禁用 `remote-disable-dsh-passwords.sh`（脚本加非交互保护 / 从 remote compose 移除） | `deploy/scripts/` | 文档与脚本一致 |
| P0.6 | Web：iframe src 改为同源 `/dsh/`；hello 必带 deviceToken | AppFlowy-Web `DshAgentPanel.tsx` | 试点账号全链路 |
| P0.7 | Mobile：公共 URL 改为同源路径；回归 lease | `dsh_remote_config.dart`、Flutter | 同实例多 Host 共享；第二设备同 workspace 不 409 |
| P0.8 | 安全回归清单（未登录 / 伪 token / 跨 ws / 无 JWT 泄漏） | 测试 | §5.2.4 |

并行：P0.9 基线测量（§10），不阻塞 P0 合入。

**P0 完成定义**：共享实例仍在，但已不是裸奔；可给试点用户用。

### P1 —— 本地实例池（约 8–12 工作日）· 先在开发机 / CI 证明语义

| ID | 任务 | 主要路径 | 验收 |
|---|---|---|---|
| P1.1 | 新建 `@muse/dsh-pool`：路由表、状态机、配额、idle | `middlewares/dsh/core/dsh-pool` | 单测：I1 幂等、满配额进等待室 |
| P1.2 | `LocalProcessExecutor` + 端口分配 + 探活 | 同上 | macOS 上起 2 个 harness，端口/DSH_HOME 不同 |
| P1.3 | Pool HTTP 反代 `/u/<hash>/`（含 WS/SSE） | 同上 | 两 iframe 互不串 cookies/workspace |
| P1.4 | Cloud `session/open\|close\|heartbeat`：JWT→成员→Pool | `muse.rs` + Pool client | 客户端不能伪造 `accountRef` |
| P1.5 | 抽出 launch token，拼 `webUrl` | Pool 读 harness 日志 / 已有 `DshWebAuth` 语义 | 无 token 打开 UI 失败 |
| P1.6 | Web / Mobile 改走 `session/open` 的 `webUrl` | `DshAgentPanel.tsx`、`muse_dsh_mobile` | 换 workspace = 换实例；关面板 close |
| P1.7 | 本地双租户夹具 | `dsh-pool/tests` | 两账号并发打开 → 两进程；重复 open → 同 pid |

**P1 完成定义**：不依赖 systemd / 不依赖 4G 机器，语义在笔记本上可测。

### P2 —— 生产执行器（约 6–10 工作日）· Linux 4C/4G

| ID | 任务 | 主要路径 | 验收 |
|---|---|---|---|
| P2.1 | `SystemdRunExecutor` + unit 模板 | dsh-pool + `deploy/` | `systemctl` 能看到 `muse-dsh-<hash>`；`MemoryMax` 生效 |
| P2.2 | 租户 UID 池 + `DSH_HOME` 0700 | 部署脚本 | 租户 A 进程读不了租户 B 目录 |
| P2.3 | 只读 runtime 目录安装（stage 产物，非 docker run） | `stage-dsh-runtime.sh` | 两实例共享 node_modules 链接 |
| P2.4 | idle TTL → suspended；resume < 10s | Pool | I2 / I6 |
| P2.5 | 切生产：nginx `/u/` → Pool；`/dsh/` 下线或只给应急 | deploy | 试点两账号互不顶绑 |
| P2.6 | 4C/4G soak：RSS / 冷启动 / ACTIVE 峰值 | `measure-dsh-capacity.sh` | 回填 §6 |

**P2 完成定义**：生产共享容器可关；Docker 不再出现在实例路径上。

### P3 —— 调度体验 + 开源 Executor 决策（约 4–6 工作日）· 仍单机

| ID | 任务 | 验收 |
|---|---|---|
| P3.1 | 等待室前端文案、LRU 挤占、夜间全部 suspend | 配额满时有序排队，不 OOM |
| P3.2 | 每实例 cgroup 指标（`memory.current` / `cpu.stat`）进日志或现有指标口 | 能按租户摊账 |
| P3.3 | **决策门**：用 §5.5 门槛写一页 Nomad spike（可选，1–2 日）。不通过则关闭 Nomad 工单 | ADR：继续 systemd 或排期 NomadExecutor |
| P3.4 | 路由表预留 `nodeId=local` 字段（不实现多机） | schema 向前兼容 |

**明确不做（本期）**：跨机器调度、Nomad 生产接入、k3s、单进程多租户改造 parent-bridge 全局单例、per-account 降级。

### 建议日历（单人全职约 5–7 周；两人可 P0 与 P1.1–1.2 部分重叠）

```text
W1     P0.1–P0.5  协议 + ingress-auth + nginx 同源
W2     P0.6–P0.8  客户端切 /dsh/  + 生产试点锁门
       P0.9 / P1.1 并行：测量 + pool 骨架
W3–W4  P1.2–P1.7  本地双租户
W5–W6  P2         systemd + 4G 机
W7     P3         等待室 + Nomad 决策门
```

---

## 10. 基线测量清单（P0.9，回填 §6）

| # | 测量 | 方法 | 达标阈值 |
|---|---|---|---|
| 1 | 单实例 idle RSS | `./middlewares/scripts/measure-dsh-capacity.sh --seconds 300` | 记录基线（预估 300–500MB） |
| 2 | 冷启动 | 启动到 `/muse/v1/parent-bridge/capabilities` | < 10s |
| 3 | agent 载荷 CPU | 一次文档任务 + `pidstat` | 界定 `ACTIVE_QUOTA=3` |
| 4 | 空闲事件频率 | 30min parent-bridge / heartbeat 日志 | idle TTL 20–30min |
| 5 | 同机 nginx+Cloud+PG 基线 | `free -m` | 界定 DSH 可用内存 |
| 6 | `LocalProcessExecutor` 起 2 实例增量 RSS | P1 后 | 确认无意外共享泄漏 |

---

## 11. 决策记录与风险

| 决策 | 选项 | 结论与理由 |
|---|---|---|
| 顺序 | 先池化 / 先鉴权 | **先鉴权（P0）**。共享实例在 P1 完成前必须继续跑 |
| 公网入口 | `dsh.` 子域 / Cloud 同源 | **同源 `/dsh/` 与 `/u/`**。否则 L0 无法用登录态 |
| 隔离粒度 | per (account, workspace) | 复用单 workspace 语义 |
| 执行载体 | systemd-run / docker / supervisord | 生产 systemd；Docker 仅构建/CI |
| 调度器 | Nomad / k3s / 自研 | **自研控制面 + Executor 接口**；Nomad 仅未来跨机器候选 |
| 动态路由 | nginx reload / Pool 反代 | **Pool 反代**，避免每租户 reload |
| 挂起 | 杀进程 + 盘上状态 | DSH 无 in-place suspend |
| 单进程多租户 | 改 keyed 单例 | **不做** |

**主要风险**：4GB 上一切取决于实测 RSS；idle > 600MB 则只能 ~3 并发且排队差 → 升 8GB 优先于软件。第二风险：P0 未完成就切多实例，把裸奔复制 N 份。第三：macOS 无 systemd，P1 用 `local` Executor 测语义、P2 必须有一台 Linux。

---

## 12. 复用点与既有资产映射

| 本方案需要的能力 | 已有资产 | 位置 |
|---|---|---|
| device token 签发/撤销 | Cloud BFF + Redis | `muse_dsh.rs`；`GATEWAY.zh-CN.md` §4 |
| JWT 或 device token 识别用户 | `MuseActor` | `muse_auth.rs` |
| workspace 成员 | `/api/muse/workspace/current` | `muse.rs`；`verifyMobileWorkspace` |
| session 类型 | `SessionGatewayV2` 等 | `core/remote-runtime/src/index.ts` |
| 每实例 profile | `dsh --profile` + `DSH_HOME` | harness `args.ts`；`deploy/entrypoint.sh` |
| 钉死 workspace 目录 | `MUSE_APPFLOWY_DSH_WORKSPACE*` | `parent-bridge.ts`；web-workspace-ingress §8 |
| launch token | `dsh web:` 日志 | `dsh_web_auth.dart` |
| 心跳 | mobile 60s、web SSE 15s | `appflowy_dsh_control_host.dart`；`parent-bridge.ts` |
| runtime 去重 | `@muse` 符号链接 | `deploy/entrypoint.sh`；`stage-dsh-runtime.sh` |
| 旧句柄失效 | `RemoteGenerationV2` | `remote-runtime` |

---

## 13. v0.2 相对 v0.1 变更摘要

- 宿主规格 2C/4G → **4C/4G**；ACTIVE_QUOTA 2 → 3；READY 仍由内存决定。
- P0 从「清单」扩成可打的宿主通道：同源入口、`ingress-auth`、`verifyHostWorkspace`、禁止误用 `/device-token/verify`。
- 调度从「写一个 supervisor」拆成 **控制面自研 + Executor 可替换**；给出 Nomad 采用门槛；明确本期不做多机。
- 动态路由改为 Pool 反代，避免 nginx reload。
- 开发计划拆成 P0–P3 带路径、验收和日历。
