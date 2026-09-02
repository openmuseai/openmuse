# `@muse/plugin-appflowy-view-rename` 技术说明

**路径：** `packages/plugins/appflowy-view-rename`  
**逻辑 Plugin：** `muse.appflowy.view-rename`

## 定位

对 Host 当前 View 的 **提议/应用重命名**。写操作 `local_write`，须 Host policy。模型仍不能带 View id。

与 markdown 相同模式：`createMusePlugin` + kit 处理 grant。Folder/collab 仍在 Domain Host。

## 验证

```bash
cd packages/plugins/appflowy-view-rename && pnpm check
```
