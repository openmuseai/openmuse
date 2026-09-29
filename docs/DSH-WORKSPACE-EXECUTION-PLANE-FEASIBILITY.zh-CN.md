# OpenMuse Workspace Sandbox 与 DSH Execution Plane 设计评估

> 状态：独立需求 / Architecture Decision Candidate
>
> 范围：OpenMuse Workspace Sandbox、插件 CLI 生态与 DSH 执行面的集成
>
> 不属于：Mobile 产品方案、S3 存储选型或其实施计划
>
> 结论日期：2026-09-29

## 1. 执行摘要

可以在**不修改 DSH 核心，或仅做极少上游改动**的前提下，为 OpenMuse 增加统一的 Bash/CLI 执行面。进一步分析后，这个执行面应被正式定义为 **OpenMuse Workspace Sandbox**，而不是一个 Bash 适配器。

关键不是再实现一个 `bash` Tool。DSH 已经具备完整链路：

- `dsh-tool-bash` 向模型暴露 Bash Tool；
- `ctx.shell` 负责 shell 语义、前后台任务和工作目录；
- `ctx.subprocess` 决定程序实际在哪个执行环境运行；
- `ctx.fs` 决定结构化文件工具、LSP 等看到哪个文件系统；
- Cordis Profile/Patch 可以替换 Provider，而不需要修改 Agent Loop。

因此，推荐方案是引入独立的 **OpenMuse Workspace Sandbox Runtime（WSR）**：它把一个 Workspace Revision 物化成隔离执行环境中的本地文件系统，装配经过 Host 验证和授权的 Plugin CLI 能力，再为 DSH 提供成对的 `ctx.fs` 与 `ctx.subprocess` Provider。现有 Bash、PTY、后台任务、文件 Tool 和 LSP 继续作为消费者使用。

WSR 同时服务于 Host 插件生态和 DSH，但不归属于其中任何一方：Host 是插件、权限和资源权威；DSH 是 Agent 消费者；WSR 是受控执行环境。

```text
               OpenMuse Host / Platform Broker
          Plugin Catalog · Grants · Resource Authority
                    │                     │
          verified CLI registry      Workspace Revision
                    │                     │
                    └──────────┬──────────┘
                               ▼
              OpenMuse Workspace Sandbox Runtime
        lease · materialize · isolate · dispatch · audit
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
  /workspace checkout   approved CLI workers   sandbox policy
        │                      │                      │
        └──────────────────────┼──────────────────────┘
                               │
                    Same Execution World
                               │
               ┌───────────────┴────────────────┐
               │                                │
        DSH Bash / FS / PTY / LSP       Office/Domain CLI workers
               │                                │
               └───────────────┬────────────────┘
                               │
             Local native / container / VM / Cloud runtime
                               │
                 hydrate / draft / checkpoint / CAS
                               │
                    Local / S3 / BYOS Blob Layer
```

这条路线同时遵循两套插件架构：OpenMuse Plugin 通过 Host Broker 声明 UI、Service 和 Agent CLI Contribution；DSH Plugin 通过 Capability Seam 消费同一个 Sandbox Lease。两者都不知道底层是本机、云端、配对 Desktop、RustFS 还是其他 Workspace Provider。

### 结论评级

| 项目 | 判断 |
| --- | --- |
| DSH 架构适配度 | 很高 |
| PoC 技术可信度 | 9/10 |
| 生产方案技术可信度 | 7/10，主要风险在 Workspace Sandbox Runtime，而非 DSH |
| PoC 对 DSH 核心侵入 | 0，外部插件 + Profile Patch |
| 推荐生产 V1 对 DSH 核心侵入 | 0 或极少 |
| OpenMuse Host 协议改动 | 中等：Manifest/Protocol 需增加 CLI Artifact 与 Sandbox Service 合同 |
| 不推荐的高侵入场景 | 在一个 DSH Realm 内复用同一个 `/workspace` 并并发路由到多个执行世界 |
| Linux Cloud MVP | 14～22 人周 |
| 含 Plugin CLI 生态的生产级 Local + Cloud | 32～48 人周；3 人并行约 14～22 周 |

## 2. 需求边界

### 2.1 要解决的问题

Agent 应能在不同 Workspace 上使用同一种交互模型：

```bash
cd /workspace
rg "OldApi" .
python scripts/migrate.py
cargo test
office docx render docs/report.docx
git diff --stat
```

Workspace 可以来自：

- 当前 Desktop 的本地目录；
- OpenMuse Cloud Workspace；
- 用户自选 S3/S3-compatible 存储；
- 用户配对的另一台 Desktop；
- 未来的企业私有 Workspace、边缘节点或其他 Provider。

Agent 不应感知 bucket、endpoint、RustFS/MinIO 实现，也不应持有 S3 凭据。

### 2.2 “运行任意程序”的准确含义

“任意程序”不能解释为“任意访问宿主机”。它应定义为：

> Agent 可以执行当前 Execution Image 或 Workspace 内已安装、且被安全策略允许的可执行程序。

以下能力仍需策略或用户授权：

- 安装系统包或运行安装脚本；
- 访问公网或企业内网；
- 读取 Workspace 之外的目录；
- 使用摄像头、GPU、USB、Docker Socket 等设备或宿主能力；
- 获取新的密钥、OAuth Token 或云资源权限；
- 启动长期服务或超出配额的后台进程。

### 2.3 明确不做

本需求不负责：

- 决定官方对象存储选 RustFS 还是 MinIO；
- 设计 Mobile/Remote Desktop 产品交互；
- 用 Bash 替代 Workspace 版本、分享、审计和权限领域；
- 把 S3 伪装成逐 syscall 的网络文件系统；
- 让 DSH 成为 Workspace 的数据权威。

## 3. 当前架构与源码结论

本节结论基于当前仓库中的 DSH 源码，而不是仅基于概念设计。

### 3.1 DSH 的能力接缝已经存在

DSH 架构文档把 Service Definition / Provider / Consumer 定义为主要扩展接缝。新增环境能力应通过插件提供服务，而不是进入 Agent Loop。详见 [DSH architecture](../../vendors/deepseek-harness/docs/architecture.md)。

当前关键契约如下：

| 能力 | DSH 契约 | 当前消费者 | WSR 的接入点 |
| --- | --- | --- | --- |
| Shell 语义 | `ctx.shell` | `dsh-tool-bash` | 通常复用，不必重写 |
| 进程执行 | `ctx.subprocess` | Bash、PTY、LSP | 实现 OpenMuse Provider |
| 文件系统 | `ctx.fs` | 文件 Tool、LSP、资源读取 | 实现 OpenMuse Provider |
| 文件约束 | `ctx.sandbox` / FS policy | sandboxed Bash/FS | 本地复用；远端需准确建模 |
| Tool 注册 | `ctx.tools` | Agent Loop | 复用现有 Bash Tool |
| 组合与替换 | Cordis Profile/Patch | Host 启动配置 | OpenMuse 生成 Workspace Profile |

相关源码：

- [Shell Service 定义](../../vendors/deepseek-harness/packages/shell/shell/src/index.ts)
- [Shell 请求类型](../../vendors/deepseek-harness/packages/shell/shell/src/types.ts)
- [Bash Tool](../../vendors/deepseek-harness/packages/shell/tool-bash/src/index.ts)
- [Subprocess Service](../../vendors/deepseek-harness/packages/subprocess/subprocess/src/index.ts)
- [Subprocess 类型](../../vendors/deepseek-harness/packages/subprocess/subprocess/src/types.ts)
- [FileSystem Service](../../vendors/deepseek-harness/packages/fs/fs/src/index.ts)
- [Sandbox Service](../../vendors/deepseek-harness/packages/sandbox/sandbox/src/index.ts)

`dsh-bash-local` 的名字容易引起误解：它主要把 Bash 语义转换成 `ctx.subprocess` 调用。只要替换 `ctx.subprocess` Provider，Bash 就可以执行在远端或隔离环境，通常不需要 fork `dsh-tool-bash`。

