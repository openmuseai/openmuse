# F0 Fixture 设计

## 1. 目录与版本

| 合同 | 目录 | 数量 | 版本 |
|---|---|---:|---|
| Resource | `contract-resource/fixtures/v1/messages.json` | 11 | v1 |
| Presentation | `contract-presentation/fixtures/v2/messages.json` | 10 | v2 |
| EngineSession | `contract-engine-session/fixtures/v1/messages.json` | 11 | v1 |

每条 fixture 包含 `name/kind/valid/value`。`name` 以前缀和稳定 ID 开头，测试名称可直接回溯需求。正例同时用于 TypeScript 和 Rust validation；代表性对象还执行 serde round-trip。负例必须描述具体拒绝原因，不能只放随机损坏 JSON。

## 2. 当前 corpus

### Resource

- R-FX-001：verified DOCX descriptor 和惰性 Ontology eligibility hint。
- R-FX-002：consumer-bound bytes handle。
- R-FX-003：working-copy optimistic commit。
- R-FX-004/005：committed/conflict receipt。
- R-FX-006：revision change event。
- R-FX-101..105：path 泄漏、bearer URL 泄漏、重复 capability、缺 conflict revision、writable URL handle。

### Presentation

- P-FX-001：DSH deliverable + optional anchor hint。
- P-FX-002：Word save-unavailable mode downgrade。
- P-FX-003/004/005：opened/fallback/cancelled terminal receipt。
- P-FX-101..105：forced engine、缺 session ownership、模糊 fallback、非法 mode、非稳定 reason code。

### EngineSession

- E-FX-001：ioffice Word view/ephemeral-edit manifest（没有 commit）。
- E-FX-002/003：cancellable probe 与 TTL result。
- E-FX-004/005/006：opaque open、ready handle、generation event。
- E-FX-101..105：edit 无 commit、raw path、缺 generation、未知 materialization、unavailable probe 宣告可用 mode。

## 3. Digest 规则

每包的 `schema-digests.json` 由 `digest:update` 使用 Host Bridge canonical schema digest 生成。Rust 测试重新计算同一 digest 并逐项比较。禁止手工修改 digest 以绕过 schema diff。

## 4. 后续扩展 corpus

F0.2 增加 provider/TTL/revoke/consumer mismatch；F0.3 增加三类 DSH session snapshot；F0.4 增加 Dart projection、duplicate/cancel/late generation；F0.5 增加 Web origin 和 Mobile low-memory profile。

真实 vendor 文件不放入 F0 公共合同包。DOCX/PDF/代码/CAD corpus 在对应 F1/F2/F3/F6 阶段管理，公共 fixture 只表达控制面事实。

## 5. 变更规则

- 修改 required、枚举或 discriminated union 必须评估 major version。
- 新 optional 字段必须添加正例、未知/超限负例，并验证旧 consumer 行为。
- Fixture 不含真实用户名、路径、URL token、客户内容或凭据。
- 时间使用固定 epoch millisecond；ID 使用假 opaque 值；digest 使用固定无敏感内容的占位哈希。
