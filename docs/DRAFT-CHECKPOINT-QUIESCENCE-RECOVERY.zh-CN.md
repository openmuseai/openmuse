# Draft、Checkpoint、Quiescence 与恢复

> 状态：Accepted
>
> 子需求：X3
>
> 日期：2026-09-29

## 1. 事务模型

每个 Sandbox checkout 是独立 Draft Transaction。Base Revision 不可变，Bash/worker 的写入进入 overlay；watcher 仅作提示，`prepareDraft` 必须对 base 与 overlay 做内容扫描和 SHA-256 校验，所以 watcher 丢事件不会漏提交。

状态为 `Open → Quiescent → Prepared → Committed`。若 expected-base CAS 失败则进入 `Conflict` 并保留 Draft；不做 last-write-wins。只有完整 process range 为零并完成 flush 后才允许 prepare。

## 2. Checkpoint 协议

`prepareDraft` 生成排序的 changed-entry manifest、manifest digest 和确定性 idempotency key。Revision Authority 以 Workspace HEAD 和 `expectedBaseRevision` 做 CAS：

- 成功：一次性产生新 Revision 与 receipt；
- 并发 HEAD 改变：返回 `CONFLICT`，overlay 不清理；
- response 丢失：本地仍保持 `Prepared`，相同 key 重试返回 replayed receipt；
- crash：从 durable journal 恢复 Prepared/Conflict 状态，再查询或重放相同提交。

因此网络故障不会出现“UI 显示成功但 Authority 没有 Revision”，也不会因重试生成两个 Revision。

## 3. 验收

统一入口：

```bash
./scripts/test_draft_checkpoint.sh
```

5 项测试覆盖 watcher 丢事件、后台 writer 拒绝/终止、expected-base 冲突、提交成功但响应丢失的幂等恢复，以及 crash 后从 journal 完成 checkpoint。