### 3.2 Base Bundle 已经包含 Bash

DSH Base Bundle 当前默认组合包括：

- local subprocess；
- local sandbox 与 sandbox policy；
- sandboxed Bash；
- Bash Tool；
- FS Tool 与 sandboxed FS。

配置证据见 [base bundle patch](../../vendors/deepseek-harness/packages/bundle/base/cordis.patch.yml)。所以本需求的本质是**替换执行 Provider**，不是向模型增加另一套同名 Bash Tool。

### 3.3 DSH 已有远端 Execution World 的验证先例

DSH 仓库中的 E2B PoC 已验证：

- Host 上保留 Agent Loop、Session 和 Model；
- 文件、普通进程、PTY 和 LSP 进入同一个远端 Linux Sandbox；
- `ctx.fs` 写入的文件立即能被 Bash 看见；
- Bash 写入的文件立即能被 `ctx.fs` 看见；
- 消费者无须为 E2B 分叉。

参见：

- [E2B README](../../vendors/deepseek-harness/packages/e2b/e2b/README.md)
- [E2B composition fixture](../../vendors/deepseek-harness/packages/e2b/e2b/tests/fixtures/composition/cordis.yml)
- [E2B composition e2e](../../vendors/deepseek-harness/packages/e2b/e2b/tests/composition.e2e.ts)
- [Portable execution-world decision note](../../vendors/deepseek-harness/.agents/notes/implemented/architecture/2026-07-28-portable-execution-world-consumers.md)

E2B 是架构验证先例，不应直接当作 OpenMuse 的生产依赖。其当前实现仍是 PoC，未覆盖持久 Workspace、断线恢复、版本提交、BYOS、网络策略和完整多租户生命周期。

### 3.4 “同一 Execution World” 是不可破坏的约束

`ctx.fs` 和 `ctx.subprocess` 必须指向同一文件系统视图。例如：

1. `ctx.fs.write("/workspace/a.txt")`；
2. Bash 执行 `cat /workspace/a.txt`；
3. 两者必须观察到同一字节内容和元数据。

如果只远程执行 Bash、但文件 Tool 仍指向 Host 本地目录，将产生双重真相，PTY/LSP 也会看到不同内容。这不是最终一致性能够合理掩盖的问题，而是错误的 Provider 组合。

### 3.5 当前 OpenMuse 集成仍是 Host Path 模式

当前实现：

- [DSH sidecar](../plugins/dsh-agent/lib/src/dsh_sidecar.dart) 启动打包的 `dsh web`；
- [Workspace binding](../plugins/dsh-agent/lib/src/dsh_workspace_binding.dart) 把 Host Snapshot 解析成本地绝对路径，并记录 `host-path` materialization；
- [Workspace sync](../plugins/dsh-agent/lib/src/dsh_workspace_sync.dart) 把这些 mount 信息同步给 DSH bridge。

这能支持当前本地 Desktop，但还没有统一的 Sandbox Lease，也没有 Cloud checkout、远端 subprocess、checkpoint/CAS 或运行时隔离。

另一个需要提前消除的风险是版本漂移：

- OpenMuse 打包闭包当前锁定 `@deepseek-ai/dsh` `0.1.7-rc.1`；
- 本仓库 `vendors/deepseek-harness` 源码包版本处于 `0.1.5-rc.2` 系列；
- E2B Provider 不在当前 OpenMuse 的生产 npm 闭包中。

因此，vendor 源码可作为架构证据，但第一阶段必须针对实际打包的 `0.1.7-rc.1` 做契约锁定和兼容测试，不能直接复制 PoC 包。

### 3.6 当前 OpenMuse Plugin Runtime 的可复用基础与缺口

当前 Host 架构已经具备正确的上层边界：

- Host 拥有 Plugin Catalog、Contribution Registry、权限、生命周期和审计；
- 插件之间不能直接 import，必须经过 Command/Service Broker；
- Resource Authority 已定义 `resourceRef / revision / materialization / commit`；
- External Runtime Plugin 被定义为独立进程，是未来可安装插件的主路径；
- `workingCopy` materialization 已进入 Resource Contract，可作为 Sandbox checkout 的下层原语。

源码与合同见：

- [Host / Plugin / DSH 总体架构](PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md)
- [Plugin Manifest v1 schema](../schemas/openmuse.plugin.v1.schema.json)
- [Rust plugin protocol](../crates/openmuse-plugin-protocol/src/lib.rs)
- [Platform Broker](../crates/openmuse-platform-runtime/src/lib.rs)
- [Resource Contract](../packages/muse_resource_contract/lib/src/resource.dart)

但当前实现还不能直接承载 Agent CLI 生态：

| 当前事实 | 对 Sandbox 设计的影响 |
| --- | --- |
| Manifest v1 只有 commands/services/editors/panels | 需要新增 `agent_cli` 与 platform artifact contribution |
| JSON Schema 使用 `additionalProperties: false` | 不能私自塞字段；必须正式升级 schema 和 Rust contract |
| Runtime kind 只有 built-in/native-process/web-view | 需要定义 `sandbox-worker` artifact，而不一定新增 Host runtime kind |
| Broker 只验证调用者 grants 与 contribution required permissions | 需要增加 Agent 主体 + 目标 Plugin 主体 + Session ceiling 的联合授权 |
| 当前外部插件生命周期仍是架构基线而非完整生产 Supervisor | 签名、哈希、进程隔离、热更新和回滚需要继续实现 |
| DSH binding 当前是 `host-path` | 需要改为不透明 Sandbox Lease，路径只在执行世界内部出现 |

因此，DSH 侧可以低侵入，但 OpenMuse 自身必须补齐正式的 Sandbox/CLI Plugin ABI。这是产品平台能力，不应藏在 DSH 插件内部。

## 4. 推荐目标架构

### 4.1 核心对象：Workspace Sandbox Lease

建议由 OpenMuse Host/Cloud Control Plane 创建一个短期、可撤销的 `WorkspaceSandboxLease`：

```text
WorkspaceSandboxLease
├── leaseRef                 # 不透明标识，不是云凭据
├── workspaceRef
├── baseRevision
├── actorRef / pluginRef / grants
├── executionClass           # local-native / local-isolated / cloud
├── runtimeImageRef
├── rootPath                 # 执行世界内部，例如 /workspace
├── policyCeilingRef         # 外层不可突破的权限上限
├── cliRegistryDigest        # 已验证、冻结的 CLI 能力快照
├── providerAttachmentRef    # ctx.fs/subprocess 的不透明附着句柄
├── expiresAt
└── commitCapabilityRef      # 可选、最小权限、不可暴露给子进程
```

一个 Lease 绑定一个 Workspace Revision、一个 Agent/用户主体、一个冻结的插件能力集合和一个 Execution World。DSH 只得到调用 Provider 所需的 attachment；普通子进程只看到 `/workspace` 和允许的 CLI，不得到 S3 Secret、Host 主 Token 或 Plugin Catalog 写权限。

### 4.2 分层与领域归属

| 领域 | 职责 | 不负责 |
| --- | --- | --- |
| DSH Agent | 推理、Tool 调度、事件记录 | 存储协议、版本权威、容器编排 |
| DSH Shell/FS consumers | Bash、PTY、LSP、结构化文件操作 | 决定 Workspace 存在哪里 |
| DSH WSR Adapter | 把 `ctx.fs`/`ctx.subprocess` 映射到同一 Execution World | Plugin 安装和 Workspace 业务权限真相 |
| Workspace Sandbox Runtime | hydrate、隔离、进程/CLI worker 生命周期、draft、quiescence | 用户/组织领域策略 |
| Workspace Authority | Revision、ACL、checkpoint、冲突、审计、分享 | 执行任意进程 |
| Plugin Authority | 签名、安装状态、artifact digest、CLI contribution、版本选择 | Workspace 内容和 DSH 会话 |
| Resource/Blob Layer | 内容寻址 Blob、S3 ABI、BYOS | 路径语义和 Agent Tool |

