# OpenMuse Broker Delegation、Policy 与 Audit

- 状态：Accepted（C2）
- Owner：OpenMuse Host Security
- 实现：`openmuse-plugin-protocol::DelegatedRequestContext`、`openmuse-platform-runtime::DelegationBroker`

## 1. 安全边界

C2 支持同一用户动作经过 `User → DSH → Sandbox Provider → Office Worker` 多跳执行，但任何一跳都不能继承另一个主体的 ambient authority。Wire context 显式区分：

- `actor`：最初发起动作的用户/Agent；
- `caller`：当前直接调用 Broker 的 Plugin；
- `target_provider`：本次准备调用的 Provider/Worker；
- `scope` 与 `revision`：authority、Workspace、Resource 与内容版本；
- `handle_ref / generation / deadline`：本次 delegation 的可撤销、可过期边界；
- `request_id / operation`：幂等关联与审计动作。

Request context 定义在 protocol crate；Policy、handle state 和 audit storage 定义在 runtime。业务 payload 不进入 Policy 对象，避免日志或授权层无意持有文档内容、shell 参数和 Secret。

## 2. 有效权限

命令实际可用权限为以下集合的交集：

```text
tenant/Host ceiling
∩ actor session grants
∩ caller Plugin grants
∩ target Provider grants
∩ Workspace classification grants
∩ capability Handle grants
∩ command required permissions
```

Broker 只有在交集完整覆盖 `command required permissions` 时才允许调用。一次性 escalation 只能收窄集合，不能突破 Host ceiling、target grant 或 Workspace classification。Manifest v2 的 `requested_permissions` 不参与直接授权；它只是 Policy 配置输入。

C2 提供 `InMemoryPolicyProvider` 作为确定性 reference provider/TCK 输入，后续账号、组织与 Workspace classification 服务必须实现同一交集语义。

## 3. Capability Handle

Handle 绑定 actor、audience、scope、permissions、generation 和 expiry。授权时依次验证：

1. 请求 deadline；
2. handle 存在且未撤销；
3. handle 未过期；
4. actor 精确匹配；
5. audience 精确等于当前 caller；
6. authority/workspace/resource scope 精确匹配；
7. generation 精确匹配；
8. Policy 权限交集完整。

任何失败均 fail-closed，并产生 denied audit receipt。Handle 不能跨 Workspace 重放，旧 generation 不会自动升级，错误 audience 不会转交给 target。`issue_handle` 是 Host 内部 port；即使输入了过宽 permissions，最终授权仍会被所有 Policy ceiling 收窄。

## 4. 审计与脱敏

每次决策生成单调 decision ref，并记录 request、actor、caller、target、authority、Workspace、revision、operation、generation、outcome 和稳定错误码。这样可以把同一 actor 的 DSH→Sandbox 与 Sandbox→Worker 两条 receipt 串起来。

Audit 不记录 capability bearer value、业务 payload、命令参数或 credential。所有可记录 Ref/operation 在写入前执行防御性脱敏：疑似绝对 path、query/presigned URL、S3 URI、token/secret/access-key 字样统一变为 `[redacted]`。日志错误只使用 C0 词汇 `DENIED / NOT_FOUND / CONFLICT / EXPIRED / STALE_GENERATION / INTEGRITY_FAILED`。

## 5. 领域隔离与后续接入

- C2 不实现 Workspace list/materialize/commit；这些由 C3 提供；
- C2 不保存账号 credential，也不把 Secret 导出给 Plugin；生产 Secret 必须 brokered use；
- Mobile approval 未来只签发当前 actor/action/scope 的短期 Handle，不修改 Desktop 或 Workspace 的长期 grant；
- Sandbox/Office worker 只能作为 `target_provider` 被授权，不能使用 DSH 的 Handle 冒充 caller；
- 当前 audit sink 为内存，生产持久化 sink 必须保持同一白名单字段和脱敏测试。

## 6. 验收

统一入口：

```bash
scripts/test_broker_delegation.sh
```

验收覆盖合法双跳调用、confused-deputy、target grant 缺失、错误 audience、撤销/过期 Handle、stale generation、跨 Workspace 重放，以及 path/token/presigned query/S3 key 不进入审计输出。
