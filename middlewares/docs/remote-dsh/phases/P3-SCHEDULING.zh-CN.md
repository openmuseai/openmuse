# P3 调度体验与 Nomad 门 — 设计 / 开发 / 测试

> 阶段：P3。仍单机。跨机器见 [ADR-001](../ADR-001-nomad-executor.zh-CN.md)。

## 1. 设计

已在 P1 控制面内实现，本阶段冻结契约并补指标/预留字段：

| 能力 | 行为 |
|---|---|
| 等待室 | `active>=ACTIVE_QUOTA` → `{queuePosition, retryAfterMs:10000}` 无 webUrl |
| LRU | `ready>=READY_QUOTA` 时 suspend 无 attachment 且最久未活跃的实例 |
| 指标 | `readCgroupMetrics`；无 cgroup 时不炸 |
| `nodeId` | 恒 `local`（`MUSE_DSH_NODE_ID` 可改字符串，不做放置算法） |
| Nomad | `ExecutorKind` 含 `"nomad"`，`InstanceHandle.allocId` 预留，**无实现** |

前端：`DshSessionOpen.isQueued`；轮询 `retryAfterMs`。

## 2. 开发

代码已在 `@muse/dsh-pool`。本阶段文档 + ADR + 矩阵即交付。不要在单机 4G 引入 Nomad/k3s。

## 3. 测试方案

复用 P1 等待室/LRU 单测。指标：无文件时 `{alive:false}`。ADR 评审：未满足门槛不得写 `NomadExecutor`。

## 4. 测试矩阵

| ID | 场景 | 期望 |
|---|---|---|
| P3-T1 | ACTIVE 用尽 | 排队，不新 start |
| P3-T2 | close+tick | 队首获得实例 |
| P3-T3 | READY 用尽 + idle | LRU suspend |
| P3-T4 | READY 用尽 + 全 busy | 排队，不 OOM 狂起进程 |
| P3-T5 | `nodeId` | open 结果为 `local` |
| P3-T6 | `kind==="nomad"` | 无生产代码路径 |
| P3-T7 | 无 cgroup | metrics 不抛 |
