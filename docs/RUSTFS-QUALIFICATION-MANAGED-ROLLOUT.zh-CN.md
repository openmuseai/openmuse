# RustFS Qualification 与 Managed Rollout Gate

> 工程状态：Accepted  
> 生产默认 Provider 状态：Ineligible / fail-closed  
> 子需求：ST5  
> 日期：2026-09-29

## 1. 当前结论

ST5 已建立可执行、版本化、fail-closed 的 qualification 判定器与证据格式；当前 RustFS 仍**不具备**成为 OpenMuse Managed Storage 默认 Provider 的资格。

已知证据只有 ST1 对 RustFS `1.0.0` 的 OpenMuse S3 Profile 9/9 兼容性验证。兼容性测试不能替代制品供应链验证、四节点多盘故障演练、生产等价 soak 或 canary。当前证据账本 [`qualification/storage/st5-rustfs-current.json`](qualification/storage/st5-rustfs-current.json) 如实保留未完成字段，不伪造 image digest、签名、SBOM 或经过时长。

这不阻塞 OpenMuse 使用其他已认证 S3 Provider 发布，也不改变“产品依赖 S3 Storage ABI，而不依赖 RustFS”的架构决策。

## 2. 自动判定合同

`openmuse-storage-qualification` 接受 `openmuse.rustfs-qualification@1` evidence，输出：

- `eligible`：所有工程、安全、演练、soak、scrub、canary 和 fallback 门禁通过；
- `blocked_by_soak`：其他证据全部通过，唯一不足是生产等价 soak 小于 2160 小时；
- `ineligible`：存在任何工程、安全、完整性、回滚或证据缺口。

只有 `eligible` 可进入 Managed 默认 Provider 的人工发布审批。判定器不会根据日历推测 soak，不接受兼容性 TCK 折算时长，也没有 waiver 参数。

## 3. 必需证据

### 制品与安全

- 固定 RustFS stable version 和不可变 image SHA-256；
- artifact signature 验证；
- SBOM SHA-256 与漏洞扫描；
- 未处置 high/critical finding 数为零；
- 升级目标、回滚目标和保留窗口固定。

### 生产等价拓扑与故障演练

- 至少 4 nodes、每节点至少 2 drives；
- node loss、drive loss、network partition、disk full、recovery；
- rolling upgrade 与 upgrade rollback；
- 每项都有 evidence ref，实测 RPO/RTO 不超过声明目标。

### 长期完整性

- 至少 2160 小时（90×24）生产等价连续 soak；
- 定期遍历 metadata-owned BlobRefs 做 digest scrub；
- digest mismatch 必须为零；出现 mismatch 后证据重新归零并进入事故流程。

### Canary 与退出能力

- shadow copy / 双读累计至少 1000 次且 mismatch 为零；
- 至少一个隔离小租户 canary；
- canary rollback 已演练；
- 至少两个 fallback Provider 同时通过 S3 Profile 和 ST4 迁移演练。

## 4. 演练与收集流程

1. 将固定版本、image digest、签名和 SBOM 结果写入新的 evidence revision；
2. 在四节点多盘生产等价环境逐项执行故障演练，保存不可变日志/指标引用；
3. 开始 soak，按真实健康时长累计 `productionEquivalentSoakHours`，中断期不计入；
4. 周期性 digest scrub，将对象数和 mismatch 写入证据；
5. shadow read 后进入小租户 canary，执行一次真实回滚；
6. 用 ST4 在 RustFS 与两个 fallback Provider 间往返迁移；
7. 使用 `--require-eligible` 执行发布 Gate，非零退出即禁止设为默认。

发布 Gate 示例：

```bash
cargo run --package openmuse-storage-qualification \
  --example evaluate_report -- evidence.json --require-eligible
```

## 5. 验收与当前证据

工程验收入口：

```bash
./scripts/test_rustfs_qualification.sh
```

自动测试证明：

1. 完整证据可以得到 `eligible`；
2. 2159 小时仍为 `blocked_by_soak`，不能提前；
3. 任一 digest mismatch 或缺失演练均为 `ineligible`；
4. fallback 必须同时有 TCK 和迁移演练；
5. 当前真实账本被明确判为 `ineligible`。

因此本分支完成的是 ST5 工程系统和长期 qualification 的启动条件，不声称完成尚未经过的 90 天实测。后续只需持续追加真实证据，不需要修改客户端或 Storage ABI。
