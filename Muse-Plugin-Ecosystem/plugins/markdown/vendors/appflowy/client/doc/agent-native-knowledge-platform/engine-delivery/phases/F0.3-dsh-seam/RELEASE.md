# F0.3 发布、Gate 与回滚

## 1. 阶段 Gate

进入 F0.4 前必须满足：

- 四包 `check` 全绿，F0.3 Muse 20 tests 与 DSH Session 新增 1 test 无跳过；
- Muse 全包 build 通过，packed dependency wiring 可解析；
- `dsh-appflowy` inject 顺序正确；
- UI 三入口 snapshot 与 Agent model-visible snapshot 已固定；
- Host scope 无权限 flags，模型 schema/result 无 secret 字段；
- Muse 自定义事件带 `ignorable:true`，Surface 事件不能使用该标记；
- Ontology package 缺失不影响测试。

## 2. Feature flag

`resourcePresentationV2` 仍保持 internal/shadow。F0.3 bundle 注册 Service/UI Consumer，但
`present_resource` 在 bundle 中显式配置 `enabled: false`。没有 F0.4 Surface 时不得替换现有
用户可见 open flow；Shadow 只验证 capability 与 request，不创建第二个 Surface。

## 3. 发布顺序

1. contracts/resource-host；
2. `dsh-resource-presentation`；
3. Host Provider；
4. UI/Agent Consumers；
5. `dsh-appflowy` bundle patch；
6. F0.4 Orchestrator feature flag cohort。

## 4. 回滚

回滚只需从 bundle patch 移除三个运行插件并关闭 `resourcePresentationV2`；公共 contracts、
journal event readers 可保留以恢复 Muse 卡片；即使一并移除，`ignorable:true` 也保证 DSH
可加载历史 Session。禁止删除或重写已经持久化的 `muse/presentation-*` events。

## 5. F0.4 接棒条件

F0.4 必须实现 Fake Adapter/Surface 的 accepted→terminal 状态、cancel/late generation、
fallback attempt 与每步 disposer。DSH API、事件名和安全投影在接棒期间不得扩展为
engine-specific 方法。
