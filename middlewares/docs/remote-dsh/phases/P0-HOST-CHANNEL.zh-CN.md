# P0 宿主通道 — 设计 / 开发 / 测试

> 阶段：P0（先于实例池）。配套 [MULTITENANCY.zh-CN.md](../MULTITENANCY.zh-CN.md) §5.2。

## 1. 设计

目标：Remote DSH 在仍是**共享单实例**时，也不能匿名打开、不能绑别人的 AppFlowy workspace。Cloud JWT 永不进入 DSH 进程。

```
浏览器/App  --GoTrue JWT/Cookie--> Cloud BFF
     |  device token
     v
nginx auth_request --> GET /api/muse/dsh/ingress-auth  (MuseActor: JWT | device token | cookie)
     |
     v  (Cookie 剥掉)
DSH parent-bridge --device token--> POST /api/muse/workspace/current
     |
     v
Postgres 成员表
```

| 层 | 实现 |
|---|---|
| L0 | `ingress-auth` + nginx `auth_request`；同源 `/dsh/` 见 `deploy/nginx/cloud-same-origin-dsh.conf`；独立子域 vhost 在 `remote-infra-setup.sh` 注入 |
| L2 | `verifyHostWorkspace`；Cloud URL 存在则默认 `MUSE_REQUIRE_HOST_AUTH=on`；远程强制 `workspace.bind` |
| L3 | `remote-disable-dsh-passwords.sh` 需 `ALLOW_DISABLE_DSH_PASSWORDS=1` |

不变量 **I0**：无有效 device token / 非成员 → 不 pin workspace。

## 2. 开发

| 路径 | 改动 |
|---|---|
| `plugins/dsh-appflowy/src/mobile-lease.ts` | `verifyHostWorkspace` + 20s 缓存；http 允许 `.test`/`.invalid` |
| `plugins/dsh-appflowy/src/parent-bridge.ts` | `hostAuthRequired`；bind 回源；401/403/503 |
| `AppFlowy-Cloud/src/biz/muse_auth.rs` | `extract_ingress_token`（Bearer / `X-Muse-Device-Token` / Cookie） |
| `AppFlowy-Cloud/src/api/muse.rs` | `GET/HEAD /api/muse/dsh/ingress-auth` |
| `deploy/nginx/cloud-same-origin-dsh.conf` | 同源 `/dsh/` `/u/` |
| `deploy/scripts/remote-infra-setup.sh` | 子域 vhost `auth_request` |
| `deploy/scripts/remote-disable-dsh-passwords.sh` | 默认拒绝 |
| Web iframe | 指向同源 `/dsh/`（AppFlowy-Web 仓；本仓无 `DshAgentPanel.tsx`） |
| Mobile | `DshRemoteConfig` 接受 `/dsh/` 路径 |

桌面 sidecar：无 `MUSE_DOCUMENT_CLOUD_URL` 时 `hostAuthRequired()===false`。

## 3. 测试方案

- 单测：无 Cloud URL 的既有 bind 回归；有 Cloud URL 时无 token / 伪 token / 跨 ws / 成功 pin / 缓存。
- HTTP：mobile-http 仍用 fixture fetch。
- Cloud：`extract_ingress_token` 单测。
- 手工：未登录 curl `/dsh/` → 401；试点账号 iframe hello+bind。

## 4. 测试矩阵

| ID | 场景 | 期望 |
|---|---|---|
| P0-T1 | 桌面无 Cloud URL，`workspace.bind` | 仍 pin（sidecar） |
| P0-T2 | Remote 无 deviceToken bind | `NO_DEVICE_TOKEN` / HTTP 401 |
| P0-T3 | 伪 token | `DEVICE_AUTH_REJECTED` / 401 |
| P0-T4 | 合法 token + 他人 workspaceId | `SCOPE_MISMATCH` / 403 |
| P0-T5 | 合法 token + 自己的 workspace | `ok, bound` |
| P0-T6 | 20s 内重复校验 | 不二次 fetch |
| P0-T7 | Cookie `access_token` | `extract_ingress_token` 命中 |
| P0-T8 | 无 `ALLOW_DISABLE_DSH_PASSWORDS` | 脚本 exit 1 |
| P0-T9 | 未登录 nginx `/dsh/` | 401（需 Cloud+nginx） |

## 5. 生产验证（2026-09-09，`openmuseai.com`）

现网 `appflowyinc/appflowy_cloud:latest`（0.16.5）**没有** `/api/muse/*`；Postgres 已应用到 `20260620090000`，本仓 Cloud 迁移只到 `20250723072011`，**不能**用本仓镜像替换官方 Cloud。

过渡：在 **不替换** 官方 Cloud 的前提下，loopback `muse-dsh-bff`（`@muse/dsh-pool` `muse-bff`）承接 `/api/muse/*`。nginx `location /api/muse` 与 `/dsh/`、`/u/` 的 `auth_request` 都打到 `127.0.0.1:8010`。Web iframe 先 `session/open` 再打开同源 `/u/<hash>/`；配置里的 `/dsh/` 只作失败回退。共享 Docker 实例 `muse-dsh` 已停，生产执行器是 `systemd-run`（`dsh-pool`）。

| ID | 现网结果 |
|---|---|
| P0-T9 未登录 `GET https://openmuseai.com/dsh/` | **401** |
| 未登录 `GET /u/<hash>/` | **401** |
| 有效 Cookie 打开 `/u/<hash>/` | **200**，DSH HTML |
| 有效 Cookie 打开 `/dsh/` | **502**（Docker 共享实例已停；Web 走 `/u/`） |
| `POST /api/muse/dsh/device-token` + JWT | **200**，两段式 token |
| device token / Cookie `ingress-auth` | **200** |
| `POST /api/muse/workspace/current` 本工作区 | **200**，`workspaceId` 一致 |
| `POST /api/muse/dsh/session/open` | **200**，`webUrl=https://openmuseai.com/u/<hash>/` |
| `/app` `/api/health` | 未回归 |