这保持了关注点分离：DSH 不知道 S3，S3 不知道 Bash，Bash 不拥有 Revision，Workspace Authority 不实现 shell，Workspace 中的文件也不能自行声明“我是已安装 Plugin”。

### 4.3 两层 Plugin 适配，而不是一个巨型插件

“Everything is Plugin” 在这里有两个独立层次：

```text
OpenMuse Plugin Plane                       DSH Plugin Plane
────────────────────                       ────────────────
com.openmuse.workspace-sandbox              @openmuse/dsh-workspace-runtime
  provides workspace.sandbox@1                consumes Sandbox Lease
  consumes Resource/Plugin Authority          provides ctx.fs + ctx.subprocess
                 │                                      │
                 └──────── opaque lease/receipt ────────┘

Office/Domain Plugin
  ├── UI contribution        → Human
  ├── Service contribution   → Host/other authorized plugins
  └── Agent CLI contribution → Sandbox Supervisor/Agent
```

Host 级 `com.openmuse.workspace-sandbox` 应作为 External Runtime Plugin 或受同等生命周期约束的内置平台插件，提供 `workspace.sandbox@1` Service。DSH 插件只是该 Service 的一个消费者，并加载 DSH 侧 adapters。这样未来其他 Agent Runtime、自动化任务或批处理也能复用 Sandbox，不需要依赖 DSH。

Office Plugin 可以在同一个签名 Bundle 中携带 UI adapter、Host service runtime 和 headless sandbox worker，并共享同一 Rust Core；但它们是**不同进程/入口和不同权限主体**，不能把正在服务 UI 的 Host Plugin 进程直接借给不可信 Agent。

### 4.4 Provider 插件拆分

建议新增独立 npm 包，而不是修改 DSH 核心包：

| 插件 | 责任 |
| --- | --- |
| `@openmuse/dsh-workspace-runtime` | 从 Host Sandbox Service 获取并附着 Lease；不拥有 Workspace/Plugin 权威 |
| `@openmuse/dsh-workspace-fs` | 实现 `ctx.fs`，所有操作进入 Lease 的文件系统 |
| `@openmuse/dsh-workspace-subprocess` | 实现 `ctx.subprocess`，含普通进程、信号、process range、PTY |
| `@openmuse/dsh-workspace-sandbox` | 把 DSH per-call policy 映射为执行世界内的 wrapper；PoC 可暂缺，生产 full enforcement 必需 |
| `@openmuse/dsh-tool-workspace` | checkpoint/history/restore/share/sync/status 等领域 Tool |
| `openmuse-workspace` CLI | 给 Bash 使用的高级 Workspace CLI |

继续复用 DSH 的：

- `dsh-tool-bash`；
- shell executor 和 jobs；
- terminal consumer；
- LSP consumer；
- 通用 FS Tool；
- Tool call/result 的事件持久化。

### 4.5 Profile 组合策略

OpenMuse 在启动 DSH Workspace Session 前生成 Profile/Patch：

```yaml
# 说明性伪配置；准确 row id 和 schema 由 0.1.7-rc.1 contract spike 固化
plugins:
  - id: openmuse-workspace-runtime
    name: "@openmuse/dsh-workspace-runtime"
    config:
      leaseRef: "opaque:..."

  - id: subprocess
    name: "@openmuse/dsh-workspace-subprocess"

  - id: fs
    name: "@openmuse/dsh-workspace-fs"

  - id: sandbox
    name: "@openmuse/dsh-workspace-sandbox"

  # 保留 DSH policy、sandbox-aware shell 和模型 Tool
  - id: bash
    name: "@deepseek-ai/dsh-bash-sandbox"

  - id: tool-bash
    name: "@deepseek-ai/dsh-tool-bash"
```

实际应 patch Base Bundle 中现有的 `subprocess`、`fs`、`sandbox` rows，保留 `sandbox-policy`，并避免同时挂载两个互相竞争的 Service Provider。OpenMuse sandbox provider 生成的 wrapper argv 必须在目标 Execution World 内执行；不能让 Host 本地 sandbox runner 去解释远端路径。

### 4.6 不同 Workspace 的统一映射

| Workspace 类型 | Execution World | 数据进入方式 | 推荐隔离 |
| --- | --- | --- | --- |
| Desktop 本地可信目录 | 当前设备 | 直接目录或安全 checkout | 现有本地 sandbox；高信任模式可显式放宽 |
| Desktop 本地不可信项目 | 本机 container/VM | bind/复制到隔离卷 | rootless container 或 VM |
| Cloud Workspace | Cloud runtime | Revision hydrate 到本地卷 | 每用户/任务 container 或 microVM |
| 用户 BYOS/S3 | Cloud 或用户指定 runtime | Resource broker 读取 Blob 并 hydrate | 同 Cloud；子进程无 S3 Secret |
| 配对 Desktop | 被配对设备 | 在该 Desktop 创建 Lease | 设备在线、同账号、显式授权 |
| E2EE Workspace | 获准解密的设备/runtime | 在能力边界内解密并 hydrate | 明示“谁能看到明文” |

Provider 差异只发生在 Lease 创建和底层 transport。DSH Tool 层保持一致。

### 4.7 Sandbox 文件系统与挂载合同

隔离环境内部建议固定为：

```text
/workspace/                   # 当前 Draft checkout；按 Lease 为 RO 或 RW
  project/
    report.docx
    data.xlsx
    analyze.py
    generate_chart.py

/runtime/                     # OpenMuse 管理，只读
  bin/
    office                    # Office curated dispatcher
    openmuse                  # 通用 capability/CLI dispatcher
  registry/
    cli-registry.json         # Host 签名/校验后生成的冻结快照
  image.json                  # toolchain/image digest

/home/agent/                  # 本 Lease 私有、临时；不映射真实用户 HOME
/tmp/                         # 本 Lease 私有 tmpfs，有容量和执行策略
/run/openmuse/
  cli.sock                    # 短期、scoped 的 CLI invocation endpoint
  session.json                # 只读、无 Secret 的 Session descriptor
```

关键规则：

- 不挂载真实 `~/.ssh`、`~/.aws`、浏览器 Profile、用户 Documents 或 Host Plugin 数据目录；
- `/workspace` 是用户数据视图，`/runtime` 是可信控制数据，二者不可互换；
- Workspace 内的 `.openmuse/plugins.json` 最多是项目建议或锁定请求，不能成为可执行代码装载权威；
- Agent 的 `PATH` 只加入 `/runtime/bin` 和经过批准的基础工具链，不加入整个 `/plugins/*/bin`；
- Plugin worker bundle 最好不直接暴露在 Agent mount namespace，由 Supervisor 根据 Registry 启动；
- `/tmp`、`/home/agent` 和 Plugin 私有 state 必须按 Lease/Plugin 隔离，不能污染 Workspace Revision。

### 4.8 Session 装配流程

```text
DSH Plugin requests sandbox session
                │
                ▼
Host Broker authenticates actor + DSH plugin
                │
                ▼
Workspace Authority resolves revision/materialization
                │
                ├──────────────┐
                ▼              ▼
Plugin Authority          Policy Engine
resolves signed CLI       computes ceiling/grants
artifacts for target
                └──────┬───────┘
                       ▼
Sandbox Service allocates runtime + immutable CLI registry
                       │
                       ▼
DSH adapter attaches ctx.fs + ctx.subprocess + session cwd
                       │
                       ▼
Agent executes Bash / office / openmuse
                       │
                       ▼
Supervisor audits, tracks processes, produces Draft/receipt
```

Lease 中的 Plugin 版本必须冻结。用户在 Host 安装、更新或卸载 Plugin 后，已有 Session 不应悄悄更换可执行代码；新版本在下一次 Lease 创建时生效，或经过显式的 Session capability refresh。

