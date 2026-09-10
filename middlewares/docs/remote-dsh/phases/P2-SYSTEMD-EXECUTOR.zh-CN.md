# P2 systemd 执行器 — 设计 / 开发 / 测试

> 阶段：P2。生产单机；macOS 不跑 live systemd。

## 1. 设计

`SystemdRunExecutor` 把 `InstanceSpec` 编成 `systemd-run --unit=muse-dsh-<hash>`：

- `MemoryMax=<MB>M`、`CPUWeight`、`PidsMax=512`、`User=/--uid=`
- `DSH_HOME` 每租户目录 0700（由部署脚本保证；执行器传 `uidForTenant`）
- `MUSE_REQUIRE_HOST_AUTH=1`
- 探活复用 LocalProcessExecutor 的 loopback HTTP

idle TTL：Pool `tick()` 调 `executor.stop` → `systemctl stop`。resume 同一 `homeDir`+port。

Docker **不算** P2 验收。

## 2. 开发

| 路径 | 内容 |
|---|---|
| `dsh-pool/src/systemd-executor.ts` | `systemdRunArgv` + 可注入 `run()` |
| `dsh-pool/src/metrics.ts` | `memory.current` / `cpu.stat` |
| `dsh-pool/src/start.ts` | `MUSE_DSH_EXECUTOR=systemd` |
| 部署 | `/opt/muse-dsh/runtime` 只读 stage；`/srv/muse-dsh/<hash>/home` |

Linux 验收：

```bash
MUSE_DSH_EXECUTOR=systemd MUSE_DSH_RUNTIME_BIN=/opt/muse-dsh/runtime/node \
  pnpm exec tsx src/start.ts
```

## 3. 测试方案

- 单测：argv 含 `MemoryMax=512M`、`--uid=`、`MUSE_REQUIRE_HOST_AUTH=1`（本阶段 CI 必过）。
- Linux 手工：`systemd-cgtop`、租户 A 读不了租户 B 的 `DSH_HOME`、resume < 10s。
- Darwin：不调用真 `systemd-run`。

## 4. 测试矩阵

| ID | 场景 | 期望 | 环境 |
|---|---|---|---|
| P2-T1 | `systemdRunArgv` | MemoryMax/uid/host-auth | CI |
| P2-T2 | 注入 `run()` start/stop | 调用 `systemd-run` / `systemctl stop` | CI（可加） |
| P2-T3 | cgroup 文件缺失 | `inspect.alive===false`，不抛 | CI |
| P2-T4 | 真 unit MemoryMax | `memory.max` = 512M | Linux |
| P2-T5 | 跨 UID 读文件 | EACCES | Linux |
| P2-T6 | idle suspend/resume | 同目录，<10s | Linux |
| P2-T7 | Docker 起实例 | **不算通过** | — |

## 5. 生产验证（2026-09-09）

宿主机 systemd 249 + cgroup v2。`PidsMax=` 在 249 上是非法 assignment，改用 `TasksMax=512`。`systemd-run` 必须 `--no-block --quiet` 且 `StandardOutput=journal`，否则非 TTY 下默认 `--pipe` 会把 DSH stdout 接到 pool 的 `execFile` 管道，写满后 HTTP 卡死。首次 live 未启用 per-tenant `--uid=`（系统无 16000+ 用户）。`MemoryMax=512M`。Docker 只留镜像/BFF，不跑实例。
