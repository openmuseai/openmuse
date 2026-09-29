# OpenMuse Plugin Manifest v2

- 状态：Accepted（C1）
- Owner：OpenMuse Plugin Platform
- Schema：`schemas/openmuse.plugin.v2.schema.json`
- Fixtures：`schemas/fixtures/plugin/v2/`

## 1. 目标与边界

Manifest v2 把“插件是什么”“在哪个平台可运行”“需要装载哪些 artifact”“如何展示 UI”“如何连接执行面”和“向 Agent 暴露哪些 CLI”拆成独立字段。它是声明与打包输入，不是授权结果：`requested_permissions` 只表达插件申请的能力，实际 grant 必须由 C2 Policy/Broker 计算。

v2 不再用 v1 的单一 `runtime` 同时暗示 UI 与进程模型：

- `ui_runtime`：`none / flutter / web-view`；
- `execution_connector`：`none / host-process / remote-dsh / sandbox-provider`；
- `presentation`：Host 可放置的 surface 及其 remote presentation 能力；
- `compatibility`：完整 target triple 的显式支持/不支持决策；
- `artifacts`：目标相关、可校验的交付单元；
- `contributes`：命令、服务、编辑器、面板和 Agent CLI。

上述字段互不授权。比如 `sandbox-provider` 不自动获得进程、Workspace 或 Secret 权限；`remote_capable` 也不表示 Mobile 可以连接任意 Desktop。

## 2. Target 与 resolution

Target 使用 `{os, arch, libc}` 精确表示。目前冻结的 OS 为 macOS、Windows、Linux、Android、iOS、Web；未知 OS/arch/libc 或不合法组合一律拒绝。Resolver 的规则是：

1. manifest 必须恰好有一个匹配 target 决策；
2. 决策必须为 `supported`；
3. 每个支持的 target 至少有一个 artifact；
4. artifact 只能指向已支持的 target；
5. 不允许用“相近架构”、Desktop artifact 或无 target artifact 给 Mobile 降级。

当前实现事实如下：

| Plugin | macOS | Windows | Linux | Android / iOS | 原因 |
|---|---|---|---|---|---|
| Helix | arm64/x86_64 | x86_64 | 否 | 否 | 依赖 Desktop PTY 与本地 native process |
| DSH Agent | arm64/x86_64 | x86_64 | 否 | 否 | Mobile remote connector 属于 M2，不复用 Desktop sidecar |
| Native Text Gate | arm64/x86_64 | x86_64 | 否 | 否 | 只有 NSView/HWND 实现 |
| Open File Viewer | arm64/x86_64 | 否 | 否 | 否 | Mobile Resource viewer 集成属于 M3 |

这里描述“当前可以构建并注册的实现”，不是产品未来承诺。后续子需求交付真实实现和 artifact 后才能把对应状态改成 supported。

## 3. Artifact 与完整性

Artifact 包含：

- 稳定 `id` 与 `kind`；
- 精确 target；
- SHA-256 digest；
- 可选 Ed25519 signature 元数据；
- SPDX license/expression；
- Host/worker `abi`。

Sandbox worker 是安全敏感 artifact：Host 注册前必须读取实际 bytes、重算 SHA-256、要求 signature，并委托信任域提供的 `ArtifactSignatureVerifier` 验签。缺签、digest 不符或 verifier 拒绝都不能产生 `VerifiedArtifact`。Manifest parser 不拥有私钥、信任根或文件路径；C4 Distribution Lock 负责把 artifact id 绑定到发布闭包与 SBOM。

Fixture 中的 digest/signature 是协议测试值，不是发布签名或发行锁。任何生产构建都必须由 C4 以实际构建产物替换。

## 4. Agent CLI ABI

`agent_cli` 使用 `(group, namespace, command)` 形成唯一身份，并声明：

- JSON `schema`：参数结构；
- `required_permissions`：命令所需能力；
- `effects`：可审计的稳定副作用类别。

CLI contribution 只进入 Registry；是否向某个 Agent 暴露、是否执行、执行时使用哪个 target worker，分别由 C2 Policy、X4 Registry/Resolver 和 X1/X2 Sandbox 决定。Schema 不能包含 shell string 拼接规则、物理 path 或 Secret。

## 5. v1 迁移策略

v1 继续由原 parser 读取，然后通过纯函数 `migrate_manifest_v1` 产生 v2 内存模型；运行时不再增加第二套隐式解释规则。迁移保持 id、贡献、权限申请和 activation events，但由于 v1 没有 target digest/signature：

- 所有已知 target 都保守标为 unsupported；
- artifacts 为空；
- Agent CLI 为空；
- native-process 映射成 Flutter UI + host-process connector；
- built-in/web-view 只映射已知 UI/connector 语义。

因此迁移是确定、无 I/O、无环境探测、可重复序列化的，但迁移结果不能直接进入 target resolution。发行/开发清单必须显式补齐 v2 artifact 元数据。这避免 v1 插件被错误打入 Mobile 或 Sandbox。

## 6. 变更与验收

v2 对未知顶层字段、嵌套字段、枚举、target 和重复身份 fail-closed。统一验收入口：

```bash
scripts/test_plugin_manifest_v2.sh
```

验收覆盖四个现有插件 fixture、Android/iOS 排除、未知字段/target、v1 parser 回归与确定迁移，以及 Sandbox worker 的签名/digest gate。Schema 与 fixture inventory 由 manifest digest 固定；修改时必须同时更新 Rust 和 Dart 投影。
