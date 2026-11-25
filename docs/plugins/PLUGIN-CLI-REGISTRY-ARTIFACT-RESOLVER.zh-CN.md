# Plugin CLI Registry 与 Artifact Resolver

> 状态：Accepted
>
> 子需求：X4
>
> 日期：2026-09-29

X4 只从 Host 已安装且通过 C1/C2/C4 admission 的 Plugin 生成 Agent CLI registry。Workspace 的 `plugins.json` 只能表达 requirement：未知 id 会显示 unavailable，不能安装 Plugin、增加 grant 或注册命令。

Resolver 依次校验 target、artifact kind、digest evidence、signature、revocation、ABI、license、grant 与 command effect。命令 identity 为 `group/namespace/command`；冲突按排序后的 Plugin id 确定性失败，不因加载顺序覆盖。

每次解析产生带 SHA-256 digest 和 generation 的不可变 snapshot，并绑定到 Sandbox Lease。Plugin 更新只影响显式创建/刷新后的新 snapshot，既有 Session 不静默升级。Capability discovery 仅返回当前 target 实际可执行且权限已授予的命令；不匹配的 Desktop-only artifact 明确 unavailable。

统一验收：

```bash
./scripts/test_cli_registry.sh
```

测试覆盖 Cloud 中 macOS-only Plugin、Lease snapshot 冻结、伪造 Workspace requirement、grant 过滤和 namespace 冲突。
