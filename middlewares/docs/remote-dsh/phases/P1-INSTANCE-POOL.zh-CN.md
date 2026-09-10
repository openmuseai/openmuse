# P1 本地实例池 — 设计 / 开发 / 测试

> 阶段：P1。包：`middlewares/dsh/core/dsh-pool`（`@muse/dsh-pool`）。

## 1. 设计

控制面只做租户语义；进程拉起走 `Executor`。本期实现 `FakeExecutor`（单测）与 `LocalProcessExecutor`（macOS/CI 真进程）。

- `tenantKey = sha256(salt || account || workspace)`；URL 只用 128bit `tenantHash`。
- `session/open` 幂等；满 `ACTIVE_QUOTA` 进等待室；满 `READY_QUOTA` 先 LRU 挂起空闲实例。
- Cloud BFF：`UserUuid` + `require_member_workspace` 后，把 `accountRef`（来自 JWT，**禁止客户端声称**）转到 `POST http://127.0.0.1:13079/internal/session/open`。
- 反代 `/u/<hash>/`，剥 Cookie 与三节 JWT。

## 2. 开发

| 路径 | 内容 |
|---|---|
| `dsh-pool/src/pool.ts` | 状态机、配额、idle、等待室、`nodeId=local` |
| `dsh-pool/src/local-executor.ts` | spawn + 探活 + launch token |
| `dsh-pool/src/server.ts` | `/internal/session/*` |
| `dsh-pool/src/proxy.ts` | `/u/<hash>/` |
| `dsh-pool/src/start.ts` | 控制面 + 反代双端口 |
| `AppFlowy-Cloud/.../muse.rs` | `session/open\|close\|heartbeat` |
| `muse_pool.rs` | reqwest 到 `MUSE_DSH_POOL_URL` |
| `muse-dsh-mobile` | `DshSessionApi`、`fromWebUrl`、coordinator 可选 session 释放 |

启动（开发）：

```bash
cd middlewares/dsh/core/dsh-pool
MUSE_DSH_EXECUTOR=fake MUSE_DSH_POOL_PORT=13079 pnpm exec tsx src/start.ts
export MUSE_DSH_POOL_URL=http://127.0.0.1:13079
```

## 3. 测试方案

Pool 单测不启真 DSH。`LocalProcessExecutor` 用 `tests/fixtures/fake-harness.mjs`。Cloud session 在无 pool URL 时 `FeatureNotAvailable`。

## 4. 测试矩阵

| ID | 场景 | 期望 |
|---|---|---|
| P1-T1 | 同 account+ws 两次 open | 同一 `instanceRef` |
| P1-T2 | 两租户 | 两次 start，不同 URL |
| P1-T3 | ACTIVE=1 时第二租户 | `queuePosition`，无 webUrl |
| P1-T4 | close + tick | 等待室排空 |
| P1-T5 | idle TTL | `suspended`；再 open 同 `homeDir` |
| P1-T6 | READY=1 且已 idle | LRU suspend 后开新租户 |
| P1-T7 | 反代带 Cookie + JWT | 上游无 Cookie、无 Authorization |
| P1-T8 | 反代 device token（两段） | 可保留（非 JWT） |
| P1-T9 | session API Dart | 打到 `/api/muse/dsh/session/*` |
| P1-T10 | `fromWebUrl` 允许 query；`tryParse` 拒绝 | 编译期 URL 无 token |

## 5. 生产验证（2026-09-09，`openmuseai.com`）

`dsh-pool` 以 systemd 跑在 `127.0.0.1:13079`（控制）/`13080`（`/u/` 反代）。BFF `MUSE_DSH_POOL_URL=http://127.0.0.1:13079`。配额 `READY=1` `ACTIVE=1`。官方 Cloud 0.16.5 仍无 `muse_pool.rs`；session 由 BFF 转发。

| 项 | 结果 |
|---|---|
| 冷启动 `session/open` | **200**，约 170s（tsx 首次编译） |
| 热路径再次 `session/open`（同源 nginx） | **200**，约 6s，同一 `webUrl` |
| Cookie 打开 `webUrl` `/u/<hash>/` | **200** DSH HTML |
| 无 Cookie `/u/<hash>/` | **401** |
| Docker `muse-dsh` | 已 stop，`--restart=no` |
