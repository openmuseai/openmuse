# F0.2 Fixture 与故障注入

## 1. FakeResourceProvider

`@muse/resource-host/testing` 提供内存 bytes Provider。每个资源固定包含 resourceRef、displayName、mediaType、formatId、bytes、整数 revision 和 read/write capabilities。

默认 revision 从 `rev.fake.1` 开始；成功 changed commit 递增；`replaceExternally()` 模拟远端并发变更；`readBytes()` 只供测试断言。它不是生产存储，不允许进入 release bundle 的 Provider 配置。

## 2. 基准资源

| 字段 | 值 |
|---|---|
| resourceRef | `resource.fake.1` |
| displayName | `draft.docx` |
| format | `ooxml.word` |
| mediaType | OOXML Word MIME |
| initial bytes | UTF-8 `v1` |
| initial revision | `rev.fake.1` |
| adapter audience | `muse.fake.adapter~1.0.0` |
| actor/workspace/session | `actor.1/workspace.1/engine-session.1` |

文件名和格式只证明 Host 不依赖格式内部结构；fake Provider 不解析 DOCX。

## 3. Faults

| Fault | 注入方式 | 验证 |
|---|---|---|
| describe throw | `failNext("describe")` | 上层可映射 Provider fault |
| materialize throw | `failNext("materialize")` | 不创建 handle |
| commit throw | `failNext("commit")` | 不创建 false receipt/event |
| wrong data kind | custom Provider | output disposer 执行 |
| invalid handle ref | deterministic `createRef` | schema fail 后 disposer 执行 |
| handle collision | constant `createRef` | loser dispose，winner 保留 |
| slow materialize | deferred Promise | shutdown 胜出后 late output dispose |
| external revision | `replaceExternally()` | conflict，无覆盖 |
| disposer throw | wrapped Provider | 继续清其他对象并 AggregateError |

## 4. 数据规则

- fixture 只用合成 bytes，不放真实 Office 文件或客户内容。
- digest 使用实际 SHA-256 计算，避免测试绕过完整性检查。
- clock/createRef 可注入，测试无 sleep、无随机竞态依赖。
- 负例逐一改变 actor/workspace/session/adapter/resource/revision/mode，避免一个宽泛“权限失败”掩盖缺失绑定。
- 生产格式 corpus 归 F1/F2/F3；本阶段 fixture 不判断渲染 fidelity。
