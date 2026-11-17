# BYOS、Provider 迁移与数据可移植

> 状态：Accepted  
> 子需求：ST4  
> 日期：2026-09-29

## 1. 结论

OpenMuse 的用户数据自主建立在标准 `BlobStorePort` 与 Workspace Metadata Authority 之上，不建立在某个对象存储产品上。用户可以选择 OpenMuse 托管 Provider、自己的 S3-compatible Provider，或由 Desktop 代管凭据的 Provider；迁移协议不因执行位置改变。

ST4 新增 `openmuse-storage-portability` 编排层。它不引用 AWS SDK、S3 URL、bucket/key 方言或明文凭据，只消费：

- `ProviderAccessBrokerPort`：返回兼容性快照，并以 provider ref + access generation 解析短期可用的 `BlobStorePort`；
- `PortabilityMetadataPort`：枚举 metadata-owned BlobRefs、CAS 切换 provider generation、批准安全删源对象并签发 metadata receipt；
- `MigrationStateStorePort`：持久化对象级复制/校验/删除进度和回执。

## 2. Server-mediated 与 Desktop-mediated BYOS

两种模式共享完全相同的状态机和对象合同：

| 模式 | 凭据所在边界 | 执行位置 | Cloud 服务能否取得明文凭据 |
| --- | --- | --- | --- |
| Server-mediated | Server Vault | Cloud worker | 否，worker 只取得 brokered provider handle |
| Desktop-mediated | 用户 Desktop Keychain/Vault | 配对 Desktop worker | 否 |

Provider configuration 只保存 `provider_ref`、`access_generation`、mediation 和 capability snapshot。迁移选择的 placement 必须同时匹配源和目标；不匹配在任何 Blob I/O 前拒绝。

## 3. 迁移状态机

```text
Copying ⇄ Paused
   │ all objects digest-verified
   ▼
ReadyToSwitch
   │ metadata CAS + provider generation++
   ▼
Observing ── rollback ──> RolledBack
   │ explicit second confirmation
   ▼
Completed
```

每个对象由 metadata-owned `resource_ref + revision + BlobRef` 驱动：

1. 从源按 BlobRef 流式读取，先核对 descriptor digest/size；
2. 以 digest-addressed object ref、稳定 idempotency key 流式写入目标；
3. 对目标执行 HEAD 并再次核对 digest/size；
4. 每个对象完成后立即持久化 `Verified`、目标 BlobRef 和 provider receipt；
5. 所有对象完成后才能进入 `ReadyToSwitch`。

服务重启后从 state store 继续未完成对象。暂停不访问 Provider；恢复时重新校验两侧 access generation。凭据撤销或轮换后旧 generation 立即失败，不允许后台继续访问。

## 4. 原子切换、迟到写与回滚

Cutover 向 Metadata Authority 提交：

- 当前 workspace provider generation；
- 每个 resource 的 expected revision；
- 已校验的目标 BlobRef；
- migration ref。

Authority 在单个事务中替换 BlobRefs、切换 provider 并增加 generation。旧 generation 的迟到写随后以 `CONFLICT`/`STALE_GENERATION` 失败。并发 resource revision 改变也使整个切换失败，不产生部分切换。

观察期保留源对象和源映射快照。Rollback 用当前 generation CAS 回源 Provider，并恢复原 BlobRefs；删源后不可再执行自动 rollback。

## 5. 安全删源与回执

删源需要独立的第二次用户确认。Portability Service 把候选 object refs 交给 Metadata Authority；只有 Authority 根据跨 Workspace 引用计数返回的安全子集才会被删除，避免误删共享 digest 对象。

每个实际删除产生 Provider receipt，随后 Metadata Authority 记录解绑/保留结果并产生 metadata receipt。只有两侧回执都持久化后迁移才进入 `Completed`。中途失败可跳过已有 delete receipt 继续。

## 6. 可移植导出

`openmuse.workspace-export@1` manifest 包含：

- Workspace ref、当前 Provider ref 与 generation；
- 每个资源的 ResourceRef、revision、media type；
- 原始 BlobRef、SHA-256 digest 与 size。

manifest 不包含 credential、presigned URL 或厂商 SDK 类型。bytes 可按 manifest 中的 BlobRefs 通过独立导出工具流式取得并逐项验 digest。

## 7. 验收

统一入口：

```bash
./scripts/test_storage_portability.sh
```

自动验收覆盖：

1. 两个独立 Blob Provider A→B、B→A 双向迁移并逐对象校验；
2. 分批复制后重建 Service，从持久化状态继续；
3. pause/resume、凭据 revoke 后零新增 Provider resolve；
4. Server/Desktop mediation 不匹配 fail closed；
5. cutover 使旧 generation 写失败，观察期可回滚原 BlobRefs；
6. 未二次确认不能删源，成功删除同时返回 Provider 与 metadata receipts；
7. export manifest 保留 revision/digest，且不含 credential；
8. crate 中无 S3 SDK、S3 URL 或 access/secret key。
