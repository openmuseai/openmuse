# F0.2 测试矩阵

执行入口：`middlewares/dsh/core/resource-host/tests/resource-host.test.ts`

| ID | Requirement | Fixture / fault | 平台 | 类型 | 预期 | 状态 |
|---|---|---|---|---|---|---|
| HR-T001 | 显式 resource binding | fake provider/resource | Node, host-independent | unit | resolve 到唯一 Provider；解绑后 not found | Pass |
| HR-T002 | generation 单调 | duplicate + stale registration | Node | negative/lifecycle | duplicate/stale fail closed | Pass |
| HR-T003 | describe authority | read ACL on/off | Node | security | unauthorized 拒绝 | Pass |
| HR-T004 | Provider 输出归属 | descriptor 返回其他 resourceRef | Node | hostile-provider | contract violation | Pass |
| HR-T005 | 控制/数据面隔离 | bytes materialization | Node | security | public handle 无 bytes/path/url | Pass |
| HR-T006 | 数据不可借返回值篡改 | mutate resolved byte clone | Node | isolation | 二次 read 保持原值 | Pass |
| HR-T007 | 完整 audience binding | actor/workspace/session/adapter/resource/revision | Node | security/negative | 任一 mismatch 拒绝 | Pass |
| HR-T008 | access mode 不升级 | read → request read-write | Node | security | access denied | Pass |
| HR-T009 | TTL/revoke/owner close | clock advance + duplicate revoke | Node | lifecycle | disposer 恰好一次、count=0 | Pass |
| HR-T010 | Provider kind 欺骗 | request bytes/return stream | Node | hostile-provider | 回收并拒绝 | Pass |
| HR-T011 | 公共句柄 schema 失败 | invalid generated handleRef | Node | fault/negative | Provider 输出回收 | Pass |
| HR-T012 | handleRef 碰撞 | deterministic duplicate ref | Node | race/negative | 第二份回收，第一份保留 | Pass |
| HR-T013 | create/shutdown race | delayed Provider output | Node | race/fault | late output 回收，无登记 | Pass |
| HR-T014 | changed commit 幂等 | duplicate same key/input | Node | mutation/idempotency | 一次写、一个 event、同 receipt | Pass |
| HR-T015 | key 输入冲突 | same key/different commitRef | Node | security/negative | `IDEMPOTENCY_CONFLICT` | Pass |
| HR-T016 | revision conflict | external update before commit | Node | concurrency | conflict、不覆盖、不发假 event | Pass |
| HR-T017 | 双并发旧 revision | two handles/same base | Node | race | committed/conflict 各一、一个 event | Pass |
| HR-T018 | bytes digest | declared digest mismatch | Node | integrity/negative | commit 前拒绝 | Pass |
| HR-T019 | content kind binding | working-copy declaration + bytes handle | Node | integrity/negative | kind mismatch 拒绝 | Pass |
| HR-T020 | event retention | retention=1, old cursor | Node | replay/negative | current window 可读；旧 cursor expired | Pass |
| HR-T021 | materialize fault | injected Provider throw | Node | fault/leak | handle count=0 | Pass |
| HR-T022 | Host dispose 幂等 | active handle + duplicate dispose | Node | lifecycle | provider/handle/binding/receipt=0 | Pass |
| HR-T023 | disposer fault 隔离 | materialization + provider disposer throw | Node | fault/leak | 其余 disposer 继续；AggregateError | Pass |
| HR-T024 | read-only Provider commit | writable=false | Node | product/security | `SAVE_UNSUPPORTED`、无 event | Pass |
| HR-T025 | no-change commit | unchanged bytes | Node | mutation/event | revision 不变、无假 event | Pass |
| HR-T026 | describe fault 无副作用 | injected describe throw | Node | fault | registry/binding 保持，0 handle/receipt | Pass |
| HR-T027 | commit fault 可安全重试 | injected commit throw | Node | fault/idempotency | 0 receipt/event；同请求可重试 | Pass |
| HR-T028 | registry resolve p95 | 10k bindings | macOS arm64/Linux x64 | performance | ≤1 ms | Planned：F0.6 benchmark |
| HR-T029 | persistent Provider CAS | real workspace/cloud Provider | Desktop/Web | integration | 原子 revision、不丢数据 | Blocked：生产 Provider |
| HR-T030 | revoke in session states | Orchestrator state matrix | Desktop | E2E/fault | bounded close、handle invalid | Blocked：F0.4 |

说明：Vitest 中 HR-T005/006 合并为一个 test，HR-T007 的多个 mismatch 合并为一个 test，HR-T022 的 leak 与幂等断言合并，因此表中 27 个 Pass requirement 对应 25 个自动 test case。

Gate 规则：HR-T001–T027 全部 Pass 即通过 F0.2 Core Gate；HR-T028–T030 进入其标注的后续阶段，不用于虚假扩大当前完成范围。
