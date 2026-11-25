# Host Sandbox Service、Lease 与 Local Runtime

> 状态：Accepted
>
> 子需求：X1
>
> 日期：2026-09-29

## 1. 结论

X1 落地独立的 `workspace.sandbox@1` 控制面和 Desktop Local Runtime。Host、DSH 与底层执行环境通过端口和 opaque attachment 解耦：通用协议只出现 Workspace/Revision/Lease identity 与固定逻辑路径，不暴露 checkout 的 Host 绝对路径，也不携带运行时凭据。

Local Runtime 分两档：

- `local-native`：复用 X0 已验收的 DSH local provider 组，面向可信 Desktop 用户；它能约束写入，但不能证明 Host 读取隔离；请求 `strictHostIsolation=true` 时必须 fail closed。
- `local-isolated`：容器/VM provider seam。本次提供容器 reference runtime 和可重复 TCK；它使用独立 mount namespace、非 root 用户、只读根、无网络、capability drop、PID/CPU/内存上限。

## 2. 服务边界

`WorkspaceSandboxService` 只管理：

- `create / attach / status / quiesce / prepareDraft / release`；
- actor、caller plugin、workspace、base revision、policy ceiling、registry digest；
- Lease TTL、generation、event sequence、attachment audience；
- cleanup receipt 与失败收敛。

`SandboxRuntimePort` 独占：

- checkout handle 到真实 substrate/mount 的解析；
- 进程、PTY、卷、容器或 VM identity；
- quiescence、draft 扫描和完整 process range 清理。

因此停用 Sandbox Plugin 只会逐 Lease 调用 runtime cleanup。清理失败被收敛为 `Degraded` 与失败 receipt，不 panic、不退出 Host。

## 3. 固定执行世界合同

| 路径 | 语义 | 生命周期 |
| --- | --- | --- |
| `/workspace` | 当前 checkout，可写性由 policy ceiling 决定 | Lease |
| `/runtime` | 冻结的 worker/runtime artifact | image/registry snapshot |
| `/home/agent` | 临时 Agent home | Lease |
| `/tmp` | 临时文件 | Lease |
| `/run/openmuse` | control socket、process metadata | Lease |

DSH consumer 只得到 audience-bound、短 TTL、generation-bound 的 attachment。Registry 更新必须显式调用 refresh 并提升 generation，旧 attachment 立即失效；Session 内不会静默换 worker。

## 4. 状态与失败语义

主路径为 `Ready → Quiescing → Quiescent → Released`。到期进入 `Expired`；runtime cleanup 失败进入 `Degraded`。`prepareDraft` 仅允许在 `Quiescent` 执行。存在活跃进程时，`RejectIfActive` 返回 `NOT_QUIESCENT`，`Terminate` 则必须终止完整 process range。

错误使用冻结域码：`UNSUPPORTED_TARGET / POLICY_DENIED / SANDBOX_UNAVAILABLE / ARTIFACT_UNAVAILABLE / LEASE_EXPIRED / STALE_GENERATION / QUOTA_EXCEEDED / NOT_QUIESCENT / DRAFT_CONFLICT / PROVIDER_FAILED`，并补充控制面 `NOT_FOUND / CONFLICT`。

## 5. 验收

统一入口：

```bash
./scripts/test_workspace_sandbox.sh
```

自动验收覆盖：

- 9 项 Rust 合同/状态机测试；
- `local-native` 严格隔离请求 fail closed；
- public lease JSON 不包含 checkout handle、Host path 或 credential；
- attachment audience、TTL、generation 与 registry refresh；
- expiry/revoke 和 plugin shutdown 的 cleanup receipt；
- 容器内 `rg`、`python3`、`cargo check --offline` 在 `/workspace` 工作；
- Host home、SSH、其他 Workspace 不在 mount namespace；容器只有 loopback；
- detached 子进程随容器 stop 被整体清理。

这证明 X1 的 Desktop isolated reference path；Cloud 多租户 runtime、镜像签名/SBOM、orphan sweeper 与远端 DSH provider 组属于 X2/X6。
