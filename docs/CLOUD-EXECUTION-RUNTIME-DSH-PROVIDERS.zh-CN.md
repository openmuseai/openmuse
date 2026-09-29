# Cloud Execution Runtime 与 DSH Provider 组

> 状态：Accepted
>
> 子需求：X2
>
> 日期：2026-09-29

## 1. 边界

X2 将 X1 的 Lease 映射到 tenant/task 独占的 container/microVM execution world。`@openmuse/dsh-workspace-runtime/fs`、`subprocess` 和 `sandbox` 是同一 runtime 的视图，不改变 Agent Loop，也不复用只适合本机路径的 `dsh-fs-sandbox`。

ST2 materialization 只向 runtime pool 交付 opaque checkout handle。Cloud runtime descriptor、DSH attachment 和日志均不出现 S3 key、Host path 或 storage credential。

## 2. 双层策略

- Outer sandbox ceiling：mount namespace、tenant/task identity、image digest、SBOM、network/device/quota；它不能被 `danger-full-access` 或 DSH policy 放宽。
- DSH per-call policy：在 outer ceiling 内决定只读或 Workspace 写、命令 deadline、前后台行为。

路径必须 canonicalize 到 `/workspace/*`；绝对 Host 路径、相对路径与 `..` fail closed。Control attachment 绑定 tenant、audience、generation 和 TTL。

## 3. 生命周期与观测

Runtime pool 记录 allocations、cold starts、cancellations、crashes、orphan cleanups 和 terminated process 数。cancel/crash/expiry/orphan 都使 attachment 失效、清空临时卷并终止完整 process range。Heartbeat 超时和 Lease expiry 由同一 sweeper 收敛。

生产 substrate 必须固定 image digest/version/SBOM digest；未通过 X6 签名、撤销和安全 Gate 的 image 不得 admission。

## 4. 验收

统一入口：

```bash
./scripts/test_cloud_execution_runtime.sh
```

5 项 TCK 覆盖 FS↔Bash、PTY、LSP framing 的同世界可见性，outer sandbox 越界拒绝，跨租户卷/进程/attachment 隔离，image/SBOM pin，以及取消、崩溃、orphan cleanup 指标。当前使用 fake checkout；替换 ST2 materializer 不改变 Provider 合同。
