# Cloud Workspace Metadata、CAS 与 Outbox

- 状态：Accepted（ST2）
- 分支：`feature/st2-cloud-resource-authority`
- 实现：`crates/openmuse-cloud-resource-authority/`

## 1. 领域边界

`CloudResourceAuthority` 是 Cloud Workspace 的资源真源。Workspace tree、ACL、Resource Revision、BlobRef、materialization handle、commit receipt 与 outbox 都属于 metadata 领域；S3/RustFS/MinIO 只通过 ST0 `BlobStorePort` 保存 immutable bytes。

```text
Flutter / Plugin / DSH
          │ Resource v1
          ▼
CloudResourceAuthority
   ├── MetadataStorePort ── tree / ACL / revision / CAS / handle / outbox
   ├── BlobStorePort ────── immutable blob bytes
   └── EventPublisherPort ─ resource.changed
```

Resource Authority 不 import AWS SDK，不读取 bucket listing 构建目录，也不把 endpoint、bucket、Secret、绝对路径或 provider error 暴露给上层。

## 2. Commit 事务

Commit 分为两个明确阶段：

1. 根据 caller 提供的 SHA-256 与 size，把内容写入 `blobs/sha256/<digest>`；Blob Store 校验流、digest 与 immutable 条件并返回 storage receipt；
2. Metadata Store 在单一临界事务内校验 ACL、`expectedRevision` 与 idempotency fingerprint，更新当前 BlobRef/Revision、保存 metadata receipt，并插入 `resource.changed` outbox。

CAS 失败不会覆盖新 Revision。Blob 已成功但 metadata 失败时，Authority 返回失败并记录 `OrphanBlob`，不生成 commit success。后续 GC/reconciliation 只能依据 metadata/receipt 处理 orphan，不能根据 ListObjects 猜测当前 Workspace 状态。

## 3. Catalog 与 Materialization

Catalog 的 list/search/pagination 只访问 `MetadataStorePort`，按 display name + ResourceRef 稳定排序。10k resource 验收使用一个拒绝全部 Blob I/O 的 Provider，证明 UI tree 不依赖对象列表。

Materialization handle 固定签发时的 Revision/BlobRef，并绑定 audience、generation 与绝对 expiry；读取继续携带 StorageRequestContext，并支持 bounded full/range stream。资源后续提交不会改变既有 handle 指向。

## 4. Outbox

Metadata revision 与 outbox entry 在同一事务提交。Dispatcher 在 publish 前增加 attempt，只有 publisher 成功后才 ack；传输失败保留 entry，可在进程重启后重放。Publisher 只接收领域事件，不接收数据库事务或 S3 client。

## 5. 生产适配

`InMemoryMetadataStore` 是事务语义 reference implementation/TCK fixture。生产数据库 adapter 必须把 resource row、idempotency receipt 与 outbox insert 放进同一数据库事务，并实现等价 CAS。ST2 不绑定 PostgreSQL、SQLite 或某个消息系统；Cloud Server 可分别实现 `MetadataStorePort` 与 `ResourceEventPublisherPort`。

## 6. 验收

统一入口：

```bash
scripts/test_cloud_resource_authority.sh
```

覆盖：

- 两个并发 writer 只有一个 commit，另一个得到 `CONFLICT`；
- Blob 成功、metadata 失败只产生可回收 orphan，Revision 不变化；
- metadata 成功、event publish 失败后 outbox 可重放并 ack；
- 10k resources 的 catalog/search 不发生 Blob/S3 I/O；
- handle 的 revision pinning、audience、generation、TTL 与 range data plane。