### 4.9 Host Service 合同草案

`workspace.sandbox@1` 走现有 Platform Broker 的 Control Plane，只传 descriptor、handle 和 receipt；PTY、文件流和 stdout/stderr 进入 Data Plane。

建议方法：

| Method | 输入摘要 | 输出摘要 | 领域边界 |
| --- | --- | --- | --- |
| `createLease` | workspaceRef、baseRevision、actorRef、execution preference、requested policy、CLI groups | lease descriptor | 调用 Resource/Plugin/Policy Authority，不返回 Host path |
| `attachConsumer` | leaseRef、consumer plugin、audience、protocol versions | provider attachment handle | handle 与 audience/TTL/generation 绑定 |
| `status` | leaseRef | state、active processes、draft dirty、quota | 不返回 Secret 和其他租户信息 |
| `quiesce` | leaseRef、deadline、strategy | process/flush receipt | 阻止新进程并收敛现有 process range |
| `prepareDraft` | leaseRef | draftRef、baseRevision、manifest digest、change summary | 只生成 Draft；不越权发布 Revision |
| `refreshCapabilities` | leaseRef、expected registry digest | 新 registry/拒绝原因 | 必须显式，不能静默替换 worker |
| `release` | leaseRef、disposition | cleanup receipt | disposition 为 preserve-draft / discard / already-checkpointed |

Workspace 发布仍由独立的 `workspace.checkpoint@1` 或 Resource Authority 完成：它接收 `draftRef + expectedBaseRevision` 并返回 new Revision/conflict。Sandbox Service 不应同时成为 Workspace Revision 真源。

建议事件：`sandbox.ready`、`sandbox.degraded`、`sandbox.processChanged`、`sandbox.draftChanged`、`sandbox.leaseExpiring`、`sandbox.released`。所有事件携带单调 sequence/generation，Consumer 断线重连后先 snapshot 再增量追赶。

最小错误分类：`UNSUPPORTED_TARGET`、`POLICY_DENIED`、`SANDBOX_UNAVAILABLE`、`ARTIFACT_UNAVAILABLE`、`LEASE_EXPIRED`、`STALE_GENERATION`、`QUOTA_EXCEEDED`、`NOT_QUIESCENT`、`DRAFT_CONFLICT`、`PROVIDER_FAILED`。不要让 Consumer 依赖 stderr 文本判断 Host 领域错误。

## 5. Workspace 生命周期与一致性

### 5.1 推荐状态机

```text
resolve workspace + permission
              │
              ▼
         allocate lease
              │
              ▼
hydrate baseRevision → local checkout
              │
              ▼
 attach ctx.fs + ctx.subprocess
              │
              ▼
      Agent edits / executes
              │
              ▼
 mutation journal + draft snapshot
              │
              ▼
 quiesce foreground/background processes
              │
              ▼
 checkpoint(expectedBaseRevision)
        │                    │
        ▼                    ▼
 publish blobs + CAS      conflict
        │                    │
        ▼                    ▼
 new immutable revision   preserve draft
              │
              ▼
 kill processes → revoke lease → scrub volume
```

### 5.2 S3 只作为 Blob/Revision 后端

不能把每个 `stat/open/read/readdir` 实时翻译成 S3 API。`rg`、`git status`、`cargo check`、Office 渲染都会产生大量细粒度随机 IO，FUSE/S3 语义差异还会破坏 rename、locking、symlink 和一致性预期。

推荐流程：

1. 从 Workspace Revision 解析 Manifest；
2. 并行下载缺失 Blob 到本地缓存；
3. 用 reflink/hardlink/COW 构造 checkout；
4. Agent 只访问本地文件系统；
5. checkpoint 时计算变化、上传新 Blob；
6. 最后用 `expectedBaseRevision` 做元数据 CAS；
7. 发布失败时保留 Draft，不覆盖并发版本。

### 5.3 Bash 写入绕过逐文件 API，是设计事实

一旦允许 Bash，程序可以通过 mmap、原子 rename、临时文件、生成器和数据库引擎修改任意数量文件。文件系统 watcher 只能作为优化提示，不能作为提交真相。

生产实现至少要选一种可靠机制：

- OverlayFS/COW upperdir 作为变化集；
- 文件系统/卷快照；
- checkpoint 时对 Manifest 做全量或分层 digest 校验；
- journal + digest verification 的组合。

Workspace Checkout 应被视为一个 **Draft Transaction**。只有 `checkpoint` 成功产生的 immutable Revision 才进入 Workspace Authority。

### 5.4 后台进程与提交

`bash` Tool 返回不代表所有写入已经结束。程序可能 fork daemon、启动 watcher 或留下后台 job。

因此：

- Subprocess Provider 必须按 Lease 跟踪完整 process range；
- checkpoint 前进入 quiescing，禁止新进程；
- 等待或终止剩余进程，并 flush 文件系统；
- 未能静默时拒绝自动提交，向 Agent/用户报告活跃 job；
- Lease 到期或崩溃恢复时清理孤儿进程和孤儿卷。

不能把“每次 Bash Tool 结束后自动同步”当作一致性协议。

### 5.5 冲突处理

最低要求是 Workspace Revision 级 Optimistic Concurrency：

```text
checkpoint(draft, expectedBaseRevision)
```

若 Authority 的 HEAD 已变化：

- 不做 last-write-wins；
- 保留当前 Draft 和执行日志；
- 文本/代码可尝试三方 merge；
- DOCX/XLSX/PPTX 等复合格式优先保留双方 Revision，再由格式领域工具合并；
- Agent 必须收到结构化 conflict，而不是模糊的上传失败。

### 5.6 Host UI 与 Agent 并发编辑

“Human UI 与 Agent 看见同一个逻辑 Workspace”不等于“两个进程应直接写同一个物理 working copy”。尤其 DOCX/XLSX/PPTX 可能在编辑器内有尚未 flush 的内存状态，Agent 对磁盘文件原地修改会绕过编辑器事务。

推荐默认模型：

```text
                    Workspace Revision v102
                         ┌──────┴──────┐
                         ▼             ▼
                  Human Edit Draft   Agent Sandbox Draft
                  UI Plugin Session   Bash / Office CLI
                         │             │
                         └──────┬──────┘
                                ▼
                       commit / merge / conflict
                                │
                                ▼
                         Revision v103/v104
```

规则：

- Human 和 Agent 默认各自拥有基于同一 Revision 的 Draft，不共享可写 inode；
- Agent checkpoint 使用 `expectedBaseRevision`，与 UI commit 并发时进入冲突/merge；
- 文本可三方合并，Office 复合格式由相应 Plugin 提供 format-aware merge，不能按 ZIP 字节盲合并；
- UI 应能预览 Agent Draft 的 change summary/render，再接受、拒绝或另存为新 Revision；
- 只有用户显式选择“独占交给 Agent”时，Host 才 flush/关闭 UI writer、授予排他 Lease，并让 Agent 原地修改该 working copy；
- read-only UI 预览可以订阅 Agent Draft，但必须标记未提交状态和来源。

这种模型牺牲了一点“屏幕上立刻看到磁盘变化”的直觉，却保住了 Revision、撤销、冲突和 Office 格式完整性。

## 6. Bash、结构化 Tool 与 Office CLI 的分工

### 6.1 Bash/CLI 负责通用计算

适合：

- `ls`, `rg`, `find`, `sed`, `cp`, `mv`；
- `git`, `cargo`, `python`, `node`, `ffmpeg`, `pandoc`；
- 项目测试、格式化、批量转换和用户脚本；
- Office CLI 的可组合调用。

### 6.2 DSH FS Tool 仍有价值

适合：

- 小范围、结构化、可版本检查的读取和编辑；
- 给模型返回受控大小的内容；
- 更清晰的错误分类；
- 不需要 shell escaping 的确定性操作。

Bash 与 FS Tool 必须共享同一 `ctx.fs`/`ctx.subprocess` Execution World，而不是互斥选择。

