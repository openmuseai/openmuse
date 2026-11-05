# F0.2 实施结果

最后更新：2026-09-15  
结论：**Host Resource Core Gate Pass；生产 Provider/E2E 尚未开始**

## 1. 完成内容

- `@muse/resource-host@0.1.0`：Registry、authority、materialization、commit、event、facade。
- `FakeResourceProvider`：bytes、revision、external change、fault injection、leak counters。
- build/link/copy/wire maps 已加入 F0 三合同包和 Resource Host。
- core README 与技术地图已登记新增包。
- F0.2 六类阶段文档和测试矩阵完成。
- 三个 vendor 运行代码未修改；Ontology Runtime 未引入。

## 2. 执行环境

| 工具 | 版本 |
|---|---|
| OS/arch | macOS / Apple Silicon workspace host |
| Node | v24.16.0 |
| pnpm | 10.34.1 |
| TypeScript | 7.0.2 |
| Vitest | 4.1.11 |

## 3. 命令与结果

```bash
cd Muse-Clients/middlewares/dsh/core/resource-host
pnpm install --offline --frozen-lockfile=false
pnpm check
Muse-Clients/middlewares/scripts/build-muse-packages.sh --skip-tests
```

结果：strict TypeScript typecheck Pass；Vitest **25/25 Pass**；declaration build Pass；整仓 Muse 包按新增依赖顺序全部构建并完成 DSH resolver linking。

覆盖结果：27 个 Core requirement 全部 Pass；部分 requirement 在同一 test 中对多个维度断言。性能、真实 Provider、Surface revoke E2E 明确留在 F0.6/生产 Provider/F0.4，不伪造结果。

## 4. 实施中发现并修复

1. 初版公开句柄 secrecy 测试用字符串 `v1`，会误匹配 protocol 版本；改为断言不存在 `bytes/path/url` 字段。
2. 无 cursor 订阅在 retention 截断后被误判 expired；改为返回当前 retention window，只有显式旧 cursor 才 expired。
3. Commit result 的联合类型不够易于 TypeScript 窄化；改为显式分支生成 closed receipt。
4. `ResourceEventV1` union 使用普通 `Omit` 会丢失分支字段；改为 distributive omit。
5. 初版 handle 只绑定 adapter/resource/revision；补齐 actor/workspace/session，并增加 mismatch 负例。
6. Provider 输出后若 handle schema 失败可能泄漏；改为 ownership-transfer `try/finally`。
7. Provider disposer 作为裸函数保存可能丢失 receiver；改为闭包调用。
8. 单一 disposer 抛错会中断后续清理；改为继续逆序回收并汇总 `AggregateError`。
9. Commit 补充 content kind 与 Host-local bytes digest 双重校验。

## 5. Gate 证据

| Gate | 结果 |
|---|---|
| fake describe/materialize/read/write/commit/event | Pass |
| ACL + full audience + TTL/revoke fail closed | Pass |
| duplicate/concurrent commit | Pass |
| create failure/shutdown/disposer fault cleanup | Pass |
| public handle no raw data | Pass |
| no Ontology dependency | Pass |
| production Provider CAS | Not in scope / Blocked |
| Host Bridge + DSH + Surface E2E | 后续 F0.3/F0.4 |

## 6. 决定

- F0.2 Host Resource Core：**Pass**。
- F0 总 Gate：仍未通过；缺 DSH seam、Dart/Orchestrator、fake Surface E2E。
- 下一阶段：F0.3 DSH Service Definition/Host Provider/UI+Agent Consumers，必须继续使用本包 public API，禁止把 Host-local resolve 暴露给 Agent。
