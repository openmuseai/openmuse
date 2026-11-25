# Cloud Execution Runtime 与 DSH Provider 组

> 状态：Accepted（control plane + DSH 0.1.7 Provider integration）
>
> 子需求：X2
>
> 日期：2026-09-29

## 1. 边界

X2 将 X1 的 Lease 映射到 tenant/task 独占的 container/microVM execution world。`@openmuse/dsh-workspace-runtime/fs`、`subprocess` 和 `sandbox` 是同一 runtime 的视图，不改变 Agent Loop，也不复用只适合本机路径的 `dsh-fs-sandbox`。

ST2 materialization 只向 runtime pool 交付 opaque checkout handle。Cloud runtime descriptor、DSH attachment 和日志均不出现 S3 key、Host path 或 storage credential。

实际 DSH `0.1.7-rc.1` Provider 包位于
`third_party/dsh/plugins/openmuse-dsh-workspace-runtime`，并以固定 tarball、npm
lock integrity 进入产品 DSH closure。其 `cordis.patch.yml` 只替换
`subprocess/sandbox/fs-sandbox` 三个 Provider row，不修改 Agent Loop 和模型可见
Tool。Rust `openmuse-cloud-execution-runtime` 仍是 substrate-neutral control-plane
reference；JavaScript 包是发布闭包中的 DSH adapter，二者不互相反向依赖。

## 2. 双层策略

- Outer sandbox ceiling：mount namespace、tenant/task identity、image digest、SBOM、network/device/quota；它不能被 `danger-full-access` 或 DSH policy 放宽。
- DSH per-call policy：在 outer ceiling 内决定只读或 Workspace 写、命令 deadline、前后台行为。

路径必须 canonicalize 到 `/workspace/*`；绝对 Host 路径、相对路径与 `..` fail closed。Control attachment 绑定 tenant、audience、generation 和 TTL。

`resolve/listDir` 的 wire result 同时携带 opaque target key、沙箱内 canonical
process path、file URL 和 opaque ancestor keys。Provider 缓存这些元数据以满足 DSH
同步 `processPath/fileUrl/contains` 合同，但不会把 Host path 编进 target key 或返回值。
Sandbox Provider 向 control plane 申请一次性 launch token；只有 remote subprocess
endpoint 能在同一个 runtime 消费它，token 重放或 enforcement 不是 `full` 时 fail
closed。Transport adapter 负责把远端 process/PTY channel 映射为 DSH 原生
Readable/Writable/handle，不把网络协议泄露到 Agent 或 Tool 层。

## 3. 生命周期与观测

Runtime pool 记录 allocations、cold starts、cancellations、crashes、orphan cleanups 和 terminated process 数。cancel/crash/expiry/orphan 都使 attachment 失效、清空临时卷并终止完整 process range。Heartbeat 超时和 Lease expiry 由同一 sweeper 收敛。

生产 substrate 必须固定 image digest/version/SBOM digest；未通过 X6 签名、撤销和安全 Gate 的 image 不得 admission。

## 4. 验收

统一入口：

```bash
./scripts/test_cloud_execution_runtime.sh
./scripts/test_dsh_remote_provider.sh
```

Rust 的 5 项 TCK 覆盖 runtime pool、FS↔Bash、PTY/LSP reference、outer
sandbox、租户隔离、image/SBOM pin 和生命周期指标。DSH integration TCK 直接加载
产品闭包中的 `@deepseek-ai/cordis` 与三个 Service Definition，覆盖：

- FS 写入后 subprocess 立即读取、Bash 写入后 FS 立即读取；
- raw pipe 完整保留 LSP `Content-Length` frame；
- terminal input/output、abort、managed process settlement；
- `read-only`、`..` escape、未知 target 和 Host-path mapping fail closed；
- attachment audience/TTL 的 client preflight 与 server-side 二次鉴权；
- 从全新 npm lock 构建的 closure 确实包含 Provider 包与 Cordis overlay。

当前 remote TCK 使用本机 fixture transport 模拟 Workspace Sandbox service；它验证
Provider/transport ABI 和同世界语义，不等价于 container/microVM 生产安全认证。真实
substrate 的跨租户、egress、quota、故障注入和渗透门禁仍由 X6 负责；替换 ST2
materializer 不改变 Provider 合同。