### 6.3 Workspace Tool/CLI 负责领域语义

建议提供：

```text
workspace.status
workspace.checkpoint
workspace.history
workspace.restore
workspace.share
workspace.sync
```

等价 CLI 可为：

```bash
openmuse-workspace status --json
openmuse-workspace checkpoint --message "Migrate API"
openmuse-workspace history --json
```

这些操作必须经过 Workspace Authority，而不是靠 `cp` 或 S3 命令模拟。

### 6.4 Office CLI 作为 Execution Image 能力

Office CLI 应读写 `/workspace` 中的普通文件，并通过受控 Host/Domain Service 完成格式能力，例如：

```bash
office docx inspect docs/report.docx --json
office sheet recalc sheets/budget.xlsx
office slide render slides/demo.pptx --out /tmp/render
```

Office CLI 不应直接访问 S3，也不应自行发布 Workspace Revision。这样本地、Cloud、BYOS 的行为一致。

### 6.5 Plugin 应有三个 Surface，但共享的是 Core，不是进程权限

```text
                           Signed Plugin Bundle
                                    │
                     ┌──────────────┼──────────────┐
                     │              │              │
                 UI Surface    Service Surface   Agent CLI Surface
                     │              │              │
                  Human UI       Host Broker    Sandbox Supervisor
                     │              │              │
                     └──────────────┼──────────────┘
                                    ▼
                           Shared Rust Domain Core
```

例如 Sheet Plugin 可以同时提供：

- Flutter/Native UI：打开 Sheet、编辑 Cell、图表和 Selection；
- Host Service：`sheet.readRange@1`、`sheet.recalculate@1`；
- Agent CLI：`office sheet read/set/recalc`。

三者可以复用相同 Rust crate、格式解析器和测试向量，但必须有独立 adapter。UI 进程可能拥有剪贴板和窗口能力，Sandbox worker 不应继承这些权限；Agent worker 崩溃也不能带走 UI Session。

### 6.6 Manifest vNext 的 Agent CLI Contribution

当前 Manifest v1 不能容纳该能力。建议以向后兼容 minor version 或 v2 正式增加 artifact 与 `agent_cli`，示意如下：

```json
{
  "id": "com.openmuse.office-docx",
  "version": "1.2.0",
  "artifacts": [
    {
      "id": "docx-worker-linux-x64",
      "kind": "sandbox-worker",
      "target": { "os": "linux", "arch": "x64" },
      "path": "workers/linux-x64/office-docx",
      "sha256": "sha256:..."
    }
  ],
  "contributes": {
    "agent_cli": [
      {
        "group": "office",
        "namespace": "docx",
        "abi": 1,
        "artifact": "docx-worker-linux-x64",
        "commands": [
          {
            "name": "render",
            "args_schema": "schemas/docx-render.v1.json",
            "required_permissions": [
              "workspace.read",
              "workspace.write"
            ],
            "network": "none"
          }
        ]
      }
    ]
  }
}
```

生产 schema 还应定义：

- artifact 的 OS/arch/libc、入口点、digest、签名链与最小 Runtime ABI；
- namespace/command 的唯一性和冲突选择规则；
- 参数与输出 schema、稳定 exit code、超时、最大输出和是否支持 streaming；
- 声明的文件效果、网络域、子进程、GPU、字体和临时空间需求；
- 是否 deterministic、是否可缓存、是否修改原文件、是否产生新资源；
- headless worker 的健康检查和协议版本。

Manifest 只是**请求能力和声明贡献**，不是授权。安装成功也不意味着该 CLI 自动进入所有 Agent Session。

### 6.7 Dispatcher 与 Worker 启动

不推荐把每个 Plugin binary 注入 `PATH`。推荐只暴露两个稳定入口：

- `office`：文档、表格、幻灯片、PDF 等官方 Office group；
- `openmuse`：通用第三方能力和发现入口，例如 `openmuse cad ...`、`openmuse media ...`。

调用链：

```text
Agent: office docx render report.docx -o preview.pdf
                         │
                         ▼
               /runtime/bin/office
                         │ argv + cwd + request id
                         ▼
               scoped cli.sock / Supervisor
                         │
           registry lookup + permission intersection
                         │
                         ▼
             nested Plugin invocation sandbox
              ├── same /workspace Draft
              ├── plugin bundle RO
              ├── private /tmp + state
              ├── command-specific egress
              └── deadline/quota/audit
                         │
                         ▼
                 DOCX headless worker
```

这条 brokered invocation 路径比直接执行 binary 多一次 IPC，但可以阻止 Agent 绕过 command registry、参数约束、逐命令权限和审计。V1 若为了 PoC 直接执行 worker，必须明确它只能获得 Session baseline 权限，不能获得网络、Secret 或额外 Host capability。

CLI ABI 最低约束：

- 参数通过 argv 或结构化 request 传递，不拼接第二层 shell command；
- `--json` 的 stdout 必须符合版本化 schema，日志走 stderr；
- 大结果通过 Workspace 输出文件或 Blob receipt 返回，不内嵌无限 stdout；
- cancellation、deadline 和 signal 必须传播到整个 worker process range；
- 统一区分 usage、permission、conflict、unsupported、transient 和 engine failure；
- 输出和错误均视为不可信内容，进入 Agent 上下文前限长和标注来源。

### 6.8 能力发现与平台兼容

Agent 可通过以下稳定入口按需发现能力，而不是把所有 Plugin 帮助文本塞进 system prompt：

```bash
office plugins --json
office docx commands --json
openmuse capabilities --json
```

Registry 只列出当前 Lease 中同时满足以下条件的贡献：

1. Plugin 已安装、签名和 digest 验证通过；
2. Plugin/CLI ABI 与 Sandbox Runtime 兼容；
3. 当前 execution target 有匹配 artifact；
4. 用户/组织允许该 Plugin 进入 Agent Sandbox；
5. Workspace classification 与命令权限允许；
6. 当前 Session policy 未禁用该能力。

因此，“用户在 macOS 安装了一个带 macOS UI 的 Plugin”不等于“Cloud Linux Sandbox 可以使用它”。Bundle 或 Plugin Catalog 必须另外提供经过验证的 Linux worker artifact。Rust 代码有利于多平台构建，但不能消除原生依赖、字体、libc、Office engine 和架构矩阵。

### 6.9 Workspace 脚本、Skill 与已安装 Plugin 必须区分

Workspace 中的 `analyze.py`、`generate_chart.py`、Skill 文档和项目脚本都是用户数据：Agent 可以在 baseline Sandbox policy 下读取或执行，但它们不能自动注册 Host Service、获得 Secret 或提升权限。

只有经过 Plugin Authority 安装和验证的 Bundle 才能贡献 `agent_cli`。Workspace 可以提交一个 Plugin requirement/lockfile，请求 Host 安装或选择版本；最终仍需 Catalog 校验和用户/组织授权。这样可以避免打开一个恶意 Workspace 就自动装载并执行其中的“插件”。

## 7. 多 Workspace 与 DSH Realm 边界

这是决定是否需要修改 DSH 契约的关键点。

当前 `SubprocessSpawnSpec` 主要携带 `argv`、`cwd`、`env`、stdio、timeout 等，没有显式 `executionWorldRef`。`ctx.fs`/`ctx.subprocess` 在一个 Cordis Realm 中也是单一能力实例。

### 7.1 推荐：一个活跃执行 Lease 对应一个 DSH Runtime/Realm

优点：

- `/workspace` 永远只有一个含义；
- 不需要向所有 FS/Subprocess 调用传播 world id；
- Session cwd、Bash、PTY、LSP 天然一致；
- 租户隔离和生命周期更容易证明；
- DSH 核心零改动。

代价是每个执行 Workspace 有独立 DSH Runtime 或至少独立 Cordis isolate，需要 Host 做生命周期和资源池管理。

### 7.2 可行：一个 Provider 用唯一绝对根路由多个 Workspace

