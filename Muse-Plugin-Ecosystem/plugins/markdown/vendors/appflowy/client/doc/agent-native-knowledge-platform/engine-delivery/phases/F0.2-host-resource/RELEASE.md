# F0.2 发布与回滚

## 1. 发布物

内部 npm 包：`@muse/resource-host@0.1.0`。构建脚本按以下顺序处理：Host Bridge → 三合同包 → Resource Host → 其他 core/plugins；打包和 DSH resolver map 已登记四个新增包。

本阶段没有公开 API 服务、数据库 migration 或用户可见 flag。包存在不等于真实 Resource Provider 已启用。

## 2. Admission

进入 F0.3 前必须满足：

- `pnpm check` 全通过；
- 依赖顺序脚本能在无旧 dist 的环境生成包；
- Host Bridge Provider 只能暴露 public descriptor/handle/receipt/event，不能暴露 `MaterializedData`；
- 生产 Provider 需要单独 threat model、atomic commit 和 recovery 测试；
- `resourcePresentationV2` 仍为 internal/shadow。

## 3. 灰度

1. F0.3 先用 Fake Provider 注册内部 capability。
2. shadow describe 与旧 locator metadata 对比，不请求数据面。
3. fake Adapter cohort 打开合成资源。
4. F0.4 leak/fault Gate 后才接第一个只读真实 Provider。
5. 写权限直到对应引擎 write Gate 前保持关闭。

## 4. 回滚

- 未被 consumer 使用时，可从 build/link map 移除包，不影响资源数据。
- 已接入后，通过关闭 `resourcePresentationV2` 停止新 binding/materialization；活动 owner 有界 dispose。
- 回滚不删除 Provider 真源、revision 或可恢复 working copy。
- 若 disposer 失败，保持报警与 orphan marker；不得通过吞错宣称回滚完成。

## 5. 观测

生产绑定前需要输出 allowlisted counters：providers、resourceBindings、materializations、writableMaterializations、commitReceipts、event retention，以及按 error code 的失败数。禁止输出 locator、bytes、path、URL、token 或文件名内容。

## 6. No-go

以下任一出现不得进入真实 Provider：authority/audience 绕过、duplicate commit 多事件、旧 revision 覆盖、create/shutdown late leak、disposer failure 被吞、公共 envelope 出现数据面、缺 Ontology consumer 导致失败。
