# F0.3 实施结果

## 1. 交付结果

- 四个包已实现并加入 local/packed DSH build map。
- `dsh-appflowy` bundle 按 Host Bridge → Provider → Consumers 顺序装配。
- UI 三入口使用同一 journaled request path。
- Agent 工具只接受当前 Session 的 recent discovered resourceRef。
- request/terminal 可从 Session log replay；模型结果走标准 Tool surface。
- Muse journal event 显式写入 `ignorable:true`；缺失 Muse 插件的 DSH 可安全加载 Session。
- F0.2 ResourceHost + FakeResourceProvider vertical slice 已通过。
- macOS、Windows portable 与 symlink-free closure 清单已同步，21 个包集合一致。
- 未修改三个 vendor，也未实现 Ontology Runtime。

## 2. 自动化结果

| 包 | Tests | typecheck | build |
|---|---:|---|---|
| `@muse/dsh-resource-presentation` | 3 | PASS | PASS |
| `@muse/dsh-resource-presentation-host` | 8 | PASS | PASS |
| `@muse/dsh-client-ui-resource-open` | 3 | PASS | PASS |
| `@muse/dsh-tool-resource-present` | 6 | PASS | PASS |
| F0.3 合计 | 20 | PASS | PASS |
| DSH Session 新增 producer contract | 1 | PASS | PASS |

F0.1 + F0.2 + F0.3 累计 97 项阶段专项 tests，其中 Muse 包 96 项、DSH 上游 seam
新增 1 项。全 Muse package build 与四包 resolver import 均已通过。此外，
DSH Session 完整测试文件 **78 tests Pass**，JSONL/SQLite unknown-ignorable load 各 1 项
focused contract Pass；`@muse/dsh-appflowy` 既有回归 **57 tests Pass**。这些既有回归不重复
计入 97 项阶段专项测试。

## 3. 关键证据

| 不变量 | 证据 |
|---|---|
| 不从 DSH wire 接受 authority | scope payload snapshot 无 canRead/canWrite |
| 不根据格式选引擎 | UI/Agent schema 与实现无 extension route/engineId |
| duplicate 不重放副作用 | 同 requestRef 两次调用仅一次 Host invoke |
| reload 不重放副作用 | `recover()` 只折叠 events |
| model-visible 可重建 | copied Session seed 的 derivedMessages 相等 |
| 仓库外事件不阻断 reload | 三类 Muse event 均为 ignorable；Surface marker misuse 被拒绝 |
| failure 有 durable terminal | Provider throw 转 failed receipt 并写 terminal |
| lifecycle 可回收 | binding invalidation rebind；dispose active=0；Tool schemas 清空 |
| 未到 Surface Gate 不暴露模型工具 | bundle config `enabled: false`；disabled registration test |

## 4. 已知边界

- Host 当前由 Fake Presentation capability 返回 terminal receipt；真正 accepted/pending/status
  状态机由 F0.4 Fake Orchestrator 实现。
- UI Consumer 尚未绑定 Flutter widget；本阶段交付的是 DSH 服务与调用面。
- Web/Mobile data-plane policy 属于 F0.5。
- P95/soak/内存曲线属于 F0.6，当前仅固定指标和可观测 counters。