例如：

```text
/workspaces/<leaseRef-A>/...
/workspaces/<leaseRef-B>/...
```

Provider 可通过 `cwd/path` 路由。但模型会看到非统一路径，路径泄漏、跨根校验和 LSP 配置更复杂。只适合可信本地多目录或受控内部实现。

### 7.3 不推荐：在同一 Realm 中让多个 Session 都使用 `/workspace`

此时单凭 `cwd=/workspace` 无法判断目标 Execution World。要正确支持，必须把 `worldRef/sessionRef` 加入 FS 和 Subprocess 契约，并修改：

- Service types；
- 所有 Provider；
- Bash/PTY/LSP/FS consumers；
- 测试与兼容层。

这从“小量侵入”升级为横跨生态的协议变更。除非经过容量测试证明独立 Realm 无法接受，否则不应进入 V1。

## 8. Sandbox 与安全模型

### 8.1 当前 DSH Sandbox 的边界

DSH 当前 Sandbox 主要约束同一执行世界中的文件影响，通过包装 argv 和 FS policy 工作。它不是完整的：

- 网络 egress 防火墙；
- syscall/seccomp 沙箱；
- CPU、内存、磁盘、PID 配额；
- 容器/VM 租户隔离；
- Secret broker；
- 供应链策略。

另外，当前 sandbox policy 对 Session cwd 有 Host 本地 `realpath` 假设，部分 unavailable diagnostic 也指向本地 OS 后端。远端 Provider 不能未经验证就宣称获得 `workspace-write` 等同保障。

### 8.2 外层 Sandbox Ceiling 与 DSH 内层 Policy

需要明确两个不同概念：

| 层 | 权威 | 作用 | 能否由 Agent 提升 |
| --- | --- | --- | --- |
| OpenMuse Sandbox Ceiling | Host Policy + Sandbox Service | mount、network、device、Secret、quota、tenant/process boundary | 不能；创建 Lease 时冻结 |
| DSH per-call policy | `ctx.sandboxPolicy` | `read-only / workspace-write / danger-full-access` 文件效果与一次性 escalation | 只能在外层 ceiling 内提升 |

DSH 源码明确将 `ctx.sandbox` 定义为 **same-world file confinement**：Linux 选择 bubblewrap/Landlock，macOS 使用 Seatbelt，Windows 使用 ACL/restricted token；无法执行请求模式时 fail closed。它不负责网络、容器、多租户、Secret 和资源配额，详见 [DSH sandbox contract](../../vendors/deepseek-harness/packages/sandbox/sandbox/README.md)。

因此，在 Cloud container/microVM 中，即使 DSH 某次调用显示 `danger-full-access`，其含义也只能是“跳过 DSH 内层文件限制”，绝不能跳出 OpenMuse 外层 Sandbox、访问宿主机或其他租户。产品 UI 和审计必须说明这个相对边界，避免把 DSH vocabulary 误解为 Host 全权限。

组合策略：

- **local-native**：可复用 DSH `sandbox-local + bash-sandbox + fs-sandbox`，但仍由 Host 控制 Workspace materialization 和 Plugin grants；
- **local-isolated/cloud**：container/microVM 是外层边界；远端 runtime 内再执行 DSH policy 对应的 wrapper，或由 OpenMuse shell/fs Provider 实现等价 per-call policy；
- 当前 `fs-sandbox` 继承 `LocalFileSystem`，不能直接包裹任意远端 FS Provider，因此远端 `@openmuse/dsh-workspace-fs` 必须自行实现并通过同一 policy TCK；
- 未证明完整 enforcement 时必须报告 `partial` 或拒绝执行，不能伪装成 `workspace-write/full`。

### 8.3 推荐 OS/Runtime 分层

Cloud 运行时至少需要：

- 每租户/任务 rootless container 或 microVM；
- 只读基础镜像，Workspace 独占可写卷，临时目录独立；
- 非 root 用户、capabilities drop、seccomp；
- CPU/内存/PID/磁盘/运行时长配额；
- 默认拒绝或代理化网络 egress；
- 禁止挂载 Host home、Docker socket、设备和控制面 socket；
- Runtime API 与数据面分离；
- 镜像版本、SBOM、漏洞扫描和签名。

本地模式分两级：

- `local-native`：启动快、兼容高，但应标记为高信任；
- `local-isolated`：container/VM，适合下载内容和不可信项目。

### 8.4 权限求交与防止 Confused Deputy

一次 Plugin CLI 调用的有效权限必须是交集，而不是任一 Manifest 的并集：

```text
effective permission
  = Host/Tenant policy ceiling
  ∩ user grants for Agent
  ∩ DSH plugin grants
  ∩ target Plugin grants
  ∩ CLI command required permissions
  ∩ Workspace classification policy
  ∩ current per-call approved escalation
```

建议的标准权限词汇至少包括：

- `workspace.read`、`workspace.write`、`workspace.commit`；
- `temp.write`、`plugin.state.read/write`；
- `process.spawn`、`process.background`；
- `network.egress` + Host 管理的 allowlist；
- `secret.use:<service>`，只允许 brokered use，不导出明文；
- `gpu.use`、`clipboard.read/write`、`device.*`。

当前 Platform Broker 只验证 caller grants 是否覆盖 contribution 的 required permissions，尚不足以表示“Agent 代表用户调用目标 Plugin worker”的双主体关系。Sandbox Service 必须记录并校验：

- original actor（用户/Agent Session）；
- caller plugin（例如 DSH Agent）；
- target plugin/version/command；
- Workspace/Revision/Lease；
- effective grants 与 policy decision id。

目标 Plugin 不能利用自己在 Host UI 场景获得的权限充当 confused deputy；DSH 的一次性 escalation 也不能突破 Host ceiling。

### 8.5 凭据

子进程环境中不能出现：

- S3 access key；
- OpenMuse 主 Session Token；
- Model Provider Secret；
- Cloud control-plane 凭据；
- 其他 Workspace 的 capability。

DSH subprocess 已有对 credential-shaped env 的清理意识，但不能只依赖字符串过滤。推荐让 Workspace Sandbox Runtime 通过出站 Resource Broker 使用短期、最小权限 capability，且控制通道对子进程不可见。

### 8.6 主要威胁与缓解

| 威胁 | 后果 | 必要缓解 |
| --- | --- | --- |
| 任意代码执行 | RCE 是功能本身 | VM/container 边界、最小挂载、非 root |
| 网络外传 | Workspace/Secret 泄露 | 默认拒绝、allowlist/proxy、域名/IP 审计 |
| 恶意依赖/install script | 供应链攻击 | 锁定镜像、隔离 package cache、策略与审批 |
| symlink/hardlink/path traversal | 逃离 Workspace | Provider 级 canonicalization、openat 风格校验 |
| archive bomb/大输出 | 资源耗尽 | 配额、流式限制、压缩比限制、输出截断 |
| fork bomb/后台 daemon | 失控和提交竞态 | cgroup/PID 配额、process range、quiescence |
| 输出 Prompt Injection | Agent 被工具输出操纵 | 标记不可信输出、截断、结构化解析、策略层 |
| 跨租户缓存污染 | 数据泄露 | 内容校验、租户/加密域隔离、不可写共享缓存 |
| Runtime 看到明文 | 数据自主承诺不清 | 明示执行位置和解密主体，支持本地执行 |
| 恶意/被替换 Plugin worker | 供应链与持久化 | 签名、digest pin、SBOM、只读 bundle、版本冻结 |
| 直接调用 worker 绕过 dispatcher | 绕过权限/审计 | worker 不进入 Agent namespace；只暴露 scoped dispatcher socket |
| Plugin confused deputy | 借用 UI/Host 高权限 | actor + caller + target 三方授权和命令级权限求交 |
| Workspace 伪造插件清单 | 打开项目即执行代码 | Workspace 清单非权威；只信 Plugin Authority registry |

## 9. 可观测性与审计

