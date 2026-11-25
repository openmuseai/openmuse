# Office Dispatcher 与首个 Worker

> 状态：Accepted
>
> 子需求：X5
>
> 日期：2026-09-29

Agent 只寻址 `/runtime/bin/office`、`/runtime/bin/openmuse` 暴露的 registry command，不得到 worker path、bundle 或 control socket。Supervisor 按 X4 snapshot digest 解析命令，校验 cwd 固定为 `/workspace`、deadline、JSON schema 与 Lease grant，再生成一次性 worker identity。

参数从始至终是 `Vec<String>`，不经过第二层 shell。Worker 获得的权限仅为 command required permissions 与 Lease 已授权范围；UI Plugin 的权限、Secret 和 Host credential 不进入 worker context。超时会终止完整 worker process range。

首个 reference worker 是只读 DOCX inspect，复用同一 Rust Core，因此 Local 与 Cloud 对相同 bytes 返回相同 `openmuse.office.inspect@1` JSON。结果较大时合同要求写 Workspace/Blob 并只返回 handle；日志走 stderr，不混入 stdout protocol。

统一验收：

```bash
./scripts/test_worker_supervisor.sh
```

5 项测试覆盖不可直寻 worker、无 shell interpolation、timeout cleanup、Local/Cloud 一致输出和 UI 权限不泄漏。
