# F0 实施结果

最后更新：2026-09-15  
阶段状态：**F0.1 Pass；F0.2 Host Resource Core Pass；F0.3 DSH Seam Pass；F0 总 Gate 未完成**

## 1. 已完成

- 新建 `@muse/contract-resource` v1.0.0：5 schemas、11 fixtures、TS/Rust validation/digest/round-trip。
- 新建 `@muse/contract-presentation` v2.0.0：3 schemas、10 fixtures、TS/Rust validation/digest/round-trip、terminal receipt ledger。
- 新建 `@muse/contract-engine-session` v1.0.0：6 schemas、11 fixtures、TS/Rust validation/digest/round-trip、session state machine。
- schema 静态检查阻止 path/URL/token/PTY/Ontology object 字段进入公共合同。
- Ontology 仅有可选惰性标签，无 package/runtime/service 依赖。
- 没有修改 `vendors/helix`、`vendors/ioffice`、`vendors/open-file-viewer` 的运行代码。
- 新建 `@muse/resource-host`：Provider Registry、全 audience materialization、commit/idempotency、event retention 和 fault-safe disposer；25 tests Pass。
- 新建 F0.3 四包：DSH Service Definition、Host Provider、UI Consumer、Agent Tool Consumer；20 Muse tests Pass。
- DSH Session 增加非 Surface `ignorable:true` producer contract；1 个阶段专项 test Pass。
- 三 UI 入口共用 journaled request；Agent recent-ref、model-safe result、Session seed replay 已验证。

## 2. 执行环境

| 项 | 实际值 |
|---|---|
| OS/arch | macOS / Apple Silicon workspace host |
| Node | v24.16.0 |
| pnpm | 10.34.1 |
| rustc | 1.96.0 (2026-05-25) |
| cargo | 1.96.0 (2026-05-25) |

## 3. 实际命令与结果

每包执行：

```bash
pnpm install --offline --frozen-lockfile=false
pnpm digest:update
pnpm check
```

`pnpm check` 包含 contract static check、TypeScript strict typecheck、Vitest、TypeScript build、`cargo fmt --check`、Rust tests。

| 包 | Contract | TS tests | Rust tests | Typecheck/build/fmt | 结果 |
|---|---:|---:|---:|---|---|
| contract-resource | 5 schema / 11 fixture | 13 | 3 | Pass | Pass |
| contract-presentation | 3 schema / 10 fixture | 14 | 3 | Pass | Pass |
| contract-engine-session | 6 schema / 11 fixture | 15 | 3 | Pass | Pass |
| 合计 | 14 schema / 32 fixture | 42 | 9 | Pass | **51 tests Pass** |

F0.2 另有 Resource Host strict TypeScript/Vitest/declaration build：**25 tests Pass**。F0.3 四包 strict TypeScript/Vitest/declaration build：**20 tests Pass**，DSH Session 新增 producer contract：**1 test Pass**。当前累计阶段专项 test case：**97 Pass**。

Rust 首次执行更新 crates.io index 并锁定依赖；之后使用生成的 `Cargo.lock`。pnpm 依赖从本机 store 离线复用。

## 4. 测试中发现并修复

1. Rust 源文件初次未满足 `cargo fmt --check`，执行格式化后复验通过。
2. Presentation union 测试构造可能跨 success/failure 分支，改为同一合法 union 内改变 completion time 来验证 terminal conflict。
3. Engine contract 泄漏扫描最初误匹配 `*-url` 枚举，收紧为 JSON 字段名 `"url":`/`"path":`。
4. Adapter manifest 的 edit/commit 关系最初只有文档约束，改为 JSON Schema `if/then + contains`；并补 strict AJV 所需 array type。
5. EngineSession Error subclass 构造器在 `super` 前访问 `this`，修正并由 strict TypeScript 复验。

## 5. 尚未执行 / Blocked

- Dart projection 和 Flutter contract tests（F0.4 未实现）。
- fake Resource → DSH seam 已完成；fake Surface E2E 与 fault leak counter（F0.4 未实现）。
- Web/Mobile、performance、soak、rollback rehearsal（F0.5–F0.6 未实现）。

## 6. 指标

合同层可报告：14/14 schema 跨 TS/Rust digest 一致；32/32 fixtures 在两端得到预期 valid/invalid 结论；42 TS + 9 Rust tests 全通过。

端到端打开/取消延迟与 Surface leak count 尚缺 F0.4 Fake Surface runtime，未测量，不能填 0
或推测值；F0.3 已固定 DSH seam 指标、active/binding counters 与后续基准入口。

## 7. Gate 决定

- F0.1 Contract Freeze：**Pass，可进入评审/合并**。
- F0.2 Host Resource Core：**Pass**；生产 Provider 绑定进入后续集成阶段。
- F0.3 DSH Seam：**Pass**；真实 Surface 仍由 feature flag 隔离。
- G0 全局合同冻结：**Blocked**，原因是 Dart projection 尚未完成。
- F0 公共打开链：**Blocked**，原因是 Host/DSH/Flutter fake E2E 尚未实现。
- F1/F2/F3 真实 Adapter 开发：可做 vendor-independent spike，但不得接入或开启产品 flag。

下一实施项：F0.4 Client Orchestrator，把同一 fake Resource/Provider 接到 fake Adapter 与 fake Surface。