DSH 已能通过 Tool call/result 事件记录模型执行的命令和结果。但仅记录命令不等于记录文件效果；程序可能间接写入大量文件。

建议每个 Lease 产生：

- Runtime image digest；
- actor、caller plugin、target plugin/version/command；
- workspaceRef、baseRevision、policy ceiling、decision id；
- CLI registry digest、worker artifact digest 与 target platform；
- exec 请求、exit code、signal、duration、resource usage；
- 网络策略命中与拒绝；
- Draft manifest digest 和变化摘要；
- checkpoint request/result、newRevision 或 conflict；
- Lease revoke/cleanup receipt。

大 stdout/stderr 不应无限进入会话日志；采用截断 + Blob 引用，并对 Secret 做确定性脱敏。

## 10. 对 DSH 的侵入级别分析

| 场景 | 做法 | DSH 核心改动 | 可信度 |
| --- | --- | --- | --- |
| 本地 PoC | 现有 Bash + 本地 FS/Subprocess，Session cwd 指向 checkout | 0 | 很高 |
| Cloud PoC | 外部 Runtime/FS/Subprocess Provider + Profile patch | 0 | 很高，E2B 已验证模式 |
| Cloud Production，单 Lease/Realm | 强化 Provider、runtime、安全和 checkpoint | 0 | 高 |
| 远端准确 sandbox 语义 | 自定义 Provider/执行镜像约束 | 0；可选改进上游诊断 | 中高 |
| provider-neutral sandbox cwd/diagnostic | 修正本地 OS 假设 | 小量，约 1～3 个文件 | 高 |
| 同 Realm 多个同名 `/workspace` | 为所有调用传播 worldRef | 中到高，跨契约改动 | 不推荐 |
| 在 Agent Loop 中特判 OpenMuse | fork/patch loop | 高且持续漂移 | 拒绝 |

“零侵入”成立的前提是遵守 DSH 当前能力模型：一个 Provider Realm 表示一个一致的 Execution World。若产品强制单进程内以同一路径复用多个远端世界，就会主动破坏这个前提。

这个结论只针对 DSH。OpenMuse Host 侧需要有计划地升级 Manifest/Protocol、Platform Broker 和外部 Runtime Supervisor；这属于完善自有 Plugin ABI，不是对 DSH 的侵入。

## 11. 收益

### 11.1 Agent 能力复用

现有 `git/rg/python/node/cargo/ffmpeg/pandoc` 和未来 Office CLI 可直接工作，不必逐一包装成 Tool。

### 11.2 Provider 可替换

Local、Cloud、配对 Desktop、BYOS 的差异被压到 Runtime/Provider，Agent prompt、Tool 和工作流保持稳定。

### 11.3 与 DSH 架构一致

复用公开 Service seam 和 Cordis composition，降低维护 fork 与跟随上游升级的成本。

### 11.4 存储透明与数据自主

Agent 不持有 S3 Secret，不依赖某家对象存储。用户可以选择数据落点，并选择明文执行发生在本机、官方 Cloud 或授权私有环境。

### 11.5 性能与生态

本地 checkout 为代码扫描、构建、Office 处理提供真实文件系统性能，也允许利用标准缓存、增量构建和内容寻址去重。

## 12. 风险与限制

| 风险 | 严重度 | 说明 / 应对 |
| --- | --- | --- |
| Runtime 隔离不足 | 极高 | 在 Cloud GA 前必须完成多租户渗透测试和逃逸测试 |
| Workspace 提交丢失/覆盖 | 极高 | immutable Revision + expected-base CAS + Draft 保留 |
| FS 与 subprocess 指向不同世界 | 极高 | Provider TCK 强制双向可见性测试 |
| 后台进程在 checkpoint 后继续写 | 高 | process range + quiescence + volume freeze |
| 冷启动与 hydrate 延迟 | 高 | 镜像预热、Blob cache、reflink、按需预取 |
| 大 Workspace 成本 | 高 | 分层 Manifest、稀疏 checkout、缓存预算、GC |
| Windows Bash 不一致 | 高 | V1 用 WSL2/container；不要用 PowerShell 冒充 Bash |
| 本地原生模式边界弱 | 高 | 明示高信任，默认推荐 isolated mode |
| DSH RC 版本漂移 | 中高 | contract tests、锁版本、升级矩阵 |
| S3/Provider 语义差异 | 中 | Blob/Manifest 层做 capability matrix，不暴露给执行面 |
| Office 二进制冲突合并困难 | 中高 | 保留双方 Revision，交给格式领域合并 |
| 日志包含敏感内容 | 高 | 输出限额、脱敏、分级保留和用户删除能力 |
| Plugin CLI 供应链 | 极高 | 只运行签名且 digest pin 的 target artifact，保留 SBOM/撤销列表 |
| Host Plugin 与 Sandbox worker 权限串用 | 极高 | 独立进程/身份/令牌；共享 Core 不共享 ambient authority |
| CLI namespace/版本冲突 | 中 | Host 确定性选择、显式 group/namespace/ABI、Lease registry 冻结 |
| Cloud 缺少匹配 worker artifact | 中 | capability discovery 只公布当前 target 可用项，禁止透明 fallback 到 Host |
| Dispatcher 成为高价值入口 | 高 | 最小协议、参数 schema、速率限制、fuzz、无 shell 拼接 |

## 13. 不推荐方案

### 13.1 只提供 workspace.read/write/list Tool

会重新实现一个能力更弱的 shell，无法自然复用构建器、语言工具链、Office CLI 和用户脚本。结构化 Tool 应补充 Bash，而不是替代 Bash。

### 13.2 把 S3 直接 FUSE 到 `/workspace`

性能、POSIX 语义、离线、rename/locking 和供应商差异都不可控。FUSE 可以作为特定场景实验，不应成为 Workspace 一致性基础。

### 13.3 新写一个 `workspace_bash` Tool，把命令 RPC 到远端

短期能跑命令，但 FS Tool、PTY、LSP 仍可能留在 Host，产生多个执行世界；还会重复实现 DSH jobs、日志和 Tool 语义。

### 13.4 每个 Tool 自己选择 Provider

会让权限、路径、缓存、错误模型和审计分叉。Provider 应服务于 Execution World，而不是某个 Tool。

### 13.5 Fork DSH Agent Loop

本需求不需要修改推理循环。Fork 会造成升级冲突、行为漂移和测试面扩大，且违背 Everything is Plugin。

### 13.6 把 S3 凭据注入 Agent 环境

这会让任意程序拥有越权和外传能力，也把存储厂商细节泄露给 Agent。应由 Resource Broker/Materializer 持有最小权限。

### 13.7 把所有 Plugin `bin/` 注入 PATH

这会让 Agent 绕过 command registry、参数 schema、命令级授权和审计，还会造成 namespace 劫持。生产设计只暴露稳定 dispatcher，由 Supervisor 启动不可直接寻址的 worker。

### 13.8 复用正在运行的 Host/UI Plugin 进程

UI 进程通常拥有窗口、剪贴板、用户配置或其他 ambient authority。让 Agent 直接驱动它会产生 confused deputy 和崩溃耦合。正确复用单位是 Rust Domain Core、格式测试和 Bundle 版本，不是进程和权限上下文。

### 13.9 允许 Workspace 自行注册 Plugin

Workspace 脚本可以在 baseline policy 下执行，但不能因为存在 `plugins.json` 就获得安装态 Plugin 权限。否则“打开项目”会退化成“安装并授权不可信代码”。

## 14. 实施计划与工作量

下表为“熟悉 Rust/TypeScript/容器和当前 Host 架构的工程师”的净工程量估算，不含 Office 格式引擎本身。部分阶段可并行并共享基础设施，不能简单把每一行上限机械相加。

