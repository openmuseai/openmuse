# Sandbox 与本地运行时

Sandbox、Host 服务、Lease 与本地运行时：定义谁能申请执行、租约如何约束、生产多租户的安全边界。Agent 只寻址稳定的命令入口，不能直接触碰 worker 路径或控制 socket。

## 文档

| 文档 | 内容 |
| --- | --- |
| [Host Sandbox Service、Lease 与 Local Runtime](HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md) | 本地 Sandbox 服务、Lease 模型与运行时装配 |
| [Office Dispatcher 与首个 Worker](OFFICE-DISPATCHER-WORKER-SUPERVISOR.zh-CN.md) | Office 命令分发、Worker 权限、超时与只读 DOCX worker |
| [Sandbox 生产安全与多租户 Gate](SANDBOX-PRODUCTION-SECURITY-GATE.zh-CN.md) | 上线前必须满足的隔离、权限与审计门禁 |

## 相关领域

- 执行平面与 DSH Provider：[dsh/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY](../dsh/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md)
- Office 查看与 DOCX 引擎：[platforms/mobile](../platforms/mobile/README.md)
- 发布门禁：[release/gates](../release/gates/README.md)
