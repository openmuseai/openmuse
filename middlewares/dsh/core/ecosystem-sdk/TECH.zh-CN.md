# `@muse/ecosystem-sdk` 技术说明

**路径：** `packages/core/ecosystem-sdk`  
**角色：** 作者工具：scaffold / validate / plan / package digest / TCK。包装 `plugin-graph` + `plugin-security`。

## 定位

CLI：`bin/muse-sdk.mjs`。示例清单指向 `packages/plugins/appflowy-database/muse.plugin.json`。

不进入 DSH 运行时。桌面/Web 启动不依赖本包。

```bash
cd packages/core/ecosystem-sdk && pnpm build
node bin/muse-sdk.mjs validate ../../plugins/appflowy-database/muse.plugin.json
```