| 阶段 | 交付 | 人周 |
| --- | --- | ---: |
| 0. Contract Spike | 锁定 DSH `0.1.7-rc.1`、验证 Profile/远端 policy、Host ABI gap | 2～3 |
| 1. Local Sandbox PoC | checkout + Bash/FS/PTY 双向可见 + 单个静态 Office worker | 3～5 |
| 2. Host Sandbox Service | `workspace.sandbox@1`、Lease、Broker adapter、生命周期 | 4～6 |
| 3. Cloud Runtime | container/microVM、FS/Subprocess Provider、runtime pool | 5～8 |
| 4. Revision Pipeline | hydrate、cache、journal/COW、Draft、checkpoint CAS、冲突 | 4～6 |
| 5. Plugin CLI ABI | Manifest/schema/Rust contract、artifact selector、registry/TCK | 3～5 |
| 6. Dispatcher/Supervisor | `office/openmuse`、scoped socket、nested worker、输出协议 | 4～7 |
| 7. Security | egress、quota、credential broker、供应链、三方授权、审计 | 5～8 |
| 8. Consumer 完整性 | PTY、LSP、后台 job、quiescence、崩溃恢复 | 3～5 |
| 9. Desktop | macOS/Linux local-native 与 isolated adapter | 2～4 |
| 10. Production hardening | 指标、故障注入、长稳、容量、升级兼容 | 3～5 |
| Workspace CLI | status/checkpoint/history/restore/sync | 2～4 |
| 首个 Office Plugin CLI adapter | 从共享 Rust Core 构建 worker、schema、打包与 TCK | 3～6 |
| Cross-target artifact pipeline | Linux/macOS/Windows 构建、签名、SBOM、Catalog | 3～5 |
| Windows 同等 Bash | WSL2/container、路径/权限/PTY 兼容 | 额外 4～7 |
| 后续 Office CLI 领域 | 文档/表格/幻灯片各自的命令设计和 engine 接入 | 额外 3～8+ / 领域 |

里程碑建议：

1. **P0：4～6 人周**：本地与单个 Cloud Linux Sandbox 跑通 `ctx.fs ↔ Bash ↔ PTY`，并从静态 Registry 调用一个 DOCX worker。
2. **P1：14～22 人周**：Linux Cloud MVP，包含单 Lease/Realm、持久 Workspace、显式 checkpoint、Sandbox Service、基础 CLI Registry/Dispatcher、隔离和审计。
3. **P2：32～48 人周累计**：Local + Cloud 生产核心，包含动态 Plugin CLI、三方授权、供应链、冲突、恢复、LSP、后台进程和完整安全能力。
4. **P3：按需扩展**：配对 Desktop、Windows Bash、企业私有 Runtime、Office CLI 与 E2EE placement。

3 名工程师并行时，P2 的合理日历周期约 14～22 周；安全评审、原生 Plugin cross-build、基础设施采购和外部渗透测试可能增加关键路径。

## 15. 验收标准与 Provider TCK

### 15.1 功能一致性

- FS Tool 写文件后，Bash 立即读取到相同字节；
- Bash 原子 rename/写入后，FS Tool 和 LSP 立即看到结果；
- PTY 与普通 subprocess 使用同一 cwd、env policy 和文件系统；
- 前台/后台进程的信号、超时、取消、exit code 语义一致；
- Local 与 Cloud 对相同测试工作负载产生相同 Workspace Revision。

### 15.2 提交正确性

- 并发 HEAD 变化必然产生 conflict，不静默覆盖；
- 进程未 quiesce 时 checkpoint 被拒绝或明确等待；
- 网络中断/Host 崩溃后 Draft 可恢复或被可证明清理；
- Blob 上传成功但元数据 CAS 失败时不会发布半成品 Revision；
- watcher 丢事件时 digest verification 仍能发现变化。

### 15.3 安全

- 无法读取 Workspace/临时区以外的 Host 文件；
- 无法访问其他 Lease 的进程、卷、cache plaintext；
- 子进程环境和 `/proc` 中没有 S3/OpenMuse/Model 凭据；
- egress deny/allowlist 可测试、可审计；
- fork bomb、磁盘填满、无限输出和超时能被可靠终止；
- symlink、hardlink、archive traversal 和 Unicode path 有专项测试。

### 15.4 Plugin CLI 生态

- 未签名、digest 不符、目标平台不匹配的 worker 不出现在 Registry；
- 同 namespace/version 冲突按确定性规则拒绝或选择，不依赖安装顺序；
- Plugin 更新后，已有 Lease 的 Registry 与 worker digest 不发生静默变化；
- Agent 无法直接执行或替换 Plugin worker bundle；
- dispatcher 对 argv、cwd、deadline、cancel、stdout/stderr、exit taxonomy 有协议测试；
- baseline Bash 无网络时，声明 network 的 Plugin command 也不能越过 Host ceiling；
- DSH `danger-full-access` 无法突破 outer Sandbox；
- UI Plugin、Host Service 和 Agent worker 使用不同 principal/credentials；
- Workspace 内伪造 `.openmuse/plugins.json` 不会注册命令或获得权限；
- macOS UI-only Plugin 在 Cloud Linux 无 worker 时明确显示 unavailable，不做 Host fallback。

### 15.5 性能基线

至少用以下 workload 做 Local/Cloud 对比：

- 10 万小文件的 `rg/find/git status`；
- 1～10 GB Office/媒体资产的增量 checkpoint；
- Rust/Node 项目的冷/热构建；
- 多轮 Agent 编辑 + 测试 + checkpoint；
- cache miss、cache hit、跨区域 BYOS 和断线恢复。

## 16. 最终建议

批准该方向，并把需求名称和边界升级为 **OpenMuse Workspace Sandbox**。DSH Execution Plane 是它的一个适配面，不是 Sandbox 本身。

推荐决策如下：

1. 使用 DSH 现有 `tool-bash → ctx.shell → ctx.subprocess` 链路；
2. 新建 Host 级 `workspace.sandbox@1` Service Plugin，使 Sandbox 能被 DSH 之外的 Agent/自动化复用；
3. 以 DSH 外部 Plugin 实现成对的 OpenMuse `ctx.fs` 与 `ctx.subprocess` Provider；
4. V1 采用“一 Workspace Sandbox Lease 对应一 DSH Runtime/Realm”，保持 `/workspace` 的唯一含义；
5. 把 DSH per-call policy 作为内层文件策略，把 container/microVM/Host policy 作为不可突破的外层 ceiling；
6. 所有后端先物化为本地 checkout，S3 只承载 Blob/Revision，不进入 syscall 热路径；
7. 把 checkout 定义为 Draft Transaction，用 quiescence + digest/COW + Revision CAS 提交；
8. Human UI 与 Agent 默认使用独立 Draft，禁止两个 writer 直接共享 Office 文件 inode；
9. 正式扩展 OpenMuse Manifest，增加签名的 `sandbox-worker` artifact 和 `agent_cli` contribution；
10. `PATH` 只暴露 `office/openmuse` dispatcher，Plugin worker 由 Supervisor 按冻结 Registry 和权限交集启动；
11. Plugin 的 UI/Service/CLI 可以共享 Rust Core，但必须分进程、分身份、分 ambient authority；
12. Workspace Authority 继续拥有版本、权限、分享和冲突真相，Plugin Authority 拥有安装/签名/artifact 真相；
13. Cloud 默认使用 container/microVM，子进程永不持有存储、模型或控制面凭据；
14. 先针对实际锁定的 DSH `0.1.7-rc.1` 和当前 Manifest v1 完成双合同 Spike；
15. 除非单 Realm 多世界成为硬性约束，否则不向 DSH 核心增加 `worldRef`；E2B 只作为测试模式和 TCK 参考。

按此方案，DSH 保持通用 Agent Host，OpenMuse Host 保持 Workspace/Plugin/Policy Authority，Workspace Sandbox Runtime 负责受控执行，Office Plugin 则以 headless worker 扩展 Agent 能力。Local、Cloud 和未来 Workspace 可以共享同一产品语义，又不会把存储、执行、插件信任和 Agent 推理重新耦合在一起。
