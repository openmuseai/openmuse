# F0 发布与回滚方案

## 1. F0 不是用户功能发布

F0 是内部平台能力。F0.1 合同包只能在 repo 内被 Host/DSH/Client 依赖，不能单独宣称 Word/Helix/Viewer 已可用。真实 Adapter 的 release gate 分别属于 F1、F2、F3。

## 2. Flag

| Flag | F0 默认 | 用途 | 回滚 |
|---|---|---|---|
| `resourcePresentationV2` | internal/shadow | 新路由和 receipt 链 | off 后新请求走旧 Consumer |
| `engineOntologyHooks` | on | 发送惰性事实标签，无 Runtime consumer | off 只减少可选字段 |
| `viewerAdapter.wave1` | off | F1 | 保持 off |
| `iofficeWordAdapter.view` | off | F2 | 保持 off |
| `iofficeWordAdapter.edit` | off | F4 | F0/F2 不可开启 |
| `helixAdapter.view/edit` | off | F3/F5 | 保持 off |

Shadow 模式只执行 authorize/describe/probe/route 计算和差异记录，不 materialize、不启动引擎、不创建第二 Surface。

## 3. 兼容与 rollout

1. 合同包合入但无 consumer，验证 workspace build。
2. Host Provider internal 注册，旧链为主。
3. DSH/UI shadow 对比 resource resolution 与 route；不产生用户副作用。
4. 员工 cohort 使用 fake Adapter 端到端。
5. fault/soak/security 通过后签署 F0；再允许 F1/F2/F3 使用公共 API。

Schema breaking change 在真实 cohort 前完成。若 cohort 后必须 break，发布新 major 并双读旧 major，禁止原地改变相同 protocol 字符串。

## 4. 观测与报警

必须按 request/attempt/adapter/version/placement/errorCode 聚合：请求数、terminal receipt 数、duplicate/conflict、probe latency、open latency、fallback、cancel、forced terminate、active lease/materialization/session gauge。日志字段通过 allowlist，不记录 path、URL、token、文本或文件 bytes。

No-go：terminal 比率不为 100%、任何重复 Surface、任何 handle audience 绕过、leak gauge 不回零、无 Ontology Runtime 时失败、无法关闭新链回到旧链。

## 5. 回滚演练

1. 制造 probe 成功但 open 失败。
2. 确认 fallback receipt 指向实际 Adapter。
3. 关闭 `resourcePresentationV2`。
4. 新请求使用旧路径；活动新 session 按 owner policy 有界关闭。
5. registry/disposer/leak 指标归零。
6. Resource 真源与 revision 不变；不得删除用户数据或 working copy。

F0.1 的代码级回滚是移除尚未被 runtime consumer 引用的三个包；一旦 F0.2+ 开始消费，则通过协议 major/flag 兼容回滚，不直接删除 schema。

## 6. 签署

| 角色 | 必须确认 |
|---|---|
| Protocol/Host | schema、authority、materialization、revision |
| DSH | durable/model-visible、Provider/Consumer seam |
| Client Platform | Dart、Surface、cancel/generation/disposer |
| QA | matrix、soak、performance、device evidence |
| Security/Release | threat model、logging、flags、rollback |

当前未签署 F0 Gate。
