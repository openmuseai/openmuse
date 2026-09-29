# OpenMuse Workspace / Resource Authority v1

- 状态：Accepted（C3）
- Owner：OpenMuse Resource Platform
- Wire contract：`muse_resource_contract`
- Provider port/TCK：`muse_resource_bridge/src/authority.dart`

## 1. 决策

Mobile、Cloud Sync、Sandbox、Office 与 Desktop Host 统一依赖 `MuseWorkspaceAuthority`，不依赖本地目录、S3 bucket 或某个数据库。跨域稳定身份为：

- `WorkspaceRef`：用户可选择和授权的 Workspace；
- `MountRef`：Workspace 中某个 Provider mount；
- `ResourceRef`：Provider 内稳定资源身份；
- `Revision`：不可变的已提交内容版本；
- `DraftRef`：未提交 working copy；
- `HandleRef`：短期、绑定 audience/generation/TTL 的 materialization。

当前 Dart API 继续使用 opaque string 表达这些 Ref，以兼容已冻结的 Resource v1 wire；Provider 和 Consumer 不得解析其格式。物理 path 只允许存在于未来 Local Provider 的私有 adapter 内部，不能出现在 Descriptor、Handle、Receipt、Event 或远程 API。

## 2. Provider Port

统一 port 提供：

- `list(cursor, limit)`：稳定排序与分页；
- `describe(ResourceRef)`：返回无 path Descriptor；
- `materialize(...)`：签发固定 revision/digest 的 Handle；
- `createDraft / writeDraft`：创建与修改 working copy；
- `commit(...)`：使用 expected revision 与 idempotency key 提交；
- `subscribe()`：发布 committed revision event。

`MuseInMemoryWorkspaceAuthority` 是 reference provider/TCK fixture，不是生产 metadata store。Local 与 Fake Cloud 运行完全相同的 TCK；Paired Desktop Provider 后续也必须通过同一套行为测试。

## 3. Materialization 安全

Handle 绑定 ResourceRef、Revision、digest、audience、generation 和 expiry：

- audience 不匹配返回 `DENIED`；
- generation 不匹配返回 `STALE_GENERATION`；
- `now >= expiresAt` 返回 `EXPIRED`；
- Handle 固定签发时的 digest，即使 Resource 后续 commit，也不能读取新 revision；
- Local reference provider 返回 `bytes-handle`，Cloud 返回 `stream-handle`，两者均不返回 path 或 bearer URL。

真正的 local file handle、remote stream 或 URL 只能由数据面根据 HandleRef 解析，并继续接受 C2 audience/scope Policy 约束。

## 4. Draft、CAS 与幂等

Draft 明确记录 base revision，与 committed Revision 分离。Commit 同时要求：

1. draft 的 resource、audience、generation、TTL 有效；
2. `expectedRevision == currentRevision`；
3. `expectedRevision == draft.baseRevision`；
4. 同一 idempotency key 的 fingerprint（commit/resource/revision/draft/audience/generation）完全相同。

成功生成新 immutable Revision、Provider receipt 与 change event；内容未变返回 `no-change`；revision 变化返回可幂等复现的 `conflict` receipt。同一 key 搭配不同输入直接返回 `CONFLICT`。

## 5. Blob 与 metadata 原子边界

Reference provider 刻意模拟“Blob 已写入、metadata commit 失败”：

- 不更新 Resource 当前 Revision；
- 不缓存成功 Receipt；
- 不发布 change event；
- 返回 `METADATA_COMMIT_FAILED`；
- orphan blob 可被观测，供回收/Outbox 测试使用。

生产 Cloud Workspace 在 ST2 中用 metadata transaction + outbox/reconciliation 实现这一边界。C3 不把内存实现误称为生产 Cloud 存储。

## 6. 验收

统一入口：

```bash
scripts/test_workspace_resource_authority.sh
```

Local 与 Fake Cloud 共同覆盖 list/describe/materialize/draft/commit/subscribe、snapshot pinning、CAS/conflict/idempotency、audience/generation/expiry、物理 path 拒绝，以及 Blob 成功而 metadata 失败不能返回 commit success。
