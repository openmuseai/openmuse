# `@muse/plugin-security` 技术说明

**路径：** `packages/core/plugin-security`  
**角色：** 安装/发行策略：artifact 签名、权限 diff、证据等级。不是 AppFlowy UI。

## 定位

`verifyArtifactV2`、`diffPermissionsV2`、trust / evidence 枚举。被 `@muse/ecosystem-sdk` 再导出。不执行 Plugin，不访问文档。

## 验证

```bash
cd packages/core/plugin-security && pnpm check
```
