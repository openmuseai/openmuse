# F0 测试矩阵

状态：`Pass` 已有自动证据；`Planned` 已设计待实现；`Blocked` 缺前置能力；`Manual` 需要设备/人工验证。

| ID | Requirement / acceptance | Fixture / fault | 端/OS/Arch | 类型 | 自动化 | 预期 | 状态与证据 |
|---|---|---|---|---|---|---|---|
| F0-T001 | Resource descriptor round-trip | R-FX-001 | TS/Rust, host-independent | contract/golden | CI | 字段完全一致 | Pass：resource tests |
| F0-T002 | Materialization 仅 opaque handle | R-FX-002/R-FX-102/R-FX-105 | TS/Rust | security/negative | CI | path/URL 与 writable URL 被拒绝 | Pass：13 TS + 3 Rust suite 内 |
| F0-T003 | optimistic commit union | R-FX-003..005/R-FX-104 | TS/Rust | contract/negative | CI | success/conflict closed union | Pass |
| F0-T004 | resource change 可重放 | R-FX-006 | TS/Rust | event/golden | CI | revision/cursor 保留 | Pass |
| F0-T005 | caller 不能强制 engine | P-FX-001/P-FX-101 | TS/Rust/DSH | security/negative | CI | engineId 拒绝 | Pass：schema + F0.3 UI/Agent Consumer |
| F0-T006 | route 明确 mode downgrade | P-FX-002 | TS/Rust | contract | CI | effective mode + reason | Pass |
| F0-T007 | fallback 记录最终 Adapter | P-FX-004/P-FX-103 | TS/Rust | contract/negative | CI | 不允许模糊 fallback | Pass |
| F0-T008 | 每 request 唯一终态 | duplicate/different receipt | TS | concurrency/idempotency | CI | duplicate 同结果；冲突拒绝 | Pass：receipt ledger tests |
| F0-T009 | manifest edit 必须有 commit | E-FX-001/E-FX-101 | TS/Rust | admission/negative | CI | 非法 manifest 拒绝 | Pass |
| F0-T010 | probe 可取消且有 TTL | E-FX-002/003 | TS/Rust | contract | CI | 字段可验证 | Pass（schema）；runtime race Planned |
| F0-T011 | open request 无 raw data | E-FX-004/E-FX-102 | TS/Rust | security/negative | CI | path/bytes/URL 拒绝 | Pass |
| F0-T012 | generation 必填 | E-FX-005/103 | TS/Rust | lifecycle/negative | CI | 无 generation 拒绝 | Pass |
| F0-T013 | view session 不可 dirty | ready→dirty writable=false | TS | state/negative | CI | 稳定错误 | Pass |
| F0-T014 | terminal session 不可复活 | closed→ready | TS | state/race | CI | 稳定错误 | Pass |
| F0-T015 | TS/Rust digest 完全相同 | 14 schema digest golden | TS/Rust | compatibility | CI | 14/14 相同 | Pass |
| F0-T016 | Dart round-trip/digest | 全部 valid fixtures | Flutter macOS/Web/Mobile | compatibility | CI | 与 TS/Rust 一致 | Blocked：F0.4 Dart package |
| F0-T017 | fake DSH→Host→Surface | docx/code/pdf fake descriptors | Desktop macOS arm64 | E2E | CI + integration | 唯一 opened receipt | Blocked：F0.2–F0.4 |
| F0-T018 | duplicate open 无副作用 | 同 request 100 次 | Desktop | race | CI | 1 Surface/1 receipt | Planned：F0.4 |
| F0-T019 | cancel/ready race | 每阶段 delay barrier | Desktop/Web | race/fault | CI | 1 terminal；无 cancel 后 Surface | Planned |
| F0-T020 | plugin reload late event | generation N event after N+1 | Desktop | race/fault | CI | late event ignored | Planned |
| F0-T021 | materialization TTL | expired/read/revoke/dispose | Host core | security/lifecycle | CI | 新读失败、幂等清理 | Pass：F0.2；真实端 E2E 待 F0.4/F0.5 |
| F0-T022 | consumer mismatch | one Adapter uses another handle | Host core | security | CI | consumer mismatch | Pass：F0.2 |
| F0-T023 | permission revoke | revoke in each session state | Desktop/Web | fault/security | CI | bounded closing；数据面失效 | Planned |
| F0-T024 | DSH 三入口同 Consumer | deliverable/mention/card snapshots | DSH | product/snapshot | CI | 等价 Presentation request | Pass：F0.3 UI snapshot |
| F0-T025 | Agent model-visible | request/result durable events | DSH | product/replay | CI | reload 可解释、无 secret | Pass：F0.3 ToolRuntime + Session seed replay |
| F0-T026 | 无 Ontology Runtime | build/test without ontology deps | all | composition | CI | 全功能工作 | Pass（合同包）；E2E Planned |
| F0-T027 | Host Bridge payload bound | max/over-limit envelopes | all | security/boundary | CI | 过限拒绝，0 大数据面消息 | Planned |
| F0-T028 | open-close soak | 1000 cycles + injected faults | Desktop macOS arm64 | reliability/perf | nightly | leak counters 全 0 | Planned |
| F0-T029 | warm fake open latency | 1000 runs | Desktop macOS arm64 | performance | nightly | p95 ≤ 300 ms | Planned |
| F0-T030 | cancel/cleanup latency | cancel at every step | Desktop macOS arm64 | performance/fault | nightly | terminal ≤ 200 ms；dispose ≤ 500 ms | Planned |
| F0-T031 | Web origin/audience | wrong origin/expired bearer | Chrome/Safari current-1 | security | CI/manual | broker 拒绝，无 token 日志 | Blocked：F0.5 |
| F0-T032 | Mobile capability subset | large resource/low memory | iOS/Android supported devices | product/perf | device farm/manual | view fallback 或明确超限 | Blocked：F0.5 |
| F0-T033 | rollback | flag on→off with active session | Desktop/Web | release/manual | rehearsal | 新请求走旧链；活动 session 有界关闭 | Planned：F0.6 |

## 证据位置

- Resource：`middlewares/dsh/core/contract-resource/tests/contracts.test.ts` 与 `rust/tests/golden.rs`
- Presentation：`middlewares/dsh/core/contract-presentation/tests/contracts.test.ts` 与 `rust/tests/golden.rs`
- EngineSession：`middlewares/dsh/core/contract-engine-session/tests/contracts.test.ts` 与 `rust/tests/golden.rs`
- DSH Seam：[../F0.3-dsh-seam/TEST-MATRIX.md](../F0.3-dsh-seam/TEST-MATRIX.md)
- 实际执行结果：[RESULTS.md](RESULTS.md)

F0 Gate 需要 F0-T001–T030 中所有非平台例外项 Pass，且 T031–T032 至少在目标发布端 Pass；当前不能签署 Gate。
