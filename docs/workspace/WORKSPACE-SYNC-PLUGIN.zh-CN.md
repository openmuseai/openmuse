# Workspace Sync Plugin：策略、持久化编排与冲突合同

> 状态：Accepted  
> 子需求：ST3  
> 日期：2026-09-29

## 1. 决策

Workspace Sync 是独立编排领域。它只依赖 Local Workspace 与 Cloud Resource 的端口，不引用 S3 SDK、具体 Cloud Resource Authority 实现或 UI 状态。Local Provider、Cloud Provider、对象存储和同步策略互不拥有彼此的内部状态。

同步不是默认行为。Workspace 在用户明确确认策略前保持未配置，任何本地 change feed、内容物化和 Cloud 上传都不会发生。

## 2. 四种策略

| 策略 | Local → Cloud | Cloud → Local | Cloud 用户写入 | Desktop 离线时 Mobile | 权威切换 |
| --- | --- | --- | --- | --- | --- |
| `local-only` | 否 | 否 | 否 | 否 | 始终 Local |
| `snapshot` | 是，生成只读快照 | 否 | 否 | 是，只读 | 始终 Local |
| `mirror` | 是 | 是 | 是 | 是 | 双端按 revision 协调 |
| `migrate` | 一次性 | 否 | 完成前否 | 完成前否 | 校验成功后 Local → Cloud |

`snapshot` 的内部提交权限不等于 Workspace 对用户可写；传给 Cloud Resource 端口的 `cloud_writable=false` 是可审计的策略结论。

## 3. 端口与关注点

- `LocalWorkspaceSyncPort`：按 cursor 读取 change feed、按需物化内容、读取当前 revision、应用已验证的 Cloud change。
- `CloudResourceSyncPort`：流式 staging、带 `expected_cloud_revision` 的提交、Cloud change feed、迁移校验与权威激活。
- `SyncStateStorePort`：持久化 policy、migration phase、authority、双向 cursor、work、stage、receipt 和 conflict。
- `WorkspaceSyncEngine`：只实现状态机、幂等键和调用次序；不解释 S3、路径、凭据或 UI。

UI 只能展示持久化后的 cursor、receipt、conflict、policy 与 authority，不允许根据网络请求返回或进度条推测成功。

## 4. Durable plan 与恢复

每个本地 change 首先写入 `Planned` work，再进行外部 I/O：

```text
change feed
  → persist Planned
  → stream upload (workRef:upload)
  → persist Staged + provider receipt
  → expected-revision commit (workRef:commit)
  → persist Committed + sync receipt + cursor
```

进程在 staging 后崩溃时，新进程读取同一 state store，直接使用已持久化的 stage 和稳定 idempotency key 重试 commit，不重复上传。可重试失败不前移 cursor；冲突会形成显式 conflict 后前移该 change 的 cursor，避免无界重放。

生产实现必须用事务数据库实现 `SyncStateStorePort`；内存实现仅是合同 fixture。

## 5. Mirror 冲突

Cloud change 携带 `base_local_revision`。只有它等于 Local 当前 revision 时才允许 apply。否则：

- 不覆盖 Local；
- 保存 Local 当前 revision 和 Cloud change revision；
- 按 media type 路由到 `Text`、`Office` 或 `Binary` conflict domain；
- 持久化 Cloud cursor 和 conflict receipt。

领域 merge provider 是后续可替换插件；ST3 不把文本 diff、Office CRDT 或二进制选择策略嵌入同步核心。

## 6. Migrate 状态机

```text
Transferring → Verifying → Complete
    Local          Local        Cloud authority
```

只有 change feed 已追平、全部 work 已提交并且 Cloud 端完整性校验成功，才调用 `activate_cloud_authority`。激活成功后在同一状态中记录 `Complete + Cloud`。校验失败或服务重启时继续保持 Local 权威；完成后重复 cycle 不得再次激活。

## 7. 安全与边界

- 未确认 policy：零上传、零 materialize；
- 所有内容通过 bounded `BlobReadStream`，同步核心不收集完整文件；
- 跨域只传 ResourceRef、revision、digest、cursor、handle/receipt；
- 不传绝对路径、S3 key、凭据、presigned URL 或 Provider SDK 类型；
- Local/Cloud 的写入都受各自 Authority 与 C2 policy decision 的最终校验，Sync Plugin 不是授权源。

## 8. 验收证据

统一入口：

```bash
./scripts/test_workspace_sync.sh
```

自动验收覆盖：

1. 四种 policy 的方向、可写性和离线可用性；
2. 未确认策略和 `local-only` 零上传；
3. `snapshot` 单向、只读并生成权威 receipt；
4. staging 后 commit 失败，跨 engine 重建续传且不重复上传；
5. Mirror base 相同时应用，不同时保留双方 revision；
6. Migrate 校验前保持 Local，校验后只激活一次 Cloud；
7. crate 不依赖 S3 SDK 或 Cloud Authority 具体实现。
