# F0.3 测试矩阵

状态：`PASS` 为本阶段自动化已通过；`DEFER` 明确属于后续阶段。

| ID | 场景 | 层级 | 端 | 预期 | 状态 |
|---|---|---|---|---|---|
| DS-001 | Presentation wire request schema digest | contract | all | 与 F0.1 v2 完全一致 | PASS |
| DS-002 | status/cancel schema digest 稳定 | contract | all | descriptor exact match | PASS |
| DS-003 | request journal 顺序 | unit | DSH | requested 先于 terminal | PASS |
| DS-004 | Provider throw | unit | DSH | safe failed receipt durable | PASS |
| DS-005 | terminal replay | replay | DSH | requests/receipts 可恢复 | PASS |
| DS-005A | 仓库外 Muse event 持久化兼容 | persistence | DSH | append 写 `ignorable:true`；未知读取器可 load；Surface 拒绝标记 | PASS |
| DS-006 | recent ref 时间界 | security | Agent | 过期拒绝 | PASS |
| DS-007 | recent ref event distance | security | Agent | 超窗口拒绝 | PASS |
| DS-008 | undiscovered/cross-session ref | security | Agent | Host invoke=0 | PASS |
| DS-009 | path-shaped ref | security | UI/Agent | 入口拒绝 | PASS |
| DS-010 | Agent-owned Session 缺失 | security | Agent | Tool error | PASS |
| DS-011 | model schema secret scan | static | Agent | path/URL/engineId/handle/token=0 | PASS |
| DS-012 | model result secret scan | product | Agent | Engine/Surface/Adapter refs=0 | PASS |
| DS-013 | Tool result seed replay | replay | Agent | derived messages 相等 | PASS |
| DS-014 | Tool effect disposer | lifecycle | DSH | schemas 清空 | PASS |
| DS-014A | bundle rollout disabled | release | Agent | `enabled:false` 时 Tool 不发布 | PASS |
| DS-015 | 三 UI 入口归一 | snapshot | Desktop/Web | 除 cause 外一致 | PASS |
| DS-016 | UI reload recovery | replay | Desktop/Web | Host invoke=0 | PASS |
| DS-017 | 缺 OpenResource Provider | composition | all | 明确错误 | PASS |
| DS-018 | Host descriptor incompatible | compatibility | Host | CAPABILITY_UNAVAILABLE | PASS |
| DS-019 | scope hints | security | Host | session/actor/workspace；无权限 flags | PASS |
| DS-020 | receipt requestRef mismatch | security | Host | 拒绝 | PASS |
| DS-021 | duplicate requestRef | idempotency | Host | invoke 一次 | PASS |
| DS-022 | terminal status/cancel cache | unit | Host | 不重复 invoke | PASS |
| DS-023 | binding invalidation | lifecycle | Host | 下一请求 rebind | PASS |
| DS-024 | Provider dispose | lifecycle | Host | binding=0, active=0 | PASS |
| DS-025 | F0.2 Fake Resource vertical slice | integration | Host | Host 内部 authority describe 成功 | PASS |
| DS-026 | full Muse package build/copy map | build | Desktop | 全包 build 成功 | PASS |
| DS-026A | macOS/Windows/closure 清单一致 | packaging | Desktop | F0.1–F0.3 包均存在；集合无漂移 | PASS |
| DS-027 | Web range/origin | integration | Web | 同协议的数据面限制 | DEFER F0.5 |
| DS-028 | Mobile low-memory/placement | integration | Mobile | deterministic downgrade | DEFER F0.5 |
| DS-029 | Fake Surface accepted/pending/cancel | integration | Client | terminal invariant | DEFER F0.4 |
| DS-030 | 1000 次 open/reload/dispose soak | soak | all | counters=0 | DEFER F0.6 |

覆盖重点不是行覆盖率，而是 authority、idempotency、replay、model surface 和 disposer 五类
不变量。DS-005A 由新增 DSH Session producer test 与既有 JSONL/SQLite unknown-ignorable
persistence contract 共同覆盖。真实格式引擎 vendor 合同测试从 F1/F2/F3 开始，不在 F0.3 提前伪造。
