# Sandbox 生产安全与多租户 Gate

> 状态：Accepted（工程 Gate）
>
> 子需求：X6
>
> 日期：2026-09-29

生产 admission 要求 rootless、只读 image、seccomp、capability drop、`/proc` 隔离及 CPU/memory/PID/disk/time 五类非零 ceiling。DSH 或 worker 请求只能缩小 ceiling，超限直接拒绝。

Artifact 必须同时满足 digest、有效签名、SBOM digest 和未撤销。Egress 默认为拒绝，只允许精确 HTTPS hostname；Secret broker 只发 scoped handle，child env 清除 secret/token/password/access-key。Workspace entry 拒绝 traversal、symlink 和 hardlink；archive 有展开大小、倍率与条目数上限；stdout 拒绝超限、非法 UTF-8、NUL 和 ANSI control injection。

每次 admission 生成 actor→caller→target→artifact→draft→expected revision 的 hash-chained audit receipt。取消、超时、PID burst 和 orphan cleanup 复用 X1/X2 的完整 process range 收敛。

统一验收：

```bash
./scripts/test_sandbox_security_gate.sh
```

自动 Gate 包含 5 项策略/攻击面测试、X1 隔离容器完整回归和真实容器 PID ceiling 压测。当前自动化范围无未处置 P0/P1，证据在 `security/sandbox-threat-findings.json`。独立渗透测试、生产容量/成本阈值和外置签名密钥仍是每个部署环境的 release ceremony，不能由源码单测替代。
