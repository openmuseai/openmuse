# DSH 0.1.7-rc.1 Provider Contract Spike 与 TCK

> 状态：Accepted  
> 子需求：X0  
> 日期：2026-09-29

## 1. 验证对象

X0 以 OpenMuse 实际打包闭包 `target/dsh-closure` 为准，而不是 workspace 中的 `../vendors/deepseek-harness` checkout。

- 产品 pin：`@deepseek-ai/dsh@0.1.7-rc.1`；
- `third_party/dsh/package-lock.json` SHA-256：`c48cdc3d9f6d58952332dbef497d9fcc6a455797d6be47195644accc6154826b`；
- 闭包内 `dsh-base/cordis.patch.yml` SHA-256：`56fcd70e699c817fed346040525e901fb6b78b19371f45aece8f57fcd0b697e3`；
- workspace vendor 当时是另一开发版本，且有未提交用户改动，未被修改也未用作版本真源。

冻结合同位于 [`../contracts/dsh/0.1.7-rc.1.contract.json`](../contracts/dsh/0.1.7-rc.1.contract.json)。DSH 升级会先因 exact version 或 package/row 差异失败，必须显式审查并更新合同。

## 2. 可替换的 Everything-is-Plugin 切面

实际闭包确认以下 Cordis Service seam：

| Context service | Service definition | 当前 Provider | 归属 |
| --- | --- | --- | --- |
| `ctx.fs` | `dsh-fs` | `dsh-fs-sandbox` | 目标解析、内容与原子 mutation |
| `ctx.subprocess` | `dsh-subprocess` | `dsh-subprocess-local` | argv、pipe、PTY、process range |
| `ctx.shell` | `dsh-shell` | `dsh-bash-sandbox` | shell 默认值、deadline、前后台 projection |
| `ctx.sandbox` | `dsh-sandbox` | `dsh-sandbox-local` | 同机进程 confinement argv |
| `ctx.sandboxPolicy` | `dsh-sandbox-policy` | 同名实现 | 每次调用 mode/workspace root |
| `ctx.jobs` | `dsh-jobs` | `dsh-jobs-local` | job id、owner、ring、cancel/wait |

因此 Host 增加执行面不需要修改 Agent Loop，也不应把 Bash 直接塞进 Agent。正确扩展点是由 Profile/Bundle patch 用相同 row id 重写完整 Provider row。X0 的 local fixture 见 [`../contracts/dsh/providers/local/cordis.patch.yml`](../contracts/dsh/providers/local/cordis.patch.yml)：它只重写 provider rows，不出现 `agent-loop`。

模型看到的 `bash`、FS tools、jobs 工具保持不变；Local、Cloud、Paired Desktop 的差异下沉到同一个 Execution World 的 Provider 组。

## 3. Execution World 不变量

一个可接入的 Provider 组必须同时满足：

1. `ctx.fs.processPath(target)` 返回同一 execution world 内 `ctx.subprocess/ctx.shell` 能打开的路径；
2. FS 写入后 Bash 立即可见，Bash 写入后 FS 立即可见；
3. subprocess 管理完整 process range，取消、timeout、service dispose 最终到 quiescence；
4. PTY 的分配、输入、输出、resize、foreground signal 和 terminate 属于 subprocess provider；
5. jobs 只管理 ownership/lifecycle/output ring，不拥有进程；
6. LSP 使用 subprocess 的 raw pipe 与 framing，但 LSP adapter 是独立 capability；
7. 同一 Context 的 Service implementation 唯一，重复注册必须失败。

## 4. Local sandbox 与 Remote provider 的关键差异

闭包中的 `dsh-fs-sandbox` 在代码上 `extends LocalFileSystem`。它是可信进程中的路径 policy fence，适用于本机 Provider；它不是可包在任意远端 FS 外面的通用 decorator。

Cloud/remote 实现必须直接实现 `dsh-fs` seam，并在服务端以 opaque target identity、server-side canonicalization、lease/grant 和 mount namespace 执行 containment。远端 `processPath` 是 sandbox 内路径，不得返回 Host 本机 path。相应的 remote subprocess/shell 必须落在同一个 sandbox/lease，不能把 local FS 与 remote Bash 混装。

同理，`dsh-sandbox-local` 是 same-host argv wrapper，不是 Cloud sandbox。Cloud Provider 组应由 Workspace Sandbox substrate 自己提供隔离，并通过 remote shell/subprocess 返回 enforcement facts。

## 5. LSP 结论

`0.1.7-rc.1` 当前产品闭包未安装 `@deepseek-ai/dsh-lsp-stdio`。X0 TCK 已验证 `ctx.subprocess` 的双向 pipe 能完成标准 `Content-Length` JSON-RPC round trip，因此 transport 基础成立；但 capability snapshot 明确记录：

```json
{
  "lspTransport": true,
  "lspAdapterInProductClosure": false
}
```

在后续 distribution lock 纳入签名/digest 合格的 LSP artifact 并运行其专属 TCK 前，产品不得宣称 LSP 可用。这个显式 unavailable 结果比从 vendor source 推断“理论支持”更可信。

## 6. TCK 覆盖

统一入口：

```bash
./scripts/test_dsh_provider_contract.sh
```

若本地没有闭包，脚本会从已 pin 的 manifest/lockfile 构建临时闭包；测试结束清理。

自动验收直接加载闭包内发布 JS，覆盖：

- exact DSH/package version、Base row 与 local overlay replacement；
- 生产闭包不含 E2B 包；
- FS→Bash、Bash→FS 双向可见和 process path 一致；
- abort、background kill 与 process settlement；
- 真 PTY allocation/input/output/exit/terminate；
- subprocess raw pipe 上的 LSP JSON-RPC framing；
- `read-only` mutation 拒绝，`workspace-write` 内部允许、越界拒绝；
- jobs output、kill、wait 和 terminal status。

当前输出：

```json
{"schema":"openmuse.dsh-provider-contract@1","dshVersion":"0.1.7-rc.1","status":"passed","capabilities":{"filesystem":true,"shell":true,"subprocess":true,"terminal":true,"jobs":true,"lspTransport":true,"lspAdapterInProductClosure":false}}
```

## 7. 对后续实现的约束

- X1 实现 Host Sandbox Service 与 Local Runtime，但仍通过这些 seam 接入；
- X2 提供完整 remote Provider 组，不复用 local `fs-sandbox`；
- X3 的 checkpoint 必须等待 subprocess/PTY/jobs quiescence；
- X4/X5 的 worker 只从受签名 registry 解析，不能向 Agent 暴露任意宿主绝对路径；
- X6 才允许把任意程序执行提升为生产能力；X0 通过不代表本机 fallback containment 达到多租户安全等级。
