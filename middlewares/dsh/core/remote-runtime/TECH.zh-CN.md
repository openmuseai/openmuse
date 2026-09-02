# `@muse/remote-runtime` 技术说明

**路径：** `packages/core/remote-runtime`  
**角色：** `muse.remote/v2` 会话契约：device token、attachment、可恢复流。供 Mobile/Web **Remote DSH**，不是文档合同。

## 定位

`DeviceTokenAuthorityV2`、`ResumableStreamV2`、session attachment。Web **本期主路径未走它**：身份与 context 走 parent-bridge + Cloud device token。Android 壳在、产品主链未接 Gateway。

不要把本包当成 parent-bridge 的替代品去塞 iframe 消息。目标是 Host Bridge 上的 session 合同；本包是会话层类型与 HMAC 工具。

## 验证

```bash
cd packages/core/remote-runtime && pnpm check
```
